//! 🪣 Edit-mode tool — Fill: the interactive collapse of `s.wfc.grid3d`. A non-mutating framework
//! tool run (`mutating: false`, `ToolRunTraceKind::None`) whose every step advances one preparation
//! unit or one `WfcJob::step`, publishes a partial-assignment payload the preview pane paints, and
//! on completion stores the finished assignment in `Grid3dPreviewResidency`. Abort never stores it.

use crate::editor::grid3d::modes::edit::windows::preview;
use crate::schema::inferences::{solve_with_job, Grid3dAssignment, Grid3dInferenceCommit};
use crate::schema::snapshot::Grid3dSnapshot;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, Operation, StepContext, StepOutcome};
use semio_framework_plugin::{Fault, LocalizedLabel, ToolDefinition, ToolRunJob, ToolRunJobPurpose, ToolRunJobRequest};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunRebasePolicy, ToolRunReasonDefinition, ToolRunReconfigurePolicy, ToolRunSettingsReads,
    ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunTickWriter, ToolRunTraceKind, ToolRunVerdict, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_START_ACTION_ID,
};
use semio_s_plugin_wfc_engine as wfc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const RUN_JOB_KIND: &str = "s.wfc.grid3d.fill.run";
pub const PAYLOAD_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
//#endregion 🔖️Constants

//#region 🔖️Payload
/// 🧩 One cell in a fill tick: `tile_id` is absent while the cell is still open.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid3dFillCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tile_id: Option<String>,
}

/// 🧩 One collapse or discard the preview keeps after the search moves on.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid3dFillTraceEvent {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub tile_id: String,
    pub discarded: bool,
}

/// 📦 The preview payload every fill tick publishes — partial cell assignments plus terminal flags.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid3dFillPayload {
    pub assignments: Vec<Grid3dFillCell>,
    pub contradiction: bool,
    pub done: bool,
    #[serde(default)]
    #[value(default)]
    pub trace: Vec<Grid3dFillTraceEvent>,
}

impl Grid3dFillPayload {
    /// 🔢 How many cells already carry a tile.
    pub fn decided_count(&self) -> usize {
        self.assignments.iter().filter(|cell| cell.tile_id.is_some()).count()
    }

    /// 🏁 Finished assignments the residency stores — only decided cells, matching `solve_with_job`.
    pub fn decided_assignments(&self) -> Vec<Grid3dAssignment> {
        self.assignments.iter().filter_map(|cell| cell.tile_id.as_ref().map(|tile| Grid3dAssignment { x: cell.x, y: cell.y, z: cell.z, tile_id: tile.clone() })).collect()
    }

    /// 🏁 The commit shape `solve_with_job` returns for the same collapse.
    pub fn as_commit(&self) -> Grid3dInferenceCommit {
        Grid3dInferenceCommit { satisfiable: !self.contradiction, assignments: self.decided_assignments() }
    }

    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }
}
//#endregion 🔖️Payload

//#region 🔖️Definition
/// 📋 Stitched into the editor manifest by `crate::editor::grid3d::create_grid3d_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework_plugin::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Fill", "Füllen"), "paint-bucket")) }
}

/// ▶️ Non-mutating collapse: pause/resume/step/abort are the framework panel's; each tick dirties the preview.
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
        settings: ToolRunSettingsReads::default(),
        windows: vec![preview::WINDOW_KIND_ID.into()],
    }
}

/// 🏃 The fill run of this document instance while it is not terminal.
pub fn live_fill_run(tool_run: Option<&semio_framework_plugin::ToolRunView>) -> Option<&semio_framework_plugin::ToolRunView> {
    tool_run.filter(|run| run.tool_id == TOOL_ID && !run.state.is_terminal())
}

/// 🎬 `toolRunStart` effect that arms this fill tool.
pub fn start_effect() -> semio_framework::kernel::Effect {
    let args = serde_json::json!({ TOOL_RUN_ARG_TOOL_ID: TOOL_ID });
    semio_framework::kernel::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: TOOL_RUN_START_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(args)),
        delay_ms: 0,
    }
}
//#endregion 🔖️Definition

