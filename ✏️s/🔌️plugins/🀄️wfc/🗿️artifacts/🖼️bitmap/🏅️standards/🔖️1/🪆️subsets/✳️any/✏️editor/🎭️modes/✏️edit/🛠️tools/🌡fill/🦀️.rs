//! 🌡 Bitmap fill — non-mutating framework tool run that collapses the overlapping model one
//! engine step at a time. The output window paints each tick's payload (`decided` mask keeps palette
//! index 0 a real colour); `SetSolve` is published only when the run commits.

use crate::editor::bitmap::modes::edit::windows::output;
use crate::editor::bitmap::transient::SetSolve;
use crate::schema::snapshot::{decode_base64, encode_base64, BitmapSnapshot};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobPayloadStream, StepContext, StepOutcome};
use semio_framework_plugin::{Effect, EditorApp, Fault, LocalizedLabel, RequestId, ToolDefinition, ToolRunJobRequest};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunStageDefinition, ToolRunState, ToolRunStepArg,
    ToolRunStepKind, ToolRunTickWriter, ToolRunTraceKind, ToolRunVerdict,
};
use semio_s_plugin_wfc_engine as engine;
use std::sync::Arc;

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const RUN_JOB_KIND: &str = "s.wfc.bitmap.fill.run";
pub const PAYLOAD_SCHEMA_ID: &str = "s.wfc.bitmap.fill.tick.payload.v1";
pub const COMMIT_SOLVE_ACTION_ID: &str = "commit-fill-solve";
const COMMIT_SOLVE_REQUEST: u64 = 0xbf11_5001;
const FILL_PAYLOAD_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
//#endregion 🔖️Constants

//#region 🎞Payload
/// 🎞 Partial or finished bitmap collapse carried in `ToolRunTick::payload`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitmapFillPayload {
    pub pixels: String,
    pub decided: String,
    pub width: u32,
    pub height: u32,
    pub contradiction: bool,
    pub done: bool,
}

impl BitmapFillPayload {
    /// 🧮 Number of cells whose `decided` byte is non-zero.
    pub fn decided_count(&self) -> usize {
        decode_base64(&self.decided).map_or(0, |mask| mask.iter().filter(|byte| **byte != 0).count())
    }

    /// 📤 Encode as the normative JSON object.
    pub fn encode_json(&self) -> String {
        format!(
            "{{\"pixels\":{},\"decided\":{},\"width\":{},\"height\":{},\"contradiction\":{},\"done\":{}}}",
            protocol::json::to_json_string(&self.pixels),
            protocol::json::to_json_string(&self.decided),
            self.width,
            self.height,
            if self.contradiction { "true" } else { "false" },
            if self.done { "true" } else { "false" },
        )
    }

    /// 📥 Decode the normative JSON object.
    pub fn decode_json(text: &str) -> Option<Self> {
        let value: protocol::json::Value = protocol::json::from_json_str(text).ok()?;
        let object = value.as_object()?;
        Some(Self {
            pixels: object.get("pixels")?.as_str()?.to_owned(),
            decided: object.get("decided")?.as_str()?.to_owned(),
            width: object.get("width")?.as_u64()? as u32,
            height: object.get("height")?.as_u64()? as u32,
            contradiction: object.get("contradiction")?.as_bool()?,
            done: object.get("done")?.as_bool()?,
        })
    }

    /// 🗜 The SetSolve transient this finished payload writes on commit.
    pub fn to_set_solve(&self) -> SetSolve {
        SetSolve {
            output_pixels: if self.contradiction || self.pixels.is_empty() { None } else { Some(self.pixels.clone()) },
            contradiction: self.contradiction,
            output_width: self.width,
            output_height: self.height,
        }
    }

