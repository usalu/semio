//! 🗄️ `db_artifact` — the document authority actor and its command pipeline: admit → dedupe →
//! base-resolve → authz → deps → validate → conflict → execute → WAL append → durability →
//! publish → project → preview-reconcile → receipt. Composes `db_state` (materialized
//! overlay), `db_wal` (durability), `db_storage` (the pluggable substrate), and `protocol`
//! (`MutationEnvelope`/`MutationDiff`) into `ArtifactEngine`, the crate's central type, plus a
//! thin `db_actor`-mailbox wrapper (`ArtifactAuthority`) around it. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_artifact` row).
//!
//! 🎯️ Design choice (compatibility surface — re-checked against `db_engine`'s live state, which
//! was itself being revised concurrently while this file was authored): `db_engine`'s
//! `document_engine_config` now builds `ArtifactEngineConfig{limits, preview_ttl_ms,
//! ..ArtifactEngineConfig::default()}` (a real `db_security::SecurityGate` baked directly into the
//! default, defaulted `..` spread for the rest),
//! `CommandReceipt.conflicts: Vec<db_conflict::ConflictRecord>` (the real type), and
//! `ArtifactAuthority::run_query(query, consistency)` (two arguments, `db_query::
//! Consistency`-aware). This revision matches that shape exactly: `security`/`emit` are real
//! `ArtifactEngineConfig` fields, `submit`'s conflict step is a genuine
//! `db_conflict::ConflictDetector` fed by retained recent-commit `TouchedSet` history (not a local
//! last-writer stand-in), and `query`/`RunQuery`/`run_query` take a `db_query::
//! Consistency` and resolve it via `db_query::resolve_consistency` + `db_index::
//! IndexConsistencyResolver`. `submit` authorizes through `ArtifactEngineConfig::security` (the real
//! `db_security::SecurityGate`) only. Because `ArtifactEngineConfig`'s new fields are absorbed via `..Default::default()` at every call site
//! observed, `db_projection` registration is also wired in as a `projections` factory field (see
//! `🔖️Engine`'s doc for why a factory, not a stored engine).
//!
//! 🎯️ Design choice (diff convention, unchanged): `protocol::MutationEnvelope`'s `diff`/`inverse`
//! payloads are schema-erased `dsl::DslValue` pathmaps encoded with `store::pack_rt::encode_wire_value`
//! — `db_artifact` has no compile-time knowledge of
//! any concrete document schema, so it adopts one generic convention for BOTH: a JSON *object*
//! whose keys are `db_state`-style `/`-segmented paths and whose values are either the new JSON
//! value to set at that path, or JSON `null` as an explicit tombstone (path deleted). This is a
//! real, documented limitation, not a workaround: a legitimate application-level `null` value is
//! indistinguishable from a delete under this convention. `envelope_from_operation` (new this
//! revision) is the generic ingestion boundary that actually exercises `protocol::Mutation`/
//! `MutationDiff` (`diff`/`mutation_id`/`dependencies`/`author_id`/`timestamp`, the diff applied once through
//! `protocol::apply_diff`) to build an envelope in this same convention from a typed operation, persisting only the changed fields — the one place in the `db` family
//! below `db_artifact` allowed to interpret operation semantics at all (per the contract's hard
//! dependency rule).
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use db_artifact_contract::{ArtifactDurableGroupJournalAppendV1, ArtifactDurableGroupRecoveryOwnerV1, ArtifactMessage, CheckpointPublicationSnapshot, CommandBatch, CommandReceipt, SubmitOptions};
use db_artifact_engine::{ArtifactEngine, ArtifactEngineOpenRejected};
use db_artifact_history::{ArtifactHistoryView, HistoryReplayFuture, HistoryReplayReservation};
use db_durability::Frontier;
use db_ids::{*, ArtifactId, DbError, GenerationId};
use db_policy::{MailboxCapacities, Priority, worker_submit_retry_attempt};
use protocol::MutationDiff as _;
use std::{future::Future, pin::Pin, sync::Arc};
#[cfg(test)]
use db_artifact_contract::{ArtifactCommittedDurableGroupRecoveryAdvanceV1, ArtifactDurableGroupRecoveryDriverCoreV1 as _, DB_PATHMAP_SCHEMA, MALFORMED_HISTORY_TRANSITION_CODE, UNKNOWN_SUPERSEDE_TARGET_CODE, committed_durable_group_decision_from_transaction, decode_pathmap_json, encode_pathmap, to_core_document_id};
#[cfg(test)]
use db_artifact_engine::{ArtifactEngineConfig, admit_wal_bytes, close_wal_record_batch, push_wal_record};
#[cfg(test)]
use db_artifact_history::history_capacity_test_lock;
#[cfg(test)]
use db_durability::DurabilityClass;
#[cfg(test)]
use db_ids::ActorId;
#[cfg(test)]
use semio_framework_value::DslValue;

