use super::*;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("math.values", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 11);
}

#[test]
fn a_value_of_the_wrong_type_is_a_localized_fault_never_a_conversion() {
    let kind = support::kind_of("math.integer");
    let inputs = WidgetInputs::new("w", kind, [("value".to_string(), GeometryValue::Number(2.0))].into_iter().collect());
    let evaluation = support::drive(integer(kind, inputs), 1);
    let fault = evaluation.fault.expect("a number is no integer");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-type", Some("value")));
    assert_ne!(fault.message.en, fault.message.de);
}