    /// 🖼 Palette indices plus a decided mask for rendering.
    pub fn render_indices(&self) -> (Vec<u8>, Vec<bool>) {
        let pixels = decode_base64(&self.pixels).unwrap_or_default();
        let decided = decode_base64(&self.decided).unwrap_or_default();
        let cells = (self.width as usize).saturating_mul(self.height as usize);
        let mut mask = vec![false; cells];
        let mut indices = vec![0u8; cells];
        for index in 0..cells {
            let is_decided = decided.get(index).copied().unwrap_or(0) != 0;
            mask[index] = is_decided;
            if is_decided {
                indices[index] = pixels.get(index).copied().unwrap_or(0);
            }
        }
        (indices, mask)
    }
}

/// 📜 Normative JSON Schema leaf for the tick payload.
pub fn payload_schema_text() -> &'static str {
    FILL_PAYLOAD_SCHEMA
}

/// 🏗 Build a payload from an in-process assignment map.
pub fn payload_from_assignment(width: u32, height: u32, cells: &[(usize, u8)], contradiction: bool, done: bool) -> BitmapFillPayload {
    let total = (width as usize).saturating_mul(height as usize);
    let mut pixels = vec![0u8; total];
    let mut decided = vec![0u8; total];
    for &(index, color) in cells {
        if index < total {
            pixels[index] = color;
            decided[index] = 1;
        }
    }
    BitmapFillPayload { pixels: encode_base64(&pixels), decided: encode_base64(&decided), width, height, contradiction, done }
}
//#endregion 🎞Payload

//#region 🔖️Definition
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillStage {
    InitializeDomains,
    FindMinimumEntropySlot,
    ChooseCandidate,
    PropagateCompatibilityEdge,
    DetectContradiction,
    BacktrackTrailEntry,
    CommitSlot,
    MaterializeCheckpoint,
    MaterializeCommit,
    Complete,
}

impl FillStage {
    const ALL: [Self; 10] = [
        Self::InitializeDomains,
        Self::FindMinimumEntropySlot,
        Self::ChooseCandidate,
        Self::PropagateCompatibilityEdge,
        Self::DetectContradiction,
        Self::BacktrackTrailEntry,
        Self::CommitSlot,
        Self::MaterializeCheckpoint,
        Self::MaterializeCommit,
        Self::Complete,
    ];

    fn index(self) -> u16 {
        self as u16
    }

