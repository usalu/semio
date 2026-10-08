use super::*;
use std::collections::BTreeMap;

#[path = "../../../⏱️phased-job/🧪️tests/🧰️support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🐚️brep-topology/🧫️fixtures/🔣️.json");

fn cube(size: f64) -> Arc<ShapeValue> {
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(size, size, size).expect("box");
    Arc::new(session.export(&handle).expect("export"))
}

fn run(id: &str, values: Vec<(&str, GeometryValue)>, fuel: usize) -> (usize, WidgetEvaluation) {
    let kind = support::kind_of(id);
    let values: BTreeMap<String, GeometryValue> = values.into_iter().map(|(port, value)| (port.to_string(), value)).collect();
    let entry = COMPUTES.iter().find(|entry| entry.id == id).expect("registered");
    support::slices((entry.start)(kind, WidgetInputs::new("w", kind, values)), fuel)
}

#[test]
fn fixtures_are_refreshed_only_on_request() {
    if support::writing() {
        support::refresh(FIXTURE_PATH, FIXTURE, COMPUTES);
    }
}

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("brep.topology", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_the_analytic_numbers_the_third_party_reading_and_the_pinned_mesh_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 20);
}

#[test]
fn a_missing_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let (_, missing) = run("brep.topology.vertex", vec![], 1);
    let fault = missing.fault.expect("fault");
    assert_eq!(fault.code, "generation3d.geometry.input-missing");
    assert!(fault.port.is_some() && fault.message.en != fault.message.de && missing.outputs.is_empty());
}

#[test]
fn decomposition_exports_one_component_per_unit_of_fuel() {
    let (slices, evaluation) = run("brep.topology.deconstruct", vec![("shape", GeometryValue::Shape(cube(2.0)))], 1);
    assert!(slices >= 27, "8 + 12 + 6 + 1 components need at least 27 slices at fuel 1, saw {slices}");
    assert!(evaluation.fault.is_none());
    let (whole, _) = run("brep.topology.deconstruct", vec![("shape", GeometryValue::Shape(cube(2.0)))], usize::MAX);
    assert_eq!(whole, 0);
}

#[test]
fn the_label_is_the_root_label_of_the_value() {
    let input = cube(2.0);
    let (_, evaluation) = run("brep.topology.label", vec![("shape", GeometryValue::Shape(input.clone()))], 1);
    assert_eq!(evaluation.outputs["label"], GeometryValue::Text(input.label().expect("label").0.to_string()));
}

#[test]
fn a_vertex_widget_places_the_vertex_at_the_point() {
    let (_, evaluation) = run("brep.topology.vertex", vec![("point", GeometryValue::Point([1.0, 2.0, 3.0]))], 1);
    let GeometryValue::Shape(vertex) = &evaluation.outputs["shape"] else { panic!("shape") };
    let semio_framework_3d::brep::engine::ShapeRoot::Vertex(id) = vertex.root else { panic!("vertex") };
    let position = vertex.body.vertices.get(id).expect("vertex").position;
    assert_eq!([position.x, position.y, position.z], [1.0, 2.0, 3.0]);
}

#[test]
fn an_empty_compound_input_is_a_localized_fault_at_the_port() {
    let (_, evaluation) = run("brep.topology.compound", vec![("solids", GeometryValue::List(vec![]))], 1);
    let fault = evaluation.fault.expect("fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.topology-empty", Some("solids")));
    assert!(fault.message.en != fault.message.de);
}
