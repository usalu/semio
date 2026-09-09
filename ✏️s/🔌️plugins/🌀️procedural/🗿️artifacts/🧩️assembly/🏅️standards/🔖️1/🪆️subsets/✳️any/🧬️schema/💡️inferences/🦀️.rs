//! 💡️ Assembly inferences — THE SOLVE ITSELF IS AN INFERENCE, exactly as the ticket's design
//! ruling states: `AssemblySnapshot` only ever persists the PROBLEM (slots/edges/modules/weights/
//! rules/seed); the SOLUTION, the contradiction/unsat verdict, and the pre-propagation entropy map
//! are all derived here via `store::InferredField`, never mutation-authored state. The 10,930 LOC
//! WFC implementation in the sibling `../🧩️wfc-engine/` compute tree becomes the internals of
//! these `compute()` bodies. Determinism: `solve_with_job` reads only `snapshot` fields (`seed`
//! included) and drives the same resumable `WfcJob` used by interactive callers; every step is
//! watchdog-wrapped and explicitly bounded. No ambient randomness enters the inference, so
//! `DepHash` caching over `AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy` is sound.

use crate::schema::snapshot::AssemblySnapshot;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Compile
pub const ASSEMBLY_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const ASSEMBLY_INFERENCE_TOOL_ID: &str = "s.assembly.solve";
pub const ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA: &str = "s.assembly.inference.request.v1";

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn assembly_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "procedural",
        artifact_kind: "s.procedural.assembly",
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

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct AssemblyInferenceRequest {
    pub snapshot: AssemblySnapshot,
    pub checkpoint: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AssemblyInferenceCommit {
    pub assignments: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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

#[cfg(test)]
#[derive(semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
            let slot = protocol::json::to_json_string(&slot);
            let module = protocol::json::to_json_string(&module);
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
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("assembly-inference-wire-decode:{error}")))?;
        let mut request: AssemblyInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("assembly-inference-wire-decode:{error}")))?;
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
            semio_framework_job::StepOutcome::Complete(candidate) => Some(
                std::str::from_utf8(&payload_bytes(&candidate.output))
                    .map_err(|error| format!("assembly-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<AssemblyInferenceCommit>(text).map_err(|error| format!("assembly-invalid-commit:{error}"))),
            ),
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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
        protocol::json::to_json_string(snapshot).into_bytes()
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
        protocol::json::to_json_string(snapshot).into_bytes()
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
    let weights: Vec<f64> = snapshot.modules.iter().map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map_or(1.0, |w| w.weight)).collect();
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
