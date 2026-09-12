use super::*;

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&NormResultsWindowConfig::default());
    store::os_store::test_support::assert_dsl_round_trip(&NormResultsWindowConfig { selected_check_index: Some(3) });
}

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&NormResultsWindowConfig::default());
    store::os_store::test_support::assert_dsl_pack_equivalence(&NormResultsWindowConfig { selected_check_index: Some(7) });
}
