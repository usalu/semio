use crate::standards::v1::subsets::any::io::text::inferences::stair_runs::table_json;
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{apply_model_mutation, set_storey_height::SetStoreyHeight};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelInferenceSession, ModelNode};
use crate::{ModelMutation, TopConstraint};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn stair(flight: StairFlight, top: TopConstraint) -> Stair {
    Stair { phase: crate::Phase::New, storey: "st-ground".into(), start: Point2 { x: 1.0, y: 2.0 }, direction: 0.0, width: 1.0, flight, top, max_riser: 0.1875, min_tread: 0.25, stringer: crate::STANDARD_STRINGER, nosing: 0.0, tread_thickness: crate::STANDARD_TREAD_THICKNESS, riser: crate::STANDARD_RISER, landing_depth: 1.0, name: "Stair".into() }
}

fn with(stair: Stair) -> ModelSnapshot {
    let mut snapshot: ModelSnapshot = from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes");
    snapshot.stairs.insert("s-1".into(), stair);
    snapshot
}

fn run_of_house(stair: Stair) -> StairRun {
    compute_stair_runs(&with(stair))["s-1"].clone()
}

fn raised(snapshot: &ModelSnapshot, storey: &str, height: f64) -> ModelSnapshot {
    apply_model_mutation(snapshot, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: storey.into(), height })).expect("the height edit applies")
}