    fn id(self) -> &'static str {
        match self {
            Self::InitializeDomains => "wfc.initialize-domains",
            Self::FindMinimumEntropySlot => "wfc.find-minimum-entropy-slot",
            Self::ChooseCandidate => "wfc.choose-candidate",
            Self::PropagateCompatibilityEdge => "wfc.propagate-compatibility-edge",
            Self::DetectContradiction => "wfc.detect-contradiction",
            Self::BacktrackTrailEntry => "wfc.backtrack-trail-entry",
            Self::CommitSlot => "wfc.commit-slot",
            Self::MaterializeCheckpoint => "wfc.materialize-checkpoint",
            Self::MaterializeCommit => "wfc.materialize-commit",
            Self::Complete => "wfc.complete",
        }
    }

    fn label(self) -> LocalizedLabel {
        match self {
            Self::InitializeDomains => LocalizedLabel::native("Initializing domains", "Domänen werden initialisiert"),
            Self::FindMinimumEntropySlot => LocalizedLabel::native("Finding minimum entropy", "Minimale Entropie wird gesucht"),
            Self::ChooseCandidate => LocalizedLabel::native("Choosing a candidate", "Kandidat wird gewählt"),
            Self::PropagateCompatibilityEdge => LocalizedLabel::native("Propagating compatibility", "Kompatibilität wird propagiert"),
            Self::DetectContradiction => LocalizedLabel::native("Detecting contradiction", "Widerspruch wird erkannt"),
            Self::BacktrackTrailEntry => LocalizedLabel::native("Backtracking", "Zurückverfolgen"),
            Self::CommitSlot => LocalizedLabel::native("Committing a slot", "Slot wird festgelegt"),
            Self::MaterializeCheckpoint => LocalizedLabel::native("Materializing checkpoint", "Prüfpunkt wird materialisiert"),
            Self::MaterializeCommit => LocalizedLabel::native("Materializing commit", "Commit wird materialisiert"),
            Self::Complete => LocalizedLabel::native("Complete", "Abgeschlossen"),
        }
    }

    fn of(stage: engine::job::WfcStage) -> Self {
        match stage {
            engine::job::WfcStage::InitializeDomains => Self::InitializeDomains,
            engine::job::WfcStage::FindMinimumEntropySlot => Self::FindMinimumEntropySlot,
            engine::job::WfcStage::ChooseCandidate => Self::ChooseCandidate,
            engine::job::WfcStage::PropagateCompatibilityEdge => Self::PropagateCompatibilityEdge,
            engine::job::WfcStage::DetectContradiction => Self::DetectContradiction,
            engine::job::WfcStage::BacktrackTrailEntry => Self::BacktrackTrailEntry,
            engine::job::WfcStage::CommitSlot => Self::CommitSlot,
            engine::job::WfcStage::MaterializeCheckpoint => Self::MaterializeCheckpoint,
            engine::job::WfcStage::MaterializeCommit => Self::MaterializeCommit,
            engine::job::WfcStage::Complete => Self::Complete,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillCounter {
    Observations,
    Decided,
    Backtracks,
}

impl FillCounter {
    const ALL: [Self; 3] = [Self::Observations, Self::Decided, Self::Backtracks];

    fn index(self) -> u16 {
        self as u16
    }

    fn id(self) -> &'static str {
        match self {
            Self::Observations => "observations",
            Self::Decided => "decided",
            Self::Backtracks => "backtracks",
        }
    }

    fn label(self) -> LocalizedLabel {
        match self {
            Self::Observations => LocalizedLabel::native("Observations", "Beobachtungen"),
            Self::Decided => LocalizedLabel::native("Decided cells", "Festgelegte Zellen"),
            Self::Backtracks => LocalizedLabel::native("Backtracks", "Rückschritte"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillReason {
    Collapsed,
    Contradiction,
    Cancelled,
    Fault,
}

impl FillReason {
    const ALL: [Self; 4] = [Self::Collapsed, Self::Contradiction, Self::Cancelled, Self::Fault];

    fn code(self) -> u16 {
        self as u16
    }

    fn id(self) -> &'static str {
        match self {
            Self::Collapsed => "collapsed",
            Self::Contradiction => "contradiction",
            Self::Cancelled => "cancelled",
            Self::Fault => "fault",
        }
    }

    fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Collapsed | Self::Contradiction => ToolRunVerdict::Success,
            Self::Cancelled => ToolRunVerdict::Warning,
            Self::Fault => ToolRunVerdict::Danger,
        }
    }

    fn template(self) -> LocalizedLabel {
        match self {
            Self::Collapsed => LocalizedLabel::native("Collapsed", "Kollabiert"),
            Self::Contradiction => LocalizedLabel::native("Contradiction", "Widerspruch"),
            Self::Cancelled => LocalizedLabel::native("Cancelled", "Abgebrochen"),
            Self::Fault => LocalizedLabel::native("Fault", "Fehler"),
        }
    }
}

/// 🛠️ Stitched into the editor manifest by `crate::editor::bitmap::create_bitmap_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Fill", "Füllen"), "paint-bucket")) }
}

/// ⏯️ Read-only collapse: restart on base or settings change, no provisional document ops.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: LocalizedLabel::native("steps", "Schritte"),
        stages: FillStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: stage.label() }).collect(),
        counters: FillCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() }).collect(),
        reasons: FillReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::None,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: None,
        settings: Default::default(),
        windows: vec![output::WFC_BITMAP_WINDOW_OUTPUT.into()],
    }
}