//#region 🔖️Ids
//#region 🔖️Actor
/// 🎭️ A live handle to one document's authority actor. Each admitted mailbox message wakes
/// one finite `WorkerPool` turn; no job waits for the next message and no authority owns an OS
/// thread. `db_state::PMap` uses `Arc`-shared HAMT nodes, making the engine movable between turns
/// without sharing mutable actor state or weakening its single-consumer mailbox semantics.
#[cfg(not(target_arch = "wasm32"))]
pub struct ArtifactAuthority {
    address: db_actor::Address<ArtifactMessage>,
    pub cancel: Arc<dyn Fn() + Send + Sync>,
    pub handoff: Arc<ArtifactRunnerHandoff>,
    pub retirement: Option<ArtifactRunnerRetirementReservation>,
    retirement_close: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    _pool_use: Arc<semio_framework_async::WorkerPoolUse>,
    done: std::sync::Mutex<Option<db_actor::ReplyReceiver<()>>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactDurableGroupJournalSinkV1 {
    address: db_actor::Address<ArtifactMessage>,
    pub now_ms: u64,
}

#[cfg(not(target_arch = "wasm32"))]
enum ArtifactDurableGroupJournalCommitStateV1 {
    NotSubmitted { decision_pack: Vec<u8>, decision_sha256: String },
    Awaiting(Pin<Box<db_actor::AskFuture<ArtifactMessage, Result<ArtifactDurableGroupJournalAppendV1, DbError>>>>),
    Rejected(String),
    Failed(String),
    Absent,
    Committed(store::durable_group::DurableOwnedGroupJournalReceiptV1),
    Empty,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactDurableGroupJournalCommitV1 {
    address: Option<db_actor::Address<ArtifactMessage>>,
    pub now_ms: u64,
    pub cancelled: Option<Arc<std::sync::atomic::AtomicBool>>,
    close_started: bool,
    pub state: ArtifactDurableGroupJournalCommitStateV1,
}

#[cfg(not(target_arch = "wasm32"))]
impl store::durable_group::DurableOwnedGroupJournalSinkV1 for ArtifactDurableGroupJournalSinkV1 {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn store::durable_group::DurableOwnedGroupJournalCommitV1> {
        Box::new(ArtifactDurableGroupJournalCommitV1 {
            address: Some(self.address.clone()),
            now_ms: self.now_ms,
            cancelled: Some(Arc::new(std::sync::atomic::AtomicBool::new(false))),
            close_started: false,
            state: ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 },
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::durable_group::DurableOwnedGroupJournalCommitV1 for ArtifactDurableGroupJournalCommitV1 {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::durable_group::DurableOwnedGroupJournalAdvanceV1, String> {
        use store::durable_group::DurableOwnedGroupJournalAdvanceV1;
        if !grant.permits_one() {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        if matches!(self.state, ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { .. }) {
            let state = std::mem::replace(&mut self.state, ArtifactDurableGroupJournalCommitStateV1::Empty);
            let ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } = state else { unreachable!() };
            if self.cancelled.as_ref().expect("live journal cancellation authority").load(std::sync::atomic::Ordering::Acquire) {
                self.state = ArtifactDurableGroupJournalCommitStateV1::Absent;
            } else if grant.maximum_copy_bytes < decision_pack.len() {
                self.state = ArtifactDurableGroupJournalCommitStateV1::Rejected("durable group journal grant is smaller than the canonical decision".to_string());
            } else {
                match store::durable_group::DurableOwnedGroupJournalRecordV1::admit(decision_pack, &decision_sha256) {
                    Ok(record) => {
                        let cancelled = Arc::clone(self.cancelled.as_ref().expect("live journal cancellation authority"));
                        let future = self.address.as_ref().expect("live journal actor authority").ask(Priority::Command, |reply| ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms: self.now_ms, reply });
                        self.state = ArtifactDurableGroupJournalCommitStateV1::Awaiting(Box::pin(future));
                    }
                    Err(error) => self.state = ArtifactDurableGroupJournalCommitStateV1::Rejected(error.to_string()),
                }
            }
        }
        if let ArtifactDurableGroupJournalCommitStateV1::Awaiting(future) = &mut self.state {
            let polled = future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop()));
            if let std::task::Poll::Ready(result) = polled {
                self.state = match result {
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Absent)) => ArtifactDurableGroupJournalCommitStateV1::Absent,
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Rejected(error))) => ArtifactDurableGroupJournalCommitStateV1::Rejected(error.to_string()),
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt))) => ArtifactDurableGroupJournalCommitStateV1::Committed(receipt),
                    Ok(Err(error)) | Err(error) => ArtifactDurableGroupJournalCommitStateV1::Failed(error.to_string()),
                };
            }
        }
        match &self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { .. } | ArtifactDurableGroupJournalCommitStateV1::Awaiting(_) => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            ArtifactDurableGroupJournalCommitStateV1::Rejected(reason) => Ok(DurableOwnedGroupJournalAdvanceV1::Rejected(reason.clone())),
            ArtifactDurableGroupJournalCommitStateV1::Failed(reason) => Err(reason.clone()),
            ArtifactDurableGroupJournalCommitStateV1::Absent => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) => Ok(DurableOwnedGroupJournalAdvanceV1::Committed(receipt.clone())),
            ArtifactDurableGroupJournalCommitStateV1::Empty => Err("durable group journal owner is closed".to_string()),
        }
    }

    fn cancel(&mut self) {
        self.cancelled.as_ref().expect("live journal cancellation authority").store(true, std::sync::atomic::Ordering::Release);
    }

    fn begin_close(&mut self) {
        self.close_started = true;
    }

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let release_bytes = match &self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } => if decision_pack.capacity() != 0 { decision_pack.capacity() } else { decision_sha256.capacity() },
            ArtifactDurableGroupJournalCommitStateV1::Rejected(message) => message.capacity(),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) => if receipt.anchor_sha256.capacity() != 0 { receipt.anchor_sha256.capacity() } else { receipt.decision_sha256.capacity() },
            ArtifactDurableGroupJournalCommitStateV1::Awaiting(_) | ArtifactDurableGroupJournalCommitStateV1::Failed(_) => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "uncertain durable journal retains its original resolution owner")),
            ArtifactDurableGroupJournalCommitStateV1::Absent => 0,
            ArtifactDurableGroupJournalCommitStateV1::Empty if self.address.is_some() || self.cancelled.is_some() => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "durable journal retains its original actor and cancellation capability")),
            ArtifactDurableGroupJournalCommitStateV1::Empty => 0,
        };
        Ok(RetirementDemand { release_bytes, depth: usize::from(!semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(self)), ..Default::default() })
    }

 }

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::ErasedSnapshotRetirement for ArtifactDurableGroupJournalCommitV1 {
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.depth) }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.close_started || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        match &mut self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } if decision_pack.capacity() != 0 => drop(std::mem::take(decision_pack)),
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_sha256, .. } if decision_sha256.capacity() != 0 => drop(std::mem::take(decision_sha256)),
            ArtifactDurableGroupJournalCommitStateV1::Rejected(message) if message.capacity() != 0 => drop(std::mem::take(message)),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) if receipt.anchor_sha256.capacity() != 0 => drop(std::mem::take(&mut receipt.anchor_sha256)),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) if receipt.decision_sha256.capacity() != 0 => drop(std::mem::take(&mut receipt.decision_sha256)),
            _ => self.state = ArtifactDurableGroupJournalCommitStateV1::Empty,
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool { matches!(self.state, ArtifactDurableGroupJournalCommitStateV1::Empty) && self.address.is_none() && self.cancelled.is_none() }

}

/// 🧳️ The runner owns its engine **boxed**, and every future it drives yields the box, never the
/// engine itself. `ArtifactEngine` measured ~529 KB by value (read off the
/// two `catch_unwind` shim frames of ticket 26/09/18 slice HS1's crash reports), and `run_turn`
/// carries each turn's result out through `std::panic::catch_unwind` → `do_call` → its own frame. A
/// debug build gives every one of those moves its own stack slot, so a by-value engine priced
/// `run_turn` at 3,598,608 B and the whole document-mount chain at 8,506,832 B — on pool workers
/// that own 2 MiB. Moving a pointer instead is what keeps the hub's first document socket from
/// aborting the process; see `📓️hs1-hub-pool-worker-stack-overflow.md`.
#[cfg(not(target_arch = "wasm32"))]
type ArtifactBuildFuture = Pin<Box<dyn Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
type ArtifactTurnFuture = Pin<Box<dyn Future<Output = Box<ArtifactEngine>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
enum ArtifactTurn {
    Future(ArtifactTurnFuture),
    CloseFlush(ArtifactTurnFuture),
    History { engine: Option<Box<ArtifactEngine>>, replay: HistoryReplayFuture, reply: Option<db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>> },
}

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETRY_MS: u64 = 1;

/// 🔁️ Automatic re-polls a faulted artifact-engine close receives before it waits for an explicit
/// re-admission (`ArtifactAuthority::readmit_close_retry`, `Database::shutdown`).
pub const ARTIFACT_CLOSE_RETRY_LIMIT: u8 = 8;

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_CLOSE_RETRY_BASE_MS: u64 = 2;

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_CLOSE_RETRY_CAP_MS: u64 = 512;

/// 🧭️ Where the bounded retry owner of a faulted artifact-engine close stands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactCloseRetryState {
    Clear,
    Scheduled,
    Admitted,
    Exhausted,
    Cancelled,
}

/// 📈️ Progress of one faulted close: `attempts` counts automatic re-polls armed so far out of `limit`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactCloseRetryProgress {
    pub state: ArtifactCloseRetryState,
    pub attempts: u8,
    pub limit: u8,
}

