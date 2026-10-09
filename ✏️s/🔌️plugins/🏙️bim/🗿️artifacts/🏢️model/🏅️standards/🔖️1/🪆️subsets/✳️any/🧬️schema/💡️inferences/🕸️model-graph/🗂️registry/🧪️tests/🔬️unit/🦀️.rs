use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use crate::standards::v1::subsets::any::schema::inferences::spaces::SpaceStatus;
use crate::{Phase, Point2, Space, SpaceBoundary};
use protocol::Inference;

fn demo() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses")
}

fn fresh(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("fresh inference")
}

fn taller() -> ModelMutation {
    ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 3.5 })
}

fn described() -> ModelMutation {
    ModelMutation::SetProjectInfo(crate::mutations::set_project_info::SetProjectInfo { description: Some("other".into()), name: None, author: None, organization: None, phase_names: None })
}

fn edited(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> ModelSnapshot {
    crate::mutations::apply_model_mutation(snapshot, mutation).expect("the mutation applies")
}

#[test]
fn the_session_agrees_with_a_fresh_inference_of_every_field() {
    let snapshot = demo();
    assert_eq!(with_inference(Some(41), &snapshot, Clone::clone), fresh(&snapshot));
    close(41);
}

#[test]
fn a_recorded_mutation_recomputes_only_what_it_touches_and_matches_a_fresh_run() {
    let snapshot = demo();
    with_inference(Some(42), &snapshot, |_| ());
    let nodes = report(Some(42)).nodes;
    let taller_model = edited(&snapshot, &taller());
    record_mutations(Some(42), &snapshot, &[taller()]);
    let inference = with_inference(Some(42), &taller_model, Clone::clone);
    let run = report(Some(42));
    assert!(!run.gated && run.computed > 0 && run.computed_by_kind.contains_key("wall-layout"), "the storey height reaches the wall layout: {run:?}");
    assert_eq!(inference, fresh(&taller_model));
    assert_eq!(run.nodes, nodes);
    close(42);
}

#[test]
fn an_edit_no_geometry_reads_is_gated_and_computes_nothing() {
    let snapshot = demo();
    with_inference(Some(43), &snapshot, |_| ());
    let other = edited(&snapshot, &described());
    record_mutations(Some(43), &snapshot, &[described()]);
    let inference = with_inference(Some(43), &other, Clone::clone);
    assert!(report(Some(43)).gated && report(Some(43)).computed == 0);
    assert_eq!(inference, fresh(&other));
    close(43);
}

#[test]
fn an_unchanged_snapshot_is_answered_from_memory() {
    let snapshot = demo();
    with_inference(Some(44), &snapshot, |_| ());
    with_inference(Some(44), &snapshot, |_| ());
    assert!(report(Some(44)).gated && report(Some(44)).computed == 0);
    close(44);
}

#[test]
fn a_change_no_recorded_diff_explains_walks_the_plan_and_matches_a_fresh_run() {
    let snapshot = demo();
    with_inference(Some(45), &snapshot, |_| ());
    let taller_model = edited(&snapshot, &taller());
    let inference = with_inference(Some(45), &taller_model, Clone::clone);
    assert!(!report(Some(45)).gated && report(Some(45)).computed > 0);
    assert_eq!(inference, fresh(&taller_model));
    close(45);
}

#[test]
fn a_recorded_sequence_is_summed_and_a_recorded_diff_that_misses_the_snapshot_is_not_trusted() {
    let snapshot = demo();
    let both = edited(&edited(&snapshot, &taller()), &described());
    with_inference(Some(46), &snapshot, |_| ());
    record_mutations(Some(46), &snapshot, &[taller(), described()]);
    assert_eq!(with_inference(Some(46), &both, Clone::clone), fresh(&both));
    assert!(report(Some(46)).computed_by_kind.contains_key("wall-layout"));
    let elsewhere = edited(&both, &ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 4.0 }));
    record_mutations(Some(46), &both, &[described()]);
    assert_eq!(with_inference(Some(46), &elsewhere, Clone::clone), fresh(&elsewhere), "the recorded diff does not explain the change, so every node is checked");
    close(46);
}

#[test]
fn sessions_are_per_instance_and_closing_drops_them() {
    let snapshot = demo();
    with_inference(Some(47), &snapshot, |inference| assert!(!inference.storey_levels.is_empty()));
    assert!(!terminal_is_empty(47));
    assert!(terminal_is_empty(48));
    close(47);
    assert!(terminal_is_empty(47));
}

#[test]
fn a_read_may_reach_the_registry_again() {
    let snapshot = demo();
    let nested = with_inference(Some(49), &snapshot, |outer| with_inference(Some(49), &snapshot, |inner| inner == outer));
    assert!(nested);
    close(49);
}

#[test]
fn a_job_steps_the_session_with_monotonic_progress_and_settles_it() {
    let snapshot = demo();
    let mut run = begin(Some(50), &snapshot);
    let mut fractions = vec![run.progress().fraction];
    loop {
        let progress = step(Some(50), &mut run, &snapshot, 3).expect("the step runs");
        fractions.push(progress.fraction);
        if progress.done {
            break;
        }
    }
    assert!(fractions.len() > 3 && fractions.windows(2).all(|pair| pair[0] <= pair[1]), "{fractions:?}");
    finish(Some(50), run, &snapshot).expect("a finished run settles the session");
    with_inference(Some(50), &snapshot, |_| ());
    assert!(report(Some(50)).gated, "the settled session answers from memory");
    assert_eq!(with_inference(Some(50), &snapshot, Clone::clone), fresh(&snapshot));
    close(50);
}

#[test]
fn a_cancelled_job_publishes_nothing_and_the_next_read_reuses_what_it_finished() {
    let snapshot = demo();
    let mut run = begin(Some(51), &snapshot);
    step(Some(51), &mut run, &snapshot, 3).expect("the step runs");
    let finished = run.finished();
    assert!(finished > 0);
    run.cancel();
    assert_eq!(finish(Some(51), run, &snapshot), Err(InferenceError::Cancelled));
    assert!(report(Some(51)).cancelled);
    assert_eq!(with_inference(Some(51), &snapshot, Clone::clone), fresh(&snapshot));
    assert_eq!(report(Some(51)).reused, finished, "the finished nodes were not computed again");
    close(51);
}

#[test]
fn a_probe_infers_a_room_the_document_does_not_have_and_leaves_the_session_alone() {
    let snapshot = demo();
    let mut probe = snapshot.clone();
    let space = Space { storey: "st-ground".into(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 4.0, y: 3.0 } }, usage: String::new(), phase: Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None };
    probe.spaces.insert("probe".into(), space);
    let rooms = probe_rooms(Some(52), &probe);
    assert_eq!(rooms.get("probe").map(|room| room.status), Some(SpaceStatus::Inferred));
    assert!((30.0..48.0).contains(&rooms["probe"].area), "the room is the enclosure: {}", rooms["probe"].area);
    assert!(with_inference(Some(52), &snapshot, |inference| inference.spaces.is_empty()), "the document has no space");
    assert!(!terminal_is_empty(52));
    close(52);
    assert!(terminal_is_empty(52));
}
