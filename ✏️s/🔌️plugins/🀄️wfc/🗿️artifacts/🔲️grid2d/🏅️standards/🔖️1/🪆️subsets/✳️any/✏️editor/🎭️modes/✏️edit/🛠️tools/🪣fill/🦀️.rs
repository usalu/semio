//! 🪣 Edit-mode tool — Fill: the interactive collapse of `s.wfc.grid2d`. A non-mutating framework
//! tool run (`mutating: false`, `ToolRunTraceKind::None`) whose every step advances one preparation
//! unit or one `WfcJob::step`, publishes a partial-assignment payload the preview pane paints, and
//! on completion dispatches `commit-fill` so `Grid2dWindowConfig.solve_json` receives the finished
//! cache. Abort never writes that cache.

use crate::editor::grid2d::modes::edit::windows::preview;
use crate::schema::inferences::{solve_with_job, Grid2dInferenceCommit};
use crate::schema::snapshot::{Grid2dSnapshot, WfcDirection2d};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, Operation, StepContext, StepOutcome};
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, ToolRunJobPort};
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
/// 🧩 One cell in a fill tick: `tile_id` is absent while the cell is still open.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Grid2dFillPayload {
    pub assignments: Vec<Grid2dFillCell>,
    pub contradiction: bool,
    pub done: bool,
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
        ..semio_framework_plugin::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Fill", "Füllen"), "paint-bucket"))
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

