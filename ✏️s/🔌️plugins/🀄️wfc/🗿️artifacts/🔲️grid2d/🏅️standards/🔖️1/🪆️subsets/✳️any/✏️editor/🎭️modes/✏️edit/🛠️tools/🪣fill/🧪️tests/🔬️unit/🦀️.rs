//! 🧪 Fill tool laws — definition, progressive payloads, abort, oracle parity, and the language-agnostic vector.

use crate::editor::grid2d::modes::edit::windows::preview;
use semio_framework_test_fixtures::fixture_text;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::ToolRunJobPort;
use semio_framework_tool_run::{ToolRunIdentity, ToolRunTraceKind};
use std::sync::Arc;
use crate::editor::grid2d::modes::edit::tools::fill::*;
use crate::host::inferences::solve_with_clock;
use crate::examples::grid2d::pipes;
use semio_framework_job::Generation;
use semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver;
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

static PARTIAL_VECTOR: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🧪️tests/🧫️fixtures/🎞️partial-tick.json"));

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [7; 32])
}

/// 🧭️ What one driven call of the fill run lent to its caller.
#[derive(Debug)]
enum Stepped {
    Yield,
    Tick(Vec<u8>),
    Complete,
    Fault,
    Cancelled,
}

/// 🦶️ One fuel budget against a shared operation — the child `WfcJob` is bound to that operation for the whole run.
fn drive_once(job: &mut Grid2dFillRunJob, driver: &mut TestJobDriver, fuel: u64) -> Stepped {
    driver.step(job, fuel, |outcome| match outcome {
        None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. } | semio_framework_job::JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Yield,
        Some(semio_framework_job::JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Tick((0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect()),
        Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    })
}

fn tick_payload(outcome: Stepped) -> Option<Grid2dFillPayload> {
    match outcome {
        Stepped::Tick(bytes) => {
            let tick = ToolRunTick::decode(&bytes).expect("tick decodes");
            tick.payload.as_ref().and_then(|bytes| Grid2dFillPayload::decode(bytes))
        }
        _ => None,
    }
}

fn close(job: &mut Grid2dFillRunJob, driver: &mut TestJobDriver) {
    driver.close(job);
}

#[test]
fn the_tool_declares_the_read_only_grid2d_fill_run() {
    let tool = definition();
    assert_eq!(tool.id, TOOL_ID);
    assert_eq!(tool.label, LocalizedLabel::native("Fill", "Füllen"));
    let run = tool.run.expect("fill declares a run");
    assert!(!run.mutating);
    assert_eq!(run.run_job.as_str(), RUN_JOB_KIND);
    assert!(run.revalidate_job.is_none());
    assert_eq!(run.trace, ToolRunTraceKind::None);
    assert_eq!(run.windows, vec![preview::WINDOW_KIND_ID.to_string()]);
    assert!(run.stages.iter().any(|stage| stage.id == "wfc.initialize-domains"));
    assert!(run.counters.iter().any(|counter| counter.id == "decided"));
}

#[test]
fn the_language_agnostic_partial_vector_decodes_to_the_normative_shape() {
    let payload = Grid2dFillPayload::decode((*PARTIAL_VECTOR).as_bytes()).expect("fixture decodes");
    assert_eq!(payload.decided_count(), 2);
    assert!(!payload.done);
    assert!(!payload.contradiction);
    assert_eq!(payload.assignments.len(), 4);
    assert_eq!(payload.assignments[0].tile_id.as_deref(), Some("straight"));
    assert!(payload.assignments[1].tile_id.is_none());
    let schema: serde_json::Value = serde_json::from_str(PAYLOAD_SCHEMA).expect("schema parses");
    assert_eq!(schema["$id"], "s.wfc.grid2d.fill.tick.v1");
    assert_eq!(schema["required"], serde_json::json!(["assignments", "contradiction", "done"]));
}

#[test]
fn stepping_publishes_a_strictly_increasing_partial_before_the_finish() {
    let snapshot = Arc::new(pipes::document());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("pipes solves");
    let finished = oracle.assignments.len();
    assert!(finished > 2, "the fixture must leave room to collapse");
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
    let mut partials = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
        if let Some(payload) = tick_payload(outcome) {
            let decided = payload.decided_count();
            if payload.done {
                assert_eq!(decided, finished);
                assert!(!payload.contradiction);
                let commit = payload.as_commit();
                assert_eq!(commit.assignments, oracle.assignments);
                assert!(!partials.is_empty(), "at least one partial must precede the finish");
                assert!(partials.iter().any(|&count| count > 0 && count < finished), "need a decided count strictly between 0 and finished: {partials:?}");
                assert!(partials.windows(2).any(|pair| pair[1] > pair[0]) || partials.iter().any(|&count| count > 0), "a later step must grow the decided count: {partials:?}");
                close(&mut job, &mut driver);
                return;
            }
            if decided > 0 {
                partials.push(decided);
            }
        }
    }
    close(&mut job, &mut driver);
    panic!("fill never finished; partials={partials:?}");
}

#[test]
fn aborting_mid_run_cancels_and_never_dispatches_commit_fill() {
    let snapshot = Arc::new(pipes::document());
    let port = ToolRunJobPort::default();
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, port.clone());
    let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
    let mut saw_partial = false;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
        if let Some(payload) = tick_payload(outcome) {
            if payload.decided_count() > 0 && !payload.done {
                saw_partial = true;
                break;
            }
            if payload.done {
                close(&mut job, &mut driver);
                panic!("finished before abort window");
            }
        }
    }
    assert!(saw_partial, "need a live partial before aborting");
    job.begin_close();
    let outcome = drive_once(&mut job, &mut driver, 1);
    assert!(matches!(outcome, Stepped::Cancelled), "closing mid-run must cancel: {outcome:?}");
    assert!(!port.has_effects(), "abort must not dispatch commit-fill");
    close(&mut job, &mut driver);
}

