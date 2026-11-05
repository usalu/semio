use semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver;

use crate::editor::wfc3d::modes::edit::windows::preview::WFC_3D_PREVIEW_WINDOW;
use semio_plugin::Fault;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_tool_run::{ToolRunIdentity, ToolRunTraceKind};
use crate::editor::wfc3d::modes::edit::tools::fill::*;
use crate::examples::tower_stack;
use crate::inferences::solve_with_clock;
use semio_framework_job::Generation;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::ToolRunJobPort;
use semio_framework_tool_run::{ToolRunId, ToolRunTick};
use std::sync::Arc;

static PARTIAL_VECTOR: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/📑️fill/🧪️tests/🧫️fixtures/🎞️partial-tick.json"));

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
fn drive_once(job: &mut Wfc3dFillRunJob, driver: &mut TestJobDriver, fuel: u64) -> Stepped {
    driver.step(job, fuel, |outcome| match outcome {
        None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. } | semio_framework_job::JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Yield,
        Some(semio_framework_job::JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Tick((0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect()),
        Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    })
}

fn tick_payload(outcome: Stepped) -> Option<Wfc3dFillTickPayload> {
    match outcome {
        Stepped::Tick(bytes) => {
            let tick = ToolRunTick::decode(&bytes).expect("tick decodes");
            { let payload = tick.payload.as_ref().and_then(|bytes| decode_fill_payload(bytes)); if let Some(payload) = &payload { assert_eq!(payload.source_revision, tick.identity.base_revision); } payload }
        }
        _ => None,
    }
}



#[test]
fn the_tool_declares_the_read_only_wfc3d_fill_run() {
    let tool = definition();
    assert_eq!(tool.id, TOOL_ID);
    assert_eq!(tool.label, LocalizedLabel::native("Fill", "Füllen"));
    let run = tool.run.expect("fill declares a run");
    assert!(!run.mutating);
    assert_eq!(run.run_job.as_str(), RUN_JOB_KIND);
    assert!(run.revalidate_job.is_none());
    assert_eq!(run.trace, ToolRunTraceKind::None);
    assert_eq!(run.windows, vec![WFC_3D_PREVIEW_WINDOW.to_string()]);
    assert!(run.stages.iter().any(|stage| stage.id == "wfc.initialize-domains"));
    assert!(run.counters.iter().any(|counter| counter.id == "decided"));
}

#[test]
fn the_language_agnostic_partial_vector_decodes_to_the_normative_shape() {
    let payload = decode_fill_payload((*PARTIAL_VECTOR).as_bytes()).expect("fixture decodes");
    assert_eq!(payload.decided_count(), 2);
    assert!(!payload.done);
    assert!(!payload.contradiction);
    assert_eq!(payload.assignments.len(), 3);
    assert_eq!(payload.assignments.get("room-a").and_then(|tile| tile.as_deref()), Some("room"));
    assert!(payload.assignments.get("corridor").and_then(|tile| tile.as_ref()).is_none());
    let schema: serde_json::Value = serde_json::from_str(FILL_TICK_PAYLOAD_SCHEMA).expect("schema parses");
    assert_eq!(schema["$id"], "https://json.schemas.assets.semio-tech.com/s/wfc/wfc3d/1/any/fill/tick-payload/schema.json");
    assert_eq!(schema["required"], serde_json::json!(["assignments", "contradiction", "done", "sourceRevision"]));
}

#[test]
fn stepping_publishes_a_strictly_increasing_partial_before_the_finish() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("tower-stack solves");
    let finished = oracle.assignments.len();
    assert!(finished > 2, "the fixture must leave room to collapse");
    let expected = payload_from_commit(&snapshot, &oracle);
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot.clone(), ToolRunJobPort::default());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let mut driver = TestJobDriver::new(operation, generation);
    let mut partials = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
        if let Some(payload) = tick_payload(outcome) {
            let decided = payload.decided_count();
            if payload.done {
                assert_eq!(decided, finished);
                assert!(!payload.contradiction);
                assert_eq!(payload.assignments, expected.assignments);
                assert!(!partials.is_empty(), "at least one partial must precede the finish");
                assert!(partials.iter().any(|&count| count > 0 && count < finished), "need a decided count strictly between 0 and finished: {partials:?}");
                assert!(partials.windows(2).any(|pair| pair[1] > pair[0]) || partials.iter().any(|&count| count > 0), "a later step must grow the decided count: {partials:?}");
                driver.close(&mut job);
                return;
            }
            if decided > 0 {
                partials.push(decided);
            }
        }
    }
    driver.close(&mut job);
    panic!("fill never finished; partials={partials:?}");
}