//#region 🏷️Stages
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillStage {
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
    pub const ALL: [Self; 10] = [
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

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
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

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::InitializeDomains => LocalizedLabel::native("Initialize domains", "Domänen initialisieren"),
            Self::FindMinimumEntropySlot => LocalizedLabel::native("Find minimum entropy", "Minimale Entropie finden"),
            Self::ChooseCandidate => LocalizedLabel::native("Choose candidate", "Kandidat wählen"),
            Self::PropagateCompatibilityEdge => LocalizedLabel::native("Propagate compatibility", "Kompatibilität propagieren"),
            Self::DetectContradiction => LocalizedLabel::native("Detect contradiction", "Widerspruch erkennen"),
            Self::BacktrackTrailEntry => LocalizedLabel::native("Backtrack", "Zurückverfolgen"),
            Self::CommitSlot => LocalizedLabel::native("Commit slot", "Zelle festlegen"),
            Self::MaterializeCheckpoint => LocalizedLabel::native("Materialize checkpoint", "Checkpoint materialisieren"),
            Self::MaterializeCommit => LocalizedLabel::native("Materialize commit", "Ergebnis materialisieren"),
            Self::Complete => LocalizedLabel::native("Complete", "Abschließen"),
        }
    }

    pub fn of(stage: wfc::job::WfcStage) -> Self {
        match stage {
            wfc::job::WfcStage::InitializeDomains => Self::InitializeDomains,
            wfc::job::WfcStage::FindMinimumEntropySlot => Self::FindMinimumEntropySlot,
            wfc::job::WfcStage::ChooseCandidate => Self::ChooseCandidate,
            wfc::job::WfcStage::PropagateCompatibilityEdge => Self::PropagateCompatibilityEdge,
            wfc::job::WfcStage::DetectContradiction => Self::DetectContradiction,
            wfc::job::WfcStage::BacktrackTrailEntry => Self::BacktrackTrailEntry,
            wfc::job::WfcStage::CommitSlot => Self::CommitSlot,
            wfc::job::WfcStage::MaterializeCheckpoint => Self::MaterializeCheckpoint,
            wfc::job::WfcStage::MaterializeCommit => Self::MaterializeCommit,
            wfc::job::WfcStage::Complete => Self::Complete,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillCounter {
    Observations,
    Decided,
    Backtracks,
}

impl FillCounter {
    pub const ALL: [Self; 3] = [Self::Observations, Self::Decided, Self::Backtracks];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Observations => "observations",
            Self::Decided => "decided",
            Self::Backtracks => "backtracks",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Observations => LocalizedLabel::native("Observations", "Beobachtungen"),
            Self::Decided => LocalizedLabel::native("Decided", "Festgelegt"),
            Self::Backtracks => LocalizedLabel::native("Backtracks", "Rückschritte"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillReason {
    Collapsed,
    Contradiction,
    Cancelled,
    Fault,
}

impl FillReason {
    pub const ALL: [Self; 4] = [Self::Collapsed, Self::Contradiction, Self::Cancelled, Self::Fault];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Collapsed => "collapsed",
            Self::Contradiction => "contradiction",
            Self::Cancelled => "cancelled",
            Self::Fault => "fault",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Collapsed | Self::Contradiction => ToolRunVerdict::Success,
            Self::Cancelled => ToolRunVerdict::Warning,
            Self::Fault => ToolRunVerdict::Danger,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::Collapsed => LocalizedLabel::native("Collapsed with {0} decided cells", "Kollabiert mit {0} festgelegten Zellen"),
            Self::Contradiction => LocalizedLabel::native("Contradiction: the grid is unsatisfiable", "Widerspruch: das Raster ist unerfüllbar"),
            Self::Cancelled => LocalizedLabel::native("Cancelled", "Abgebrochen"),
            Self::Fault => LocalizedLabel::native("Fault: {0}", "Fehler: {0}"),
        }
    }
}
//#endregion 🏷️Stages

//#region 🧵️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PrepStage {
    Tiles,
    Rules,
    Model,
    Mask,
    Topology,
    Fixed,
    Solve,
}

/// 🀄️ Interactive fill run: one preparation unit or one `WfcJob::step` per framework step.
pub struct Grid3dFillRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    window_id: String,
    snapshot: Arc<Grid3dSnapshot>,
    operation: Option<Operation>,
    prep: PrepStage,
    cursor: usize,
    tile_of: BTreeMap<String, usize>,
    tile_ids: Vec<String>,
    builder: Option<wfc::tiled::TiledModelBuilder>,
    relations: Vec<wfc::ids::RelationId>,
    model: Option<wfc::model::CompiledModel>,
    mask: Vec<bool>,
    topology: Option<wfc::grid3d::Grid3dTopology>,
    fixed: Vec<(wfc::ids::NodeId, wfc::ids::PatternId)>,
    child: Option<wfc::job::WfcJob<wfc::grid3d::Grid3dTopology>>,
    stage: FillStage,
    observations: u64,
    backtracks: u64,
    last_payload: Grid3dFillPayload,
    settled: Option<StepOutcome>,
    closing: bool,
    committed: bool,
    needs_tick: bool,
    pending_finish: Option<(bool, Grid3dFillPayload)>,
    trace: Vec<Grid3dFillTraceEvent>,
}

