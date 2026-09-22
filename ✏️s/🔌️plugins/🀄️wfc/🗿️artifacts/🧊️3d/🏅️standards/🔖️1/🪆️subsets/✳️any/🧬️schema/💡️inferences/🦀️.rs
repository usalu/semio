//! 💡️ `wfc3d` inferences — THE SOLVE ITSELF IS AN INFERENCE. `Wfc3dSnapshot` only ever persists the
//! PROBLEM (slots/edges/tiles/rules/seed); the SOLUTION, the contradiction verdict and the
//! pre-propagation entropy map are all derived here via `store::InferredField`, never
//! mutation-authored state. The engine crate `semio_s_plugin_wfc_engine` supplies the internals of
//! these `compute()` bodies.
//!
//! Determinism: `solve_with_job` reads only `snapshot` fields (`seed` included) and drives the same
//! resumable `WfcJob<GraphTopology>` interactive callers use; every step is watchdog-wrapped and
//! explicitly bounded. No ambient randomness enters the inference, so `DepHash` caching over
//! `Wfc3dSolve`/`Wfc3dContradiction`/`Wfc3dEntropy` is sound.
//!
//! Rule semantics: the rules ARE the compatibility table. They compile onto an EMPTY `ModelBuilder`,
//! so a tile pair no rule mentions is FORBIDDEN — an allow-list, the same law every other `wfc`
//! artifact states. `allowed: true` pushes `allow`, `allowed: false` pushes `deny`, and deny always
//! wins over allow at compile time regardless of call order. `relation: None` states the rule for
//! every relation at once. A rule is written for an UNORDERED pair, so both directed orders are
//! pushed. An empty rule set is therefore unsatisfiable the moment two slots are adjacent.

use crate::schema::snapshot::Wfc3dSnapshot;
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;

//#region 📦️RetainedPayload
/// 📦️ One single-page payload, with the job module's EXACT source handback on refusal. Dropping a
/// `JobPayloadRejectedPage` without taking its source back trips that module's own lifecycle
/// assertion and then aborts the process from a second panic inside `RetainedJobPayload::drop`. An
/// empty payload is the honest answer to a refused admission; leaking the page never was.
fn retained_payload(context: &mut semio_framework_job::StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
    match context.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            semio_framework_job::RetainedJobPayload::empty(stream)
        }
    }
}

/// ♻️ Releases every page one retained payload still owns. A payload that reaches `Drop` with pages
/// outstanding trips the job module's own lifecycle assertion and then aborts the process from a
/// second panic inside `RetainedJobPayload::drop`.
fn retire_payload(mut payload: semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}
//#endregion 📦️RetainedPayload

//#region 🔖️Compile
pub const WFC3D_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const WFC3D_INFERENCE_TOOL_ID: &str = "s.wfc.wfc3d.solve";
pub const WFC3D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.wfc3d.inference.request.v1";


/// 📜️ The PUBLISHED request schema of `s.wfc.wfc3d.solve` — what a client has to send, readable
/// from `inference_list`/`capabilities_describe` without reading a line of this crate. Authored
/// here rather than as a facet leaf because the facet leaf beside it (`🔣️.json`) is the RESULT
/// schema; a request and its result are two schemas, and publishing only one was the gap
/// (`📓️ce3-four-mcp-gates-green.md` §3.3).
pub const WFC3D_INFERENCE_REQUEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://json.schemas.assets.semio-tech.com/s/wfc/wfc3d/1/any/inference.request.json",
  "title": "Wfc3dInferenceRequest",
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
    "snapshot": { "type": "object", "description": "The problem stated in full instead of read from an artifact: the wfc3d solve's own document shape." },
    "checkpoint": { "type": "array", "description": "A previous run's checkpoint bytes, to resume instead of restart.", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }
  }
}"#;

