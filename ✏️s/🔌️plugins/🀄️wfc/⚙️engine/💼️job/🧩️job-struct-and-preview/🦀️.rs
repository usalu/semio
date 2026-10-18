//! 🧩️ Job struct and preview: the WFC job, its construction from a model or a checkpoint, the preview, the commit and the single step advance. See [`super`].

use super::*;

//#region 🧩️Job
pub struct WfcJob<T> {
    pub(super) operation: Operation,
    pub(super) model: CompiledModel,
    pub(super) topology: T,
    pub(super) config: WfcJobConfig,
    pub(super) initial_domains: Option<Vec<PatternSet>>,
    pub(super) fixed: Vec<(NodeId, PatternId)>,
    pub(super) state: WfcState,
    pub(super) checkpoint_build: Option<CheckpointBuild>,
    pub(super) final_checkpoint: Option<Vec<u8>>,
    pub(super) commit_build: Option<CommitBuild>,
    pub(super) completed_commit: Option<WfcCommit>,
    pub(super) preview_units: u64,
    pub(super) last_preview_ms: Option<u64>,
    pub(super) publication: Option<Box<Publication>>,
    pub(super) closing: bool,
    pub(super) visible: Vec<VisibleCell>,
}

impl<T: Topology + Clone> WfcJob<T> {
    pub fn new(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>) -> Self {
        let node_count = topology.node_count();
        assert!(initial_domains.as_ref().is_none_or(|domains| domains.len() == node_count));
        let state = WfcState {
            operation_id: operation.operation.0,
            base_revision: operation.base_revision.0,
            generation: operation.generation.0,
            operation_seed: operation.seed,
            preview_sequence: operation.preview_sequence,
            topology_nodes: node_count,
            model_fingerprint: model.fingerprint(),
            stage: WfcStage::InitializeDomains,
            domains: Vec::new(),
            domain_counts: Vec::new(),
            domain_weight_sums: Vec::new(),
            domain_weighted_log_sums: Vec::new(),
            singleton_count: 0,
            empty_count: 0,
            revisions: Vec::new(),
            init_node: 0,
            init_cursor: InitCursor::default(),
            queue: VecDeque::new(),
            queued_marks: Vec::new(),
            queue_epoch: 1,
            arc_cursor: None,
            choice_cursor: None,
            trail: Vec::new(),
            decisions: Vec::new(),
            backtrack_frame: None,
            entropy_heap: BinaryHeap::new(),
            rng: JobRng::from_seed(operation.seed),
            active_slot: None,
            tested_tile: None,
            contradiction: None,
            propagation_wave: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            changed_domains: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            backtrack_path: Vec::with_capacity(PREVIEW_ITEM_LIMIT),
            observations: 0,
            compatibility_edges: 0,
            backtracks: 0,
            observed: Vec::new(),
        };
        Self { operation, model, topology, config, initial_domains, fixed, state, checkpoint_build: None, final_checkpoint: None, commit_build: None, completed_commit: None, preview_units: 0, last_preview_ms: None, publication: None, closing: false, visible: Vec::new() }
    }

