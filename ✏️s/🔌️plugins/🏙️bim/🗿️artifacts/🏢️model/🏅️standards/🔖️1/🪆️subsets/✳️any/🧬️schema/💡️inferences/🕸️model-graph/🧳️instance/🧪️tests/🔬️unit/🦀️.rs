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

fn mounted() -> ArtifactInstanceOperationOwnerHandle {
    ArtifactInstanceOperationOwnerHandle::detached()
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
    let (snapshot, instance) = (demo(), mounted());
    assert_eq!(with_inference(Some(&instance), &snapshot, Clone::clone), fresh(&snapshot));
}

#[test]
fn a_call_without_an_instance_keeps_nothing_and_agrees_with_a_fresh_inference() {
    let snapshot = demo();
    assert_eq!(with_inference(None, &snapshot, Clone::clone), fresh(&snapshot));
    assert_eq!(report(None), UpdateReport::default(), "a throwaway session reports nothing");
}

#[test]
fn a_recorded_mutation_recomputes_only_what_it_touches_and_matches_a_fresh_run() {
    let (snapshot, instance) = (demo(), mounted());
    with_inference(Some(&instance), &snapshot, |_| ());
    let nodes = report(Some(&instance)).nodes;
    let taller_model = edited(&snapshot, &taller());
    record_mutations(Some(&instance), &snapshot, &[taller()]);
    let inference = with_inference(Some(&instance), &taller_model, Clone::clone);
    let run = report(Some(&instance));
    assert!(!run.gated && run.computed > 0 && run.computed_by_kind.contains_key("wall-layout"), "the storey height reaches the wall layout: {run:?}");
    assert_eq!(inference, fresh(&taller_model));
    assert_eq!(run.nodes, nodes);
}

#[test]
fn an_edit_no_geometry_reads_is_gated_and_computes_nothing() {
    let (snapshot, instance) = (demo(), mounted());
    with_inference(Some(&instance), &snapshot, |_| ());
    let other = edited(&snapshot, &described());
    record_mutations(Some(&instance), &snapshot, &[described()]);
    let inference = with_inference(Some(&instance), &other, Clone::clone);
    assert!(report(Some(&instance)).gated && report(Some(&instance)).computed == 0);
    assert_eq!(inference, fresh(&other));
}

#[test]
fn an_unchanged_snapshot_is_answered_from_memory() {
    let (snapshot, instance) = (demo(), mounted());
    with_inference(Some(&instance), &snapshot, |_| ());
    with_inference(Some(&instance), &snapshot, |_| ());
    assert!(report(Some(&instance)).gated && report(Some(&instance)).computed == 0);
}

#[test]
fn a_change_no_recorded_diff_explains_walks_the_plan_and_matches_a_fresh_run() {
    let (snapshot, instance) = (demo(), mounted());
    with_inference(Some(&instance), &snapshot, |_| ());
    let taller_model = edited(&snapshot, &taller());
    let inference = with_inference(Some(&instance), &taller_model, Clone::clone);
    assert!(!report(Some(&instance)).gated && report(Some(&instance)).computed > 0);
    assert_eq!(inference, fresh(&taller_model));
}

#[test]
fn a_recorded_sequence_is_summed_and_a_recorded_diff_that_misses_the_snapshot_is_not_trusted() {
    let (snapshot, instance) = (demo(), mounted());
    let both = edited(&edited(&snapshot, &taller()), &described());
    with_inference(Some(&instance), &snapshot, |_| ());
    record_mutations(Some(&instance), &snapshot, &[taller(), described()]);
    assert_eq!(with_inference(Some(&instance), &both, Clone::clone), fresh(&both));
    assert!(report(Some(&instance)).computed_by_kind.contains_key("wall-layout"));
    let elsewhere = edited(&both, &ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 4.0 }));
    record_mutations(Some(&instance), &both, &[described()]);
    assert_eq!(with_inference(Some(&instance), &elsewhere, Clone::clone), fresh(&elsewhere), "the recorded diff does not explain the change, so every node is checked");
}

#[test]
fn the_instance_owns_its_sessions_and_the_close_ladder_drops_them() {
    let (snapshot, first, second) = (demo(), mounted(), mounted());
    with_inference(Some(&first), &snapshot, |inference| assert!(!inference.storey_levels.is_empty()));
    assert!(first.has_inference(DOCUMENT) && !second.has_inference(DOCUMENT), "a session belongs to the instance that read through it");
    assert_eq!(report(Some(&second)), UpdateReport::default());
    assert!(report(Some(&first)).nodes > 0);
}

#[test]
fn a_clone_of_the_handle_reaches_the_same_sessions() {
    let (snapshot, instance) = (demo(), mounted());
    let other = instance.clone();
    with_inference(Some(&instance), &snapshot, |_| ());
    with_inference(Some(&other), &snapshot, |_| ());
    assert!(report(Some(&other)).gated, "the clone answered from the session the first read settled");
}

#[test]
fn a_read_may_reach_the_instance_again() {
    let (snapshot, instance) = (demo(), mounted());
    let nested = with_inference(Some(&instance), &snapshot, |outer| with_inference(Some(&instance), &snapshot, |inner| inner == outer));
    assert!(nested);
}

