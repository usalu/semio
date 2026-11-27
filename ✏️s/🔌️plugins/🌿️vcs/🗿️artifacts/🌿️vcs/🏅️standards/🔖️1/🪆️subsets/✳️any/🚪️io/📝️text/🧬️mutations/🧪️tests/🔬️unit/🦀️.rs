#[semio_framework_async_macros::async_test]
async fn vcs_demo_mutation_op_text_round_trips() {
    semio_os_store_test_support::assert_op_line_round_trip(&crate::mutations::change_counter(3));
}
