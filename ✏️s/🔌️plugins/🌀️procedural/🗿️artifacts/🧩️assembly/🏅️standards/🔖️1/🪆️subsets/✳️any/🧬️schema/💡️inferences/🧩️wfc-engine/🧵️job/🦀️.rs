//! 🧵️ Persistent, fuel-bounded WFC execution for interactive and batch workers.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId, RelationId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::topology::Topology;

//#region 🧭️Protocol
const PREVIEW_ITEM_LIMIT: usize = 256;
const PREVIEW_UNIT_INTERVAL: u64 = 16;
const PREVIEW_TIME_INTERVAL_MS: u64 = 16;
const CHECKPOINT_INTERVAL: u64 = 64;
const CHECKPOINT_MAGIC: &[u8; 8] = b"SWFCJ002";
pub(crate) const MAX_CHECKPOINT_BYTES: usize = 1 << 20;
const CHECKPOINT_HEADER_IDENTITY_U64_FIELDS: usize = 8;
const CHECKPOINT_HEADER_RNG_U64_FIELDS: usize = 4;
const CHECKPOINT_HEADER_PROGRESS_U64_FIELDS: usize = 3;
const CHECKPOINT_HEADER_COUNT_U64_FIELDS: usize = 5;
const CHECKPOINT_HEADER_U64_FIELDS: usize = CHECKPOINT_HEADER_IDENTITY_U64_FIELDS + CHECKPOINT_HEADER_RNG_U64_FIELDS + CHECKPOINT_HEADER_PROGRESS_U64_FIELDS + CHECKPOINT_HEADER_COUNT_U64_FIELDS;
const CHECKPOINT_FIXED_HEADER_BYTES: usize = CHECKPOINT_MAGIC.len() + CHECKPOINT_HEADER_U64_FIELDS * std::mem::size_of::<u64>();
const CHECKPOINT_TRAIL_ENTRY_BYTES: usize = 2 * std::mem::size_of::<u32>();
const CHECKPOINT_DECISION_ENTRY_BYTES: usize = 2 * std::mem::size_of::<u32>() + 5 * std::mem::size_of::<u64>();
const CHECKPOINT_OBSERVED_ENTRY_BYTES: usize = 2 * std::mem::size_of::<u32>();
const MAX_COMMIT_BYTES: usize = 1 << 20;
const COMMIT_FIXED_MAX_BYTES: usize = 160;
const COMMIT_ITEM_MAX_BYTES: usize = 11;
const MAX_COMMIT_ITEMS: usize = (MAX_COMMIT_BYTES - COMMIT_FIXED_MAX_BYTES) / COMMIT_ITEM_MAX_BYTES;

fn empty_job_fault() -> JobFault {
    JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }
}

fn retained_payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        if let Some(page) = payload.page(index) {
            bytes.extend_from_slice(page);
        }
    }
    bytes
}

fn retained_payload(context: &mut StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
    context.payload_from_bytes(stream, bytes).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(stream))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum WfcStage {
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

impl WfcStage {
    fn label(self) -> &'static str {
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
}

#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct WfcPreview {
    pub sequence: u64,
    pub stage: WfcStage,
    pub active_slot: Option<u32>,
    pub candidates: Vec<u32>,
    pub tested_tile: Option<u32>,
    pub propagation_wave: Vec<u32>,
    pub changed_domains: Vec<(u32, Vec<u32>)>,
    pub contradiction: Option<u32>,
    pub backtrack_path: Vec<u32>,
    pub incomplete_grid: Vec<Option<u32>>,
    pub domain_count: usize,
    pub truncated: bool,
    pub observations: u64,
    pub compatibility_edges: u64,
    pub backtracks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct WfcCommit {
    pub assignment: Vec<u32>,
    pub observations: u64,
    pub compatibility_edges: u64,
    pub backtracks: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WfcSampler {
    #[default]
    WeightedRoulette,
    Uniform,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WfcJobConfig {
    pub sampler: WfcSampler,
}
//#endregion 🧭️Protocol

//#region 🎲️Determinism
#[derive(Clone, Copy, Debug)]
struct JobRng {
    state: [u64; 4],
}

impl JobRng {
    fn from_seed(seed: u64) -> Self {
        let mut cursor = seed;
        let mut state = [0; 4];
        for word in &mut state {
            cursor = cursor.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = cursor;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            *word = z ^ (z >> 31);
        }
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let result = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
        result
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    fn range(&mut self, hi: u64) -> u64 {
        if hi == 0 {
            return 0;
        }
        ((u128::from(self.next_u64()) * u128::from(hi)) >> 64) as u64
    }
}
//#endregion 🎲️Determinism

//#region 📚️PersistentState
#[derive(Clone, Copy, Debug)]
struct Removal {
    node: NodeId,
    pattern: PatternId,
}

#[derive(Clone, Copy, Debug)]
struct Decision {
    node: NodeId,
    candidate: PatternId,
    trail_mark: usize,
    rng_state: [u64; 4],
}

#[derive(Clone, Copy, Debug, Eq)]
struct EntropyEntry {
    entropy_bits: u64,
    node: NodeId,
    revision: u64,
}

impl PartialEq for EntropyEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.entropy_bits, self.node, self.revision) == (other.entropy_bits, other.node, other.revision)
    }
}

impl Ord for EntropyEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.entropy_bits.cmp(&self.entropy_bits).then_with(|| other.node.cmp(&self.node)).then_with(|| other.revision.cmp(&self.revision))
    }
}

impl PartialOrd for EntropyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum InitPhase {
    #[default]
    Acquire,
    BuildFull,
    Fixed,
    Measure,
}

#[derive(Clone, Debug, Default)]
struct InitCursor {
    phase: InitPhase,
    word: usize,
    fixed: usize,
    pattern: usize,
    required: Option<PatternId>,
    domain: Option<PatternSet>,
    count: u32,
    weight_sum: f64,
    weighted_log_sum: f64,
}

#[derive(Clone, Copy, Debug)]
enum ChoicePhase {
    Weigh,
    Select,
    Remove,
}

#[derive(Clone, Copy, Debug)]
struct ChoiceCursor {
    phase: ChoicePhase,
    pattern: usize,
    total: f64,
    target: f64,
    running: f64,
    ordinal: u64,
    candidate: Option<PatternId>,
}

#[derive(Clone, Debug)]
struct ArcCursor {
    source: NodeId,
    raw_index: usize,
    raw_bound: usize,
    target: Option<NodeId>,
    relation: Option<RelationId>,
    allowed: Option<PatternSet>,
    allowed_word: usize,
    allowed_pattern: usize,
    allowed_accumulator: u64,
    restrict_word: usize,
    removed_word: u64,
    removed_word_index: usize,
    changed: bool,
}

impl ArcCursor {
    fn new(source: NodeId, raw_bound: usize) -> Self {
        Self { source, raw_index: 0, raw_bound, target: None, relation: None, allowed: None, allowed_word: 0, allowed_pattern: 0, allowed_accumulator: 0, restrict_word: 0, removed_word: 0, removed_word_index: 0, changed: false }
    }

    fn clear_arc(&mut self) {
        self.target = None;
        self.relation = None;
        self.allowed = None;
        self.allowed_word = 0;
        self.allowed_pattern = 0;
        self.allowed_accumulator = 0;
        self.restrict_word = 0;
        self.removed_word = 0;
        self.removed_word_index = 0;
        self.changed = false;
    }
}

#[derive(Clone, Debug)]
struct WfcState {
    operation_id: u64,
    base_revision: u64,
    generation: u64,
    operation_seed: u64,
    preview_sequence: u64,
    topology_nodes: usize,
    model_fingerprint: u64,
    stage: WfcStage,
    domains: Vec<PatternSet>,
    domain_counts: Vec<u32>,
    domain_weight_sums: Vec<f64>,
    domain_weighted_log_sums: Vec<f64>,
    singleton_count: usize,
    empty_count: usize,
    revisions: Vec<u64>,
    init_node: usize,
    init_cursor: InitCursor,
    queue: VecDeque<NodeId>,
    queued_marks: Vec<u64>,
    queue_epoch: u64,
    arc_cursor: Option<ArcCursor>,
    choice_cursor: Option<ChoiceCursor>,
    trail: Vec<Removal>,
    decisions: Vec<Decision>,
    backtrack_frame: Option<Decision>,
    entropy_heap: BinaryHeap<EntropyEntry>,
    rng: JobRng,
    active_slot: Option<NodeId>,
    tested_tile: Option<PatternId>,
    contradiction: Option<NodeId>,
    propagation_wave: Vec<NodeId>,
    changed_domains: Vec<NodeId>,
    backtrack_path: Vec<NodeId>,
    observations: u64,
    compatibility_edges: u64,
    backtracks: u64,
    observed: Vec<(NodeId, PatternId)>,
}
//#endregion 📚️PersistentState

