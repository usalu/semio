use super::*;

/// 🧮️ Round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE): a
/// non-default fixture must survive `ArtifactDsl`/`ArtifactPack` byte-for-byte.
#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_dsl_pack_round_trips() {
    let config = VcsDemoConfig {};
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

/// 🧮️ Round-trip law per `VcsDemoConfigMutation` variant (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-
/// SCHEMA-FLOW-CONFIG-ON-NODE).
#[semio_framework_async_macros::async_test]
async fn vcs_demo_config_operation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::Noop);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&VcsDemoConfigMutation::Noop, &VcsDemoConfig {}).await;
    assert_eq!(<VcsDemoConfigMutation as Mutation<VcsDemoConfig>>::DESCRIPTORS.len(), 1);
    assert_eq!(VcsDemoConfigMutation::Noop.descriptor().aggregate_variant, "Noop");
}
