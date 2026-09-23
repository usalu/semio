//! 🔣️ Edit-mode tool — Fill: interactive WFC collapse as a non-mutating framework tool run.
//! Puzzle 3d supplies the start/pause/resume/step/abort shell; energy supplies the tick-payload
//! channel. The preview paints each tick; completion writes `SetSolve`; abort does not.

use crate::editor::wfc2d::modes::edit::windows::preview::WFC_2D_PREVIEW_WINDOW;
use crate::editor::wfc2d::transient::{SetSolve, Wfc2dAssignment, Wfc2dTransientMutation};
use crate::Wfc2dSnapshot;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::{Effect, Fault, LocalizedLabel, RequestId, ToolDefinition, ToolRunJobPurpose, ToolRunJobRequest};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy,
    ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceKind, ToolRunVerdict,
    TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_START_ACTION_ID,
};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_plugin_wfc_engine::ids::{NodeId, PatternId};
use semio_s_plugin_wfc_engine::job::{close_job, WfcJob, WfcJobConfig, WfcStage};
use semio_s_plugin_wfc_engine::model::ModelBuilder;
use semio_s_plugin_wfc_engine::topology::GraphTopologyBuild;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const RUN_JOB_KIND: &str = "s.wfc.wfc2d.fill.run";
pub const COMMIT_FILL_ACTION_ID: &str = "commit-fill";
pub const FILL_TICK_PAYLOAD_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
const FILL_HOP_REQUEST: u64 = 0x0077_fc2d_f111;
const MAX_ID_BYTES: usize = 1_024;
//#endregion 🔖️Constants

