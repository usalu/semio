//! 🧵️ Persistent, fuel-bounded WFC execution for interactive and batch workers.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};

#[path = "🏃️headless/🦀️.rs"]
mod headless;
pub use headless::{admit_payload_authority, close_job, close_payload_authority, payload_bytes, run_headless, HeadlessSite, HEADLESS_GRANT};

#[path = "📤️publication/🦀️.rs"]
mod publication;
pub use publication::{close_result, Kind as PublicationKind, Publication};

use crate::bitset::PatternSet;
use crate::ids::{NodeId, PatternId, RelationId};
use crate::model::CompiledModel;
use crate::topology::Topology;

#[path = "🧭️protocol-and-determinism/🦀️.rs"]
mod protocol_and_determinism;
use protocol_and_determinism::{
    empty_job_fault, JobRng, CHECKPOINT_DECISION_ENTRY_BYTES, CHECKPOINT_FIXED_HEADER_BYTES,
    CHECKPOINT_HEADER_U64_FIELDS, CHECKPOINT_INTERVAL, CHECKPOINT_MAGIC,
    CHECKPOINT_OBSERVED_ENTRY_BYTES, CHECKPOINT_TRAIL_ENTRY_BYTES, COMMIT_FIXED_MAX_BYTES,
    COMMIT_ITEM_MAX_BYTES, MAX_COMMIT_BYTES, MAX_COMMIT_ITEMS, PREVIEW_ITEM_LIMIT,
    PREVIEW_TIME_INTERVAL_MS, PREVIEW_UNIT_INTERVAL,
};
pub use protocol_and_determinism::{
    SearchPulse, VisibleCell, WfcCommit, WfcFault, WfcJobConfig, WfcPreview, WfcSampler, WfcStage,
    MAX_CHECKPOINT_BYTES,
};

#[path = "📚️persistent-state-and-materialization/🦀️.rs"]
mod persistent_state_and_materialization;
use persistent_state_and_materialization::{
    ensure_materialization_space, put_u32, put_u64, ArcCursor, CheckpointBuild, CheckpointCounts,
    CheckpointPhase, ChoiceCursor, ChoicePhase, CommitBuild, Decision, EntropyEntry, InitCursor,
    InitPhase, Removal, WfcState,
};

#[path = "🧩️job-struct-and-preview/🦀️.rs"]
mod job_struct_and_preview;
pub use job_struct_and_preview::WfcJob;

#[path = "🔁️job-queues-patterns-and-initialize/🦀️.rs"]
mod job_queues_patterns_and_initialize;

#[path = "🌊️job-propagate-backtrack-and-checkpoint/🦀️.rs"]
mod job_propagate_backtrack_and_checkpoint;

#[path = "🔄️restore-job-state-and-resume/🦀️.rs"]
mod restore_job_state_and_resume;
use restore_job_state_and_resume::{RestorePreview, RestoreStage, WfcRun};
pub use restore_job_state_and_resume::WfcRestore;

#[path = "⏯️restore-job-run-and-interactive-job/🦀️.rs"]
mod restore_job_run_and_interactive_job;

#[path = "🚦️job-run-and-interactive-job/🦀️.rs"]
mod job_run_and_interactive_job;

//#region 🚚️Drive
// 🚚️ The synchronous drivers live in `🏃️headless`; `payload_bytes`, `close_job` and `run_headless` are re-exported above.
//#endregion 🚚️Drive

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
//#endregion 🧪️Tests