impl Grid3dFillRunJob {
    pub fn new(identity: ToolRunIdentity, snapshot: Arc<Grid3dSnapshot>, window_id: Option<String>) -> Self {
        Self {
            identity,
            writer: ToolRunTickWriter::new(identity),
            window_id: window_id.unwrap_or_else(|| preview::WINDOW_KIND_ID.to_string()),
            snapshot,
            operation: None,
            prep: PrepStage::Tiles,
            cursor: 0,
            tile_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            builder: Some(wfc::tiled::TiledModelBuilder::new()),
            relations: Vec::new(),
            model: None,
            mask: Vec::new(),
            topology: None,
            fixed: Vec::new(),
            child: None,
            stage: FillStage::InitializeDomains,
            observations: 0,
            backtracks: 0,
            last_payload: Grid3dFillPayload::default(),
            settled: None,
            closing: false,
            committed: false,
            needs_tick: false,
            pending_finish: None,
            trace: Vec::new(),
        }
    }

    fn cell_count(snapshot: &Grid3dSnapshot) -> usize {
        (snapshot.width as usize).saturating_mul(snapshot.height as usize).saturating_mul(snapshot.depth as usize)
    }

    fn boundary(periodic: bool) -> wfc::grid2d::Boundary {
        if periodic {
            wfc::grid2d::Boundary::Wrap
        } else {
            wfc::grid2d::Boundary::Open
        }
    }

    fn xyz(snapshot: &Grid3dSnapshot, index: usize) -> (u32, u32, u32) {
        let width = snapshot.width as usize;
        let height = snapshot.height as usize;
        let plane = width * height;
        let z = index / plane;
        let rest = index % plane;
        ((rest % width) as u32, (rest / width) as u32, z as u32)
    }

    fn is_masked(&self, x: u32, y: u32, z: u32) -> bool {
        self.snapshot.masked.iter().any(|cell| cell.x == x && cell.y == y && cell.z == z)
    }

    fn empty_payload(&self, contradiction: bool, done: bool) -> Grid3dFillPayload {
        let mut assignments = Vec::new();
        for z in 0..self.snapshot.depth {
            for y in 0..self.snapshot.height {
                for x in 0..self.snapshot.width {
                    if self.is_masked(x, y, z) {
                        continue;
                    }
                    assignments.push(Grid3dFillCell { x, y, z, tile_id: None });
                }
            }
        }
        Grid3dFillPayload { assignments, contradiction, done, trace: Vec::new() }
    }

    fn payload_from_commit(&self, commit: &wfc::job::WfcCommit, done: bool) -> Grid3dFillPayload {
        let mut assignments = Vec::new();
        for (index, pattern) in commit.assignment.iter().enumerate() {
            let (x, y, z) = Self::xyz(&self.snapshot, index);
            if self.is_masked(x, y, z) {
                continue;
            }
            let tile_id = self.tile_ids.get(*pattern as usize).cloned();
            assignments.push(Grid3dFillCell { x, y, z, tile_id });
        }
        Grid3dFillPayload { assignments, contradiction: false, done, trace: Vec::new() }
    }

    /// 👁️ Builds the live payload from `domain_masks` (full, not the truncated `incomplete_grid`) after reading `preview()`.
    /// 👁️ Builds the live payload from `observed()` after reading `preview()` — full in-process state, not the truncated published `incomplete_grid`.
    /// 👁️ Live singletons. Undone collapses stay visible through `trace`.
    fn payload_from_child(&self, contradiction: bool, done: bool) -> Grid3dFillPayload {
        let Some(child) = self.child.as_ref() else {
            return self.empty_payload(contradiction, done);
        };
        let mut patterns = Vec::new();
        child.write_singleton_patterns(&mut patterns);
        let mut assignments = Vec::new();
        for z in 0..self.snapshot.depth {
            for y in 0..self.snapshot.height {
                for x in 0..self.snapshot.width {
                    if self.is_masked(x, y, z) {
                        continue;
                    }
                    let width = self.snapshot.width as usize;
                    let height = self.snapshot.height as usize;
                    let cell = ((z as usize) * height + y as usize) * width + x as usize;
                    let tile_id = patterns.get(cell).copied().filter(|pattern| *pattern != u32::MAX).and_then(|pattern| self.tile_ids.get(pattern as usize).cloned());
                    assignments.push(Grid3dFillCell { x, y, z, tile_id });
                }
            }
        }
        Grid3dFillPayload { assignments, contradiction, done, trace: Vec::new() }
    }