//#region 🔖️Vocabulary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillPrepStage {
    BuildModel,
    BuildTopology,
    ApplyPins,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillRunStage {
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

impl FillRunStage {
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
            Self::InitializeDomains => LocalizedLabel::native("Initializing domains", "Domänen werden initialisiert"),
            Self::FindMinimumEntropySlot => LocalizedLabel::native("Finding minimum entropy slot", "Slot mit minimaler Entropie wird gesucht"),
            Self::ChooseCandidate => LocalizedLabel::native("Choosing a candidate", "Kandidat wird gewählt"),
            Self::PropagateCompatibilityEdge => LocalizedLabel::native("Propagating compatibility", "Kompatibilität wird fortgepflanzt"),
            Self::DetectContradiction => LocalizedLabel::native("Detecting contradiction", "Widerspruch wird erkannt"),
            Self::BacktrackTrailEntry => LocalizedLabel::native("Backtracking", "Zurückverfolgen"),
            Self::CommitSlot => LocalizedLabel::native("Committing a slot", "Slot wird festgeschrieben"),
            Self::MaterializeCheckpoint => LocalizedLabel::native("Materializing checkpoint", "Prüfpunkt wird materialisiert"),
            Self::MaterializeCommit => LocalizedLabel::native("Materializing commit", "Commit wird materialisiert"),
            Self::Complete => LocalizedLabel::native("Complete", "Fertig"),
        }
    }

    fn from_engine(stage: WfcStage) -> Self {
        match stage {
            WfcStage::InitializeDomains => Self::InitializeDomains,
            WfcStage::FindMinimumEntropySlot => Self::FindMinimumEntropySlot,
            WfcStage::ChooseCandidate => Self::ChooseCandidate,
            WfcStage::PropagateCompatibilityEdge => Self::PropagateCompatibilityEdge,
            WfcStage::DetectContradiction => Self::DetectContradiction,
            WfcStage::BacktrackTrailEntry => Self::BacktrackTrailEntry,
            WfcStage::CommitSlot => Self::CommitSlot,
            WfcStage::MaterializeCheckpoint => Self::MaterializeCheckpoint,
            WfcStage::MaterializeCommit => Self::MaterializeCommit,
            WfcStage::Complete => Self::Complete,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillRunCounter {
    Observations,
    Decided,
    Backtracks,
}

impl FillRunCounter {
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
pub enum FillRunReason {
    Collapsed,
    Contradiction,
    Cancelled,
    Fault,
}

impl FillRunReason {
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
            Self::Collapsed => LocalizedLabel::native("Collapsed", "Kollabiert"),
            Self::Contradiction => LocalizedLabel::native("Contradiction", "Widerspruch"),
            Self::Cancelled => LocalizedLabel::native("Cancelled", "Abgebrochen"),
            Self::Fault => LocalizedLabel::native("Fault", "Fehler"),
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🧱️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dFillTraceEvent {
    pub slot_id: String,
    pub tile_id: String,
    pub discarded: bool,
}

/// 🧱️ Partial slot assignments the preview paints while a fill run is live.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dFillTickPayload {
    pub assignments: BTreeMap<String, Option<String>>,
    pub contradiction: bool,
    pub done: bool,
    #[value(default)]
    pub trace: Vec<Wfc2dFillTraceEvent>,
}

impl Wfc2dFillTickPayload {
    /// 🧮 How many slots already carry a tile.
    pub fn decided_count(&self) -> usize {
        self.assignments.values().filter(|tile| tile.is_some()).count()
    }

    /// 🏁️ The `SetSolve` mutations a successful (or contradictory) finish publishes.
    pub fn set_solve_mutations(&self) -> Vec<Wfc2dTransientMutation> {
        let assignments = self
            .assignments
            .iter()
            .filter_map(|(slot_id, tile_id)| tile_id.as_ref().map(|tile_id| Wfc2dAssignment { slot_id: slot_id.clone(), tile_id: tile_id.clone() }))
            .collect();
        vec![SetSolve { assignments, contradiction: self.contradiction }.into()]
    }
}

/// 📦️ Encodes one fill tick payload as UTF-8 JSON.
pub fn encode_fill_payload(payload: &Wfc2dFillTickPayload) -> Vec<u8> {
    protocol::json::to_json_string(payload).into_bytes()
}

/// 🔓️ Decodes a fill tick payload. `None` when the bytes are not a well-formed record.
pub fn decode_fill_payload(bytes: &[u8]) -> Option<Wfc2dFillTickPayload> {
    let text = std::str::from_utf8(bytes).ok()?;
    protocol::json::from_json_str(text).ok()
}

/// 🧮 Slot id → tile id (or absent) from the live child job. Reads `observed()` and singleton domains
/// through `domain_masks()` — never the truncated published `incomplete_grid`.
pub fn assignments_from_job(job: &WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>, tile_ids: &[String], snapshot: &Wfc2dSnapshot) -> BTreeMap<String, Option<String>> {
    let mut assignments: BTreeMap<String, Option<String>> = snapshot.slots.iter().map(|slot| (slot.id.clone(), None)).collect();
    for &(node, pattern) in job.observed() {
        let Some(slot) = snapshot.slots.get(node.index()) else { continue };
        let Some(tile) = tile_ids.get(pattern.index()) else { continue };
        assignments.insert(slot.id.clone(), Some(tile.clone()));
    }
    let masks = job.domain_masks();
    let _preview = job.preview(0);
    for (index, mask) in masks.iter().enumerate() {
        let Some(slot) = snapshot.slots.get(index) else { continue };
        if assignments.get(&slot.id).and_then(|tile| tile.as_ref()).is_some() {
            continue;
        }
        let mut found = None;
        let mut count = 0_u32;
        for pattern_index in 0..mask.len() {
            let pattern = PatternId::from_index(pattern_index);
            if !mask.get(pattern) {
                continue;
            }
            count += 1;
            found = Some(pattern_index);
            if count > 1 {
                break;
            }
        }
        if count == 1 {
            if let Some(pattern_index) = found {
                if let Some(tile) = tile_ids.get(pattern_index) {
                    assignments.insert(slot.id.clone(), Some(tile.clone()));
                }
            }
        }
    }
    assignments
}

/// 🏁 Payload that must match `solve_with_job` for the same snapshot.
pub fn payload_from_commit(snapshot: &Wfc2dSnapshot, commit: &crate::inferences::Wfc2dInferenceCommit) -> Wfc2dFillTickPayload {
    let assignments = snapshot
        .slots
        .iter()
        .map(|slot| (slot.id.clone(), commit.assignments.get(&slot.id).cloned()))
        .collect();
    Wfc2dFillTickPayload { assignments, contradiction: commit.contradiction, done: true, trace: Vec::new() }
}
//#endregion 🧱️Payload

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::wfc2d::create_wfc2d_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework_plugin::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Fill", "Füllen"), "paint-bucket")) }
}

/// ⏯️ Read-only collapse: restart on any base or settings change, no revalidate job, no trace subjects.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: LocalizedLabel::native("steps", "Schritte"),
        stages: FillRunStage::ALL.iter().map(|stage| semio_framework_tool_run::ToolRunStageDefinition { id: stage.id().into(), label: stage.label() }).collect(),
        counters: FillRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() }).collect(),
        reasons: FillRunReason::ALL
            .iter()
            .map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() })
            .collect(),
        trace: ToolRunTraceKind::None,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads::default(),
        windows: vec![WFC_2D_PREVIEW_WINDOW.into()],
    }
}

