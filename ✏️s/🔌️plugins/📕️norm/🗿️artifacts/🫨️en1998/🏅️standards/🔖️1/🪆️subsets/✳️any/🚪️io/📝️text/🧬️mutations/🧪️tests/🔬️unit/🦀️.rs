use crate::text::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_insert_building() {
    semio_os_store_test_support::assert_op_line_round_trip(&demo_mutation_cases()[1]);
}

#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in demo_mutation_cases() {
        semio_os_store_test_support::assert_op_line_round_trip(&mutation);
    }
}