//#region 💾️BoundedMaterialization
#[derive(Clone, Copy, Debug)]
struct CheckpointCounts {
    domain_count: usize,
    pattern_count: usize,
    trail_count: usize,
    decision_count: usize,
    observed_count: usize,
}

impl CheckpointCounts {
    fn from_state(state: &WfcState, pattern_count: usize) -> Self {
        Self { domain_count: state.domains.len(), pattern_count, trail_count: state.trail.len(), decision_count: state.decisions.len(), observed_count: state.observed.len() }
    }

    fn checked_bytes(self) -> Option<usize> {
        let domain_bytes = self.domain_count.checked_mul(self.pattern_count.div_ceil(u64::BITS as usize))?.checked_mul(std::mem::size_of::<u64>())?;
        CHECKPOINT_FIXED_HEADER_BYTES
            .checked_add(domain_bytes)?
            .checked_add(self.trail_count.checked_mul(CHECKPOINT_TRAIL_ENTRY_BYTES)?)?
            .checked_add(self.decision_count.checked_mul(CHECKPOINT_DECISION_ENTRY_BYTES)?)?
            .checked_add(self.observed_count.checked_mul(CHECKPOINT_OBSERVED_ENTRY_BYTES)?)
    }
}

#[derive(Clone, Copy, Debug)]
enum CheckpointPhase {
    Header,
    Domains,
    Trail,
    Decisions,
    Observed,
    Done,
}

#[derive(Clone, Debug)]
struct CheckpointBuild {
    bytes: Vec<u8>,
    byte_limit: usize,
    phase: CheckpointPhase,
    outer: usize,
    inner: usize,
    terminal: bool,
}

impl CheckpointBuild {
    fn new(state: &WfcState, pattern_count: usize, terminal: bool) -> Result<Self, JobFault> {
        let capacity = CheckpointCounts::from_state(state, pattern_count).checked_bytes().ok_or_else(empty_job_fault)?;
        if capacity > MAX_CHECKPOINT_BYTES {
            return Err(empty_job_fault());
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(capacity).map_err(|_| empty_job_fault())?;
        Ok(Self { bytes, byte_limit: capacity, phase: CheckpointPhase::Header, outer: 0, inner: 0, terminal })
    }
}

#[derive(Clone, Debug)]
struct CommitBuild {
    bytes: Vec<u8>,
    byte_limit: usize,
    assignment: Vec<u32>,
    item_limit: usize,
    cursor: usize,
    started: bool,
}

impl CommitBuild {
    fn new(node_count: usize) -> Result<Self, JobFault> {
        if node_count > MAX_COMMIT_ITEMS {
            return Err(empty_job_fault());
        }
        let capacity = node_count.checked_mul(COMMIT_ITEM_MAX_BYTES).and_then(|bytes| bytes.checked_add(COMMIT_FIXED_MAX_BYTES)).ok_or_else(empty_job_fault)?;
        if capacity > MAX_COMMIT_BYTES {
            return Err(empty_job_fault());
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(capacity).map_err(|_| empty_job_fault())?;
        let mut assignment = Vec::new();
        assignment.try_reserve_exact(node_count).map_err(|_| empty_job_fault())?;
        Ok(Self { bytes, byte_limit: capacity, assignment, item_limit: node_count, cursor: 0, started: false })
    }
}

fn ensure_materialization_space(bytes: &[u8], byte_limit: usize, additional: usize, detail: &'static [u8]) -> Result<(), JobFault> {
    if bytes.len().checked_add(additional).map_or(true, |length| length > byte_limit) {
        let _ = detail;
        return Err(empty_job_fault());
    }
    Ok(())
}

fn put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

//#endregion 💾️BoundedMaterialization

//#region 🧩️Job
pub(crate) struct WfcJob<T> {
    operation: Operation,
    model: CompiledModel,
    topology: T,
    config: WfcJobConfig,
    initial_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    state: WfcState,
    checkpoint_build: Option<CheckpointBuild>,
    final_checkpoint: Option<Vec<u8>>,
    commit_build: Option<CommitBuild>,
    completed_commit: Option<WfcCommit>,
    preview_units: u64,
    last_preview_ms: Option<u64>,
    closing: bool,
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
        Self { operation, model, topology, config, initial_domains, fixed, state, checkpoint_build: None, final_checkpoint: None, commit_build: None, completed_commit: None, preview_units: 0, last_preview_ms: None, closing: false }
    }