/// ▶️ Effects that start a fill run from the existing `solve` action.
pub fn start_fill_effects() -> Vec<Effect> {
    vec![
        Effect::SetActiveTool { tool_id: TOOL_ID.into() },
        Effect::DispatchAction {
            req: RequestId(FILL_HOP_REQUEST),
            action: TOOL_RUN_START_ACTION_ID.into(),
            args: Some(dsl::DslValue::object([(TOOL_RUN_ARG_TOOL_ID.to_string(), dsl::DslValue::String(TOOL_ID.into()))])),
            delay_ms: 0,
        },
    ]
}

fn commit_fill_effect(payload: &Wfc2dFillTickPayload) -> Effect {
    let args = serde_json::json!({ "payloadJson": protocol::json::to_json_string(payload) });
    Effect::DispatchAction {
        req: RequestId(FILL_HOP_REQUEST ^ 1),
        action: COMMIT_FILL_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(args)),
        delay_ms: 0,
    }
}
//#endregion 🔖️Definition

//#region 🎲️Prepare
fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() || id.len() > MAX_ID_BYTES {
        return Err("wfc2d-fill-id-invalid".into());
    }
    Ok(())
}

struct FillPreparation {
    snapshot: Wfc2dSnapshot,
    stage: FillPrepStage,
    pattern_of: BTreeMap<String, PatternId>,
    tile_ids: Vec<String>,
    raw_weights: Vec<f64>,
    relation_names: Vec<String>,
    relation_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::RelationId>,
    builder: Option<ModelBuilder>,
    model: Option<semio_s_plugin_wfc_engine::model::CompiledModel>,
    topology_build: Option<GraphTopologyBuild>,
    topology: Option<semio_s_plugin_wfc_engine::topology::GraphTopology>,
    node_of: BTreeMap<String, NodeId>,
    fixed: Vec<(NodeId, PatternId)>,
}

impl FillPreparation {
    fn new(snapshot: Wfc2dSnapshot) -> Self {
        Self {
            snapshot,
            stage: FillPrepStage::BuildModel,
            pattern_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            raw_weights: Vec::new(),
            relation_names: Vec::new(),
            relation_of: BTreeMap::new(),
            builder: None,
            model: None,
            topology_build: None,
            topology: None,
            node_of: BTreeMap::new(),
            fixed: Vec::new(),
        }
    }

