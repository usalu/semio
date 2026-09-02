//! 💡️ Assembly inferences — THE SOLVE ITSELF IS AN INFERENCE, exactly as the ticket's design
//! ruling states: `AssemblySnapshot` only ever persists the PROBLEM (slots/edges/modules/weights/
//! rules/seed); the SOLUTION, the contradiction/unsat verdict, and the pre-propagation entropy map
//! are all derived here via `store::InferredField`, never mutation-authored state. The 10,930 LOC
//! WFC implementation in the sibling `../🧩️wfc-engine/` compute tree becomes the internals of
//! these `compute()` bodies. Determinism: `solve_with_job` reads only `snapshot` fields (`seed`
//! included) and drives the same resumable `WfcJob` used by interactive callers; every step is
//! watchdog-wrapped and explicitly bounded. No ambient randomness enters the inference, so
//! `DepHash` caching over `AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy` is sound.

use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use semio_framework_job::InteractiveJob as _;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Compile
pub const ASSEMBLY_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const ASSEMBLY_INFERENCE_TOOL_ID: &str = "s.assembly.solve";
pub const ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA: &str = "s.assembly.inference.request.v1";

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn assembly_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "procedural",
        artifact_kind: "s.assembly",
        artifact_schema: "s.assembly",
        artifact_schema_version: 1,
        document_schema: "s.assembly",
        document_schema_version: 1,
        inference_schema: ASSEMBLY_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    }
}
const MAX_ASSEMBLY_MODULES: usize = 65_536;
const MAX_ASSEMBLY_SLOTS: usize = 65_536;
const MAX_ASSEMBLY_RULES: usize = 262_144;
const MAX_ASSEMBLY_EDGES: usize = 262_144;
const MAX_ASSEMBLY_WEIGHTS: usize = 262_144;
const MAX_ASSEMBLY_ID_BYTES: usize = 1_024;
const MAX_ASSEMBLY_OUTPUT_BYTES: usize = 1 << 20;
const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyInferenceRequest {
    pub snapshot: AssemblySnapshot,
    pub checkpoint: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyInferenceCommit {
    pub assignments: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AssemblyInferenceStage {
    Weights,
    Modules,
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

#[derive(serde::Serialize, serde::Deserialize)]
struct AssemblyInferencePreview {
    sequence: u64,
    stage: AssemblyInferenceStage,
    completed: usize,
    total: usize,
}

/// 🧵 Worker-owned parent transaction for Assembly compile, solve, restore, and authoritative mapping.
pub struct AssemblyInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: AssemblySnapshot,
    stage: AssemblyInferenceStage,
    cursor: usize,
    weight_by_id: BTreeMap<String, f64>,
    pattern_of: BTreeMap<String, crate::wfc_engine::ids::PatternId>,
    node_of: BTreeMap<String, crate::wfc_engine::ids::NodeId>,
    module_ids: Vec<String>,
    raw_weights: Vec<f64>,
    allowed_pairs: BTreeSet<(u32, u32)>,
    model_build: Option<crate::wfc_engine::model::AssemblyModelBuild>,
    model: Option<crate::wfc_engine::model::CompiledModel>,
    topology_build: Option<crate::wfc_engine::topology::AssemblyTopologyBuild>,
    topology: Option<crate::wfc_engine::topology::GraphTopology>,
    fixed: Vec<(crate::wfc_engine::ids::NodeId, crate::wfc_engine::ids::PatternId)>,
    checkpoint: Option<Vec<u8>>,
    restore: Option<crate::wfc_engine::job::WfcRestore<crate::wfc_engine::topology::GraphTopology>>,
    child: Option<crate::wfc_engine::job::WfcJob<crate::wfc_engine::topology::GraphTopology>>,
    child_commit: Option<crate::wfc_engine::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    assignments: BTreeMap<String, String>,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    output_started: bool,
    encoded_entries: usize,
    encode_total: usize,
    preview_units: u64,
    last_preview_ms: Option<u64>,
    closing: bool,
}

impl AssemblyInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: AssemblyInferenceRequest) -> Result<Self, String> {
        let snapshot = request.snapshot;
        if snapshot.modules.len() > MAX_ASSEMBLY_MODULES
            || snapshot.slots.len() > MAX_ASSEMBLY_SLOTS
            || snapshot.rules.len() > MAX_ASSEMBLY_RULES
            || snapshot.edges.len() > MAX_ASSEMBLY_EDGES
            || snapshot.weights.len() > MAX_ASSEMBLY_WEIGHTS
            || snapshot.edges.len().saturating_mul(2) > u32::MAX as usize
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > crate::wfc_engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("assembly-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: AssemblyInferenceStage::Weights,
            cursor: 0,
            weight_by_id: BTreeMap::new(),
            pattern_of: BTreeMap::new(),
            node_of: BTreeMap::new(),
            module_ids: Vec::new(),
            raw_weights: Vec::new(),
            allowed_pairs: BTreeSet::new(),
            model_build: None,
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
            closing: false,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_ASSEMBLY_ID_BYTES {
            Err("assembly-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    fn progress(&self) -> (usize, usize) {
        match self.stage {
            AssemblyInferenceStage::Weights => (self.cursor, self.snapshot.weights.len()),
            AssemblyInferenceStage::Modules => (self.cursor, self.snapshot.modules.len()),
            AssemblyInferenceStage::Rules => (self.cursor, self.snapshot.rules.len()),
            AssemblyInferenceStage::Model => self.model_build.as_ref().map_or((0, self.snapshot.modules.len()), |build| build.progress()),
            AssemblyInferenceStage::Slots | AssemblyInferenceStage::Fixed | AssemblyInferenceStage::MapCommit => (self.cursor, self.snapshot.slots.len()),
            AssemblyInferenceStage::Edges => (self.cursor, self.snapshot.edges.len()),
            AssemblyInferenceStage::Topology => self.topology_build.as_ref().map_or((0, self.snapshot.slots.len()), |build| build.progress()),
            AssemblyInferenceStage::Restore | AssemblyInferenceStage::Solve => (0, 1),
            AssemblyInferenceStage::EncodeCommit => (self.encoded_entries, self.encode_total),
            AssemblyInferenceStage::Complete => (1, 1),
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
        let payload = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Preview, &preview).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview));
        semio_framework_job::StepOutcome::PreviewReady(payload)
    }

    fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            AssemblyInferenceStage::Weights => {
                if let Some(weight) = self.snapshot.weights.get(self.cursor) {
                    Self::validate_id(&weight.module_id)?;
                    self.weight_by_id.entry(weight.module_id.clone()).or_insert(weight.weight);
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = AssemblyInferenceStage::Modules;
                }
            }
            AssemblyInferenceStage::Modules => {
                if let Some(module) = self.snapshot.modules.get(self.cursor) {
                    Self::validate_id(&module.child_id)?;
                    let pattern = crate::wfc_engine::ids::PatternId::from_index(self.cursor);
                    self.pattern_of.insert(module.child_id.clone(), pattern);
                    self.module_ids.push(module.child_id.clone());
                    self.raw_weights.push(self.weight_by_id.get(&module.child_id).copied().unwrap_or(1.0));
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = AssemblyInferenceStage::Rules;
                }
            }
            AssemblyInferenceStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    Self::validate_id(&rule.module_a_id)?;
                    Self::validate_id(&rule.module_b_id)?;
                    if rule.allowed {
                        if let (Some(a), Some(b)) = (self.pattern_of.get(&rule.module_a_id), self.pattern_of.get(&rule.module_b_id)) {
                            self.allowed_pairs.insert((a.get(), b.get()));
                            self.allowed_pairs.insert((b.get(), a.get()));
                        }
                    }
                    self.cursor += 1;
                } else if self.snapshot.slots.is_empty() {
                    self.stage = AssemblyInferenceStage::EncodeCommit;
                } else {
                    let weights = std::mem::take(&mut self.raw_weights);
                    let pairs = std::mem::take(&mut self.allowed_pairs);
                    self.model_build = Some(crate::wfc_engine::model::AssemblyModelBuild::new(weights, pairs).map_err(|error| format!("{error:?}"))?);
                    self.stage = AssemblyInferenceStage::Model;
                }
            }
            AssemblyInferenceStage::Model => {
                if let Some(model) = self.model_build.as_mut().expect("model build").step().map_err(|error| format!("{error:?}"))? {
                    self.model = Some(model);
                    self.model_build = None;
                    self.cursor = 0;
                    self.topology_build = Some(crate::wfc_engine::topology::AssemblyTopologyBuild::new(self.snapshot.slots.len()));
                    self.stage = AssemblyInferenceStage::Slots;
                }
            }
            AssemblyInferenceStage::Slots => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    Self::validate_id(&slot.id)?;
                    if let Some(pinned) = &slot.pinned_module_id {
                        Self::validate_id(pinned)?;
                    }
                    self.node_of.insert(slot.id.clone(), crate::wfc_engine::ids::NodeId::from_index(self.cursor));
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = AssemblyInferenceStage::Edges;
                }
            }
            AssemblyInferenceStage::Edges => {
                if let Some(edge) = self.snapshot.edges.get(self.cursor) {
                    Self::validate_id(&edge.from_slot_id)?;
                    Self::validate_id(&edge.to_slot_id)?;
                    if let (Some(&from), Some(&to)) = (self.node_of.get(&edge.from_slot_id), self.node_of.get(&edge.to_slot_id)) {
                        let relation = crate::wfc_engine::ids::RelationId(0);
                        let topology = self.topology_build.as_mut().expect("topology build");
                        topology.add_arc(from, to, relation).map_err(|error| format!("{error:?}"))?;
                        topology.add_arc(to, from, relation).map_err(|error| format!("{error:?}"))?;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = AssemblyInferenceStage::Topology;
                }
            }
            AssemblyInferenceStage::Topology => {
                if let Some(topology) = self.topology_build.as_mut().expect("topology build").step() {
                    self.topology = Some(topology);
                    self.topology_build = None;
                    self.cursor = 0;
                    self.stage = AssemblyInferenceStage::Fixed;
                }
            }
            AssemblyInferenceStage::Fixed => {
                if let Some(slot) = self.snapshot.slots.get(self.cursor) {
                    if let Some(pinned) = &slot.pinned_module_id {
                        if let (Some(&node), Some(&pattern)) = (self.node_of.get(&slot.id), self.pattern_of.get(pinned)) {
                            self.fixed.push((node, pattern));
                        }
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().expect("compiled model");
                    let topology = self.topology.take().expect("compiled topology");
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(crate::wfc_engine::job::WfcRestore::new(self.operation, model, topology, crate::wfc_engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = AssemblyInferenceStage::Restore;
                    } else {
                        self.child = Some(crate::wfc_engine::job::WfcJob::new(self.operation, model, topology, crate::wfc_engine::job::WfcJobConfig::default(), None, fixed));
                        self.stage = AssemblyInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile assembly inference stage"),
        }
        Ok(())
    }

    fn map_one(&mut self) -> Result<(), String> {
        let commit = self.child_commit.as_ref().ok_or("assembly-commit-missing")?;
        if self.cursor < self.snapshot.slots.len() {
            let slot = &self.snapshot.slots[self.cursor];
            let pattern = usize::try_from(*commit.assignment.get(self.cursor).ok_or("assembly-commit-missing-slot")?).map_err(|_| "assembly-commit-pattern-capacity")?;
            let module = self.module_ids.get(pattern).ok_or("assembly-commit-pattern-out-of-range")?;
            self.assignments.insert(slot.id.clone(), module.clone());
            self.cursor += 1;
        } else {
            self.child_commit = None;
            self.encode_total = self.assignments.len();
            self.stage = AssemblyInferenceStage::EncodeCommit;
        }
        Ok(())
    }

    fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let source = self.rejected_output_page.take().unwrap_or_default();
        let writer = self.output.as_mut().ok_or("assembly-output-writer-missing")?;
        let mut page = match context.admit_payload_page(writer, source) {
            Ok(page) => page,
            Err(rejected) => {
                self.rejected_output_page = Some(rejected.into_source());
                return Err("assembly-inference-output-admission-exceeded".into());
            }
        };
        if !self.output_started {
            page.write(br#"{"assignments":{"#).map_err(|_| "assembly-inference-output-page")?;
            page.commit();
            self.output_started = true;
            return Ok(false);
        }
        if let Some((slot, module)) = self.assignments.pop_first() {
            let slot = serde_json::to_string(&slot).map_err(|error| error.to_string())?;
            let module = serde_json::to_string(&module).map_err(|error| error.to_string())?;
            if self.encoded_entries != 0 {
                page.write(b",").map_err(|_| "assembly-inference-output-page")?;
            }
            page.write(slot.as_bytes()).and_then(|_| page.write(b":")).and_then(|_| page.write(module.as_bytes())).map_err(|_| "assembly-inference-output-page")?;
            page.commit();
            self.encoded_entries += 1;
            return Ok(false);
        }
        page.write(b"}}").map_err(|_| "assembly-inference-output-page")?;
        page.commit();
        self.stage = AssemblyInferenceStage::Complete;
        Ok(true)
    }
}

impl semio_framework_job::InteractiveJob for AssemblyInferenceJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, b"stale-assembly-inference-operation").unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        loop {
            context.set_stage(match self.stage {
                AssemblyInferenceStage::Weights => "assembly.infer.weights",
                AssemblyInferenceStage::Modules => "assembly.infer.modules",
                AssemblyInferenceStage::Rules => "assembly.infer.rules",
                AssemblyInferenceStage::Model => "assembly.infer.model",
                AssemblyInferenceStage::Slots => "assembly.infer.slots",
                AssemblyInferenceStage::Edges => "assembly.infer.edges",
                AssemblyInferenceStage::Topology => "assembly.infer.topology",
                AssemblyInferenceStage::Fixed => "assembly.infer.fixed",
                AssemblyInferenceStage::Restore => "assembly.infer.restore",
                AssemblyInferenceStage::Solve => "assembly.infer.solve",
                AssemblyInferenceStage::MapCommit => "assembly.infer.map-commit",
                AssemblyInferenceStage::EncodeCommit => "assembly.infer.encode-commit",
                AssemblyInferenceStage::Complete => "assembly.infer.complete",
            });
            match self.stage {
                AssemblyInferenceStage::Weights
                | AssemblyInferenceStage::Modules
                | AssemblyInferenceStage::Rules
                | AssemblyInferenceStage::Model
                | AssemblyInferenceStage::Slots
                | AssemblyInferenceStage::Edges
                | AssemblyInferenceStage::Topology
                | AssemblyInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, error.as_bytes()).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                AssemblyInferenceStage::Restore => {
                    let outcome = self.restore.as_mut().expect("restore job").step(context);
                    return match outcome {
                        StepOutcome::Complete(_) => {
                            self.child = self.restore.as_mut().expect("restore job").take_job();
                            self.restore = None;
                            self.stage = AssemblyInferenceStage::Solve;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                AssemblyInferenceStage::Solve => {
                    let outcome = self.child.as_mut().expect("WFC child").step(context);
                    return match outcome {
                        StepOutcome::Complete(candidate) => {
                            self.final_checkpoint = Some(candidate.state);
                            self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                            self.child = None;
                            self.cursor = 0;
                            self.stage = AssemblyInferenceStage::MapCommit;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                AssemblyInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, error.as_bytes()).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                AssemblyInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("assembly output writer").finish() {
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
                    Ok(false) => {}
                    Err(error) => {
                        let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, error.as_bytes()).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                },
                AssemblyInferenceStage::Complete => unreachable!("complete returns immediately"),
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

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
    }
}

pub struct AssemblyInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for AssemblyInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for AssemblyInferenceJobFactory {
    type Payload = AssemblyInferenceRequest;
    type Job = AssemblyInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_ASSEMBLY_MODULES + MAX_ASSEMBLY_SLOTS + MAX_ASSEMBLY_RULES + MAX_ASSEMBLY_EDGES + MAX_ASSEMBLY_WEIGHTS, 4_096, MAX_ASSEMBLY_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        AssemblyInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let mut request: AssemblyInferenceRequest = serde_json::from_slice(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("assembly-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        AssemblyInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

pub fn register_assembly_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(AssemblyInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job used by the public factory.
fn solve_with_job(snapshot: &AssemblySnapshot) -> Result<AssemblyInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "assembly.wfc.inference.headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("assembly-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("assembly-inference-headless-contention:{error:?}"))?;
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
            semio_framework_job::StepOutcome::Complete(candidate) => Some(serde_json::from_slice(&payload_bytes(&candidate.output)).map_err(|error| format!("assembly-invalid-commit:{error}"))),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("assembly-inference-cancelled".into())),
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
            return result.expect("terminal assembly inference outcome has result");
        }
        session.resume().map_err(|error| format!("assembly-inference-headless-resume:{error:?}"))?;
    }
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → module id), or `Unsolved` for every non-`Solved` outcome
/// (`Unsatisfiable`/`Contradiction`/budget/cancellation) — see `AssemblyContradiction` for the
/// dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssemblySolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct AssemblySolve;

impl store::InferredField<AssemblySnapshot> for AssemblySolve {
    type Key = String;
    type Value = AssemblySolveResult;

    const FIELD_ID: &'static str = "s.assembly.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "modules", "weights", "rules"]
    }
    fn plan(_snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "assembly".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(snapshot).expect("AssemblySnapshot serialization never fails")
    }
    fn compute(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) => AssemblySolveResult::Solved { assignments: solution.assignments },
            _ => AssemblySolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own — the natural sibling to `AssemblySolve` the design
/// calls out explicitly, so a caller who only needs "is this spec even solvable" never has to
/// decode a full assignment map to find out.
pub struct AssemblyContradiction;

impl store::InferredField<AssemblySnapshot> for AssemblyContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.assembly.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "modules", "weights", "rules"]
    }
    fn plan(_snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "assembly".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(snapshot).expect("AssemblySnapshot serialization never fails")
    }
    fn compute(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).is_ok()
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the module WEIGHT distribution — `0.0` for a `pinned_module_id`
/// slot (fully determined), else the prior entropy over every module's `AssemblyModuleWeight`
/// (neutral `1.0` when a module has no explicit weight entry). SCOPE, honestly stated: this is the
/// PRIOR entropy before arc-consistency propagation narrows any slot's domain — a real, useful WFC
/// heuristic (the same weighted-distribution math `wfc_engine::weights::WeightTable` encodes), but
/// not the POST-propagation entropy a live "which cell should I collapse next" UI would want; wiring
/// this field through `wfc_engine::propagate`/`prop_ac3` for a truly narrowed per-slot domain is a
/// real remaining increment, not done here.
pub struct AssemblyEntropy;

impl store::InferredField<AssemblySnapshot> for AssemblyEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.assembly.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["slots", "modules", "weights"]
    }
    fn plan(snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.slots.iter().map(|slot| store::InferenceStep { key: slot.id.clone(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &AssemblySnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_module_id.clone());
        bytes.extend_from_slice(pinned.unwrap_or_default().as_bytes());
        bytes.push(0);
        for module in &snapshot.modules {
            bytes.extend_from_slice(module.child_id.as_bytes());
            bytes.push(0);
        }
        for weight in &snapshot.weights {
            bytes.extend_from_slice(weight.module_id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&weight.weight.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &AssemblySnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_module_id.as_ref());
        if pinned.is_some() {
            return 0.0;
        }
        shannon_entropy_over_modules(snapshot)
    }
}

fn shannon_entropy_over_modules(snapshot: &AssemblySnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.modules.iter().map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map(|w| w.weight).unwrap_or(1.0)).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|w| w / total).filter(|p| *p > 0.0).map(|p| p * p.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::assembly::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge};
    use semio_framework::ToolJobFactory as _;
    use semio_framework_job::{allocate_operation_id, root_cancel_token, CommitValidation, Generation, InteractiveJob, Operation, RevisionId, StepBudget, StepContext, StepOutcome};
    use semio_framework_plugin::app::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode, WireArtifactInferenceRequest, WireArtifactInferenceResult, ARTIFACT_INFERENCE_WIRE_VERSION};
    use semio_framework_plugin::reactor::jobs::{cancel_job, checkpoint_jobs, restore_job, start_job, step_job, JobBudget, JobStep, JOB_KIND_INFER};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use store::InferredField;

    fn kit_child(id: &str) -> store::ArtifactChild<SemioKitSnapshot> {
        store::ArtifactChild::new(id.to_string(), store::os_io::ArtifactRef { artifact_id: id.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() } })
    }

    /// 🧸 Two slots, one edge, two modules ("a","b") mutually allowed to be adjacent — a WFC
    /// instance small enough to solve deterministically by hand: any seed must find SOME solution.
    fn two_slot_two_module_snapshot() -> AssemblySnapshot {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.seed = 7;
        snapshot.slots = vec![AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None }, AssemblySlot { id: "s2".into(), x: 1.0, y: 0.0, z: 0.0, pinned_module_id: None }];
        snapshot.edges = vec![AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() }];
        snapshot.modules = vec![kit_child("a"), kit_child("b")];
        snapshot.rules = vec![AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, params: SemioValue::default() }];
        snapshot
    }

    fn operation(seed: u64) -> Operation {
        Operation::new(allocate_operation_id(), RevisionId(11), Generation(7), seed)
    }

    struct MountedCompetingJob;

    impl InteractiveJob for MountedCompetingJob {
        fn step(&mut self, _context: &mut StepContext<'_>) -> StepOutcome {
            StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: b"replacement".to_vec() })
        }
    }

    struct MountedCompetingFactory {
        keys: [semio_framework::ToolFactoryKey; 1],
    }

    impl semio_framework::ToolJobFactory for MountedCompetingFactory {
        type Payload = AssemblyInferenceRequest;
        type Job = MountedCompetingJob;

        fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            "competing.assembly.inference.v1"
        }

        fn classification(&self) -> semio_framework::InteractiveJobClassification {
            semio_framework::InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
            semio_framework::ToolExecutionContract::bounded_first_step(1, 1, 1, 16, 100)
        }

        fn create_job(&mut self, _operation: Operation, _payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
            Ok(MountedCompetingJob)
        }
    }

    fn cold_route_request(snapshot: AssemblySnapshot, cancellation_id: &str, revision: u64, generation: u64) -> Vec<u8> {
        let request = WireArtifactInferenceRequest {
            wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
            owner: "procedural".into(),
            artifact_kind: "s.assembly".into(),
            artifact_schema: "s.assembly".into(),
            artifact_schema_version: 1,
            document_schema: "s.assembly".into(),
            document_schema_version: 1,
            inference_schema: ASSEMBLY_INFERENCE_TOOL_ID.into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            revision,
            generation,
            source_dialect: "s.assembly.standard.v1.dialect.canonical".into(),
            policy: Vec::new(),
            budgets: WireArtifactInferenceBudget { allocation_bytes: 4 << 20, work_units: 1, recursion_depth: 4 },
            cancellation_id: cancellation_id.into(),
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: serde_json::to_vec(&AssemblyInferenceRequest { snapshot, checkpoint: None }).expect("assembly cold payload"),
            dependencies: Vec::new(),
        };
        serde_json::to_vec(&request).expect("cold route request")
    }

    fn drive_cold_route(job: u64) -> WireArtifactInferenceResult {
        for _ in 0..200_000 {
            match step_job(job, JobBudget { fuel: 1, deadline_ms: 2 }).await {
                JobStep::Running(_) => {}
                JobStep::Done(bytes) => return serde_json::from_slice(&bytes).expect("cold route result"),
                JobStep::Failed(bytes) => {
                    let fault = dsl::decode_fault_bytes(&bytes);
                    panic!("cold route fault: {} {}", fault.code.0, fault.message);
                }
            }
        }
        panic!("cold route did not terminate");
    }

    fn step_job(job: &mut AssemblyInferenceJob, token: semio_framework_job::CancelToken, sequence: &mut u64) -> StepOutcome {
        let operation = job.operation();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), sequence);
        job.step(&mut context)
    }

    fn drive_job(job: &mut AssemblyInferenceJob) -> semio_framework_job::CommitCandidate {
        let mut sequence = 0;
        for _ in 0..4_000_000 {
            match step_job(job, root_cancel_token(), &mut sequence) {
                StepOutcome::Complete(candidate) => return candidate,
                StepOutcome::Fault(fault) => panic!("assembly inference fault: {}", String::from_utf8_lossy(&fault.detail)),
                StepOutcome::Cancelled => panic!("assembly inference unexpectedly cancelled"),
                _ => {}
            }
        }
        panic!("assembly inference did not complete");
    }

    fn checkpoint_job(job: &mut AssemblyInferenceJob) -> Vec<u8> {
        let mut sequence = 0;
        for _ in 0..4_000_000 {
            match step_job(job, root_cancel_token(), &mut sequence) {
                StepOutcome::CheckpointReady(checkpoint) => return checkpoint.state,
                StepOutcome::Fault(fault) => panic!("assembly inference fault before checkpoint: {}", String::from_utf8_lossy(&fault.detail)),
                outcome if outcome.is_terminal() => panic!("assembly inference terminated before checkpoint"),
                _ => {}
            }
        }
        panic!("assembly inference did not checkpoint");
    }

    #[test]
    fn solve_over_an_always_allowed_pair_finds_an_assignment_for_every_slot() {
        let snapshot = two_slot_two_module_snapshot();
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        match &values["assembly"] {
            AssemblySolveResult::Solved { assignments } => assert_eq!(assignments.len(), 2, "both slots must be assigned"),
            AssemblySolveResult::Unsolved => panic!("a trivially satisfiable spec must solve"),
        }
    }

    #[test]
    fn contradiction_field_agrees_with_solve_field_on_a_satisfiable_spec() {
        let snapshot = two_slot_two_module_snapshot();
        let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
        assert_eq!(satisfiable["assembly"], true);
    }

    #[test]
    fn an_unsatisfiable_spec_is_reported_as_a_contradiction_not_a_panic() {
        let mut snapshot = two_slot_two_module_snapshot();
        // 🚫 No rule allows "a" next to "a" or "b" next to "b" AND no rule allows "a"-"b" either
        // once we remove it — an edge with a fully closed-world empty allow-set is unsatisfiable.
        snapshot.rules.clear();
        let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
        assert_eq!(satisfiable["assembly"], false);
        let solved = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(solved["assembly"], AssemblySolveResult::Unsolved);
    }

    #[test]
    fn pinned_slot_always_resolves_to_its_pinned_module() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.slots[0].pinned_module_id = Some("a".into());
        snapshot.slots[1].pinned_module_id = Some("b".into());
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        match &values["assembly"] {
            AssemblySolveResult::Solved { assignments } => {
                assert_eq!(assignments["s1"], "a");
                assert_eq!(assignments["s2"], "b");
            }
            AssemblySolveResult::Unsolved => panic!("a pinned-and-allowed pair must solve"),
        }
    }

    #[test]
    fn empty_assembly_solves_trivially_with_no_assignments() {
        let snapshot = AssemblySnapshot::default();
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(values["assembly"], AssemblySolveResult::Solved { assignments: BTreeMap::new() });
    }

    #[test]
    fn pinned_slot_has_zero_entropy_unpinned_slot_does_not() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.slots[0].pinned_module_id = Some("a".into());
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert_eq!(entropy["s1"], 0.0);
        assert!(entropy["s2"] > 0.0, "an unpinned slot over two equally-weighted modules must have positive entropy");
    }

    #[test]
    fn uniform_weights_over_two_modules_yield_ln2_entropy() {
        let snapshot = two_slot_two_module_snapshot();
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert!((entropy["s1"] - std::f64::consts::LN_2).abs() < 1e-9);
    }

    #[test]
    fn skewed_weights_lower_entropy_than_uniform() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.weights = vec![AssemblyModuleWeight { module_id: "a".into(), weight: 100.0 }, AssemblyModuleWeight { module_id: "b".into(), weight: 0.01 }];
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert!(entropy["s1"] < std::f64::consts::LN_2, "a skewed distribution must have lower entropy than the uniform case");
    }

    /// 🔁 Determinism law: identical snapshots (same seed) must produce byte-identical solve
    /// results — `InferredField::compute` must be a pure function of `snapshot`, WFC's internal
    /// randomness notwithstanding, since the seed itself lives in the snapshot.
    #[test]
    fn identical_seed_and_spec_always_produce_the_same_solution() {
        let snapshot = two_slot_two_module_snapshot();
        let first = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        let second = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(first, second);
    }

    #[test]
    fn changing_only_the_seed_still_solves_a_trivially_satisfiable_spec() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.seed = 999;
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert!(matches!(values["assembly"], AssemblySolveResult::Solved { .. }));
    }

    #[test]
    fn maximum_admission_is_moved_without_clone_and_previews_on_fixed_cadence() {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.weights = vec![AssemblyModuleWeight { module_id: String::new(), weight: 1.0 }; MAX_ASSEMBLY_WEIGHTS];
        let allocation = snapshot.weights.as_ptr();
        let mut factory = AssemblyInferenceJobFactory::default();
        let mut job = factory.create_job(operation(113), AssemblyInferenceRequest { snapshot, checkpoint: None }).expect("maximum admitted request");
        assert_eq!(allocation, job.snapshot.weights.as_ptr());
        assert!(job.weight_by_id.is_empty() && job.output.is_empty() && job.model_build.is_none());
        let mut sequence = 0;
        let mut units_since_preview = 0;
        let mut previews = 0;
        for _ in 0..65 {
            units_since_preview += 1;
            match step_job(&mut job, root_cancel_token(), &mut sequence) {
                StepOutcome::PreviewReady(bytes) => {
                    let preview: AssemblyInferencePreview = serde_json::from_slice(&bytes).expect("preview");
                    assert_eq!(preview.stage, AssemblyInferenceStage::Weights);
                    assert!(units_since_preview <= PARENT_PREVIEW_UNIT_INTERVAL as usize);
                    if previews == 0 {
                        assert_eq!(units_since_preview, 1);
                    }
                    units_since_preview = 0;
                    previews += 1;
                }
                StepOutcome::Yield => {}
                outcome => panic!("unexpected maximum-admission outcome: {outcome:?}"),
            }
        }
        assert!(previews >= 5);
    }

    #[test]
    fn registered_public_factory_routes_exact_key_and_preserves_commit_freshness() {
        struct CompetingJob;
        impl InteractiveJob for CompetingJob {
            fn step(&mut self, _context: &mut StepContext<'_>) -> StepOutcome {
                StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: b"replacement".to_vec() })
            }
        }
        struct CompetingFactory {
            keys: [semio_framework::ToolFactoryKey; 1],
        }
        impl semio_framework::ToolJobFactory for CompetingFactory {
            type Payload = AssemblyInferenceRequest;
            type Job = CompetingJob;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                &self.keys
            }

            fn payload_schema_id(&self) -> &str {
                "competing.assembly.inference.v1"
            }

            fn classification(&self) -> semio_framework::InteractiveJobClassification {
                semio_framework::InteractiveJobClassification::Migrated
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolExecutionContract::bounded_first_step(1, 1, 1, 16, 100)
            }

            fn create_job(&mut self, _operation: Operation, _payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
                Ok(CompetingJob)
            }
        }

        let bus = semio_framework::ActionBus::new();
        register_assembly_inference_factory(&bus).expect("factory registration");
        register_assembly_inference_factory(&bus).expect("idempotent factory registration");
        let key = semio_framework::ToolFactoryKey::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID);
        assert!(bus.contains(&key));
        assert!(matches!(bus.register(CompetingFactory { keys: [key.clone()] }), Err(semio_framework::ToolRegistrationError::DuplicateKey { key: rejected }) if rejected == key));
        assert_eq!(bus.keys(), vec![key]);
        assert_eq!(bus.dispatch_count(), 0);
        let operation = operation(127);
        let spec = semio_framework::ToolOperationSpec::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, AssemblyInferenceRequest { snapshot: AssemblySnapshot::default(), checkpoint: None }, operation);
        let mut dispatch = bus.dispatch(spec).expect("registered inference lookup");
        assert_eq!(dispatch.spec.operation.base_revision, RevisionId(11));
        assert_eq!(dispatch.spec.operation.generation, Generation(7));
        let mut sequence = 0;
        let candidate = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            match dispatch.job.step(&mut context) {
                StepOutcome::Complete(candidate) => break candidate,
                StepOutcome::Fault(fault) => panic!("registered inference fault: {}", String::from_utf8_lossy(&fault.detail)),
                _ => {}
            }
        };
        assert_eq!(serde_json::from_slice::<AssemblyInferenceCommit>(&candidate.output).expect("commit"), AssemblyInferenceCommit::default());
        assert_eq!(semio_framework_job::validate_commit(&operation, RevisionId(11), Generation(7)), CommitValidation::Accepted);
        assert!(matches!(semio_framework_job::validate_commit(&operation, RevisionId(12), Generation(7)), CommitValidation::Stale { .. }));
        assert!(matches!(semio_framework_job::validate_commit(&operation, RevisionId(11), Generation(8)), CommitValidation::Stale { .. }));
    }

    #[test]
    fn registered_factory_job_rejects_stale_generation_before_first_unit() {
        let bus = semio_framework::ActionBus::new();
        register_assembly_inference_factory(&bus).expect("factory registration");
        let operation = operation(131);
        let spec =
            semio_framework::ToolOperationSpec::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, AssemblyInferenceRequest { snapshot: two_slot_two_module_snapshot(), checkpoint: None }, operation);
        let mut dispatch = bus.dispatch(spec).expect("registered inference lookup");
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(dispatch.job.step(&mut context), StepOutcome::Fault(_)));
    }

    #[test]
    fn restart_rebuilds_all_maps_from_owned_snapshot_after_process_state_loss() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.edges.clear();
        let operation = operation(snapshot.seed);
        let mut original = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("original job");
        let checkpoint = checkpoint_job(&mut original);
        drop(original);
        let mut restarted = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: Some(checkpoint) }).expect("restarted job");
        assert!(restarted.weight_by_id.is_empty() && restarted.pattern_of.is_empty() && restarted.node_of.is_empty() && restarted.module_ids.is_empty());
        let resumed = drive_job(&mut restarted);
        let mut fresh = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot, checkpoint: None }).expect("fresh job");
        let expected = drive_job(&mut fresh);
        assert_eq!(resumed.output, expected.output);
    }

    #[test]
    fn cancellation_is_lossless_during_compile_restore_and_authoritative_mapping() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.edges.clear();
        let operation = operation(snapshot.seed);
        let mut compile = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("compile job");
        let token = root_cancel_token();
        token.cancel_now();
        let mut sequence = 0;
        assert_eq!(step_job(&mut compile, token, &mut sequence), StepOutcome::Cancelled);
        assert_eq!((compile.stage, compile.cursor), (AssemblyInferenceStage::Weights, 0));

        let mut source = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("source job");
        let checkpoint = checkpoint_job(&mut source);
        let mut restored = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot, checkpoint: Some(checkpoint) }).expect("restore job");
        let mut cancelled_restore = false;
        for _ in 0..4_000_000 {
            if restored.stage == AssemblyInferenceStage::Restore {
                let token = root_cancel_token();
                token.cancel_now();
                let before = (restored.stage, restored.cursor, restored.assignments.len(), restored.output.len());
                assert_eq!(step_job(&mut restored, token, &mut sequence), StepOutcome::Cancelled);
                assert_eq!(before, (restored.stage, restored.cursor, restored.assignments.len(), restored.output.len()));
                cancelled_restore = true;
                break;
            }
            let _ = step_job(&mut restored, root_cancel_token(), &mut sequence);
        }
        assert!(cancelled_restore);
        for target in [AssemblyInferenceStage::MapCommit, AssemblyInferenceStage::EncodeCommit] {
            let mut cancelled_target = false;
            for _ in 0..4_000_000 {
                if restored.stage == target {
                    let token = root_cancel_token();
                    token.cancel_now();
                    let before = (restored.cursor, restored.assignments.len(), restored.output.len());
                    assert_eq!(step_job(&mut restored, token, &mut sequence), StepOutcome::Cancelled);
                    assert_eq!(before, (restored.cursor, restored.assignments.len(), restored.output.len()));
                    cancelled_target = true;
                    break;
                }
                match step_job(&mut restored, root_cancel_token(), &mut sequence) {
                    StepOutcome::Fault(fault) => panic!("restore fault: {}", String::from_utf8_lossy(&fault.detail)),
                    outcome if outcome.is_terminal() => panic!("restored inference completed before {target:?}"),
                    _ => {}
                }
            }
            assert!(cancelled_target, "did not reach {target:?}");
        }
        assert!(matches!(drive_job(&mut restored).output.as_slice(), [b'{', ..]));
    }

    #[semio_framework_async_macros::async_test]
    async fn mounted_semio_infer_routes_exact_assembly_job_through_checkpoint_restart() {
        register_assembly_inference_factory(&semio_framework::ActionBus::production()).expect("production assembly registration");
        let request = cold_route_request(two_slot_two_module_snapshot(), "assembly-mounted-restart", 41, 9);
        start_job(8_101, JOB_KIND_INFER, &request).await;

        let checkpoint = loop {
            match step_job(8_101, JobBudget { fuel: 1, deadline_ms: 2 }).await {
                JobStep::Running(_) => {
                    let entries = checkpoint_jobs().await;
                    if let Some(checkpoint) = entries.iter().find(|entry| entry.job == 8_101).and_then(|entry| entry.checkpoint.clone()) {
                        break checkpoint;
                    }
                }
                JobStep::Done(_) => panic!("mounted route completed before publishing a restart checkpoint"),
                JobStep::Failed(bytes) => {
                    let fault = dsl::decode_fault_bytes(&bytes);
                    panic!("mounted route failed before checkpoint: {} {}", fault.code.0, fault.message);
                }
            }
        };
        cancel_job(8_101).await;
        restore_job(8_101, JOB_KIND_INFER, &request, Some(checkpoint)).await;
        let result = drive_cold_route(8_101).await;
        assert_eq!(result.inference_schema, ASSEMBLY_INFERENCE_TOOL_ID);
        assert_eq!(result.revision, 41);
        assert_eq!(result.generation, 9);
        assert!(result.complete);
        let commit: AssemblyInferenceCommit = serde_json::from_slice(&result.canonical_payload).expect("assembly commit");
        assert_eq!(commit.assignments.len(), 2);
    }

    #[test]
    fn production_route_registration_is_idempotent_and_collision_safe() {
        let bus = semio_framework::ActionBus::production();
        register_assembly_inference_factory(&bus).expect("first production registration");
        register_assembly_inference_factory(&bus).expect("idempotent production registration");
        let key = semio_framework::ToolFactoryKey::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID);
        assert!(matches!(bus.register(MountedCompetingFactory { keys: [key.clone()] }), Err(semio_framework::ToolRegistrationError::DuplicateKey { key: rejected }) if rejected == key));
        assert_eq!(bus.payload_schema_id(&key).as_deref(), Some(ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA));
    }

    #[semio_framework_async_macros::async_test]
    async fn mounted_semio_infer_cancel_discards_the_live_job() {
        register_assembly_inference_factory(&semio_framework::ActionBus::production()).expect("production assembly registration");
        let request = cold_route_request(two_slot_two_module_snapshot(), "assembly-mounted-cancel", 42, 10);
        start_job(8_102, JOB_KIND_INFER, &request).await;
        let _ = step_job(8_102, JobBudget { fuel: 1, deadline_ms: 2 }).await;
        cancel_job(8_102).await;
        assert!(checkpoint_jobs().await.iter().all(|entry| entry.job != 8_102));
        assert!(matches!(step_job(8_102, JobBudget { fuel: 1, deadline_ms: 2 }).await, JobStep::Failed(_)));
    }

    fn run_exact_factory_on_pool(workers: usize) -> Vec<u8> {
        let bus = semio_framework::ActionBus::new();
        register_assembly_inference_factory(&bus).expect("assembly registration");
        let operation = operation(177);
        let payload = serde_json::to_vec(&AssemblyInferenceRequest { snapshot: two_slot_two_module_snapshot(), checkpoint: None }).expect("wire payload");
        let dispatch = bus.dispatch_wire(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, &payload, None, operation).expect("wire dispatch");
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::HeadlessBatch, workers));
        let session = semio_framework_job::WorkerJobSession::new(
            dispatch.job,
            semio_framework_job::BatchJobParams {
                operation: operation.operation,
                generation: operation.generation,
                cancel: root_cancel_token(),
                config: semio_framework_job::BatchDriveConfig { site: "assembly.wfc.worker-count", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
                now_us: semio_framework_job::default_now_us,
            },
        );
        for _ in 0..200_000 {
            match session.step(&pool, semio_framework_job::Lane::UserVisible).await.expect("worker outcome") {
                StepOutcome::Complete(candidate) => return candidate.output,
                StepOutcome::Fault(fault) => panic!("worker-count route fault: {}", String::from_utf8_lossy(&fault.detail)),
                StepOutcome::Cancelled => panic!("worker-count route cancelled"),
                _ => {}
            }
        }
        panic!("worker-count route did not terminate");
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_factory_replays_byte_identically_on_actual_worker_pools() {
        let default = std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
        let one = run_exact_factory_on_pool(1).await;
        assert_eq!(run_exact_factory_on_pool(2).await, one);
        assert_eq!(run_exact_factory_on_pool(4).await, one);
        assert_eq!(run_exact_factory_on_pool(default).await, one);
    }
}
//#endregion 🧪️Tests
