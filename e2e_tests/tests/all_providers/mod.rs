// Copyright 2019 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use e2e_tests::TestClient;
use parsec_client::core::interface::requests::{Opcode, ProviderID, Result};
use std::collections::HashSet;
use uuid::Uuid;

#[test]
fn list_providers() {
    let mut client = TestClient::new();
    let providers = client.list_providers().expect("list providers failed");
    assert_eq!(providers.len(), 4);
    let uuids: HashSet<Uuid> = providers.iter().map(|p| p.uuid).collect();
    // Core provider
    assert!(uuids.contains(&Uuid::parse_str("47049873-2a43-4845-9d72-831eab668784").unwrap()));
    // Mbed Crypto provider
    assert!(uuids.contains(&Uuid::parse_str("1c1139dc-ad7c-47dc-ad6b-db6fdb466552").unwrap()));
    // PKCS 11 provider
    assert!(uuids.contains(&Uuid::parse_str("30e39502-eba6-4d60-a4af-c518b7f5e38f").unwrap()));
    // TPM provider
    assert!(uuids.contains(&Uuid::parse_str("1e4954a4-ff21-46d3-ab0c-661eeb667e1d").unwrap()));
}

#[test]
fn list_opcodes() {
    let mut client = TestClient::new();
    let mut crypto_providers_hsm = HashSet::new();
    let mut core_provider_opcodes = HashSet::new();

    let _ = crypto_providers_hsm.insert(Opcode::PsaGenerateKey);
    let _ = crypto_providers_hsm.insert(Opcode::PsaDestroyKey);
    let _ = crypto_providers_hsm.insert(Opcode::PsaSignHash);
    let _ = crypto_providers_hsm.insert(Opcode::PsaVerifyHash);
    let _ = crypto_providers_hsm.insert(Opcode::PsaImportKey);
    let _ = crypto_providers_hsm.insert(Opcode::PsaExportPublicKey);

    let mut crypto_providers_tpm = crypto_providers_hsm.clone();
    let _ = crypto_providers_tpm.insert(Opcode::PsaAsymmetricDecrypt);
    let _ = crypto_providers_tpm.insert(Opcode::PsaAsymmetricEncrypt);

    let mut crypto_providers_mbed_crypto = crypto_providers_tpm.clone();
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaHashCompute);
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaHashCompare);
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaRawKeyAgreement);
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaAeadEncrypt);
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaAeadDecrypt);
    let _ = crypto_providers_mbed_crypto.insert(Opcode::PsaExportKey);

    let _ = core_provider_opcodes.insert(Opcode::Ping);
    let _ = core_provider_opcodes.insert(Opcode::ListProviders);
    let _ = core_provider_opcodes.insert(Opcode::ListOpcodes);

    assert_eq!(
        client
            .list_opcodes(ProviderID::Core)
            .expect("list providers failed"),
        core_provider_opcodes
    );
    assert_eq!(
        client
            .list_opcodes(ProviderID::Tpm)
            .expect("list providers failed"),
        crypto_providers_tpm
    );
    assert_eq!(
        client
            .list_opcodes(ProviderID::Pkcs11)
            .expect("list providers failed"),
        crypto_providers_hsm
    );
    assert_eq!(
        client
            .list_opcodes(ProviderID::MbedCrypto)
            .expect("list providers failed"),
        crypto_providers_mbed_crypto
    );
}

#[cfg(feature = "testing")]
#[test]
fn mangled_list_providers() {
    let mut client = RequestTestClient::new();
    let mut req = Request::new();
    req.header.version_maj = 1;
    req.header.provider = ProviderID::Core;
    req.header.opcode = Opcode::ListProviders;

    req.body = RequestBody::_from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55]);

    let resp = client.send_request(req).expect("Failed to read response");
    assert_eq!(resp.header.status, ResponseStatus::DeserializingBodyFailed);
}

#[test]
fn sign_verify_with_provider_discovery() -> Result<()> {
    let mut client = TestClient::new();
    let key_name = String::from("sign_verify_with_provider_discovery");
    client.generate_rsa_sign_key(key_name)
}
