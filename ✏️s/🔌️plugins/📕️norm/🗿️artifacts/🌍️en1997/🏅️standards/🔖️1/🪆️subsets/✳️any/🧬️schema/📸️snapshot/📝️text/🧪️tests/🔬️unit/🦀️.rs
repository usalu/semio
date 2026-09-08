
use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1997Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(EN1997_DEFAULT_EXAMPLE_TEXT).expect("parse default .en1997 example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