/// 💾 `commit-fill` effect that lands the finished cache in `solve_json`.
pub fn commit_effect(commit: &Grid2dInferenceCommit) -> semio_framework::kernel::Effect {
    let args = serde_json::json!({ "solveJson": protocol::json::to_json_string(commit) });
    semio_framework::kernel::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: COMMIT_FILL_ACTION_ID.into(),
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
    Topology,
    Fixed,
    Solve,
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
    settled: Option<StepOutcome>,
    closing: bool,
    committed: bool,
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
            closing: false,
            committed: false,
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
        Grid2dFillPayload { assignments, contradiction, done }
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
        Grid2dFillPayload { assignments, contradiction: false, done }
    }

    /// 👁️ Builds the live payload from `observed()` — never from the truncated `incomplete_grid` publication.
    fn payload_from_child(&self, contradiction: bool, done: bool) -> Grid2dFillPayload {
        let Some(child) = self.child.as_ref() else {
            return self.empty_payload(contradiction, done);
        };
        let width = self.snapshot.width as usize;
        let mut decided = BTreeMap::<(u32, u32), String>::new();
        for (node, pattern) in child.observed() {
            let index = node.index();
            let x = (index % width) as u32;
            let y = (index / width) as u32;
            if self.is_masked(x, y) {
                continue;
            }
            if let Some(tile) = self.tile_ids.get(pattern.index()) {
                decided.insert((x, y), tile.clone());
            }
        }
        let mut assignments = Vec::new();
        for y in 0..self.snapshot.height {
            for x in 0..self.snapshot.width {
                if self.is_masked(x, y) {
                    continue;
                }
                assignments.push(Grid2dFillCell { x, y, tile_id: decided.get(&(x, y)).cloned() });
            }
        }
        Grid2dFillPayload { assignments, contradiction, done }
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
        self.writer.payload(self.last_payload.encode());
        match self.writer.finish().and_then(|tick| tick.encode().ok()).and_then(|bytes| context.payload_from_bytes(semio_framework_job::JobPayloadStream::Preview, &bytes).map_err(|rejected| drop(rejected.into_source())).ok()) {
            Some(payload) => StepOutcome::PreviewReady(payload),
            None => StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        }
    }

    fn fault(&mut self, context: &mut StepContext<'_>, message: &str) -> StepOutcome {
        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage.index(), FillReason::Fault.code(), None, &[]);
        self.last_payload = self.empty_payload(false, true);
        let settled = StepOutcome::Fault(semio_framework_job::JobFault {
            detail: match context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, message.as_bytes()) {
                Ok(payload) => payload,
                Err(rejected) => {
                    drop(rejected.into_source());
                    semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault)
                }
            },
        });
        let preview = self.publish_tick(context, ToolRunState::Faulted, Some((FillReason::Fault, &[])));
        match &preview {
            StepOutcome::PreviewReady(_) => {
                self.settled = Some(settled);
                preview
            }
            _ => {
                let mut settled = settled;
                while !matches!(settled.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                preview
            }
        }
    }

    fn finish_success(&mut self, context: &mut StepContext<'_>, contradiction: bool, payload: Option<Grid2dFillPayload>) -> StepOutcome {
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
        let settled = StepOutcome::Complete(semio_framework_job::CommitCandidate {
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
        });
        let preview = self.publish_tick(context, ToolRunState::Complete, Some((reason, &[decided])));
        match &preview {
            StepOutcome::PreviewReady(_) => {
                self.settled = Some(settled);
                preview
            }
            _ => {
                let mut settled = settled;
                while !matches!(settled.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                preview
            }
        }
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

impl InteractiveJob for Grid2dFillRunJob {
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
        if self.pending_tick {
            self.pending_tick = false;
            return self.publish_tick(context, ToolRunState::Running, None);
        }
        if self.snapshot.tiles.is_empty() || self.snapshot.width == 0 || self.snapshot.height == 0 {
            return self.finish_success(context, true, None);
        }
        let operation = self.bind_operation(context);
        if self.prep != PrepStage::Solve {
            if let Err(error) = self.advance_prep(operation) {
                return self.fault(context, &error);
            }
            context.consume_fuel(1);
            if self.prep == PrepStage::Solve {
                self.last_payload = self.empty_payload(false, false);
                return self.publish_tick(context, ToolRunState::Running, None);
            }
            return StepOutcome::Yield;
        }
        let Some(child) = self.child.as_mut() else {
            return self.fault(context, "wfc-grid2d-fill-missing-child");
        };
        let mut outcome = child.step(context);
        self.stage = FillStage::of(child.preview(0).stage);
        let (observations, _, backtracks) = child.metrics();
        self.observations = observations;
        self.backtracks = backtracks;
        match &outcome {
            StepOutcome::Complete(_) => {
                let commit = child.take_completed_commit();
                let contradiction = commit.is_none();
                let payload = if contradiction {
                    self.empty_payload(true, true)
                } else {
                    self.payload_from_commit(commit.as_ref().expect("completed commit"), true)
                };
                wfc::job::retire_outcome(&mut outcome);
                if let Some(mut child) = self.child.take() {
                    wfc::job::close_job(&mut child);
                }
                self.pending_finish = Some((contradiction, payload));
                return StepOutcome::Yield;
            }
            StepOutcome::Fault(fault) if wfc::job::payload_bytes(&fault.detail) == b"wfc-unsatisfiable" => {
                wfc::job::retire_outcome(&mut outcome);
                if let Some(mut child) = self.child.take() {
                    wfc::job::close_job(&mut child);
                }
                self.pending_finish = Some((true, self.empty_payload(true, true)));
                return StepOutcome::Yield;
            }
            StepOutcome::Cancelled => {
                wfc::job::retire_outcome(&mut outcome);
                return StepOutcome::Cancelled;
            }
            StepOutcome::Fault(_) => {
                wfc::job::retire_outcome(&mut outcome);
                return self.fault(context, "wfc-grid2d-fill-child-fault");
            }
            StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => {
                self.last_payload = self.payload_from_child(false, false);
                wfc::job::retire_outcome(&mut outcome);
                self.pending_tick = true;
                StepOutcome::Yield
            }
            StepOutcome::Yield => StepOutcome::Yield,
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.pending_tick = false;
        self.pending_finish = None;
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

//#endregion 🧵️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
