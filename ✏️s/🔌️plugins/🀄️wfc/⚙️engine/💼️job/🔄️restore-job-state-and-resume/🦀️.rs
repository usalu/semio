//! 🔄️ Restore job state and resume: the run outcome, the restore stage, header and preview and the restore job with its resumption steps. See [`super`].

use super::*;

//#region 🧩️Job
/// 🚦️ What one bounded run of the search loop decided before any outcome is lent.
pub(super) enum WfcRun {
    Yield,
    Cancelled,
    Staged,
}

//#region 🔄️RestoreJob
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue)]
pub(super) enum RestoreStage {
    Header,
    Domains,
    Trail,
    Decisions,
    Observed,
    Verify,
    Rebuild,
    Complete,
}

#[derive(Clone, Debug)]
pub(super) struct RestoreHeader {
    pub(super) operation_id: u64,
    pub(super) base_revision: u64,
    pub(super) generation: u64,
    pub(super) operation_seed: u64,
    pub(super) preview_sequence: u64,
    pub(super) topology_nodes: usize,
    pub(super) model_fingerprint: u64,
    pub(super) terminal: bool,
    pub(super) rng_state: [u64; 4],
    pub(super) observations: u64,
    pub(super) compatibility_edges: u64,
    pub(super) backtracks: u64,
    pub(super) domain_count: usize,
    pub(super) pattern_count: usize,
    pub(super) trail_count: usize,
    pub(super) decision_count: usize,
    pub(super) observed_count: usize,
}

#[derive(semio_framework_value_derive::ToValue)]
pub(super) struct RestorePreview {
    pub(super) sequence: u64,
    pub(super) stage: RestoreStage,
    pub(super) completed: usize,
    pub(super) total: usize,
}

/// 🔄️ Fuel-bounded checkpoint decoding and cache/heap reconstruction.
pub struct WfcRestore<T> {
    pub(super) operation: Operation,
    pub(super) model: Option<CompiledModel>,
    pub(super) topology: Option<T>,
    pub(super) config: WfcJobConfig,
    pub(super) initial_domains: Option<Vec<PatternSet>>,
    pub(super) fixed: Vec<(NodeId, PatternId)>,
    pub(super) bytes: Vec<u8>,
    pub(super) cursor: usize,
    pub(super) stage: RestoreStage,
    pub(super) header: Option<RestoreHeader>,
    pub(super) domains: Vec<PatternSet>,
    pub(super) domain_words: Vec<u64>,
    pub(super) trail: Vec<Removal>,
    pub(super) decisions: Vec<Decision>,
    pub(super) observed: Vec<(NodeId, PatternId)>,
    pub(super) domain_cursor: usize,
    pattern_cursor: usize,
    pub(super) count: u32,
    pub(super) weight_sum: f64,
    pub(super) weighted_log_sum: f64,
    pub(super) domain_counts: Vec<u32>,
    pub(super) weight_sums: Vec<f64>,
    pub(super) weighted_log_sums: Vec<f64>,
    pub(super) singleton_count: usize,
    pub(super) empty_count: usize,
    pub(super) revisions: Vec<u64>,
    pub(super) queued_marks: Vec<u64>,
    pub(super) entropy_heap: BinaryHeap<EntropyEntry>,
    pub(super) restored: Option<WfcJob<T>>,
    pub(super) preview_units: u64,
    pub(super) last_preview_ms: Option<u64>,
    pub(super) publication: Option<Box<Publication>>,
    pub(super) closing: bool,
}

impl<T: Topology + Clone> WfcRestore<T> {
    pub fn new(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>, bytes: Vec<u8>) -> Result<Self, String> {
        if bytes.len() > MAX_CHECKPOINT_BYTES {
            return Err("wfc-checkpoint-admission-exceeded".into());
        }
        Ok(Self {
            operation,
            model: Some(model),
            topology: Some(topology),
            config,
            initial_domains,
            fixed,
            bytes,
            cursor: 0,
            stage: RestoreStage::Header,
            header: None,
            domains: Vec::new(),
            domain_words: Vec::new(),
            trail: Vec::new(),
            decisions: Vec::new(),
            observed: Vec::new(),
            domain_cursor: 0,
            pattern_cursor: 0,
            count: 0,
            weight_sum: 0.0,
            weighted_log_sum: 0.0,
            domain_counts: Vec::new(),
            weight_sums: Vec::new(),
            weighted_log_sums: Vec::new(),
            singleton_count: 0,
            empty_count: 0,
            revisions: Vec::new(),
            queued_marks: Vec::new(),
            entropy_heap: BinaryHeap::new(),
            restored: None,
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            closing: false,
        })
    }

    pub(super) fn take(&mut self, count: usize) -> Result<&[u8], String> {
        let end = self.cursor.checked_add(count).ok_or("wfc-checkpoint-overflow")?;
        let value = self.bytes.get(self.cursor..end).ok_or("wfc-checkpoint-truncated")?;
        self.cursor = end;
        Ok(value)
    }

