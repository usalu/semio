//! 🧪 Fill tool — payload shape, schema leaf, and interactive step progress.

use super::*;
use crate::schema::snapshot::{encode_base64, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel};
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, OperationId, StepBudget, StepOutcome};
use semio_framework_plugin::ToolRunJobPort;
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

fn stripes() -> BitmapSnapshot {
    let indices: Vec<u8> = (0..16u32).map(|cell| (cell % 2) as u8).collect();
    BitmapSnapshot {
        seed: 11,
        input: BitmapInput { width: 4, height: 4, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&indices) },
        output: BitmapOutputSpec { width: 6, height: 4, periodic: true },
        model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None },
        pinned: Vec::new(),
        ..BitmapSnapshot::default()
    }
}

fn odd_checkerboard(seed: u64) -> BitmapSnapshot {
    let indices: Vec<u8> = (0..16u32).map(|cell| ((cell % 4 + cell / 4) % 2) as u8).collect();
    BitmapSnapshot {
        seed,
        input: BitmapInput { width: 4, height: 4, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&indices) },
        output: BitmapOutputSpec { width: 5, height: 5, periodic: true },
        model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None },
        pinned: Vec::new(),
        ..BitmapSnapshot::default()
    }
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 7 }, [0; 32])
}

fn close_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
}

fn drive_once(job: &mut BitmapFillRunJob, operation: OperationId, generation: Generation, cancel: &semio_framework_job::CancelToken, sequence: &mut u64, fuel: u64) -> StepOutcome {
    let now = semio_framework_job::default_now_us().expect("clock");
    let budget = StepBudget::new(fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4);
    let mut verdict = None;
    semio_framework_job::drive_step(job, "wfc.bitmap.fill.run.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, sequence, &mut verdict)
}

fn tick_payload(outcome: StepOutcome) -> Option<BitmapFillPayload> {
    match outcome {
        StepOutcome::PreviewReady(mut payload) => {
            let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect();
            close_payload(&mut payload);
            let tick = ToolRunTick::decode(&bytes).ok()?;
            tick.payload.as_ref().and_then(|raw| std::str::from_utf8(raw).ok()).and_then(BitmapFillPayload::decode_json)
        }
        other => {
            let mut other = other;
            while !matches!(other.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
            None
        }
    }
}

fn close(job: &mut BitmapFillRunJob) {
    job.begin_close();
    for _ in 0..1_000_000 {
        match job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            InteractiveJobCloseStep::Complete => {
                assert!(job.terminal_is_empty());
                return;
            }
            _ => {}
        }
    }
    panic!("fill close ladder did not terminate");
}

#[test]
fn the_fill_tool_declares_the_contract_shape() {
    let run = run_definition();
    assert!(!run.mutating);
    assert_eq!(run.rebase, ToolRunRebasePolicy::Restart);
    assert_eq!(run.reconfigure, ToolRunReconfigurePolicy::Restart);
    assert_eq!(run.trace, ToolRunTraceKind::None);
    assert!(run.revalidate_job.is_none());
    assert_eq!(run.run_job.as_str(), RUN_JOB_KIND);
    assert!(run.windows.iter().any(|window| window.as_str() == crate::editor::bitmap::modes::edit::windows::output::WFC_BITMAP_WINDOW_OUTPUT));
    assert_eq!(definition().id, TOOL_ID);
}

#[test]
fn the_tick_payload_schema_leaf_is_normative_json_schema() {
    let schema = payload_schema_text();
    assert!(schema.contains(PAYLOAD_SCHEMA_ID));
    assert!(schema.contains("\"decided\""));
    assert!(schema.contains("\"pixels\""));
    let payload = payload_from_assignment(2, 2, &[(0, 0), (3, 1)], false, false);
    let encoded = payload.encode_json();
    assert_eq!(BitmapFillPayload::decode_json(&encoded).expect("round-trips"), payload);
    assert_eq!(payload.decided_count(), 2);
}

#[test]
fn stepping_publishes_a_partial_payload_then_a_richer_one() {
    let snapshot = stripes();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
    let (operation_id, generation) = (operation.operation, operation.generation);
    let mut job = BitmapFillRunJob::new(identity(), ToolRunJobPort::default(), snapshot, operation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut payloads = Vec::new();
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, 1);
        let terminal = matches!(outcome, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_));
        if let Some(payload) = tick_payload(outcome) {
            payloads.push(payload);
        }
        if terminal {
            break;
        }
    }
    close(&mut job);
    let finished = payloads.iter().rev().find(|payload| payload.done).expect("a finished payload");
    let finished_count = finished.decided_count();
    assert!(finished_count > 1, "fixture must have room to collapse");
    let partial_index = payloads.iter().position(|payload| !payload.done && payload.decided_count() > 0 && payload.decided_count() < finished_count).expect("partial");
    let later = payloads[partial_index + 1..].iter().any(|payload| payload.decided_count() > payloads[partial_index].decided_count());
    assert!(later || finished_count > payloads[partial_index].decided_count());
}

