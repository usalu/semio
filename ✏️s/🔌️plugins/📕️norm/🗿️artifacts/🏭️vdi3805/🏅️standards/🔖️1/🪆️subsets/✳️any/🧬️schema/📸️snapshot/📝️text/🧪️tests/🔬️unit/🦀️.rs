use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips_the_reference_fixture() {
    store::os_store::test_support::assert_dsl_round_trip(&crate::reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(REFERENCE_EXAMPLE_TEXT).expect("parse bundled example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
