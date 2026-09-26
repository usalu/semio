use super::*;
use crate::document::AnnexChoice;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1999Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn aluminium_roof_purlin_example_fixture_parses_and_round_trips() {
    let document = En1999Snapshot::compliant_roof_purlin();
    assert!(!document.materials.is_empty());
    assert_eq!(document.annex, AnnexChoice::De);
    assert!(document.materials.iter().any(|m| m.designation.to_lowercase().contains("6082")));
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
