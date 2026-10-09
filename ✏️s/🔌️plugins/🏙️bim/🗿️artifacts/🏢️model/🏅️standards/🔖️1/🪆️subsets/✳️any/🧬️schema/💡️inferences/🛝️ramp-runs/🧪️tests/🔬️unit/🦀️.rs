use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::ramp_runs::table_json;
use crate::{TopConstraint, Vertex};
use semio_framework_geometry::loops;

const EPS: f64 = 1e-9;

fn level() -> StoreyLevel {
    StoreyLevel { elevation: 0.0, top_elevation: 3.0, absolute_elevation: 0.0, absolute_top_elevation: 3.0 }
}

fn vertex(x: f64, y: f64, bulge: f64) -> Vertex {
    Vertex { point: crate::Point2 { x, y }, bulge }
}

fn ramp(path: Vec<Vertex>, rise: f64) -> Ramp {
    Ramp {
        storey: "st-ground".into(),
        path,
        width: 1.2,
        landing_start: 1.5,
        landing_end: 1.5,
        landing_turn: 1.5,
        max_slope: 1.0 / 12.0,
        thickness: 0.2,
        material: "m-concrete".into(),
        base_offset: 0.0,
        top: TopConstraint::Unconnected { height: rise },
        railing_left: false,
        railing_right: false,
        name: "Ramp".into(),
    }
}

fn straight(length: f64, rise: f64) -> Ramp {
    ramp(vec![vertex(0.0, 0.0, 0.0), vertex(length, 0.0, 0.0)], rise)
}

#[test]
fn a_straight_ramp_spreads_its_rise_over_the_sloped_run_between_two_landings() {
    let run = run_of(&straight(10.0, 0.5), &level(), None);
    assert!((run.length - 10.0).abs() < EPS);
    assert!((run.run_length - 7.0).abs() < EPS);
    assert!((run.rise - 0.5).abs() < EPS);
    assert!((run.slope - 0.5 / 7.0).abs() < EPS);
    assert!((run.angle - (0.5f64 / 7.0).atan()).abs() < EPS);
    assert_eq!(run.landings.len(), 2);
    assert_eq!(run.flights.len(), 1);
    assert!((run.flights[0].from - 1.5).abs() < EPS && (run.flights[0].to - 8.5).abs() < EPS);
    assert!(run.landings[0].z.abs() < EPS && (run.landings[1].z - 0.5).abs() < EPS);
    assert!(run.compliance.compliant);
}

#[test]
fn the_height_follows_the_path_flat_on_landings_and_linear_between_them() {
    let run = run_of(&straight(10.0, 0.5), &level(), None);
    assert!(run.z_at(0.0).abs() < EPS && run.z_at(1.5).abs() < EPS);
    assert!((run.z_at(5.0) - 0.5 * 3.5 / 7.0).abs() < EPS);
    assert!((run.z_at(8.5) - 0.5).abs() < EPS && (run.z_at(10.0) - 0.5).abs() < EPS);
    assert!((run.z_at(-3.0) - run.z_at(0.0)).abs() < EPS && (run.z_at(99.0) - run.z_at(10.0)).abs() < EPS);
}

#[test]
fn a_ramp_steeper_than_its_limit_is_not_compliant() {
    let run = run_of(&straight(10.0, 0.7), &level(), None);
    assert!((run.slope - 0.1).abs() < EPS);
    assert!(run.compliance.run_ok && !run.compliance.slope_ok && !run.compliance.compliant);
    let limit = Ramp { max_slope: 0.1, ..straight(10.0, 0.7) };
    assert!(run_of(&limit, &level(), None).compliance.compliant);
}

#[test]
fn a_corner_gets_a_landing_centred_on_it() {
    let bent = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(6.0, 0.0, 0.0), vertex(6.0, 5.0, 0.0)], 0.6);
    let run = run_of(&bent, &level(), None);
    assert!((run.length - 11.0).abs() < EPS);
    let spans: Vec<(f64, f64)> = run.landings.iter().map(|landing| (landing.from, landing.to)).collect();
    assert_eq!(spans.len(), 3);
    assert!((spans[1].0 - 5.25).abs() < EPS && (spans[1].1 - 6.75).abs() < EPS);
    assert!((run.run_length - (3.75 + 2.75)).abs() < EPS);
    assert_eq!(run.flights.len(), 2);
    assert!((run.flights[1].z_from - run.flights[0].z_to).abs() < EPS);
}

#[test]
fn a_smooth_join_is_no_corner() {
    let smooth = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(4.0, 0.0, 0.0), vertex(8.0, 0.0, 0.0)], 0.4);
    assert_eq!(run_of(&smooth, &level(), None).landings.len(), 2);
}

