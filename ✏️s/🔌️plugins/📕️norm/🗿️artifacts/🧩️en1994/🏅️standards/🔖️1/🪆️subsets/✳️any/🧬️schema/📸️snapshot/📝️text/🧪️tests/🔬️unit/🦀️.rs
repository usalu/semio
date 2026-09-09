use super::*;
use crate::document::AnnexChoice;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1994Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1994Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn composite_bridge_girder_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT).expect("parse composite bridge girder example");
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.fire_rating, "r90");
    assert_eq!(document.deck_type, "re-entrant");
    assert_eq!(document.fatigue_detail, "shear_connector");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
