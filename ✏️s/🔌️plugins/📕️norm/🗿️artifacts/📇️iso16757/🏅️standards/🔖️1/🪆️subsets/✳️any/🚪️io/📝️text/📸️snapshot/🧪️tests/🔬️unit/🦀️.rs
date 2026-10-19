use crate::snapshot::*;
use semio_s_artifact_norm_iso16757_model::CatalogueValue;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips_the_reference_fixture() {
    store::os_store::test_support::assert_dsl_round_trip(&Iso16757Snapshot::reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(ISO16757_DEFAULT_EXAMPLE_TEXT).expect("parse default .iso16757 example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn catalogue_value_integer_variant_round_trips_through_the_dsl_field_bridge() {
    let value = CatalogueValue::Integer { value: 50 };
    let printed = <CatalogueValue as semio_framework_dsl_record::DslField>::to_value(&value);
    let parsed = <CatalogueValue as semio_framework_dsl_record::DslField>::from_value(&printed).expect("integer variant must round trip");
    assert_eq!(parsed, value);
}
