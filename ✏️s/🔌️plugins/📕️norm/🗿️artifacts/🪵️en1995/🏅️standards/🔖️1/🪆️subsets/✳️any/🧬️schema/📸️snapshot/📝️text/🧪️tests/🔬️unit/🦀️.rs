use super::*;
use crate::document::AnnexChoice;
use crate::MemberRole;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1995Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn glulam_footbridge_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT).expect("parse glulam footbridge example");
    assert_eq!(document.annex, AnnexChoice::De);
    assert_eq!(document.members.len(), 1);
    assert_eq!(document.members[0].role, MemberRole::Bridge);
    assert_eq!(document.members[0].strength_class, "GL28h");
    assert!(document.connections.is_empty());
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn every_bundled_example_round_trips_through_the_dsl() {
    for example in crate::examples() {
        let document = parse_dsl(&example.document()).unwrap_or_else(|e| panic!("{}: {e:?}", example.id()));
        assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document, "{}", example.id());
    }
}
