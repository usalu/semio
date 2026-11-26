use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::InferenceError;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin::ArtifactInstanceOperationOwnerHandle;
use std::borrow::Cow;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{ModelInferenceSession, UpdateReport};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, RunProgress, SessionRun};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::{ordered, Diagnostic, DiagnosticIndex};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::SpaceRoom;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
#[allow(unused_imports)]
use protocol::{InferenceWorkFailure, Mutation, MutationDiff};
#[allow(unused_imports)]
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::lend_handle;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::{with_inference, report, record_mutations, DOCUMENT, begin, Analysis, probe, probe_rooms, PROBE, shown};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::parse_dsl;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::SpaceStatus;
use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Phase, Point2, SpaceBoundary};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Space;
use protocol::Inference;

fn demo() -> ModelSnapshot {
    parse_dsl(semio_framework_test_fixtures::fixture_text!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("the committed demo parses")
}

fn fresh(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("fresh inference")
}

const OWNER_STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 20,
    maximum_copy_bytes: 1 << 26,
    maximum_capacity_bytes: 1 << 26,
    maximum_release_bytes: 1 << 26,
    maximum_depth: 128,
};

struct MountedFixture(ArtifactInstanceOperationOwnerHandle);

impl std::ops::Deref for MountedFixture {
    type Target = ArtifactInstanceOperationOwnerHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MountedFixture {
    fn close_with(&self, caller: &mut OriginalJobFixtureCaller) {
        let loan = RetainedCloneGrant { maximum_copy_bytes: 1 << 16, ..OWNER_STOCK };
        for _ in 0..(1 << 18) {
            if self.close_terminal_is_empty().expect("mounted owner observable") {
                return;
            }
            caller.retirement_turn(loan, |cx| Ok(self.close_inference_step(cx, loan)))
                .expect("funded close turn")
                .expect("mounted inference retires");
        }
        panic!("mounted inference did not close: demand {:?}, available {:?}, fuel {}", self.inference_retirement_demands(loan.maximum_copy_bytes), caller.remaining_grant(), caller.fuel_remaining());
    }
}

impl Drop for MountedFixture {
    fn drop(&mut self) {
        if !std::thread::panicking() && !self.inference_is_empty() {
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(2), Generation(1), OWNER_STOCK, || Some(1), 1000, 1 << 24);
            self.close_with(&mut caller);
            caller.retire(OWNER_STOCK);
        }
    }
}

fn mounted() -> MountedFixture {
    MountedFixture(ArtifactInstanceOperationOwnerHandle::new(Box::new(semio_plugin::plugin_app_close_prelude::EmptyArtifactInstanceOperationOwner)))
}

fn taller() -> ModelMutation {
    ModelMutation::SetStoreyHeight(semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 3.5 })
}

fn described() -> ModelMutation {
    ModelMutation::SetProjectInfo(semio_s_artifact_bim_model_mut_any_17::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo { description: Some("other".into()), name: None, author: None, organization: None, phase_names: None })
}

fn edited(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> ModelSnapshot {
    crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation(snapshot, mutation).expect("the mutation applies")
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
    let elsewhere = edited(&both, &ModelMutation::SetStoreyHeight(semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 4.0 }));
    record_mutations(Some(&instance), &both, &[described()]);
    assert_eq!(with_inference(Some(&instance), &elsewhere, Clone::clone), fresh(&elsewhere), "the recorded diff does not explain the change, so every node is checked");
}

