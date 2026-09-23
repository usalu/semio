//! 🌡 Bitmap fill — non-mutating framework tool run. The solver drains headlessly for the step's
//! own time budget. One tick carries the live grid plus every collapse and discard in that burst,
//! so the output window can show choices the search later undoes. `SetSolve` is published only when
//! the run commits.

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
/// 👁️ Visible collapses and discards published together, so one paint shows a burst of thinking
/// instead of a single cell after a round trip.
const FILL_VISIBLE_BURST: usize = 48;
const FILL_TRACE_RETAIN: usize = 512;
//#endregion 🔖️Constants

//#region 🎞Payload
/// 👁️ One cell the search tried during a burst. `discarded` marks a collapse the search undid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitmapFillTraceEvent {
    pub index: u32,
    pub color: u32,
    pub discarded: bool,
}

/// 🎞 Partial or finished bitmap collapse carried in `ToolRunTick::payload`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitmapFillPayload {
    pub pixels: String,
    pub decided: String,
    pub width: u32,
    pub height: u32,
    pub contradiction: bool,
    pub done: bool,
    pub trace: Vec<BitmapFillTraceEvent>,
}

impl BitmapFillPayload {
    /// 🧮 Number of cells whose `decided` byte is non-zero.
    pub fn decided_count(&self) -> usize {
        decode_base64(&self.decided).map_or(0, |mask| mask.iter().filter(|byte| **byte != 0).count())
    }

    /// 📤 Encode as the normative JSON object.
    pub fn encode_json(&self) -> String {
        format!(
            "{{\"pixels\":{},\"decided\":{},\"width\":{},\"height\":{},\"contradiction\":{},\"done\":{},\"trace\":{}}}",
            protocol::json::to_json_string(&self.pixels),
            protocol::json::to_json_string(&self.decided),
            self.width,
            self.height,
            if self.contradiction { "true" } else { "false" },
            if self.done { "true" } else { "false" },
            trace_json(&self.trace),
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
            trace: decode_trace(object.get("trace")),
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
    BitmapFillPayload { pixels: encode_base64(&pixels), decided: encode_base64(&decided), width, height, contradiction, done, trace: Vec::new() }
}

fn trace_json(trace: &[BitmapFillTraceEvent]) -> String {
    let mut out = String::from("[");
    for (index, event) in trace.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!("{{\"index\":{},\"color\":{},\"discarded\":{}}}", event.index, event.color, if event.discarded { "true" } else { "false" }));
    }
    out.push(']');
    out
}

fn decode_trace(value: Option<&protocol::json::Value>) -> Vec<BitmapFillTraceEvent> {
    let Some(items) = value.and_then(|value| value.as_array()) else { return Vec::new() };
    items.iter().filter_map(|item| {
        let item = item.as_object()?;
        Some(BitmapFillTraceEvent { index: item.get("index")?.as_u64()? as u32, color: item.get("color")?.as_u64()? as u32, discarded: item.get("discarded")?.as_bool()? })
    }).collect()
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
    trace: Vec<BitmapFillTraceEvent>,
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
            trace: Vec::new(),
        }
    }

    fn remember(&mut self, event: BitmapFillTraceEvent) {
        self.trace.push(event);
        if self.trace.len() > FILL_TRACE_RETAIN {
            let overflow = self.trace.len() - FILL_TRACE_RETAIN;
            self.trace.drain(0..overflow);
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

    fn color_of(&self, pattern: u32) -> Option<u32> {
        let parts = self.parts.as_ref()?;
        Some(parts.decoder.anchor_tile(engine::ids::PatternId(pattern)).get())
    }

    /// 🖼 Cells that are singleton right now. Undone collapses are absent here and live only in the trace.
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
        for (index, domain) in child.domain_masks().iter().enumerate().take(cells) {
            if domain.count_ones() == 1 {
                if let Some(pattern) = domain.first_set() {
                    assignment.push((index, parts.decoder.anchor_tile(pattern).get() as u8));
                }
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

    fn drain_child(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.child.is_none() {
            return self.fault(context, "bitmap-fill-child-missing");
        }
        let cells = (self.snapshot.output.width as usize).saturating_mul(self.snapshot.output.height as usize);
        let mut flips = Vec::new();
        let mut trace = 0usize;
        let mut solved = false;
        let mut unsatisfiable = false;
        loop {
            if context.is_cancelled() {
                self.committed_solve = None;
                return StepOutcome::Cancelled;
            }
            if context.fuel_exhausted() || context.deadline_exceeded() {
                break;
            }
            let pulse = self.child.as_mut().expect("child").advance_one();
            context.consume_fuel(1);
            self.child.as_mut().expect("child").drain_visible(&mut flips);
            for flip in flips.drain(..) {
                let index = flip.node as usize;
                if index >= cells {
                    continue;
                }
                if let Some(color) = self.color_of(flip.pattern) {
                    self.remember(BitmapFillTraceEvent { index: flip.node, color, discarded: flip.discarded });
                    trace += 1;
                }
            }
            match pulse {
                engine::job::SearchPulse::Solved => {
                    solved = true;
                    break;
                }
                engine::job::SearchPulse::Unsatisfiable => {
                    unsatisfiable = true;
                    break;
                }
                engine::job::SearchPulse::Collapsed { .. } | engine::job::SearchPulse::Discarded { .. } | engine::job::SearchPulse::Worked => {}
            }
            if trace >= FILL_VISIBLE_BURST {
                break;
            }
        }
        if solved || unsatisfiable {
            let width = self.snapshot.output.width;
            let height = self.snapshot.output.height;
            let payload = if unsatisfiable {
                BitmapFillPayload {
                    pixels: String::new(),
                    decided: encode_base64(&vec![0u8; (width as usize).saturating_mul(height as usize)]),
                    width,
                    height,
                    contradiction: true,
                    done: true,
                    trace: self.trace.clone(),
                }
            } else {
                let mut payload = self.payload_from_child(false, true).unwrap_or_else(|| payload_from_assignment(width, height, &[], false, true));
                payload.trace = self.trace.clone();
                payload
            };
            if !unsatisfiable {
                self.commit_solve(&payload);
            }
            if let Some(mut child) = self.child.take() {
                engine::job::close_job(&mut child);
            }
            self.prep = PrepStage::Done;
            let reason = if unsatisfiable { FillReason::Contradiction } else { FillReason::Collapsed };
            return self.publish_tick(context, payload, FillStage::Complete, reason, ToolRunState::Complete);
        }
        if trace == 0 {
            return StepOutcome::Yield;
        }
        let width = self.snapshot.output.width;
        let height = self.snapshot.output.height;
        let mut payload = self.payload_from_child(false, false).unwrap_or_else(|| payload_from_assignment(width, height, &[], false, false));
        payload.trace = self.trace.clone();
        let stage = self.child.as_ref().map(|job| FillStage::of(job.preview(0).stage)).unwrap_or(FillStage::ChooseCandidate);
        self.publish_tick(context, payload, stage, FillReason::Collapsed, ToolRunState::Running)
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
                        StepOutcome::Yield
                    }
                    Err(error) => self.fault(context, &error),
                }
            }
            PrepStage::Child => self.drain_child(context),
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
