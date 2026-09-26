use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1992Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1992Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn compliant_office_frame_round_trips() {
    let document = En1992Snapshot::compliant_office_frame();
    assert!(!document.members.is_empty());
    assert_eq!(document.annex, crate::document::AnnexChoice::De);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn failing_under_reinforced_round_trips() {
    let document = En1992Snapshot::failing_under_reinforced();
    assert!(!document.members.is_empty());
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

