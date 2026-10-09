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

fn arc_length(chord: f64, bulge: f64) -> f64 {
    let sweep = 4.0 * bulge.atan().abs();
    sweep * chord / (2.0 * (sweep / 2.0).sin())
}

#[semio_framework_async_macros::async_test]
async fn an_arc_beam_sweeps_its_profile_along_the_arc() {
    let mut snapshot = case(CASE).snapshot;
    let (start, end) = (crate::Point2 { x: 0.0, y: 0.0 }, crate::Point2 { x: 6.0, y: 0.0 });
    snapshot.beams.get_mut("b-along-x").expect("beam").axis = crate::Axis::Arc { start, end, bulge: 0.4 };
    let solid = &compute_element_solids(&snapshot)["b-along-x"];
    let wanted = 0.2 * 0.4 * arc_length(6.0, 0.4);
    assert!(((solid.volume - wanted) / wanted).abs() < 5e-3, "{} against {wanted}", solid.volume);
    assert!(solid.bounds.min.y < -0.5 || solid.bounds.max.y > 0.5, "the beam bulges out of the chord");
    assert!(close(solid.bounds.max.z, 3.0 - 0.1, 1e-9), "a level arc beam keeps its top");
}

#[semio_framework_async_macros::async_test]
async fn an_inclined_beam_climbs_linearly_and_keeps_its_section_perpendicular() {
    let mut snapshot = case(CASE).snapshot;
    snapshot.beams.get_mut("b-along-x").expect("beam").end_top_offset = Some(0.4);
    let solid = &compute_element_solids(&snapshot)["b-along-x"];
    let (run, rise) = (6.0_f64, 0.5_f64);
    assert!(close(solid.volume, 0.2 * 0.4 * run.hypot(rise), 1e-9), "profile area times the sloping length");
    assert!(close(solid.bounds.max.z, 3.4, 1e-9), "the top line ends at the storey top plus the end offset");
    assert!(close(solid.bounds.min.z, 2.9 - 0.4 * run / run.hypot(rise), 1e-9), "the section hangs perpendicular to the axis at the low end");
}

fn joined(snapshot: &mut ModelSnapshot, at: [(f64, f64); 2], side: f64) {
    snapshot.column_types.insert("ct-join".into(), crate::ColumnType { name: "Join".into(), profile: Profile::Rectangle { width: side, depth: side }, material: "m-concrete".into() });
    for (index, (x, y)) in at.into_iter().enumerate() {
        snapshot.columns.insert(format!("c-{index}"), crate::Column { storey: "st-ground".into(), column_type: "ct-join".into(), position: crate::Point2 { x, y }, rotation: 0.0, tilt: None, base_offset: 0.0, top: crate::TopConstraint::StoreyTop { offset: 0.0 }, phase: crate::Phase::New, name: String::new() });
    }
}

#[semio_framework_async_macros::async_test]
async fn a_beam_end_inside_a_column_is_cut_back_to_the_face_of_the_column() {
    let mut snapshot = case(CASE).snapshot;
    let plain = compute_element_solids(&snapshot)["b-along-x"].clone();
    joined(&mut snapshot, [(0.0, 0.0), (6.0, 0.0)], 0.4);
    let solids = compute_element_solids(&snapshot);
    let trimmed = &solids["b-along-x"];
    assert!(close(trimmed.volume, 0.2 * 0.4 * (6.0 - 0.2 - 0.2), 1e-9), "half a column off each end");
    assert!(close(trimmed.bounds.min.x, 0.2, 1e-9) && close(trimmed.bounds.max.x, 5.8, 1e-9), "{:?}", trimmed.bounds);
    assert!(close(plain.bounds.max.z, trimmed.bounds.max.z, 1e-12), "the height is untouched");
    assert!(close(solids["b-i"].volume, compute_element_solids(&case(CASE).snapshot)["b-i"].volume, 1e-12), "a beam elsewhere is untouched");
}

#[semio_framework_async_macros::async_test]
async fn a_column_that_does_not_reach_the_beam_does_not_join_and_a_swallowing_join_is_ignored() {
    let mut snapshot = case(CASE).snapshot;
    let plain = compute_element_solids(&snapshot)["b-along-x"].volume;
    joined(&mut snapshot, [(0.0, 3.0), (6.0, 3.0)], 0.4);
    assert!(close(compute_element_solids(&snapshot)["b-along-x"].volume, plain, 1e-12), "columns away from the ends do not join");
    joined(&mut snapshot, [(0.0, 0.0), (6.0, 0.0)], 20.0);
    assert!(close(compute_element_solids(&snapshot)["b-along-x"].volume, plain, 1e-12), "joins that would swallow the whole beam are ignored");
}

#[semio_framework_async_macros::async_test]
async fn a_beam_depends_on_the_storeys_of_the_columns_it_can_join() {
    let mut snapshot = case(CASE).snapshot;
    joined(&mut snapshot, [(0.0, 0.0), (6.0, 0.0)], 0.4);
    snapshot.columns.get_mut("c-1").expect("column").top = crate::TopConstraint::Storey { storey: "st-first".into(), offset: 0.0 };
    let plan = solid_steps(&snapshot, SolidFamily::Beam);
    let parents = plan.iter().find(|step| step.key == ModelNode::Solid(SolidKey::of(SolidFamily::Beam, "b-along-x"))).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents, vec![ModelNode::Storey("st-ground".into()), ModelNode::Storey("st-first".into())]);
    let candidates: Vec<String> = joining(&snapshot, &snapshot.beams["b-along-x"]).into_iter().map(|(id, _)| id.clone()).collect();
    assert_eq!(candidates, ["c-0", "c-1"]);
}