    pub fn from_checkpoint(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>, bytes: &[u8]) -> Result<Self, String>
    where
        T: Send + 'static,
    {
        let restore = WfcRestore::new(operation, model, topology, config, initial_domains, fixed, bytes.to_vec())?;
        let params = semio_framework_job::BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: semio_framework_job::BatchDriveConfig { site: "wfc.restore.batch", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: 64, step_budget_us: 4_000 },
            now_us: semio_framework_job::default_now_us,
        };
        let mut session = match semio_framework_job::BatchJobSession::try_new(restore, params) {
            Ok(session) => session,
            Err(mut rejected) => {
                rejected.begin_close();
                while !rejected.terminal_is_empty() {
                    let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                return Err("wfc-restore-batch-admission-rejected".into());
            }
        };
        loop {
            session.step().map_err(|error| format!("wfc-restore-batch-contention:{error:?}"))?;
            let Some(mut outcome) = session.take_outcome() else { continue };
            let terminal = outcome.is_terminal();
            let result = match &outcome {
                StepOutcome::Complete(_) => Some(session.checked_out_job_mut().and_then(WfcRestore::take_job).ok_or_else(|| "wfc-restore-completed-without-job".into())),
                StepOutcome::Cancelled => Some(Err("wfc-restore-cancelled".into())),
                StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&retained_payload_bytes(&fault.detail)).into_owned())),
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => None,
            };
            while !outcome.terminal_is_empty() {
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            if terminal {
                session.begin_close();
                while !session.terminal_is_empty() {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                return result.expect("terminal restore outcome has result");
            }
            session.resume().map_err(|error| format!("wfc-restore-batch-resume:{error:?}"))?;
        }
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

    #[cfg(test)]
    fn commit(&self) -> Option<WfcCommit> {
        (self.state.stage == WfcStage::Complete).then(|| WfcCommit {
            assignment: self.state.domains.iter().map(|domain| domain.first_set().expect("complete WFC domain").get()).collect(),
            observations: self.state.observations,
            compatibility_edges: self.state.compatibility_edges,
            backtracks: self.state.backtracks,
        })
    }

    #[cfg(test)]
    pub(crate) fn domain_masks(&self) -> Vec<PatternSet> {
        self.state.domains.clone()
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> (u64, u64, u64) {
        (self.state.observations, self.state.compatibility_edges, self.state.backtracks)
    }

    #[cfg(test)]
    pub(crate) fn observed(&self) -> &[(NodeId, PatternId)] {
        &self.state.observed
    }

    fn reset_queue(&mut self) {
        self.state.queue.clear();
        self.state.queue_epoch = self.state.queue_epoch.wrapping_add(1).max(1);
    }

    fn push_queue(&mut self, node: NodeId) {
        if self.state.queued_marks[node.index()] != self.state.queue_epoch {
            self.state.queued_marks[node.index()] = self.state.queue_epoch;
            self.state.queue.push_back(node);
        }
    }

    fn pop_queue(&mut self) -> Option<NodeId> {
        let node = self.state.queue.pop_front()?;
        self.state.queued_marks[node.index()] = 0;
        Some(node)
    }

    fn entropy(&self, node: NodeId) -> f64 {
        let index = node.index();
        let sum = self.state.domain_weight_sums[index];
        if sum <= 0.0 {
            0.0
        } else {
            sum.ln() - self.state.domain_weighted_log_sums[index] / sum
        }
    }

    fn push_entropy(&mut self, node: NodeId) {
        if self.state.domain_counts[node.index()] > 1 {
            self.state.entropy_heap.push(EntropyEntry { entropy_bits: self.entropy(node).to_bits(), node, revision: self.state.revisions[node.index()] });
        }
    }

    fn change_count(&mut self, node: NodeId, old: u32, new: u32) {
        self.state.singleton_count = self.state.singleton_count + usize::from(new == 1) - usize::from(old == 1);
        self.state.empty_count = self.state.empty_count + usize::from(new == 0) - usize::from(old == 0);
        self.state.domain_counts[node.index()] = new;
    }

    fn remove_pattern(&mut self, node: NodeId, pattern: PatternId, record: bool) {
        if !self.state.domains[node.index()].get(pattern) {
            return;
        }
        self.state.domains[node.index()].set(pattern, false);
        let old = self.state.domain_counts[node.index()];
        self.change_count(node, old, old - 1);
        let weight = self.model.weights().w(pattern);
        self.state.domain_weight_sums[node.index()] -= weight;
        self.state.domain_weighted_log_sums[node.index()] -= self.model.weights().w_ln_w(pattern);
        if record {
            self.state.trail.push(Removal { node, pattern });
        }
    }

    fn add_pattern(&mut self, node: NodeId, pattern: PatternId) {
        if self.state.domains[node.index()].get(pattern) {
            return;
        }
        self.state.domains[node.index()].set(pattern, true);
        let old = self.state.domain_counts[node.index()];
        self.change_count(node, old, old + 1);
        let weight = self.model.weights().w(pattern);
        self.state.domain_weight_sums[node.index()] += weight;
        self.state.domain_weighted_log_sums[node.index()] += self.model.weights().w_ln_w(pattern);
    }

    fn initialize_one(&mut self) {
        if self.state.init_node == self.topology.node_count() {
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
            return;
        }
        let node = NodeId::from_index(self.state.init_node);
        match self.state.init_cursor.phase {
            InitPhase::Acquire => {
                if let Some(initial) = self.initial_domains.as_mut() {
                    let domain = std::mem::replace(&mut initial[self.state.init_node], PatternSet::new_empty(0));
                    assert_eq!(domain.len(), self.model.pattern_count());
                    self.state.init_cursor.domain = Some(domain);
                    self.state.init_cursor.phase = InitPhase::Fixed;
                } else {
                    self.state.init_cursor.domain = Some(PatternSet::with_word_capacity(self.model.pattern_count()));
                    self.state.init_cursor.phase = InitPhase::BuildFull;
                }
            }
            InitPhase::BuildFull => {
                let cursor = &mut self.state.init_cursor;
                let word_count = self.model.pattern_count().div_ceil(64);
                if cursor.word < word_count {
                    let remaining = self.model.pattern_count().saturating_sub(cursor.word * 64);
                    let word = if remaining >= 64 { u64::MAX } else { (1u64 << remaining) - 1 };
                    cursor.domain.as_mut().expect("initial domain").push_word(word);
                    cursor.word += 1;
                } else {
                    cursor.phase = InitPhase::Fixed;
                }
            }
            InitPhase::Fixed => {
                let cursor = &mut self.state.init_cursor;
                if cursor.fixed < self.fixed.len() {
                    let (fixed_node, pattern) = self.fixed[cursor.fixed];
                    cursor.fixed += 1;
                    if fixed_node == node {
                        if cursor.required.is_some_and(|required| required != pattern) {
                            self.state.contradiction = Some(node);
                        }
                        cursor.required = Some(pattern);
                    }
                } else {
                    cursor.phase = InitPhase::Measure;
                }
            }
            InitPhase::Measure => {
                let cursor = &mut self.state.init_cursor;
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    let domain = cursor.domain.as_mut().expect("initial domain");
                    if domain.get(pattern) && cursor.required.is_some_and(|required| required != pattern) {
                        domain.set(pattern, false);
                    }
                    if domain.get(pattern) {
                        let weight = self.model.weights().w(pattern);
                        cursor.count += 1;
                        cursor.weight_sum += weight;
                        cursor.weighted_log_sum += self.model.weights().w_ln_w(pattern);
                    }
                } else {
                    let cursor = std::mem::take(&mut self.state.init_cursor);
                    self.state.domains.push(cursor.domain.expect("measured domain"));
                    self.state.domain_counts.push(cursor.count);
                    self.state.domain_weight_sums.push(cursor.weight_sum);
                    self.state.domain_weighted_log_sums.push(cursor.weighted_log_sum);
                    self.state.singleton_count += usize::from(cursor.count == 1);
                    self.state.empty_count += usize::from(cursor.count == 0);
                    self.state.revisions.push(0);
                    self.state.queued_marks.push(0);
                    if cursor.count == 0 {
                        self.state.contradiction = Some(node);
                    }
                    self.push_queue(node);
                    self.push_entropy(node);
                    self.state.init_node += 1;
                }
            }
        }
    }

    fn find_slot(&mut self) {
        if let Some(entry) = self.state.entropy_heap.pop() {
            if self.state.revisions[entry.node.index()] == entry.revision && self.state.domain_counts[entry.node.index()] > 1 {
                self.state.active_slot = Some(entry.node);
                self.state.stage = WfcStage::ChooseCandidate;
            }
            return;
        }
        self.state.stage = if self.state.singleton_count == self.state.domains.len() { WfcStage::Complete } else { WfcStage::DetectContradiction };
    }

    fn begin_choice(&mut self, node: NodeId) {
        let count = self.state.domain_counts[node.index()] as u64;
        let cursor = match self.config.sampler {
            WfcSampler::Uniform => ChoiceCursor { phase: ChoicePhase::Select, pattern: 0, total: 0.0, target: 0.0, running: 0.0, ordinal: self.state.rng.range(count), candidate: None },
            WfcSampler::WeightedRoulette => ChoiceCursor { phase: ChoicePhase::Weigh, pattern: 0, total: 0.0, target: 0.0, running: 0.0, ordinal: 0, candidate: None },
        };
        self.state.choice_cursor = Some(cursor);
    }

    fn choose_one(&mut self) {
        let node = self.state.active_slot.expect("choose stage has active slot");
        if self.state.choice_cursor.is_none() {
            self.begin_choice(node);
            return;
        }
        let cursor = self.state.choice_cursor.as_mut().expect("choice cursor");
        match cursor.phase {
            ChoicePhase::Weigh => {
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if self.state.domains[node.index()].get(pattern) {
                        cursor.total += self.model.weights().w(pattern);
                    }
                } else {
                    cursor.target = self.state.rng.next_f64() * cursor.total;
                    cursor.pattern = 0;
                    cursor.phase = ChoicePhase::Select;
                }
            }
            ChoicePhase::Select => {
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if self.state.domains[node.index()].get(pattern) {
                        let selected = match self.config.sampler {
                            WfcSampler::Uniform => {
                                let selected = cursor.ordinal == 0;
                                cursor.ordinal = cursor.ordinal.saturating_sub(1);
                                selected
                            }
                            WfcSampler::WeightedRoulette => {
                                cursor.running += self.model.weights().w(pattern);
                                cursor.running >= cursor.target
                            }
                        };
                        if selected {
                            cursor.candidate = Some(pattern);
                            cursor.pattern = 0;
                            cursor.phase = ChoicePhase::Remove;
                            self.state.decisions.push(Decision { node, candidate: pattern, trail_mark: self.state.trail.len(), rng_state: self.state.rng.state });
                            self.state.tested_tile = Some(pattern);
                            self.state.observations += 1;
                            self.state.observed.push((node, pattern));
                        }
                    }
                } else {
                    let fallback = self.state.domains[node.index()].first_set().expect("unresolved domain");
                    cursor.candidate = Some(fallback);
                    cursor.pattern = 0;
                    cursor.phase = ChoicePhase::Remove;
                    self.state.decisions.push(Decision { node, candidate: fallback, trail_mark: self.state.trail.len(), rng_state: self.state.rng.state });
                    self.state.tested_tile = Some(fallback);
                    self.state.observations += 1;
                    self.state.observed.push((node, fallback));
                }
            }
            ChoicePhase::Remove => {
                let candidate = cursor.candidate.expect("selected candidate");
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if pattern != candidate {
                        self.remove_pattern(node, pattern, true);
                    }
                } else {
                    self.state.revisions[node.index()] += 1;
                    self.reset_queue();
                    self.push_queue(node);
                    self.state.choice_cursor = None;
                    self.state.stage = WfcStage::PropagateCompatibilityEdge;
                }
            }
        }
    }

    fn acquire_source(&mut self) {
        let Some(source) = self.pop_queue() else {
            self.state.stage = WfcStage::DetectContradiction;
            return;
        };
        if self.state.propagation_wave.len() < PREVIEW_ITEM_LIMIT {
            self.state.propagation_wave.push(source);
        }
        self.state.arc_cursor = Some(ArcCursor::new(source, self.topology.out_arc_bound(source)));
    }

    fn propagate_one(&mut self) {
        if self.state.contradiction.is_some() {
            self.state.stage = WfcStage::DetectContradiction;
            return;
        }
        if self.state.arc_cursor.is_none() {
            self.acquire_source();
            return;
        }
        let cursor = self.state.arc_cursor.as_mut().expect("arc cursor");
        if cursor.target.is_none() {
            if cursor.raw_index == cursor.raw_bound {
                self.state.arc_cursor = None;
                return;
            }
            let raw = cursor.raw_index;
            cursor.raw_index += 1;
            if let Some((target, relation)) = self.topology.out_arc_at(cursor.source, raw) {
                cursor.target = Some(target);
                cursor.relation = Some(relation);
                cursor.allowed = Some(PatternSet::with_word_capacity(self.model.pattern_count()));
            }
            return;
        }

        let word_count = self.model.pattern_count().div_ceil(64);
        if cursor.allowed_word < word_count {
            if cursor.allowed_pattern < self.model.pattern_count() {
                let pattern = PatternId::from_index(cursor.allowed_pattern);
                cursor.allowed_pattern += 1;
                if self.state.domains[cursor.source.index()].get(pattern) {
                    cursor.allowed_accumulator |= self.model.allowed(cursor.relation.expect("relation"), pattern).word(cursor.allowed_word);
                }
            } else {
                cursor.allowed.as_mut().expect("allowed domain").push_word(cursor.allowed_accumulator);
                cursor.allowed_word += 1;
                cursor.allowed_pattern = 0;
                cursor.allowed_accumulator = 0;
            }
            return;
        }

        let target = cursor.target.expect("target");
        if cursor.removed_word != 0 {
            let bit = cursor.removed_word.trailing_zeros() as usize;
            cursor.removed_word &= cursor.removed_word - 1;
            let pattern = PatternId::from_index(cursor.removed_word_index * 64 + bit);
            let _ = cursor;
            self.remove_pattern(target, pattern, true);
            self.state.arc_cursor.as_mut().expect("arc cursor").changed = true;
            return;
        }
        if cursor.restrict_word < word_count {
            let word_index = cursor.restrict_word;
            cursor.restrict_word += 1;
            let old = self.state.domains[target.index()].word(word_index);
            let new = old & cursor.allowed.as_ref().expect("allowed domain").word(word_index);
            cursor.removed_word = old & !new;
            cursor.removed_word_index = word_index;
            return;
        }

        self.state.compatibility_edges += 1;
        if cursor.changed {
            self.state.revisions[target.index()] += 1;
            if self.state.changed_domains.len() < PREVIEW_ITEM_LIMIT && !self.state.changed_domains.contains(&target) {
                self.state.changed_domains.push(target);
            }
            if self.state.domain_counts[target.index()] == 0 {
                self.state.contradiction = Some(target);
            } else {
                self.push_queue(target);
                self.push_entropy(target);
            }
        }
        self.state.arc_cursor.as_mut().expect("arc cursor").clear_arc();
    }

    fn detect(&mut self) {
        if self.state.contradiction.is_some() || self.state.empty_count > 0 {
            self.state.backtrack_frame = self.state.decisions.pop();
            self.state.backtracks += 1;
            self.state.stage = WfcStage::BacktrackTrailEntry;
        } else if self.state.queue.is_empty() && self.state.arc_cursor.is_none() {
            self.state.stage = if self.state.singleton_count == self.state.domains.len() { WfcStage::Complete } else { WfcStage::CommitSlot };
        } else {
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
        }
    }

    fn backtrack_one(&mut self) {
        let Some(frame) = self.state.backtrack_frame else {
            self.state.stage = WfcStage::Complete;
            return;
        };
        if self.state.trail.len() > frame.trail_mark {
            let removed = self.state.trail.pop().expect("trail past frame mark");
            self.add_pattern(removed.node, removed.pattern);
            self.state.revisions[removed.node.index()] += 1;
            self.push_entropy(removed.node);
            if self.state.backtrack_path.len() < PREVIEW_ITEM_LIMIT && self.state.backtrack_path.last() != Some(&removed.node) {
                self.state.backtrack_path.push(removed.node);
            }
            return;
        }
        self.state.backtrack_frame = None;
        self.state.contradiction = None;
        self.state.arc_cursor = None;
        self.reset_queue();
        self.remove_pattern(frame.node, frame.candidate, true);
        self.state.revisions[frame.node.index()] += 1;
        if self.state.domain_counts[frame.node.index()] == 0 {
            self.state.contradiction = Some(frame.node);
            self.state.stage = WfcStage::DetectContradiction;
        } else {
            self.push_queue(frame.node);
            self.push_entropy(frame.node);
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
        }
    }

    fn reset_preview_delta(&mut self) {
        self.state.propagation_wave.clear();
        self.state.changed_domains.clear();
        self.state.backtrack_path.clear();
    }

    fn begin_checkpoint(&mut self, terminal: bool) -> Result<(), JobFault> {
        self.checkpoint_build = Some(CheckpointBuild::new(&self.state, self.model.pattern_count(), terminal)?);
        self.state.stage = WfcStage::MaterializeCheckpoint;
        Ok(())
    }

    fn checkpoint_one(&mut self) -> Result<Option<Vec<u8>>, JobFault> {
        let build = self.checkpoint_build.as_mut().expect("checkpoint build");
        match build.phase {
            CheckpointPhase::Header => {
                ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_FIXED_HEADER_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                build.bytes.extend_from_slice(CHECKPOINT_MAGIC);
                let fields: [u64; CHECKPOINT_HEADER_U64_FIELDS] = [
                    self.state.operation_id,
                    self.state.base_revision,
                    self.state.generation,
                    self.state.operation_seed,
                    self.state.preview_sequence,
                    self.state.topology_nodes as u64,
                    self.state.model_fingerprint,
                    u64::from(build.terminal),
                    self.state.rng.state[0],
                    self.state.rng.state[1],
                    self.state.rng.state[2],
                    self.state.rng.state[3],
                    self.state.observations,
                    self.state.compatibility_edges,
                    self.state.backtracks,
                    self.state.domains.len() as u64,
                    self.model.pattern_count() as u64,
                    self.state.trail.len() as u64,
                    self.state.decisions.len() as u64,
                    self.state.observed.len() as u64,
                ];
                for field in fields {
                    put_u64(&mut build.bytes, field);
                }
                build.phase = CheckpointPhase::Domains;
            }
            CheckpointPhase::Domains => {
                if build.outer < self.state.domains.len() {
                    let domain = &self.state.domains[build.outer];
                    if build.inner < domain.word_count() {
                        ensure_materialization_space(&build.bytes, build.byte_limit, std::mem::size_of::<u64>(), b"wfc-checkpoint-byte-limit-exceeded")?;
                        put_u64(&mut build.bytes, domain.word(build.inner));
                        build.inner += 1;
                    } else {
                        build.outer += 1;
                        build.inner = 0;
                    }
                } else {
                    build.phase = CheckpointPhase::Trail;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Trail => {
                if let Some(removal) = self.state.trail.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_TRAIL_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, removal.node.get());
                    put_u32(&mut build.bytes, removal.pattern.get());
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Decisions;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Decisions => {
                if let Some(decision) = self.state.decisions.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_DECISION_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, decision.node.get());
                    put_u32(&mut build.bytes, decision.candidate.get());
                    put_u64(&mut build.bytes, decision.trail_mark as u64);
                    for word in decision.rng_state {
                        put_u64(&mut build.bytes, word);
                    }
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Observed;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Observed => {
                if let Some(&(node, pattern)) = self.state.observed.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_OBSERVED_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, node.get());
                    put_u32(&mut build.bytes, pattern.get());
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Done;
                }
            }
            CheckpointPhase::Done => {
                return Ok(Some(std::mem::take(&mut build.bytes)));
            }
        }
        Ok(None)
    }

    fn commit_one(&mut self) -> Result<Option<Vec<u8>>, JobFault> {
        let build = self.commit_build.as_mut().expect("commit build");
        if !build.started {
            ensure_materialization_space(&build.bytes, build.byte_limit, br#"{"assignment":["#.len(), b"wfc-commit-byte-limit-exceeded")?;
            build.bytes.extend_from_slice(br#"{"assignment":["#);
            build.started = true;
            return Ok(None);
        }
        if build.cursor < self.state.domains.len() {
            if build.assignment.len() >= build.item_limit {
                return Err(empty_job_fault());
            }
            let value = self.state.domains[build.cursor].first_set().expect("complete domain").get();
            let encoded = value.to_string();
            let additional = usize::from(build.cursor != 0).checked_add(encoded.len()).ok_or_else(empty_job_fault)?;
            ensure_materialization_space(&build.bytes, build.byte_limit, additional, b"wfc-commit-byte-limit-exceeded")?;
            if build.cursor != 0 {
                build.bytes.push(b',');
            }
            build.assignment.push(value);
            build.bytes.extend_from_slice(encoded.as_bytes());
            build.cursor += 1;
            return Ok(None);
        }
        if build.assignment.len() != build.item_limit {
            return Err(empty_job_fault());
        }
        let tail = format!("],\"observations\":{},\"compatibility_edges\":{},\"backtracks\":{}}}", self.state.observations, self.state.compatibility_edges, self.state.backtracks);
        ensure_materialization_space(&build.bytes, build.byte_limit, tail.len(), b"wfc-commit-byte-limit-exceeded")?;
        build.bytes.extend_from_slice(tail.as_bytes());
        self.completed_commit = Some(WfcCommit { assignment: std::mem::take(&mut build.assignment), observations: self.state.observations, compatibility_edges: self.state.compatibility_edges, backtracks: self.state.backtracks });
        Ok(Some(std::mem::take(&mut build.bytes)))
    }

    pub(crate) fn take_completed_commit(&mut self) -> Option<WfcCommit> {
        self.completed_commit.take()
    }

    fn preview_stage(&self) -> bool {
        matches!(self.state.stage, WfcStage::InitializeDomains | WfcStage::FindMinimumEntropySlot | WfcStage::ChooseCandidate | WfcStage::PropagateCompatibilityEdge | WfcStage::DetectContradiction | WfcStage::BacktrackTrailEntry)
    }

    fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PREVIEW_TIME_INTERVAL_MS)
    }

    fn emit_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let Some(now_ms) = context.now_us().map(|now_us| now_us / 1_000) else { return StepOutcome::Yield };
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence.max(self.state.preview_sequence),
            Err(_) => return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        };
        self.state.preview_sequence = sequence + 1;
        self.operation.preview_sequence = self.state.preview_sequence;
        let preview = self.preview(sequence);
        self.reset_preview_delta();
        self.preview_units = 0;
        self.last_preview_ms = Some(now_ms);
        let bytes = protocol::json::to_json_string(&preview).into_bytes();
        StepOutcome::PreviewReady(retained_payload(context, semio_framework_job::JobPayloadStream::Preview, &bytes))
    }
}