#[semio_framework_async_macros::async_test]
async fn a_straight_stair_splits_the_storey_height_into_equal_risers() {
    let run = run_of_house(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    assert!(close(run.rise, 3.0) && close(run.base_z, 0.0) && close(run.top_z, 3.0));
    assert_eq!(run.riser_count, 16);
    assert!(close(run.riser_height, 0.1875));
    assert!(close(run.tread, 0.25), "the Blondel tread 0.245 is raised to the minimum tread 0.25");
    assert_eq!(run.tread_count, 15);
    assert!(close(run.stride, 0.625) && close(run.run_length, 3.75));
    assert!(run.compliance.compliant && run.landings.is_empty());
    assert_eq!(run.flights.len(), 1);
    let flight = &run.flights[0];
    assert_eq!((flight.first_riser, flight.risers, flight.treads), (1, 16, 15));
    assert!(close(flight.start.x, 1.0) && close(flight.start.y, 2.0) && close(flight.base_z, 0.0) && close(flight.length, 3.75));
}

#[semio_framework_async_macros::async_test]
async fn the_riser_count_follows_the_storey_height() {
    let base = with(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    for (height, count) in [(3.0, 16), (3.4, 19), (2.0, 11), (0.1875, 1), (3.1, 17)] {
        let run = compute_stair_runs(&raised(&base, "st-ground", height))["s-1"].clone();
        assert_eq!(run.riser_count, count, "storey height {height}");
        assert!(close(run.riser_height * f64::from(count), height), "equal risers add up to the rise");
        assert!(run.riser_height <= 0.1875 + 1e-12);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_stair_constrained_to_another_storey_follows_that_storey() {
    let base = with(stair(StairFlight::Straight, TopConstraint::Storey { storey: "st-roof".into(), offset: -0.5 }));
    let before = compute_stair_runs(&base)["s-1"].clone();
    assert!(close(before.rise, 5.3) && close(before.top_z, 5.3));
    let after = compute_stair_runs(&raised(&base, "st-first", 3.2))["s-1"].clone();
    assert!(close(after.rise - before.rise, 0.4), "raising an intermediate storey lifts the target");
    assert!(after.riser_count > before.riser_count);
    let unaffected = compute_stair_runs(&raised(&base, "st-roof", 0.9))["s-1"].clone();
    assert_eq!(unaffected, before, "the height of the target storey itself does not move its elevation");
}

#[semio_framework_async_macros::async_test]
async fn an_unconnected_stair_keeps_its_rise() {
    let base = with(stair(StairFlight::Straight, TopConstraint::Unconnected { height: 2.4 }));
    let before = compute_stair_runs(&base)["s-1"].clone();
    assert!(close(before.rise, 2.4) && before.riser_count == 13);
    assert_eq!(compute_stair_runs(&raised(&base, "st-ground", 3.6))["s-1"], before);
}

#[semio_framework_async_macros::async_test]
async fn a_quarter_turn_stair_has_two_flights_around_a_square_landing() {
    let run = run_of_house(stair(StairFlight::LTurn { split: 0.5, turn: Turn::Left }, TopConstraint::StoreyTop { offset: 0.0 }));
    assert_eq!(run.riser_count, 16);
    let (first, second) = (&run.flights[0], &run.flights[1]);
    assert_eq!((first.risers, first.treads, second.risers, second.treads, second.first_riser), (8, 7, 8, 7, 9));
    assert_eq!(run.tread_count, first.treads + second.treads + 1, "the landing replaces one tread");
    assert!(close(run.run_length, 14.0 * 0.25));
    let landing = run.landings[0];
    assert!(close(landing.z, 8.0 * 0.1875) && close(second.base_z, landing.z));
    assert!(close(landing.width, 1.0) && close(landing.depth, 1.0) && landing.after_flight == 0);
    assert!(close(landing.centre.x, 1.0 + 7.0 * 0.25 + 0.5) && close(landing.centre.y, 2.0));
    assert!(close(second.start.x, landing.centre.x) && close(second.start.y, 2.5), "the second flight starts on the left edge of the landing");
    assert!(close(second.direction, std::f64::consts::FRAC_PI_2));
    let right = run_of_house(stair(StairFlight::LTurn { split: 0.25, turn: Turn::Right }, TopConstraint::StoreyTop { offset: 0.0 }));
    assert_eq!((right.flights[0].risers, right.flights[1].risers), (4, 12));
    assert!(close(right.flights[1].direction, -std::f64::consts::FRAC_PI_2) && close(right.flights[1].start.y, 1.5));
}

#[semio_framework_async_macros::async_test]
async fn a_half_turn_stair_returns_beside_the_first_flight() {
    let run = run_of_house(stair(StairFlight::UTurn { gap: 0.2 }, TopConstraint::StoreyTop { offset: 0.0 }));
    assert_eq!((run.flights[0].risers, run.flights[1].risers), (8, 8));
    let (landing, second) = (run.landings[0], &run.flights[1]);
    assert!(close(landing.width, 2.2) && close(landing.depth, 1.0));
    assert!(close(second.start.x, 1.0 + 7.0 * 0.25) && close(second.start.y, 2.0 + 1.2));
    assert!(close(second.direction, std::f64::consts::PI));
    assert!(close(landing.centre.y, 2.0 + 0.6) && close(landing.centre.x, 1.0 + 7.0 * 0.25 + 0.5));
}

#[semio_framework_async_macros::async_test]
async fn a_spiral_stair_winds_its_treads_around_the_walking_line() {
    let run = run_of_house(stair(StairFlight::Spiral { radius: 1.5, sweep: 2.0 * std::f64::consts::PI }, TopConstraint::StoreyTop { offset: 0.0 }));
    let winder = run.flights[0].winder.expect("a winder");
    assert!(close(winder.outer_radius, 1.5) && close(winder.inner_radius, 0.5) && close(winder.centre.x, 1.0) && close(winder.centre.y, 3.0));
    assert!(close(winder.start_angle, -std::f64::consts::FRAC_PI_2));
    assert!(close(run.tread, 2.0 * std::f64::consts::PI * 1.0 / 15.0), "arc length per tread on the walking line of radius 1");
    assert!(close(run.run_length, 2.0 * std::f64::consts::PI) && run.landings.is_empty());
    let clockwise = run_of_house(stair(StairFlight::Spiral { radius: 1.5, sweep: -std::f64::consts::PI }, TopConstraint::StoreyTop { offset: 0.0 }));
    assert!(close(clockwise.flights[0].winder.expect("a winder").centre.y, 1.0));
}

#[semio_framework_async_macros::async_test]
async fn the_code_flags_report_each_rule_separately() {
    let steep = run_of_house(Stair { max_riser: 0.25, ..stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }) });
    assert_eq!(steep.riser_count, 12);
    assert!(steep.compliance.riser_ok && steep.compliance.tread_ok && !steep.compliance.blondel_ok && !steep.compliance.compliant, "2R+T = 0.75 breaks Blondel");
    let shallow = run_of_house(Stair { min_tread: 0.4, ..stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }) });
    assert!(close(shallow.tread, 0.4) && shallow.compliance.tread_ok && !shallow.compliance.blondel_ok);
    let comfortable = run_of_house(Stair { max_riser: 0.17, min_tread: 0.2, ..stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }) });
    assert!(close(comfortable.tread, 0.62 - 2.0 * comfortable.riser_height) && comfortable.compliance.compliant);
}

