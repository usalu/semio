use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_update_site() {
    store::os_store::test_support::assert_op_line_round_trip(&demo_mutation_cases()[1]);
}

#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in demo_mutation_cases() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