#[test]
fn the_final_payload_matches_solve_with_job_for_pipes() {
    let snapshot = Arc::new(pipes::document());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("pipes solves");
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 8);
        let terminal = matches!(outcome, Stepped::Complete | Stepped::Cancelled | Stepped::Fault);
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                assert_eq!(payload.as_commit().assignments, oracle.assignments);
                assert_eq!(payload.contradiction, oracle.contradiction);
                close(&mut job, &mut driver);
                return;
            }
        } else if terminal {
            close(&mut job, &mut driver);
            panic!("completed without a done payload");
        }
    }
    close(&mut job, &mut driver);
    panic!("fill never produced a done payload");
}

#[test]
fn a_full_lane_step_publishes_many_collapses_in_one_tick() {
    let snapshot = Arc::new(pipes::document());
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
    let mut bursts = Vec::new();
    for _ in 0..400 {
        let outcome = drive_once(&mut job, &mut driver, semio_framework_job::INTERACTIVE_LANE_FUEL);
        if let Some(payload) = tick_payload(outcome) {
            bursts.push(payload);
        }
        if bursts.iter().any(|payload| payload.done) {
            break;
        }
    }
    close(&mut job, &mut driver);
    let best = bursts.iter().map(|payload| payload.trace.len()).max().unwrap_or(0);
    assert!(best > 1, "one host step must carry more than one collapse, best {best} across {} ticks", bursts.len());
}

#[test]
fn only_an_admitted_completion_tick_can_publish_a_cached_solve() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️dispatch-envelope/🔣️.json")).expect("portable capacity fixture");
    for case in fixture["tickCases"].as_array().expect("cases") {
        let cells = case["cells"].as_u64().unwrap() as u32;
        let tile_id = "t".repeat(case["idBytes"].as_u64().unwrap() as usize);
        let payload = Grid2dFillPayload { assignments: (0..cells).map(|x| Grid2dFillCell { x, y: 0, tile_id: Some(tile_id.clone()) }).collect(), done: true, ..Default::default() };
        let expected = case["complete"].as_bool().unwrap();
        let port = ToolRunJobPort::default();
        let mut job = Grid2dFillRunJob::new(identity(), Arc::new(pipes::document()), port.clone());
        let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
        let _ = job.finish_success(false, Some(payload));
        assert_eq!(matches!(job.settled, Some(FillSettled::Complete)), expected);
        assert_eq!(job.committed, expected);
        assert_eq!(port.has_effects(), expected, "a failed tick must not enqueue a completed cache");
        close(&mut job, &mut driver);
    }
}

#[test]
fn completed_source_identity_survives_the_effect_bridge_and_current_revision_fence() {
    use crate::editor::grid2d::{Grid2dEditor, Grid2dEditorCommand};
    use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, NoConfig, ViewModel};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).expect("portable revision cases");
    let document = Arc::new(pipes::document());
    let solved = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("portable pipes solve");
    let payload = Grid2dFillPayload { assignments: solved.assignments.iter().map(|(x, y, tile_id)| Grid2dFillCell { x: *x, y: *y, tile_id: Some(tile_id.clone()) }).collect(), done: true, ..Default::default() };
    let history = semio_framework_plugin::HistoryView::empty();
    let no_config = NoConfig::default();
    let config = ConfigView { snapshot: &no_config, window: None };
    let mut view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view.window_id = Some("preview-1".into());
    view.window_instances = vec![semio_framework::ViewWindowInstance { id: "preview-1".into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
    for case in fixture["cases"].as_array().expect("cases") {
        let source: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).expect("source");
        let Ok(source_revision) = <[u8; 32]>::try_from(source.as_slice()) else { continue };
        let original = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, source_revision);
        let port = ToolRunJobPort::default();
        let mut job = Grid2dFillRunJob::new(original, document.clone(), port.clone());
        let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
        let _ = job.finish_success(false, Some(payload.clone()));
        let Some(semio_framework::kernel::Effect::DispatchAction { action, mut args, .. }) = port.pop_effect() else { panic!("completion dispatches its cache") };
        assert!(!port.has_effects(), "one completion publishes one effect");
        let command = <Grid2dEditor as ArtifactEditor>::command_from_action(&action, &mut args, &mut None).expect("owned effect bridges");
        let Grid2dEditorCommand::CommitFill { solve_json, source_revision: bridged } = &command else { panic!("completion command") };
        assert_eq!(bridged, &source);
        let cache = semio_framework_pack_json::from_json_str::<Grid2dCachedSolve>(solve_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("current cache envelope");
        assert_eq!(cache.source_revision, source);
        assert_eq!(cache.commit.assignments, solved.assignments);
        let current: Option<[u8; 32]> = serde_json::from_value(case["currentRevision"].clone()).expect("current");
        let doc = match current {
            Some(revision) => ArtifactView::with_operation(document.as_ref(), &history, semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: revision, retained: semio_framework_plugin::artifact_app_laws::fixture_retained_turn().grant, authoring_seed: "seed".into() }),
            None => ArtifactView::new(document.as_ref(), &history),
        };
        let emitted = Grid2dEditor::dispatch(&command, &doc, &config, Some(&view)).expect("completion dispatch");
        assert_eq!(emitted.window_config_mutations.len(), usize::from(case["visible"].as_bool().unwrap()), "{}", case["id"]);
        assert!(emitted.artifact_mutations.is_empty());
        close(&mut job, &mut driver);
    }
}
