//! 🪣 Edit-mode tool — Fill: the interactive collapse of `s.wfc.grid2d`. A non-mutating framework
//! tool run (`mutating: false`, `ToolRunTraceKind::None`) whose every step advances one preparation
//! unit or one `WfcJob::step`, publishes a partial-assignment payload the preview pane paints, and
//! on completion dispatches `commit-fill` so `Grid2dWindowConfig.solve_json` receives the finished
//! cache. Abort never writes that cache.

use crate::editor::grid2d::modes::edit::windows::preview;
use crate::schema::inferences::{Grid2dInferenceCommit};


use crate::host::inferences::{solve_with_job};
use crate::schema::snapshot::{Grid2dSnapshot, WfcDirection2d};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ToolDefinition;
use semio_framework_plugin::ToolRunJobPort;
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
pub const RUN_JOB_KIND: &str = "s.wfc.grid2d.fill.run";
pub const COMMIT_FILL_ACTION_ID: &str = "commit-fill";
pub const PAYLOAD_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
//#endregion 🔖️Constants

//#region 🔖️Payload
/// 🧩 One collapse or discard the preview keeps on screen after the search moves on.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid2dFillTraceEvent {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
    pub discarded: bool,
}

/// 🧩 One cell in a fill tick: `tile_id` is absent while the cell is still open.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid2dFillCell {
    pub x: u32,
    pub y: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tile_id: Option<String>,
}

/// 📦 The preview payload every fill tick publishes — partial cell assignments plus terminal flags.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid2dFillPayload {
    pub assignments: Vec<Grid2dFillCell>,
    pub contradiction: bool,
    pub done: bool,
    #[serde(default)]
    #[value(default)]
    pub trace: Vec<Grid2dFillTraceEvent>,
}

impl Grid2dFillPayload {
    /// 🔢 How many cells already carry a tile.
    pub fn decided_count(&self) -> usize {
        self.assignments.iter().filter(|cell| cell.tile_id.is_some()).count()
    }

    /// 🏁 The finished cache shape the preview pane already paints from `solve_json`.
    pub fn as_commit(&self) -> Grid2dInferenceCommit {
        Grid2dInferenceCommit {
            assignments: self.assignments.iter().filter_map(|cell| cell.tile_id.as_ref().map(|tile| (cell.x, cell.y, tile.clone()))).collect(),
            contradiction: self.contradiction,
            entropy: Vec::new(),
        }
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
/// 📋 Stitched into the editor manifest by `crate::editor::grid2d::create_grid2d_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        run: Some(run_definition()),
        ..::semio_framework_async::poll::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Fill", "Füllen"), "paint-bucket"))
    }
}

/// ▶️ Non-mutating collapse: pause/resume/step/abort are the framework panel's; each tick dirties the preview.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: LocalizedLabel::native("steps", "Schritte"),
        stages: FillStage::ALL
            .iter()
            .map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: stage.label() })
            .collect(),
        counters: FillCounter::ALL
            .iter()
            .map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() })
            .collect(),
        reasons: FillReason::ALL
            .iter()
            .map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() })
            .collect(),
        trace: ToolRunTraceKind::None,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads::default(),
        windows: vec![preview::WINDOW_KIND_ID.into()],
        member: None,
    }
}

/// 🏃 The fill run of this document instance while it is not terminal.
pub fn live_fill_run(tool_run: Option<&semio_framework_plugin::ToolRunView>) -> Option<&semio_framework_plugin::ToolRunView> {
    tool_run.filter(|run| run.tool_id == TOOL_ID && !run.state.is_terminal())
}

/// 🎬 `toolRunStart` effect that arms this fill tool.
pub fn start_effect() -> semio_framework::kernel::Effect {
    let args = semio_framework::dsl_value!({ TOOL_RUN_ARG_TOOL_ID: TOOL_ID });
    semio_framework::kernel::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: TOOL_RUN_START_ACTION_ID.into(),
        args: Some(args),
        delay_ms: 0,
    }
}

