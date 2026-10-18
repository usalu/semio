//! 📚️ Persistent state and materialization: the removal, decision and entropy records, the init, choice and arc cursors, the search state and the bounded checkpoint and commit materialization. See [`super`].

use super::*;

//#region 📚️PersistentState
#[derive(Clone, Copy, Debug)]
pub(super) struct Removal {
    pub(super) node: NodeId,
    pub(super) pattern: PatternId,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Decision {
    pub(super) node: NodeId,
    pub(super) candidate: PatternId,
    pub(super) trail_mark: usize,
    pub(super) rng_state: [u64; 4],
}

#[derive(Clone, Copy, Debug, Eq)]
pub(super) struct EntropyEntry {
    pub(super) entropy_bits: u64,
    pub(super) node: NodeId,
    pub(super) revision: u64,
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
pub(super) enum InitPhase {
    #[default]
    Acquire,
    BuildFull,
    Fixed,
    Measure,
}

#[derive(Clone, Debug, Default)]
pub(super) struct InitCursor {
    pub(super) phase: InitPhase,
    pub(super) word: usize,
    pub(super) fixed: usize,
    pub(super) pattern: usize,
    pub(super) required: Option<PatternId>,
    pub(super) domain: Option<PatternSet>,
    pub(super) count: u32,
    pub(super) weight_sum: f64,
    pub(super) weighted_log_sum: f64,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ChoicePhase {
    Weigh,
    Select,
    Remove,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ChoiceCursor {
    pub(super) phase: ChoicePhase,
    pub(super) pattern: usize,
    pub(super) total: f64,
    pub(super) target: f64,
    pub(super) running: f64,
    pub(super) ordinal: u64,
    pub(super) candidate: Option<PatternId>,
}

#[derive(Clone, Debug)]
pub(super) struct ArcCursor {
    pub(super) source: NodeId,
    pub(super) raw_index: usize,
    pub(super) raw_bound: usize,
    pub(super) target: Option<NodeId>,
    pub(super) relation: Option<RelationId>,
    pub(super) allowed: Option<PatternSet>,
    pub(super) allowed_word: usize,
    pub(super) allowed_pattern: usize,
    pub(super) allowed_accumulator: u64,
    pub(super) restrict_word: usize,
    pub(super) removed_word: u64,
    pub(super) removed_word_index: usize,
    pub(super) changed: bool,
}

impl ArcCursor {
    pub(super) fn new(source: NodeId, raw_bound: usize) -> Self {
        Self { source, raw_index: 0, raw_bound, target: None, relation: None, allowed: None, allowed_word: 0, allowed_pattern: 0, allowed_accumulator: 0, restrict_word: 0, removed_word: 0, removed_word_index: 0, changed: false }
    }

    pub(super) fn clear_arc(&mut self) {
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
pub(super) struct WfcState {
    pub(super) operation_id: u64,
    pub(super) base_revision: u64,
    pub(super) generation: u64,
    pub(super) operation_seed: u64,
    pub(super) preview_sequence: u64,
    pub(super) topology_nodes: usize,
    pub(super) model_fingerprint: u64,
    pub(super) stage: WfcStage,
    pub(super) domains: Vec<PatternSet>,
    pub(super) domain_counts: Vec<u32>,
    pub(super) domain_weight_sums: Vec<f64>,
    pub(super) domain_weighted_log_sums: Vec<f64>,
    pub(super) singleton_count: usize,
    pub(super) empty_count: usize,
    pub(super) revisions: Vec<u64>,
    pub(super) init_node: usize,
    pub(super) init_cursor: InitCursor,
    pub(super) queue: VecDeque<NodeId>,
    pub(super) queued_marks: Vec<u64>,
    pub(super) queue_epoch: u64,
    pub(super) arc_cursor: Option<ArcCursor>,
    pub(super) choice_cursor: Option<ChoiceCursor>,
    pub(super) trail: Vec<Removal>,
    pub(super) decisions: Vec<Decision>,
    pub(super) backtrack_frame: Option<Decision>,
    pub(super) entropy_heap: BinaryHeap<EntropyEntry>,
    pub(super) rng: JobRng,
    pub(super) active_slot: Option<NodeId>,
    pub(super) tested_tile: Option<PatternId>,
    pub(super) contradiction: Option<NodeId>,
    pub(super) propagation_wave: Vec<NodeId>,
    pub(super) changed_domains: Vec<NodeId>,
    pub(super) backtrack_path: Vec<NodeId>,
    pub(super) observations: u64,
    pub(super) compatibility_edges: u64,
    pub(super) backtracks: u64,
    pub(super) observed: Vec<(NodeId, PatternId)>,
}
//#endregion 📚️PersistentState

//#region 💾️BoundedMaterialization
#[derive(Clone, Copy, Debug)]
pub(super) struct CheckpointCounts {
    pub(super) domain_count: usize,
    pub(super) pattern_count: usize,
    pub(super) trail_count: usize,
    pub(super) decision_count: usize,
    pub(super) observed_count: usize,
}

impl CheckpointCounts {
    pub(super) fn from_state(state: &WfcState, pattern_count: usize) -> Self {
        Self { domain_count: state.domains.len(), pattern_count, trail_count: state.trail.len(), decision_count: state.decisions.len(), observed_count: state.observed.len() }
    }

    pub(super) fn checked_bytes(self) -> Option<usize> {
        let domain_bytes = self.domain_count.checked_mul(self.pattern_count.div_ceil(u64::BITS as usize))?.checked_mul(size_of::<u64>())?;
        CHECKPOINT_FIXED_HEADER_BYTES
            .checked_add(domain_bytes)?
            .checked_add(self.trail_count.checked_mul(CHECKPOINT_TRAIL_ENTRY_BYTES)?)?
            .checked_add(self.decision_count.checked_mul(CHECKPOINT_DECISION_ENTRY_BYTES)?)?
            .checked_add(self.observed_count.checked_mul(CHECKPOINT_OBSERVED_ENTRY_BYTES)?)
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CheckpointPhase {
    Header,
    Domains,
    Trail,
    Decisions,
    Observed,
    Done,
}

#[derive(Clone, Debug)]
pub(super) struct CheckpointBuild {
    pub(super) bytes: Vec<u8>,
    pub(super) byte_limit: usize,
    pub(super) phase: CheckpointPhase,
    pub(super) outer: usize,
    pub(super) inner: usize,
    pub(super) terminal: bool,
}

impl CheckpointBuild {
    pub(super) fn new(state: &WfcState, pattern_count: usize, terminal: bool) -> Result<Self, WfcFault> {
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
pub(super) struct CommitBuild {
    pub(super) bytes: Vec<u8>,
    pub(super) byte_limit: usize,
    pub(super) assignment: Vec<u32>,
    pub(super) item_limit: usize,
    pub(super) cursor: usize,
    pub(super) started: bool,
}

impl CommitBuild {
    pub(super) fn new(node_count: usize) -> Result<Self, WfcFault> {
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

pub(super) fn ensure_materialization_space(bytes: &[u8], byte_limit: usize, additional: usize, detail: &'static [u8]) -> Result<(), WfcFault> {
    if bytes.len().checked_add(additional).is_none_or(|length| length > byte_limit) {
        let _ = detail;
        return Err(empty_job_fault());
    }
    Ok(())
}

pub(super) fn put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

pub(super) fn put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}
//#endregion 💾️BoundedMaterialization