#[test]
fn the_instance_owns_its_sessions_and_the_close_ladder_drops_them() {
    let (snapshot, first, second) = (demo(), mounted(), mounted());
    with_inference(Some(&first), &snapshot, |inference| assert!(!inference.storey_levels.is_empty()));

    assert_eq!(report(Some(&second)), UpdateReport::default());
    assert!(report(Some(&first)).nodes > 0);
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), OWNER_STOCK, || Some(1), 1000, 1 << 24);
    first.close_with(&mut caller);
    caller.retire(OWNER_STOCK);
    assert!(first.inference_is_empty());
    assert_eq!(report(Some(&first)), UpdateReport::default());
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
        let progress = lend_handle(&instance, DOCUMENT, |session| session.step_cold(&mut run, &snapshot, 3)).expect("the step runs");
        fractions.push(progress.fraction);
        if progress.done {
            break;
        }
    }
    assert!(fractions.len() > 3 && fractions.windows(2).all(|pair| pair[0] <= pair[1]), "{fractions:?}");
    lend_handle(&instance, DOCUMENT, |session| session.finish_cold(run, &snapshot).map(|_| ())).expect("a finished run settles the session");
    with_inference(Some(&instance), &snapshot, |_| ());
    assert!(report(Some(&instance)).gated, "the settled session answers from memory");
    assert_eq!(with_inference(Some(&instance), &snapshot, Clone::clone), fresh(&snapshot));
}

#[test]
fn a_cancelled_job_publishes_nothing_and_the_next_read_reuses_what_it_finished() {
    let (snapshot, instance) = (demo(), mounted());
    let mut run = begin(Some(&instance), &snapshot);
    lend_handle(&instance, DOCUMENT, |session| session.step_cold(&mut run, &snapshot, 3)).expect("the step runs");
    let finished = run.finished();
    assert!(finished > 0);
    run.cancel();
    assert_eq!(lend_handle(&instance, DOCUMENT, |session| session.finish_cold(run, &snapshot).map(|_| ())), Err(InferenceError::Cancelled));
    assert!(report(Some(&instance)).cancelled);
    assert_eq!(with_inference(Some(&instance), &snapshot, Clone::clone), fresh(&snapshot));
    assert_eq!(report(Some(&instance)).reused, finished, "the finished nodes were not computed again");
}

#[test]
fn an_analysis_settles_the_session_in_bounded_steps_and_a_cancelled_one_keeps_its_nodes() {
    let (snapshot, instance) = (demo(), mounted());
    let mut cancelled = Analysis::new(Some(&instance));
    let first = cancelled.advance_cold(&snapshot, 3).expect("a step");
    assert!(!first.done && cancelled.fraction() > 0.0);
    cancelled.cancel_cold(&snapshot);
    assert!(report(Some(&instance)).cancelled);
    let mut analysis = Analysis::new(Some(&instance));
    while !analysis.advance_cold(&snapshot, 3).expect("a step").done {}
    assert!(analysis.is_done() && analysis.fraction() == 1.0);
    assert!(report(Some(&instance)).reused > 0, "the nodes the cancelled analysis finished were not computed again");
    with_inference(Some(&instance), &snapshot, |_| ());
    assert!(report(Some(&instance)).gated, "the analysed session answers from memory");
}

#[test]
fn an_analysis_without_an_instance_runs_on_a_session_of_its_own_to_the_end() {
    let snapshot = demo();
    let mut analysis = Analysis::new(None);
    let mut steps = 0;
    while !analysis.advance_cold(&snapshot, 5).expect("a step").done {
        steps += 1;
    }
    assert!(steps > 1 && analysis.is_done());
}

