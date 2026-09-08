
use super::*;

#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_default_is_english_locale() {
    let config = VcsDemoConfig::default();
    assert_eq!(config.locale, "en-US");
}

/// 🧮️ Round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE): a
/// non-default fixture must survive `ArtifactDsl`/`ArtifactPack` byte-for-byte.
#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_dsl_pack_round_trips() {
    let config = VcsDemoConfig { locale: "de-DE".into() };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

/// 🧮️ Round-trip law per `VcsDemoConfigMutation` variant (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-
/// SCHEMA-FLOW-CONFIG-ON-NODE).
#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_operation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::Snapshot { config: VcsDemoConfig { locale: "de-DE".into() } });
    store::os_store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::SetLocale { value: "de-DE".into() });
    assert_eq!(<VcsDemoConfigMutation as Mutation<VcsDemoConfig>>::DESCRIPTORS.len(), 2);
    assert_eq!(VcsDemoConfigMutation::Snapshot { config: VcsDemoConfig::default() }.descriptor().aggregate_variant, "Snapshot");
    assert_eq!(VcsDemoConfigMutation::SetLocale { value: "de-DE".into() }.descriptor().semantic_kind, "set-locale");
}

/// ⏪️ `backwards()` always returns a `Snapshot` of the pre-operation config, so applying it after
/// the forward op exactly restores the original — the "whole-config-snapshot-undo" law.
#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_operation_backwards_restores_the_base_config() {
    let base = VcsDemoConfig { locale: "en-US".into() };
    let operation = VcsDemoConfigMutation::SetLocale { value: "de-DE".into() };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward.locale, "de-DE");
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![VcsDemoConfigMutation::Snapshot { config: base.clone() }]);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(restored, base);
}
