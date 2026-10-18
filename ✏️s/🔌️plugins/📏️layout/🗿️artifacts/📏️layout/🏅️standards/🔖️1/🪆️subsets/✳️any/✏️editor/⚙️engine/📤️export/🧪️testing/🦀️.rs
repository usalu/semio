//! 🧪️ Test oracle of the layout export job: the original-caller batch driver and the headless export oracles built on it. See [`super`].

use super::*;
use semio_framework_job::{OriginalJobFixtureCaller, OriginalTurnFailure, RetainedCloneGrant, RetainedCloneStep};
use semio_framework_job_worker::{BatchJobSession, OriginalWorkerFixtureCaller, WorkerJobPoll};

/// 🎟️ The one stock every fixture caller of the batch driver is born with.
pub const LAYOUT_EXPORT_FIXTURE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX / 4, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };

/// 🎟️ The grant of one fixture turn: wide enough for a complete publication frontier, narrow enough to stay bounded.
pub const LAYOUT_EXPORT_FIXTURE_TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 32 };

/// 🚧️ How many turns this oracle gives one ladder before it calls the ladder broken. A close ladder that answers `Blocked` forever turns `while !terminal_is_empty()` into a livelock that the 30-minute test-binary watchdog turns into a lost binary; bounded here, a stuck ladder is a named failing test.
pub const LAYOUT_EXPORT_FIXTURE_TURN_BUDGET: usize = 100_000;

/// 🔭️ One owned observation of a borrowed step outcome: the payload bytes are copied out before the loan ends.
#[derive(Debug, PartialEq, Eq)]
pub enum LayoutExportObserved {
    Yield,
    Preview(Vec<u8>),
    Checkpoint { applied_progress: u64, state: Vec<u8> },
    Complete { state_bytes: usize, output_bytes: usize },
    Cancelled,
    Fault(String),
}

impl LayoutExportObserved {
    pub fn of_view(view: &semio_framework_job::JobOutcomeView<'_>) -> Self {
        use semio_framework_job::JobOutcomeView as View;
        match view {
            View::Yield { .. } => Self::Yield,
            View::PreviewReady { payload, .. } => Self::Preview(payload.single_page().unwrap_or_default().to_vec()),
            View::CheckpointReady { state, applied_progress, .. } => Self::Checkpoint { applied_progress: *applied_progress, state: state.single_page().unwrap_or_default().to_vec() },
            View::Complete { state, output, .. } => Self::Complete { state_bytes: state.map_or(0, |payload| payload.len()), output_bytes: output.map_or(0, |payload| payload.len()) },
            View::Cancelled { .. } => Self::Cancelled,
            View::Fault { detail, .. } => Self::Fault(detail.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("layout-export-fault").to_owned()),
        }
    }

    pub fn of_borrow(borrow: &JobOutcomeBorrow<'_>) -> Self {
        match borrow {
            JobOutcomeBorrow::Yield { .. } => Self::Yield,
            JobOutcomeBorrow::PreviewReady { payload, .. } => Self::Preview(payload.single_page().unwrap_or_default().to_vec()),
            JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. } => Self::Checkpoint { applied_progress: *applied_progress, state: state.single_page().unwrap_or_default().to_vec() },
            JobOutcomeBorrow::Complete { state, output, .. } => Self::Complete { state_bytes: state.map_or(0, |payload| payload.len()), output_bytes: output.map_or(0, |payload| payload.len()) },
            JobOutcomeBorrow::Cancelled { .. } => Self::Cancelled,
            JobOutcomeBorrow::Fault { detail, .. } => Self::Fault(detail.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("layout-export-fault").to_owned()),
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete { .. } | Self::Cancelled | Self::Fault(_))
    }
}

/// 🎟️ Admits a fixture caller whose identity is the exact operation and generation a job will run under.
pub fn layout_export_fixture_caller(operation: Operation, generation: Generation) -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(operation.operation, generation, LAYOUT_EXPORT_FIXTURE_STOCK, semio_framework_job::default_now_us, u64::MAX, u64::MAX)
}