/// 💾 `commit-fill` effect that lands the finished cache in `solve_json`.
pub fn commit_effect(commit: &Grid2dInferenceCommit) -> semio_framework::kernel::Effect {
    let args = semio_framework::dsl_value!({ "solveJson": semio_framework_pack_json::to_json_string(commit) });
    semio_framework::kernel::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: COMMIT_FILL_ACTION_ID.into(),
        args: Some(args),
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
    Topology,
    Fixed,
    Solve,
}

/// 🚦️ What one bounded fill run decided before any outcome is lent.
enum FillRun {
    Yield,
    Cancelled,
    Staged,
}

/// 🏁️ The terminal verdict held back until its final tick has been delivered and closed.
enum FillSettled {
    Complete,
    Fault(Vec<u8>),
}

/// 🀄️ Interactive fill run: one preparation unit or one `WfcJob::step` per framework step.
pub struct Grid2dFillRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    port: ToolRunJobPort,
    snapshot: Arc<Grid2dSnapshot>,
    operation: Option<Operation>,
    prep: PrepStage,
    cursor: usize,
    tile_of: BTreeMap<String, usize>,
    tile_ids: Vec<String>,
    builder: Option<wfc::tiled::TiledModelBuilder>,
    tiles: Vec<wfc::ids::TileId>,
    relations: Vec<wfc::ids::RelationId>,
    model: Option<wfc::model::CompiledModel>,
    topology: Option<wfc::grid2d::Grid2dTopology>,
    fixed: Vec<(wfc::ids::NodeId, wfc::ids::PatternId)>,
    child: Option<wfc::job::WfcJob<wfc::grid2d::Grid2dTopology>>,
    stage: FillStage,
    observations: u64,
    backtracks: u64,
    last_payload: Grid2dFillPayload,
    pending_tick: bool,
    pending_finish: Option<(bool, Grid2dFillPayload)>,
    settled: Option<FillSettled>,
    publication: Option<Box<wfc::job::Publication>>,
    closing: bool,
    committed: bool,
    trace: Vec<Grid2dFillTraceEvent>,
}