    pub(super) fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(size_of::<u32>())?.try_into().expect("u32 bytes")))
    }

    pub(super) fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(size_of::<u64>())?.try_into().expect("u64 bytes")))
    }

    fn decode_header(&mut self) -> Result<(), String> {
        if self.take(CHECKPOINT_MAGIC.len())? != CHECKPOINT_MAGIC {
            return Err("wfc-checkpoint-version-mismatch".into());
        }
        let operation_id = self.u64()?;
        let base_revision = self.u64()?;
        let generation = self.u64()?;
        let operation_seed = self.u64()?;
        let preview_sequence = self.u64()?;
        let topology_nodes = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        let model_fingerprint = self.u64()?;
        let terminal = match self.u64()? {
            0 => false,
            1 => true,
            _ => return Err("wfc-checkpoint-terminal-invalid".into()),
        };
        let mut rng_state = [0; 4];
        for word in &mut rng_state {
            *word = self.u64()?;
        }
        let observations = self.u64()?;
        let compatibility_edges = self.u64()?;
        let backtracks = self.u64()?;
        let domain_count = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        let pattern_count = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        let trail_count = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        let decision_count = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        let observed_count = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
        if self.cursor != CHECKPOINT_FIXED_HEADER_BYTES {
            return Err("wfc-checkpoint-header-size-mismatch".into());
        }
        let expected_bytes = CheckpointCounts { domain_count, pattern_count, trail_count, decision_count, observed_count }.checked_bytes().ok_or("wfc-checkpoint-capacity")?;
        let model = self.model.as_ref().expect("restore model");
        let topology = self.topology.as_ref().expect("restore topology");
        let operation_matches = operation_id == self.operation.operation.0 && base_revision == self.operation.base_revision.0 && generation == self.operation.generation.0 && operation_seed == self.operation.seed;
        if !operation_matches
            || model_fingerprint != model.fingerprint()
            || topology_nodes != topology.node_count()
            || domain_count != topology_nodes
            || pattern_count != model.pattern_count()
            || observed_count as u64 != observations
            || expected_bytes != self.bytes.len()
        {
            return Err("wfc-checkpoint-input-mismatch".into());
        }
        self.operation.preview_sequence = preview_sequence;
        self.header = Some(RestoreHeader {
            operation_id,
            base_revision,
            generation,
            operation_seed,
            preview_sequence,
            topology_nodes,
            model_fingerprint,
            terminal,
            rng_state,
            observations,
            compatibility_edges,
            backtracks,
            domain_count,
            pattern_count,
            trail_count,
            decision_count,
            observed_count,
        });
        self.stage = RestoreStage::Domains;
        Ok(())
    }

    pub(super) fn decode_one(&mut self) -> Result<(), String> {
        if self.stage == RestoreStage::Header {
            return self.decode_header();
        }
        let header = self.header.as_ref().expect("restore header").clone();
        match self.stage {
            RestoreStage::Header => unreachable!("header handled above"),
            RestoreStage::Domains => {
                if self.domains.len() == header.domain_count {
                    self.stage = RestoreStage::Trail;
                } else if self.domain_words.len() == header.pattern_count.div_ceil(64) {
                    let words = std::mem::take(&mut self.domain_words);
                    self.domains.push(PatternSet::from_words(header.pattern_count, words).ok_or("wfc-checkpoint-domain-invalid")?);
                } else {
                    let word = self.u64()?;
                    self.domain_words.push(word);
                }
            }
            RestoreStage::Trail => {
                if self.trail.len() == header.trail_count {
                    self.stage = RestoreStage::Decisions;
                } else {
                    let node = NodeId(self.u32()?);
                    let pattern = PatternId(self.u32()?);
                    if node.index() >= header.domain_count || pattern.index() >= header.pattern_count {
                        return Err("wfc-checkpoint-trail-out-of-range".into());
                    }
                    self.trail.push(Removal { node, pattern });
                }
            }
            RestoreStage::Decisions => {
                if self.decisions.len() == header.decision_count {
                    self.stage = RestoreStage::Observed;
                } else {
                    let node = NodeId(self.u32()?);
                    let candidate = PatternId(self.u32()?);
                    let trail_mark = usize::try_from(self.u64()?).map_err(|_| "wfc-checkpoint-capacity")?;
                    let mut rng_state = [0; 4];
                    for word in &mut rng_state {
                        *word = self.u64()?;
                    }
                    if node.index() >= header.domain_count || candidate.index() >= header.pattern_count || trail_mark > header.trail_count {
                        return Err("wfc-checkpoint-decision-out-of-range".into());
                    }
                    self.decisions.push(Decision { node, candidate, trail_mark, rng_state });
                }
            }
            RestoreStage::Observed => {
                if self.observed.len() == header.observed_count {
                    self.stage = RestoreStage::Verify;
                } else {
                    let node = NodeId(self.u32()?);
                    let pattern = PatternId(self.u32()?);
                    if node.index() >= header.domain_count || pattern.index() >= header.pattern_count {
                        return Err("wfc-checkpoint-observation-out-of-range".into());
                    }
                    self.observed.push((node, pattern));
                }
            }
            RestoreStage::Verify => {
                if self.cursor != self.bytes.len() {
                    return Err("wfc-checkpoint-trailing-bytes".into());
                }
                self.stage = RestoreStage::Rebuild;
            }
            RestoreStage::Rebuild => self.rebuild_one(),
            RestoreStage::Complete => self.finish(),
        }
        Ok(())
    }

    fn rebuild_one(&mut self) {
        let header = self.header.as_ref().expect("restore header");
        if self.domain_cursor == header.domain_count {
            self.stage = RestoreStage::Complete;
            return;
        }
        if self.pattern_cursor < header.pattern_count {
            let pattern = PatternId::from_index(self.pattern_cursor);
            self.pattern_cursor += 1;
            if self.domains[self.domain_cursor].get(pattern) {
                let weight = self.model.as_ref().expect("restore model").weights().w(pattern);
                self.count += 1;
                self.weight_sum += weight;
                self.weighted_log_sum += self.model.as_ref().expect("restore model").weights().w_ln_w(pattern);
            }
            return;
        }
        self.domain_counts.push(self.count);
        self.weight_sums.push(self.weight_sum);
        self.weighted_log_sums.push(self.weighted_log_sum);
        self.singleton_count += usize::from(self.count == 1);
        self.empty_count += usize::from(self.count == 0);
        if self.count > 1 {
            let entropy = if self.weight_sum > 0.0 { self.weight_sum.ln() - self.weighted_log_sum / self.weight_sum } else { 0.0 };
            self.entropy_heap.push(EntropyEntry { entropy_bits: entropy.to_bits(), node: NodeId::from_index(self.domain_cursor), revision: 0 });
        }
        self.revisions.push(0);
        self.queued_marks.push(0);
        self.domain_cursor += 1;
        self.pattern_cursor = 0;
        self.count = 0;
        self.weight_sum = 0.0;
        self.weighted_log_sum = 0.0;
    }

    pub(super) fn finish(&mut self) {
        let header = self.header.take().expect("restore header");
        let state = WfcState {
            operation_id: header.operation_id,
            base_revision: header.base_revision,
            generation: header.generation,
            operation_seed: header.operation_seed,
            preview_sequence: header.preview_sequence,
            topology_nodes: header.topology_nodes,
            model_fingerprint: header.model_fingerprint,
            stage: if header.terminal { WfcStage::Complete } else { WfcStage::FindMinimumEntropySlot },
            domains: std::mem::take(&mut self.domains),
            domain_counts: std::mem::take(&mut self.domain_counts),
            domain_weight_sums: std::mem::take(&mut self.weight_sums),
            domain_weighted_log_sums: std::mem::take(&mut self.weighted_log_sums),
            singleton_count: self.singleton_count,
            empty_count: self.empty_count,
            revisions: std::mem::take(&mut self.revisions),
            init_node: header.domain_count,
            init_cursor: InitCursor::default(),
            queue: VecDeque::new(),
            queued_marks: std::mem::take(&mut self.queued_marks),
            queue_epoch: 1,
            arc_cursor: None,
            choice_cursor: None,
            trail: std::mem::take(&mut self.trail),
            decisions: std::mem::take(&mut self.decisions),
            backtrack_frame: None,
            entropy_heap: std::mem::take(&mut self.entropy_heap),
            rng: JobRng { state: header.rng_state },
            active_slot: None,
            tested_tile: None,
            contradiction: None,
            propagation_wave: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            changed_domains: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            backtrack_path: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            observations: header.observations,
            compatibility_edges: header.compatibility_edges,
            backtracks: header.backtracks,
            observed: std::mem::take(&mut self.observed),
        };
        self.restored = Some(WfcJob {
            operation: self.operation,
            model: self.model.take().expect("restore model"),
            topology: self.topology.take().expect("restore topology"),
            config: self.config,
            initial_domains: self.initial_domains.take(),
            fixed: std::mem::take(&mut self.fixed),
            state,
            checkpoint_build: None,
            final_checkpoint: None,
            commit_build: None,
            completed_commit: None,
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            closing: false,
            visible: Vec::new(),
        });
    }

    pub fn take_job(&mut self) -> Option<WfcJob<T>> {
        self.restored.take()
    }

    pub(super) fn progress(&self) -> (usize, usize) {
        let Some(header) = &self.header else {
            return (0, 1);
        };
        match self.stage {
            RestoreStage::Header => (0, 1),
            RestoreStage::Domains => (self.domains.len(), header.domain_count),
            RestoreStage::Trail => (self.trail.len(), header.trail_count),
            RestoreStage::Decisions => (self.decisions.len(), header.decision_count),
            RestoreStage::Observed => (self.observed.len(), header.observed_count),
            RestoreStage::Verify => (0, 1),
            RestoreStage::Rebuild => (self.domain_cursor, header.domain_count),
            RestoreStage::Complete => (1, 1),
        }
    }
}
//#endregion 🔄️RestoreJob
//#endregion 🧩️Job
