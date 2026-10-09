use super::*;
use crate::{Axis, Entry, ModelDiff, Point2, ProjectPatch, StoreyPatch, WallPatch};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the snapshot decodes")
}

fn fresh(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("infers")
}

fn line(from: (f64, f64), to: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: from.0, y: from.1 }, end: Point2 { x: to.0, y: to.1 } }
}

fn taller() -> ModelDiff {
    ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }))
}

fn moved_wall() -> ModelDiff {
    ModelDiff::walls("w-ground-south", Entry::Patched(WallPatch { axis: Some(line((0.0, 0.5), (8.0, 0.5))), ..Default::default() }))
}

fn renamed() -> ModelDiff {
    ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() }
}

fn after(snapshot: &ModelSnapshot, diff: &ModelDiff) -> ModelSnapshot {
    protocol::apply_diff(diff, snapshot).expect("the edit applies")
}

#[test]
fn a_stepped_run_reports_a_monotonic_progress_and_equals_a_fresh_inference() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let mut run = session.begin_full();
    assert_eq!(run.progress().fraction, 0.0, "nothing is planned before the first step");
    let mut fractions = vec![run.progress().fraction];
    loop {
        let progress = session.step(&mut run, &snapshot, 4).expect("the step runs");
        fractions.push(progress.fraction);
        assert!(progress.completed <= progress.total);
        if progress.done {
            break;
        }
    }
    assert!(fractions.len() > 3, "a fuel of 4 splits the run into several steps: {fractions:?}");
    assert!(fractions.windows(2).all(|pair| pair[0] <= pair[1]), "the progress never decreases: {fractions:?}");
    assert_eq!(fractions.last(), Some(&1.0));
    let inference = session.finish(run, &snapshot).expect("a finished run is adopted").clone();
    assert_eq!(inference, fresh(&snapshot), "the stepped run equals a fresh inference");
    assert_eq!(session.report().computed, session.report().nodes, "a cold run computes every node once");
}

#[test]
fn a_cancelled_run_keeps_the_finished_nodes_and_leaves_the_inference_untouched() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let before = session.inference().clone();
    let mut run = session.begin_full();
    let first = session.step(&mut run, &snapshot, 5).expect("the step runs");
    assert!(!first.done && run.finished() > 0, "the run is under way: {first:?}");
    let finished = run.finished();
    run.cancel();
    assert!(run.is_cancelled());
    assert_eq!(session.step(&mut run, &snapshot, 5), Err(InferenceError::Cancelled), "a cancelled run refuses to step");
    assert_eq!(session.finish(run, &snapshot).err(), Some(InferenceError::Cancelled));
    assert!(session.report().cancelled && session.report().fault == Some(InferenceError::Cancelled));
    assert_eq!(session.report().computed, finished, "the report counts what the cancelled run finished");
    assert_eq!(session.inference(), &before, "a cancelled run publishes nothing");
    let inference = session.try_refresh(&snapshot).expect("the next run completes").clone();
    let report = session.report().clone();
    assert_eq!(report.computed, report.nodes - finished, "only the unfinished nodes compute again: {report:?}");
    assert_eq!(report.reused, finished, "the finished nodes were served by the cache");
    assert_eq!(inference, fresh(&snapshot));
}

#[test]
fn a_cancelled_update_unsettles_the_session_so_a_gated_diff_cannot_serve_stale_values() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let edit = taller();
    let edited = after(&snapshot, &edit);
    let mut run = session.begin(&edit);
    session.step(&mut run, &edited, 2).expect("the step runs");
    run.cancel();
    assert!(session.finish(run, &edited).is_err());
    let inference = session.update(&edited, &renamed()).clone();
    assert!(!session.report().gated, "a diff that touches nothing is not trusted after a cancelled run");
    assert_eq!(inference, fresh(&edited));
}

#[test]
fn a_finished_early_run_is_a_value_not_a_panic() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let mut run = session.begin_full();
    session.step(&mut run, &snapshot, 3).expect("the step runs");
    let error = session.finish(run, &snapshot).err().expect("an unfinished run is refused");
    assert!(matches!(error, InferenceError::Compute { .. }), "{error}");
    assert_eq!(session.report().fault.as_ref(), Some(&error));
    assert_eq!(session.refresh(&snapshot), &fresh(&snapshot), "the session recovers with the next run");
    assert_eq!(session.report().fault, None);
}

#[test]
fn a_gated_run_is_done_without_a_step() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let run = session.begin(&renamed());
    assert!(run.progress().done && run.progress().fraction == 1.0);
    session.finish(run, &snapshot).expect("a gated run finishes");
    assert!(session.report().gated && session.report().computed == 0);
}

#[test]
fn a_synced_session_equals_a_fresh_inference_after_every_kind_of_edit() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.sync(&snapshot);
    assert!(!session.report().gated, "the first sync walks the plan");
    assert_eq!(session.sync(&snapshot), &fresh(&snapshot));
    assert!(session.report().gated && session.report().computed == 0, "the same snapshot is answered from memory");
    let mut current = snapshot;
    for edit in [taller(), moved_wall(), renamed()] {
        let next = after(&current, &edit);
        session.record(edit.clone());
        let synced = session.sync(&next).clone();
        assert_eq!(synced, fresh(&next), "recorded edit {edit:?}");
        current = next;
    }
    assert!(session.report().gated, "a project rename touches nothing the graph reads");
}

#[test]
fn a_change_no_recorded_diff_explains_walks_the_plan_and_matches_a_fresh_run() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.sync(&snapshot);
    let edited = after(&snapshot, &taller());
    let synced = session.sync(&edited).clone();
    assert!(!session.report().gated && session.report().computed > 0);
    assert_eq!(synced, fresh(&edited));
    let described = after(&edited, &renamed());
    session.record(renamed());
    let other = after(&described, &moved_wall());
    let synced = session.sync(&other).clone();
    assert!(!session.report().gated, "a recorded diff that does not carry the held snapshot to the read one is not trusted");
    assert_eq!(synced, fresh(&other));
}

#[test]
fn a_recorded_diff_computes_only_what_it_touches() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    session.sync(&snapshot);
    let nodes = session.report().nodes;
    let edit = moved_wall();
    let edited = after(&snapshot, &edit);
    session.record(edit);
    session.sync(&edited);
    let report = session.report();
    assert!(report.computed > 0 && report.computed < nodes, "{} of {nodes} nodes", report.computed);
}

#[test]
fn a_run_another_run_overtook_is_dropped_and_changes_nothing() {
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let mut slow = session.begin_full();
    session.step(&mut slow, &snapshot, 3).expect("the step runs");
    let edited = after(&snapshot, &taller());
    let current = session.refresh(&edited).clone();
    assert_eq!(session.finish(slow, &snapshot).expect("an overtaken run is dropped, not an error").clone(), current);
    assert_eq!(session.inference(), &fresh(&edited), "the newer run's inference stands");
    assert_eq!(session.sync(&edited), &fresh(&edited));
    assert!(session.report().gated, "the session is settled at the newer snapshot");
}