/// 🚀 Host effect that starts this fill run from the Solve action.
pub fn start_fill_effect() -> Effect {
    Effect::DispatchAction {
        req: RequestId(COMMIT_SOLVE_REQUEST ^ 1),
        action: semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.into(),
        args: Some(dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::object([(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID.to_string(), TOOL_ID.into())]))),
        delay_ms: 0,
    }
}
//#endregion 🔖️Definition

//#region 🧵️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PrepStage {
    Compile,
    Child,
    Done,
}

/// ⏯️ Interactive fill run: one prep unit, then one `WfcJob::step` per framework step.
pub struct BitmapFillRunJob {
    operation: semio_framework_job::Operation,
    snapshot: Arc<BitmapSnapshot>,
    identity: ToolRunIdentity,
    port: semio_framework_plugin::ToolRunJobPort,
    writer: ToolRunTickWriter,
    prep: PrepStage,
    parts: Option<crate::inferences::BitmapCollapseParts>,
    child: Option<engine::job::WfcJob<engine::grid2d::Grid2dTopology>>,
    last_payload: Option<BitmapFillPayload>,
    committed_solve: Option<SetSolve>,
    pending_tick: bool,
    pending_finish: Option<(bool, BitmapFillPayload)>,
    closing: bool,
}

impl BitmapFillRunJob {
    /// 🏗 Build from a tool-run request.
    pub fn from_request(request: ToolRunJobRequest<'_, EditorApp<crate::editor::bitmap::BitmapEditor>>) -> Result<Self, Fault> {
        Ok(Self::new(request.identity, request.port.clone(), request.snapshot.as_ref().clone(), semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(u64::from(request.identity.generation)), request.snapshot.seed)))
    }

    /// 🏗 Build a fresh run over `snapshot`.
    pub fn new(identity: ToolRunIdentity, port: semio_framework_plugin::ToolRunJobPort, snapshot: BitmapSnapshot, operation: semio_framework_job::Operation) -> Self {
        Self {
            operation,
            snapshot: Arc::new(snapshot),
            identity,
            port,
            writer: ToolRunTickWriter::new(identity),
            prep: PrepStage::Compile,
            parts: None,
            child: None,
            last_payload: None,
            committed_solve: None,
            pending_tick: false,
            pending_finish: None,
            closing: false,
        }
    }

    /// 👀 Last tick payload published by this job.
    pub fn last_payload(&self) -> Option<&BitmapFillPayload> {
        self.last_payload.as_ref()
    }

    /// 🗜 SetSolve produced on a successful commit — `None` after abort or before commit.
    pub fn committed_solve(&self) -> Option<&SetSolve> {
        self.committed_solve.as_ref()
    }

    fn publish_tick(&mut self, context: &mut StepContext<'_>, payload: BitmapFillPayload, stage: FillStage, reason: FillReason, state: ToolRunState) -> StepOutcome {
        let decided = payload.decided_count() as u64;
        let observations = self.child.as_ref().map(|job| job.preview(0).observations).unwrap_or(0);
        let backtracks = self.child.as_ref().map(|job| job.preview(0).backtracks).unwrap_or(0);
        let total = (self.snapshot.output.width as u64).saturating_mul(self.snapshot.output.height as u64);
        let counters = vec![
            ToolRunCounter { counter: FillCounter::Observations.index(), value: observations },
            ToolRunCounter { counter: FillCounter::Decided.index(), value: decided },
            ToolRunCounter { counter: FillCounter::Backtracks.index(), value: backtracks },
        ];
        let progress = ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: stage.index(),
            completed: decided,
            total: Some(total),
            counters,
            units_per_second: 0.0,
            conflicts: 0,
            steps: semio_framework_tool_run::ToolRunStepRing::new(),
        };
        let _ = self.writer.step(ToolRunStepKind::Info, stage.index(), reason.code(), None, &[ToolRunStepArg::Unsigned(decided)]);
        self.writer.progress(progress);
        self.writer.payload(payload.encode_json().into_bytes());
        self.last_payload = Some(payload);
        let Some(tick) = self.writer.finish() else {
            return StepOutcome::Yield;
        };
        let Ok(bytes) = tick.encode() else {
            return StepOutcome::Yield;
        };
        match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
            Ok(retained) => StepOutcome::PreviewReady(retained),
            Err(rejected) => {
                drop(rejected.into_source());
                StepOutcome::Yield
            }
        }
    }

    fn payload_from_child(&self, contradiction: bool, done: bool) -> Option<BitmapFillPayload> {
        let child = self.child.as_ref()?;
        let parts = self.parts.as_ref()?;
        let width = self.snapshot.output.width;
        let height = self.snapshot.output.height;
        let cells = (width as usize).saturating_mul(height as usize);
        let mut assignment = Vec::new();
        if done && !contradiction {
            if let Some(commit) = child.commit() {
                for (index, pattern) in commit.assignment.iter().enumerate() {
                    let color = parts.decoder.anchor_tile(engine::ids::PatternId(*pattern)).get() as u8;
                    assignment.push((index, color));
                }
                return Some(payload_from_assignment(width, height, &assignment, false, true));
            }
        }
        for &(node, pattern) in child.observed() {
            let index = node.index();
            if index < cells {
                let color = parts.decoder.anchor_tile(pattern).get() as u8;
                assignment.push((index, color));
            }
        }
        Some(payload_from_assignment(width, height, &assignment, contradiction, done))
    }

    fn commit_solve(&mut self, payload: &BitmapFillPayload) {
        let solve = payload.to_set_solve();
        self.committed_solve = Some(solve.clone());
        let args = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::object([
            ("pixels".to_string(), solve.output_pixels.clone().unwrap_or_default().into()),
            ("contradiction".to_string(), solve.contradiction.into()),
            ("width".to_string(), f64::from(solve.output_width).into()),
            ("height".to_string(), f64::from(solve.output_height).into()),
        ]));
        self.port.dispatch(Effect::DispatchAction { req: RequestId(COMMIT_SOLVE_REQUEST), action: COMMIT_SOLVE_ACTION_ID.into(), args: Some(args), delay_ms: 0 });
    }

    fn fault(&mut self, context: &mut StepContext<'_>, message: &str) -> StepOutcome {
        let detail = match context.payload_from_bytes(JobPayloadStream::Fault, message.as_bytes()) {
            Ok(payload) => payload,
            Err(rejected) => {
                drop(rejected.into_source());
                semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Fault)
            }
        };
        StepOutcome::Fault(semio_framework_job::JobFault { detail })
    }
}