/// 🏃️ Drives one job to its terminal outcome through an admitted batch session, step by step on one original caller.
/// `at_terminal` runs between the terminal outcome and the session close: the close ladder drains the job's shared output queue, so a law that reads a job's exported bytes takes them there, exactly where production's download and media wrappers take ownership of the sealed queue before the job closes.
pub fn drive_job_batch<J: InteractiveJob + 'static, R>(job: J, operation: Operation, generation: Generation, cancel_before_first_step: bool, site: &'static str, at_terminal: impl FnOnce() -> R) -> (LayoutExportObserved, R) {
    const TURN: RetainedCloneGrant = LAYOUT_EXPORT_FIXTURE_TURN;
    let mut at_terminal = Some(at_terminal);
    let mut caller = layout_export_fixture_caller(operation, generation);
    let mut job = Some(job);
    let mut params = Some(BatchJobParams {
        operation: operation.operation,
        generation,
        cancel: caller.admit_alias(TURN),
        now_us: semio_framework_job::default_now_us,
        config: BatchDriveConfig { retained: TURN, site, stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
    });
    let admitted = caller.turn(TURN, |cx| BatchJobSession::try_admit_owned(&mut job, &mut params, cx).map_err(OriginalTurnFailure::from)).expect("layout export fixture admission turn");
    let Some((mut session, _posted)) = admitted else {
        let mut unadmitted = job.take().expect("the refused admission leaves the exact job with the caller");
        InteractiveJob::begin_close(&mut unadmitted);
        let mut turns = 0usize;
        while !InteractiveJob::terminal_is_empty(&unadmitted) {
            turns += 1;
            assert!(turns < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the refused admission's close ladder never reached terminal-empty");
            caller.turn(TURN, |cx| InteractiveJob::close_step(&mut unadmitted, cx)).expect("layout export fixture refused close turn");
        }
        panic!("layout export fixture admits its retained session");
    };
    if cancel_before_first_step {
        caller.cancel().cancel_now();
    }
    let mut turns = 0usize;
    let (terminal, value) = loop {
        turns += 1;
        assert!(turns < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the layout export fixture never reached a terminal outcome");
        let poll = caller.turn(TURN, |cx| session.step_on_original_caller(cx)).expect("layout export fixture step turn");
        if !matches!(poll, WorkerJobPoll::Outcome | WorkerJobPoll::Terminal) || !session.checkout_outcome() {
            continue;
        }
        assert!(session.take_checked_out_retained_step_receipt().is_some(), "the checked-out outcome carries its paid step receipt");
        let observed = LayoutExportObserved::of_view(&session.checked_out_outcome().expect("layout export fixture outcome view").expect("a checked-out outcome"));
        let mut acknowledged = false;
        for _ in 0..16 {
            let step = caller
                .turn(TURN, |cx| {
                    let step = session.acknowledge_outcome(cx.retained_grant());
                    cx.consume_retained(step.progress())?;
                    Ok(step)
                })
                .expect("layout export fixture acknowledgement turn");
            if matches!(step, RetainedCloneStep::Complete(_)) {
                acknowledged = true;
                break;
            }
        }
        assert!(acknowledged, "the paid acknowledgement completes inside its bounded turns");
        if observed.is_terminal() {
            break (observed, (at_terminal.take().expect("one terminal outcome per session"))());
        }
        session.resume().expect("layout export fixture resumes the exact owner");
    };
    session.begin_close();
    let mut turns = 0usize;
    while !session.terminal_is_empty() {
        turns += 1;
        assert!(turns < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the session's close ladder never reached terminal-empty: it answers Blocked or releases nothing on every turn");
        caller.close_owner(TURN, &mut session).expect("layout export fixture close turn");
    }
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    (terminal, value)
}

/// 🧪️ Runs one export request through the batch driver and answers its committed output or the terminal refusal.
pub fn run_layout_export_headless_batch(operation: Operation, request: LayoutExportRequest) -> Result<LayoutExportCommit, String> {
    let kind = request.kind;
    let name = output_name(&request);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let job = LayoutExportJob::from_admitted(operation, (operation.operation, operation.generation), request, ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
    let output_chunks = job.output_chunks.as_ref().expect("a fresh export job owns its output queue").clone();
    let (observed, commit) = drive_job_batch(job, operation, operation.generation, false, "layout.export.batch", || LayoutExportCommit::from_chunks(kind, &name, &output_chunks));
    drop(output_chunks);
    drop(snapshot_owner);
    match observed {
        LayoutExportObserved::Complete { .. } => commit,
        LayoutExportObserved::Cancelled => Err("layout-export-cancelled".into()),
        LayoutExportObserved::Fault(detail) => Err(detail),
        other => Err(format!("layout-export-unexpected-terminal:{other:?}")),
    }
}

/// 🧪️ The headless export of one snapshot under a fresh operation.
pub fn headless_batch_export(kind: LayoutExportKind, snapshot: &LayoutSnapshot, page_id: Option<&str>, preflight_json: Option<&str>) -> Result<LayoutExportCommit, String> {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(0), Generation(0), 0);
    run_layout_export_headless_batch(
        operation,
        LayoutExportRequest {
            kind,
            page_id: page_id.map(str::to_owned),
            snapshot: Arc::new(snapshot.clone()),
            preflight_json: preflight_json.map(str::to_owned),
            parent_document_id: "layout.headless-batch".into(),
            canonical_base_revision_hex: "0000000000000000000000000000000000000000000000000000000000000000".into(),
        },
    )
}

pub fn export_document_svg_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<String, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    headless_batch_export(LayoutExportKind::Svg, doc, Some(page_id), None).map(|commit| commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

pub fn export_document_pdf_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Pdf, doc, Some(page_id), None).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

pub fn export_document_png_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Png, doc, Some(page_id), None).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

pub fn export_package_zip_headless_batch(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    let snapshot: LayoutSnapshot = semio_framework_pack_json::from_json_str(doc_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let commit = headless_batch_export(LayoutExportKind::Package, &snapshot, None, Some(preflight_json)).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}
