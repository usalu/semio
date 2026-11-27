use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[test]
fn playground_snapshot_dsl_round_trips() {
    semio_os_store_test_support::assert_dsl_round_trip(&crate::standards::v1::subsets::any::schema::empty_playground_snapshot());
}

#[test]
fn default_example_dsl_round_trips() {
    let document = parse_dsl(PLAYGROUND_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default playground example");
    semio_os_store_test_support::assert_dsl_round_trip(&document);
}
