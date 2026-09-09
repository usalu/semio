use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1993Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1993Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn high_strength_connection_example_fixture_parses_and_round_trips() {
    use crate::document::AnnexChoice;
    let document = parse_dsl(EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT).expect("parse high strength connection example");
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.bolt_n_bolts, 4);
    assert_eq!(document.fatigue_method, "safe_life");
    assert_eq!(document.hss_section_class, 3);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