impl ArtifactCloseRetryProgress {
    /// 🧱️ True once no automatic retry remains: only an explicit re-admission can re-poll the close.
    pub fn is_blocked(&self) -> bool {
        matches!(self.state, ArtifactCloseRetryState::Exhausted | ArtifactCloseRetryState::Cancelled)
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactCloseRetry {
    pub state: ArtifactCloseRetryState,
    attempts: u8,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactCloseRetry {
    pub const fn clear() -> Self {
        Self { state: ArtifactCloseRetryState::Clear, attempts: 0, generation: 0 }
    }
}

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETRY_LIMIT: u8 = 8;

#[cfg(not(target_arch = "wasm32"))]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum ArtifactRunnerDriver {
    RunnableIdle,
    Queued,
    Polling,
    PollingWake,
    Parked,
    ClosingReady,
    ClosingPolling,
    ClosingPollingWake,
    ClosingParked,
    Terminal,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerHandoff {
    pool: Arc<semio_framework_async::WorkerPool>,
    pool_use: std::sync::Mutex<Option<Arc<semio_framework_async::WorkerPoolUse>>>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    close_runner: std::sync::Mutex<Option<Arc<dyn Fn() -> bool + Send + Sync>>>,
    retirement_maintenance: std::sync::Mutex<Option<(Arc<semio_framework_async::WorkerPool>, semio_framework_async::WorkerMaintenanceTicket)>>,
    pub close_error: std::sync::Mutex<Option<DbError>>,
    close_retry: std::sync::Mutex<ArtifactCloseRetry>,
    #[cfg(test)]
    close_faults: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    close_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    retirement_turns: std::sync::atomic::AtomicUsize,
    active_history: std::sync::atomic::AtomicBool,
    pub driver: std::sync::atomic::AtomicU8,
    pub terminal: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerHandoff {
    fn drop(&mut self) {
        if self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerHandoff {
    /// ⏲️ Arms the next bounded-backoff re-poll of a faulted close on the pool timer wheel. The
    /// timer admits exactly one re-poll through the retirement hook; stray wakes and coalesced
    /// maintenance requests never re-poll a faulted close.
    fn arm_close_retry(self: &Arc<Self>) {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Clear | ArtifactCloseRetryState::Admitted) {
            return;
        }
        if retry.attempts >= ARTIFACT_CLOSE_RETRY_LIMIT {
            retry.state = ArtifactCloseRetryState::Exhausted;
            return;
        }
        let delay = ARTIFACT_CLOSE_RETRY_BASE_MS.checked_shl(u32::from(retry.attempts)).unwrap_or(ARTIFACT_CLOSE_RETRY_CAP_MS).min(ARTIFACT_CLOSE_RETRY_CAP_MS);
        let Some(deadline) = self.pool.now_ms().checked_add(delay) else {
            retry.state = ArtifactCloseRetryState::Exhausted;
            return;
        };
        retry.attempts += 1;
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Scheduled;
        let generation = retry.generation;
        drop(retry);
        let handoff = Arc::downgrade(self);
        self.pool.callback_at(deadline, move || {
            if let Some(handoff) = handoff.upgrade() {
                handoff.admit_close_retry(generation);
            }
        });
    }

    fn admit_close_retry(&self, generation: u64) {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if retry.state != ArtifactCloseRetryState::Scheduled || retry.generation != generation {
            return;
        }
        retry.state = ArtifactCloseRetryState::Admitted;
        drop(retry);
        self.request_retirement_maintenance();
    }

    fn clear_close_retry(&self) {
        *self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = ArtifactCloseRetry::clear();
    }

    /// 🚦️ A healthy close always polls; a faulted one polls only under its admitted retry.
    fn close_poll_admitted(&self) -> bool {
        let faulted = self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        !faulted || self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner).state == ArtifactCloseRetryState::Admitted
    }

    fn close_retry_progress(&self) -> Option<ArtifactCloseRetryProgress> {
        if self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            return None;
        }
        let retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        Some(ArtifactCloseRetryProgress { state: retry.state, attempts: retry.attempts, limit: ARTIFACT_CLOSE_RETRY_LIMIT })
    }

    /// 🔂️ Grants an exhausted or cancelled faulted close a fresh retry budget and admits one re-poll now.
    fn readmit_close_retry(&self) -> bool {
        if self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            return false;
        }
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Exhausted | ArtifactCloseRetryState::Cancelled) {
            return false;
        }
        retry.attempts = 0;
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Admitted;
        drop(retry);
        self.request_retirement_maintenance();
        true
    }

    /// 🛑️ Stops automatic re-polls of a faulted close; the exact runner stays retained for re-admission.
    fn cancel_close_retry(&self) -> bool {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Scheduled | ArtifactCloseRetryState::Admitted) {
            return false;
        }
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Cancelled;
        true
    }

    /// 🏁️ The runner's terminal transition: surrenders the handoff's pool use, publishes `Terminal`, then wakes a committed
    /// retirement cursor. Its last turn may have found the runner still closing and gone idle, and a terminal runner makes
    /// no further progress that would wake it — a stranded cursor keeps its `WorkerPoolUse` and a database shutdown waits in
    /// its `PoolUse` phase until the deadline.
    fn enter_terminal(&self) {
        if self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
        self.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
        self.terminal.store(true, std::sync::atomic::Ordering::Release);
        self.request_retirement_maintenance();
    }

    fn request_retirement_maintenance(&self) {
        let owner = self.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        if let Some((pool, ticket)) = owner {
            let _ = pool.request_maintenance(ticket);
        }
    }
}

/// 📏️ Live document authorities one process holds at once: each reserves a retirement slot when it spawns, so dropping
/// it can always hand its runner to a retirement cursor. A slot is a fixed cell, never a pool maintenance hook — the pool
/// has a few dozen of those in all, and every authority used to take one, which capped a hub at about 60 open documents.
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETIREMENT_SLOTS: usize = 4_096;

/// 🪝️ Worker pools that retire artifact runners at once: each gets ONE maintenance hook, shared by every retirement slot
/// reserved on it, installed with its first reservation and retired with its last.
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETIREMENT_POOLS: usize = 256;

/// 🔑️ One pool's shared retirement hook: its row, the row's generation and the pool's maintenance ticket.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
struct ArtifactRunnerRetirementHookKey {
    pub row: usize,
    pub generation: u64,
    ticket: semio_framework_async::WorkerMaintenanceTicket,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerRetirementHook {
    pub generation: u64,
    pool: Arc<semio_framework_async::WorkerPool>,
    ticket: semio_framework_async::WorkerMaintenanceTicket,
    pub slots: usize,
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENT_HOOKS: std::sync::Mutex<[Option<ArtifactRunnerRetirementHook>; ARTIFACT_RUNNER_RETIREMENT_POOLS]> = std::sync::Mutex::new([const { None }; ARTIFACT_RUNNER_RETIREMENT_POOLS]);

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerRetirementCursor {
    pub generation: u64,
    pub close: Arc<dyn Fn() -> bool + Send + Sync>,
    handoff: Arc<ArtifactRunnerHandoff>,
    pool: Arc<semio_framework_async::WorkerPool>,
    _pool_use: Arc<semio_framework_async::WorkerPoolUse>,
    hook: ArtifactRunnerRetirementHookKey,
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENTS: [std::sync::Mutex<Option<ArtifactRunnerRetirementCursor>>; ARTIFACT_RUNNER_RETIREMENT_SLOTS] = [const { std::sync::Mutex::new(None) }; ARTIFACT_RUNNER_RETIREMENT_SLOTS];

/// 🔬️ How many retained artifact-runner retirement cursors still hold the `WorkerPoolUse` clone a
/// non-terminal `ArtifactAuthority::drop` handed them. A database shutdown stuck in its `PoolUse`
/// phase with every other registry empty is blocked by exactly these.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_runner_retirement_live_slots() -> usize {
    ARTIFACT_RUNNER_RETIREMENTS.iter().filter(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()).count()
}

/// 📈️ The most advanced faulted-close retry among the dropped authorities retiring on `pool`: a blocked
/// (exhausted or cancelled) owner wins over one still retrying, so a shutdown can report exactly why
/// its pool use is still retained.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_progress(pool: &Arc<semio_framework_async::WorkerPool>) -> Option<ArtifactCloseRetryProgress> {
    ARTIFACT_RUNNER_RETIREMENTS
        .iter()
        .filter_map(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().filter(|owner| Arc::ptr_eq(&owner.pool, pool)).map(|owner| owner.handoff.clone()))
        .filter_map(|handoff| handoff.close_retry_progress())
        .max_by_key(|progress| (progress.is_blocked(), progress.attempts))
}

/// 🔂️ Re-admits every exhausted or cancelled faulted close retiring on `pool`; returns how many were re-admitted.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_readmit(pool: &Arc<semio_framework_async::WorkerPool>) -> usize {
    artifact_close_retry_handoffs(pool).into_iter().filter(|handoff| handoff.readmit_close_retry()).count()
}