    fn build_model(&mut self) -> Result<(), String> {
        for (index, tile) in self.snapshot.tiles.iter().enumerate() {
            validate_id(&tile.id)?;
            self.pattern_of.insert(tile.id.clone(), PatternId::from_index(index));
            self.tile_ids.push(tile.id.clone());
            self.raw_weights.push(if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 });
        }
        for edge in &self.snapshot.edges {
            validate_id(&edge.relation)?;
            if !self.relation_names.contains(&edge.relation) {
                self.relation_names.push(edge.relation.clone());
            }
        }
        if self.snapshot.slots.is_empty() || self.raw_weights.is_empty() {
            return Ok(());
        }
        self.relation_names.sort();
        if self.relation_names.is_empty() {
            self.relation_names.push(crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.to_string());
        }
        let mut builder = ModelBuilder::new();
        for weight in &self.raw_weights {
            builder.add_pattern(*weight);
        }
        for name in &self.relation_names {
            let relation = builder.add_relation(name);
            self.relation_of.insert(name.clone(), relation);
        }
        for rule in &self.snapshot.rules {
            validate_id(&rule.id)?;
            validate_id(&rule.tile_a_id)?;
            validate_id(&rule.tile_b_id)?;
            let Some(&a) = self.pattern_of.get(&rule.tile_a_id) else { continue };
            let Some(&b) = self.pattern_of.get(&rule.tile_b_id) else { continue };
            let relations: Vec<_> = match &rule.relation {
                Some(name) => self.relation_of.get(name).copied().into_iter().collect(),
                None => self.relation_of.values().copied().collect(),
            };
            for relation in relations {
                if rule.allowed {
                    builder.allow(relation, a, b);
                    builder.allow(relation, b, a);
                } else {
                    builder.deny(relation, a, b);
                    builder.deny(relation, b, a);
                }
            }
        }
        self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
        Ok(())
    }

    fn build_topology(&mut self) -> Result<(), String> {
        let mut topology_build = GraphTopologyBuild::new(self.snapshot.slots.len());
        for (index, slot) in self.snapshot.slots.iter().enumerate() {
            validate_id(&slot.id)?;
            if let Some(pinned) = &slot.pinned_tile_id {
                validate_id(pinned)?;
            }
            self.node_of.insert(slot.id.clone(), NodeId::from_index(index));
        }
        for edge in &self.snapshot.edges {
            validate_id(&edge.from_slot_id)?;
            validate_id(&edge.to_slot_id)?;
            if let (Some(&from), Some(&to), Some(&relation)) = (self.node_of.get(&edge.from_slot_id), self.node_of.get(&edge.to_slot_id), self.relation_of.get(&edge.relation)) {
                topology_build.add_arc(from, to, relation).map_err(|error| format!("{error:?}"))?;
                topology_build.add_arc(to, from, relation).map_err(|error| format!("{error:?}"))?;
            }
        }
        loop {
            if let Some(topology) = topology_build.step() {
                self.topology = Some(topology);
                break;
            }
        }
        Ok(())
    }

    fn apply_pins(&mut self, operation: Operation) -> Result<WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>, String> {
        for slot in &self.snapshot.slots {
            if let Some(pinned) = &slot.pinned_tile_id {
                if let (Some(&node), Some(&pattern)) = (self.node_of.get(&slot.id), self.pattern_of.get(pinned)) {
                    self.fixed.push((node, pattern));
                }
            }
        }
        let model = self.model.take().ok_or("wfc2d-fill-model-missing")?;
        let topology = self.topology.take().ok_or("wfc2d-fill-topology-missing")?;
        let fixed = std::mem::take(&mut self.fixed);
        Ok(WfcJob::new(operation, model, topology, WfcJobConfig::default(), None, fixed))
    }
}
//#endregion 🎲️Prepare

//#region 🎲️Job
enum FillPhase {
    Prepare(Box<FillPreparation>),
    Run {
        snapshot: Wfc2dSnapshot,
        tile_ids: Vec<String>,
        child: Option<Box<WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>>>,
    },
    Settled,
    Closed,
}

/// ⏯️ The fill tool's run job: three preparation units, then one engine `WfcJob::step` per framework step.
pub struct Wfc2dFillRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    port: semio_framework_plugin::ToolRunJobPort,
    operation: Option<Operation>,
    phase: FillPhase,
    set_solve: Option<Vec<Wfc2dTransientMutation>>,
    pending_tick: Option<(Wfc2dFillTickPayload, FillRunStage)>,
    pending_finish: Option<Wfc2dFillTickPayload>,
    settled: Option<StepOutcome>,
    closing: bool,
    steps: u64,
    trace: Vec<Wfc2dFillTraceEvent>,
}

