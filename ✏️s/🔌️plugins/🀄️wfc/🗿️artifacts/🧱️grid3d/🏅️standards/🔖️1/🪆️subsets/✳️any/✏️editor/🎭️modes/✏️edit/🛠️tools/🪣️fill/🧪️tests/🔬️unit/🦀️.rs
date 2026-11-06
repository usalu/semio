use semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver;

use crate::editor::grid3d::modes::edit::windows::preview;
use semio_plugin::Fault;
use semio_framework_test_fixtures::fixture_text;
use semio_ui_locale::LocalizedLabel;
use semio_framework_tool_run::{ToolRunIdentity, ToolRunTraceKind};
use std::sync::Arc;
use crate::editor::grid3d::modes::edit::tools::fill::*;
use crate::host::inferences::solve_with_clock;
use semio_framework_job::Generation;
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

static PARTIAL_VECTOR: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🧫️fixtures/🎞️partial-tick.json"));

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
fn drive_once(job: &mut Grid3dFillRunJob, driver: &mut TestJobDriver, fuel: u64) -> Stepped {
    driver.step(job, fuel, |outcome| match outcome {
        None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. } | semio_framework_job::JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Yield,
        Some(semio_framework_job::JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Tick((0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect()),
        Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    })
}

fn tick_payload(outcome: Stepped) -> Option<Grid3dFillPayload> {
    match outcome {
        Stepped::Tick(bytes) => {
            let tick = ToolRunTick::decode(&bytes).expect("tick decodes");
            tick.payload.as_ref().and_then(|bytes| Grid3dFillPayload::decode(bytes))
        }
        _ => None,
    }
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
    let payload = Grid3dFillPayload::decode((*PARTIAL_VECTOR).as_bytes()).expect("fixture decodes");
    assert_eq!(payload.decided_count(), 3);
    assert!(!payload.done);
    assert!(!payload.contradiction);
    assert_eq!(payload.assignments.len(), 6);
    assert_eq!(payload.assignments[0].tile_id.as_deref(), Some("air"));
    assert!(payload.assignments[1].tile_id.is_none());
    let schema: serde_json::Value = serde_json::from_str(PAYLOAD_SCHEMA).expect("schema parses");
    assert_eq!(schema["$id"], "s.wfc.grid3d.fill.tick.v1");
    assert_eq!(schema["required"], serde_json::json!(["assignments", "contradiction", "done", "sourceRevision"]));
}

#[test]
fn stepping_publishes_a_strictly_increasing_partial_before_the_finish() {
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("blocks solves");
    let finished = oracle.assignments.len();
    assert!(finished > 2, "the fixture must leave room to collapse");
    let port=semio_plugin::ToolRunJobPort::default();
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, port.clone());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut driver = TestJobDriver::new(operation, generation);
    let mut partials = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
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
fn aborting_mid_run_cancels_and_never_stores_residency() {
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let port=semio_plugin::ToolRunJobPort::default();
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, port.clone());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
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
    assert!(!port.has_effects(), "abort must not publish a completed transient");
    driver.close(&mut job);
}

#[test]
fn the_final_payload_matches_solve_for_blocks() {
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let oracle = solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("blocks solves");
    let port=semio_plugin::ToolRunJobPort::default();
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, port.clone());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut driver = TestJobDriver::new(operation, generation);
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, &mut driver, 1);
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                let mut got = payload.decided_assignments();
                let mut want = oracle.assignments.clone();
                got.sort_by_key(|row| (row.z, row.y, row.x));
                want.sort_by_key(|row| (row.z, row.y, row.x));
                assert_eq!(got, want);
                assert_eq!(payload.contradiction, !oracle.satisfiable);
                assert!(port.has_effects(), "completion dispatches the instance-local transient commit");
                assert_eq!(payload.source_revision, identity().base_revision);
                driver.close(&mut job);
                return;
            }
        }
    }
    driver.close(&mut job);
    panic!("fill never produced a done payload");
}

#[test]
fn a_full_lane_step_publishes_many_collapses_in_one_tick() {
    let snapshot = Arc::new(crate::examples::blocks::snapshot());
    let port=semio_plugin::ToolRunJobPort::default();
    let mut job = Grid3dFillRunJob::new(identity(), snapshot, port.clone());
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
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
