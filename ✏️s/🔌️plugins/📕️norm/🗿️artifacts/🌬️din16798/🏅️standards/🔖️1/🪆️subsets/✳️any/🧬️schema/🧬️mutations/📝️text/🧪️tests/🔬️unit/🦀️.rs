use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    let m = demo_mutation_cases().into_iter().next().expect("cases");
    store::os_store::test_support::assert_op_line_round_trip(&m);
}

#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in demo_mutation_cases() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
