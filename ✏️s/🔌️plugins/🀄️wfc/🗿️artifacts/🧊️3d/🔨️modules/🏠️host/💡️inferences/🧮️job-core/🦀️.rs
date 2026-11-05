//! 🧮️ Job core of the WFC 3D inference: construction, the solve steps and the run, lent and child-end vocabulary. See [`super`].

use super::*;

impl Wfc3dInferenceJob {
    pub fn new(mut operation: semio_framework_job::Operation, request: Wfc3dInferenceRequest) -> Result<Self, String> {
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
            assignments: BTreeMap::new(),
            output: Some(semio_framework_job_payload::RetainedPayloadBuilder::new(semio_framework_job_payload::JobPayloadStream::CommitOutput)),
            output_chunk: None,
            output_cursor: 0,
            output_next: 0,
            output_complete: false,
            output_started: false,
            encoded_entries: 0,
            encode_total: 0,
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            lent: Wfc3dInferenceJobLent::Own,
            restore_done: false,
            child_end: None,
            output_payload: None,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub(crate) fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_WFC3D_ID_BYTES {
            Err("wfc3d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
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

    pub(crate) fn stage_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Wfc3dInferenceJobRun {
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
        self.publication = Some(engine::job::Publication::new(engine::job::PublicationKind::Preview, preview.to_vec(), Vec::new()));
        Wfc3dInferenceJobRun::Staged
    }

    /// 🧯️ Stages the fault detail as the next lent outcome.
    pub(crate) fn stage_fault(&mut self, detail: Vec<u8>) -> Wfc3dInferenceJobRun {
        self.publication = Some(engine::job::Publication::new(engine::job::PublicationKind::Fault, detail, Vec::new()));
        Wfc3dInferenceJobRun::Staged
    }

    pub(crate) fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    pub(crate) fn advance_compile(&mut self) -> Result<(), String> {
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

    pub(crate) fn map_one(&mut self) -> Result<(), String> {
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
    pub(crate) fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::receive_original_retained_work;
        if !self.output.as_ref().expect("output builder").is_initialized() {
            let turn = self.output.as_mut().unwrap().advance_initialization(context); receive_original_retained_work(context, turn)?; return Ok(());
        }
        if let Some(chunk) = self.output_chunk.as_ref() {
            if self.output_cursor < chunk.len() {
                let turn = self.output.as_mut().unwrap().append_original(context, chunk, &mut self.output_cursor);
                receive_original_retained_work(context, turn)?; return Ok(());
            }
            semio_framework_job::close_original_granted_step(context, |grant| {
                let released_bytes = self.output_chunk.as_ref().unwrap().capacity();
                if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_release_bytes < released_bytes { return semio_framework_value::RetainedWorkOutcome { result: Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() }), work: 0 }; }
                self.output_chunk = None;
                match self.output_next { 0 => self.output_started = true, 1 => self.encoded_entries += 1, _ => self.output_complete = true }
                semio_framework_value::RetainedWorkOutcome { result: Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_job::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } }), work: 1 }
            })?;
            return Ok(());
        }
        if self.output_complete {
            let output = self.output.as_mut().unwrap();
            if output.published().is_none() { let turn = output.seal(context); receive_original_retained_work(context, turn)?; return Ok(()); }
            let turn = output.take_finished(&mut self.output_payload, context.retained_grant()); receive_original_retained_work(context, turn)?;
            if self.output_payload.is_some() { self.stage = Wfc3dInferenceStage::Complete; }
            return Ok(());
        }
        let grant = context.retained_grant();
        if context.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < 32768 { return Ok(()); }
        let (chunk, next) = if !self.output_started { (br#"{"assignments":{"#.to_vec(), 0) }
        else if let Some((slot, tile)) = self.assignments.pop_first() {
            let slot = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&slot);
            let tile = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&tile);
            (format!("{}{}:{}", if self.encoded_entries == 0 { "" } else { "," }, slot, tile).into_bytes(), 1)
        } else { (b"}}".to_vec(), 2) };
        self.output_chunk = Some(chunk); self.output_cursor = 0; self.output_next = next;
        let capacity = self.output_chunk.as_ref().unwrap().capacity();
        context.consume_fuel(1);
        context.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?;
        Ok(())
    }

}

impl Wfc3dInferenceJob {
    /// 🎞️ Decided slot assignments from the child job's in-process domains (not the truncated grid).
    pub fn fill_decided_assignments(&self) -> BTreeMap<String, Option<String>> {
        let mut assignments = BTreeMap::new();
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

/// 🚦️ What one bounded run decided before any outcome is lent.
pub(crate) enum Wfc3dInferenceJobRun {
    Yield,
    Cancelled,
    Staged,
    StepRestore,
    StepChild,
    StepOutput,
}

/// 🤝️ Which retained owner produced the outcome currently on loan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Wfc3dInferenceJobLent {
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