impl Grid2dFillRunJob {
    pub fn new(identity: ToolRunIdentity, snapshot: Arc<Grid2dSnapshot>, port: ToolRunJobPort) -> Self {
        Self {
            identity,
            writer: ToolRunTickWriter::new(identity),
            port,
            snapshot,
            operation: None,
            prep: PrepStage::Tiles,
            cursor: 0,
            tile_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            builder: Some(wfc::tiled::TiledModelBuilder::new()),
            tiles: Vec::new(),
            relations: Vec::new(),
            model: None,
            topology: None,
            fixed: Vec::new(),
            child: None,
            stage: FillStage::InitializeDomains,
            observations: 0,
            backtracks: 0,
            last_payload: Grid2dFillPayload::default(),
            pending_tick: false,
            pending_finish: None,
            settled: None,
            publication: None,
            closing: false,
            committed: false,
            trace: Vec::new(),
        }
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

    fn relation_slot(direction: WfcDirection2d) -> usize {
        match direction {
            WfcDirection2d::Right => 0,
            WfcDirection2d::Left => 1,
            WfcDirection2d::Bottom => 2,
            WfcDirection2d::Top => 3,
        }
    }

    fn is_masked(&self, x: u32, y: u32) -> bool {
        self.snapshot.masked.iter().any(|cell| cell.x == x && cell.y == y)
    }

    fn empty_payload(&self, contradiction: bool, done: bool) -> Grid2dFillPayload {
        let mut assignments = Vec::new();
        for y in 0..self.snapshot.height {
            for x in 0..self.snapshot.width {
                if self.is_masked(x, y) {
                    continue;
                }
                assignments.push(Grid2dFillCell { x, y, tile_id: None });
            }
        }
        Grid2dFillPayload { assignments, contradiction, done, trace: Vec::new() }
    }

    /// 🏁 Builds the finished payload from the child's dense assignment vector.
    fn payload_from_commit(&self, commit: &wfc::job::WfcCommit, done: bool) -> Grid2dFillPayload {
        let width = self.snapshot.width as usize;
        let mut assignments = Vec::new();
        for (index, pattern) in commit.assignment.iter().enumerate() {
            let x = (index % width) as u32;
            let y = (index / width) as u32;
            if self.is_masked(x, y) {
                continue;
            }
            let tile_id = self.tile_ids.get(*pattern as usize).cloned();
            assignments.push(Grid2dFillCell { x, y, tile_id });
        }
        Grid2dFillPayload { assignments, contradiction: false, done, trace: Vec::new() }
    }

    /// 👁️ Live singletons. Cells the search has since undone are absent here and stay visible through `trace`.
    fn payload_from_child(&self, contradiction: bool, done: bool) -> Grid2dFillPayload {
        let Some(child) = self.child.as_ref() else {
            return self.empty_payload(contradiction, done);
        };
        let width = self.snapshot.width as usize;
        let mut patterns = Vec::new();
        child.write_singleton_patterns(&mut patterns);
        let mut assignments = Vec::new();
        for y in 0..self.snapshot.height {
            for x in 0..self.snapshot.width {
                if self.is_masked(x, y) {
                    continue;
                }
                let index = (y as usize).saturating_mul(width).saturating_add(x as usize);
                let tile_id = patterns.get(index).copied().filter(|pattern| *pattern != u32::MAX).and_then(|pattern| self.tile_ids.get(pattern as usize).cloned());
                assignments.push(Grid2dFillCell { x, y, tile_id });
            }
        }
        Grid2dFillPayload { assignments, contradiction, done, trace: Vec::new() }
    }

    fn remember(&mut self, event: Grid2dFillTraceEvent) {
        self.trace.push(event);
        if self.trace.len() > 512 {
            let overflow = self.trace.len() - 512;
            self.trace.drain(0..overflow);
        }
    }

    fn note_singleton_change(&mut self, index: usize, previous: u32, next: u32) {
        let width = self.snapshot.width as usize;
        if width == 0 {
            return;
        }
        let x = (index % width) as u32;
        let y = (index / width) as u32;
        if y >= self.snapshot.height || self.is_masked(x, y) {
            return;
        }
        if previous != u32::MAX {
            if let Some(tile) = self.tile_ids.get(previous as usize) {
                self.remember(Grid2dFillTraceEvent { x, y, tile_id: tile.clone(), discarded: true });
            }
        }
        if next != u32::MAX {
            if let Some(tile) = self.tile_ids.get(next as usize) {
                self.remember(Grid2dFillTraceEvent { x, y, tile_id: tile.clone(), discarded: false });
            }
        }
    }

    fn drain_child(&mut self, context: &mut StepContext<'_>) -> FillRun {
        if self.child.is_none() {
            return self.fault("wfc-grid2d-fill-missing-child");
        }
        let cells = (self.snapshot.width as usize).saturating_mul(self.snapshot.height as usize);
        let mut flips = Vec::new();
        let mut fresh = 0usize;
        let mut solved = false;
        let mut unsatisfiable = false;
        loop {
            if context.is_cancelled() {
                return FillRun::Cancelled;
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
            return self.finish_success(contradiction, Some(payload));
        }
        if unsatisfiable {
            if let Some(mut child) = self.child.take() {
                wfc::job::close_job(&mut child);
            }
            return self.finish_success(true, Some(self.empty_payload(true, true)));
        }
        if fresh == 0 {
            return FillRun::Yield;
        }
        if let Some(child) = self.child.as_ref() {
            self.stage = FillStage::of(child.preview(0).stage);
        }
        self.last_payload = self.payload_from_child(false, false);
        self.publish_tick(ToolRunState::Running, None, None)
    }

    fn publish_tick(&mut self, state: ToolRunState, reason: Option<(FillReason, &[ToolRunStepArg])>, settled: Option<FillSettled>) -> FillRun {
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
        match self.writer.finish().and_then(|tick| tick.encode().ok()) {
            Some(bytes) => {
                self.publication = Some(wfc::job::Publication::new(wfc::job::PublicationKind::Preview, bytes, Vec::new()));
                self.settled = settled;
            }
            None => {
                self.settled = None;
                self.publication = Some(wfc::job::Publication::new(wfc::job::PublicationKind::Fault, Vec::new(), Vec::new()));
            }
        }
        FillRun::Staged
    }

    fn fault(&mut self, message: &str) -> FillRun {
        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage.index(), FillReason::Fault.code(), None, &[]);
        self.last_payload = self.empty_payload(false, true);
        self.publish_tick(ToolRunState::Faulted, Some((FillReason::Fault, &[])), Some(FillSettled::Fault(message.as_bytes().to_vec())))
    }

    fn finish_success(&mut self, contradiction: bool, payload: Option<Grid2dFillPayload>) -> FillRun {
        self.last_payload = payload.unwrap_or_else(|| {
            if contradiction {
                self.empty_payload(true, true)
            } else {
                self.payload_from_child(false, true)
            }
        });
        if !self.committed {
            self.port.dispatch(commit_effect(&self.last_payload.as_commit()));
            self.committed = true;
        }
        let reason = if contradiction { FillReason::Contradiction } else { FillReason::Collapsed };
        let decided = ToolRunStepArg::Unsigned(self.last_payload.decided_count() as u64);
        self.stage = FillStage::Complete;
        self.publish_tick(ToolRunState::Complete, Some((reason, &[decided])), Some(FillSettled::Complete))
    }

    fn advance_prep(&mut self, operation: Operation) -> Result<(), String> {
        match self.prep {
            PrepStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    let weight = if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 };
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    let id = builder.tile(weight);
                    self.tiles.push(id);
                    self.tile_of.insert(tile.id.clone(), self.cursor);
                    self.tile_ids.push(tile.id.clone());
                    self.cursor += 1;
                } else if self.snapshot.tiles.is_empty() {
                    self.prep = PrepStage::Solve;
                } else {
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    self.relations = wfc::grid2d::declare_stencil_relations_tiled(builder, &wfc::grid2d::Stencil2d::VonNeumann).map_err(|error| format!("{error:?}"))?;
                    self.cursor = 0;
                    self.prep = PrepStage::Rules;
                }
            }
            PrepStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    if rule.allowed {
                        if let (Some(&a), Some(&b)) = (self.tile_of.get(&rule.tile_a_id), self.tile_of.get(&rule.tile_b_id)) {
                            let relation = self.relations[Self::relation_slot(rule.direction)];
                            let builder = self.builder.as_mut().expect("tiled model builder");
                            builder.allow_mirrored(relation, self.tiles[a], self.tiles[b]);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.prep = PrepStage::Model;
                }
            }
            PrepStage::Model => {
                let builder = self.builder.take().expect("tiled model builder");
                self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                self.prep = PrepStage::Topology;
            }
            PrepStage::Topology => {
                let width = self.snapshot.width as usize;
                let height = self.snapshot.height as usize;
                let mask = if self.snapshot.masked.is_empty() {
                    None
                } else {
                    let mut mask = vec![true; width * height];
                    for cell in &self.snapshot.masked {
                        if (cell.x as usize) < width && (cell.y as usize) < height {
                            mask[(cell.y as usize) * width + (cell.x as usize)] = false;
                        }
                    }
                    Some(mask)
                };
                let boundary_x = if self.snapshot.periodic_x { wfc::grid2d::Boundary::Wrap } else { wfc::grid2d::Boundary::Open };
                let boundary_y = if self.snapshot.periodic_y { wfc::grid2d::Boundary::Wrap } else { wfc::grid2d::Boundary::Open };
                self.topology = Some(
                    wfc::grid2d::Grid2dTopology::new(width, height, &wfc::grid2d::Stencil2d::VonNeumann, self.relations.clone(), boundary_x, boundary_y, mask).map_err(|error| format!("{error:?}"))?,
                );
                self.cursor = 0;
                self.prep = PrepStage::Fixed;
            }
            PrepStage::Fixed => {
                let pinned_count = self.snapshot.pinned.len();
                if self.cursor < pinned_count {
                    let cell = &self.snapshot.pinned[self.cursor];
                    let topology = self.topology.as_ref().expect("grid topology");
                    if let (Some(node), Some(&tile)) = (topology.node_at(cell.x as usize, cell.y as usize), self.tile_of.get(&cell.tile_id)) {
                        if topology.is_active(cell.x as usize, cell.y as usize) {
                            self.fixed.push((node, wfc::ids::PatternId::from_index(tile)));
                        }
                    }
                    self.cursor += 1;
                } else if self.cursor < pinned_count + self.snapshot.masked.len() {
                    let cell = self.snapshot.masked[self.cursor - pinned_count];
                    let topology = self.topology.as_ref().expect("grid topology");
                    if let Some(node) = topology.node_at(cell.x as usize, cell.y as usize) {
                        self.fixed.push((node, wfc::ids::PatternId::from_index(0)));
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

impl Grid2dFillRunJob {
    /// ⏭️ Runs one bounded preparation unit or child span; payloads are staged, never lent from here.
    fn run(&mut self, context: &mut StepContext<'_>) -> FillRun {
        if let Some((contradiction, payload)) = self.pending_finish.take() {
            return self.finish_success(contradiction, Some(payload));
        }
        if self.pending_tick {
            self.pending_tick = false;
            return self.publish_tick(ToolRunState::Running, None, None);
        }
        if self.snapshot.tiles.is_empty() || self.snapshot.width == 0 || self.snapshot.height == 0 {
            return self.finish_success(true, None);
        }
        let operation = self.bind_operation(context);
        if self.prep != PrepStage::Solve {
            if let Err(error) = self.advance_prep(operation) {
                return self.fault(&error);
            }
            context.consume_fuel(1);
            if self.prep == PrepStage::Solve {
                self.last_payload = self.empty_payload(false, false);
                return self.publish_tick(ToolRunState::Running, None, None);
            }
            return FillRun::Yield;
        }
        return self.drain_child(context);
    }

    /// 🤝️ Closes the lent tick turn by turn from the next step's own wallet.
    fn retire_delivered<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let publication = self.publication.as_mut().expect("a delivered fill publication is staged");
        let step = publication.close_step(context.retained_grant());
        context.consume_retained(step.progress())?;
        if let InteractiveJobCloseStep::Refused { kind, progress } = step {
            return Err(semio_framework_value::ValueError::literal(kind, "fill publication close was refused").with_retained_progress(progress));
        }
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    /// 📏️ Quotes the next close turn: the staged publication, the held verdict, then the child's own frontier.
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(FillSettled::Fault(message)) = self.settled.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: message.capacity(), depth: 1, ..Default::default() });
        }
        if self.settled.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(child) = self.child.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: child.next_close_copy_byte_demand()?, capacity_bytes: child.next_close_capacity_byte_demand(0)?, release_bytes: child.next_close_release_byte_demand()?, depth: child.next_close_depth_demand()?.saturating_add(1) });
        }
        Ok(semio_framework_value::RetirementDemand::default())
    }
}

