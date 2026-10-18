//! 🧭️ Protocol and determinism: the preview and checkpoint limits, the fault, stage, preview and commit types, the sampler, search pulse and job config and the job random number generator. See [`super`].

//#region 🧭️Protocol
pub(super) const PREVIEW_ITEM_LIMIT: usize = 256;

pub(super) const PREVIEW_UNIT_INTERVAL: u64 = 16;

pub(super) const PREVIEW_TIME_INTERVAL_MS: u64 = 16;

pub(super) const CHECKPOINT_INTERVAL: u64 = 64;

pub(super) const CHECKPOINT_MAGIC: &[u8; 8] = b"SWFCJ002";

pub const MAX_CHECKPOINT_BYTES: usize = 1 << 20;

const CHECKPOINT_HEADER_IDENTITY_U64_FIELDS: usize = 8;

const CHECKPOINT_HEADER_RNG_U64_FIELDS: usize = 4;

const CHECKPOINT_HEADER_PROGRESS_U64_FIELDS: usize = 3;

const CHECKPOINT_HEADER_COUNT_U64_FIELDS: usize = 5;

pub(super) const CHECKPOINT_HEADER_U64_FIELDS: usize = CHECKPOINT_HEADER_IDENTITY_U64_FIELDS + CHECKPOINT_HEADER_RNG_U64_FIELDS + CHECKPOINT_HEADER_PROGRESS_U64_FIELDS + CHECKPOINT_HEADER_COUNT_U64_FIELDS;

pub(super) const CHECKPOINT_FIXED_HEADER_BYTES: usize = CHECKPOINT_MAGIC.len() + CHECKPOINT_HEADER_U64_FIELDS * size_of::<u64>();

pub(super) const CHECKPOINT_TRAIL_ENTRY_BYTES: usize = 2 * size_of::<u32>();

pub(super) const CHECKPOINT_DECISION_ENTRY_BYTES: usize = 2 * size_of::<u32>() + 5 * size_of::<u64>();

pub(super) const CHECKPOINT_OBSERVED_ENTRY_BYTES: usize = 2 * size_of::<u32>();

pub(super) const MAX_COMMIT_BYTES: usize = 1 << 20;

pub(super) const COMMIT_FIXED_MAX_BYTES: usize = 160;

pub(super) const COMMIT_ITEM_MAX_BYTES: usize = 11;

pub(super) const MAX_COMMIT_ITEMS: usize = (MAX_COMMIT_BYTES - COMMIT_FIXED_MAX_BYTES) / COMMIT_ITEM_MAX_BYTES;

/// 💥️ A bounded materialization or restore refusal; it publishes as an empty-detail fault.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WfcFault;

pub(super) fn empty_job_fault() -> WfcFault {
    WfcFault
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
    pub(super) fn label(self) -> &'static str {
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

/// 👁️ What one headless [`WfcJob::advance_one`] unit did. `Collapsed` and `Discarded` are the
/// events a fill preview paints, including choices the search later undoes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchPulse {
    Worked,
    Collapsed { node: u32, pattern: u32 },
    Discarded { node: u32, pattern: u32 },
    Solved,
    Unsatisfiable,
}

/// 👁 One cell that became a singleton or stopped being one during `advance_one`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisibleCell {
    pub node: u32,
    pub pattern: u32,
    pub discarded: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WfcJobConfig {
    pub sampler: WfcSampler,
}
//#endregion 🧭️Protocol

//#region 🎲️Determinism
#[derive(Clone, Copy, Debug)]
pub(super) struct JobRng {
    pub(super) state: [u64; 4],
}

impl JobRng {
    pub(super) fn from_seed(seed: u64) -> Self {
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

    pub(super) fn next_u64(&mut self) -> u64 {
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

    pub(super) fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    pub(super) fn range(&mut self, hi: u64) -> u64 {
        if hi == 0 {
            return 0;
        }
        ((u128::from(self.next_u64()) * u128::from(hi)) >> 64) as u64
    }
}
//#endregion 🎲️Determinism
