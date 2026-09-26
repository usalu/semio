use super::*;

#[semio_framework_async_macros::async_test]
async fn de_office_compliant_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1991_DE_OFFICE_COMPLIANT_EXAMPLE_TEXT).expect("parse de office compliant example");
    assert_eq!(document.annex, crate::document::AnnexChoice::De);
    let printed = print_dsl(&document);
    let again = parse_dsl(&printed).expect("round-trip parse");
    assert_eq!(again.annex, document.annex);
    assert_eq!(again.snow_zone, document.snow_zone);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips_dsl() {
    let document = crate::En1991Snapshot::default();
    let printed = print_dsl(&document);
    let again = parse_dsl(&printed).expect("default parse");
    assert_eq!(again.altitude, document.altitude);
}
