//! 💡️ WFC 2D inferences — THE SOLVE ITSELF IS AN INFERENCE. `Wfc2dSnapshot` only ever persists the
//! PROBLEM (slots/edges/tiles/rules/seed); the SOLUTION, the contradiction verdict and the entropy
//! map are all derived here, never mutation-authored state.
//!
//! The compile route is assembly's graph route generalised to NAMED RELATIONS: every distinct
//! `Wfc2dSlotEdge::relation` string becomes its own `RelationId` in a `ModelBuilder`, a rule with
//! `relation: None` applies to all of them, and `allowed: false` compiles to a `deny` (which the
//! model resolves after every `allow`, so a deny always wins). The topology is the engine's
//! production `GraphTopologyBuild`; the solver is the resumable `WfcJob<GraphTopology>` interactive
//! callers drive, stepped here under an explicit fuel/step budget so every step is watchdog-bounded.
//!
//! Determinism: `solve_with_job` reads only snapshot fields (`seed` included) and no ambient
//! randomness enters, so `DepHash` caching over `Wfc2dSolve`/`Wfc2dContradiction`/`Wfc2dEntropy` is
//! sound.

use crate::schema::snapshot::Wfc2dSnapshot;
use std::collections::{BTreeMap, BTreeSet};

//#region 📦️RetainedPayload
/// 📦️ One single-page payload, with the job module's EXACT source handback on refusal. Dropping a
/// `JobPayloadRejectedPage` without taking its source back trips that module's own lifecycle
/// assertion and then aborts from a second panic inside `RetainedJobPayload::drop`. An empty payload
/// is the honest answer to a refused admission; leaking the page never was.
fn retained_payload(context: &mut semio_framework_job::StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
    match context.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            semio_framework_job::RetainedJobPayload::empty(stream)
        }
    }
}
/// ♻️ Releases every page a retained payload still owns. A `RetainedJobPayload` that reaches `Drop`
/// without a one-page close ladder asserts — the CHILD job's `CommitCandidate` carries TWO of them
/// (`state` and `output`) and this parent keeps only the one it forwards, so the other must be
/// retired here rather than dropped.
fn retire_payload(mut payload: semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}
//#endregion 📦️RetainedPayload

//#region 🔖️Compile
pub const WFC_2D_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const WFC_2D_INFERENCE_TOOL_ID: &str = "s.wfc.wfc2d.solve";
pub const WFC_2D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.wfc2d.inference.request.v1";

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn wfc2d_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.wfc2d",
        artifact_schema: "s.wfc.wfc2d",
        artifact_schema_version: 1,
        inference_schema: WFC_2D_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    }
}

const MAX_WFC_2D_TILES: usize = 65_536;
const MAX_WFC_2D_SLOTS: usize = 65_536;
const MAX_WFC_2D_RULES: usize = 262_144;
const MAX_WFC_2D_EDGES: usize = 262_144;
const MAX_WFC_2D_ID_BYTES: usize = 1_024;
const MAX_WFC_2D_OUTPUT_BYTES: usize = 1 << 20;
const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dInferenceRequest {
    pub snapshot: Wfc2dSnapshot,
    pub checkpoint: Option<Vec<u8>>,
}

/// 🏁 What `s.wfc.wfc2d.solve` commits: the assignment, the satisfiability verdict, and the
/// pre-propagation entropy map. None of it is ever written back into the document.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dInferenceCommit {
    pub assignments: BTreeMap<String, String>,
    pub contradiction: bool,
    pub entropy: BTreeMap<String, f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Wfc2dInferenceStage {
    Tiles,
    Relations,
    Rules,
    Model,
    Slots,
    Edges,
    Topology,
    Fixed,
    Restore,
    Solve,
    MapCommit,
    EncodeCommit,
    Complete,
}

