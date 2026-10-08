use super::*;
use std::collections::BTreeMap;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("math.arithmetic", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 50);
}

#[test]
fn round_always_answers_an_integer_and_every_other_kind_a_number() {
    let (_, cases) = support::cases(FIXTURE);
    for case in cases.iter().filter(|case| case.get("outputs").is_some()) {
        let kind = support::kind_of(case["kind"].as_str().unwrap());
        let evaluation = support::evaluate(kind, support::inputs_of(kind, case), COMPUTES, usize::MAX);
        let result = evaluation.outputs.get("result").expect("result output");
        assert_eq!(matches!(result, GeometryValue::Integer(_)), kind.id == "math.round", "{}", kind.id);
        assert_eq!(evaluation.quality, kind.quality, "{}", kind.id);
    }
}

#[test]
fn an_integer_input_is_accepted_where_a_number_is_declared() {
    let kind = support::kind_of("math.add");
    let values = BTreeMap::from([("a".to_string(), GeometryValue::Integer(2)), ("b".to_string(), GeometryValue::Number(0.5))]);
    let evaluation = support::drive(registry_entry("math.add")(kind, WidgetInputs::new("w", kind, values)), 1);
    assert_eq!(evaluation.outputs.get("result"), Some(&GeometryValue::Number(2.5)));
}

#[test]
fn a_missing_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let kind = support::kind_of("math.divide");
    let values = BTreeMap::from([("a".to_string(), GeometryValue::Number(1.0))]);
    let evaluation = support::drive(registry_entry("math.divide")(kind, WidgetInputs::new("w", kind, values)), 1);
    let fault = evaluation.fault.expect("a fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-missing", Some("b")));
    assert!(evaluation.outputs.is_empty());
}

fn registry_entry(id: &str) -> StartFn {
    COMPUTES.iter().find(|entry| entry.id == id).expect("registered").start
}
