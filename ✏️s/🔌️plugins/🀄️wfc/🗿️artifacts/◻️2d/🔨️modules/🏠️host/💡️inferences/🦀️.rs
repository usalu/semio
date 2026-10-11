//! 🏠️ Inference admission, publication and retained job lifecycle.
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::Wfc2dSnapshot;
use std::collections::{BTreeMap, BTreeSet};
use crate::standards::v1::subsets::any::schema::inferences::*;

pub const WFC_2D_INFERENCE_JOB_KIND: &str = "semio.infer";

pub const WFC_2D_INFERENCE_TOOL_ID: &str = "s.wfc.wfc2d.solve";

pub const WFC_2D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.wfc2d.inference.request.v1";

/// 📜️ The PUBLISHED request schema of `s.wfc.wfc2d.solve` — what a client has to send, readable
/// from `inference_list`/`capabilities_describe` without reading a line of this crate. Authored
/// here rather than as a facet leaf because the facet leaf beside it (`🔣️.json`) is the RESULT
/// schema; a request and its result are two schemas, and publishing only one was the gap
/// (`📓️ce3-four-mcp-gates-green.md` §3.3).
pub const WFC_2D_INFERENCE_REQUEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any/inference.request.json",
  "title": "Wfc2dInferenceRequest",
  "type": "object",
  "additionalProperties": false,
  "oneOf": [{ "required": ["document"] }, { "required": ["snapshot"] }],
  "properties": {
    "document": {
      "type": "object",
      "description": "The artifact this solve runs on, bound by the gateway from `inference_run`'s `artifactId` — a caller names the artifact, never these bytes.",
      "additionalProperties": false,
      "required": ["pack", "spr"],
      "properties": { "pack": { "type": "string", "contentEncoding": "base64" }, "spr": { "type": "string", "contentEncoding": "base64" } }
    },
    "snapshot": { "type": "object", "description": "The problem stated in full instead of read from an artifact: the wfc2d solve's own document shape." },
    "checkpoint": { "type": "array", "description": "A previous run's checkpoint bytes, to resume instead of restart.", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }
  }
}"#;

/// 📜️ The whole published contract for `s.wfc.wfc2d.solve`: request schema, result schema, the
/// unit its bounded job counts, and the artifact binding that makes it callable at all.
pub const WFC_2D_INFERENCE_CONTRACT: semio_framework_plugin::ArtifactInferencePayloadContract = semio_framework_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: WFC_2D_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: WFC_2D_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
    progress_unit: "slots",
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: None,
};

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
        payload: Some(WFC_2D_INFERENCE_CONTRACT),
    }
}

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dInferenceRequest {
    /// 📸️ The problem, stated in full by the caller. Mutually exclusive with `document`: exactly one
    /// of the two says which snapshot this solve runs over.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Wfc2dSnapshot>,
    /// 🔗️ The ARTIFACT this solve runs on, as its own canonical `pack`/`spr` pair. This is the
    /// field `WFC_2D_INFERENCE_CONTRACT`'s artifact binding names, so an agent that calls
    /// `inference_run` with `artifactId` never has to state a snapshot it could not type: the
    /// gateway binds the document here and the guest decodes it into its own snapshot below.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<semio_framework_plugin::ArtifactDocumentPayload>,
    pub checkpoint: Option<Vec<u8>>,
}

