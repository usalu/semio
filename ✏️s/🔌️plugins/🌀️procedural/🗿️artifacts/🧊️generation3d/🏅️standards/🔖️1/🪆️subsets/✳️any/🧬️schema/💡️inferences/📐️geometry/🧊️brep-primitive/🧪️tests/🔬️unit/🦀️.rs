use super::*;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 7);
}

#[test]
fn the_seed_table_lists_catalogue_kinds_of_this_category_only() {
    for entry in COMPUTES {
        assert_eq!(support::kind_of(entry.id).category, "brep.primitive");
    }
}

#[test]
fn a_box_is_deterministic_value_for_value_and_survives_a_json_round_trip() {
    let kind = support::kind_of("brep.primitive.box");
    let build = || {
        let values = [("width", 2.0), ("depth", 3.0), ("height", 4.0)].into_iter().map(|(port, value)| (port.to_string(), GeometryValue::Number(value))).collect();
        support::drive(box_(kind, WidgetInputs::new("w", kind, values)), usize::MAX)
    };
    let (first, second) = (build(), build());
    assert_eq!(first, second);
    let GeometryValue::Shape(shape) = &first.outputs["shape"] else { panic!("a shape") };
    let text = semio_framework_pack_json::to_json_string(&**shape);
    let decoded: semio_framework_3d::brep::engine::ShapeValue = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the shape value decodes");
    assert_eq!(&decoded, &**shape);
}
