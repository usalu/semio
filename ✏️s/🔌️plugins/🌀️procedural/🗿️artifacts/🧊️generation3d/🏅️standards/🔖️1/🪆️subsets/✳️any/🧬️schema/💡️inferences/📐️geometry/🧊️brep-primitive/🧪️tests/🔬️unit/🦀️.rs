use super::*;

use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;
use semio_s_artifact_procedural_3d_geometry_oracle_support as support;

const FIXTURE: &str = include_str!("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🧊️brep-primitive/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(catalogue(), FIXTURE, COMPUTES), 7);
}

#[test]
fn the_seed_table_lists_catalogue_kinds_of_this_category_only() {
    for entry in COMPUTES {
        assert_eq!(support::kind_of(catalogue(), entry.id).category, "brep.primitive");
    }
}

#[test]
fn a_box_is_deterministic_value_for_value_and_survives_a_json_round_trip() {
    let kind = support::kind_of(catalogue(), "brep.primitive.box");
    let build = || {
        let values = [("width", 2.0), ("depth", 3.0), ("height", 4.0)].into_iter().map(|(port, value)| (port.to_string(), GeometryValue::Number(value))).collect();
        support::drive(box_(kind, WidgetInputs::new_cold("w", kind, values)), usize::MAX)
    };
    let (first, second) = (build(), build());
    assert_eq!(first, second);
    let GeometryValue::Shape(shape) = &*first.outputs.get("shape").unwrap() else { panic!("a shape") };
    let text = semio_framework_pack_json::to_json_string(&**shape);
    let decoded: semio_framework_3d_brep_engine::ShapeValue = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the shape value decodes");
    assert_eq!(&decoded, &**shape);
}