impl Wfc2dInferenceRequest {
    /// 📸️ The snapshot this request states, from whichever of its two carriers is present — the
    /// ONE resolution path every caller of this inference takes.
    pub fn resolve_snapshot(&self) -> Result<Wfc2dSnapshot, String> {
        match (&self.snapshot, &self.document) {
            (Some(snapshot), _) => Ok(snapshot.clone()),
            (None, Some(document)) => document.settled_snapshot::<Wfc2dSnapshot, crate::Wfc2dMutation>(),
            (None, None) => Err("s.wfc.wfc2d-inference-no-snapshot:state `snapshot`, or name the artifact with `artifactId` so `document` is bound".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) snapshot: Wfc2dSnapshot,
    pub(crate) stage: Wfc2dInferenceStage,
    pub(crate) cursor: usize,
    pub(crate) pattern_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::PatternId>,
    pub(crate) node_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::NodeId>,
    pub(crate) tile_ids: Vec<String>,
    pub(crate) raw_weights: Vec<f64>,
    pub(crate) relation_names: Vec<String>,
    pub(crate) relation_of: BTreeMap<String, semio_s_plugin_wfc_engine::ids::RelationId>,
    pub(crate) builder: Option<semio_s_plugin_wfc_engine::model::ModelBuilder>,
    pub(crate) model: Option<semio_s_plugin_wfc_engine::model::CompiledModel>,
    pub(crate) topology_build: Option<semio_s_plugin_wfc_engine::topology::GraphTopologyBuild>,
    pub(crate) topology: Option<semio_s_plugin_wfc_engine::topology::GraphTopology>,
    pub(crate) fixed: Vec<(semio_s_plugin_wfc_engine::ids::NodeId, semio_s_plugin_wfc_engine::ids::PatternId)>,
    pub(crate) checkpoint: Option<Vec<u8>>,
    pub(crate) restore: Option<semio_s_plugin_wfc_engine::job::WfcRestore<semio_s_plugin_wfc_engine::topology::GraphTopology>>,
    pub(crate) child: Option<semio_s_plugin_wfc_engine::job::WfcJob<semio_s_plugin_wfc_engine::topology::GraphTopology>>,
    pub(crate) child_commit: Option<semio_s_plugin_wfc_engine::job::WfcCommit>,
    pub(crate) commit: Wfc2dInferenceCommit,
    pub(crate) commit_bytes: Vec<u8>,
    pub(crate) commit_cursor: usize,
    pub(crate) output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    pub(crate) preview_units: u64,
    pub(crate) last_preview_ms: Option<u64>,
    pub(crate) publication: Option<Box<semio_s_plugin_wfc_engine::job::Publication>>,
    pub(crate) lent: Wfc2dInferenceJobLent,
    pub(crate) restore_done: bool,
    pub(crate) child_end: Option<InferenceChildEnd>,
    pub(crate) output_payload: Option<semio_framework_job::RetainedJobPayload>,
    pub(crate) closing: bool,
}

impl Wfc2dInferenceJob {
    pub(crate) fn new(mut operation: semio_framework_job::Operation, request: Wfc2dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.resolve_snapshot()?;
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
            commit: Wfc2dInferenceCommit::default(),
            commit_bytes: Vec::new(),
            commit_cursor: 0,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            lent: Wfc2dInferenceJobLent::Own,
            restore_done: false,
            child_end: None,
            output_payload: None,
            closing: false,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub(crate) fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_WFC_2D_ID_BYTES {
            Err("wfc2d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
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

    pub(crate) fn stage_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Wfc2dInferenceJobRun {
        let (completed, total) = self.progress();
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return self.stage_fault(Vec::new()),
        };
        let mut preview = [0; 25];
        preview[..8].copy_from_slice(&sequence.to_le_bytes());
        preview[8..16].copy_from_slice(&(completed as u64).to_le_bytes());
        preview[16..24].copy_from_slice(&(total as u64).to_le_bytes());
        preview[24] = self.stage as u8;
        self.preview_units = 0;
        self.last_preview_ms = context.now_us().map(|now_us| now_us / 1_000);
        self.publication = Some(semio_s_plugin_wfc_engine::job::Publication::new(semio_s_plugin_wfc_engine::job::PublicationKind::Preview, preview.to_vec(), Vec::new()));
        Wfc2dInferenceJobRun::Staged
    }

    /// 🧯️ Stages the fault detail as the next lent outcome.
    pub(crate) fn stage_fault(&mut self, detail: Vec<u8>) -> Wfc2dInferenceJobRun {
        self.publication = Some(semio_s_plugin_wfc_engine::job::Publication::new(semio_s_plugin_wfc_engine::job::PublicationKind::Fault, detail, Vec::new()));
        Wfc2dInferenceJobRun::Staged
    }

    pub(crate) fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    /// 🏗️ One relation per distinct edge `relation` string. Each is SELF-INVERSE and every arc is
    /// added in both directions, so adjacency stays symmetric exactly as assembly's single-relation
    /// route was — the relation only scopes WHICH rules apply, never a direction.
    pub(crate) fn compile_rules(&mut self) -> Result<(), String> {
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

    pub(crate) fn advance_compile(&mut self) -> Result<(), String> {
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
                    self.commit_bytes = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&self.commit).into_bytes();
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
    pub(crate) fn enter_contradiction(&mut self) {
        self.child_commit = None;
        self.commit.assignments.clear();
        self.commit.contradiction = true;
        for slot in &self.snapshot.slots {
            self.commit.entropy.insert(slot.id.clone(), slot_entropy(&self.snapshot, slot));
        }
        self.commit_bytes = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&self.commit).into_bytes();
        self.commit_cursor = 0;
        self.stage = Wfc2dInferenceStage::EncodeCommit;
    }

    pub(crate) fn map_one(&mut self) -> Result<(), String> {
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
            self.commit_bytes = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&self.commit).into_bytes();
            if self.commit_bytes.len() > MAX_WFC_2D_OUTPUT_BYTES {
                return Err("wfc2d-inference-output-admission-exceeded".into());
            }
            self.stage = Wfc2dInferenceStage::EncodeCommit;
        }
        Ok(())
    }
}

/// 🚦️ What one bounded run decided before any outcome is lent.
pub(crate) enum Wfc2dInferenceJobRun {
    Yield,
    Cancelled,
    Staged,
    StepRestore,
    StepChild,
}

/// 🤝️ Which retained owner produced the outcome currently on loan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Wfc2dInferenceJobLent {
    Own,
    Restore,
    Child,
}

/// 🏁️ How the child solve ended; both ends are consumed here and never lent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InferenceChildEnd {
    Complete,
    Unsatisfiable,
}

