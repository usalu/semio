use super::*;
use std::collections::BTreeMap;

#[path = "../../../⏱️phased-job/🧰️test-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🎯️brep-evaluate/🧫️fixtures/🔣️.json");

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
    support::assert_category("brep.evaluate", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_the_analytic_numbers_the_third_party_reading_and_the_pinned_mesh_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 30);
}

#[test]
fn a_missing_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let (_, missing) = run("brep.evaluate.curvePoint", vec![], 1);
    let fault = missing.fault.expect("fault");
    assert_eq!(fault.code, "generation3d.geometry.input-missing");
    assert!(fault.port.is_some() && fault.message.en != fault.message.de && missing.outputs.is_empty());
}

#[test]
fn an_evaluation_is_a_cheap_job_that_is_done_on_its_first_step() {
    let circle = {
        let mut session = KernelSession::new();
        let handle = session.brep().circle_curve_sync([0.0; 3], [0.0, 0.0, 1.0], 2.0).expect("circle");
        GeometryValue::Shape(Arc::new(session.export(&handle).expect("export")))
    };
    let (slices, evaluation) = run("brep.evaluate.curvePoint", vec![("curve", circle), ("parameter", GeometryValue::Number(0.5))], 1);
    assert_eq!(slices, 0);
    assert!(evaluation.fault.is_none());
}

#[test]
fn a_circle_follows_its_closed_form_at_many_parameters() {
    let mut session = KernelSession::new();
    let handle = session.brep().circle_curve_sync([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 2.0).expect("circle");
    let circle = GeometryValue::Shape(Arc::new(session.export(&handle).expect("export")));
    for step in 0..16 {
        let t = std::f64::consts::TAU * step as f64 / 16.0;
        let (_, point) = run("brep.evaluate.curvePoint", vec![("curve", circle.clone()), ("parameter", GeometryValue::Number(t))], 1);
        let GeometryValue::Point(found) = point.outputs["point"] else { panic!("point") };
        let expected = [1.0 + 2.0 * t.cos(), 2.0 + 2.0 * t.sin(), 3.0];
        assert!((0..3).all(|axis| (found[axis] - expected[axis]).abs() < 1e-9), "t = {t}: {found:?} vs {expected:?}");
        let (_, closest) = run("brep.evaluate.curveClosestParameter", vec![("curve", circle.clone()), ("point", GeometryValue::Point([1.0 + 5.0 * t.cos(), 2.0 + 5.0 * t.sin(), 3.0]))], 1);
        let GeometryValue::Number(distance) = closest.outputs["distance"] else { panic!("number") };
        assert!((distance - 3.0).abs() < 1e-7, "t = {t}: distance {distance}");
    }
}

#[test]
fn the_wire_parameter_is_the_member_index_plus_the_fraction_in_member_direction() {
    let mut session = KernelSession::new();
    let handle = session.brep().rectangle_wire_sync(2.0, 1.0).expect("rectangle");
    let wire = GeometryValue::Shape(Arc::new(session.export(&handle).expect("export")));
    for (parameter, expected) in [(0.0, [0.0, 0.0, 0.0]), (1.0, [2.0, 0.0, 0.0]), (2.0, [2.0, 1.0, 0.0]), (3.0, [0.0, 1.0, 0.0]), (4.0, [0.0, 0.0, 0.0]), (2.25, [1.5, 1.0, 0.0])] {
        let (_, evaluation) = run("brep.evaluate.curvePoint", vec![("curve", wire.clone()), ("parameter", GeometryValue::Number(parameter))], 1);
        let GeometryValue::Point(found) = evaluation.outputs["point"] else { panic!("point") };
        assert!((0..3).all(|axis| (found[axis] - expected[axis]).abs() < 1e-9), "s = {parameter}: {found:?}");
    }
}
