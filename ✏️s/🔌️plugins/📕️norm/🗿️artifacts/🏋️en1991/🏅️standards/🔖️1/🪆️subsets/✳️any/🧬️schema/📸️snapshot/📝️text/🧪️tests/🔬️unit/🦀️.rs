use super::*;
use crate::document::{AnnexChoice, ImposedCategory};
use crate::part_1_2::FireCurve;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1991Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1991Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn retail_hydrocarbon_fire_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1991_RETAIL_HYDROCARBON_FIRE_EXAMPLE_TEXT).expect("parse retail hydrocarbon fire example");
    assert_eq!(document.category, ImposedCategory::D);
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.fire_curve, FireCurve::Hydrocarbon);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
