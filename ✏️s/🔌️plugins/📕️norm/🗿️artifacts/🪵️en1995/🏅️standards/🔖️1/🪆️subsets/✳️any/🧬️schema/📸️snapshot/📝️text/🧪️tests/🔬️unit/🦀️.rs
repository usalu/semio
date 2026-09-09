use super::*;
use crate::document::AnnexChoice;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1995Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn glulam_footbridge_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT).expect("parse glulam footbridge example");
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.service_class, "sc2");
    assert_eq!(document.load_duration, "long");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