impl InteractiveJob for Grid2dFillRunJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return self.retire_delivered(context);
        }
        if self.closing || context.is_cancelled() {
            let _ = self.writer.step(ToolRunStepKind::Warning, self.stage.index(), FillReason::Cancelled.code(), None, &[]);
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if self.publication.is_none() {
            match self.settled.take() {
                Some(FillSettled::Complete) => {
                    let result = JobOutcomeBorrow::admit_complete(context, None, None)?;
                    if result.is_none() {
                        self.settled = Some(FillSettled::Complete);
                    }
                    return Ok(result);
                }
                Some(FillSettled::Fault(message)) => {
                    self.publication = Some(wfc::job::Publication::new(wfc::job::PublicationKind::Fault, message, Vec::new()));
                }
                None => {}
            }
        }
        if self.publication.is_some() {
            return self.publication.as_mut().expect("a staged fill publication").poll(context);
        }
        match self.run(context) {
            FillRun::Yield | FillRun::Staged => Ok(None),
            FillRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fill outcome has no staged publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(child) = self.child.as_mut() {
            InteractiveJob::begin_close(child);
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        let demand = match self.close_demands() {
            Ok(demand) => demand,
            Err(error) => return InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(grant);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(settled) = self.settled.take() {
            let released_bytes = match settled {
                FillSettled::Fault(message) => message.capacity(),
                FillSettled::Complete => 0,
            };
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } };
        }
        if let Some(child) = self.child.as_mut() {
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
            let step = InteractiveJob::close_step(child, child_grant);
            if InteractiveJob::terminal_is_empty(child) {
                self.child = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.settled.is_none() && self.publication.is_none() && self.child.is_none()
    }
}

//#endregion 🧵️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