#[semio_framework_async_macros::async_test]
async fn degenerate_stairs_are_reported_not_hidden() {
    let flat = run_of_house(stair(StairFlight::Straight, TopConstraint::Unconnected { height: 0.0 }));
    assert_eq!((flat.riser_count, flat.flights.len()), (0, 0));
    assert!(!flat.compliance.rise_positive && !flat.compliance.compliant);
    let unlimited = run_of_house(Stair { max_riser: 0.0, ..stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }) });
    assert_eq!(unlimited.riser_count, 0);
    let capped = run_of_house(Stair { max_riser: 1e-4, ..stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }) });
    assert_eq!(capped.riser_count, MAX_RISERS);
    assert!(!capped.compliance.riser_ok);
    let single = run_of_house(stair(StairFlight::UTurn { gap: 0.1 }, TopConstraint::Unconnected { height: 0.15 }));
    assert_eq!((single.riser_count, single.flights.len(), single.landings.len()), (1, 1, 0), "one riser cannot turn");
    assert!(single.compliance.compliant);
}

#[semio_framework_async_macros::async_test]
async fn a_stair_on_a_missing_storey_is_not_planned() {
    let mut snapshot = with(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    snapshot.stairs.get_mut("s-1").expect("stair").storey = "st-nowhere".into();
    assert!(compute_stair_runs(&snapshot).is_empty());
}

const FLIGHTS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪜️stair-runs/🪜️flights/📸️snapshot/🔣️.json");
const FLIGHTS_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪜️stair-runs/🪜️flights/💡️inference/🪜️stair-runs/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_table() {
    let snapshot: ModelSnapshot = from_json_str(FLIGHTS, JsonMemberPolicy::Reject).expect("the flights model decodes");
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(FLIGHTS_TABLE, &table_json(&compute_stair_runs(&snapshot)));
    assert!(problems.is_empty(), "{} disagreements with the shapely oracle: {:?}", problems.len(), &problems[..problems.len().min(12)]);
}

#[semio_framework_async_macros::async_test]
async fn the_model_inference_carries_the_runs() {
    use protocol::Inference;
    let snapshot = with(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred.stair_runs, compute_stair_runs(&snapshot));
    assert_eq!(inferred, crate::ModelInference::infer(&snapshot).expect("infers"), "determinism");
    assert!(compute_stair_runs(&ModelSnapshot::default()).is_empty(), "default");
}


#[semio_framework_async_macros::async_test]
async fn a_stair_depends_on_its_storey_and_the_storey_its_top_targets() {
    let snapshot = with(stair(StairFlight::Straight, TopConstraint::Storey { storey: "st-first".into(), offset: 0.0 }));
    let steps = plan::build(&snapshot, kinds::closure(kinds::RUNS));
    let step = steps.iter().find(|step| step.key == ModelNode::StairRun("s-1".into())).expect("planned");
    assert_eq!(step.parents, vec![ModelNode::Storey("st-ground".into()), ModelNode::Storey("st-first".into())]);
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    for step in &steps {
        for parent in &step.parents {
            assert!(position(parent) < position(&step.key), "parents come first");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_storey_diff_re_infers_the_stairs_on_it_and_a_material_diff_does_not() {
    use crate::{Entry, MaterialPatch, ModelDiff, StoreyPatch};
    let snapshot = with(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).stair_runs.clone();
    let paint = ModelDiff::materials("m-brick", Entry::Patched(MaterialPatch { density: Some(1900.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &paint).stair_runs.clone();
    assert_eq!(first, untouched, "a material edit leaves the stair runs alone");
    assert_eq!(session.report().computed_by_kind.get("stair-run"), None, "and recomputes no run");
    let height = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).stair_runs.clone();
    assert_eq!((first["s-1"].riser_count, recomputed["s-1"].riser_count), (16, 19), "a storey height edit re-infers the riser count");
}

#[semio_framework_async_macros::async_test]
async fn the_runs_are_deterministic_and_empty_by_default() {
    let snapshot = with(stair(StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }));
    assert_eq!(compute_stair_runs(&snapshot), compute_stair_runs(&snapshot));
    assert!(compute_stair_runs(&ModelSnapshot::default()).is_empty());
}
