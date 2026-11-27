use crate::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    semio_os_store_test_support::assert_dsl_round_trip(&En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
}

#[semio_framework_async_macros::async_test]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(EN1997_DEFAULT_EXAMPLE_TEXT).expect("parse default .en1997 example");
    semio_os_store_test_support::assert_dsl_round_trip(&document);
}
