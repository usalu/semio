use super::*;
use std::sync::Arc;
use semio_framework_3d::brep::engine::ShapeValue;
use std::collections::BTreeMap;

#[path = "../../../../../🧬️schema/💡️inferences/📐️geometry/⏱️phased-job/🧪️tests/🧰️support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");
const FIXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/💾️brep-interchange/🧫️fixtures/🔣️.json");

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

fn text_of(evaluation: &WidgetEvaluation, port: &str) -> String {
    let Some(GeometryValue::Text(text)) = evaluation.outputs.get(port) else { panic!("text output {port}: {:?}", evaluation.fault) };
    text.clone()
}

fn volume_of(evaluation: &WidgetEvaluation) -> (f64, usize) {
    let Some(GeometryValue::Shape(shape)) = evaluation.outputs.get("shape") else { panic!("shape output: {:?}", evaluation.fault) };
    (support::kernel_measures(shape).0.expect("a volume"), shape.components(semio_framework_3d::brep::engine::GeometryKind::Face).len())
}

#[test]
fn fixtures_are_refreshed_only_on_request() {
    if support::writing() {
        support::refresh(FIXTURE_PATH, FIXTURE, COMPUTES);
    }
}

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("brep.interchange", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_matches_its_stated_text_shape_and_fault_at_every_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 20);
}

#[test]
fn every_format_round_trips_a_box_through_its_export_and_import_widgets() {
    let input = cube(2.0);
    for (export, import, port, faceted) in [
        ("brep.interchange.exportStep", "brep.interchange.importStep", "step", false),
        ("brep.interchange.exportStl", "brep.interchange.importStl", "stl", true),
        ("brep.interchange.exportObj", "brep.interchange.importObj", "obj", true),
        ("brep.interchange.exportDwg", "brep.interchange.importDwg", "dwg", true),
    ] {
        let mut values = vec![("shape", GeometryValue::Shape(input.clone()))];
        if faceted {
            values.push(("deflection", GeometryValue::Number(0.1)));
        }
        let (_, exported) = run(export, values, 1);
        let text = text_of(&exported, port);
        let mut values = vec![("data", GeometryValue::Text(text))];
        if faceted {
            values.push(("tolerance", GeometryValue::Number(1e-4)));
        }
        let (_, imported) = run(import, values, 1);
        let (volume, faces) = volume_of(&imported);
        assert!((volume - 8.0).abs() < 1e-6, "{export}: volume {volume}");
        assert_eq!(faces, if faceted { 12 } else { 6 }, "{export}: faces");
    }
}

#[test]
fn mesh_formats_are_stepped_jobs_that_yield_before_they_finish() {
    let (slices, exported) = run("brep.interchange.exportStl", vec![("shape", GeometryValue::Shape(cube(2.0))), ("deflection", GeometryValue::Number(0.1))], 1);
    assert!(slices >= 2, "export needed {slices} Working slices at fuel 1");
    let (slices, imported) = run("brep.interchange.importStl", vec![("data", GeometryValue::Text(text_of(&exported, "stl"))), ("tolerance", GeometryValue::Number(1e-4))], 1);
    assert!(slices >= 1, "import needed {slices} Working slices at fuel 1");
    assert!(imported.fault.is_none());
}

#[test]
fn empty_undecodable_and_unparseable_data_are_localized_faults_at_the_data_port() {
    for (id, data, code) in [
        ("brep.interchange.importStep", "", "generation3d.geometry.interchange-empty"),
        ("brep.interchange.importStep", "not a step file", "generation3d.geometry.kernel"),
        ("brep.interchange.importStl", "   ", "generation3d.geometry.interchange-empty"),
        ("brep.interchange.importStl", "%%%", "generation3d.geometry.interchange-decode"),
        ("brep.interchange.importStl", "AAAA", "generation3d.geometry.interchange-parse"),
        ("brep.interchange.importObj", "v 0 0 0\nf 1 2 3", "generation3d.geometry.interchange-parse"),
        ("brep.interchange.importDwg", "AAAA", "generation3d.geometry.interchange-parse"),
    ] {
        let mut values = vec![("data", GeometryValue::Text(data.to_string()))];
        if id != "brep.interchange.importStep" {
            values.push(("tolerance", GeometryValue::Number(1e-4)));
        }
        let (_, evaluation) = run(id, values, 1);
        let fault = evaluation.fault.unwrap_or_else(|| panic!("{id} {data:?}: expected a fault"));
        assert_eq!(fault.code, code, "{id} {data:?}");
        assert!(fault.message.en != fault.message.de && evaluation.outputs.is_empty());
    }
}

#[test]
fn a_shape_without_a_surface_cannot_be_exported_as_a_mesh() {
    let mut session = KernelSession::new();
    let handle = session.brep().vertex_sync([0.0; 3]).expect("vertex");
    let vertex = Arc::new(session.export(&handle).expect("export"));
    let (_, evaluation) = run("brep.interchange.exportObj", vec![("shape", GeometryValue::Shape(vertex)), ("deflection", GeometryValue::Number(0.1))], 1);
    assert_eq!(evaluation.fault.expect("fault").code, "generation3d.geometry.interchange-empty");
}

#[test]
fn a_compound_exports_as_step_with_every_solid() {
    let mut session = KernelSession::new();
    let first = session.brep().box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let second = session.brep().box_prim_sync(2.0, 2.0, 2.0).expect("box");
    let second = session.brep().translate_sync(&second, [5.0, 0.0, 0.0]).expect("move");
    let compound = session.brep().compound_sync(&[first, second]).expect("compound");
    let shape = Arc::new(session.export(&compound).expect("export"));
    let (_, exported) = run("brep.interchange.exportStep", vec![("shape", GeometryValue::Shape(shape))], usize::MAX);
    assert_eq!(text_of(&exported, "step").matches("MANIFOLD_SOLID_BREP").count(), 2);
}