#[test]
fn overlapping_landings_merge_and_are_clipped_to_the_path() {
    let bent = Ramp { landing_start: 6.0, landing_turn: 3.0, landing_end: 20.0, ..ramp(vec![vertex(0.0, 0.0, 0.0), vertex(6.0, 0.0, 0.0), vertex(6.0, 5.0, 0.0)], 0.0) };
    let run = run_of(&bent, &level(), None);
    assert_eq!(run.landings.len(), 1);
    assert!(run.landings[0].from.abs() < EPS && (run.landings[0].to - 11.0).abs() < EPS);
    assert!(run.flights.is_empty() && run.compliance.compliant);
}

#[test]
fn a_rise_without_a_sloped_run_is_not_compliant() {
    let flat = Ramp { landing_start: 6.0, landing_end: 6.0, ..straight(10.0, 0.3) };
    let run = run_of(&flat, &level(), None);
    assert!(run.run_length.abs() < EPS && run.slope.abs() < EPS);
    assert!(!run.compliance.run_ok && !run.compliance.compliant);
    assert!((run.z_at(5.0) - 0.15).abs() < EPS);
}

#[test]
fn a_descending_ramp_has_a_positive_slope() {
    let down = Ramp { base_offset: 0.5, top: TopConstraint::Unconnected { height: -0.5 }, ..straight(10.0, 0.0) };
    let run = run_of(&down, &level(), None);
    assert!((run.rise + 0.5).abs() < EPS && run.slope > 0.0);
    assert!((run.z_at(0.0) - 0.5).abs() < EPS && run.z_at(10.0).abs() < EPS);
}

#[test]
fn the_top_follows_a_storey_constraint() {
    let to_first = Ramp { top: TopConstraint::Storey { storey: "st-first".into(), offset: 0.0 }, ..straight(40.0, 0.0) };
    let target = StoreyLevel { elevation: 3.0, top_elevation: 5.8, absolute_elevation: 3.0, absolute_top_elevation: 5.8 };
    let run = run_of(&to_first, &level(), Some(&target));
    assert!((run.rise - 3.0).abs() < EPS && (run.top_z - 3.0).abs() < EPS);
    let raised = StoreyLevel { elevation: 3.4, top_elevation: 6.2, absolute_elevation: 3.4, absolute_top_elevation: 6.2 };
    assert!((run_of(&to_first, &level(), Some(&raised)).rise - 3.4).abs() < EPS);
}

#[test]
fn a_curved_path_measures_its_arc_length() {
    let quarter = ramp(vec![vertex(0.0, 0.0, (std::f64::consts::PI / 8.0).tan()), vertex(5.0, 5.0, 0.0)], 0.3);
    let run = run_of(&quarter, &level(), None);
    assert!((run.length - 5.0 * std::f64::consts::FRAC_PI_2).abs() < 1e-9);
}

#[test]
fn a_degenerate_path_has_no_run() {
    let point = ramp(vec![vertex(1.0, 1.0, 0.0), vertex(1.0, 1.0, 0.0)], 0.3);
    let run = run_of(&point, &level(), None);
    assert!(run.length.abs() < EPS && !run.compliance.run_ok);
    assert!(strip_of(&point).is_empty());
}

#[test]
fn the_strip_of_a_straight_ramp_is_its_rectangle() {
    let strip = strip_of(&straight(10.0, 0.5));
    let outline = strip.outline();
    assert!((loops::area(&outline) - 12.0).abs() < 1e-9);
    assert!(loops::is_ccw(&outline));
    let (left, right, centre) = strip.section_at(4.0);
    assert!((left.y - 0.6).abs() < EPS && (right.y + 0.6).abs() < EPS && (centre.x - 4.0).abs() < EPS);
}

#[test]
fn a_mitred_corner_keeps_the_width_and_adds_the_corner_wedge() {
    let bent = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(6.0, 0.0, 0.0), vertex(6.0, 5.0, 0.0)], 0.6);
    let strip = strip_of(&bent);
    assert!((loops::area(&strip.outline()) - 1.2 * 11.0).abs() < 1e-9);
    let (left, right, _) = strip.section_at(6.0);
    assert!((left.x - 5.4).abs() < EPS && (left.y - 0.6).abs() < EPS);
    assert!((right.x - 6.6).abs() < EPS && (right.y + 0.6).abs() < EPS);
}

