use crate::*;

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_round_trips() {
    semio_os_store_test_support::assert_dsl_round_trip(&NormResultsWindowConfig::default());
    semio_os_store_test_support::assert_dsl_round_trip(&NormResultsWindowConfig { selected_check_index: Some(3) });
}

#[semio_framework_async_macros::async_test]
async fn norm_config_dsl_pack_equivalence() {
    semio_os_store_test_support::assert_dsl_pack_equivalence(&NormResultsWindowConfig::default());
    semio_os_store_test_support::assert_dsl_pack_equivalence(&NormResultsWindowConfig { selected_check_index: Some(7) });
}