#[test]
fn a_job_steps_the_session_with_monotonic_progress_and_settles_it() {
    let (snapshot, instance) = (demo(), mounted());
    let mut run = begin(Some(&instance), &snapshot);
    let mut fractions = vec![run.progress().fraction];
    loop {
        let progress = step(Some(&instance), &mut run, &snapshot, 3).expect("the step runs");
        fractions.push(progress.fraction);
        if progress.done {
            break;
        }
    }
    assert!(fractions.len() > 3 && fractions.windows(2).all(|pair| pair[0] <= pair[1]), "{fractions:?}");
    finish(Some(&instance), run, &snapshot).expect("a finished run settles the session");
    with_inference(Some(&instance), &snapshot, |_| ());
    assert!(report(Some(&instance)).gated, "the settled session answers from memory");
    assert_eq!(with_inference(Some(&instance), &snapshot, Clone::clone), fresh(&snapshot));
}

#[test]
fn a_cancelled_job_publishes_nothing_and_the_next_read_reuses_what_it_finished() {
    let (snapshot, instance) = (demo(), mounted());
    let mut run = begin(Some(&instance), &snapshot);
    step(Some(&instance), &mut run, &snapshot, 3).expect("the step runs");
    let finished = run.finished();
    assert!(finished > 0);
    run.cancel();
    assert_eq!(finish(Some(&instance), run, &snapshot), Err(InferenceError::Cancelled));
    assert!(report(Some(&instance)).cancelled);
    assert_eq!(with_inference(Some(&instance), &snapshot, Clone::clone), fresh(&snapshot));
    assert_eq!(report(Some(&instance)).reused, finished, "the finished nodes were not computed again");
}

#[test]
fn an_analysis_settles_the_session_in_bounded_steps_and_a_cancelled_one_keeps_its_nodes() {
    let (snapshot, instance) = (demo(), mounted());
    let mut cancelled = Analysis::new(Some(&instance), 3);
    let first = cancelled.advance(&snapshot).expect("a step");
    assert!(!first.done && cancelled.fraction() > 0.0);
    cancelled.cancel(&snapshot);
    assert!(report(Some(&instance)).cancelled);
    let mut analysis = Analysis::new(Some(&instance), 3);
    while !analysis.advance(&snapshot).expect("a step").done {}
    assert!(analysis.is_done() && analysis.fraction() == 1.0);
    assert!(report(Some(&instance)).reused > 0, "the nodes the cancelled analysis finished were not computed again");
    with_inference(Some(&instance), &snapshot, |_| ());
    assert!(report(Some(&instance)).gated, "the analysed session answers from memory");
}

#[test]
fn an_analysis_without_an_instance_runs_on_a_session_of_its_own_to_the_end() {
    let snapshot = demo();
    let mut analysis = Analysis::new(None, 5);
    let mut steps = 0;
    while !analysis.advance(&snapshot).expect("a step").done {
        steps += 1;
    }
    assert!(steps > 1 && analysis.is_done());
}

#[test]
fn a_probe_infers_a_room_the_document_does_not_have_and_leaves_the_session_alone() {
    let (snapshot, instance) = (demo(), mounted());
    let mut probe = snapshot.clone();
    let space = Space { storey: "st-ground".into(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 4.0, y: 3.0 } }, usage: String::new(), phase: Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None };
    probe.spaces.insert("probe".into(), space);
    let rooms = probe_rooms(Some(&instance), &probe);
    assert_eq!(rooms.get("probe").map(|room| room.status), Some(SpaceStatus::Inferred));
    assert!((30.0..48.0).contains(&rooms["probe"].area), "the room is the enclosure: {}", rooms["probe"].area);
    assert!(with_inference(Some(&instance), &snapshot, |inference| inference.spaces.is_empty()), "the document has no space");
    assert!(instance.has_inference(PROBE) && instance.has_inference(DOCUMENT));
}

#[test]
fn an_engine_fault_shows_the_last_complete_inference_and_a_finding_in_both_languages_never_an_empty_model() {
    let (snapshot, instance) = (demo(), mounted());
    let complete = with_inference(Some(&instance), &snapshot, Clone::clone);
    assert!(!complete.storey_levels.is_empty());
    let shown_after = instance.lend_inference::<ModelInferenceSession, _>(DOCUMENT, |session| shown(session, Err(InferenceError::Cancelled)).into_owned());
    assert_eq!(shown_after.storey_levels, complete.storey_levels, "the last complete values stay");
    assert_eq!(shown_after.wall_layout, complete.wall_layout);
    let finding = shown_after.diagnostics.iter().find(|finding| finding.code == DiagnosticCode::InferenceFault).expect("the fault is a finding");
    assert_eq!(finding.severity, crate::standards::v1::subsets::any::schema::inferences::diagnostics::Severity::Error);
    assert!(finding.text("en").is_some_and(|text| text.contains("could not be analysed")) && finding.text("de").is_some_and(|text| text.contains("nicht vollständig analysiert")));
    assert_eq!(shown_after.diagnostic_index.total.error, complete.diagnostic_index.total.error + 1, "the problem counters count it");
    assert_eq!(shown_after.diagnostics.len(), complete.diagnostics.len() + 1);
}

#[test]
fn a_session_that_did_not_fault_is_shown_as_it_is() {
    let snapshot = demo();
    let mut session = ModelInferenceSession::default();
    session.try_sync(&snapshot).expect("syncs");
    assert!(matches!(shown(&session, Ok(())), Cow::Borrowed(_)));
}
