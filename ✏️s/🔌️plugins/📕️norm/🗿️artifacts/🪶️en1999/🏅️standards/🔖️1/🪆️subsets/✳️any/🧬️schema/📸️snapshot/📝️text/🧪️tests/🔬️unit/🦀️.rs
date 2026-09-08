
use super::*;
use crate::document::AnnexChoice;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1999Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn aluminium_roof_purlin_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT).expect("parse aluminium roof purlin example");
    assert_eq!(document.alloy, "aw6082t6");
    assert_eq!(document.annex, AnnexChoice::En);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