/// 🧵 Worker-owned parent transaction for the wfc2d compile, solve, restore and mapping stages.
pub struct Wfc2dInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: Wfc2dSnapshot,
    stage: Wfc2dInferenceStage,
    cursor: usize,
    pattern_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::PatternId>,
    node_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::NodeId>,
    tile_ids: Vec<String>,
    raw_weights: Vec<f64>,
    relation_names: Vec<String>,
    relation_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::RelationId>,
    builder: Option<semio_s_plugin_wfc_engine::model::ModelBuilder>,
    model: Option<semio_s_plugin_wfc_engine::model::CompiledModel>,
    topology_build: Option<semio_s_plugin_wfc_engine::topology::GraphTopologyBuild>,
    topology: Option<semio_s_plugin_wfc_engine::topology::GraphTopology>,
    fixed: Vec<(semio_s_plugin_wfc_engine::ids::NodeId, semio_s_plugin_wfc_engine::ids::PatternId)>,
    checkpoint: Option<Vec<u8>>,
    restore: Option<semio_s_plugin_wfc_engine::job::WfcRestore<semio_s_plugin_wfc_engine::topology::GraphTopology>>,
    child: Option<semio_s_plugin_wfc_engine::job::WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>>,
    child_commit: Option<semio_s_plugin_wfc_engine::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    commit: Wfc2dInferenceCommit,
    commit_bytes: Vec<u8>,
    commit_cursor: usize,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    preview_units: u64,
    last_preview_ms: Option<u64>,
    closing: bool,
}

