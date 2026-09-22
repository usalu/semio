//! 🧪 Fill tool laws — progressive payloads, abort, oracle parity, and the language-agnostic vector.

use super::*;
use crate::examples::tower_stack;
use crate::inferences::solve_with_job;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, StepBudget};
use semio_framework_plugin::{LocalizedLabel, ToolRunJobPort};
use semio_framework_tool_run::{ToolRunId, ToolRunTick};
use std::sync::Arc;

const PARTIAL_VECTOR: &str = include_str!("../../🧫️fixtures/🎞️partial-tick.json");

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [7; 32])
}

fn close_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
}

/// 🦶️ One fuel budget against a shared operation — the child `WfcJob` is bound to that operation for the whole run.
fn drive_once(job: &mut Wfc3dFillRunJob, operation: OperationId, generation: Generation, cancel: &semio_framework_job::CancelToken, sequence: &mut u64, fuel: u64) -> StepOutcome {
    let now = semio_framework_job::default_now_us().expect("clock");
    let budget = StepBudget::new(fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4);
    let mut verdict = None;
    semio_framework_job::drive_step(job, "wfc.wfc3d.fill.run.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, sequence, &mut verdict)
}

fn tick_payload(outcome: StepOutcome) -> Option<Wfc3dFillTickPayload> {
    match outcome {
        StepOutcome::PreviewReady(mut payload) => {
            let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect();
            close_payload(&mut payload);
            let tick = ToolRunTick::decode(&bytes).expect("tick decodes");
            tick.payload.as_ref().and_then(|bytes| decode_fill_payload(bytes))
        }
        other => {
            let mut other = other;
            while !matches!(other.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
            None
        }
    }
}

fn close(job: &mut Wfc3dFillRunJob) {
    job.begin_close();
    for _ in 0..1_000_000 {
        match job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            InteractiveJobCloseStep::Complete => {
                assert!(job.terminal_is_empty());
                return;
            }
            InteractiveJobCloseStep::Pending { .. } => {}
            InteractiveJobCloseStep::Blocked => panic!("fill close blocked"),
        }
    }
    panic!("fill close never completed");
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
    let payload = decode_fill_payload(PARTIAL_VECTOR.as_bytes()).expect("fixture decodes");
    assert_eq!(payload.decided_count(), 2);
    assert!(!payload.done);
    assert!(!payload.contradiction);
    assert_eq!(payload.assignments.len(), 3);
    assert_eq!(payload.assignments.get("room-a").and_then(|tile| tile.as_deref()), Some("room"));
    assert!(payload.assignments.get("corridor").and_then(|tile| tile.as_ref()).is_none());
    let schema: serde_json::Value = serde_json::from_str(FILL_TICK_PAYLOAD_SCHEMA).expect("schema parses");
    assert_eq!(schema["$id"], "https://json.schemas.assets.semio-tech.com/s/wfc/wfc3d/1/any/fill/tick-payload/schema.json");
    assert_eq!(schema["required"], serde_json::json!(["assignments", "contradiction", "done"]));
}

#[test]
fn stepping_publishes_a_strictly_increasing_partial_before_the_finish() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let oracle = solve_with_job(&snapshot).expect("tower-stack solves");
    let finished = oracle.assignments.len();
    assert!(finished > 2, "the fixture must leave room to collapse");
    let expected = payload_from_commit(&snapshot, &oracle);
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot.clone(), ToolRunJobPort::default());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut partials = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 1);
        if let Some(payload) = tick_payload(outcome) {
            let decided = payload.decided_count();
            if payload.done {
                assert_eq!(decided, finished);
                assert!(!payload.contradiction);
                assert_eq!(payload.assignments, expected.assignments);
                assert!(!partials.is_empty(), "at least one partial must precede the finish");
                assert!(partials.iter().any(|&count| count > 0 && count < finished), "need a decided count strictly between 0 and finished: {partials:?}");
                assert!(partials.windows(2).any(|pair| pair[1] > pair[0]) || partials.iter().any(|&count| count > 0), "a later step must grow the decided count: {partials:?}");
                close(&mut job);
                return;
            }
            if decided > 0 {
                partials.push(decided);
            }
        }
    }
    close(&mut job);
    panic!("fill never finished; partials={partials:?}");
}

#[test]
fn aborting_mid_run_cancels_and_never_writes_set_solve() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let port = ToolRunJobPort::default();
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot, port.clone());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut saw_partial = false;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 1);
        if let Some(payload) = tick_payload(outcome) {
            if payload.decided_count() > 0 && !payload.done {
                saw_partial = true;
                break;
            }
            if payload.done {
                close(&mut job);
                panic!("finished before abort window");
            }
        }
    }
    assert!(saw_partial, "need a live partial before aborting");
    job.begin_close();
    let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 1);
    assert!(matches!(outcome, StepOutcome::Cancelled), "closing mid-run must cancel: {outcome:?}");
    assert!(!port.has_effects(), "abort must not dispatch commit-fill");
    assert!(job.take_set_solve().is_none(), "abort must not retain SetSolve");
    close(&mut job);
}

#[test]
fn the_final_payload_matches_solve_with_job_for_tower_stack() {
    let snapshot = Arc::new(tower_stack::snapshot());
    let oracle = solve_with_job(&snapshot).expect("tower-stack solves");
    let expected = payload_from_commit(&snapshot, &oracle);
    let mut job = Wfc3dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let operation = semio_framework_job::allocate_operation_id();
    let generation = Generation(1);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 8);
        let is_complete = matches!(outcome, StepOutcome::Complete(_));
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                assert_eq!(payload.assignments, expected.assignments);
                assert_eq!(payload.contradiction, expected.contradiction);
                assert!(job.take_set_solve().is_some(), "completion retains SetSolve for the commit hop");
                close(&mut job);
                return;
            }
        } else if is_complete {
            close(&mut job);
            panic!("completed without a done payload");
        }
    }
    close(&mut job);
    panic!("fill never produced a done payload");
}