#[test]
fn a_curved_strip_keeps_arcs_and_the_exact_area() {
    let quarter = ramp(vec![vertex(0.0, 0.0, (std::f64::consts::PI / 8.0).tan()), vertex(5.0, 5.0, 0.0)], 0.3);
    let strip = strip_of(&quarter);
    assert!((loops::area(&strip.outline()) - 1.2 * 5.0 * std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    assert!(strip.outline().iter().any(|corner| corner.bulge != 0.0));
}

#[test]
fn the_run_table_is_canonical_json_of_every_ramp() {
    let mut snapshot = ModelSnapshot::default();
    snapshot.storeys.insert("st-ground".into(), crate::Storey { building: "b".into(), name: "G".into(), level: 0, height: 3.0, cut_height: None });
    snapshot.ramps.insert("rp".into(), straight(10.0, 0.5));
    let runs = compute_ramp_runs(&snapshot);
    assert_eq!(runs.len(), 1);
    let table: serde_json::Value = serde_json::from_str(&table_json(&runs)).expect("a JSON table");
    assert!((table["rp"]["run_length"].as_f64().expect("a number") - 7.0).abs() < 1e-9);
}

const RAMPS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json");
const RAMPS_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛝️ramp-runs/🏞️ramps/💡️inference/🛝️ramp-runs/🔣️.json");

fn ramps() -> ModelSnapshot {
    semio_framework_pack_json::from_json_str(RAMPS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the ramps model decodes")
}

fn with_hosted_railings() -> ModelSnapshot {
    let mut document: serde_json::Value = serde_json::from_str(RAMPS).expect("json");
    let railing = |host: &str| {
        serde_json::json!({
            "storey": "st-ground", "path": [], "height": 1.0, "post_spacing": 1.2,
            "profile": { "Rectangle": { "width": 0.06, "depth": 0.04 } }, "post_profile": { "Rectangle": { "width": 0.05, "depth": 0.05 } },
            "infill": "None", "material": "m-concrete", "base_offset": 0.0, "host": { "element": host, "side": "Left", "edge": 0, "inset": 0.05 }, "phase": "New", "name": "Guard",
        })
    };
    document["railings"] = serde_json::json!({ "rail-bent": railing("r-bent"), "rail-curved": railing("r-curved") });
    semio_framework_pack_json::from_json_str(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the ramps and railings decode")
}

#[test]
fn the_subject_reproduces_the_third_party_oracle_table() {
    let runs = compute_ramp_runs(&ramps());
    assert!(!runs.contains_key("r-orphan"), "a ramp on a missing storey has no run");
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(RAMPS_TABLE, &table_json(&runs));
    assert!(problems.is_empty(), "{} disagreements with the shapely oracle: {:?}", problems.len(), &problems[..problems.len().min(12)]);
}

#[test]
fn a_ramp_depends_on_its_storey_and_the_storey_its_top_targets() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode};
    let steps = plan::build(&ramps(), kinds::closure(kinds::RAMP_RUNS));
    let parents = |id: &str| steps.iter().find(|step| step.key == ModelNode::RampRun(id.into())).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("r-straight"), vec![ModelNode::Storey("st-ground".into())]);
    assert_eq!(parents("r-to-first"), vec![ModelNode::Storey("st-ground".into()), ModelNode::Storey("st-first".into())]);
    assert!(steps.iter().all(|step| step.key != ModelNode::RampRun("r-orphan".into())), "a ramp on a missing storey is not planned");
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    steps.iter().for_each(|step| step.parents.iter().for_each(|parent| assert!(position(parent) < position(&step.key), "parents come first")));
}

#[test]
fn the_ramps_are_gated_a_storey_edit_re_infers_the_ramps_it_reaches_and_a_material_edit_none() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
    use crate::{Entry, MaterialPatch, ModelDiff, ModelInference, RampPatch, StoreyPatch};
    use protocol::Inference;
    let snapshot = ramps();
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).ramp_runs.clone();
    let untouched = session.update(&snapshot, &ModelDiff::default()).ramp_runs.clone();
    assert!(session.report().gated && session.report().computed == 0 && first == untouched, "a diff outside the reads serves the stored result");
    let paint = ModelDiff::materials("m-concrete", Entry::Patched(MaterialPatch { density: Some(2500.0), ..Default::default() }));
    let painted = protocol::apply_diff(&paint, &snapshot).expect("applies");
    assert_eq!(session.update(&painted, &paint).ramp_runs.clone(), first, "a material edit leaves the runs alone");
    assert_eq!(session.report().computed_by_kind.get("ramp-run"), None, "and recomputes no run");
    let renamed = ModelDiff::ramps("r-straight", Entry::Patched(RampPatch { name: Some("Renamed".into()), ..Default::default() }));
    let renamed_snapshot = protocol::apply_diff(&renamed, &snapshot).expect("applies");
    session.update(&renamed_snapshot, &renamed);
    assert_eq!(session.report().computed_by_kind.get("ramp-run"), None, "a name is no input of a run");
    let wider = ModelDiff::ramps("r-straight", Entry::Patched(RampPatch { width: Some(1.8), ..Default::default() }));
    let wider_snapshot = protocol::apply_diff(&wider, &snapshot).expect("applies");
    let after_width = session.update(&wider_snapshot, &wider).ramp_runs.clone();
    assert_eq!(session.report().computed_by_kind.get("ramp-run"), Some(&1), "one ramp edit re-infers one run");
    assert!((after_width["r-straight"].width - 1.8).abs() < EPS);
    assert_eq!(after_width, ModelInference::infer(&wider_snapshot).expect("infers").ramp_runs);
    let deeper = ModelDiff::storeys("st-base", Entry::Patched(StoreyPatch { height: Some(3.0), ..Default::default() }));
    let deeper_snapshot = protocol::apply_diff(&deeper, &wider_snapshot).expect("applies");
    session.update(&deeper_snapshot, &deeper);
    assert_eq!(session.report().computed_by_kind.get("ramp-run"), None, "a storey no ramp is resolved by leaves every run");
    let taller = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let taller_snapshot = protocol::apply_diff(&taller, &deeper_snapshot).expect("applies");
    let recomputed = session.update(&taller_snapshot, &taller).ramp_runs.clone();
    assert!((first["r-to-first"].rise - 3.0).abs() < EPS && (recomputed["r-to-first"].rise - 3.4).abs() < EPS, "a ramp that follows the next storey climbs more");
    assert!((recomputed["r-storey-top"].rise - first["r-storey-top"].rise).abs() < EPS, "a ramp on the next storey keeps its rise");
    assert!((recomputed["r-straight"].rise - first["r-straight"].rise).abs() < EPS);
    assert_eq!(recomputed, ModelInference::infer(&taller_snapshot).expect("infers").ramp_runs, "the incremental result equals a fresh one");
}