    fn remember(&mut self, event: Grid3dFillTraceEvent) {
        self.trace.push(event);
        if self.trace.len() > 512 {
            let overflow = self.trace.len() - 512;
            self.trace.drain(0..overflow);
        }
    }

    fn note_singleton_change(&mut self, index: usize, previous: u32, next: u32) {
        let (x, y, z) = Self::xyz(&self.snapshot, index);
        if z >= self.snapshot.depth || self.is_masked(x, y, z) {
            return;
        }
        if previous != u32::MAX {
            if let Some(tile) = self.tile_ids.get(previous as usize) {
                self.remember(Grid3dFillTraceEvent { x, y, z, tile_id: tile.clone(), discarded: true });
            }
        }
        if next != u32::MAX {
            if let Some(tile) = self.tile_ids.get(next as usize) {
                self.remember(Grid3dFillTraceEvent { x, y, z, tile_id: tile.clone(), discarded: false });
            }
        }
    }

    fn drain_child(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.child.is_none() {
            return self.fault(context, "wfc-grid3d-fill-missing-child");
        }
        let cells = Self::cell_count(&self.snapshot);
        let mut flips = Vec::new();
        let mut fresh = 0usize;
        let mut solved = false;
        let mut unsatisfiable = false;
        loop {
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.fuel_exhausted() || context.deadline_exceeded() {
                break;
            }
            let pulse = self.child.as_mut().expect("child").advance_one();
            context.consume_fuel(1);
            self.child.as_mut().expect("child").drain_visible(&mut flips);
            for flip in flips.drain(..) {
                if (flip.node as usize) >= cells {
                    continue;
                }
                let before = self.trace.len();
                if flip.discarded {
                    self.note_singleton_change(flip.node as usize, flip.pattern, u32::MAX);
                } else {
                    self.note_singleton_change(flip.node as usize, u32::MAX, flip.pattern);
                }
                fresh += self.trace.len() - before;
            }
            let (observations, _, backtracks) = self.child.as_ref().expect("child").metrics();
            self.observations = observations;
            self.backtracks = backtracks;
            match pulse {
                wfc::job::SearchPulse::Solved => {
                    solved = true;
                    break;
                }
                wfc::job::SearchPulse::Unsatisfiable => {
                    unsatisfiable = true;
                    break;
                }
                wfc::job::SearchPulse::Collapsed { .. } | wfc::job::SearchPulse::Discarded { .. } | wfc::job::SearchPulse::Worked => {}
            }
            if fresh >= 48 {
                break;
            }
        }
        if solved {
            let commit = self.child.as_ref().and_then(|child| child.commit());
            if let Some(mut child) = self.child.take() {
                wfc::job::close_job(&mut child);
            }
            let contradiction = commit.is_none();
            let payload = match commit {
                Some(commit) => self.payload_from_commit(&commit, true),
                None => self.empty_payload(true, true),
            };
            return self.finish_success(context, contradiction, Some(payload));
        }
        if unsatisfiable {
            if let Some(mut child) = self.child.take() {
                wfc::job::close_job(&mut child);
            }
            return self.finish_success(context, true, Some(self.empty_payload(true, true)));
        }
        if fresh == 0 {
            return StepOutcome::Yield;
        }
        if let Some(child) = self.child.as_ref() {
            self.stage = FillStage::of(child.preview(0).stage);
        }
        self.last_payload = self.payload_from_child(false, false);
        self.publish_tick(context, ToolRunState::Running, None)
    }