/// 🛑️ Cancels every scheduled faulted-close retry retiring on `pool`; returns how many were cancelled.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_cancel(pool: &Arc<semio_framework_async::WorkerPool>) -> usize {
    artifact_close_retry_handoffs(pool).into_iter().filter(|handoff| handoff.cancel_close_retry()).count()
}

#[cfg(not(target_arch = "wasm32"))]
fn artifact_close_retry_handoffs(pool: &Arc<semio_framework_async::WorkerPool>) -> Vec<Arc<ArtifactRunnerHandoff>> {
    ARTIFACT_RUNNER_RETIREMENTS.iter().filter_map(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().filter(|owner| Arc::ptr_eq(&owner.pool, pool)).map(|owner| owner.handoff.clone())).collect()
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_AUTHORITY_POOL_USES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many LIVE `ArtifactAuthority` owners still hold the `WorkerPoolUse` clone
/// `spawn_with_pool_use` stored in `_pool_use`. A handoff surrenders its own clone at the runner's
/// terminal transition, so `artifact_runner_handoff_pool_use_live_slots() == 0` does NOT exclude a
/// live authority: the authority outlives its runner's terminal transition and keeps a second
/// clone until the authority itself is dropped.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_authority_live_pool_uses() -> usize {
    ARTIFACT_AUTHORITY_POOL_USES.load(std::sync::atomic::Ordering::Acquire)
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_HANDOFF_POOL_USES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many LIVE artifact-runner handoffs still park the `WorkerPoolUse` clone
/// `spawn_with_pool_use` gave them. A handoff surrenders it only at the runner's terminal
/// transition, so an authority already gone from `open_artifacts` — and holding no retirement
/// cursor — can still pin the database's pool use through a runner that never went terminal. This
/// is the family no registry census can see, because handoffs live in their runner, not in a slot
/// table.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_runner_handoff_pool_use_live_slots() -> usize {
    ARTIFACT_RUNNER_HANDOFF_POOL_USES.load(std::sync::atomic::Ordering::Acquire)
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENT_GENERATIONS: [std::sync::atomic::AtomicU64; ARTIFACT_RUNNER_RETIREMENT_SLOTS] = [const { std::sync::atomic::AtomicU64::new(0) }; ARTIFACT_RUNNER_RETIREMENT_SLOTS];

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENT_NEXT_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerRetirementReservation {
    pub index: usize,
    pub generation: u64,
    pool: Arc<semio_framework_async::WorkerPool>,
    hook: Option<ArtifactRunnerRetirementHookKey>,
}

/// 🪝️ Joins `pool`'s shared retirement hook, installing it for the pool's first reservation.
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_hook_join(pool: &Arc<semio_framework_async::WorkerPool>, generation: u64) -> Result<ArtifactRunnerRetirementHookKey, DbError> {
    let mut hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((row, hook)) = hooks.iter_mut().enumerate().find_map(|(row, hook)| hook.as_mut().filter(|hook| Arc::ptr_eq(&hook.pool, pool)).map(|hook| (row, hook))) {
        hook.slots += 1;
        return Ok(ArtifactRunnerRetirementHookKey { row, generation: hook.generation, ticket: hook.ticket });
    }
    let row = hooks.iter().position(Option::is_none).ok_or_else(|| DbError::Unavailable("artifact runner retirement pool capacity exhausted".to_string()))?;
    let ticket = pool
        .install_maintenance_hook(semio_framework_async::Lane::UserVisible, artifact_runner_retirement_step, [row as u64, generation])
        .map_err(|error| DbError::Unavailable(format!("artifact runner retirement maintenance admission: {error:?}")))?;
    hooks[row] = Some(ArtifactRunnerRetirementHook { generation, pool: pool.clone(), ticket, slots: 1 });
    Ok(ArtifactRunnerRetirementHookKey { row, generation, ticket })
}

/// 🪝️ One retirement slot of the hook's pool was released: the pool's last one removes the hook's row (the hook itself
/// retires on its next turn, or at once when it is not running).
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_hook_leave(key: ArtifactRunnerRetirementHookKey) {
    let mut hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(hook) = hooks[key.row].as_mut().filter(|hook| hook.generation == key.generation) else { return };
    hook.slots -= 1;
    if hook.slots != 0 {
        return;
    }
    let retired = hooks[key.row].take().expect("checked artifact runner retirement hook");
    drop(hooks);
    let _ = retired.pool.remove_maintenance_hook(retired.ticket);
}

/// 🚥️ How one retirement slot's turn ended.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactRunnerRetirementTurn {
    Absent,
    Waiting,
    Faulted,
    Retired,
}

/// 🧹️ One turn of a pool's shared retirement hook: every slot of that pool whose authority was dropped before its runner
/// went terminal advances one close step. A reservation whose cursor is not committed yet is skipped — its commit requests
/// the hook again — so an early turn can never strand a cursor; the hook retires once its pool's last slot is released.
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_step([row, generation]: [u64; 2]) -> semio_framework_async::WorkerMaintenanceStep {
    let Ok(row) = usize::try_from(row) else { return semio_framework_async::WorkerMaintenanceStep::Retire };
    let pool = {
        let hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match hooks.get(row).and_then(Option::as_ref) {
            Some(hook) if hook.generation == generation => hook.pool.clone(),
            _ => return semio_framework_async::WorkerMaintenanceStep::Retire,
        }
    };
    let mut faulted = false;
    for index in 0..ARTIFACT_RUNNER_RETIREMENT_SLOTS {
        if ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == 0 {
            continue;
        }
        faulted |= artifact_runner_retirement_slot_turn(index, &pool) == ArtifactRunnerRetirementTurn::Faulted;
    }
    let hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    match hooks.get(row).and_then(Option::as_ref) {
        Some(hook) if hook.generation == generation && faulted => semio_framework_async::WorkerMaintenanceStep::Fault,
        Some(hook) if hook.generation == generation => semio_framework_async::WorkerMaintenanceStep::Idle,
        _ => semio_framework_async::WorkerMaintenanceStep::Retire,
    }
}