#[test]
fn the_final_payload_matches_solve_with_job() {
    let snapshot = stripes();
    let oracle = crate::inferences::solve_with_job(&snapshot).expect("oracle");
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
    let (operation_id, generation) = (operation.operation, operation.generation);
    let mut job = BitmapFillRunJob::new(identity(), ToolRunJobPort::default(), snapshot, operation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut finished = None;
    for _ in 0..500_000 {
        let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, 8);
        let terminal = matches!(outcome, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_));
        if let Some(payload) = tick_payload(outcome) {
            if payload.done {
                finished = Some(payload);
            }
        }
        if terminal {
            break;
        }
    }
    close(&mut job);
    let finished = finished.expect("finished");
    assert_eq!(finished.contradiction, oracle.contradiction);
    if oracle.contradiction {
        assert!(finished.pixels.is_empty() || finished.decided_count() == 0);
    } else {
        assert_eq!(finished.pixels, oracle.pixels);
    }
    let solve = finished.to_set_solve();
    assert_eq!(solve.contradiction, oracle.contradiction);
}

#[test]
fn aborting_mid_run_does_not_publish_set_solve() {
    let snapshot = stripes();
    let port = ToolRunJobPort::default();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
    let (operation_id, generation) = (operation.operation, operation.generation);
    let mut job = BitmapFillRunJob::new(identity(), port.clone(), snapshot, operation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut saw_partial = false;
    for _ in 0..20_000 {
        let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, 1);
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
    let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, 1);
    assert!(matches!(outcome, StepOutcome::Cancelled), "closing mid-run must cancel: {outcome:?}");
    let _ = tick_payload(outcome);
    assert!(!port.has_effects(), "abort must not dispatch commit-fill-solve / SetSolve");
    close(&mut job);
    assert!(!port.has_effects(), "close after abort still must not publish SetSolve");
    assert!(job.committed_solve().is_none());
}

#[test]
fn a_full_lane_step_publishes_a_burst_of_collapses_and_keeps_discards() {
    let snapshot = stripes();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
    let (operation_id, generation) = (operation.operation, operation.generation);
    let mut job = BitmapFillRunJob::new(identity(), ToolRunJobPort::default(), snapshot.clone(), operation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut bursts = Vec::new();
    for _ in 0..20 {
        let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, semio_framework_job::INTERACTIVE_LANE_FUEL);
        if let Some(payload) = tick_payload(outcome) {
            bursts.push(payload);
        }
        if bursts.iter().any(|payload| payload.done) {
            break;
        }
    }
    close(&mut job);
    assert!(bursts.iter().any(|payload| payload.trace.len() > 1), "one host step must carry more than one collapse");
    let mut discarded = None;
    for seed in 1..6 {
        let snapshot = odd_checkerboard(seed);
        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
        let (operation_id, generation) = (operation.operation, operation.generation);
        let mut job = BitmapFillRunJob::new(identity(), ToolRunJobPort::default(), snapshot.clone(), operation);
        let mut sequence = 0u64;
        let mut found = None;
        for _ in 0..40 {
            let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, semio_framework_job::INTERACTIVE_LANE_FUEL);
            if let Some(payload) = tick_payload(outcome) {
                if payload.trace.iter().any(|event| event.discarded) {
                    found = Some((snapshot, payload));
                    break;
                }
            }
        }
        close(&mut job);
        if found.is_some() {
            discarded = found;
            break;
        }
    }
    let (document, payload) = discarded.expect("an odd periodic checkerboard must discard a collapsed cell");
    let painted = crate::editor::bitmap::modes::edit::windows::output::layers_from_fill_payload(&document, &payload, &[]);
    let mut silent = payload.clone();
    silent.trace.clear();
    let bare = crate::editor::bitmap::modes::edit::windows::output::layers_from_fill_payload(&document, &silent, &[]);
    assert_ne!(painted, bare, "a discarded cell must stay on the canvas");
    assert!(bursts.iter().any(|payload| payload.trace.iter().any(|event| !event.discarded)), "a collapse must be in the trace");
    let document = snapshot;
    let with_trace = bursts.iter().find(|payload| payload.trace.iter().any(|event| event.discarded)).or_else(|| bursts.first());
    let payload = with_trace.expect("a burst");
    let painted = crate::editor::bitmap::modes::edit::windows::output::layers_from_fill_payload(&document, payload, &[]);
    let mut silent = payload.clone();
    silent.trace.clear();
    let bare = crate::editor::bitmap::modes::edit::windows::output::layers_from_fill_payload(&document, &silent, &[]);
    if payload.trace.iter().any(|event| event.discarded) {
        assert_ne!(painted, bare, "a discarded cell must change the paint");
    }
}

#[test]
fn rooms_16_paints_several_new_cells_in_one_host_step() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(0), snapshot.seed);
    let (operation_id, generation) = (operation.operation, operation.generation);
    let mut job = BitmapFillRunJob::new(identity(), ToolRunJobPort::default(), snapshot, operation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut previous = 0usize;
    let mut best_delta = 0usize;
    for _ in 0..48 {
        let outcome = drive_once(&mut job, operation_id, generation, &cancel, &mut sequence, semio_framework_job::INTERACTIVE_LANE_FUEL);
        if let Some(payload) = tick_payload(outcome) {
            let len = payload.trace.len();
            best_delta = best_delta.max(len.saturating_sub(previous));
            previous = len;
            if payload.done {
                break;
            }
        }
    }
    close(&mut job);
    assert!(best_delta > 1, "one host step on rooms-16 must paint more than one new cell, best {best_delta}");
}
