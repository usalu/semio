use super::*;
use std::collections::BTreeMap;

#[path = "../../../⏱️phased-job/🧪️tests/🧰️support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛠️brep-feature/🧫️fixtures/🔣️.json");

fn cube() -> Arc<ShapeValue> {
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(2.0, 2.0, 2.0).expect("box");
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
    support::assert_category("brep.feature", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_the_analytic_numbers_the_third_party_reading_and_the_pinned_mesh_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 20);
}

#[test]
fn every_feature_is_a_stepped_job_that_yields_before_it_finishes() {
    let edges = {
        let mut session = KernelSession::new();
        let shape = cube();
        let imported = session.import(&shape).expect("import");
        let handle = imported.handle;
        let table = session.brep().deconstruct_sync(&handle).expect("deconstruct");
        let label = session.brep().label_of(&table.edges[0]).expect("label");
        SelectionValue { component: SelectionKind::Edge, ids: vec![label.0] }
    };
    let shape = GeometryValue::Shape(cube());
    for (id, values) in [
        ("brep.feature.fillet", vec![("shape", shape.clone()), ("radius", GeometryValue::Number(0.25))]),
        ("brep.feature.chamfer", vec![("shape", shape.clone()), ("distance", GeometryValue::Number(0.25))]),
        ("brep.feature.shell", vec![("shape", shape.clone()), ("thickness", GeometryValue::Number(0.2))]),
        ("brep.feature.offsetSolid", vec![("shape", shape.clone()), ("distance", GeometryValue::Number(0.25))]),
        ("brep.feature.filletEdges", vec![("shape", shape.clone()), ("edges", GeometryValue::Selection(edges.clone())), ("radius", GeometryValue::Number(0.25))]),
    ] {
        let (slices, evaluation) = run(id, values, 1);
        assert!(slices >= 2, "{id}: needed {slices} Working slices at fuel 1");
        assert!(evaluation.fault.is_none(), "{id}: {:?}", evaluation.fault);
    }
}

#[test]
fn a_selection_taken_from_an_older_value_goes_stale_once_the_labels_are_gone() {
    let (_, rounded) = run("brep.feature.fillet", vec![("shape", GeometryValue::Shape(cube())), ("radius", GeometryValue::Number(0.25))], usize::MAX);
    let GeometryValue::Shape(rounded) = rounded.outputs["shape"].clone() else { panic!("shape") };
    let plain = cube();
    let only_in_rounded = rounded.components(semio_framework_3d::brep::engine::GeometryKind::Edge).into_iter().map(|component| component.label.0).find(|label| !plain.components(semio_framework_3d::brep::engine::GeometryKind::Edge).iter().any(|edge| edge.label.0 == *label)).expect("the fillet mints new edge labels");
    let selection = SelectionValue { component: SelectionKind::Edge, ids: vec![only_in_rounded] };
    let (_, evaluation) = run("brep.feature.chamferEdges", vec![("shape", GeometryValue::Shape(plain)), ("edges", GeometryValue::Selection(selection)), ("distance", GeometryValue::Number(0.1))], usize::MAX);
    let fault = evaluation.fault.expect("stale");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.selection-stale", Some("edges")));
    assert!(fault.message.en != fault.message.de);
}

#[test]
fn a_feature_never_edits_the_input_value() {
    let input = cube();
    let before = input.content_hash();
    let (_, evaluation) = run("brep.feature.fillet", vec![("shape", GeometryValue::Shape(input.clone())), ("radius", GeometryValue::Number(0.25))], usize::MAX);
    assert!(evaluation.fault.is_none());
    assert_eq!(input.content_hash(), before);
}

#[test]
fn cancelling_a_feature_midway_ends_in_the_cancelled_fault_without_outputs() {
    let kind = support::kind_of("brep.feature.fillet");
    let values = BTreeMap::from([("shape".to_string(), GeometryValue::Shape(cube())), ("radius".to_string(), GeometryValue::Number(0.25))]);
    let mut job = (COMPUTES[0].start)(kind, WidgetInputs::new("w", kind, values));
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("finished") };
    assert_eq!(evaluation.fault.expect("fault").code, "generation3d.geometry.cancelled");
    assert!(evaluation.outputs.is_empty());
}

#[test]
fn a_missing_or_mistyped_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let (_, missing) = run("brep.feature.fillet", vec![("radius", GeometryValue::Number(0.25))], 1);
    let fault = missing.fault.expect("fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-missing", Some("shape")));
    assert!(fault.message.en != fault.message.de);
    let (_, mistyped) = run("brep.feature.fillet", vec![("shape", GeometryValue::Number(1.0)), ("radius", GeometryValue::Number(0.25))], 1);
    assert_eq!(mistyped.fault.expect("fault").code, "generation3d.geometry.input-type");
}