impl InteractiveJob for BitmapFillRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || context.is_cancelled() {
            self.committed_solve = None;
            self.pending_tick = false;
            self.pending_finish = None;
            let _ = self.writer.step(ToolRunStepKind::Warning, FillStage::Complete.index(), FillReason::Cancelled.code(), None, &[]);
            return StepOutcome::Cancelled;
        }
        if let Some((contradiction, payload)) = self.pending_finish.take() {
            let reason = if contradiction { FillReason::Contradiction } else { FillReason::Collapsed };
            self.commit_solve(&payload);
            self.prep = PrepStage::Done;
            return self.publish_tick(context, payload, FillStage::Complete, reason, ToolRunState::Complete);
        }
        if self.pending_tick {
            self.pending_tick = false;
            let payload = self.last_payload.clone().unwrap_or_else(|| payload_from_assignment(self.snapshot.output.width, self.snapshot.output.height, &[], false, false));
            let stage = self.child.as_ref().map(|job| FillStage::of(job.preview(0).stage)).unwrap_or(FillStage::CommitSlot);
            return self.publish_tick(context, payload, stage, FillReason::Collapsed, ToolRunState::Running);
        }
        match self.prep {
            PrepStage::Compile => {
                match crate::inferences::compile_bitmap_collapse(self.snapshot.as_ref()) {
                    Ok(parts) => {
                        let model = parts.model.clone();
                        let topology = parts.topology.clone();
                        let fixed = parts.fixed.clone();
                        self.parts = Some(parts);
                        let operation = semio_framework_job::Operation::new(context.operation(), semio_framework_job::RevisionId(0), context.generation(), self.snapshot.seed);
                        self.operation = operation;
                        self.child = Some(engine::job::WfcJob::new(operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed));
                        self.prep = PrepStage::Child;
                        if let Some(payload) = self.payload_from_child(false, false) {
                            self.last_payload = Some(payload.clone());
                            return self.publish_tick(context, payload, FillStage::InitializeDomains, FillReason::Collapsed, ToolRunState::Running);
                        }
                        StepOutcome::Yield
                    }
                    Err(error) => self.fault(context, &error),
                }
            }
            PrepStage::Child => {
                let Some(child) = self.child.as_mut() else {
                    return self.fault(context, "bitmap-fill-child-missing");
                };
                let mut outcome = child.step(context);
                match &outcome {
                    StepOutcome::Yield => StepOutcome::Yield,
                    StepOutcome::Cancelled => {
                        engine::job::retire_outcome(&mut outcome);
                        self.committed_solve = None;
                        StepOutcome::Cancelled
                    }
                    StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => {
                        if let Some(payload) = self.payload_from_child(false, false) {
                            self.last_payload = Some(payload);
                        }
                        engine::job::retire_outcome(&mut outcome);
                        self.pending_tick = true;
                        StepOutcome::Yield
                    }
                    StepOutcome::Complete(_) => {
                        let commit = child.take_completed_commit();
                        let contradiction = commit.is_none();
                        let payload = if contradiction {
                            BitmapFillPayload {
                                pixels: String::new(),
                                decided: encode_base64(&vec![0u8; (self.snapshot.output.width as usize).saturating_mul(self.snapshot.output.height as usize)]),
                                width: self.snapshot.output.width,
                                height: self.snapshot.output.height,
                                contradiction: true,
                                done: true,
                            }
                        } else {
                            self.payload_from_child(false, true).unwrap_or_else(|| {
                                payload_from_assignment(self.snapshot.output.width, self.snapshot.output.height, &[], false, true)
                            })
                        };
                        engine::job::retire_outcome(&mut outcome);
                        if let Some(mut child) = self.child.take() {
                            engine::job::close_job(&mut child);
                        }
                        self.pending_finish = Some((contradiction, payload));
                        StepOutcome::Yield
                    }
                    StepOutcome::Fault(fault) if engine::job::payload_bytes(&fault.detail) == b"wfc-unsatisfiable" => {
                        engine::job::retire_outcome(&mut outcome);
                        if let Some(mut child) = self.child.take() {
                            engine::job::close_job(&mut child);
                        }
                        let payload = BitmapFillPayload {
                            pixels: String::new(),
                            decided: encode_base64(&vec![0u8; (self.snapshot.output.width as usize).saturating_mul(self.snapshot.output.height as usize)]),
                            width: self.snapshot.output.width,
                            height: self.snapshot.output.height,
                            contradiction: true,
                            done: true,
                        };
                        self.pending_finish = Some((true, payload));
                        StepOutcome::Yield
                    }
                    StepOutcome::Fault(_) => {
                        let detail = String::from_utf8_lossy(&engine::job::payload_bytes(match &outcome {
                            StepOutcome::Fault(fault) => &fault.detail,
                            _ => unreachable!(),
                        })).into_owned();
                        engine::job::retire_outcome(&mut outcome);
                        self.fault(context, &detail)
                    }
                }
            }
            PrepStage::Done => StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
            }),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.pending_tick = false;
        self.pending_finish = None;
        if let Some(child) = &mut self.child {
            InteractiveJob::begin_close(child);
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if let Some(child) = &mut self.child {
            return InteractiveJob::close_step(child, maximum_items, maximum_bytes);
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.child.as_ref().map_or(true, InteractiveJob::terminal_is_empty)
    }
}
//#endregion 🧵️Job

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