impl Wfc2dFillRunJob {
    pub fn new(identity: ToolRunIdentity, snapshot: Arc<Wfc2dSnapshot>, port: semio_framework_plugin::ToolRunJobPort) -> Self {
        Self {
            identity,
            writer: ToolRunTickWriter::new(identity),
            port,
            operation: None,
            phase: FillPhase::Prepare(Box::new(FillPreparation::new((*snapshot).clone()))),
            set_solve: None,
            pending_tick: None,
            pending_finish: None,
            settled: None,
            closing: false,
            steps: 0,
            trace: Vec::new(),
        }
    }


    /// 🔗 Binds the child `WfcJob` to the same operation and generation the framework drives this run with.
    fn bind_operation(&mut self, context: &StepContext<'_>) -> Operation {
        if let Some(operation) = self.operation {
            return operation;
        }
        let seed = match &self.phase {
            FillPhase::Prepare(preparation) => preparation.snapshot.seed,
            FillPhase::Run { snapshot, .. } => snapshot.seed,
            FillPhase::Settled | FillPhase::Closed => 0,
        };
        let operation = Operation::new(context.operation(), semio_framework_job::RevisionId(0), context.generation(), seed);
        self.operation = Some(operation);
        operation
    }

    /// 🏁️ `SetSolve` mutations produced on a successful finish; `None` after abort or before commit.
    pub fn take_set_solve(&mut self) -> Option<Vec<Wfc2dTransientMutation>> {
        self.set_solve.take()
    }

    fn empty_payload(snapshot: &Wfc2dSnapshot) -> Wfc2dFillTickPayload {
        Wfc2dFillTickPayload {
            assignments: snapshot.slots.iter().map(|slot| (slot.id.clone(), None)).collect(),
            contradiction: false,
            done: false,
            trace: Vec::new(),
        }
    }