impl Wfc2dInferenceJob {
    /// ⏭️ Consumes the child's end: its commit is taken, its sealed state payload stays lent from the child until close.
    fn finish_child(&mut self, context: &mut semio_framework_job::StepContext<'_>, end: InferenceChildEnd) -> Wfc2dInferenceJobRun {
        if matches!(end, InferenceChildEnd::Complete) {
            self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
        } else {
            self.child_commit = None;
        }
        self.cursor = 0;
        if self.child_commit.is_none() {
            self.enter_contradiction();
        } else {
            self.stage = Wfc2dInferenceStage::MapCommit;
        }
        self.stage_preview(context)
    }

    /// ⏭️ Runs the bounded compile, map and encode spans; a child or restore turn is requested, never taken here.
    fn run(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Wfc2dInferenceJobRun {
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
                        return self.stage_fault(error.into_bytes());
                    }
                }
                Wfc2dInferenceStage::Restore => {
                    if self.restore_done {
                        self.child = self.restore.as_mut().expect("restore job").take_job();
                        if let Some(mut restore) = self.restore.take() {
                            semio_s_plugin_wfc_engine::job::close_job(&mut restore);
                        }
                        self.restore_done = false;
                        self.stage = Wfc2dInferenceStage::Solve;
                        return self.stage_preview(context);
                    }
                    return Wfc2dInferenceJobRun::StepRestore;
                }
                Wfc2dInferenceStage::Solve => {
                    let Some(end) = self.child_end.take() else {
                        return Wfc2dInferenceJobRun::StepChild;
                    };
                    return self.finish_child(context, end);
                }
                Wfc2dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        return self.stage_fault(error.into_bytes());
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
                                    return Wfc2dInferenceJobRun::Yield;
                                }
                            };
                            self.stage = Wfc2dInferenceStage::Complete;
                            self.output_payload = Some(output);
                            return Wfc2dInferenceJobRun::Yield;
                        }
                        Some(Ok(false)) => return Wfc2dInferenceJobRun::Yield,
                        Some(Err(_)) | None => return self.stage_fault(b"wfc2d-inference-output-admission-exceeded".to_vec()),
                    }
                }
                Wfc2dInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return Wfc2dInferenceJobRun::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.stage_preview(context);
            }
            if context.should_yield() {
                return Wfc2dInferenceJobRun::Yield;
            }
        }
    }

    /// 🤝️ Closes the lent preview or fault turn by turn from the next step's own wallet.
    fn retire_delivered<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let publication = self.publication.as_mut().expect("a delivered inference publication is staged");
        let step = publication.close_step(context.retained_grant());
        context.consume_retained(step.progress())?;
        if let semio_framework_job::InteractiveJobCloseStep::Refused { kind, progress } = step {
            return Err(semio_framework_value::ValueError::literal(kind, "inference publication close was refused").with_retained_progress(progress));
        }
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    /// 📏️ Quotes the next close turn in the order the ladder spends them.
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inference close depth overflow"))?;
            Ok(demand)
        };
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(output) = self.output.as_ref() {
            return if output.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(output.retirement_demands()?) };
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return if payload.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(payload.retirement_demands()?) };
        }
        if let Some(restore) = self.restore.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: restore.next_close_copy_byte_demand()?, capacity_bytes: restore.next_close_capacity_byte_demand(0)?, release_bytes: restore.next_close_release_byte_demand()?, depth: restore.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(child) = self.child.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: child.next_close_copy_byte_demand()?, capacity_bytes: child.next_close_capacity_byte_demand(0)?, release_bytes: child.next_close_release_byte_demand()?, depth: child.next_close_depth_demand()?.saturating_add(1) });
        }
        Ok(semio_framework_value::RetirementDemand::default())
    }
}