#[test]
fn every_projection_of_a_model_with_ramps_is_cache_transparent_warm_equals_cold_equals_uncached() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
    use crate::ModelInference;
    use protocol::Inference;
    for snapshot in [ramps(), with_hosted_railings()] {
        let uncached = ModelInference::infer(&snapshot).expect("infers");
        let mut session = ModelInferenceSession::new();
        let cold = session.refresh(&snapshot).clone();
        assert!(session.report().computed > 0 && !session.report().gated);
        let warm = session.refresh(&snapshot).clone();
        assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
        for (name, equal) in [
            ("ramp_runs", cold.ramp_runs == uncached.ramp_runs && warm.ramp_runs == uncached.ramp_runs),
            ("element_solids", cold.element_solids == uncached.element_solids && warm.element_solids == uncached.element_solids),
            ("plan_linework", cold.plan_linework == uncached.plan_linework && warm.plan_linework == uncached.plan_linework),
            ("quantities", cold.quantities == uncached.quantities && warm.quantities == uncached.quantities),
            ("diagnostics", cold.diagnostics == uncached.diagnostics && warm.diagnostics == uncached.diagnostics),
        ] {
            assert!(equal, "{name}: cold, warm and uncached agree");
        }
        assert_eq!(cold, uncached);
        assert_eq!(warm, uncached);
        assert_eq!(compute_ramp_runs(&snapshot), uncached.ramp_runs, "the thin projection equals the whole inference");
    }
    let hosted = ModelInference::infer(&with_hosted_railings()).expect("infers");
    assert!(hosted.element_solids.contains_key("rail-bent") && hosted.element_solids.contains_key("r-bent"), "the ramp and the railing it hosts have solids");
    assert!(ModelInference::infer(&ModelSnapshot::default()).expect("infers").ramp_runs.is_empty(), "default");
}

#[test]
fn the_hosted_railing_of_a_ramp_follows_the_ramp_when_it_moves() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
    use crate::{Entry, ModelDiff, ModelInference, RampPatch};
    use protocol::Inference;
    let snapshot = with_hosted_railings();
    let mut session = ModelInferenceSession::new();
    let before = session.refresh(&snapshot).clone();
    let longer = ModelDiff::ramps("r-bent", Entry::Patched(RampPatch { path: Some(vec![vertex(0.0, 9.0, 0.0), vertex(6.0, 9.0, 0.0), vertex(6.0, 18.0, 0.0)]), ..Default::default() }));
    let edited = protocol::apply_diff(&longer, &snapshot).expect("applies");
    let after = session.update(&edited, &longer).clone();
    assert_eq!(after, ModelInference::infer(&edited).expect("infers"), "the incremental result equals a fresh one");
    assert!(after.element_solids["rail-bent"] != before.element_solids["rail-bent"], "the hosted railing is re-inferred with its host");
    assert_eq!(after.element_solids["rail-curved"], before.element_solids["rail-curved"], "a railing on another ramp is not");
}
