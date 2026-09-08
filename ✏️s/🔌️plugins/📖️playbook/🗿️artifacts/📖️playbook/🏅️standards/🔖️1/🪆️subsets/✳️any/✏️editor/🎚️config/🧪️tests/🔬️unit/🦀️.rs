
use super::*;

#[semio_framework_async_macros::async_test]
async fn playbook_config_default_matches_the_existing_runtime_defaults() {
    let config = PlaybookConfig::default();
    assert_eq!(config.locale, "en-US");
}

#[semio_framework_async_macros::async_test]
async fn playbook_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_config_round_trip(&PlaybookConfig::default());
    let populated = PlaybookConfig { locale: "de-DE".into(), contributions_json: "[]".into() };
    store::os_store::test_support::assert_config_round_trip(&populated);
}

#[semio_framework_async_macros::async_test]
async fn playbook_config_pack_round_trips() {
    let config = PlaybookConfig { locale: "de-DE".into(), contributions_json: "[]".into() };
    let bytes = store::ArtifactPack::encode_pack(&config);
    let decoded = <PlaybookConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode playbook config pack");
    assert_eq!(decoded, config);
}

fn config_round_trip(base: &PlaybookConfig, operation: &PlaybookConfigMutation) -> PlaybookConfig {
    let forward = operation.diff(base).diff().clone();
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().clone();
    }
    assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_apply_and_restore_every_field() {
    let base = PlaybookConfig::default();
    assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetLocale(SetLocale { value: "de-DE".into() })).locale, "de-DE");
    assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetContributions(SetContributions { json: "[]".into() })).contributions_json, "[]");
}

#[semio_framework_async_macros::async_test]
async fn playbook_config_operation_binary_matches_text() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: PlaybookConfig::default() }));
}
