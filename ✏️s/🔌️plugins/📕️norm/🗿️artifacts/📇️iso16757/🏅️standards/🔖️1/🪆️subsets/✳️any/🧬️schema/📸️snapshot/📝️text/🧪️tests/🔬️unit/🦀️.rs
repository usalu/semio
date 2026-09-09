use super::*;
use crate::CatalogueValue;

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
    // ⚡️ Regression: `CatalogueValue`'s `Shape::Value` bridge goes through `dsl::DslValue::Number`,
    // which before it gained `UInt`/`Int`/`Float` fidelity used to turn `Integer { value: 50 }`
    // into a JSON float `50.0` that `serde_json::from_value` then rejected for the `i64` field.
    // Not exercised by the reference fixture (it only uses `Decimal`), so covered directly here.
    let value = CatalogueValue::Integer { value: 50 };
    let printed = <CatalogueValue as dsl::DslField>::to_value(&value);
    let parsed = <CatalogueValue as dsl::DslField>::from_value(&printed).expect("integer variant must round trip");
    assert_eq!(parsed, value);
}
