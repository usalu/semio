use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::compute_stair_runs;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::ModelInference;
use protocol::Inference;

const CASE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🪜️stairs-flights/🔣️.json");

fn geometry(snapshot: &ModelSnapshot, id: &str) -> StairGeometry {
    let levels = compute_storey_levels(snapshot);
    let stair = &snapshot.stairs[id];
    let target = match &stair.top {
        crate::TopConstraint::Storey { storey, .. } => levels.get(storey),
        _ => None,
    };
    stair_geometry(stair, &levels[&stair.storey], target)
}

#[semio_framework_async_macros::async_test]
async fn stairs_reproduce_the_third_party_oracle_table() {
    let case = case(CASE);
    assert_matches_oracle(&case, &compute_element_solids(&case.snapshot), SolidFamily::Stair);
}

#[semio_framework_async_macros::async_test]
async fn the_riser_law_and_the_step_count_agree_with_the_oracle_and_the_stair_runs() {
    let case = case(CASE);
    let runs = compute_stair_runs(&case.snapshot);
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null()) {
        let geometry = geometry(&case.snapshot, id);
        assert_eq!(u64::from(geometry.run.riser_count), row["riser_count"].as_u64().expect("number"), "{id}.riser_count");
        assert!(close(row["riser_height"].as_f64().expect("number"), geometry.run.riser_height, 1e-12), "{id}.riser_height");
        assert!(close(row["tread"].as_f64().expect("number"), geometry.run.tread, 1e-12), "{id}.tread");
        assert_eq!(geometry.steps as u64, row["steps"].as_u64().expect("number"), "{id}.steps");
        assert_eq!(runs[id], geometry.run, "{id}: the solid and the stair-runs field share one run");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_straight_stair_is_a_block_per_tread_with_the_analytic_volume() {
    let solids = compute_element_solids(&case(CASE).snapshot);
    let (count, rise): (f64, f64) = (17.0, 3.0);
    let (riser, tread) = (rise / count, 0.62 - 2.0 * rise / count);
    let blocks: f64 = (1..=16).map(|k| 1.0 * tread * f64::from(k) * riser).sum();
    assert!(close(blocks, solids["st-straight"].volume, 1e-9), "sixteen treads of width 1.0, each a block up to its own level");
    assert!(close(16.0 * riser, solids["st-straight"].bounds.max.z, 1e-12), "the last riser arrives on the upper floor: the top tread is the 16th level");
    assert!(close(16.0 * tread, solids["st-straight"].bounds.max.x, 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn changing_the_storey_height_changes_the_riser_count_and_the_solid() {
    let snapshot = case(CASE).snapshot;
    let (before, after) = (geometry(&snapshot, "st-straight"), geometry(&raised(&snapshot, "st-ground", 0.4), "st-straight"));
    assert_eq!((before.run.riser_count, after.run.riser_count), (17, 19), "ceil(3.0 / 0.18) and ceil(3.4 / 0.18)");
    assert_eq!((before.steps, after.steps), (16, 18));
    let (solids_before, solids_after) = (compute_element_solids(&snapshot), compute_element_solids(&raised(&snapshot, "st-ground", 0.4)));
    assert!(close(3.4 * 18.0 / 19.0, solids_after["st-straight"].bounds.max.z, 1e-12) && close(3.0 * 16.0 / 17.0, solids_before["st-straight"].bounds.max.z, 1e-12));
    assert!(close(solids_after["st-straight-free"].volume, solids_before["st-straight-free"].volume, 1e-12), "an Unconnected stair keeps its rise");
    assert!(close(2.4 - 2.4 / 14.0, solids_after["st-straight-free"].bounds.max.z, 1e-12));
    assert!(solids_after["st-l-right"].bounds.max.z > solids_before["st-l-right"].bounds.max.z, "a stair constrained to the storey above rises with that storey: ground +0.4 lifts the first storey");
}

#[semio_framework_async_macros::async_test]
async fn turning_stairs_have_flights_and_a_landing_at_the_landing_level() {
    let snapshot = case(CASE).snapshot;
    let l_left = geometry(&snapshot, "st-l-left");
    assert_eq!((l_left.run.flights.len(), l_left.run.landings.len()), (2, 1));
    let u_turn = geometry(&snapshot, "st-u");
    assert_eq!((u_turn.run.flights.len(), u_turn.run.landings.len()), (2, 1));
    let steps = steps_of(&u_turn.run);
    let landing = steps.last().expect("the landing is the last prism");
    assert!(close(u_turn.run.landings[0].z, landing.top_z, 1e-12), "the landing prism tops out at the landing height");
    let spiral = geometry(&snapshot, "st-spiral");
    assert!(spiral.run.flights[0].winder.is_some() && spiral.run.landings.is_empty());
    assert_eq!(spiral.steps, 14, "fifteen risers have fourteen wedge treads");
}

#[semio_framework_async_macros::async_test]
async fn a_stair_depends_on_its_storey_and_the_storey_its_top_targets() {
    let snapshot = case(CASE).snapshot;
    let plan = solid_steps(&snapshot, SolidFamily::Stair);
    let parents = |id: &str| plan.iter().find(|step| step.key == ModelNode::Solid(SolidKey::of(SolidFamily::Stair, id))).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("st-straight"), vec![ModelNode::Storey("st-ground".into()), ModelNode::StairRun("st-straight".into())]);
    assert_eq!(parents("st-l-right"), vec![ModelNode::Storey("st-ground".into()), ModelNode::StairRun("st-l-right".into())]);
}

#[semio_framework_async_macros::async_test]
async fn stairs_without_a_tread_are_absent_and_the_result_is_deterministic() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(["st-one-riser", "st-no-rise"].iter().all(|id| !inferred.element_solids.contains_key(*id)));
    assert_eq!(inferred.element_solids["st-straight"].groups.iter().map(|g| g.part.as_str()).collect::<Vec<_>>(), vec![parts::STEP]);
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Stair).is_empty());
}
