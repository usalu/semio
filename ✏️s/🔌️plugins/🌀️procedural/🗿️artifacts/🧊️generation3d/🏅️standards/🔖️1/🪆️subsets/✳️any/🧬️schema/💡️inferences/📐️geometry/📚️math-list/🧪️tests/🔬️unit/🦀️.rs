use super::*;
use std::collections::BTreeMap;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("math.list", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 17);
}

fn run(id: &str, values: BTreeMap<String, GeometryValue>) -> WidgetEvaluation {
    let kind = support::kind_of(id);
    let start = COMPUTES.iter().find(|entry| entry.id == id).expect("registered").start;
    support::drive(start(kind, WidgetInputs::new("w", kind, values)), 1)
}

#[test]
fn a_count_beyond_the_catalogue_limit_is_refused_instead_of_truncated() {
    for (count, id) in [(100_001, "math.range"), (1, "math.range"), (0, "math.series"), (100_001, "math.series")] {
        let values = BTreeMap::from([("start".to_string(), GeometryValue::Number(0.0)), ("end".to_string(), GeometryValue::Number(1.0)), ("step".to_string(), GeometryValue::Number(1.0)), ("count".to_string(), GeometryValue::Integer(count))]);
        let fault = run(id, values).fault.expect("a fault");
        assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.math-range", Some("count")), "{id} {count}");
    }
}

#[test]
fn the_largest_sequence_has_exactly_the_limit_of_numbers_and_its_ends() {
    let values = BTreeMap::from([("start".to_string(), GeometryValue::Number(-1.0)), ("end".to_string(), GeometryValue::Number(1.0)), ("count".to_string(), GeometryValue::Integer(100_000))]);
    let evaluation = run("math.range", values);
    let Some(GeometryValue::List(numbers)) = evaluation.outputs.get("numbers") else { panic!("numbers: {evaluation:?}") };
    assert_eq!((numbers.len(), numbers.first(), numbers.last()), (100_000, Some(&GeometryValue::Number(-1.0)), Some(&GeometryValue::Number(1.0))));
}

#[test]
fn a_list_item_is_the_value_itself_so_shapes_are_shared_not_copied() {
    let shape = support::build_shape(&serde_json::json!({ "recipe": "box", "size": [1.0, 1.0, 1.0] }));
    let item = GeometryValue::shape(shape);
    let values = BTreeMap::from([("list".to_string(), GeometryValue::List(vec![GeometryValue::Integer(1), item.clone()])), ("index".to_string(), GeometryValue::Integer(1))]);
    let evaluation = run("math.listItem", values);
    let (Some(GeometryValue::Shape(found)), GeometryValue::Shape(original)) = (evaluation.outputs.get("item"), &item) else { panic!("a shape: {evaluation:?}") };
    assert!(std::sync::Arc::ptr_eq(found, original));
}