    fn publish(&mut self, cx: &mut StepContext<'_>, payload: Wfc2dFillTickPayload, stage: FillRunStage, state: ToolRunState, reason: Option<FillRunReason>) -> StepOutcome {
        let decided = payload.decided_count() as u64;
        let (observations, backtracks) = if self.closing {
            (0, 0)
        } else {
            match &self.phase {
                FillPhase::Run { child: Some(child), .. } => {
                    let preview = child.preview(0);
                    (preview.observations, preview.backtracks)
                }
                FillPhase::Run { child: None, .. } => (0, 0),
                _ => (0, 0),
            }
        };
        if let Some(reason) = reason {
            let _ = self.writer.step(ToolRunStepKind::Info, stage.index(), reason.code(), None, &[]);
        }
        self.writer.progress(ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: stage.index(),
            completed: decided,
            total: Some(payload.assignments.len() as u64),
            counters: vec![
                ToolRunCounter { counter: FillRunCounter::Observations.index(), value: observations },
                ToolRunCounter { counter: FillRunCounter::Decided.index(), value: decided },
                ToolRunCounter { counter: FillRunCounter::Backtracks.index(), value: backtracks },
            ],
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        });
        let mut payload = payload;
        payload.trace.clone_from(&self.trace);
        self.writer.payload(encode_fill_payload(&payload));
        match self.writer.finish().and_then(|tick| tick.encode().ok()).and_then(|bytes| cx.payload_from_bytes(JobPayloadStream::Preview, bytes.as_slice()).map_err(|rejected| drop(rejected.into_source())).ok()) {
            Some(payload) => StepOutcome::PreviewReady(payload),
            None => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
        }
    }

    fn finish_success(&mut self, cx: &mut StepContext<'_>, payload: Wfc2dFillTickPayload) -> StepOutcome {
        self.set_solve = Some(payload.set_solve_mutations());
        self.port.dispatch(commit_fill_effect(&payload));
        let reason = if payload.contradiction { FillRunReason::Contradiction } else { FillRunReason::Collapsed };
        let settled = StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: RetainedJobPayload::empty(JobPayloadStream::CommitState),
            output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
        });
        let tick = self.publish(cx, payload, FillRunStage::Complete, ToolRunState::Complete, Some(reason));
        match &tick {
            StepOutcome::PreviewReady(_) => {
                self.settled = Some(settled);
                self.phase = FillPhase::Settled;
                tick
            }
            _ => {
                let mut settled = settled;
                while !matches!(settled.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                tick
            }
        }
    }


    fn remember(&mut self, event: Wfc2dFillTraceEvent) {
        self.trace.push(event);
        if self.trace.len() > 512 {
            let overflow = self.trace.len() - 512;
            self.trace.drain(0..overflow);
        }
    }

    fn drain_child(&mut self, cx: &mut StepContext<'_>, snapshot: Wfc2dSnapshot, tile_ids: Vec<String>, mut child: Box<WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>>) -> StepOutcome {
        let cells = snapshot.slots.len();
        let mut flips = Vec::new();
        let mut fresh = 0usize;
        let mut solved = false;
        let mut unsatisfiable = false;
        loop {
            if cx.is_cancelled() {
                if let FillPhase::Run { child: slot, .. } = &mut self.phase {
                    *slot = Some(child);
                }
                return StepOutcome::Cancelled;
            }
            if cx.fuel_exhausted() || cx.deadline_exceeded() {
                break;
            }
            let pulse = child.advance_one();
            cx.consume_fuel(1);
            child.drain_visible(&mut flips);
            for flip in flips.drain(..) {
                let Some(slot) = snapshot.slots.get(flip.node as usize) else { continue };
                if let Some(tile) = tile_ids.get(flip.pattern as usize) {
                    self.remember(Wfc2dFillTraceEvent { slot_id: slot.id.clone(), tile_id: tile.clone(), discarded: flip.discarded });
                    fresh += 1;
                }
            }
            match pulse {
                semio_s_plugin_wfc_engine::job::SearchPulse::Solved => {
                    solved = true;
                    break;
                }
                semio_s_plugin_wfc_engine::job::SearchPulse::Unsatisfiable => {
                    unsatisfiable = true;
                    break;
                }
                semio_s_plugin_wfc_engine::job::SearchPulse::Collapsed { .. } | semio_s_plugin_wfc_engine::job::SearchPulse::Discarded { .. } | semio_s_plugin_wfc_engine::job::SearchPulse::Worked => {}
            }
            if fresh >= 48 {
                break;
            }
        }
        if solved || unsatisfiable {
            let commit = if solved { child.commit() } else { None };
            close_job(child.as_mut());
            self.phase = FillPhase::Settled;
            let payload = if let Some(commit) = commit {
                let assignments = snapshot.slots.iter().enumerate().map(|(index, slot)| {
                    let tile = commit.assignment.get(index).and_then(|pattern| tile_ids.get(*pattern as usize).cloned());
                    (slot.id.clone(), tile)
                }).collect();
                Wfc2dFillTickPayload { assignments, contradiction: false, done: true, trace: Vec::new() }
            } else {
                Wfc2dFillTickPayload {
                    assignments: snapshot.slots.iter().map(|slot| (slot.id.clone(), None)).collect(),
                    contradiction: true,
                    done: true,
                    trace: Vec::new(),
                }
            };
            self.pending_finish = Some(payload);
            return StepOutcome::Yield;
        }
        let stage = FillRunStage::from_engine(child.preview(0).stage);
        let mut patterns = Vec::new();
        child.write_singleton_patterns(&mut patterns);
        if let FillPhase::Run { child: slot, .. } = &mut self.phase {
            *slot = Some(child);
        }
        if fresh == 0 {
            return StepOutcome::Yield;
        }
        let assignments = snapshot.slots.iter().enumerate().map(|(index, slot)| {
            let tile = patterns.get(index).copied().filter(|pattern| *pattern != u32::MAX).and_then(|pattern| tile_ids.get(pattern as usize).cloned());
            (slot.id.clone(), tile)
        }).collect();
        let payload = Wfc2dFillTickPayload { assignments, contradiction: false, done: false, trace: Vec::new() };
        self.publish(cx, payload, stage, ToolRunState::Running, None)
    }

    fn finish_fault(&mut self, cx: &mut StepContext<'_>, snapshot: &Wfc2dSnapshot, _detail: &str) -> StepOutcome {
        let mut payload = Self::empty_payload(snapshot);
        payload.done = true;
        let _ = self.writer.step(ToolRunStepKind::Danger, FillRunStage::Complete.index(), FillRunReason::Fault.code(), None, &[]);
        let tick = self.publish(cx, payload, FillRunStage::Complete, ToolRunState::Faulted, Some(FillRunReason::Fault));
        self.settled = Some(StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }));
        self.phase = FillPhase::Settled;
        tick
    }
}