    fn publish_tick(&mut self, context: &mut StepContext<'_>, state: ToolRunState, reason: Option<(FillReason, &[ToolRunStepArg])>) -> StepOutcome {
        if let Some((reason, args)) = reason {
            let _ = self.writer.step(ToolRunStepKind::Info, self.stage.index(), reason.code(), None, args);
        }
        let decided = self.last_payload.decided_count() as u64;
        self.writer.progress(ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: self.stage.index(),
            completed: decided,
            total: Some(self.last_payload.assignments.len() as u64),
            counters: vec![
                ToolRunCounter { counter: FillCounter::Observations.index(), value: self.observations },
                ToolRunCounter { counter: FillCounter::Decided.index(), value: decided },
                ToolRunCounter { counter: FillCounter::Backtracks.index(), value: self.backtracks },
            ],
            units_per_second: 0.0,
            conflicts: 0,
            steps: semio_framework_tool_run::ToolRunStepRing::new(),
        });
        self.last_payload.trace.clone_from(&self.trace);
        self.writer.payload(self.last_payload.encode());
        match self.writer.finish().and_then(|tick| tick.encode().ok()).and_then(|bytes| context.payload_from_bytes(semio_framework_job::JobPayloadStream::Preview, &bytes).map_err(|rejected| drop(rejected.into_source())).ok()) {
            Some(payload) => StepOutcome::PreviewReady(payload),
            None => StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        }
    }

    fn fault(&mut self, context: &mut StepContext<'_>, message: &str) -> StepOutcome {
        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage.index(), FillReason::Fault.code(), None, &[]);
        self.last_payload = self.empty_payload(false, true);
        self.settled = Some(StepOutcome::Fault(semio_framework_job::JobFault {
            detail: match context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, message.as_bytes()) {
                Ok(payload) => payload,
                Err(rejected) => {
                    drop(rejected.into_source());
                    semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault)
                }
            },
        }));
        self.publish_tick(context, ToolRunState::Faulted, Some((FillReason::Fault, &[])))
    }

    fn finish_success(&mut self, context: &mut StepContext<'_>, contradiction: bool, payload: Option<Grid3dFillPayload>) -> StepOutcome {
        self.last_payload = payload.unwrap_or_else(|| {
            if contradiction {
                self.empty_payload(true, true)
            } else {
                self.payload_from_child(false, true)
            }
        });
        if !self.committed {
            let assignments = if contradiction || self.last_payload.contradiction { Vec::new() } else { self.last_payload.decided_assignments() };
            preview::commit_fill_result(&self.window_id, &self.snapshot, assignments);
            self.committed = true;
        }
        let reason = if contradiction || self.last_payload.contradiction { FillReason::Contradiction } else { FillReason::Collapsed };
        let decided = ToolRunStepArg::Unsigned(self.last_payload.decided_count() as u64);
        self.settled = Some(StepOutcome::Complete(semio_framework_job::CommitCandidate {
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
        }));
        self.stage = FillStage::Complete;
        self.publish_tick(context, ToolRunState::Complete, Some((reason, &[decided])))
    }

    /// 🔗 Binds the child `WfcJob` to the same operation and generation the framework drives this run with.
    fn bind_operation(&mut self, context: &StepContext<'_>) -> Operation {
        if let Some(operation) = self.operation {
            return operation;
        }
        let operation = Operation::new(context.operation(), semio_framework_job::RevisionId(0), context.generation(), self.snapshot.seed);
        self.operation = Some(operation);
        operation
    }

    fn advance_prep(&mut self, operation: Operation) -> Result<(), String> {
        match self.prep {
            PrepStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    let weight = if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 };
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    let _id = builder.tile(weight);
                    self.tile_of.insert(tile.id.clone(), self.cursor);
                    self.tile_ids.push(tile.id.clone());
                    self.cursor += 1;
                } else if self.snapshot.tiles.is_empty() {
                    self.prep = PrepStage::Solve;
                } else {
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    self.relations = wfc::grid3d::declare_stencil_relations_3d_tiled(builder, &wfc::grid3d::Stencil3d::Face6).map_err(|error| format!("{error:?}"))?;
                    self.cursor = 0;
                    self.prep = PrepStage::Rules;
                }
            }
            PrepStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    if let (Some(&a), Some(&b)) = (self.tile_of.get(&rule.tile_a_id), self.tile_of.get(&rule.tile_b_id)) {
                        let builder = self.builder.as_mut().expect("tiled model builder");
                        let forward = self.relations[rule.direction.stencil_index()];
                        let backward = self.relations[rule.direction.opposite().stencil_index()];
                        let (a, b) = (wfc::ids::TileId::from_index(a), wfc::ids::TileId::from_index(b));
                        if rule.allowed {
                            builder.allow(forward, a, b);
                            builder.allow(backward, b, a);
                        } else {
                            builder.deny(forward, a, b);
                            builder.deny(backward, b, a);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.prep = PrepStage::Model;
                }
            }
            PrepStage::Model => {
                let builder = self.builder.take().expect("tiled model builder");
                self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                self.mask = vec![true; Self::cell_count(&self.snapshot)];
                self.cursor = 0;
                self.prep = PrepStage::Mask;
            }
            PrepStage::Mask => {
                if let Some(cell) = self.snapshot.masked.get(self.cursor) {
                    if cell.x < self.snapshot.width && cell.y < self.snapshot.height && cell.z < self.snapshot.depth {
                        let index = (cell.z as usize) * (self.snapshot.width as usize) * (self.snapshot.height as usize) + (cell.y as usize) * (self.snapshot.width as usize) + cell.x as usize;
                        self.mask[index] = false;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.prep = PrepStage::Topology;
                }
            }
            PrepStage::Topology => {
                let mask = (!self.snapshot.masked.is_empty()).then(|| std::mem::take(&mut self.mask));
                self.topology = Some(
                    wfc::grid3d::Grid3dTopology::new(
                        self.snapshot.width as usize,
                        self.snapshot.height as usize,
                        self.snapshot.depth as usize,
                        &wfc::grid3d::Stencil3d::Face6,
                        self.relations.clone(),
                        Self::boundary(self.snapshot.periodic_x),
                        Self::boundary(self.snapshot.periodic_y),
                        Self::boundary(self.snapshot.periodic_z),
                        mask,
                    )
                    .map_err(|error| format!("{error:?}"))?,
                );
                self.cursor = 0;
                self.prep = PrepStage::Fixed;
            }
            PrepStage::Fixed => {
                if let Some(cell) = self.snapshot.pinned.get(self.cursor) {
                    if let (Some(topology), Some(&tile)) = (self.topology.as_ref(), self.tile_of.get(&cell.tile_id)) {
                        if let Some(node) = topology.node_at(cell.x as usize, cell.y as usize, cell.z as usize) {
                            self.fixed.push((node, wfc::ids::PatternId::from_index(tile)));
                        }
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().expect("compiled model");
                    let topology = self.topology.take().expect("compiled topology");
                    let fixed = std::mem::take(&mut self.fixed);
                    self.child = Some(wfc::job::WfcJob::new(operation, model, topology, wfc::job::WfcJobConfig::default(), None, fixed));
                    self.prep = PrepStage::Solve;
                }
            }
            PrepStage::Solve => {}
        }
        Ok(())
    }
}

