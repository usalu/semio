use super::*;
use serde_json::json;
use std::collections::BTreeMap;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

fn start_of(id: &str) -> StartFn {
    COMPUTES.iter().find(|entry| entry.id == id).expect("registered").start
}

fn pair_inputs(kind: &Kind) -> WidgetInputs {
    let pair = json!({ "recipe": "compound", "of": [{ "recipe": "box", "size": [1, 1, 1] }, { "recipe": "translate", "of": { "recipe": "box", "size": [2, 1, 1] }, "by": [3, 0, 0] }] });
    WidgetInputs::new("w", kind, BTreeMap::from([("solid".to_string(), GeometryValue::shape(support::build_shape(&pair)))]))
}

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("analysis.measure", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 60);
}

#[test]
fn a_compound_is_measured_one_solid_per_unit_of_fuel() {
    let kind = support::kind_of("analysis.volume");
    let mut job = start_of("analysis.volume")(kind, pair_inputs(kind));
    assert!(matches!(job.step(1), WidgetStep::Working { progress } if (progress - 0.5).abs() < 1e-6));
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("the second unit concludes") };
    assert_eq!(evaluation.outputs.get("volume"), Some(&GeometryValue::Number(3.0)));
    let mut whole = start_of("analysis.volume")(kind, pair_inputs(kind));
    assert!(matches!(whole.step(usize::MAX), WidgetStep::Done(_)));
}

#[test]
fn a_cancelled_job_ends_with_a_localized_cancelled_fault_and_stays_ended() {
    let kind = support::kind_of("analysis.volume");
    let mut job = start_of("analysis.volume")(kind, pair_inputs(kind));
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    job.cancel();
    for _ in 0..2 {
        let WidgetStep::Done(evaluation) = job.step(1) else { panic!("a cancelled job answers Done") };
        let fault = evaluation.fault.expect("a fault");
        assert_eq!(fault.code, "generation3d.geometry.cancelled");
        assert!(!fault.message.en.is_empty() && fault.message.en != fault.message.de);
        assert!(evaluation.outputs.is_empty());
    }
}

#[test]
fn a_non_shape_input_is_a_fault_at_that_port() {
    let kind = support::kind_of("analysis.area");
    let values = BTreeMap::from([("shape".to_string(), GeometryValue::Number(1.0))]);
    let fault = support::drive(start_of("analysis.area")(kind, WidgetInputs::new("w", kind, values)), 1).fault.expect("a fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-type", Some("shape")));
}