/// 🧹️ One close step of the cursor in slot `index` when it retires on `pool`: `Absent` for a free, uncommitted or
/// foreign slot, `Waiting` while the runner still closes, `Faulted` for a panicking or blocked close (the exact cursor
/// stays), `Retired` once terminal — the slot and its share of the pool's hook are released.
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_slot_turn(index: usize, pool: &Arc<semio_framework_async::WorkerPool>) -> ArtifactRunnerRetirementTurn {
    let slot = &ARTIFACT_RUNNER_RETIREMENTS[index];
    let mut row = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !row.as_ref().is_some_and(|owner| Arc::ptr_eq(&owner.pool, pool)) {
        return ArtifactRunnerRetirementTurn::Absent;
    }
    let mut cursor = row.take();
    drop(row);
    let owner = cursor.as_mut().expect("checked artifact runner retirement owner");
    #[cfg(test)]
    owner.handoff.retirement_turns.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    let terminal = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (owner.close)())) {
        Ok(terminal) => terminal,
        Err(_) => {
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = cursor;
            return ArtifactRunnerRetirementTurn::Faulted;
        }
    };
    if !terminal {
        let blocked = owner.handoff.close_retry_progress().is_some_and(|progress| progress.is_blocked());
        *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = cursor;
        return if blocked { ArtifactRunnerRetirementTurn::Faulted } else { ArtifactRunnerRetirementTurn::Waiting };
    }
    owner.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    owner.handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    let hook = owner.hook;
    drop(cursor);
    ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].store(0, std::sync::atomic::Ordering::Release);
    artifact_runner_retirement_hook_leave(hook);
    ArtifactRunnerRetirementTurn::Retired
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerRetirementReservation {
    pub fn try_reserve(pool: Arc<semio_framework_async::WorkerPool>) -> Result<Self, DbError> {
        let generation = ARTIFACT_RUNNER_RETIREMENT_NEXT_GENERATION
            .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0))
            .map_err(|_| DbError::LimitExceeded("artifact runner retirement generation"))?;
        for (index, slot) in ARTIFACT_RUNNER_RETIREMENT_GENERATIONS.iter().enumerate() {
            if slot.compare_exchange(0, generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                continue;
            }
            let hook = match artifact_runner_retirement_hook_join(&pool, generation) {
                Ok(hook) => hook,
                Err(error) => {
                    slot.store(0, std::sync::atomic::Ordering::Release);
                    return Err(error);
                }
            };
            return Ok(Self { index, generation, pool, hook: Some(hook) });
        }
        Err(DbError::Unavailable("artifact runner retirement capacity exhausted".to_string()))
    }

    pub fn commit(mut self, close: Arc<dyn Fn() -> bool + Send + Sync>, handoff: Arc<ArtifactRunnerHandoff>, pool_use: Arc<semio_framework_async::WorkerPoolUse>) {
        let hook = self.hook.take().expect("artifact runner retirement reservation lost its hook");
        *ARTIFACT_RUNNER_RETIREMENTS[self.index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactRunnerRetirementCursor { generation: self.generation, close, handoff: handoff.clone(), pool: self.pool.clone(), _pool_use: pool_use, hook });
        *handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((self.pool.clone(), hook.ticket));
        let _ = self.pool.request_maintenance(hook.ticket);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerRetirementReservation {
    fn drop(&mut self) {
        let Some(hook) = self.hook.take() else { return };
        ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[self.index].store(0, std::sync::atomic::Ordering::Release);
        artifact_runner_retirement_hook_leave(hook);
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerClosePoll {
    handoff: Arc<ArtifactRunnerHandoff>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerClosePoll {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        let faulted = self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver != ArtifactRunnerDriver::ClosingPolling as u8 && driver != ArtifactRunnerDriver::ClosingPollingWake as u8 {
                return;
            }
            let next = if driver == ArtifactRunnerDriver::ClosingPollingWake as u8 && !faulted { ArtifactRunnerDriver::ClosingReady } else { ArtifactRunnerDriver::ClosingParked };
            if self.handoff.driver.compare_exchange(driver, next as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                if next == ArtifactRunnerDriver::ClosingReady {
                    self.handoff.request_retirement_maintenance();
                }
                return;
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[must_use = "terminal artifact runner jobs retain the only retry or close authority"]
pub struct ArtifactRunnerTerminalJob {
    pub handoff: Arc<ArtifactRunnerHandoff>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunner {
    pool: Arc<semio_framework_async::WorkerPool>,
    address: db_actor::Address<ArtifactMessage>,
    receiver: db_actor::Receiver<ArtifactMessage>,
    pub generation: u64,
    pub builder: std::sync::Mutex<Option<ArtifactBuildFuture>>,
    pub engine: std::sync::Mutex<Option<Box<ArtifactEngine>>>,
    pub turn: std::sync::Mutex<Option<ArtifactTurn>>,
    ready: std::sync::Mutex<Option<db_actor::ReplySender<Result<(), ArtifactEngineOpenRejected>>>>,
    done: std::sync::Mutex<Option<db_actor::ReplySender<()>>>,
    handoff: Arc<ArtifactRunnerHandoff>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    pub cancelled: std::sync::atomic::AtomicBool,
    close_driving: std::sync::atomic::AtomicBool,
    pub terminal: std::sync::atomic::AtomicBool,
}

/// ⏱️ How long one worker turn of a document authority keeps re-polling a self-woken turn
/// future before it hands the worker back to the pool.
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_TURN_MICROS: u64 = 2_000;

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerPoll {
    runner: Arc<ArtifactRunner>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerPoll {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        loop {
            let driver = self.runner.handoff.driver.load(Ordering::Acquire);
            let cancelled = self.runner.cancelled.load(Ordering::Acquire);
            let faulted = self.runner.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
            let next = match (driver, cancelled) {
                (value, true) if faulted && (value == ArtifactRunnerDriver::Polling as u8 || value == ArtifactRunnerDriver::PollingWake as u8) => ArtifactRunnerDriver::ClosingParked,
                (value, true) if value == ArtifactRunnerDriver::Polling as u8 || value == ArtifactRunnerDriver::PollingWake as u8 => ArtifactRunnerDriver::ClosingReady,
                (value, false) if value == ArtifactRunnerDriver::Polling as u8 => ArtifactRunnerDriver::RunnableIdle,
                (value, false) if value == ArtifactRunnerDriver::PollingWake as u8 => ArtifactRunnerDriver::Queued,
                _ => return,
            };
            if self.runner.handoff.driver.compare_exchange(driver, next as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                if next == ArtifactRunnerDriver::Queued {
                    self.runner.submit_scheduled();
                } else if next == ArtifactRunnerDriver::ClosingReady {
                    self.runner.handoff.request_retirement_maintenance();
                }
                return;
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerWake {
    runner: std::sync::Weak<ArtifactRunner>,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ArtifactRunnerWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            if self.generation == runner.generation {
                runner.schedule();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunner {
    fn close_one(self: &Arc<Self>) -> bool {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        if !self.handoff.close_poll_admitted() {
            return self.terminal.load(Ordering::Acquire);
        }
        self.close_driving.store(true, Ordering::Release);
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver == ArtifactRunnerDriver::Terminal as u8
                || driver == ArtifactRunnerDriver::Polling as u8
                || driver == ArtifactRunnerDriver::PollingWake as u8
                || driver == ArtifactRunnerDriver::ClosingReady as u8
                || driver == ArtifactRunnerDriver::ClosingPolling as u8
                || driver == ArtifactRunnerDriver::ClosingPollingWake as u8
            {
                break;
            }
            if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break;
            }
        }
        let generation = self.generation;
        let witness = self.clone();
        self.clone().run_turn(generation, true);
        let closed = witness.terminal.load(Ordering::Acquire);
        witness.close_driving.store(false, Ordering::Release);
        closed
    }

    fn retained_terminal_job(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind) -> (semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job) {
        let runner = self.clone();
        let generation = self.generation;
        (kind, Box::new(move || runner.run_turn(generation, false)))
    }

    fn park_terminal_job(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind) {
        use std::sync::atomic::Ordering;
        let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_some() {
            return;
        }
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver != ArtifactRunnerDriver::RunnableIdle as u8 && driver != ArtifactRunnerDriver::Queued as u8 {
                return;
            }
            if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::Parked as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                *terminal = Some(self.retained_terminal_job(kind));
                return;
            }
        }
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver == ArtifactRunnerDriver::RunnableIdle as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    self.submit_scheduled();
                    return;
                }
            } else if driver == ArtifactRunnerDriver::Polling as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::PollingWake as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    return;
                }
            } else if driver == ArtifactRunnerDriver::ClosingPolling as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingPollingWake as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    return;
                }
            } else if driver == ArtifactRunnerDriver::ClosingParked as u8 {
                if self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                    return;
                }
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    self.handoff.request_retirement_maintenance();
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn submit_scheduled(self: &Arc<Self>) {
        let runner = Arc::downgrade(self);
        let generation = self.generation;
        self.submit_exact(
            Box::new(move || {
                if let Some(runner) = runner.upgrade() {
                    if runner.generation == generation {
                        runner.run_turn(generation, false);
                    }
                }
            }),
            0,
        );
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_RUNNER_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        drop(job);
                        ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(format!("artifact authority WorkerPool submission failed: {kind:?}")))));
                        self.finish();
                    } else {
                        drop(job);
                        self.park_terminal_job(kind);
                    }
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact runner retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact runner retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_RUNNER_RETRY_MS) else {
            self.terminalize_retry_authority("artifact runner retry deadline exhausted");
            return;
        };
        let runner = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != runner.retry_generation.load(Ordering::Acquire) {
                return;
            }
            runner.retry_armed.store(false, Ordering::Release);
            let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if runner.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    runner.park_terminal_job(semio_framework_async::WorkerSubmitErrorKind::Saturated);
                } else {
                    runner.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(self: &Arc<Self>, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            if self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                drop(job);
                self.park_terminal_job(semio_framework_async::WorkerSubmitErrorKind::Saturated);
            } else {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(detail.to_string()))));
            self.finish();
        }
    }

    fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    pub fn finish(self: &Arc<Self>) {
        if self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|turn| matches!(turn, ArtifactTurn::History { replay, .. } if !replay.terminal_is_empty())) {
            return;
        }
        {
            let mut engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if engine.as_ref().is_some_and(|owner| owner.close_flush_pending()) {
                let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if turn.is_none() {
                    let mut owner = engine.take().expect("checked artifact engine close-flush owner");
                    drop(engine);
                    *turn = Some(ArtifactTurn::CloseFlush(Box::pin(async move {
                        let _ = owner.close_flush().await;
                        owner
                    })));
                    drop(turn);
                    self.schedule();
                    return;
                }
            }
            if let Some(owner) = engine.as_mut() {
                #[cfg(test)]
                {
                    self.handoff.close_polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    if self.handoff.close_faults.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |faults| faults.checked_sub(1)).is_ok() {
                        drop(engine);
                        self.record_close_fault(DbError::Io("injected artifact engine close fault".to_string()));
                        return;
                    }
                }
                let waker = std::task::Waker::from(Arc::new(ArtifactRunnerWake { runner: Arc::downgrade(self), generation: self.generation }));
                match owner.poll_close(&mut std::task::Context::from_waker(&waker)) {
                    std::task::Poll::Pending => {
                        drop(engine);
                        self.clear_close_fault();
                        return;
                    }
                    std::task::Poll::Ready(Ok(())) => {
                        engine.take();
                        self.clear_close_fault();
                    }
                    std::task::Poll::Ready(Err(error)) => {
                        drop(engine);
                        self.record_close_fault(error);
                        return;
                    }
                }
            }
        }
        if !self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if builder.is_none() {
                let turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                drop(turn);
            }
            self.handoff.enter_terminal();
            if let Some(done) = self.done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                done.send(());
            }
        }
    }

    fn record_close_fault(self: &Arc<Self>, error: DbError) {
        *self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
        self.handoff.arm_close_retry();
    }

    fn clear_close_fault(&self) {
        if self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            self.handoff.clear_close_retry();
        }
    }

    fn start_turn(engine: Box<ArtifactEngine>, message: ArtifactMessage) -> ArtifactTurn {
        match message {
            ArtifactMessage::History { operation_generation, cancelled, reservation, reply } => {
                let replay = engine.history_replay(operation_generation, cancelled, reservation);
                ArtifactTurn::History { engine: Some(engine), replay, reply: Some(reply) }
            }
            // 🪜️ One boxed coroutine PER message kind, never one coroutine with a ten-arm `match`
            // inside it. A coroutine's frame is the SUM of its suspend points' live locals, and a
            // debug build overlaps nothing: with all ten engine methods awaited inside a single
            // async block, `start_turn::{closure}` reserved **10,711,040 B** and the runner entered
            // it twice, which is 20.4 MiB of the 21,520,944 B that aborted three hub laws
            // (ticket 26/09/18 slice HS1, `🐍️hs1-frame-sizes.py` over the nextest crash reports).
            // Splitting the match OUT of the async block turns that sum into a max: each arm's
            // coroutine holds exactly one engine-method future, and `ArtifactTurnFuture` erases them
            // all to the same type, so nothing downstream changes.
            ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.append_durable_group_decision(record, cancelled, now_ms).await);
                engine
            })),
            ArtifactMessage::RecoverDurableGroupDecisions { driver, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.recover_durable_group_decisions(driver).await);
                engine
            })),
            ArtifactMessage::Submit { batch, options, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.submit(batch, options, now_ms).await);
                engine.park_writer();
                let _ = engine.maintain_index().await;
                engine
            })),
            ArtifactMessage::Query { path, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.get(&path).await);
                engine
            })),
            ArtifactMessage::Frontier { reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.frontier().await);
                engine
            })),
            ArtifactMessage::CheckpointPublicationSnapshot { reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.checkpoint_publication_snapshot().await);
                engine
            })),
            ArtifactMessage::RunQuery { query, consistency, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.query(query, consistency).await);
                engine
            })),
            ArtifactMessage::SnapshotNow { now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.snapshot_now(now_ms).await);
                engine
            })),
            ArtifactMessage::Compact { holder, consolidate_snapshots, budget, now_ms, cancelled, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled).await);
                engine
            })),
        }
    }

    /// 🔁️ Keeps polling in this worker turn when the future woke itself while it was being
    /// polled (a cooperative yield, a storage step that completed inline) and the turn budget
    /// remains: every such wake used to cost a full worker-pool round trip.
    fn repoll_within_turn(&self, turn_ends: std::time::Instant) -> bool {
        use std::sync::atomic::Ordering;
        std::time::Instant::now() < turn_ends
            && !self.cancelled.load(Ordering::Acquire)
            && self.handoff.driver.compare_exchange(ArtifactRunnerDriver::PollingWake as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    fn run_turn(self: Arc<Self>, generation: u64, direct_close: bool) {
        use std::panic::AssertUnwindSafe;
        use std::sync::atomic::Ordering;

        if generation != self.generation || self.terminal.load(Ordering::Acquire) {
            return;
        }
        let (_close_poll, _poll) = if direct_close {
            if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return;
            }
            (Some(ArtifactRunnerClosePoll { handoff: self.handoff.clone() }), None)
        } else if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Queued as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            (None, Some(ArtifactRunnerPoll { runner: self.clone() }))
        } else {
            return;
        };
        if self.cancelled.load(Ordering::Acquire) {
            let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match turn.as_mut() {
                Some(ArtifactTurn::History { replay, .. }) => replay.request_close(DbError::Closed),
                Some(ArtifactTurn::CloseFlush(_)) => {}
                _ => {
                    drop(turn);
                    self.finish();
                    return;
                }
            }
        }

        let waker = std::task::Waker::from(Arc::new(ArtifactRunnerWake { runner: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        let turn_ends = std::time::Instant::now() + std::time::Duration::from_micros(ARTIFACT_RUNNER_TURN_MICROS);

        let mut builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(future) = builder.as_mut() {
            let polled = loop {
                let polled = std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
                if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                    break polled;
                }
            };
            match polled {
                Ok(std::task::Poll::Pending) => return,
                Ok(std::task::Poll::Ready(Ok(engine))) => {
                    builder.take();
                    drop(builder);
                    engine.park_writer();
                    *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Ok(()));
                    }
                    if self.address.has_messages() {
                        self.schedule();
                    }
                    return;
                }
                Ok(std::task::Poll::Ready(Err(error))) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(error));
                    }
                    self.finish();
                    return;
                }
                Err(_) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Internal("document authority construction panicked".to_string()))));
                    }
                    self.finish();
                    return;
                }
            }
        }
        drop(builder);

        let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(active) = turn.as_mut() {
            match active {
                ArtifactTurn::Future(future) => match loop {
                    let polled = std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
                    if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                        break polled;
                    }
                } {
                    Ok(std::task::Poll::Pending) => return,
                    Ok(std::task::Poll::Ready(engine)) => {
                        turn.take();
                        drop(turn);
                        engine.park_writer();
                        *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                            self.finish();
                        } else if self.address.has_messages() {
                            self.schedule();
                        }
                        return;
                    }
                    Err(_) => {
                        turn.take();
                        drop(turn);
                        self.address.close();
                        self.finish();
                        return;
                    }
                },
                ArtifactTurn::CloseFlush(future) => match std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))) {
                    Ok(std::task::Poll::Pending) => return,
                    Ok(std::task::Poll::Ready(engine)) => {
                        turn.take();
                        drop(turn);
                        *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                        self.finish();
                        return;
                    }
                    Err(_) => {
                        turn.take();
                        drop(turn);
                        self.address.close();
                        self.finish();
                        return;
                    }
                },
                ArtifactTurn::History { engine, replay, reply } => {
                    if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                        replay.request_close(DbError::Closed);
                    }
                    match loop {
                        let polled = std::panic::catch_unwind(AssertUnwindSafe(|| Pin::new(&mut *replay).poll(&mut context)));
                        if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                            break polled;
                        }
                    } {
                        Ok(std::task::Poll::Pending) => return,
                        Ok(std::task::Poll::Ready(result)) => {
                            let engine = engine.take();
                            if let Some(reply) = reply.take() {
                                reply.send(result);
                            }
                            let replay_empty = replay.terminal_is_empty();
                            if replay_empty {
                                self.handoff.active_history.store(false, Ordering::Release);
                                turn.take();
                            }
                            drop(turn);
                            if let Some(engine) = engine {
                                *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                            }
                            if !replay_empty {
                                self.schedule();
                            } else if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                                self.finish();
                            } else if self.address.has_messages() {
                                self.schedule();
                            }
                            return;
                        }
                        Err(_) => {
                            replay.request_close(DbError::Internal("history replay cursor panicked".to_string()));
                            drop(turn);
                            self.address.close();
                            self.schedule();
                            return;
                        }
                    }
                }
            }
        }
        drop(turn);

        if let Some(envelope) = self.receiver.try_recv() {
            let engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(engine) = engine {
                let turn = Self::start_turn(engine, envelope.payload);
                self.handoff.active_history.store(matches!(&turn, ArtifactTurn::History { .. }), Ordering::Release);
                *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(turn);
                self.schedule();
            } else {
                self.address.close();
                self.finish();
            }
            return;
        }
        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
            self.finish();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactAuthority {
    /// 🚀️ Builds the engine on the injected pool and resolves only after construction, so
    /// a caller never receives an authority whose engine failed to open.
    pub async fn spawn<F: Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        build: impl FnOnce() -> F + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, ArtifactEngineOpenRejected> {
        let pool_use = pool.acquire_use().map_err(|error| ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(format!("artifact authority WorkerPool use rejected: {error:?}"))))?;
        Self::spawn_with_pool_use(pool, pool_use, build, capacities).await
    }

    /// 🧵️ Mounts an authority under an already-retained process-pool use. Database document
    /// mounts pass their exact use cell through this boundary, so no second lifecycle admission
    /// can conflict after the catalog transaction has begun.
    pub async fn spawn_with_pool_use<F: Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        pool_use: Arc<semio_framework_async::WorkerPoolUse>,
        build: impl FnOnce() -> F + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, ArtifactEngineOpenRejected> {
        let retirement = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).map_err(ArtifactEngineOpenRejected::BeforeWal)?;
        let (address, receiver) = db_actor::mailbox::<ArtifactMessage>(capacities);
        let (ready_tx, ready_rx) = db_actor::oneshot::<Result<(), ArtifactEngineOpenRejected>>();
        let (done_tx, done_rx) = db_actor::oneshot();
        ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let handoff = Arc::new(ArtifactRunnerHandoff {
            pool: pool.clone(),
            pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
            terminal_job: std::sync::Mutex::new(None),
            close_runner: std::sync::Mutex::new(None),
            retirement_maintenance: std::sync::Mutex::new(None),
            close_error: std::sync::Mutex::new(None),
            close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
            #[cfg(test)]
            close_faults: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            close_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            retirement_turns: std::sync::atomic::AtomicUsize::new(0),
            active_history: std::sync::atomic::AtomicBool::new(false),
            driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::RunnableIdle as u8),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let runner = Arc::new(ArtifactRunner {
            pool,
            address: address.clone(),
            receiver,
            generation: address.generation().0,
            builder: std::sync::Mutex::new(Some(Box::pin(build()))),
            engine: std::sync::Mutex::new(None),
            turn: std::sync::Mutex::new(None),
            ready: std::sync::Mutex::new(Some(ready_tx)),
            done: std::sync::Mutex::new(Some(done_tx)),
            handoff: handoff.clone(),
            retry_job: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            close_driving: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let weak = Arc::downgrade(&runner);
        address.set_consumer_wake(Arc::new(move || {
            if let Some(runner) = weak.upgrade() {
                runner.schedule();
            }
        }));
        let weak = Arc::downgrade(&runner);
        *handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || -> bool { weak.upgrade().is_none_or(|runner| runner.close_one()) }));
        let retirement_runner = runner.clone();
        let retirement_close: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(move || retirement_runner.close_one());
        let runner_for_cancel = runner.clone();
        let cancel: Arc<dyn Fn() + Send + Sync> = Arc::new(move || runner_for_cancel.cancel());
        runner.schedule();

        match ready_rx.await {
            Ok(Ok(())) => {
                ARTIFACT_AUTHORITY_POOL_USES.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                Ok(ArtifactAuthority { address, cancel, handoff, retirement: Some(retirement), retirement_close: Some(retirement_close), _pool_use: pool_use, done: std::sync::Mutex::new(Some(done_rx)) })
            }
            Ok(Err(err)) => Err(err),
            Err(_) => Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Closed)),
        }
    }

    /// 📓️ Nonblocking retained fixed-three decision append through this authority's
    /// existing WAL owner. The opaque record can only be constructed by Store admission.
    pub fn append_durable_group_decision_retained(
        &self,
        record: store::durable_group::DurableOwnedGroupJournalRecordV1,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        now_ms: u64,
    ) -> db_actor::AskFuture<ArtifactMessage, Result<ArtifactDurableGroupJournalAppendV1, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms, reply })
    }

    /// 🔐️ Builds the Store journal port over this authority's clone-cheap mailbox address;
    /// it never acquires or opens another WAL writer.
    pub fn durable_group_journal_sink(&self, now_ms: u64) -> Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1> {
        Box::new(ArtifactDurableGroupJournalSinkV1 { address: self.address.clone(), now_ms })
    }

    /// 🧭 Starts one actor-serialized committed-Event replay while the returned owner retains
    /// every exact Store across caller cancellation.
    pub fn durable_group_recovery_retained<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        document: ArtifactId,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        let (mut owner, driver) = ArtifactDurableGroupRecoveryOwnerV1::new(document, admission);
        owner.request = Some(Box::pin(self.address.ask(Priority::Command, |reply| ArtifactMessage::RecoverDurableGroupDecisions { driver, reply })));
        owner
    }

    pub fn resume_durable_group_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        owner: &mut ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> Result<(), DbError>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        if owner.request.is_some() || owner.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Conflict("durable group recovery scan is already active or complete".to_string()));
        }
        let driver = owner.driver();
        owner.request = Some(Box::pin(self.address.ask(Priority::Command, |reply| ArtifactMessage::RecoverDurableGroupDecisions { driver, reply })));
        Ok(())
    }

    /// 📨️ Nonblocking retained submit cursor used by `db_engine::SubmitFuture`.
    pub fn submit_retained(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> db_actor::AskFuture<ArtifactMessage, Result<CommandReceipt, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply })
    }

    pub fn history_retained(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> db_actor::AskFuture<ArtifactMessage, Result<ArtifactHistoryView, DbError>> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::History { operation_generation, cancelled, reservation, reply })
    }

    pub fn compact_retained(
        &self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> db_actor::AskFuture<ArtifactMessage, Result<db_compact::CompactionReport, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Compact { holder, consolidate_snapshots, budget, now_ms, cancelled, reply })
    }

    pub fn generation(&self) -> GenerationId {
        self.address.generation()
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactRunnerTerminalJob> {
        if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::Parked as u8 {
            return None;
        }
        self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactRunnerTerminalJob { handoff: self.handoff.clone(), owner: Some(owner) })
    }

    pub fn close_step(&self) -> bool {
        let active = self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire);
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        if !active
            && driver != ArtifactRunnerDriver::Parked as u8
            && driver != ArtifactRunnerDriver::ClosingReady as u8
            && driver != ArtifactRunnerDriver::ClosingPolling as u8
            && driver != ArtifactRunnerDriver::ClosingPollingWake as u8
            && driver != ArtifactRunnerDriver::ClosingParked as u8
            && self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
        {
            return false;
        }
        let close = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let closed = close.as_ref().is_none_or(|close| close());
        let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            return active;
        }
        if closed {
            let owner = terminal.take();
            drop(owner);
        }
        true
    }

    /// 📈️ Progress of this authority's faulted close retry, `None` while its close is healthy.
    pub fn close_retry_progress(&self) -> Option<ArtifactCloseRetryProgress> {
        self.handoff.close_retry_progress()
    }

    /// 🔂️ Grants an exhausted or cancelled faulted close a fresh bounded retry budget.
    pub fn readmit_close_retry(&self) -> bool {
        self.handoff.readmit_close_retry()
    }

    /// 🛑️ Stops the automatic retries of a faulted close; the runner stays retained for re-admission.
    pub fn cancel_close_retry(&self) -> bool {
        self.handoff.cancel_close_retry()
    }

    pub fn terminal_is_empty(&self) -> bool {
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        driver != ArtifactRunnerDriver::Parked as u8
            && driver != ArtifactRunnerDriver::ClosingReady as u8
            && driver != ArtifactRunnerDriver::ClosingPolling as u8
            && driver != ArtifactRunnerDriver::ClosingPollingWake as u8
            && driver != ArtifactRunnerDriver::ClosingParked as u8
            && self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire)
    }

    pub async fn submit(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply }).await?
    }

    pub async fn query(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let path = path.to_string();
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Query { path, reply }).await?
    }

    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Frontier { reply }).await
    }

    /// 📍️ Captures one checkpoint-publication snapshot through the document actor.
    pub async fn checkpoint_publication_snapshot(&self) -> Result<CheckpointPublicationSnapshot, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::CheckpointPublicationSnapshot { reply }).await
    }

    pub async fn run_query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::RunQuery { query, consistency, reply }).await?
    }

    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::SnapshotNow { now_ms, reply }).await?
    }

    pub async fn compact(&self, holder: db_io_core::DbIoText, consolidate_snapshots: bool, budget: db_compact::CompactionBudget, now_ms: u64, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<db_compact::CompactionReport, DbError> {
        self.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled).await?
    }

    /// 🚪️ Requests closure and advances one finite retained runner turn. The authority remains
    /// caller-owned until the returned terminal acknowledgement is `true`.
    /// 🏁️ Takes the one signal that resolves once this authority's runner is terminal — its WAL
    /// closed and its writer released — so a successor mount of the same document can wait for it.
    pub fn take_terminal_signal(&self) -> Option<db_actor::ReplyReceiver<()>> {
        self.done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub fn shutdown_step(&self) -> bool {
        self.address.close();
        (self.cancel)();
        self.close_step();
        self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn shutdown_debug_witness(&self) -> String {
        let close_error = self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let close_retry = self.handoff.close_retry_progress();
        format!(
            "driver={} terminal={} active_history={} terminal_job={} retirement_maintenance={} runner_pool_use={} close_error={:?} close_retry={:?}",
            self.handoff.driver.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            self.handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            self.handoff.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            close_error,
            close_retry,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactAuthority {
    fn drop(&mut self) {
        ARTIFACT_AUTHORITY_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        if self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.address.close();
        (self.cancel)();
        let Some(retirement) = self.retirement.take() else { return };
        let Some(close) = self.retirement_close.take() else { return };
        retirement.commit(close, self.handoff.clone(), self._pool_use.clone());
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact runner job already resolved").0
    }

    pub fn resume(mut self) -> Result<(), Self> {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Parked as u8, ArtifactRunnerDriver::Queued as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) == ArtifactRunnerDriver::Terminal as u8 {
                return Ok(());
            }
            self.owner = Some(owner);
            return Err(self);
        }
        match self.handoff.pool.try_submit(semio_framework_async::Lane::UserVisible, owner.1) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.handoff.driver.store(ArtifactRunnerDriver::Parked as u8, std::sync::atomic::Ordering::Release);
                self.owner = Some((error.kind(), error.into_job()));
                Err(self)
            }
        }
    }

    pub fn close(mut self) -> Result<(), Self> {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        if driver == ArtifactRunnerDriver::Parked as u8 {
            if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Parked as u8, ArtifactRunnerDriver::ClosingReady as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                self.owner = Some(owner);
                return Err(self);
            }
        } else if driver == ArtifactRunnerDriver::Terminal as u8 {
            return Ok(());
        } else if driver != ArtifactRunnerDriver::ClosingReady as u8 {
            self.owner = Some(owner);
            return Err(self);
        }
        let close = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let closed = close.as_ref().is_none_or(|close| close());
        if closed {
            self.handoff.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
            Ok(())
        } else {
            self.owner = Some(owner);
            Err(self)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::Terminal as u8 {
                *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
            }
        }
    }
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/⚔️concurrent-write/🦀️.rs"]
mod concurrent_write_tests;
//#endregion 🧪️Tests