#[semio_framework_async_macros::async_test]
async fn the_original_analysis_borrows_only_the_rendered_revision() {
    use semio_framework_job::RevisionId;
    use semio_plugin::{ArtifactView, ChildContentView, HistoryView, MountedOwnerPolicyV1};
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_view_inference;

    let (snapshot, instance) = (demo(), mounted());
    let revision = [7; 32];
    let history = HistoryView::empty();
    let render = semio_plugin::AppRenderOperationContext {
        app_instance_id: 1,
        base_revision: RevisionId(1),
        canonical_base_revision: revision,
        generation: Generation(1),
        mounted_policy: MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,
    };
    let grant = OWNER_STOCK;
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), grant, || Some(1), 1000, 1 << 24);
    let preview_snapshot = ModelSnapshot::default();
    let preview = ArtifactView::with_render_context(&preview_snapshot, &history, ChildContentView::EMPTY, render, "bim-test", None).await;
    assert!(with_view_inference(Some(&instance), &preview, |projection| projection.storey_levels.is_empty()));
    let mut preview_access = Default::default();
    let preview_refused = (0..64).any(|_| {
        let read = caller.turn(grant, |cx| Ok(Analysis::read_rendered(&instance, &mut preview_access, revision, cx, |_, _| ())))
            .expect("funded preview loan turn");
        match read {
            Err(_) => true,
            Ok(None) => false,
            Ok(Some(())) => panic!("preview was offered to the export reader"),
        }
    });
    assert!(preview_refused);

    let doc = ArtifactView::with_render_context(&snapshot, &history, ChildContentView::EMPTY, render, "bim-test", None).await
        .with_committed_render_snapshot(true);
    let expected = fresh(&snapshot);
    assert_eq!(with_view_inference(Some(&instance), &doc, Clone::clone), expected);
    assert!(with_view_inference(Some(&instance), &preview, |projection| projection.storey_levels.is_empty()));
    let mut export_access = Default::default();
    let count = (0..64).find_map(|_| {
        caller.turn(grant, |cx| Ok(Analysis::read_rendered(&instance, &mut export_access, revision, cx, |projection, _| projection.storey_levels.len())))
            .expect("funded committed loan turn")
            .expect("preview leaves the committed loan available")
    });
    assert_eq!(count, Some(expected.storey_levels.len()));

    let mut completed = false;
    let mut analysis = Analysis::for_rendered_revision(&instance, revision);
    for _ in 0..64 {
        let turn = caller.turn(grant, |cx| Ok(analysis.advance(&snapshot, cx))).expect("funded analysis turn");
        if turn.expect("the rendered revision is eligible").done {
            completed = true;
            break;
        }
    }
    let mut other = Analysis::for_rendered_revision(&instance, [8; 32]);
    let mut refused = false;
    for _ in 0..64 {
        match caller.turn(grant, |cx| Ok(other.advance(&snapshot, cx))).expect("funded analysis turn") {
            Err(_) => {
                refused = true;
                break;
            }
            Ok(progress) if progress.done => break,
            Ok(_) => {}
        }
    }
    instance.close_with(&mut caller);
    caller.retire(grant);
    assert!(completed, "the supplied view's projection settles through the original caller");
    assert_eq!(analysis.fraction(), 1.0);
    assert!(refused, "another revision must not borrow the rendered projection");
    assert!(instance.inference_is_empty(), "the mounted projection retires through the original caller");
}

#[test]
fn a_probe_infers_a_room_the_document_does_not_have_and_leaves_the_session_alone() {
    let (snapshot, instance) = (demo(), mounted());
    let mut probe = snapshot.clone();
    let space = Space { storey: "st-ground".into(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 4.0, y: 3.0 } }, usage: String::new(), phase: Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None };
    probe.spaces.cold_insert("probe".into(), space);
    let rooms = probe_rooms(Some(&instance), &probe);
    assert_eq!(rooms.get("probe").map(|room| room.status), Some(SpaceStatus::Inferred));
    assert!((30.0..48.0).contains(&rooms["probe"].area), "the room is the enclosure: {}", rooms["probe"].area);
    assert!(with_inference(Some(&instance), &snapshot, |inference| inference.spaces.is_empty()), "the document has no space");
    for name in [PROBE, DOCUMENT] {
        instance.with_inference_session_cold(name, || panic!("expected retained inference session {name}"), |_: &mut ModelInferenceSession| Ok(())).expect("the mounted owner keeps both named sessions");
    }
}

#[test]
fn an_engine_fault_shows_the_last_complete_inference_and_a_finding_in_both_languages_never_an_empty_model() {
    let (snapshot, instance) = (demo(), mounted());
    let complete = with_inference(Some(&instance), &snapshot, Clone::clone);
    assert!(!complete.storey_levels.is_empty());
    let shown_after = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::lend_handle(&instance, DOCUMENT, |session| shown(session, Err(InferenceError::Cancelled)).into_owned());
    assert_eq!(shown_after.storey_levels, complete.storey_levels, "the last complete values stay");
    assert_eq!(shown_after.wall_layout, complete.wall_layout);
    let finding = shown_after.diagnostics.iter().find(|finding| finding.code == DiagnosticCode::InferenceFault).expect("the fault is a finding");
    assert_eq!(finding.severity, semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::Severity::Error);
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
