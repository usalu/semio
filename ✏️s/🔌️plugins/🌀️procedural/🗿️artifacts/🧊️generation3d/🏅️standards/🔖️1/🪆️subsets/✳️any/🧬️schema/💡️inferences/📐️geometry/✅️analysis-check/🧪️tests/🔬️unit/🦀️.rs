use super::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

fn start_of(id: &str) -> StartFn {
    COMPUTES.iter().find(|entry| entry.id == id).expect("registered").start
}

fn case_named(name: &str) -> Value {
    support::cases(FIXTURE).1.into_iter().find(|case| case["name"] == name).unwrap_or_else(|| panic!("fixture case {name}"))
}

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("analysis.check", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 28);
}

#[test]
fn the_winding_census_agrees_with_the_kernels_consistency_flag_on_every_fixture_mesh() {
    let meshes: Vec<Value> = support::cases(FIXTURE).1.iter().filter_map(|case| case["inputs"].get("mesh").cloned()).filter(|mesh| !mesh["faces"].as_array().expect("faces").is_empty()).collect();
    assert!(meshes.len() >= 5);
    for recipe in meshes {
        let mesh = support::build_mesh(&recipe);
        assert_eq!(inconsistent_edges(&mesh.polygons()) == 0, mesh.quality_report().consistent_winding, "{recipe}");
    }
}

#[test]
fn a_boolean_that_needs_the_kernels_job_is_advanced_in_slices_and_equals_the_whole_run() {
    let case = case_named("a cylinder and a box interfere in a circular segment");
    let kind = support::kind_of("analysis.interference");
    let mut job = start_of("analysis.interference")(kind, support::inputs_of(kind, &case));
    let mut slices = 0;
    let stepped = loop {
        match job.step(1) {
            WidgetStep::Working { progress } => {
                assert!((0.0..1.0).contains(&progress));
                slices += 1;
            }
            WidgetStep::Done(evaluation) => break evaluation,
        }
    };
    assert!(slices >= 2, "the interference of a cylinder and a box takes several one-unit slices, took {slices}");
    assert_eq!(stepped, support::evaluate(kind, support::inputs_of(kind, &case), COMPUTES, usize::MAX));
}

#[test]
fn cancelling_an_interference_job_mid_run_ends_it_with_a_cancelled_fault() {
    let case = case_named("a cylinder and a box interfere in a circular segment");
    let kind = support::kind_of("analysis.interference");
    let mut job = start_of("analysis.interference")(kind, support::inputs_of(kind, &case));
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    job.cancel();
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("a cancelled job answers Done") };
    assert_eq!(evaluation.fault.expect("a fault").code, "generation3d.geometry.cancelled");
    assert!(evaluation.outputs.is_empty());
}

#[test]
fn an_empty_intersection_is_an_answer_not_a_fault_and_has_no_shape() {
    let case = case_named("separated boxes do not interfere");
    let kind = support::kind_of("analysis.interference");
    let evaluation = support::evaluate(kind, support::inputs_of(kind, &case), COMPUTES, usize::MAX);
    assert!(evaluation.fault.is_none());
    assert_eq!(evaluation.outputs.keys().map(String::as_str).collect::<Vec<_>>(), ["interferes", "volume"]);
}

#[test]
fn a_report_lists_every_kernel_finding_before_the_verdict() {
    let shape = support::build_shape(&json!({ "recipe": "box", "size": [1, 1, 1] }));
    let scope = scope_of(&shape, "shape").expect("scope");
    let mut report = validity(&shape.body, &scope).expect("validity");
    let manifold = manifold_report(&shape.body, &scope).expect("manifold");
    report.issues.push(semio_framework_3d::brep::queries::analysis::ValidityIssue { entity: "face:7".into(), code: "warning-sliver".into(), message: "thin".into(), severity: IssueSeverity::Warning });
    assert_eq!(describe(&report, &manifold), "warning warning-sliver face:7: thin\nverdict watertight");
}

#[test]
fn a_non_mesh_input_is_a_fault_at_that_port() {
    let kind = support::kind_of("analysis.meshQuality");
    let values = BTreeMap::from([("mesh".to_string(), GeometryValue::Number(1.0))]);
    let fault = support::drive(start_of("analysis.meshQuality")(kind, WidgetInputs::new("w", kind, values)), 1).fault.expect("a fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-type", Some("mesh")));
}