impl InteractiveJob for Grid3dFillRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || context.is_cancelled() {
            let _ = self.writer.step(ToolRunStepKind::Warning, self.stage.index(), FillReason::Cancelled.code(), None, &[]);
            return StepOutcome::Cancelled;
        }
        if let Some(outcome) = self.settled.take() {
            return outcome;
        }
        if let Some((contradiction, payload)) = self.pending_finish.take() {
            return self.finish_success(context, contradiction, Some(payload));
        }
        if self.needs_tick {
            self.needs_tick = false;
            return self.publish_tick(context, ToolRunState::Running, None);
        }
        if self.snapshot.tiles.is_empty() || self.snapshot.width == 0 || self.snapshot.height == 0 || self.snapshot.depth == 0 {
            return self.finish_success(context, true, None);
        }
        let operation = self.bind_operation(context);
        if self.prep != PrepStage::Solve {
            if let Err(error) = self.advance_prep(operation) {
                return self.fault(context, &error);
            }
            self.last_payload = self.payload_from_child(false, false);
            context.consume_fuel(1);
            return self.publish_tick(context, ToolRunState::Running, None);
        }
        return self.drain_child(context);
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(child) = self.child.as_mut() {
            InteractiveJob::begin_close(child);
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(outcome) = self.settled.as_mut() {
            if matches!(outcome.close_step(1, maximum_bytes), semio_framework_job::JobPayloadCloseStep::Complete) {
                self.settled = None;
            }
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(child) = self.child.as_mut() {
            return match InteractiveJob::close_step(child, maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete => {
                    self.child = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                other => other,
            };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.settled.is_none() && self.child.is_none()
    }
}

/// 🏭 Builds the fill run job for the framework ledger.
pub fn build_run_job(request: ToolRunJobRequest<'_, semio_framework_plugin::EditorApp<crate::editor::grid3d::Grid3dEditor>>) -> Result<Option<ToolRunJob>, Fault> {
    if request.tool_id != TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
        return Ok(None);
    }
    Ok(Some(Box::new(Grid3dFillRunJob::new(request.identity, request.snapshot, request.window_id.map(str::to_string)))))
}
//#endregion 🧵️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🎪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🎪️Tests