impl Wfc2dInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: Wfc2dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.snapshot;
        if snapshot.tiles.len() > MAX_WFC_2D_TILES
            || snapshot.slots.len() > MAX_WFC_2D_SLOTS
            || snapshot.rules.len() > MAX_WFC_2D_RULES
            || snapshot.edges.len() > MAX_WFC_2D_EDGES
            || snapshot.edges.len().saturating_mul(2) > u32::MAX as usize
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > semio_s_plugin_wfc_engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("wfc2d-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: Wfc2dInferenceStage::Tiles,
            cursor: 0,
            pattern_of: BTreeMap::new(),
            node_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            raw_weights: Vec::new(),
            relation_names: Vec::new(),
            relation_of: BTreeMap::new(),
            builder: None,
            model: None,
            topology_build: None,
            topology: None,
            fixed: Vec::new(),
            checkpoint: request.checkpoint,
            restore: None,
            child: None,
            child_commit: None,
            final_checkpoint: None,
            commit: Wfc2dInferenceCommit::default(),
            commit_bytes: Vec::new(),
            commit_cursor: 0,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            preview_units: 0,
            last_preview_ms: None,
            closing: false,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_WFC_2D_ID_BYTES {
            Err("wfc2d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    fn progress(&self) -> (usize, usize) {
        match self.stage {
            Wfc2dInferenceStage::Tiles => (self.cursor, self.snapshot.tiles.len()),
            Wfc2dInferenceStage::Relations => (self.cursor, self.snapshot.edges.len()),
            Wfc2dInferenceStage::Rules => (self.cursor, self.snapshot.rules.len()),
            Wfc2dInferenceStage::Model => (0, 1),
            Wfc2dInferenceStage::Slots | Wfc2dInferenceStage::Fixed | Wfc2dInferenceStage::MapCommit => (self.cursor, self.snapshot.slots.len()),
            Wfc2dInferenceStage::Edges => (self.cursor, self.snapshot.edges.len()),
            Wfc2dInferenceStage::Topology => self.topology_build.as_ref().map_or((0, self.snapshot.slots.len()), |build| build.progress()),
            Wfc2dInferenceStage::Restore | Wfc2dInferenceStage::Solve => (0, 1),
            Wfc2dInferenceStage::EncodeCommit => (self.commit_cursor, self.commit_bytes.len()),
            Wfc2dInferenceStage::Complete => (1, 1),
        }
    }

    fn emit_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        let (completed, total) = self.progress();
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        };
        let mut preview = [0; 25];
        preview[..8].copy_from_slice(&sequence.to_le_bytes());
        preview[8..16].copy_from_slice(&(completed as u64).to_le_bytes());
        preview[16..24].copy_from_slice(&(total as u64).to_le_bytes());
        preview[24] = self.stage as u8;
        self.preview_units = 0;
        self.last_preview_ms = context.now_us().map(|now_us| now_us / 1_000);
        let payload = retained_payload(context, semio_framework_job::JobPayloadStream::Preview, &preview);
        semio_framework_job::StepOutcome::PreviewReady(payload)
    }

    fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    /// 🏗️ One relation per distinct edge `relation` string. Each is SELF-INVERSE and every arc is
    /// added in both directions, so adjacency stays symmetric exactly as assembly's single-relation
    /// route was — the relation only scopes WHICH rules apply, never a direction.
    fn compile_rules(&mut self) -> Result<(), String> {
        let builder = self.builder.as_mut().ok_or("wfc2d-model-builder-missing")?;
        let Some(rule) = self.snapshot.rules.get(self.cursor) else {
            return Ok(());
        };
        Self::validate_id(&rule.tile_a_id)?;
        Self::validate_id(&rule.tile_b_id)?;
        let (Some(&a), Some(&b)) = (self.pattern_of.get(&rule.tile_a_id), self.pattern_of.get(&rule.tile_b_id)) else {
            return Ok(());
        };
        let scoped: Vec<semio_s_plugin_wfc_engine::ids::RelationId> = match &rule.relation {
            Some(name) => self.relation_of.get(name).copied().into_iter().collect(),
            None => self.relation_of.values().copied().collect(),
        };
        for relation in scoped {
            if rule.allowed {
                builder.allow(relation, a, b);
                builder.allow(relation, b, a);
            } else {
                builder.deny(relation, a, b);
                builder.deny(relation, b, a);
            }
        }
        Ok(())
    }

    fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            Wfc2dInferenceStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    Self::validate_id(&tile.id)?;
                    let pattern = semio_s_plugin_wfc_engine::ids::PatternId::from_index(self.cursor);
                    self.pattern_of.insert(tile.id.clone(), pattern);
                    self.tile_ids.push(tile.id.clone());
                    self.raw_weights.push(if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 });
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc2dInferenceStage::Relations;
                }
            }
            Wfc2dInferenceStage::Relations => {
                if let Some(edge) = self.snapshot.edges.get(self.cursor) {
                    Self::validate_id(&edge.relation)?;
                    if !self.relation_names.contains(&edge.relation) {
                        self.relation_names.push(edge.relation.clone());
                    }
                    self.cursor += 1;
                } else if self.snapshot.slots.is_empty() || self.raw_weights.is_empty() {
                    self.commit_bytes = protocol::json::to_json_string(&self.commit).into_bytes();
                    self.stage = Wfc2dInferenceStage::EncodeCommit;
                } else {
                    self.relation_names.sort();
                    if self.relation_names.is_empty() {
                        self.relation_names.push(crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.to_string());
                    }
                    let mut builder = semio_s_plugin_wfc_engine::model::ModelBuilder::new();
                    for weight in &self.raw_weights {
                        builder.add_pattern(*weight);
                    }
                    for name in &self.relation_names {
                        let relation = builder.add_relation(name);
                        self.relation_of.insert(name.clone(), relation);
                    }
                    self.builder = Some(builder);
                    self.cursor = 0;
                    self.stage = Wfc2dInferenceStage::Rules;
                }
            }
            Wfc2dInferenceStage::Rules => {
                if self.cursor < self.snapshot.rules.len() {
                    self.compile_rules()?;
                    self.cursor += 1;
                } else {
                    let builder = self.builder.take().ok_or("wfc2d-model-builder-missing")?;
                    self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                    self.stage = Wfc2dInferenceStage::Model;
                }
            }
            Wfc2dInferenceStage::Model => {
                self.cursor = 0;
                self.topology_build = Some(semio_s_plugin_wfc_engine::topology::GraphTopologyBuild::new(self.snapshot.slots.len()));
                self.stage = Wfc2dInferenceStage::Slots;
            }
            Wfc2dInferenceStage::Slots => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    Self::validate_id(&slot.id)?;
                    if let Some(pinned) = &slot.pinned_tile_id {
                        Self::validate_id(pinned)?;
                    }
                    self.node_of.insert(slot.id.clone(), semio_s_plugin_wfc_engine::ids::NodeId::from_index(self.cursor));
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc2dInferenceStage::Edges;
                }
            }
            Wfc2dInferenceStage::Edges => {
                if let Some(edge) = self.snapshot.edges.get(self.cursor) {
                    Self::validate_id(&edge.from_slot_id)?;
                    Self::validate_id(&edge.to_slot_id)?;
                    if let (Some(&from), Some(&to), Some(&relation)) = (self.node_of.get(&edge.from_slot_id), self.node_of.get(&edge.to_slot_id), self.relation_of.get(&edge.relation)) {
                        let topology = self.topology_build.as_mut().ok_or("wfc2d-topology-build-missing")?;
                        topology.add_arc(from, to, relation).map_err(|error| format!("{error:?}"))?;
                        topology.add_arc(to, from, relation).map_err(|error| format!("{error:?}"))?;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc2dInferenceStage::Topology;
                }
            }
            Wfc2dInferenceStage::Topology => {
                if let Some(topology) = self.topology_build.as_mut().ok_or("wfc2d-topology-build-missing")?.step() {
                    self.topology = Some(topology);
                    self.topology_build = None;
                    self.cursor = 0;
                    self.stage = Wfc2dInferenceStage::Fixed;
                }
            }
            Wfc2dInferenceStage::Fixed => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    if let Some(pinned) = &slot.pinned_tile_id {
                        if let (Some(&node), Some(&pattern)) = (self.node_of.get(&slot.id), self.pattern_of.get(pinned)) {
                            self.fixed.push((node, pattern));
                        }
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().ok_or("wfc2d-compiled-model-missing")?;
                    let topology = self.topology.take().ok_or("wfc2d-compiled-topology-missing")?;
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(semio_s_plugin_wfc_engine::job::WfcRestore::new(self.operation, model, topology, semio_s_plugin_wfc_engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = Wfc2dInferenceStage::Restore;
                    } else {
                        self.child = Some(semio_s_plugin_wfc_engine::job::WfcJob::new(self.operation, model, topology, semio_s_plugin_wfc_engine::job::WfcJobConfig::default(), None, fixed));
                        self.stage = Wfc2dInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile wfc2d inference stage"),
        }
        Ok(())
    }

    /// 🩺 The unsatisfiable answer: no assignment, the verdict set, and the PRIOR entropy map still
    /// filled in — a caller asking "why is this stuck" gets the per-slot freedom it had to begin with.
    fn enter_contradiction(&mut self) {
        self.child_commit = None;
        self.commit.assignments.clear();
        self.commit.contradiction = true;
        for slot in &self.snapshot.slots {
            self.commit.entropy.insert(slot.id.clone(), slot_entropy(&self.snapshot, slot));
        }
        self.commit_bytes = protocol::json::to_json_string(&self.commit).into_bytes();
        self.commit_cursor = 0;
        self.stage = Wfc2dInferenceStage::EncodeCommit;
    }

    fn map_one(&mut self) -> Result<(), String> {
        let commit = self.child_commit.as_ref().ok_or("wfc2d-commit-missing")?;
        if self.cursor < self.snapshot.slots.len() {
            let slot = &self.snapshot.slots[self.cursor];
            let pattern = usize::try_from(*commit.assignment.get(self.cursor).ok_or("wfc2d-commit-missing-slot")?).map_err(|_| "wfc2d-commit-pattern-capacity")?;
            let tile = self.tile_ids.get(pattern).ok_or("wfc2d-commit-pattern-out-of-range")?;
            self.commit.assignments.insert(slot.id.clone(), tile.clone());
            self.commit.entropy.insert(slot.id.clone(), slot_entropy(&self.snapshot, slot));
            self.cursor += 1;
        } else {
            self.child_commit = None;
            self.commit.contradiction = false;
            self.commit_bytes = protocol::json::to_json_string(&self.commit).into_bytes();
            if self.commit_bytes.len() > MAX_WFC_2D_OUTPUT_BYTES {
                return Err("wfc2d-inference-output-admission-exceeded".into());
            }
            self.stage = Wfc2dInferenceStage::EncodeCommit;
        }
        Ok(())
    }
}

/// 🎲 One slot's PRE-PROPAGATION Shannon entropy over the tile weight distribution — `0.0` for a
/// pinned slot (fully determined). Honest scope: this is the prior, before arc consistency narrows
/// any domain; wiring the post-propagation domain through would need the solver's live state and is
/// a real remaining increment.
fn slot_entropy(snapshot: &Wfc2dSnapshot, slot: &crate::schema::snapshot::Wfc2dSlot) -> f64 {
    if slot.pinned_tile_id.is_some() {
        return 0.0;
    }
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}

impl semio_framework_job::InteractiveJob for Wfc2dInferenceJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-wfc2d-inference-operation");
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        loop {
            context.set_stage(match self.stage {
                Wfc2dInferenceStage::Tiles => "wfc2d.infer.tiles",
                Wfc2dInferenceStage::Relations => "wfc2d.infer.relations",
                Wfc2dInferenceStage::Rules => "wfc2d.infer.rules",
                Wfc2dInferenceStage::Model => "wfc2d.infer.model",
                Wfc2dInferenceStage::Slots => "wfc2d.infer.slots",
                Wfc2dInferenceStage::Edges => "wfc2d.infer.edges",
                Wfc2dInferenceStage::Topology => "wfc2d.infer.topology",
                Wfc2dInferenceStage::Fixed => "wfc2d.infer.fixed",
                Wfc2dInferenceStage::Restore => "wfc2d.infer.restore",
                Wfc2dInferenceStage::Solve => "wfc2d.infer.solve",
                Wfc2dInferenceStage::MapCommit => "wfc2d.infer.map-commit",
                Wfc2dInferenceStage::EncodeCommit => "wfc2d.infer.encode-commit",
                Wfc2dInferenceStage::Complete => "wfc2d.infer.complete",
            });
            match self.stage {
                Wfc2dInferenceStage::Tiles
                | Wfc2dInferenceStage::Relations
                | Wfc2dInferenceStage::Rules
                | Wfc2dInferenceStage::Model
                | Wfc2dInferenceStage::Slots
                | Wfc2dInferenceStage::Edges
                | Wfc2dInferenceStage::Topology
                | Wfc2dInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Wfc2dInferenceStage::Restore => {
                    let outcome = self.restore.as_mut().expect("restore job").step(context);
                    return match outcome {
                        StepOutcome::Complete(candidate) => {
                            retire_payload(candidate.state);
                            retire_payload(candidate.output);
                            self.child = self.restore.as_mut().expect("restore job").take_job();
                            if let Some(mut restore) = self.restore.take() {
                                semio_s_plugin_wfc_engine::job::close_job(&mut restore);
                            }
                            self.stage = Wfc2dInferenceStage::Solve;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                Wfc2dInferenceStage::Solve => {
                    let outcome = self.child.as_mut().expect("WFC child").step(context);
                    return match outcome {
                        StepOutcome::Complete(candidate) => {
                            retire_payload(candidate.output);
                            self.final_checkpoint = Some(candidate.state);
                            self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                            // 🚪️ Every `WfcJob` runs its close ladder before it is dropped (engine §7):
                            // dropping one that still owns retained pages trips the job module's own
                            // lifecycle assertion and aborts the process from a second panic in `drop`.
                            if let Some(mut child) = self.child.take() {
                                semio_s_plugin_wfc_engine::job::close_job(&mut child);
                            }
                            self.cursor = 0;
                            if self.child_commit.is_none() {
                                self.enter_contradiction();
                            } else {
                                self.stage = Wfc2dInferenceStage::MapCommit;
                            }
                            self.emit_preview(context)
                        }
                        // 🩺 `wfc-unsatisfiable` is the engine's way of saying the spec admits no total
                        // assignment. That is an ANSWER, not a failure: it becomes the contradiction
                        // verdict plus the prior entropy map, and the fault's own payload is retired
                        // here rather than propagated. Any other fault is a real fault and passes through.
                        StepOutcome::Fault(fault) if semio_s_plugin_wfc_engine::job::payload_bytes(&fault.detail) == b"wfc-unsatisfiable" => {
                            retire_payload(fault.detail);
                            if let Some(mut child) = self.child.take() {
                                semio_s_plugin_wfc_engine::job::close_job(&mut child);
                            }
                            self.cursor = 0;
                            self.enter_contradiction();
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                Wfc2dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Wfc2dInferenceStage::EncodeCommit => {
                    let bytes = std::mem::take(&mut self.commit_bytes);
                    let mut cursor = self.commit_cursor;
                    let written = self.output.as_mut().map(|writer| writer.write_slice_page(context, &bytes, &mut cursor));
                    self.commit_cursor = cursor;
                    self.commit_bytes = bytes;
                    match written {
                        Some(Ok(true)) => {
                            let output = match self.output.take().expect("wfc2d output writer").finish() {
                                Ok(output) => output,
                                Err(mut writer) => {
                                    writer.begin_close();
                                    self.output = Some(writer);
                                    return StepOutcome::Yield;
                                }
                            };
                            self.stage = Wfc2dInferenceStage::Complete;
                            return StepOutcome::Complete(semio_framework_job::CommitCandidate {
                                state: self.final_checkpoint.take().unwrap_or_else(|| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)),
                                output,
                            });
                        }
                        Some(Ok(false)) => return StepOutcome::Yield,
                        Some(Err(_)) | None => {
                            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"wfc2d-inference-output-admission-exceeded");
                            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                        }
                    }
                }
                Wfc2dInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.emit_preview(context);
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(restore) = self.restore.as_mut() {
            restore.begin_close();
        }
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
        if let Some(output) = self.output.as_mut() {
            output.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if let Some(output) = self.output.as_mut() {
            if !output.terminal_is_empty() {
                return match output.close_step(maximum_items, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.output = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(checkpoint) = self.final_checkpoint.as_mut() {
            if !checkpoint.terminal_is_empty() {
                return match checkpoint.close_step(maximum_items, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.final_checkpoint = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        // 🚪️ The engine's own bounded ladder closes a child job: a child that answers `Complete`
        // while still owning pages leaves the framework's stage-1 check returning `Blocked` forever,
        // which a `while !terminal_is_empty()` driver spins on rather than failing. `close_job`
        // panics loudly on a genuinely stuck child instead.
        if let Some(mut restore) = self.restore.take() {
            semio_s_plugin_wfc_engine::job::close_job(&mut restore);
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(mut child) = self.child.take() {
            semio_s_plugin_wfc_engine::job::close_job(&mut child);
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    /// 🕳️ Ownership only, deliberately NOT gated on `closing`: the framework's close ladder calls
    /// `begin_close` itself at stage 0 and then refuses to advance past stage 1 while this answers
    /// false, so gating on a flag the driver may set later is a hang, not a safety net.
    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none()
    }
}

pub struct Wfc2dInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for Wfc2dInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(WFC_2D_INFERENCE_JOB_KIND, WFC_2D_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for Wfc2dInferenceJobFactory {
    type Payload = Wfc2dInferenceRequest;
    type Job = Wfc2dInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC_2D_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_WFC_2D_TILES + MAX_WFC_2D_SLOTS + MAX_WFC_2D_RULES + MAX_WFC_2D_EDGES, 4_096, MAX_WFC_2D_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Wfc2dInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc2d-inference-wire-decode:{error}")))?;
        let mut request: Wfc2dInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc2d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Wfc2dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.wfc2d.solve` inference — five handcrafted facet leaves.
pub fn wfc2d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: WFC_2D_INFERENCE_TOOL_ID,
        inference: ::semio_framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}

pub fn register_wfc2d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Wfc2dInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve_with_job(snapshot: &Wfc2dSnapshot) -> Result<Wfc2dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Wfc2dInferenceJob::new(operation, Wfc2dInferenceRequest { snapshot: snapshot.clone(), checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "wfc2d.wfc.inference.headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("wfc2d-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("wfc2d-inference-headless-contention:{error:?}"))?;
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        let payload_bytes = |payload: &semio_framework_job::RetainedJobPayload| {
            let mut bytes = Vec::with_capacity(payload.len());
            for index in 0..payload.page_count() {
                if let Some(page) = payload.page(index) {
                    bytes.extend_from_slice(page);
                }
            }
            bytes
        };
        let result = match &outcome {
            semio_framework_job::StepOutcome::Complete(candidate) => Some(
                std::str::from_utf8(&payload_bytes(&candidate.output))
                    .map_err(|error| format!("wfc2d-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<Wfc2dInferenceCommit>(text).map_err(|error| format!("wfc2d-invalid-commit:{error}"))),
            ),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("wfc2d-inference-cancelled".into())),
            semio_framework_job::StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => None,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            let mut guard = 0_u32;
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                guard += 1;
                assert!(guard < 1_000_000, "the wfc2d headless session refused to close");
            }
            return result.expect("terminal wfc2d inference outcome has result");
        }
        session.resume().map_err(|error| format!("wfc2d-inference-headless-resume:{error:?}"))?;
    }
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → tile id), or `Unsolved` for every non-solved outcome.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Wfc2dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct Wfc2dSolve;

impl store::InferredField<Wfc2dSnapshot> for Wfc2dSolve {
    type Key = String;
    type Value = Wfc2dSolveResult;

    const FIELD_ID: &'static str = "s.wfc.wfc2d.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc2d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) if !solution.contradiction => Wfc2dSolveResult::Solved { assignments: solution.assignments },
            _ => Wfc2dSolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "is this spec solvable at
/// all" never has to decode a whole assignment map to find out.
pub struct Wfc2dContradiction;

impl store::InferredField<Wfc2dSnapshot> for Wfc2dContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.wfc2d.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc2d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).map_or(true, |commit| commit.contradiction)
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned slot.
pub struct Wfc2dEntropy;

impl store::InferredField<Wfc2dSnapshot> for Wfc2dEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.wfc2d.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["slots", "tiles"]
    }
    fn plan(snapshot: &Wfc2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.slots.iter().map(|slot| store::InferenceStep { key: slot.id.clone(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &Wfc2dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_tile_id.clone());
        bytes.extend_from_slice(pinned.unwrap_or_default().as_bytes());
        bytes.push(0);
        for tile in &snapshot.tiles {
            bytes.extend_from_slice(tile.id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&tile.weight.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &Wfc2dSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        snapshot.slots.iter().find(|slot| &slot.id == key).map_or(0.0, |slot| slot_entropy(snapshot, slot))
    }
}
//#endregion 🔖️Entropy

//#region 🔖️Relations
/// 🔗 The relation universe a solve would compile, ascending — the same list `Relations` walks, made
/// public so the editor's graph window can label an edge's class without rebuilding the model.
pub fn solve_relations(snapshot: &Wfc2dSnapshot) -> Vec<String> {
    let mut names: BTreeSet<String> = snapshot.edges.iter().map(|edge| edge.relation.clone()).collect();
    if names.is_empty() {
        names.insert(crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.to_string());
    }
    names.into_iter().collect()
}
//#endregion 🔖️Relations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
