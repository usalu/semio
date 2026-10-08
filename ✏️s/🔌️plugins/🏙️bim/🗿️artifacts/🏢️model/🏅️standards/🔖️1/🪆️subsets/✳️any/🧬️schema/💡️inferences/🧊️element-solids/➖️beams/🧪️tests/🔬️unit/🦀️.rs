use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, parts, SolidFamily};
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/➖️beams-profiles/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn beams_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Beam);
}

#[semio_framework_async_macros::async_test]
async fn beams_have_the_analytic_volumes_of_profile_area_times_length() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    assert!(close(0.2 * 0.4 * 6.0, solids["b-along-x"].volume, 1e-12));
    assert!(close(0.2 * 0.4 * 5.0, solids["b-diagonal"].volume, 1e-12), "a 3-4-5 diagonal beam");
    assert!(close((2.0 * 0.15 * 0.012 + 0.008 * (0.3 - 0.024)) * 6.0, solids["b-i"].volume, 1e-12));
    let round = std::f64::consts::PI * 0.05f64.powi(2) * 4.0;
    assert!((solids["b-pipe"].volume - round).abs() <= round * 3e-3 && solids["b-pipe"].volume < round, "a pipe, inscribed within the sagitta bound of a small radius");
}

#[semio_framework_async_macros::async_test]
async fn a_beam_hangs_from_the_storey_top_plus_its_offset() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let along = &solids["b-along-x"];
    assert!(close(along.bounds.max.z, 3.0 - 0.1, 1e-12) && close(along.bounds.min.z, 3.0 - 0.1 - 0.4, 1e-12), "profile top at storey top - 0.1, depth 0.4");
    assert!(close(along.bounds.min.y, -0.1, 1e-12) && close(along.bounds.max.y, 0.1, 1e-12), "width 0.2 centred on the axis");
    assert!(close(solids["b-diagonal"].bounds.max.z, 5.8, 1e-12), "on the first storey: 3.0 + 2.8");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_lifts_the_beams_of_that_and_the_storeys_above() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-ground", 0.4)));
    for id in ["b-along-x", "b-i", "b-pipe", "b-diagonal"] {
        assert!(close(after[id].bounds.max.z - before[id].bounds.max.z, 0.4, 1e-9), "{id} follows the top of its storey or the storey below");
        assert!(close(after[id].volume, before[id].volume, 1e-12), "{id} keeps its size");
    }
    let lowered = compute_element_solids(&raised(&snapshot, "st-first", -0.4));
    assert!(close(lowered["b-diagonal"].bounds.max.z - before["b-diagonal"].bounds.max.z, -0.4, 1e-9));
    assert!(close(lowered["b-along-x"].bounds.max.z, before["b-along-x"].bounds.max.z, 1e-12), "a beam below is untouched");
}

#[semio_framework_async_macros::async_test]
async fn the_section_is_dropped_so_its_highest_point_touches_the_reference_line() {
    let section = section_of(&Profile::Rectangle { width: 0.2, depth: 0.4 });
    let (top, bottom) = (section.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max), section.iter().map(|p| p.y).fold(f64::INFINITY, f64::min));
    assert!(close(top, 0.0, 1e-12) && close(bottom, -0.4, 1e-12));
    assert!(section_of(&Profile::Rectangle { width: 0.0, depth: 0.4 }).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn beams_depend_on_their_storey_only_and_are_deterministic() {
    let snapshot = case(CASE).snapshot;
    let plan = solid_steps(&snapshot, SolidFamily::Beam);
    assert!(plan.iter().all(|step| step.parents.len() == 1));
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(["b-zero", "b-unknown-type"].iter().all(|id| !inferred.element_solids.contains_key(*id)));
    assert_eq!(inferred.element_solids["b-i"].groups[0].material, "m-steel");
    assert_eq!(inferred.element_solids["b-i"].groups[0].part, parts::BODY);
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Beam).is_empty());
}