//#region 🔄️RestoreJob
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue)]
enum RestoreStage {
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
struct RestoreHeader {
    operation_id: u64,
    base_revision: u64,
    generation: u64,
    operation_seed: u64,
    preview_sequence: u64,
    topology_nodes: usize,
    model_fingerprint: u64,
    terminal: bool,
    rng_state: [u64; 4],
    observations: u64,
    compatibility_edges: u64,
    backtracks: u64,
    domain_count: usize,
    pattern_count: usize,
    trail_count: usize,
    decision_count: usize,
    observed_count: usize,
}

#[derive(semio_framework_value_derive::ToValue)]
struct RestorePreview {
    sequence: u64,
    stage: RestoreStage,
    completed: usize,
    total: usize,
}

/// 🔄️ Fuel-bounded checkpoint decoding and cache/heap reconstruction.
pub(crate) struct WfcRestore<T> {
    operation: Operation,
    model: Option<CompiledModel>,
    topology: Option<T>,
    config: WfcJobConfig,
    initial_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    bytes: Vec<u8>,
    cursor: usize,
    stage: RestoreStage,
    header: Option<RestoreHeader>,
    domains: Vec<PatternSet>,
    domain_words: Vec<u64>,
    trail: Vec<Removal>,
    decisions: Vec<Decision>,
    observed: Vec<(NodeId, PatternId)>,
    domain_cursor: usize,
    pattern_cursor: usize,
    count: u32,
    weight_sum: f64,
    weighted_log_sum: f64,
    domain_counts: Vec<u32>,
    weight_sums: Vec<f64>,
    weighted_log_sums: Vec<f64>,
    singleton_count: usize,
    empty_count: usize,
    revisions: Vec<u64>,
    queued_marks: Vec<u64>,
    entropy_heap: BinaryHeap<EntropyEntry>,
    restored: Option<WfcJob<T>>,
    preview_units: u64,
    last_preview_ms: Option<u64>,
    closing: bool,
}

impl<T: Topology + Clone> WfcRestore<T> {
    pub(crate) fn new(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>, bytes: Vec<u8>) -> Result<Self, String> {
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
            closing: false,
        })
    }

    fn take(&mut self, count: usize) -> Result<&[u8], String> {
        let end = self.cursor.checked_add(count).ok_or("wfc-checkpoint-overflow")?;
        let value = self.bytes.get(self.cursor..end).ok_or("wfc-checkpoint-truncated")?;
        self.cursor = end;
        Ok(value)
    }

    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(std::mem::size_of::<u32>())?.try_into().expect("u32 bytes")))
    }

    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(std::mem::size_of::<u64>())?.try_into().expect("u64 bytes")))
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

    fn decode_one(&mut self) -> Result<(), String> {
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
            RestoreStage::Rebuild => self.rebuild_one()?,
            RestoreStage::Complete => self.finish()?,
        }
        Ok(())
    }

    fn rebuild_one(&mut self) -> Result<(), String> {
        let header = self.header.as_ref().expect("restore header");
        if self.domain_cursor == header.domain_count {
            self.stage = RestoreStage::Complete;
            return Ok(());
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
            return Ok(());
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
        Ok(())
    }

    fn finish(&mut self) -> Result<(), String> {
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
            closing: false,
        });
        Ok(())
    }

    pub(crate) fn take_job(&mut self) -> Option<WfcJob<T>> {
        self.restored.take()
    }

    fn progress(&self) -> (usize, usize) {
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

impl<T: Topology + Clone + Send> InteractiveJob for WfcRestore<T> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-wfc-restore-operation") });
        }
        loop {
            context.set_stage("wfc.restore");
            if let Err(error) = self.decode_one() {
                return StepOutcome::Fault(JobFault { detail: retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes()) });
            }
            if self.stage == RestoreStage::Complete && self.restored.is_some() {
                return StepOutcome::Complete(CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            let Some(now_ms) = context.now_us().map(|now_us| now_us / 1_000) else { return StepOutcome::Yield };
            if self.last_preview_ms.is_none() || self.preview_units >= PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PREVIEW_TIME_INTERVAL_MS) {
                let (completed, total) = self.progress();
                let sequence = match context.next_preview_sequence() {
                    Ok(sequence) => sequence,
                    Err(_) => return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
                };
                let preview = RestorePreview { sequence, stage: self.stage, completed, total };
                self.preview_units = 0;
                self.last_preview_ms = Some(now_ms);
                let bytes = protocol::json::to_json_string(&preview).into_bytes();
                return StepOutcome::PreviewReady(retained_payload(context, semio_framework_job::JobPayloadStream::Preview, &bytes));
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(restored) = self.restored.as_mut() {
            restored.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(restored) = self.restored.as_mut() {
            let step = restored.close_step(maximum_items, maximum_bytes);
            if restored.terminal_is_empty() {
                self.restored = None;
            }
            return step;
        }
        if !self.bytes.is_empty() {
            if maximum_bytes == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.bytes.pop();
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 };
        }
        macro_rules! pop_owner {
            ($owners:expr) => {
                if $owners.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            };
        }
        pop_owner!(self.fixed);
        if let Some(domains) = self.initial_domains.as_mut() {
            pop_owner!(domains);
            self.initial_domains = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        pop_owner!(self.domains);
        pop_owner!(self.domain_words);
        pop_owner!(self.trail);
        pop_owner!(self.decisions);
        pop_owner!(self.observed);
        pop_owner!(self.domain_counts);
        pop_owner!(self.weight_sums);
        pop_owner!(self.weighted_log_sums);
        pop_owner!(self.revisions);
        pop_owner!(self.queued_marks);
        if self.entropy_heap.pop().is_some() || self.header.take().is_some() || self.model.take().is_some() || self.topology.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.model.is_none()
            && self.topology.is_none()
            && self.initial_domains.is_none()
            && self.fixed.is_empty()
            && self.bytes.is_empty()
            && self.header.is_none()
            && self.domains.is_empty()
            && self.domain_words.is_empty()
            && self.trail.is_empty()
            && self.decisions.is_empty()
            && self.observed.is_empty()
            && self.domain_counts.is_empty()
            && self.weight_sums.is_empty()
            && self.weighted_log_sums.is_empty()
            && self.revisions.is_empty()
            && self.queued_marks.is_empty()
            && self.entropy_heap.is_empty()
            && self.restored.is_none()
    }
}
//#endregion 🔄️RestoreJob

impl<T: Topology + Clone + Send> InteractiveJob for WfcJob<T> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-wfc-operation") });
        }
        loop {
            context.set_stage(self.state.stage.label());
            match self.state.stage {
                WfcStage::InitializeDomains => self.initialize_one(),
                WfcStage::FindMinimumEntropySlot => self.find_slot(),
                WfcStage::ChooseCandidate => self.choose_one(),
                WfcStage::PropagateCompatibilityEdge => self.propagate_one(),
                WfcStage::DetectContradiction => self.detect(),
                WfcStage::BacktrackTrailEntry => self.backtrack_one(),
                WfcStage::CommitSlot => {
                    if self.state.observations == 1 || self.state.observations % CHECKPOINT_INTERVAL == 0 {
                        if let Err(fault) = self.begin_checkpoint(false) {
                            return StepOutcome::Fault(fault);
                        }
                    } else {
                        self.state.stage = WfcStage::FindMinimumEntropySlot;
                    }
                    return self.emit_preview(context);
                }
                WfcStage::MaterializeCheckpoint => {
                    let checkpoint = match self.checkpoint_one() {
                        Ok(checkpoint) => checkpoint,
                        Err(fault) => return StepOutcome::Fault(fault),
                    };
                    if let Some(bytes) = checkpoint {
                        let terminal = self.checkpoint_build.as_ref().expect("checkpoint build").terminal;
                        self.checkpoint_build = None;
                        if terminal {
                            self.final_checkpoint = Some(bytes);
                            match CommitBuild::new(self.state.domains.len()) {
                                Ok(build) => self.commit_build = Some(build),
                                Err(fault) => return StepOutcome::Fault(fault),
                            }
                            self.state.stage = WfcStage::MaterializeCommit;
                        } else {
                            self.state.stage = WfcStage::FindMinimumEntropySlot;
                            let state = retained_payload(context, semio_framework_job::JobPayloadStream::CheckpointState, &bytes);
                            return StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.state.observations });
                        }
                    }
                }
                WfcStage::MaterializeCommit => {
                    let commit = match self.commit_one() {
                        Ok(commit) => commit,
                        Err(fault) => return StepOutcome::Fault(fault),
                    };
                    if let Some(output) = commit {
                        self.state.stage = WfcStage::Complete;
                        let state = retained_payload(context, semio_framework_job::JobPayloadStream::CommitState, &self.final_checkpoint.take().expect("final checkpoint"));
                        let output = retained_payload(context, semio_framework_job::JobPayloadStream::CommitOutput, &output);
                        return StepOutcome::Complete(CommitCandidate { state, output });
                    }
                }
                WfcStage::Complete => {
                    if self.state.contradiction.is_some() || self.state.empty_count > 0 {
                        return StepOutcome::Fault(JobFault { detail: retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"wfc-unsatisfiable") });
                    }
                    if let Err(fault) = self.begin_checkpoint(true) {
                        return StepOutcome::Fault(fault);
                    }
                }
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if self.preview_stage() && context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.emit_preview(context);
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        macro_rules! pop_owner {
            ($owners:expr) => {
                if $owners.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            };
        }
        if let Some(domains) = self.initial_domains.as_mut() {
            pop_owner!(domains);
            self.initial_domains = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        pop_owner!(self.fixed);
        pop_owner!(self.state.domains);
        pop_owner!(self.state.domain_counts);
        pop_owner!(self.state.domain_weight_sums);
        pop_owner!(self.state.domain_weighted_log_sums);
        pop_owner!(self.state.revisions);
        if self.state.queue.pop_front().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        pop_owner!(self.state.queued_marks);
        pop_owner!(self.state.trail);
        pop_owner!(self.state.decisions);
        pop_owner!(self.state.propagation_wave);
        pop_owner!(self.state.changed_domains);
        pop_owner!(self.state.backtrack_path);
        pop_owner!(self.state.observed);
        if self.state.entropy_heap.pop().is_some() || self.checkpoint_build.take().is_some() || self.commit_build.take().is_some() || self.completed_commit.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(bytes) = self.final_checkpoint.as_mut() {
            if !bytes.is_empty() {
                if maximum_bytes == 0 {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                bytes.pop();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 };
            }
            self.final_checkpoint = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.initial_domains.is_none()
            && self.fixed.is_empty()
            && self.state.domains.is_empty()
            && self.state.domain_counts.is_empty()
            && self.state.domain_weight_sums.is_empty()
            && self.state.domain_weighted_log_sums.is_empty()
            && self.state.revisions.is_empty()
            && self.state.queue.is_empty()
            && self.state.queued_marks.is_empty()
            && self.state.trail.is_empty()
            && self.state.decisions.is_empty()
            && self.state.entropy_heap.is_empty()
            && self.state.propagation_wave.is_empty()
            && self.state.changed_domains.is_empty()
            && self.state.backtrack_path.is_empty()
            && self.state.observed.is_empty()
            && self.checkpoint_build.is_none()
            && self.final_checkpoint.is_none()
            && self.commit_build.is_none()
            && self.completed_commit.is_none()
    }
}
//#endregion 🧩️Job

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use semio_framework_job::{allocate_operation_id, root_cancel_token, Generation, RevisionId, StepBudget};

    use super::*;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::topology::{GraphTopology, GraphTopologyBuilder};

    fn checkerboard(nodes: usize, seed: u64) -> WfcJob<GraphTopology> {
        let mut model = ModelBuilder::new();
        let black = model.add_pattern(1.0);
        let white = model.add_pattern(2.0);
        let adjacent = model.add_relation("adjacent");
        model.allow_mirrored(adjacent, black, white);
        let model = model.compile().expect("model");
        let mut topology = GraphTopologyBuilder::new(nodes);
        for node in 0..nodes.saturating_sub(1) {
            topology.arc(NodeId::from_index(node), NodeId::from_index(node + 1), adjacent);
            topology.arc(NodeId::from_index(node + 1), NodeId::from_index(node), adjacent);
        }
        let operation = Operation::new(allocate_operation_id(), RevisionId(3), Generation(5), seed);
        WfcJob::new(operation, model, topology.build().expect("topology"), WfcJobConfig::default(), None, Vec::new())
    }

    fn drive(job: &mut WfcJob<GraphTopology>, fuel: u64) -> StepOutcome {
        let mut sequence = job.operation.preview_sequence;
        for _ in 0..2_000_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(fuel, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            let outcome = job.step(&mut context);
            if outcome.is_terminal() {
                return outcome;
            }
        }
        panic!("WFC job did not terminate");
    }

    fn checkpoint(job: &mut WfcJob<GraphTopology>) -> Vec<u8> {
        let mut sequence = job.operation.preview_sequence;
        for _ in 0..2_000_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            match job.step(&mut context) {
                StepOutcome::CheckpointReady(checkpoint) => return checkpoint.state,
                outcome if outcome.is_terminal() => panic!("job terminated before checkpoint"),
                _ => {}
            }
        }
        panic!("WFC job did not checkpoint");
    }

    fn terminal_checkpoint(job: &mut WfcJob<GraphTopology>) -> Vec<u8> {
        let mut sequence = job.operation.preview_sequence;
        for _ in 0..2_000_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            if let StepOutcome::Complete(candidate) = job.step(&mut context) {
                return candidate.state;
            }
        }
        panic!("WFC job did not complete");
    }

    fn prepare_maximum_checkpoint_state(job: &mut WfcJob<GraphTopology>) -> usize {
        let _ = terminal_checkpoint(job);
        job.state.trail.clear();
        job.state.decisions.clear();
        job.state.observed.clear();
        job.state.observations = 0;
        let base_bytes = CheckpointCounts::from_state(&job.state, job.model.pattern_count()).checked_bytes().expect("base checkpoint size");
        let remaining = MAX_CHECKPOINT_BYTES.checked_sub(base_bytes).expect("maximum admits base checkpoint");
        assert_eq!(remaining % CHECKPOINT_OBSERVED_ENTRY_BYTES, 0);
        let observed_count = remaining / CHECKPOINT_OBSERVED_ENTRY_BYTES;
        job.state.observations = u64::try_from(observed_count).expect("observation count");
        job.state.observed = vec![(NodeId(0), PatternId(0)); observed_count];
        assert_eq!(CheckpointCounts::from_state(&job.state, job.model.pattern_count()).checked_bytes(), Some(MAX_CHECKPOINT_BYTES));
        observed_count
    }

    fn maximum_checkpoint(job: &mut WfcJob<GraphTopology>) -> Vec<u8> {
        prepare_maximum_checkpoint_state(job);
        job.begin_checkpoint(true).expect("maximum checkpoint build");
        assert_eq!(job.checkpoint_build.as_ref().expect("checkpoint build").byte_limit, MAX_CHECKPOINT_BYTES);
        for _ in 0..2_000_000 {
            if let Some(bytes) = job.checkpoint_one().expect("maximum checkpoint unit") {
                assert_eq!(bytes.len(), MAX_CHECKPOINT_BYTES);
                return bytes;
            }
        }
        panic!("maximum WFC checkpoint did not materialize");
    }

    #[test]
    fn batch_size_and_replay_are_deterministic() {
        let mut one = checkerboard(127, 19);
        let mut many = checkerboard(127, 19);
        assert!(matches!(drive(&mut one, 1), StepOutcome::Complete(_)));
        assert!(matches!(drive(&mut many, 64), StepOutcome::Complete(_)));
        assert_eq!(one.commit(), many.commit());
    }

    #[test]
    fn checkpoint_resume_preserves_rng_trail_and_progress() {
        let mut original = checkerboard(41, 71);
        let bytes = checkpoint(&mut original);
        let mut restored = WfcJob::from_checkpoint(original.operation, original.model.clone(), original.topology.clone(), original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes).expect("restore");
        assert!(matches!(drive(&mut original, 3), StepOutcome::Complete(_)));
        assert!(matches!(drive(&mut restored, 11), StepOutcome::Complete(_)));
        assert_eq!(original.commit(), restored.commit());
    }

    #[test]
    fn checkpoint_restore_rejects_foreign_operation_and_topology() {
        let mut original = checkerboard(11, 71);
        let bytes = checkpoint(&mut original);
        let foreign_operation = Operation::new(allocate_operation_id(), original.operation.base_revision, original.operation.generation, original.operation.seed);
        assert!(WfcJob::from_checkpoint(foreign_operation, original.model.clone(), original.topology.clone(), original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes).is_err());
        let foreign_topology = checkerboard(12, 71).topology;
        assert!(WfcJob::from_checkpoint(original.operation, original.model.clone(), foreign_topology, original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes).is_err());
    }

    #[test]
    fn previews_report_monotonic_sequences_and_progress() {
        let mut job = checkerboard(31, 83);
        let mut sequence = 0;
        let mut previews = Vec::new();
        loop {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            match job.step(&mut context) {
                StepOutcome::PreviewReady(bytes) => previews.push(serde_json::from_slice::<WfcPreview>(&bytes).expect("preview")),
                outcome if outcome.is_terminal() => break,
                _ => {}
            }
        }
        assert!(!previews.is_empty());
        assert!(previews.windows(2).all(|pair| pair[1].sequence == pair[0].sequence + 1 && pair[1].observations >= pair[0].observations && pair[1].compatibility_edges >= pair[0].compatibility_edges && pair[1].backtracks >= pair[0].backtracks));
    }

    #[test]
    fn every_interactive_solver_stage_is_preview_eligible_at_fixed_cadence() {
        let mut job = checkerboard(17, 83);
        for stage in [WfcStage::InitializeDomains, WfcStage::FindMinimumEntropySlot, WfcStage::ChooseCandidate, WfcStage::PropagateCompatibilityEdge, WfcStage::DetectContradiction, WfcStage::BacktrackTrailEntry] {
            job.state.stage = stage;
            job.preview_units = PREVIEW_UNIT_INTERVAL;
            job.last_preview_ms = Some(0);
            assert!(job.preview_stage());
            assert!(job.preview_due(0));
        }
    }

    #[test]
    fn first_preview_is_immediate_and_continuous_gap_is_bounded() {
        let mut job = checkerboard(4_096, 83);
        let mut sequence = 0;
        let mut units_since_preview = 0;
        let mut preview_count = 0;
        for _ in 0..100_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            units_since_preview += 1;
            match job.step(&mut context) {
                StepOutcome::PreviewReady(_) => {
                    assert!(units_since_preview <= PREVIEW_UNIT_INTERVAL as usize);
                    if preview_count == 0 {
                        assert_eq!(units_since_preview, 1);
                    }
                    units_since_preview = 0;
                    preview_count += 1;
                    if preview_count == 64 {
                        break;
                    }
                }
                outcome if outcome.is_terminal() => break,
                _ => {}
            }
        }
        assert_eq!(preview_count, 64);
    }

    #[test]
    fn uniform_sampling_consumes_exactly_one_rng_word() {
        let mut ranged = JobRng::from_seed(0x5eed);
        let mut direct = ranged;
        let value = ranged.range(u64::MAX - 58);
        let _ = direct.next_u64();
        assert!(value < u64::MAX - 58);
        assert_eq!(ranged.state, direct.state);
    }

    #[test]
    fn checkpoint_resume_preserves_preview_sequence() {
        let mut job = checkerboard(31, 89);
        job.topology = GraphTopologyBuilder::new(31).build().expect("disjoint topology");
        let bytes = checkpoint(&mut job);
        let previous = job.operation.preview_sequence;
        let mut restored = WfcJob::from_checkpoint(job.operation, job.model.clone(), job.topology.clone(), job.config, job.initial_domains.clone(), job.fixed.clone(), &bytes).expect("restore");
        let mut sequence = 0;
        let resumed = loop {
            let mut context = StepContext::new(restored.operation.operation, restored.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            if let StepOutcome::PreviewReady(bytes) = restored.step(&mut context) {
                break serde_json::from_slice::<WfcPreview>(&bytes).expect("preview");
            }
        };
        assert_eq!(resumed.sequence + 1, previous + 1);
    }

    #[test]
    fn disjoint_and_adversarial_graphs_finish() {
        let mut disjoint = checkerboard(0, 1);
        assert!(matches!(drive(&mut disjoint, 1), StepOutcome::Complete(_)));
        let mut long = checkerboard(4_096, 2);
        assert!(matches!(drive(&mut long, 32), StepOutcome::Complete(_)));
        assert_eq!(long.commit().expect("commit").assignment.len(), 4_096);
    }

    #[test]
    fn cancellation_and_generation_freshness_do_not_mutate_progress() {
        let mut job = checkerboard(10, 3);
        let mut sequence = 0;
        let before = job.metrics();
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut cancelled = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(10, u64::MAX), cancel, || Some(0), &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.metrics(), before);
        let mut stale = StepContext::new(job.operation.operation, Generation(job.operation.generation.0 + 1), StepBudget::new(10, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!(job.metrics(), before);
    }

    #[test]
    fn cancellation_interrupts_checkpoint_and_commit_materialization_without_progress() {
        let mut job = checkerboard(31, 97);
        let mut sequence = 0;
        let mut checked_checkpoint = false;
        let mut checked_commit = false;
        for _ in 0..2_000_000 {
            let stage = job.state.stage;
            if matches!(stage, WfcStage::MaterializeCheckpoint | WfcStage::MaterializeCommit) {
                let before = match stage {
                    WfcStage::MaterializeCheckpoint => job.checkpoint_build.as_ref().map(|build| build.bytes.len()).unwrap_or(0),
                    WfcStage::MaterializeCommit => job.commit_build.as_ref().map(|build| build.cursor).unwrap_or(0),
                    _ => unreachable!(),
                };
                let cancel = root_cancel_token();
                cancel.cancel_now();
                let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), cancel, || Some(0), &mut sequence);
                assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
                let after = match stage {
                    WfcStage::MaterializeCheckpoint => job.checkpoint_build.as_ref().map(|build| build.bytes.len()).unwrap_or(0),
                    WfcStage::MaterializeCommit => job.commit_build.as_ref().map(|build| build.cursor).unwrap_or(0),
                    _ => unreachable!(),
                };
                assert_eq!(before, after);
                checked_checkpoint |= stage == WfcStage::MaterializeCheckpoint;
                checked_commit |= stage == WfcStage::MaterializeCommit;
            }
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            if job.step(&mut context).is_terminal() {
                break;
            }
        }
        assert!(checked_checkpoint && checked_commit);
    }

    #[test]
    fn maximum_checkpoint_restore_is_bounded_and_cancellable_in_every_phase() {
        let mut source = checkerboard(1, 101);
        let bytes = maximum_checkpoint(&mut source);
        let mut restore = WfcRestore::new(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), bytes).expect("maximum admitted restore");
        assert!(restore.domains.is_empty());
        assert!(restore.domain_counts.is_empty());
        assert!(restore.entropy_heap.is_empty());
        let mut cancelled = Vec::new();
        let mut sequence = 0;
        for _ in 0..2_000_000 {
            let stage = restore.stage;
            if !cancelled.contains(&stage) {
                let before = (restore.cursor, restore.domains.len(), restore.trail.len(), restore.decisions.len(), restore.observed.len(), restore.domain_cursor, restore.restored.is_some());
                let token = root_cancel_token();
                token.cancel_now();
                let mut context = StepContext::new(source.operation.operation, source.operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
                assert_eq!(restore.step(&mut context), StepOutcome::Cancelled);
                let after = (restore.cursor, restore.domains.len(), restore.trail.len(), restore.decisions.len(), restore.observed.len(), restore.domain_cursor, restore.restored.is_some());
                assert_eq!(before, after);
                cancelled.push(stage);
            }
            let mut context = StepContext::new(source.operation.operation, source.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            if matches!(restore.step(&mut context), StepOutcome::Complete(_)) {
                break;
            }
        }
        assert_eq!(cancelled, vec![RestoreStage::Header, RestoreStage::Domains, RestoreStage::Trail, RestoreStage::Decisions, RestoreStage::Observed, RestoreStage::Verify, RestoreStage::Rebuild, RestoreStage::Complete]);
        assert!(restore.take_job().is_some());
    }

    #[test]
    fn checkpoint_admission_rejects_one_byte_over_the_fixed_maximum() {
        let source = checkerboard(1, 103);
        let bytes = vec![0; MAX_CHECKPOINT_BYTES.checked_add(1).expect("maximum plus one")];
        assert!(matches!(WfcRestore::new(source.operation, source.model, source.topology, source.config, None, Vec::new(), bytes), Err(error) if error == "wfc-checkpoint-admission-exceeded"));
    }

    #[test]
    fn minimum_checkpoint_is_exactly_the_fixed_header_and_restores() {
        let mut source = checkerboard(0, 105);
        let bytes = terminal_checkpoint(&mut source);
        assert_eq!(bytes.len(), CHECKPOINT_FIXED_HEADER_BYTES);
        assert_eq!(CheckpointCounts::from_state(&source.state, source.model.pattern_count()).checked_bytes(), Some(CHECKPOINT_FIXED_HEADER_BYTES));
        let restored = WfcJob::from_checkpoint(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), &bytes).expect("minimum checkpoint restore");
        assert!(restored.state.domains.is_empty());
    }

    #[test]
    fn checkpoint_restore_rejects_size_arithmetic_overflow() {
        let mut source = checkerboard(0, 106);
        let mut bytes = terminal_checkpoint(&mut source);
        let observed_count_offset = CHECKPOINT_FIXED_HEADER_BYTES.checked_sub(std::mem::size_of::<u64>()).expect("observed count offset");
        bytes[observed_count_offset..CHECKPOINT_FIXED_HEADER_BYTES].copy_from_slice(&u64::MAX.to_le_bytes());
        let mut restore = WfcRestore::new(source.operation, source.model, source.topology, source.config, None, Vec::new(), bytes).expect("admitted overflow fixture");
        let mut sequence = 0;
        let mut context = StepContext::new(source.operation.operation, source.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(restore.step(&mut context), StepOutcome::Fault(JobFault { detail }) if detail == b"wfc-checkpoint-capacity"));
        assert_eq!(CheckpointCounts { domain_count: usize::MAX, pattern_count: usize::MAX, trail_count: usize::MAX, decision_count: usize::MAX, observed_count: usize::MAX }.checked_bytes(), None);
    }

    #[test]
    fn maximum_admitted_checkpoint_and_commit_allocation_stay_below_watchdog() {
        let mut source = checkerboard(1, 107);
        prepare_maximum_checkpoint_state(&mut source);
        let pressure = vec![vec![0u8; 4_096]; 64];
        let start = Instant::now();
        let checkpoint = CheckpointBuild::new(&source.state, source.model.pattern_count(), true).expect("maximum checkpoint allocation");
        let checkpoint_elapsed = start.elapsed();
        let start = Instant::now();
        let mut commit = CommitBuild::new(MAX_COMMIT_ITEMS).expect("maximum commit allocation");
        let commit_elapsed = start.elapsed();
        assert!(checkpoint.bytes.capacity() >= MAX_CHECKPOINT_BYTES);
        assert_eq!(checkpoint.byte_limit, MAX_CHECKPOINT_BYTES);
        assert!(commit.bytes.capacity() >= COMMIT_FIXED_MAX_BYTES + MAX_COMMIT_ITEMS * COMMIT_ITEM_MAX_BYTES);
        assert!(commit.byte_limit <= MAX_COMMIT_BYTES);
        assert!(commit.assignment.capacity() >= MAX_COMMIT_ITEMS);
        let assignment_capacity = commit.assignment.capacity();
        for _ in 0..MAX_COMMIT_ITEMS {
            commit.assignment.push(0);
        }
        assert_eq!(commit.assignment.len(), MAX_COMMIT_ITEMS);
        assert_eq!(commit.assignment.capacity(), assignment_capacity, "the exact-maximum lossless side vector must never grow while materializing");
        assert!(matches!(CommitBuild::new(MAX_COMMIT_ITEMS + 1), Err(JobFault { detail }) if detail == b"wfc-commit-admission-exceeded"));
        source.state.observed.push((NodeId(0), PatternId(0)));
        assert!(matches!(CheckpointBuild::new(&source.state, source.model.pattern_count(), true), Err(JobFault { detail }) if detail == b"wfc-checkpoint-admission-exceeded"));
        assert!(checkpoint_elapsed < Duration::from_millis(8), "maximum checkpoint allocation exceeded watchdog: {checkpoint_elapsed:?}");
        assert!(commit_elapsed < Duration::from_millis(8), "maximum commit allocation exceeded watchdog: {commit_elapsed:?}");
        assert_eq!(pressure.len(), 64);
    }

    #[test]
    fn every_large_domain_unit_including_checkpoint_stays_below_watchdog() {
        let mut job = checkerboard(8_192, 4);
        let mut sequence = 0;
        let mut samples = Vec::new();
        let mut saw_checkpoint = false;
        for _ in 0..500_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            let start = Instant::now();
            let outcome = job.step(&mut context);
            samples.push(start.elapsed());
            saw_checkpoint |= matches!(outcome, StepOutcome::CheckpointReady(_));
            if outcome.is_terminal() {
                break;
            }
        }
        samples.sort_unstable();
        let p99 = samples[samples.len() * 99 / 100];
        assert!(saw_checkpoint);
        assert!(p99 < Duration::from_millis(2), "WFC unit p99 exceeded 2 ms: {p99:?}");
        assert!(samples.last().copied().expect("sample") < Duration::from_millis(8));
    }
}
//#endregion 🧪️Tests
