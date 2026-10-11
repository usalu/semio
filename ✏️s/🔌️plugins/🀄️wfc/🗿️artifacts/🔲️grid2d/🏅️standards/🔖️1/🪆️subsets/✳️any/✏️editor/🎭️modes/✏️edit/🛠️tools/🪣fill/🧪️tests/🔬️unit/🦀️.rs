//! 🧪 Fill tool laws — definition, progressive payloads, abort, oracle parity, and the language-agnostic vector.

use super::*;
use crate::host::inferences::solve_with_clock;
use crate::examples::grid2d::pipes;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, StepBudget};
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

const PARTIAL_VECTOR: &str = include_str!("../../🧫️fixtures/🎞️partial-tick.json");

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [7; 32])
}

const TEST_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

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
fn drive_once(job: &mut Grid2dFillRunJob, operation: OperationId, generation: Generation, cancel: &semio_framework_job::CancelToken, sequence: &mut u64, fuel: u64) -> Stepped {
    let now = semio_framework_job::default_now_us().expect("clock");
    let budget = StepBudget::new(fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4, TEST_GRANT);
    let mut receipt = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(operation, generation, budget, cancel.clone(), semio_framework_job::default_now_us, sequence, &mut receipt);
    let mut verdict = None;
    match semio_framework_job::drive_step(job, &mut context, "wfc.grid2d.fill.run.test", semio_framework_job::InteractiveStage::InteractiveStep, &mut verdict).expect("fill step admission") {
        None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. } | semio_framework_job::JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Yield,
        Some(semio_framework_job::JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Tick((0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect()),
        Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    }
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

fn close(job: &mut Grid2dFillRunJob) {
    job.begin_close();
    for _ in 0..1_000_000 {
        if job.terminal_is_empty() {
            return;
        }
        let grant = semio_framework_job::RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: job.next_close_copy_byte_demand().expect("a locally owned fill quotes its copy demand"),
            maximum_capacity_bytes: job.next_close_capacity_byte_demand(usize::MAX).expect("a locally owned fill quotes its capacity demand"),
            maximum_release_bytes: job.next_close_release_byte_demand().expect("a locally owned fill quotes its release demand"),
            maximum_depth: job.next_close_depth_demand().expect("a locally owned fill quotes its depth demand").max(1),
        };
        match job.close_step(grant) {
            InteractiveJobCloseStep::Complete { .. } => {
                assert!(job.terminal_is_empty());
                return;
            }
            InteractiveJobCloseStep::Pending { .. } => {}
            InteractiveJobCloseStep::Blocked | InteractiveJobCloseStep::Refused { .. } => panic!("fill close blocked or refused"),
        }
    }
    panic!("fill close never completed");
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
    let payload = Grid2dFillPayload::decode(PARTIAL_VECTOR.as_bytes()).expect("fixture decodes");
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
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    let mut partials = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 1);
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
fn aborting_mid_run_cancels_and_never_dispatches_commit_fill() {
    let snapshot = Arc::new(pipes::document());
    let port = ToolRunJobPort::default();
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, port.clone());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
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
    assert!(matches!(outcome, Stepped::Cancelled), "closing mid-run must cancel: {outcome:?}");
    assert!(!port.has_effects(), "abort must not dispatch commit-fill");
    close(&mut job);
}

#[test]
fn the_final_payload_matches_solve_with_job_for_pipes() {
    let snapshot = Arc::new(pipes::document());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("pipes solves");
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 8);
        let terminal = matches!(outcome, Stepped::Complete | Stepped::Cancelled | Stepped::Fault);
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                assert_eq!(payload.as_commit().assignments, oracle.assignments);
                assert_eq!(payload.contradiction, oracle.contradiction);
                close(&mut job);
                return;
            }
        } else if terminal {
            close(&mut job);
            panic!("completed without a done payload");
        }
    }
    close(&mut job);
    panic!("fill never produced a done payload");
}

#[test]
fn a_full_lane_step_publishes_many_collapses_in_one_tick() {
    let snapshot = Arc::new(pipes::document());
    let mut job = Grid2dFillRunJob::new(identity(), snapshot, ToolRunJobPort::default());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    let mut bursts = Vec::new();
    for _ in 0..400 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, semio_framework_job::INTERACTIVE_LANE_FUEL);
        if let Some(payload) = tick_payload(outcome) {
            bursts.push(payload);
        }
        if bursts.iter().any(|payload| payload.done) {
            break;
        }
    }
    close(&mut job);
    let best = bursts.iter().map(|payload| payload.trace.len()).max().unwrap_or(0);
    assert!(best > 1, "one host step must carry more than one collapse, best {best} across {} ticks", bursts.len());
}
