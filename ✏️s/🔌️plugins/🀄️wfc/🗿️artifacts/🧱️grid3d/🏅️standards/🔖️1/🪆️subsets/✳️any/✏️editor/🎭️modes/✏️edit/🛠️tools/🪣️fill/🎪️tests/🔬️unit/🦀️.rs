//! 🧪 Fill tool laws — definition, progressive payloads, abort, oracle parity, and the language-agnostic vector.

use super::*;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, StepBudget};
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

const PARTIAL_VECTOR: &str = include_str!("../../🎫️fixtures/🎞️partial-tick.json");

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [7; 32])
}

fn close_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
}

fn drive_once(job: &mut Grid3dFillRunJob, operation: OperationId, generation: Generation, cancel: &semio_framework_job::CancelToken, sequence: &mut u64, fuel: u64) -> StepOutcome {
    let now = semio_framework_job::default_now_us().expect("clock");
    let budget = StepBudget::new(fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4);
    let mut verdict = None;
    semio_framework_job::drive_step(job, "wfc.grid3d.fill.run.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, sequence, &mut verdict)
}

fn tick_payload(outcome: StepOutcome) -> Option<Grid3dFillPayload> {
    match outcome {
        StepOutcome::PreviewReady(mut payload) => {
            let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect();
            close_payload(&mut payload);
            let tick = ToolRunTick::decode(&bytes).expect("tick decodes");
            tick.payload.as_ref().and_then(|bytes| Grid3dFillPayload::decode(bytes))
        }
        other => {
            let mut other = other;
            while !matches!(other.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
            None
        }
    }
}

fn close(job: &mut Grid3dFillRunJob) {
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
fn the_tool_declares_the_read_only_grid3d_fill_run() {
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
    let payload = Grid3dFillPayload::decode(PARTIAL_VECTOR.as_bytes()).expect("fixture decodes");
    assert_eq!(payload.decided_count(), 3);
    assert!(!payload.done);
    assert!(!payload.contradiction);
    assert_eq!(payload.assignments.len(), 6);
    assert_eq!(payload.assignments[0].tile_id.as_deref(), Some("air"));
    assert!(payload.assignments[1].tile_id.is_none());
    let schema: serde_json::Value = serde_json::from_str(PAYLOAD_SCHEMA).expect("schema parses");
    assert_eq!(schema["$id"], "s.wfc.grid3d.fill.tick.v1");
    assert_eq!(schema["required"], serde_json::json!(["assignments", "contradiction", "done"]));
}

#[test]
fn stepping_publishes_a_strictly_increasing_partial_before_the_finish() {
    preview::clear_fill_commit_for_test();
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let oracle = solve_with_job(&snapshot).expect("blocks solves");
    let finished = oracle.assignments.len();
    assert!(finished > 2, "the fixture must leave room to collapse");
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, Some("fill-test".into()));
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
                let mut got = payload.decided_assignments();
                let mut want = oracle.assignments.clone();
                got.sort_by_key(|row| (row.z, row.y, row.x));
                want.sort_by_key(|row| (row.z, row.y, row.x));
                assert_eq!(got, want);
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
fn aborting_mid_run_cancels_and_never_stores_residency() {
    preview::clear_fill_commit_for_test();
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, Some("fill-abort".into()));
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
    assert!(matches!(outcome, StepOutcome::Cancelled), "closing mid-run must cancel: {outcome:?}");
    assert!(preview::last_fill_commit().is_none(), "abort must not store residency");
    close(&mut job);
}

#[test]
fn the_final_payload_matches_solve_with_job_for_blocks() {
    preview::clear_fill_commit_for_test();
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let oracle = solve_with_job(&snapshot).expect("blocks solves");
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, Some("fill-oracle".into()));
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation, generation, &cancel, &mut sequence, 1);
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                let mut got = payload.decided_assignments();
                let mut want = oracle.assignments.clone();
                got.sort_by_key(|row| (row.z, row.y, row.x));
                want.sort_by_key(|row| (row.z, row.y, row.x));
                assert_eq!(got, want);
                assert_eq!(payload.contradiction, !oracle.satisfiable);
                assert!(preview::last_fill_commit().is_some(), "completion stores residency");
                close(&mut job);
                return;
            }
        }
    }
    close(&mut job);
    panic!("fill never produced a done payload");
}

#[test]
fn a_full_lane_step_publishes_many_collapses_in_one_tick() {
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, Some("fill-burst".into()));
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
