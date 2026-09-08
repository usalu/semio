
#[semio_framework_async_macros::async_test]
async fn vcs_demo_mutation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&crate::mutations::change_counter(3));
}