/// 📜️ The whole published contract for `s.wfc.wfc3d.solve`: request schema, result schema, the
/// unit its bounded job counts, and the artifact binding that makes it callable at all.
pub const WFC3D_INFERENCE_CONTRACT: semio_framework_plugin::ArtifactInferencePayloadContract = semio_framework_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: WFC3D_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: WFC3D_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("🔣️.json"),
    progress_unit: "slots",
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
};

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn wfc3d_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.wfc3d",
        artifact_schema: "s.wfc.wfc3d",
        artifact_schema_version: 1,
        inference_schema: WFC3D_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: Some(WFC3D_INFERENCE_CONTRACT),
    }
}

/// 🧮️ The compatibility table is `relations × tiles²` bits, so the tile and relation universes are
/// the two admissions that actually bound compile work; slots/edges/rules only bound linear passes.
const MAX_WFC3D_TILES: usize = 256;
const MAX_WFC3D_RELATIONS: usize = 32;
const MAX_WFC3D_SLOTS: usize = 65_536;
const MAX_WFC3D_EDGES: usize = 262_144;
const MAX_WFC3D_RULES: usize = 262_144;
const MAX_WFC3D_ID_BYTES: usize = 1_024;
const MAX_WFC3D_OUTPUT_BYTES: usize = 1 << 20;
/// ⛽️ The headless drive's budget, shared with the sibling `grid3d`. Assembly's inherited
/// `fuel_per_step: 1` buys ONE work unit per session round trip, so a solve spends its wall time in
/// session bookkeeping rather than in the solver; the fuel is raised so one step does as much work as
/// its wall budget admits.
///
/// 📄️ Raising the fuel is only sound because every stage that admits a payload page ENDS its step —
/// see `encode_one`. A `StepContext` grants ONE page per step, and a stage that loops over pages
/// inside a multi-unit step is refused, then cannot even admit its own fault detail, so the job dies
/// as a `StepOutcome::Fault` carrying an empty page.
///
/// ⏱️ The wall budget is deliberately far above `semio_framework_trace`'s shared
/// `INTERACTIVE_STEP_CEILING_US` (8 ms), against which every step is measured whatever budget its
/// config asked for; four consecutive over-ceiling steps quarantine the session. That is safe here
/// because a step is bounded by the preview cadence and by the one-page-per-step encode, not by this
/// deadline — the deadline only stops a solve that would otherwise spin.
const HEADLESS_FUEL_PER_STEP: u64 = 1 << 16;
const HEADLESS_STEP_BUDGET_US: u64 = 250_000;
const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Wfc3dInferenceRequest {
    /// 📸️ The problem, stated in full by the caller. Mutually exclusive with `document`: exactly one
    /// of the two says which snapshot this solve runs over.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Wfc3dSnapshot>,
    /// 🔗️ The ARTIFACT this solve runs on, as its own canonical `pack`/`spr` pair. This is the
    /// field `WFC3D_INFERENCE_CONTRACT`'s artifact binding names, so an agent that calls
    /// `inference_run` with `artifactId` never has to state a snapshot it could not type: the
    /// gateway binds the document here and the guest decodes it into its own snapshot below.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<semio_framework_plugin::ArtifactDocumentPayload>,
    pub checkpoint: Option<Vec<u8>>,
}

