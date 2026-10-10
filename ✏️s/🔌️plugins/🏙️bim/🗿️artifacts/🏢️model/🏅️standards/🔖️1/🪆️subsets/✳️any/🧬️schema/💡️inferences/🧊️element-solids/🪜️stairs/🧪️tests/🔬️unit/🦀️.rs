use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::testing::{assert_matches_oracle, case, close, group_extent, raised};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidKey;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{plan::solid_steps, ModelInferenceSession, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::compute_stair_runs;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::{Entry, ModelDiff, ModelInference, ModelSnapshot, StairPatch};
use protocol::Inference;
use std::collections::BTreeMap;

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

fn volumes(solid: &ElementSolid) -> BTreeMap<String, f64> {
    let mut found = BTreeMap::new();
    for (index, group) in solid.groups.iter().enumerate() {
        *found.entry(group.part.clone()).or_insert(0.0) += group_extent(solid, index as u32).2;
    }
    found
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
async fn every_part_of_every_stair_has_the_volume_the_oracle_derives() {
    let case = case(CASE);
    let solids = compute_element_solids(&case.snapshot);
    for (id, row) in case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null()) {
        let found = volumes(&solids[id]);
        let expected = row["parts"].as_object().expect("parts");
        assert_eq!(found.keys().collect::<Vec<_>>(), expected.keys().collect::<Vec<_>>(), "{id}: the parts");
        for (part, volume) in expected {
            let tolerance = row["volume_tolerance"].as_f64().expect("number");
            assert!(close(volume.as_f64().expect("number"), found[part], tolerance), "{id}.{part}: oracle {volume}, subject {}", found[part]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_straight_stair_is_a_slab_per_tread_and_a_board_per_closed_riser() {
    let snapshot = case(CASE).snapshot;
    let solids = compute_element_solids(&snapshot);
    let run = geometry(&snapshot, "st-straight").run;
    let (going, rise, thickness, width) = (run.tread, run.riser_height, 0.04, 1.0);
    let found = volumes(&solids["st-straight"]);
    assert!(close(16.0 * going * width * thickness, found[parts::STEP], 1e-12), "sixteen tread slabs of tread x width x thickness");
    assert!(close(16.0 * (rise - thickness) * width * thickness, found[parts::RISER], 1e-12), "sixteen riser boards of (riser - thickness) x width x thickness");
    assert!(close(16.0 * rise, solids["st-straight"].bounds.max.z, 1e-12), "the last riser arrives on the upper floor: the top tread is the 16th level");
    assert!(close(16.0 * going, solids["st-straight"].bounds.max.x, 1e-12));
    assert_eq!(solids["st-straight"].groups.iter().map(|g| g.part.as_str()).collect::<Vec<_>>(), vec![parts::STEP, parts::RISER], "no stringer, no landing");
}

#[semio_framework_async_macros::async_test]
async fn nosing_extends_every_tread_against_the_travel_and_open_risers_leave_the_boards_out() {
    let snapshot = case(CASE).snapshot;
    let solids = compute_element_solids(&snapshot);
    let (closed, open) = (&solids["st-closed-stringer"], &solids["st-open-stringer"]);
    let run = geometry(&snapshot, "st-closed-stringer").run;
    assert!(close(16.0 * (run.tread + 0.03) * 1.0 * 0.04, volumes(closed)[parts::STEP], 1e-12), "every slab is `tread + nosing` deep");
    assert!(close(-0.03, closed.bounds.min.x, 1e-12), "the first nosing overhangs the foot of the flight");
    assert!(close(17.0 * run.riser_height, closed.bounds.max.z, 1e-9), "a closed stringer runs up to the arrival level");
    assert!(close(1.12, closed.bounds.max.y - closed.bounds.min.y, 1e-12), "closed stringers stand outside the treads: width + 2 x 0.06");
    assert!(open.groups.iter().all(|group| group.part != parts::RISER), "open risers have no board");
    assert!(close(1.0, open.bounds.max.y - open.bounds.min.y, 1e-12), "open stringers stand inside the width of the treads");
    assert!(solids["st-open-stringer-closed-risers"].groups.iter().any(|group| group.part == parts::RISER));
}

#[semio_framework_async_macros::async_test]
async fn the_stringer_band_has_the_analytic_area_of_each_kind() {
    let flight = |closed| Flight { going: 0.25, rise: 0.2, thickness: 0.04, nosing: 0.0, treads: 3, closed };
    let area = |kind, closed| area_of(&flight(closed).profile(kind, 0.3)).abs();
    assert!(close(0.21875, area(StringerKind::Closed, true), 1e-12), "the band under the pitch line, cut by the floor: 0.375 - 0.15625");
    assert!(close(0.15375, area(StringerKind::Mono, true), 1e-12), "the band under the line through the back lower corners: 0.196 - 0.04225");
    assert!(close(0.22775, area(StringerKind::Open, false), 1e-12), "the band under the saw tooth: 0.27 - 0.04225");
    assert!(flight(true).profile(StringerKind::None, 0.3).is_empty());
    assert!(Flight { treads: 0, ..flight(true) }.profile(StringerKind::Closed, 0.3).is_empty(), "no tread, no stringer");
    let boards = |kind, stringer, width| Flight::boards(kind, stringer, width).into_iter().flat_map(|(from, to)| [from, to]).collect::<Vec<f64>>();
    let same = |found: Vec<f64>, wanted: &[f64]| found.len() == wanted.len() && found.iter().zip(wanted).all(|(a, b)| close(*b, *a, 1e-12));
    assert!(same(boards(StringerKind::Closed, 0.06, 1.0), &[0.5, 0.56, -0.56, -0.5]), "outside the treads");
    assert!(same(boards(StringerKind::Open, 0.05, 1.0), &[0.45, 0.5, -0.5, -0.45]), "inside the treads");
    assert!(same(boards(StringerKind::Mono, 0.12, 1.2), &[-0.06, 0.06]), "one board in the middle");
    assert!(Flight::boards(StringerKind::None, 0.12, 1.2).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_plan_footprints_of_the_stringers_follow_the_boards_of_their_flights() {
    let snapshot = case(CASE).snapshot;
    let found = |id: &str| {
        let geometry = geometry(&snapshot, id);
        stringer_footprints(&snapshot.stairs[id], &geometry.run)
    };
    assert!(found("st-straight").is_empty());
    let closed = found("st-closed-stringer");
    assert_eq!(closed.len(), 2, "two boards on one straight flight");
    let xs: Vec<f64> = closed[0].1.iter().map(|corner| corner.x).collect();
    assert!(close(-0.03, xs.iter().copied().fold(f64::INFINITY, f64::min), 1e-12), "a closed stringer starts at the first nosing");
    assert_eq!(found("st-l-closed-deep").len(), 4, "two boards on each of the two flights");
    assert_eq!(found("st-mono-stringer").len(), 1);
    assert!(found("st-spiral-stringer").is_empty(), "a winding flight has no stringer");
}

#[semio_framework_async_macros::async_test]
async fn the_stringer_of_a_spiral_stair_is_ignored_and_says_so() {
    let snapshot = case(CASE).snapshot;
    assert!(stringer_ignored(&snapshot.stairs["st-spiral-stringer"]));
    assert!(["st-spiral", "st-straight", "st-closed-stringer", "st-u-open-deep"].iter().all(|id| !stringer_ignored(&snapshot.stairs[*id])));
    let solids = compute_element_solids(&snapshot);
    assert!(close(volumes(&solids["st-spiral"])[parts::STEP], volumes(&solids["st-spiral-stringer"])[parts::STEP], 1e-12), "the stringer adds nothing to a winding flight");
    assert!(solids["st-spiral-stringer"].groups.iter().all(|group| group.part == parts::STEP));
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let flagged: Vec<&str> = inferred.diagnostics.iter().filter(|row| row.code.slug() == "stair.stringer-ignored-on-spiral").flat_map(|row| row.elements.iter().map(String::as_str)).collect();
    assert_eq!(flagged, vec!["st-spiral-stringer"], "exactly the spiral with a stringer is flagged");
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
async fn turning_stairs_have_flights_and_a_landing_slab_at_the_landing_level() {
    let snapshot = case(CASE).snapshot;
    for id in ["st-l-left", "st-u", "st-l-closed-deep", "st-u-open-deep"] {
        let geometry = geometry(&snapshot, id);
        assert_eq!((geometry.run.flights.len(), geometry.run.landings.len()), (2, 1), "{id}");
        let parts = parts_of(&snapshot.stairs[id], &geometry.run);
        let (low, high) = parts.landings.bounds().expect("the landing slab");
        assert!(close(geometry.run.landings[0].z, high[2], 1e-12) && close(geometry.run.landings[0].z - snapshot.stairs[id].tread_thickness, low[2], 1e-12), "{id}: the slab is one tread thick and tops out at the landing height");
        let landing = geometry.run.landings[0];
        assert!(close(landing.depth, snapshot.stairs[id].landing_depth, 1e-12), "{id}: the landing is as deep as authored");
        assert!(close(landing.depth * landing.width * snapshot.stairs[id].tread_thickness, solid_volume(&parts.landings), 1e-12), "{id}: the landing slab volume");
    }
    let spiral = geometry(&snapshot, "st-spiral");
    assert!(spiral.run.flights[0].winder.is_some() && spiral.run.landings.is_empty());
    assert_eq!(spiral.steps, 14, "fifteen risers have fourteen wedge treads");
}

fn solid_volume(mesh: &TriMesh) -> f64 {
    mesh.signed_volume()
}

#[semio_framework_async_macros::async_test]
async fn a_deeper_landing_pushes_the_second_flight_and_grows_the_slab_only() {
    let mut deeper = case(CASE).snapshot;
    let base = compute_element_solids(&deeper);
    deeper.stairs.get_mut("st-l-left").expect("stair").landing_depth = 1.8;
    let after = compute_element_solids(&deeper);
    let (before_parts, after_parts) = (volumes(&base["st-l-left"]), volumes(&after["st-l-left"]));
    assert!(close(1.8 * 1.0 * 0.04, after_parts[parts::LANDING], 1e-12) && close(1.0 * 1.0 * 0.04, before_parts[parts::LANDING], 1e-12));
    assert!(close(before_parts[parts::STEP], after_parts[parts::STEP], 1e-12) && close(before_parts[parts::RISER], after_parts[parts::RISER], 1e-12), "the treads and risers keep their size");
    assert!(close(0.8, after["st-l-left"].bounds.max.x - base["st-l-left"].bounds.max.x, 1e-9), "the second flight starts 0.8 further along the first");
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
async fn the_construction_of_a_stair_is_part_of_its_dependency() {
    let snapshot = case(CASE).snapshot;
    let stair = snapshot.stairs["st-straight"].clone();
    let base = dependency(&stair);
    let changed = |edit: &dyn Fn(&mut Stair)| {
        let mut other = stair.clone();
        edit(&mut other);
        dependency(&other)
    };
    assert_ne!(base, changed(&|row| row.stringer.kind = StringerKind::Closed));
    assert_ne!(base, changed(&|row| row.stringer.depth = 0.3));
    assert_ne!(base, changed(&|row| row.nosing = 0.02));
    assert_ne!(base, changed(&|row| row.tread_thickness = 0.05));
    assert_ne!(base, changed(&|row| row.riser = RiserKind::Open));
    assert_eq!(base, changed(&|row| row.landing_depth = 2.0), "the landing depth is read through the run, which is a parent");
    assert_eq!(base, changed(&|row| row.name = "Renamed".into()));
}

#[test]
fn editing_the_construction_or_the_landing_of_a_stair_recomputes_it_and_equals_a_fresh_inference() {
    let snapshot = case(CASE).snapshot;
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let edits = [
        ModelDiff::stairs("st-straight", Entry::Patched(StairPatch { nosing: Some(0.02), ..Default::default() })),
        ModelDiff::stairs("st-straight", Entry::Patched(StairPatch { stringer: Some(crate::StairStringer { kind: StringerKind::Mono, width: 0.1, depth: 0.2 }), ..Default::default() })),
        ModelDiff::stairs("st-l-left", Entry::Patched(StairPatch { landing_depth: Some(1.7), ..Default::default() })),
    ];
    let mut current = snapshot;
    for edit in edits {
        current = protocol::apply_diff(&edit, &current).expect("applies");
        let incremental = session.update(&current, &edit).clone();
        let report = session.report().clone();
        assert!(!report.gated && report.computed_by_kind.get("solid").copied().unwrap_or(0) >= 1, "the solid of the stair is recomputed: {report:?}");
        assert_eq!(report.computed_by_kind.get("wall-layout"), None, "no wall is touched");
        assert_eq!(incremental, ModelInference::infer(&current).expect("infers"), "the incremental result equals a fresh inference");
    }
}

#[semio_framework_async_macros::async_test]
async fn stairs_without_a_tread_are_absent_and_the_result_is_deterministic() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred, ModelInference::infer(&snapshot).expect("infers"));
    assert!(["st-one-riser", "st-no-rise"].iter().all(|id| !inferred.element_solids.contains_key(*id)));
    assert_eq!(inferred.element_solids["st-straight"].groups.iter().map(|g| g.part.as_str()).collect::<Vec<_>>(), vec![parts::STEP, parts::RISER]);
    assert!(solid_steps(&ModelSnapshot::default(), SolidFamily::Stair).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_volume_of_a_stair_in_the_take_off_is_the_sum_of_its_parts() {
    let snapshot = case(CASE).snapshot;
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    for id in ["st-straight", "st-closed-stringer", "st-l-closed-deep", "st-u-open-deep", "st-mono-stringer", "st-spiral-stringer"] {
        let summed: f64 = volumes(&inferred.element_solids[id]).values().sum();
        let quantity = &inferred.quantities.elements[id];
        assert!(close(summed, quantity.net_volume, 1e-9) && close(summed, quantity.gross_volume, 1e-9), "{id}: treads + risers + stringers + landings");
        assert_eq!(quantity.risers, inferred.stair_runs[id].riser_count);
    }
}
