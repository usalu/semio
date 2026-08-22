//! 🧵️ Persistent, compatibility-edge-bounded WFC execution for interactive callers.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId, RelationId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::topology::Topology;

//#region 🧭️Protocol
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WfcStage {
    InitializeDomains,
    FindMinimumEntropySlot,
    ChooseCandidate,
    PropagateCompatibilityEdge,
    DetectContradiction,
    BacktrackTrailEntry,
    CommitSlot,
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
            Self::Complete => "wfc.complete",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub observations: u64,
    pub compatibility_edges: u64,
    pub backtracks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
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
        let limit = u64::MAX - u64::MAX % hi;
        loop {
            let value = self.next_u64();
            if value < limit {
                return value % hi;
            }
        }
    }
}
//#endregion 🎲️Determinism

//#region 📚️PersistentState
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct Removal {
    node: NodeId,
    pattern: PatternId,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct Decision {
    node: NodeId,
    candidate: PatternId,
    trail_mark: usize,
    rng_state: [u64; 4],
}

#[derive(Clone, Copy, Debug, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct ArcCursor {
    source: NodeId,
    arcs: Vec<(NodeId, RelationId)>,
    index: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct WfcState {
    model_fingerprint: u64,
    stage: WfcStage,
    domains: Vec<PatternSet>,
    revisions: Vec<u64>,
    init_cursor: usize,
    queue: VecDeque<NodeId>,
    queued: Vec<bool>,
    arc_cursor: Option<ArcCursor>,
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

//#region 🧩️Job
pub(crate) struct WfcJob<T> {
    operation: Operation,
    model: CompiledModel,
    topology: T,
    config: WfcJobConfig,
    initial_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    state: WfcState,
}

impl<T: Topology + Clone> WfcJob<T> {
    pub fn new(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>) -> Self {
        let node_count = topology.node_count();
        assert!(initial_domains.as_ref().is_none_or(|domains| domains.len() == node_count));
        Self {
            state: WfcState {
                model_fingerprint: model.fingerprint(),
                stage: WfcStage::InitializeDomains,
                domains: Vec::with_capacity(node_count),
                revisions: vec![0; node_count],
                init_cursor: 0,
                queue: VecDeque::with_capacity(node_count),
                queued: vec![false; node_count],
                arc_cursor: None,
                trail: Vec::new(),
                decisions: Vec::new(),
                backtrack_frame: None,
                entropy_heap: BinaryHeap::new(),
                rng: JobRng::from_seed(operation.seed),
                active_slot: None,
                tested_tile: None,
                contradiction: None,
                propagation_wave: Vec::new(),
                changed_domains: Vec::new(),
                backtrack_path: Vec::new(),
                observations: 0,
                compatibility_edges: 0,
                backtracks: 0,
                observed: Vec::new(),
            },
            operation,
            model,
            topology,
            config,
            initial_domains,
            fixed,
        }
    }

    pub fn from_checkpoint(operation: Operation, model: CompiledModel, topology: T, config: WfcJobConfig, initial_domains: Option<Vec<PatternSet>>, fixed: Vec<(NodeId, PatternId)>, bytes: &[u8]) -> Result<Self, String> {
        let state: WfcState = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
        if state.model_fingerprint != model.fingerprint() || state.domains.len() > topology.node_count() {
            return Err("wfc-checkpoint-input-mismatch".into());
        }
        Ok(Self { operation, model, topology, config, initial_domains, fixed, state })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.state).expect("WFC checkpoint state is serializable")
    }

    /// 🪪️ Exposes the immutable runtime identity required by the shared batch driver.
    pub(crate) fn operation(&self) -> Operation {
        self.operation
    }

    pub fn preview(&self, sequence: u64) -> WfcPreview {
        let domains = &self.state.domains;
        WfcPreview {
            sequence,
            stage: self.state.stage,
            active_slot: self.state.active_slot.map(NodeId::get),
            candidates: self.state.active_slot.and_then(|node| domains.get(node.index())).map(|domain| domain.iter_ones().map(PatternId::get).collect()).unwrap_or_default(),
            tested_tile: self.state.tested_tile.map(PatternId::get),
            propagation_wave: self.state.propagation_wave.iter().copied().map(NodeId::get).collect(),
            changed_domains: self.state.changed_domains.iter().filter_map(|&node| domains.get(node.index()).map(|domain| (node.get(), domain.iter_ones().map(PatternId::get).collect()))).collect(),
            contradiction: self.state.contradiction.map(NodeId::get),
            backtrack_path: self.state.backtrack_path.iter().copied().map(NodeId::get).collect(),
            incomplete_grid: domains.iter().map(|domain| (domain.count_ones() == 1).then(|| domain.first_set().expect("singleton domain").get())).collect(),
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

    pub(crate) fn domain_masks(&self) -> Vec<PatternSet> {
        self.state.domains.clone()
    }

    pub(crate) fn metrics(&self) -> (u64, u64, u64) {
        (self.state.observations, self.state.compatibility_edges, self.state.backtracks)
    }

    pub(crate) fn observed(&self) -> &[(NodeId, PatternId)] {
        &self.state.observed
    }

    fn push_queue(&mut self, node: NodeId) {
        if !self.state.queued[node.index()] {
            self.state.queued[node.index()] = true;
            self.state.queue.push_back(node);
        }
    }

    fn entropy(&self, node: NodeId) -> f64 {
        let domain = &self.state.domains[node.index()];
        let (sum, weighted_log_sum) = domain.iter_ones().fold((0.0, 0.0), |(sum, weighted), pattern| {
            let weight = self.model.weights().w(pattern);
            (sum + weight, weighted + weight * weight.ln())
        });
        if sum <= 0.0 {
            0.0
        } else {
            sum.ln() - weighted_log_sum / sum
        }
    }

    fn push_entropy(&mut self, node: NodeId) {
        if self.state.domains[node.index()].count_ones() > 1 {
            self.state.entropy_heap.push(EntropyEntry { entropy_bits: self.entropy(node).to_bits(), node, revision: self.state.revisions[node.index()] });
        }
    }

    fn initialize_one(&mut self) {
        let index = self.state.init_cursor;
        if index == self.topology.node_count() {
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
            return;
        }
        let node = NodeId::from_index(index);
        let mut domain = self.initial_domains.as_ref().map(|domains| domains[index].clone()).unwrap_or_else(|| self.model.full_domain());
        for &(fixed_node, pattern) in &self.fixed {
            if fixed_node == node {
                let mut singleton = PatternSet::new_empty(self.model.pattern_count());
                singleton.set(pattern, true);
                domain.and_with(&singleton);
            }
        }
        if domain.count_ones() == 0 {
            self.state.contradiction = Some(node);
        }
        self.state.domains.push(domain);
        self.state.init_cursor += 1;
        self.push_queue(node);
        self.push_entropy(node);
    }

    fn find_slot(&mut self) {
        while let Some(entry) = self.state.entropy_heap.pop() {
            let domain = &self.state.domains[entry.node.index()];
            if self.state.revisions[entry.node.index()] == entry.revision && domain.count_ones() > 1 {
                self.state.active_slot = Some(entry.node);
                self.state.stage = WfcStage::ChooseCandidate;
                return;
            }
        }
        self.state.stage = if self.state.domains.iter().all(|domain| domain.count_ones() == 1) { WfcStage::Complete } else { WfcStage::DetectContradiction };
    }

    fn choose_candidate(&mut self) {
        let node = self.state.active_slot.expect("choose stage has active slot");
        let domain = &self.state.domains[node.index()];
        let candidate = match self.config.sampler {
            WfcSampler::Uniform => domain.iter_ones().nth(self.state.rng.range(domain.count_ones() as u64) as usize).expect("unresolved domain"),
            WfcSampler::WeightedRoulette => {
                let total: f64 = domain.iter_ones().map(|pattern| self.model.weights().w(pattern)).sum();
                let target = self.state.rng.next_f64() * total;
                let mut sum = 0.0;
                domain
                    .iter_ones()
                    .find(|&pattern| {
                        sum += self.model.weights().w(pattern);
                        sum >= target
                    })
                    .or_else(|| domain.first_set())
                    .expect("unresolved domain")
            }
        };
        self.state.decisions.push(Decision { node, candidate, trail_mark: self.state.trail.len(), rng_state: self.state.rng.state });
        self.state.tested_tile = Some(candidate);
        let old = self.state.domains[node.index()].clone();
        for removed in old.iter_ones().filter(|&pattern| pattern != candidate) {
            self.state.domains[node.index()].set(removed, false);
            self.state.trail.push(Removal { node, pattern: removed });
        }
        self.state.revisions[node.index()] += 1;
        self.state.observations += 1;
        self.state.observed.push((node, candidate));
        self.state.queue.clear();
        self.state.queued.fill(false);
        self.push_queue(node);
        self.state.stage = WfcStage::PropagateCompatibilityEdge;
    }

    fn propagate_one(&mut self) {
        if self.state.contradiction.is_some() {
            self.state.stage = WfcStage::DetectContradiction;
            return;
        }
        if self.state.arc_cursor.as_ref().is_none_or(|cursor| cursor.index == cursor.arcs.len()) {
            let Some(source) = self.state.queue.pop_front() else {
                self.state.stage = WfcStage::DetectContradiction;
                return;
            };
            self.state.queued[source.index()] = false;
            let mut arcs = Vec::new();
            self.topology.for_each_out_arc(source, |target, relation| arcs.push((target, relation)));
            self.state.propagation_wave.push(source);
            if arcs.is_empty() {
                self.state.arc_cursor = None;
                return;
            }
            self.state.arc_cursor = Some(ArcCursor { source, arcs, index: 0 });
        }
        let cursor = self.state.arc_cursor.as_mut().expect("arc cursor initialized");
        let (target, relation) = cursor.arcs[cursor.index];
        cursor.index += 1;
        let source = cursor.source;
        let mut allowed = PatternSet::new_empty(self.model.pattern_count());
        for pattern in self.state.domains[source.index()].iter_ones() {
            allowed.or_with(self.model.allowed(relation, pattern));
        }
        let old = self.state.domains[target.index()].clone();
        self.state.domains[target.index()].and_with(&allowed);
        let removed: Vec<_> = old.iter_ones().filter(|&pattern| !self.state.domains[target.index()].get(pattern)).collect();
        self.state.compatibility_edges += 1;
        if !removed.is_empty() {
            for pattern in removed {
                self.state.trail.push(Removal { node: target, pattern });
            }
            self.state.revisions[target.index()] += 1;
            if !self.state.changed_domains.contains(&target) {
                self.state.changed_domains.push(target);
            }
            if self.state.domains[target.index()].count_ones() == 0 {
                self.state.contradiction = Some(target);
            } else {
                self.push_queue(target);
                self.push_entropy(target);
            }
        }
    }

    fn detect(&mut self) {
        if self.state.contradiction.is_some() {
            self.state.backtrack_frame = self.state.decisions.pop();
            self.state.backtracks += 1;
            self.state.stage = WfcStage::BacktrackTrailEntry;
        } else if self.state.queue.is_empty() && self.state.arc_cursor.as_ref().is_none_or(|cursor| cursor.index == cursor.arcs.len()) {
            self.state.stage = if self.state.domains.iter().all(|domain| domain.count_ones() == 1) { WfcStage::Complete } else { WfcStage::CommitSlot };
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
            self.state.domains[removed.node.index()].set(removed.pattern, true);
            self.state.revisions[removed.node.index()] += 1;
            if self.state.backtrack_path.last() != Some(&removed.node) {
                self.state.backtrack_path.push(removed.node);
            }
            return;
        }
        self.state.backtrack_frame = None;
        self.state.contradiction = None;
        self.state.arc_cursor = None;
        self.state.queue.clear();
        self.state.queued.fill(false);
        self.state.domains[frame.node.index()].set(frame.candidate, false);
        self.state.trail.push(Removal { node: frame.node, pattern: frame.candidate });
        self.state.revisions[frame.node.index()] += 1;
        if self.state.domains[frame.node.index()].count_ones() == 0 {
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
}

impl<T: Topology + Clone + Send> InteractiveJob for WfcJob<T> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-wfc-operation".to_vec() });
        }
        loop {
            context.set_stage(self.state.stage.label());
            match self.state.stage {
                WfcStage::InitializeDomains => self.initialize_one(),
                WfcStage::FindMinimumEntropySlot => self.find_slot(),
                WfcStage::ChooseCandidate => self.choose_candidate(),
                WfcStage::PropagateCompatibilityEdge => self.propagate_one(),
                WfcStage::DetectContradiction => self.detect(),
                WfcStage::BacktrackTrailEntry => self.backtrack_one(),
                WfcStage::CommitSlot => {
                    self.state.stage = WfcStage::FindMinimumEntropySlot;
                    let preview = self.preview(context.next_preview_sequence());
                    self.reset_preview_delta();
                    return StepOutcome::PreviewReady(serde_json::to_vec(&preview).expect("WFC preview is serializable"));
                }
                WfcStage::Complete => {
                    if self.state.contradiction.is_some() || self.state.domains.iter().any(|domain| domain.count_ones() == 0) {
                        return StepOutcome::Fault(JobFault { detail: b"wfc-unsatisfiable".to_vec() });
                    }
                    let output = serde_json::to_vec(&self.commit().expect("complete WFC job has commit")).expect("WFC commit is serializable");
                    return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output });
                }
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
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
        let mut sequence = 0;
        for _ in 0..100_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(fuel, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            let outcome = job.step(&mut context);
            if outcome.is_terminal() {
                return outcome;
            }
        }
        panic!("WFC job did not terminate");
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
    fn checkpoint_resume_preserves_rng_queue_trail_and_cursors() {
        let mut original = checkerboard(41, 71);
        let mut sequence = 0;
        for _ in 0..23 {
            let mut context = StepContext::new(original.operation.operation, original.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            assert!(!original.step(&mut context).is_terminal());
        }
        let checkpoint = original.checkpoint_bytes();
        let mut restored = WfcJob::from_checkpoint(original.operation, original.model.clone(), original.topology.clone(), original.config, original.initial_domains.clone(), original.fixed.clone(), &checkpoint).expect("restore");
        assert_eq!(restored.checkpoint_bytes(), checkpoint);
        assert!(matches!(drive(&mut original, 3), StepOutcome::Complete(_)));
        assert!(matches!(drive(&mut restored, 11), StepOutcome::Complete(_)));
        assert_eq!(original.commit(), restored.commit());
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
    fn cancellation_and_generation_freshness_do_not_mutate_state() {
        let mut job = checkerboard(10, 3);
        let before = job.checkpoint_bytes();
        let mut sequence = 0;
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut cancelled = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(10, u64::MAX), cancel, || 0, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.checkpoint_bytes(), before);
        let mut stale = StepContext::new(job.operation.operation, Generation(job.operation.generation.0 + 1), StepBudget::new(10, u64::MAX), root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!(job.checkpoint_bytes(), before);
    }

    #[test]
    fn one_unit_adversarial_step_stays_below_watchdog() {
        let mut job = checkerboard(65_536, 4);
        let mut sequence = 0;
        let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
        let start = Instant::now();
        let outcome = job.step(&mut context);
        assert!(start.elapsed() < Duration::from_millis(8), "one bounded WFC unit exceeded watchdog");
        assert!(!outcome.is_terminal());
    }
}
//#endregion 🧪️Tests