impl Wfc3dInferenceRequest {
    /// 📸️ The snapshot this request states, from whichever of its two carriers is present — the
    /// ONE resolution path every caller of this inference takes.
    pub fn resolve_snapshot(&self) -> Result<Wfc3dSnapshot, String> {
        match (&self.snapshot, &self.document) {
            (Some(snapshot), _) => Ok(snapshot.clone()),
            (None, Some(document)) => document.settled_snapshot::<Wfc3dSnapshot, crate::Wfc3dMutation>(),
            (None, None) => Err("s.wfc.wfc3d-inference-no-snapshot:state `snapshot`, or name the artifact with `artifactId` so `document` is bound".into()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc3dInferenceCommit {
    pub assignments: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Wfc3dInferenceStage {
    Tiles,
    Relations,
    Rules,
    Model,
    Compile,
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

/// 🧵 Worker-owned parent transaction for wfc3d compile, solve, restore and authoritative mapping.
///
/// 🚪️ Two lifecycle laws the `Solve`/`Restore` arms of [`Wfc3dInferenceJob::step`] keep, both measured
/// rather than assumed. A `semio_framework_job::CommitCandidate` carries TWO retained payloads
/// (`state` and `output`): the child's checkpoint becomes THIS job's commit state, its commit output
/// is the raw assignment vector `map_one` reads from `take_completed_commit()` instead, and whichever
/// is not kept is retired through [`retire_payload`] — a dropped payload with pages outstanding trips
/// `RetainedJobPayload`'s own Drop assertion and aborts the process. And a child `WfcJob`/`WfcRestore`
/// DROPPED without `engine::job::close_job` leaves its admitted pages registered in the session's
/// payload ledger, after which the session's own close reports `Blocked` forever: that is why the
/// ancestor `assembly` artifact could never run its solve inference in a test at all.
///
/// ⛓️ The `Rules` stage lowers each authored rule to `(allowed, relation or None = all, a, b)`; a rule
/// naming a relation this document never authored binds to no arc, so it is kept in the document and
/// simply skipped. The `Model` stage then replays ONE lowered rule per fuel unit onto the EMPTY
/// builder — `allow` for an admitted pair, `deny` for a forbidden one (deny wins at compile), both
/// directed orders because a rule is written for an unordered pair, and every relation when the rule
/// names none.
pub struct Wfc3dInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: Wfc3dSnapshot,
    stage: Wfc3dInferenceStage,
    cursor: usize,
    pattern_of: BTreeMap<String, engine::ids::PatternId>,
    node_of: BTreeMap<String, engine::ids::NodeId>,
    tile_ids: Vec<String>,
    raw_weights: Vec<f64>,
    relation_ids: BTreeMap<String, engine::ids::RelationId>,
    relation_names: Vec<String>,
    /// ⛓️ Every authored rule, lowered to `(allowed, relation or None = all, tile a, tile b)` — the
    /// `Model` stage replays one of these per fuel unit onto the builder.
    compatibility: Vec<(bool, Option<usize>, u32, u32)>,
    builder: Option<engine::model::ModelBuilder>,
    model: Option<engine::model::CompiledModel>,
    topology_build: Option<engine::topology::GraphTopologyBuild>,
    topology: Option<engine::topology::GraphTopology>,
    fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    checkpoint: Option<Vec<u8>>,
    restore: Option<engine::job::WfcRestore<engine::topology::GraphTopology>>,
    child: Option<engine::job::WfcJob<engine::topology::GraphTopology>>,
    child_commit: Option<engine::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    assignments: BTreeMap<String, String>,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    output_started: bool,
    encoded_entries: usize,
    encode_total: usize,
    preview_units: u64,
    last_preview_ms: Option<u64>,
}

impl Wfc3dInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: Wfc3dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.resolve_snapshot()?;
        if snapshot.tiles.len() > MAX_WFC3D_TILES
            || snapshot.slots.len() > MAX_WFC3D_SLOTS
            || snapshot.rules.len() > MAX_WFC3D_RULES
            || snapshot.edges.len() > MAX_WFC3D_EDGES
            || snapshot.edges.len().saturating_mul(2) > u32::MAX as usize
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("wfc3d-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: Wfc3dInferenceStage::Tiles,
            cursor: 0,
            pattern_of: BTreeMap::new(),
            node_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            raw_weights: Vec::new(),
            relation_ids: BTreeMap::new(),
            relation_names: Vec::new(),
            compatibility: Vec::new(),
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
            assignments: BTreeMap::new(),
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            output_started: false,
            encoded_entries: 0,
            encode_total: 0,
            preview_units: 0,
            last_preview_ms: None,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_WFC3D_ID_BYTES {
            Err("wfc3d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    fn progress(&self) -> (usize, usize) {
        match self.stage {
            Wfc3dInferenceStage::Tiles => (self.cursor, self.snapshot.tiles.len()),
            Wfc3dInferenceStage::Relations => (self.cursor, self.snapshot.edges.len()),
            Wfc3dInferenceStage::Rules => (self.cursor, self.snapshot.rules.len()),
            Wfc3dInferenceStage::Model => (self.cursor, self.compatibility.len()),
            Wfc3dInferenceStage::Compile => (0, 1),
            Wfc3dInferenceStage::Slots | Wfc3dInferenceStage::Fixed | Wfc3dInferenceStage::MapCommit => (self.cursor, self.snapshot.slots.len()),
            Wfc3dInferenceStage::Edges => (self.cursor, self.snapshot.edges.len()),
            Wfc3dInferenceStage::Topology => self.topology_build.as_ref().map_or((0, self.snapshot.slots.len()), |build| build.progress()),
            Wfc3dInferenceStage::Restore | Wfc3dInferenceStage::Solve => (0, 1),
            Wfc3dInferenceStage::EncodeCommit => (self.encoded_entries, self.encode_total),
            Wfc3dInferenceStage::Complete => (1, 1),
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

    fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            Wfc3dInferenceStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    Self::validate_id(&tile.id)?;
                    let pattern = engine::ids::PatternId::from_index(self.cursor);
                    self.pattern_of.insert(tile.id.clone(), pattern);
                    self.tile_ids.push(tile.id.clone());
                    self.raw_weights.push(if tile.weight > 0.0 { tile.weight } else { 1.0 });
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Relations;
                }
            }
            Wfc3dInferenceStage::Relations => {
                if let Some(edge) = self.snapshot.edges.get(self.cursor) {
                    Self::validate_id(&edge.relation)?;
                    if !self.relation_ids.contains_key(&edge.relation) {
                        if self.relation_names.len() >= MAX_WFC3D_RELATIONS {
                            return Err("wfc3d-inference-relation-admission-exceeded".into());
                        }
                        let id = engine::ids::RelationId::from_index(self.relation_names.len());
                        self.relation_ids.insert(edge.relation.clone(), id);
                        self.relation_names.push(edge.relation.clone());
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Rules;
                }
            }
            Wfc3dInferenceStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    Self::validate_id(&rule.tile_a_id)?;
                    Self::validate_id(&rule.tile_b_id)?;
                    if let (Some(a), Some(b)) = (self.pattern_of.get(&rule.tile_a_id), self.pattern_of.get(&rule.tile_b_id)) {
                        let scope = match &rule.relation {
                            None => Some(None),
                            Some(relation) => self.relation_ids.get(relation).map(|id| Some(id.index())),
                        };
                        if let Some(scope) = scope {
                            self.compatibility.push((rule.allowed, scope, a.get(), b.get()));
                        }
                    }
                    self.cursor += 1;
                } else if self.snapshot.slots.is_empty() || self.tile_ids.is_empty() {
                    self.stage = Wfc3dInferenceStage::EncodeCommit;
                } else {
                    let mut builder = engine::model::ModelBuilder::new();
                    for weight in &self.raw_weights {
                        builder.add_pattern(*weight);
                    }
                    for name in &self.relation_names {
                        builder.add_relation(name);
                    }
                    self.builder = Some(builder);
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Model;
                }
            }
            Wfc3dInferenceStage::Model => {
                if let Some(&(allowed, scope, a, b)) = self.compatibility.get(self.cursor) {
                    let builder = self.builder.as_mut().ok_or("wfc3d-model-builder-missing")?;
                    let source = engine::ids::PatternId::from_index(a as usize);
                    let target = engine::ids::PatternId::from_index(b as usize);
                    let relations: Vec<usize> = match scope {
                        Some(index) => vec![index],
                        None => (0..self.relation_names.len()).collect(),
                    };
                    for index in relations {
                        let relation = engine::ids::RelationId::from_index(index);
                        if allowed {
                            builder.allow(relation, source, target);
                            builder.allow(relation, target, source);
                        } else {
                            builder.deny(relation, source, target);
                            builder.deny(relation, target, source);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Compile;
                }
            }
            Wfc3dInferenceStage::Compile => {
                let builder = self.builder.take().ok_or("wfc3d-model-builder-missing")?;
                self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                self.cursor = 0;
                self.topology_build = Some(engine::topology::GraphTopologyBuild::new(self.snapshot.slots.len()));
                self.stage = Wfc3dInferenceStage::Slots;
            }
            Wfc3dInferenceStage::Slots => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    Self::validate_id(&slot.id)?;
                    if let Some(pinned) = &slot.pinned_tile_id {
                        Self::validate_id(pinned)?;
                    }
                    self.node_of.insert(slot.id.clone(), engine::ids::NodeId::from_index(self.cursor));
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Edges;
                }
            }
            Wfc3dInferenceStage::Edges => {
                if let Some(edge) = self.snapshot.edges.get(self.cursor) {
                    if let (Some(&from), Some(&to), Some(&relation)) = (self.node_of.get(&edge.from_slot_id), self.node_of.get(&edge.to_slot_id), self.relation_ids.get(&edge.relation)) {
                        let topology = self.topology_build.as_mut().ok_or("wfc3d-topology-build-missing")?;
                        topology.add_arc(from, to, relation).map_err(|error| format!("{error:?}"))?;
                        topology.add_arc(to, from, relation).map_err(|error| format!("{error:?}"))?;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Topology;
                }
            }
            Wfc3dInferenceStage::Topology => {
                if let Some(topology) = self.topology_build.as_mut().ok_or("wfc3d-topology-build-missing")?.step() {
                    self.topology = Some(topology);
                    self.topology_build = None;
                    self.cursor = 0;
                    self.stage = Wfc3dInferenceStage::Fixed;
                }
            }
            Wfc3dInferenceStage::Fixed => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    if let Some(pinned) = &slot.pinned_tile_id {
                        if let (Some(&node), Some(&pattern)) = (self.node_of.get(&slot.id), self.pattern_of.get(pinned)) {
                            self.fixed.push((node, pattern));
                        }
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().ok_or("wfc3d-compiled-model-missing")?;
                    let topology = self.topology.take().ok_or("wfc3d-compiled-topology-missing")?;
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(engine::job::WfcRestore::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = Wfc3dInferenceStage::Restore;
                    } else {
                        self.child = Some(engine::job::WfcJob::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed));
                        self.stage = Wfc3dInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile wfc3d inference stage"),
        }
        Ok(())
    }

    fn map_one(&mut self) -> Result<(), String> {
        let commit = self.child_commit.as_ref().ok_or("wfc3d-commit-missing")?;
        if self.cursor < self.snapshot.slots.len() {
            let slot = &self.snapshot.slots[self.cursor];
            let pattern = usize::try_from(*commit.assignment.get(self.cursor).ok_or("wfc3d-commit-missing-slot")?).map_err(|_| "wfc3d-commit-pattern-capacity")?;
            let tile = self.tile_ids.get(pattern).ok_or("wfc3d-commit-pattern-out-of-range")?;
            self.assignments.insert(slot.id.clone(), tile.clone());
            self.cursor += 1;
        } else {
            self.child_commit = None;
            self.encode_total = self.assignments.len();
            self.stage = Wfc3dInferenceStage::EncodeCommit;
        }
        Ok(())
    }

    /// 📄️ Admits exactly ONE payload page per call, so its caller MUST end the step on every
    /// non-final return: a `StepContext` grants a single page per step (`payload_page_granted`), and a
    /// second admission inside the same step is refused as `OpportunityExhausted` — whose own fault
    /// detail then cannot be admitted either, so the job dies as a `StepOutcome::Fault` carrying an
    /// EMPTY page. Measured here: with `fuel_per_step` above `1` and the encode stage looping, every
    /// solve in this artifact failed exactly that way.
    fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let source = self.rejected_output_page.take().unwrap_or_default();
        let writer = self.output.as_mut().ok_or("wfc3d-output-writer-missing")?;
        let mut page = match context.admit_payload_page(writer, source) {
            Ok(page) => page,
            Err(rejected) => {
                self.rejected_output_page = Some(rejected.into_source());
                return Err("wfc3d-inference-output-admission-exceeded".into());
            }
        };
        if !self.output_started {
            page.write(br#"{"assignments":{"#).map_err(|_| "wfc3d-inference-output-page")?;
            page.commit();
            self.output_started = true;
            return Ok(false);
        }
        if let Some((slot, tile)) = self.assignments.pop_first() {
            let slot = protocol::json::to_json_string(&slot);
            let tile = protocol::json::to_json_string(&tile);
            if self.encoded_entries != 0 {
                page.write(b",").map_err(|_| "wfc3d-inference-output-page")?;
            }
            page.write(slot.as_bytes()).and_then(|_| page.write(b":")).and_then(|_| page.write(tile.as_bytes())).map_err(|_| "wfc3d-inference-output-page")?;
            page.commit();
            self.encoded_entries += 1;
            return Ok(false);
        }
        page.write(b"}}").map_err(|_| "wfc3d-inference-output-page")?;
        page.commit();
        self.stage = Wfc3dInferenceStage::Complete;
        Ok(true)
    }
}


impl Wfc3dInferenceJob {
    /// 🎞️ Decided slot assignments from the child job's in-process domains (not the truncated grid).
    pub fn fill_decided_assignments(&self) -> std::collections::BTreeMap<String, Option<String>> {
        let mut assignments = std::collections::BTreeMap::new();
        for slot in &self.snapshot.slots {
            assignments.insert(slot.id.clone(), None);
        }
        if let Some(child) = self.child.as_ref() {
            let domains = child.domain_masks();
            for (index, domain) in domains.iter().enumerate() {
                if domain.count_ones() != 1 {
                    continue;
                }
                let Some(pattern) = domain.first_set() else { continue };
                let Some(slot) = self.snapshot.slots.get(index) else { continue };
                let Some(tile) = self.tile_ids.get(pattern.index()) else { continue };
                assignments.insert(slot.id.clone(), Some(tile.clone()));
            }
        }
        assignments
    }

    /// 📊 Fill progress counters from the child job, or zeros before it exists.
    pub fn fill_metrics(&self) -> (u64, u64) {
        let Some(child) = self.child.as_ref() else { return (0, 0) };
        let (observations, _edges, backtracks) = child.metrics();
        (observations, backtracks)
    }

    /// 🏷️ Engine stage id while the child runs; initialize-domains during preparation.
    pub fn fill_stage_id(&self) -> &'static str {
        let Some(child) = self.child.as_ref() else { return "wfc.initialize-domains" };
        match child.preview(0).stage {
            engine::job::WfcStage::InitializeDomains => "wfc.initialize-domains",
            engine::job::WfcStage::FindMinimumEntropySlot => "wfc.find-minimum-entropy-slot",
            engine::job::WfcStage::ChooseCandidate => "wfc.choose-candidate",
            engine::job::WfcStage::PropagateCompatibilityEdge => "wfc.propagate-compatibility-edge",
            engine::job::WfcStage::DetectContradiction => "wfc.detect-contradiction",
            engine::job::WfcStage::BacktrackTrailEntry => "wfc.backtrack-trail-entry",
            engine::job::WfcStage::CommitSlot => "wfc.commit-slot",
            engine::job::WfcStage::MaterializeCheckpoint => "wfc.materialize-checkpoint",
            engine::job::WfcStage::MaterializeCommit => "wfc.materialize-commit",
            engine::job::WfcStage::Complete => "wfc.complete",
        }
    }
}

impl semio_framework_job::InteractiveJob for Wfc3dInferenceJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-wfc3d-inference-operation");
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        loop {
            context.set_stage(match self.stage {
                Wfc3dInferenceStage::Tiles => "wfc3d.infer.tiles",
                Wfc3dInferenceStage::Relations => "wfc3d.infer.relations",
                Wfc3dInferenceStage::Rules => "wfc3d.infer.rules",
                Wfc3dInferenceStage::Model => "wfc3d.infer.model",
                Wfc3dInferenceStage::Compile => "wfc3d.infer.compile",
                Wfc3dInferenceStage::Slots => "wfc3d.infer.slots",
                Wfc3dInferenceStage::Edges => "wfc3d.infer.edges",
                Wfc3dInferenceStage::Topology => "wfc3d.infer.topology",
                Wfc3dInferenceStage::Fixed => "wfc3d.infer.fixed",
                Wfc3dInferenceStage::Restore => "wfc3d.infer.restore",
                Wfc3dInferenceStage::Solve => "wfc3d.infer.solve",
                Wfc3dInferenceStage::MapCommit => "wfc3d.infer.map-commit",
                Wfc3dInferenceStage::EncodeCommit => "wfc3d.infer.encode-commit",
                Wfc3dInferenceStage::Complete => "wfc3d.infer.complete",
            });
            match self.stage {
                Wfc3dInferenceStage::Tiles
                | Wfc3dInferenceStage::Relations
                | Wfc3dInferenceStage::Rules
                | Wfc3dInferenceStage::Model
                | Wfc3dInferenceStage::Compile
                | Wfc3dInferenceStage::Slots
                | Wfc3dInferenceStage::Edges
                | Wfc3dInferenceStage::Topology
                | Wfc3dInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Wfc3dInferenceStage::Restore => {
                    let outcome = self.restore.as_mut().expect("restore job").step(context);
                    return match outcome {
                        StepOutcome::Complete(candidate) => {
                            retire_payload(candidate.state);
                            retire_payload(candidate.output);
                            let mut restore = self.restore.take().expect("restore job");
                            self.child = restore.take_job();
                            engine::job::close_job(&mut restore);
                            self.stage = Wfc3dInferenceStage::Solve;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                Wfc3dInferenceStage::Solve => {
                    let outcome = self.child.as_mut().expect("WFC child").step(context);
                    return match outcome {
                        StepOutcome::Complete(candidate) => {
                            self.final_checkpoint = Some(candidate.state);
                            retire_payload(candidate.output);
                            let mut child = self.child.take().expect("WFC child");
                            self.child_commit = child.take_completed_commit();
                            engine::job::close_job(&mut child);
                            self.cursor = 0;
                            self.stage = Wfc3dInferenceStage::MapCommit;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                Wfc3dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Wfc3dInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("wfc3d output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return StepOutcome::Yield;
                            }
                        };
                        return StepOutcome::Complete(semio_framework_job::CommitCandidate {
                            state: self.final_checkpoint.take().unwrap_or_else(|| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)),
                            output,
                        });
                    }
                    Ok(false) => return StepOutcome::Yield,
                    Err(error) => {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                },
                Wfc3dInferenceStage::Complete => unreachable!("complete returns immediately"),
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
        if let Some(restore) = self.restore.as_mut() {
            match restore.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Complete if restore.terminal_is_empty() => {
                    self.restore = None;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if let Some(child) = self.child.as_mut() {
            match child.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Complete if child.terminal_is_empty() => {
                    self.child = None;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if self.rejected_output_page.is_some() {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.rejected_output_page = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    /// 🚪️ Terminal-empty is "this job owns nothing retainable any more", NOT "someone called
    /// `begin_close`". A `WorkerJobSession`/`BatchJobSession` drives the ladder itself and does not
    /// route `begin_close` into the job, so gating this on a `closing` flag makes the session's own
    /// `close_step` loop forever on a job that has in fact already released everything — measured on
    /// this artifact's first live solve, and the reason the ancestor `assembly` artifact could never
    /// run its solve inference in a test at all.
    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
    }
}

pub struct Wfc3dInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for Wfc3dInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(WFC3D_INFERENCE_JOB_KIND, WFC3D_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for Wfc3dInferenceJobFactory {
    type Payload = Wfc3dInferenceRequest;
    type Job = Wfc3dInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC3D_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_WFC3D_TILES + MAX_WFC3D_SLOTS + MAX_WFC3D_RULES + MAX_WFC3D_EDGES, 4_096, MAX_WFC3D_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Wfc3dInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc3d-inference-wire-decode:{error}")))?;
        let mut request: Wfc3dInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc3d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Wfc3dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.wfc3d.solve` inference — five handcrafted facet leaves.
pub fn wfc3d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.wfc.wfc3d.solve",
        inference: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}

pub fn register_wfc3d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Wfc3dInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job used by the public factory.
pub(crate) fn solve_with_job(snapshot: &Wfc3dSnapshot) -> Result<Wfc3dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Wfc3dInferenceJob::new(operation, Wfc3dInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "wfc3d.inference.headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("wfc3d-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("wfc3d-inference-headless-contention:{error:?}"))?;
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
                    .map_err(|error| format!("wfc3d-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<Wfc3dInferenceCommit>(text).map_err(|error| format!("wfc3d-invalid-commit:{error}"))),
            ),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("wfc3d-inference-cancelled".into())),
            semio_framework_job::StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => None,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return result.expect("terminal wfc3d inference outcome has result");
        }
        session.resume().map_err(|error| format!("wfc3d-inference-headless-resume:{error:?}"))?;
    }
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → tile id), or `Unsolved` for every non-`Solved` outcome
/// (unsatisfiable/contradiction/budget/cancellation) — see `Wfc3dContradiction` for the dedicated
/// satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Wfc3dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct Wfc3dSolve;

impl store::InferredField<Wfc3dSnapshot> for Wfc3dSolve {
    type Key = String;
    type Value = Wfc3dSolveResult;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) => Wfc3dSolveResult::Solved { assignments: solution.assignments },
            _ => Wfc3dSolveResult::Unsolved,
        }
    }
}

/// 🏁 The solve as one call, for the editor/viewer preview lane: the inferred assignment is rendered,
/// never persisted.
pub fn solve_assignments(snapshot: &Wfc3dSnapshot) -> BTreeMap<String, String> {
    solve_with_job(snapshot).map(|commit| commit.assignments).unwrap_or_default()
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "is this spec even solvable"
/// never has to decode a full assignment map to find out.
pub struct Wfc3dContradiction;

impl store::InferredField<Wfc3dSnapshot> for Wfc3dContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).is_ok()
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned slot (fully
/// determined), else the prior entropy over every tile's weight. SCOPE, honestly stated: this is the
/// PRIOR entropy before arc-consistency propagation narrows any slot's domain — a real WFC heuristic
/// (the same weighted-distribution math `engine::weights::WeightTable` encodes), but not the
/// POST-propagation entropy a live "which slot should I collapse next" UI would want.
pub struct Wfc3dEntropy;

impl store::InferredField<Wfc3dSnapshot> for Wfc3dEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["slots", "tiles"]
    }
    fn plan(snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.slots.iter().map(|slot| store::InferenceStep { key: slot.id.clone(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
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
    fn compute(snapshot: &Wfc3dSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_tile_id.as_ref());
        if pinned.is_some() {
            return 0.0;
        }
        shannon_entropy_over_tiles(snapshot)
    }
}

pub fn shannon_entropy_over_tiles(snapshot: &Wfc3dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| tile.weight).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|w| w / total).filter(|p| *p > 0.0).map(|p| p * p.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