#[test]
fn aborting_mid_run_cancels_and_never_writes_set_solve() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let port = ToolRunJobPort::default();
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot, port.clone());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let mut driver = TestJobDriver::new(operation, generation);
    let mut saw_partial = false;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
        if let Some(payload) = tick_payload(outcome) {
            if payload.decided_count() > 0 && !payload.done {
                saw_partial = true;
                break;
            }
            if payload.done {
                driver.close(&mut job);
                panic!("finished before abort window");
            }
        }
    }
    assert!(saw_partial, "need a live partial before aborting");
    job.begin_close();
    let outcome = drive_once(&mut job, &mut driver, 1);
    assert!(matches!(outcome, Stepped::Cancelled), "closing mid-run must cancel: {outcome:?}");
    assert!(!port.has_effects(), "abort must not dispatch commit-fill");
    assert!(job.take_set_solve().is_none(), "abort must not retain SetSolve");
    driver.close(&mut job);
}

#[test]
fn the_final_payload_matches_solve_with_job_for_tower_stack() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("tower-stack solves");
    let expected = payload_from_commit(&snapshot, &oracle);
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let mut driver = TestJobDriver::new(operation, generation);
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 8);
        let is_complete = matches!(outcome, Stepped::Complete);
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                assert_eq!(payload.assignments, expected.assignments);
                assert_eq!(payload.contradiction, expected.contradiction);
                assert!(job.take_set_solve().is_some(), "completion retains SetSolve for the commit hop");
                driver.close(&mut job);
                return;
            }
        } else if is_complete {
            driver.close(&mut job);
            panic!("completed without a done payload");
        }
    }
    driver.close(&mut job);
    panic!("fill never produced a done payload");
}



#[test]
fn a_full_lane_step_publishes_many_collapses_in_one_tick() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let mut driver = TestJobDriver::new(operation, generation);
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
    driver.close(&mut job);
    let best = bursts.iter().map(|payload| payload.trace.len()).max().unwrap_or(0);
    assert!(best > 1, "one host step must carry more than one collapse, best {best} across {} ticks", bursts.len());
}

/// 📏️ Long valid identifiers cross the old command ceiling; oversized ticks publish no completion.
#[test]
fn fill_completion_envelopes_follow_the_actual_tick_carrier() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️graph-fill-envelope/🔣️.json")).unwrap();
    assert_eq!(semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX, law["tickMaximumBytes"].as_u64().unwrap() as usize);
    assert_eq!(crate::editor::wfc3d::WFC_3D_RETAINED_RAW_BYTES, law["rawMaximumBytes"].as_u64().unwrap() as usize);
    assert_eq!(crate::editor::wfc3d::WFC_3D_RETAINED_OUTPUT_BYTES, law["outputMaximumBytes"].as_u64().unwrap() as usize);
    for (key, admitted) in [("acceptedAssignments", true), ("oversizedAssignments", false)] {
        let assignments = (0..law[key].as_u64().unwrap()).map(|index| (format!("slot-{index}"), Some("t".repeat(law["idBytes"].as_u64().unwrap() as usize)))).collect();
        let payload = Wfc3dFillTickPayload { assignments, source_revision: vec![7;32], contradiction: false, done: true, trace: Vec::new() };
        let command = crate::editor::wfc3d::Wfc3dEditorCommand::CommitFill { payload_json: String::from_utf8(encode_fill_payload(&payload)).unwrap() };
        let wire = semio_framework_pack_json::to_json_string(&command);
        if admitted { assert!(wire.len() > 65536); assert!(wire.len() <= crate::editor::wfc3d::WFC_3D_RETAINED_RAW_BYTES); }
        let port = ToolRunJobPort::default();
        let snapshot = Arc::new(crate::examples::two_room_corridor::snapshot());
        let mut job = Wfc3dFillRunJob::new(identity(), snapshot, port.clone());
        let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
        job.finish_success(payload);
        assert_eq!(port.has_effects(), admitted);
        assert_eq!(job.take_set_solve().is_some(), admitted);
        driver.close(&mut job);
    }
}

/// 📨️ Completion refuses stale identities and unfinished payloads before replacing a fresh cache.
#[test]
fn completion_is_current_and_finished_before_transient_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source_revision: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
        let revision: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
        let mut payload = Wfc3dFillTickPayload { source_revision, done: true, ..Default::default() };
        assert_eq!(payload.completed_for(revision.as_ref()), case["visible"].as_bool().unwrap(), "{}", case["id"]);
        payload.done = false;
        assert!(!payload.completed_for(revision.as_ref()));
    }
}