    pub fn from_checkpoint(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>, bytes: &[u8]) -> Result<Self, String>
    where
        T: Send + 'static,
    {
        let mut restore = WfcRestore::new(operation, model, topology, config, initial_domains, fixed, bytes.to_vec())?;
        let mut authority = admit_payload_authority(operation);
        let cancel = semio_framework_job::root_cancel_token();
        let mut sequence = 0;
        let result = loop {
            let mut receipt = RetainedCloneProgress::default();
            let mut context = StepContext::with_payload_authority(operation.operation, operation.generation, semio_framework_job::StepBudget::new(64, u64::MAX, HEADLESS_GRANT), &cancel, semio_framework_job::default_now_us, &mut sequence, &mut receipt, &authority).map_err(|error| format!("wfc-restore-context:{error:?}"))?;
            let end = match restore.step(&mut context) {
                Ok(Some(JobOutcomeBorrow::Complete { .. })) => Some(Ok(())),
                Ok(Some(JobOutcomeBorrow::Fault { detail, .. })) => Some(Err(String::from_utf8_lossy(&payload_bytes(detail)).into_owned())),
                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => Some(Err("wfc-restore-cancelled".to_string())),
                Ok(None | Some(JobOutcomeBorrow::Yield { .. } | JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. })) => None,
                Err(error) => Some(Err(format!("wfc-restore-step:{error:?}"))),
            };
            match end {
                Some(Ok(())) => break restore.take_job().ok_or_else(|| "wfc-restore-completed-without-job".to_string()),
                Some(Err(error)) => break Err(error),
                None => {}
            }
        };
        close_job(&mut restore);
        close_payload_authority(&mut authority);
        result
    }

    pub fn preview(&self, sequence: u64) -> WfcPreview {
        let mut remaining = PREVIEW_ITEM_LIMIT;
        let mut candidates = Vec::new();
        if let Some(domain) = self.state.active_slot.and_then(|node| self.state.domains.get(node.index())) {
            for index in 0..self.model.pattern_count() {
                if remaining == 0 {
                    break;
                }
                remaining -= 1;
                let pattern = PatternId::from_index(index);
                if domain.get(pattern) {
                    candidates.push(pattern.get());
                }
            }
        }
        let mut changed_domains = Vec::new();
        for &node in self.state.changed_domains.iter().take(PREVIEW_ITEM_LIMIT) {
            if remaining == 0 {
                break;
            }
            let mut patterns = Vec::new();
            for index in 0..self.model.pattern_count() {
                if remaining == 0 {
                    break;
                }
                remaining -= 1;
                let pattern = PatternId::from_index(index);
                if self.state.domains[node.index()].get(pattern) {
                    patterns.push(pattern.get());
                }
            }
            changed_domains.push((node.get(), patterns));
        }
        let incomplete_grid = self
            .state
            .domains
            .iter()
            .zip(self.state.domain_counts.iter())
            .take(PREVIEW_ITEM_LIMIT)
            .map(|(domain, &count)| {
                if count != 1 {
                    return None;
                }
                for index in 0..self.model.pattern_count() {
                    if remaining == 0 {
                        return None;
                    }
                    remaining -= 1;
                    let pattern = PatternId::from_index(index);
                    if domain.get(pattern) {
                        return Some(pattern.get());
                    }
                }
                None
            })
            .collect();
        let truncated = self.state.domains.len() > PREVIEW_ITEM_LIMIT
            || self.model.pattern_count() > PREVIEW_ITEM_LIMIT
            || self.state.propagation_wave.len() == PREVIEW_ITEM_LIMIT
            || self.state.changed_domains.len() == PREVIEW_ITEM_LIMIT
            || self.state.backtrack_path.len() == PREVIEW_ITEM_LIMIT
            || remaining == 0;
        WfcPreview {
            sequence,
            stage: self.state.stage,
            active_slot: self.state.active_slot.map(NodeId::get),
            candidates,
            tested_tile: self.state.tested_tile.map(PatternId::get),
            propagation_wave: self.state.propagation_wave.iter().copied().map(NodeId::get).collect(),
            changed_domains,
            contradiction: self.state.contradiction.map(NodeId::get),
            backtrack_path: self.state.backtrack_path.iter().copied().map(NodeId::get).collect(),
            incomplete_grid,
            domain_count: self.state.domains.len(),
            truncated,
            observations: self.state.observations,
            compatibility_edges: self.state.compatibility_edges,
            backtracks: self.state.backtracks,
        }
    }

    pub fn commit(&self) -> Option<WfcCommit> {
        (self.state.stage == WfcStage::Complete).then(|| WfcCommit {
            assignment: self.state.domains.iter().map(|domain| domain.first_set().expect("complete WFC domain").get()).collect(),
            observations: self.state.observations,
            compatibility_edges: self.state.compatibility_edges,
            backtracks: self.state.backtracks,
        })
    }

    pub fn domain_masks(&self) -> Vec<PatternSet> {
        self.state.domains.clone()
    }

    /// 👁 Writes one pattern id per node. `u32::MAX` means the domain is not a singleton.
    pub fn write_singleton_patterns(&self, into: &mut Vec<u32>) {
        into.clear();
        into.reserve(self.state.domain_counts.len());
        for (index, domain) in self.state.domains.iter().enumerate() {
            let pattern = if self.state.domain_counts[index] == 1 { domain.first_set().map(|pattern| pattern.get()).unwrap_or(u32::MAX) } else { u32::MAX };
            into.push(pattern);
        }
    }

    pub fn metrics(&self) -> (u64, u64, u64) {
        (self.state.observations, self.state.compatibility_edges, self.state.backtracks)
    }

    pub fn observed(&self) -> &[(NodeId, PatternId)] {
        &self.state.observed
    }

    /// 👁️ One solver unit with no payload admission, so a fill run can drain headlessly and
    /// publish a single trace of the collapses and discards that happened in the burst.
    pub fn advance_one(&mut self) -> SearchPulse {
        if self.state.stage == WfcStage::Complete {
            return self.terminal_pulse();
        }
        let observations = self.state.observations;
        let backtracks = self.state.backtracks;
        let observed_len = self.state.observed.len();
        match self.state.stage {
            WfcStage::InitializeDomains => self.initialize_one(),
            WfcStage::FindMinimumEntropySlot => self.find_slot(),
            WfcStage::ChooseCandidate => self.choose_one(),
            WfcStage::PropagateCompatibilityEdge => self.propagate_one(),
            WfcStage::DetectContradiction => self.detect(),
            WfcStage::BacktrackTrailEntry => self.backtrack_one(),
            WfcStage::CommitSlot => self.state.stage = WfcStage::FindMinimumEntropySlot,
            WfcStage::MaterializeCheckpoint | WfcStage::MaterializeCommit | WfcStage::Complete => {}
        }
        if self.state.observations > observations {
            if let Some((node, pattern)) = self.state.observed.get(observed_len) {
                return SearchPulse::Collapsed { node: node.get(), pattern: pattern.get() };
            }
        }
        if self.state.backtracks > backtracks {
            if let Some(frame) = self.state.backtrack_frame {
                return SearchPulse::Discarded { node: frame.node.get(), pattern: frame.candidate.get() };
            }
        }
        if self.state.stage == WfcStage::Complete {
            return self.terminal_pulse();
        }
        SearchPulse::Worked
    }

    fn terminal_pulse(&self) -> SearchPulse {
        if self.state.contradiction.is_some() || self.state.empty_count > 0 || self.state.singleton_count != self.state.domains.len() {
            SearchPulse::Unsatisfiable
        } else {
            SearchPulse::Solved
        }
    }

}
//#endregion 🧩️Job