impl InteractiveJob for Wfc2dFillRunJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(outcome) = self.settled.take() {
            return outcome;
        }
        if let Some(payload) = self.pending_finish.take() {
            return self.finish_success(cx, payload);
        }
        if let Some((payload, stage)) = self.pending_tick.take() {
            return self.publish(cx, payload, stage, ToolRunState::Running, None);
        }
        self.steps = self.steps.saturating_add(1);
        let operation = self.bind_operation(cx);
        match &mut self.phase {
            FillPhase::Prepare(preparation) => {
                let snapshot = preparation.snapshot.clone();
                let result = match preparation.stage {
                    FillPrepStage::BuildModel => {
                        let built = preparation.build_model();
                        preparation.stage = FillPrepStage::BuildTopology;
                        built.map(|_| None)
                    }
                    FillPrepStage::BuildTopology => {
                        let built = preparation.build_topology();
                        preparation.stage = FillPrepStage::ApplyPins;
                        built.map(|_| None)
                    }
                    FillPrepStage::ApplyPins => preparation.apply_pins(operation).map(Some),
                };
                match result {
                    Ok(None) => {
                        cx.consume_fuel(1);
                        self.publish(cx, Self::empty_payload(&snapshot), FillRunStage::InitializeDomains, ToolRunState::Running, None)
                    }
                    Ok(Some(child)) => {
                        let tile_ids = preparation.tile_ids.clone();
                        self.phase = FillPhase::Run { snapshot: snapshot.clone(), tile_ids, child: Some(Box::new(child)) };
                        cx.consume_fuel(1);
                        self.publish(cx, Self::empty_payload(&snapshot), FillRunStage::InitializeDomains, ToolRunState::Running, None)
                    }
                    Err(error) => {
                        cx.consume_fuel(1);
                        self.finish_fault(cx, &snapshot, &error)
                    }
                }
            }
            FillPhase::Run { .. } => {
                let (snapshot, tile_ids, child) = match &mut self.phase {
                    FillPhase::Run { snapshot, tile_ids, child } => (snapshot.clone(), tile_ids.clone(), child.take().expect("fill child")),
                    _ => unreachable!(),
                };
                self.drain_child(cx, snapshot, tile_ids, child)
            }
            FillPhase::Settled | FillPhase::Closed => StepOutcome::Cancelled,
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.pending_tick = None;
        self.pending_finish = None;
        if let FillPhase::Run { child: Some(child), .. } = &mut self.phase {
            InteractiveJob::begin_close(child.as_mut());
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
        if let FillPhase::Run { child, .. } = &mut self.phase {
            if let Some(child_job) = child.as_mut() {
                return match InteractiveJob::close_step(child_job.as_mut(), maximum_items, maximum_bytes) {
                    InteractiveJobCloseStep::Complete => {
                        *child = None;
                        self.phase = FillPhase::Closed;
                        InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                    pending => pending,
                };
            }
        }
        self.phase = FillPhase::Closed;
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.settled.is_none() && matches!(self.phase, FillPhase::Closed)
    }
}
//#endregion 🎲️Job

//#region 🏗️Factory
/// 🏗️ Builds the fill run job for the framework ledger.
pub fn build_tool_run_job(request: ToolRunJobRequest<'_, semio_framework_plugin::EditorApp<crate::editor::wfc2d::Wfc2dEditor>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
    if request.tool_id != TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
        return Ok(None);
    }
    Ok(Some(Box::new(Wfc2dFillRunJob::new(request.identity, request.snapshot.clone(), request.port.clone()))))
}
//#endregion 🏗️Factory

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
