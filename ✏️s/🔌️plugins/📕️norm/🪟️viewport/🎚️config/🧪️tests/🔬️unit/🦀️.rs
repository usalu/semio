use crate::*;

fn sample() -> NormViewportWindowConfig {
    NormViewportWindowConfig { camera_preset: Some("iso".into()), hidden_layers: vec!["loads".into(), "annotations".into()], colour_map: Some("viridis".into()), unit_system: Some("si".into()) }
}

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&NormViewportWindowConfig::default());
    store::os_store::test_support::assert_dsl_round_trip(&sample());
}

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&NormViewportWindowConfig::default());
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample());
}
