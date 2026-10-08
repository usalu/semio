use super::*;
use std::collections::BTreeMap;

#[path = "../../../⏱️phased-job/🧰️test-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔁️brep-transform/🧫️fixtures/🔣️.json");

fn cube(size: f64) -> Arc<ShapeValue> {
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(size, size, size).expect("box");
    Arc::new(session.export(&handle).expect("export"))
}

fn away() -> Arc<ShapeValue> {
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let moved = session.brep().translate_sync(&handle, [3.0, 0.0, 0.0]).expect("translate");
    Arc::new(session.export(&moved).expect("export"))
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
    support::assert_category("brep.transform", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_the_analytic_numbers_the_third_party_reading_and_the_pinned_mesh_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 25);
}

#[test]
fn a_missing_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let (_, missing) = run("brep.transform.translate", vec![], 1);
    let fault = missing.fault.expect("fault");
    assert_eq!(fault.code, "generation3d.geometry.input-missing");
    assert!(fault.port.is_some() && fault.message.en != fault.message.de && missing.outputs.is_empty());
}

#[test]
fn patterns_are_stepped_jobs_that_yield_before_they_finish() {
    for (id, values) in [
        ("brep.transform.linearPattern", vec![("shape", GeometryValue::Shape(cube(1.0))), ("direction", GeometryValue::Vector([1.0, 0.0, 0.0])), ("spacing", GeometryValue::Number(2.0)), ("count", GeometryValue::Integer(3))]),
        ("brep.transform.circularPattern", vec![("shape", GeometryValue::Shape(away())), ("axis", GeometryValue::Vector([0.0, 0.0, 1.0])), ("count", GeometryValue::Integer(3))]),
    ] {
        let (slices, evaluation) = run(id, values, 1);
        assert!(slices >= 2, "{id}: needed {slices} Working slices at fuel 1");
        assert!(evaluation.fault.is_none(), "{id}: {:?}", evaluation.fault);
    }
}

#[test]
fn a_moved_vertex_sits_where_the_offset_puts_it() {
    let mut session = KernelSession::new();
    let vertex = session.brep().vertex_sync([1.0, 1.0, 1.0]).expect("vertex");
    let value = Arc::new(session.export(&vertex).expect("export"));
    let (_, evaluation) = run("brep.transform.translate", vec![("shape", GeometryValue::Shape(value)), ("offset", GeometryValue::Vector([1.0, 2.0, 3.0]))], usize::MAX);
    let GeometryValue::Shape(moved) = &evaluation.outputs["shape"] else { panic!("shape") };
    let semio_framework_3d::brep::engine::ShapeRoot::Vertex(id) = moved.root else { panic!("vertex") };
    let position = moved.body.vertices.get(id).expect("vertex").position;
    assert_eq!([position.x, position.y, position.z], [2.0, 3.0, 4.0]);
}

#[test]
fn a_motion_never_edits_the_input_value() {
    let input = cube(2.0);
    let before = input.content_hash();
    let (_, evaluation) = run("brep.transform.rotate", vec![("shape", GeometryValue::Shape(input.clone())), ("axis", GeometryValue::Vector([0.0, 0.0, 1.0])), ("angle", GeometryValue::Number(1.0))], usize::MAX);
    assert!(evaluation.fault.is_none());
    assert_eq!(input.content_hash(), before);
}

#[test]
fn a_copy_is_geometrically_the_input_and_a_new_value() {
    let input = cube(2.0);
    let (_, evaluation) = run("brep.transform.copy", vec![("shape", GeometryValue::Shape(input.clone()))], usize::MAX);
    let GeometryValue::Shape(copy) = &evaluation.outputs["shape"] else { panic!("shape") };
    assert_eq!(support::measures(&support::tessellate(copy, 0.1)).triangles, support::measures(&support::tessellate(&input, 0.1)).triangles);
}
