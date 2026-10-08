use super::*;
use std::collections::BTreeMap;

#[path = "../../../⏱️phased-job/🧪️tests/🧰️support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✂️brep-intersect/🧫️fixtures/🔣️.json");

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
    support::assert_category("brep.intersect", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_the_analytic_numbers_the_third_party_reading_and_the_pinned_mesh_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 20);
}

#[test]
fn a_missing_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let (_, missing) = run("brep.intersect.section", vec![], 1);
    let fault = missing.fault.expect("fault");
    assert_eq!(fault.code, "generation3d.geometry.input-missing");
    assert!(fault.port.is_some() && fault.message.en != fault.message.de && missing.outputs.is_empty());
}

#[test]
fn section_and_split_are_stepped_jobs_that_yield_before_they_finish() {
    let plane = GeometryValue::Plane(PlaneValue { origin: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0] });
    for id in ["brep.intersect.section", "brep.intersect.split"] {
        let (slices, evaluation) = run(id, vec![("solid", GeometryValue::Shape(cube(2.0))), ("plane", plane.clone())], 1);
        assert!(slices >= 2, "{id}: needed {slices} Working slices at fuel 1");
        assert!(evaluation.fault.is_none(), "{id}: {:?}", evaluation.fault);
    }
}

#[test]
fn cancelling_a_split_midway_ends_in_the_cancelled_fault() {
    let kind = support::kind_of("brep.intersect.split");
    let values = BTreeMap::from([("solid".to_string(), GeometryValue::Shape(cube(2.0))), ("plane".to_string(), GeometryValue::Plane(PlaneValue { origin: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0] }))]);
    let entry = COMPUTES.iter().find(|entry| entry.id == kind.id).expect("registered");
    let mut job = (entry.start)(kind, WidgetInputs::new("w", kind, values));
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("finished") };
    assert_eq!(evaluation.fault.expect("fault").code, "generation3d.geometry.cancelled");
}

#[test]
fn the_wire_of_a_surface_intersection_is_a_consistent_value_the_kernel_can_import() {
    let plane = |origin: [f64; 3], normal: [f64; 3]| {
        let mut session = KernelSession::new();
        let handle = session.brep().plane_surface_sync(origin, normal).expect("plane");
        GeometryValue::Shape(Arc::new(session.export(&handle).expect("export")))
    };
    let (_, evaluation) = run("brep.intersect.surfaceSurface", vec![("a", plane([0.0; 3], [0.0, 0.0, 1.0])), ("b", plane([0.0; 3], [1.0, 0.0, 0.0])), ("tolerance", GeometryValue::Number(1e-6))], usize::MAX);
    let GeometryValue::List(wires) = &evaluation.outputs["wires"] else { panic!("list") };
    for wire in wires {
        let GeometryValue::Shape(wire) = wire else { panic!("shape") };
        wire.check().expect("consistent");
        KernelSession::new().import(wire).expect("importable");
    }
}
