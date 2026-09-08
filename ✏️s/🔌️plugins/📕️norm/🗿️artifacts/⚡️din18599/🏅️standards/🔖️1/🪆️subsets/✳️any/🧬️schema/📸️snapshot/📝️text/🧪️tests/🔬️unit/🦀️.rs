
use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&Din18599Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("parse bundled example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
