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
async fn default_snapshot_is_de_building() {
    let document = En1994Snapshot::default();
    assert_eq!(document.annex, AnnexChoice::De);
    assert_eq!(document.structure_kind, "building");
    assert!(!document.beams.is_empty());
}