impl semio_framework_job::InteractiveJob for Wfc2dInferenceJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return self.retire_delivered(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.stage_fault(b"stale-wfc2d-inference-operation".to_vec());
        }
        if self.publication.is_some() {
            self.lent = Wfc2dInferenceJobLent::Own;
            return self.publication.as_mut().expect("a staged inference publication").poll(context);
        }
        if self.output_payload.is_some() {
            return JobOutcomeBorrow::admit_complete(context, self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref());
        }
        match self.run(context) {
            Wfc2dInferenceJobRun::Yield | Wfc2dInferenceJobRun::Staged => Ok(None),
            Wfc2dInferenceJobRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
            Wfc2dInferenceJobRun::StepRestore => {
                let result = self.restore.as_mut().expect("restore job").step(context)?;
                if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    self.restore_done = true;
                    return Ok(None);
                }
                self.lent = Wfc2dInferenceJobLent::Restore;
                Ok(result)
            }
            Wfc2dInferenceJobRun::StepChild => {
                let result = self.child.as_mut().expect("WFC child").step(context)?;
                let end = if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    Some(InferenceChildEnd::Complete)
                } else {
                    if let Some(JobOutcomeBorrow::Fault { detail, .. }) = &result {
            if semio_s_plugin_wfc_engine::job::payload_bytes(detail) == b"wfc-unsatisfiable" {
                Some(InferenceChildEnd::Unsatisfiable)
            } else {
                None
            }
        } else {
            None
        }
                };
                if let Some(end) = end {
                    self.child_end = Some(end);
                    return Ok(None);
                }
                self.lent = Wfc2dInferenceJobLent::Child;
                Ok(result)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind;
        let absent = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "inference outcome has no retained owner");
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref()),
            _ => match self.lent {
                Wfc2dInferenceJobLent::Own => self.publication.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Wfc2dInferenceJobLent::Restore => self.restore.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Wfc2dInferenceJobLent::Child => self.child.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
            },
        }
    }

    fn begin_close(&mut self) {
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

    fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        let refused = |error: semio_framework_value::ValueError| InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() };
        let demand = match self.close_demands() {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let child_grant = semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
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
        if let Some(output) = self.output.as_mut() {
            if output.terminal_is_empty() {
                self.output = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match output.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(payload) = self.output_payload.as_mut() {
            if payload.terminal_is_empty() {
                self.output_payload = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match payload.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(restore) = self.restore.as_mut() {
            let step = restore.close_step(child_grant);
            if restore.terminal_is_empty() {
                self.restore = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(child) = self.child.as_mut() {
            let step = child.close_step(child_grant);
            if child.terminal_is_empty() {
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

    /// 🚪️ Ownership ONLY — deliberately NOT gated on a closing flag: the retained session calls `begin_close`
    /// itself, and a flag-gated predicate would spin its own close ladder forever.
    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.output.is_none() && self.output_payload.is_none() && self.restore.is_none() && self.child.is_none()
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
        let mut request: Wfc2dInferenceRequest = crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc2d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Wfc2dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.wfc2d.solve` inference — five handcrafted facet leaves.
pub fn wfc2d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: WFC_2D_INFERENCE_TOOL_ID,
        inference: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"), typescript: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts"), graphql: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql"), json_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"), proto: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto") },
    }
}

pub fn register_wfc2d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Wfc2dInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve_with_job(snapshot: &Wfc2dSnapshot) -> Result<Wfc2dInferenceCommit, String> {
    solve_with_clock(snapshot, semio_framework_job::default_now_us)
}

/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on
/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.
pub fn solve_with_clock(snapshot: &Wfc2dSnapshot, now_us: fn() -> Option<u64>) -> Result<Wfc2dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Wfc2dInferenceJob::new(operation, Wfc2dInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    semio_s_plugin_wfc_engine::job::run_headless(
        job,
        operation,
        now_us,
        semio_s_plugin_wfc_engine::job::HeadlessSite { site: "wfc2d.wfc.inference.headless", label: "wfc2d-inference-headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
        |_job, output| {
            std::str::from_utf8(output)
                .map_err(|error| format!("wfc2d-invalid-commit:{error}"))
                .and_then(|text| crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<Wfc2dInferenceCommit>(text).map_err(|error| format!("wfc2d-invalid-commit:{error}")))
        },
    )
}

impl store::InferredField<Wfc2dSnapshot> for Wfc2dSolve {
    type Dependency = Vec<u8>;
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
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) if !solution.contradiction => Wfc2dSolveResult::Solved { assignments: solution.assignments },
            _ => Wfc2dSolveResult::Unsolved,
        }
    }
}

impl store::InferredField<Wfc2dSnapshot> for Wfc2dContradiction {
    type Dependency = Vec<u8>;
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
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).map_or(true, |commit| commit.contradiction)
    }
}

impl store::InferredField<Wfc2dSnapshot> for Wfc2dEntropy {
    type Dependency = Vec<u8>;
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
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
