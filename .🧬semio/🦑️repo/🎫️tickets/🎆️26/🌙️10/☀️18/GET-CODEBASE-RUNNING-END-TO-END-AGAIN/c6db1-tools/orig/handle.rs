//! 🎫️ Db engine artifact handle: submit, history and query operations.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use db_engine_base::{CheckpointPublicationSnapshot, CommandReceipt, Consistency, DatabaseDocumentMountRegistry, DatabaseDocumentMountSlot, Frontier, LiveQuery, LiveQuerySpec, PreviewHandle, Query, QueryResultEntry, QueryStream, SnapshotFuture, SnapshotKind, engine_query_maintenance_step, now_ms, reserve_engine_query_retirement, to_engine_frontier, to_engine_receipt};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_ids::ActorId;

//#region 🔖️History
pub use db_artifact_history::ArtifactHistoryEntry as HistoryEntry;

pub struct HistoryView {
    pub inner: Option<db_artifact_history::ArtifactHistoryView>,
    pub admission: Option<ArtifactHistoryAdmission>,
    pub terminal_state: Option<Arc<ArtifactHistoryState>>,
    return_on_drop: bool,
}

impl HistoryView {
    pub fn new(inner: db_artifact_history::ArtifactHistoryView, admission: Option<ArtifactHistoryAdmission>, terminal_state: Arc<ArtifactHistoryState>) -> Self {
        Self { inner: Some(inner), admission, terminal_state: Some(terminal_state), return_on_drop: true }
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        self.inner.as_ref().map_or(&[], |inner| inner.entries.as_slice())
    }

    pub fn operation_id_eq(&self, entry: usize, operation: usize, expected: &str) -> bool {
        self.inner.as_ref().is_some_and(|inner| inner.operation_id_eq(entry, operation, expected))
    }

    pub fn close_step(&mut self) -> bool {
        if self.inner.as_mut().is_some_and(db_artifact_history::ArtifactHistoryView::close_step) {
            return true;
        }
        if self.inner.take().is_some() {
            return true;
        }
        if self.admission.take().is_some() {
            return true;
        }
        if self.terminal_state.take().is_some() {
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.is_none() && self.admission.is_none()
    }

    fn rearm_terminal_return(&mut self) {
        self.return_on_drop = true;
    }
}

impl Drop for HistoryView {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && self.return_on_drop {
            if let Some(state) = self.terminal_state.as_ref().cloned() {
                let owner = HistoryView { inner: self.inner.take(), admission: self.admission.take(), terminal_state: Some(state.clone()), return_on_drop: false };
                state.finished.store(false, std::sync::atomic::Ordering::Release);
                register_artifact_history(&state);
                *state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(owner));
            }
        }
        assert!(self.terminal_is_empty(), "history result reached Drop before page/range/admission retirement");
    }
}
//#endregion 🔖️History

/// 🎫️ Counts the caller-held `ArtifactHandle`s of one mounted document. The last one to drop
/// unmounts the document: its `Ready` slot becomes `Closing` and its authority retires, so a
/// document holds its WAL writer and retained state only while someone holds it, and
/// the mount capacity bounds concurrently open documents instead of every document a process ever
/// opened. A mount that finds the slot `Closing` waits for that retirement before it replays the
/// WAL. Leases are only ever created under the registry lock. Database shutdown blocks on these
/// only; internal retained owners that still share the authority retire on their own.
pub struct ArtifactHandleLease {
    pub handles: Arc<std::sync::atomic::AtomicUsize>,
    pub registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>,
    pub document: String,
}

impl ArtifactHandleLease {
    pub fn new(handles: Arc<std::sync::atomic::AtomicUsize>, registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>, document: String) -> Self {
        handles.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self { handles, registry, document }
    }

    fn unmount_idle(&self) {
        let Some(registry) = self.registry.upgrade() else { return };
        let retired = {
            let mut registry = registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let idle = matches!(registry.slots.get(&self.document), Some(DatabaseDocumentMountSlot::Ready(reply)) if Arc::ptr_eq(&reply.handles, &self.handles));
            if !idle || self.handles.load(std::sync::atomic::Ordering::Acquire) != 0 {
                return;
            }
            let Some(DatabaseDocumentMountSlot::Ready(reply)) = registry.slots.remove(&self.document) else { return };
            if let Some(terminal) = reply.authority.take_terminal_signal() {
                registry.slots.insert(self.document.clone(), DatabaseDocumentMountSlot::Closing { terminal });
            }
            reply
        };
        drop(retired);
    }
}

impl Clone for ArtifactHandleLease {
    fn clone(&self) -> Self {
        Self::new(self.handles.clone(), self.registry.clone(), self.document.clone())
    }
}

impl Drop for ArtifactHandleLease {
    fn drop(&mut self) {
        if self.handles.fetch_sub(1, std::sync::atomic::Ordering::AcqRel) == 1 {
            self.unmount_idle();
        }
    }
}

/// 🧹️ One queued compaction turn that keeps its document mounted until the turn answers: the
/// actor mailbox refuses every queued message once the last handle unmounts the document.
pub struct ArtifactCompactionFuture {
    ask: db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_compact::CompactionReport, DbError>>,
    pub _lease: ArtifactHandleLease,
}

impl Future for ArtifactCompactionFuture {
    type Output = Result<Result<db_compact::CompactionReport, DbError>, DbError>;

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.get_mut().ask).poll(context)
    }
}

/// 📓️ A durable group journal sink that keeps its document mounted for as long as it lives, like
/// the `ArtifactHandle` it came from.
struct ArtifactHandleJournalSinkV1 {
    sink: Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1>,
    pub _lease: ArtifactHandleLease,
}

impl store::durable_group::DurableOwnedGroupJournalSinkV1 for ArtifactHandleJournalSinkV1 {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn store::durable_group::DurableOwnedGroupJournalCommitV1> {
        self.sink.begin_commit(decision_pack, decision_sha256)
    }
}
//#region 🔖️ArtifactHandle
const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;

const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;

/// 📐️ One submit holds any batch the document backbone declares legal (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a
/// writer's whole post-cut outbox arrives as one batch and must commit, so the envelope, nested-item and byte credits are the
/// declaration's own maxima — never a smaller engine-side cap a declared batch could hit.
const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES;

const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;

/// 📐️ Owner bytes of the largest declared batch: the credit page, its fixed owners (envelopes, dependency ids, target
/// segments), every declared byte, and one declared identifier per envelope for the document id the hub re-keys.
const ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES
    + (protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * size_of::<protocol::MutationEnvelope>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES * size_of::<protocol::MutationId>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS * size_of::<String>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES) as u64;

const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES.div_ceil(ARTIFACT_SUBMIT_PAGE_BYTES);

const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;

/// 📐️ In-flight submit owners across every document: four declared-maximal batches at once; a fifth waits (`Unavailable`).
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = ARTIFACT_SUBMIT_OPERATION_PAGES * 4;

const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;

const ARTIFACT_SUBMIT_RETRY_MS: u64 = 1;

const ARTIFACT_SUBMIT_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct ArtifactSubmitAdmissionSlot {
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
    pub occupied: bool,
}

const EMPTY_ARTIFACT_SUBMIT_SLOT: ArtifactSubmitAdmissionSlot = ArtifactSubmitAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactSubmitAdmissionState {
    pub slots: [ArtifactSubmitAdmissionSlot; ARTIFACT_SUBMIT_OPERATION_ITEMS],
    pub bytes: u64,
    pub next_generation: u64,
}

static ARTIFACT_SUBMIT_ADMISSION: Mutex<ArtifactSubmitAdmissionState> = Mutex::new(ArtifactSubmitAdmissionState { slots: [EMPTY_ARTIFACT_SUBMIT_SLOT; ARTIFACT_SUBMIT_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

struct ArtifactSubmitAdmission {
    pub slot: usize,
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
}

impl ArtifactSubmitAdmission {
    pub fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
        if items == 0 || items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit item credit"));
        }
        if bytes == 0 || bytes > ARTIFACT_SUBMIT_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
        }
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("artifact submit item capacity exhausted".to_string()));
        };
        if state.bytes.checked_add(bytes).is_none_or(|next| next > ARTIFACT_SUBMIT_TOTAL_BYTES) {
            return Err(DbError::Unavailable("artifact submit byte capacity exhausted".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("artifact submit generation"))?;
        state.slots[slot] = ArtifactSubmitAdmissionSlot { generation, bytes, items, occupied: true };
        state.bytes += bytes;
        Ok(Self { slot, generation, bytes, items })
    }
}

impl Drop for ArtifactSubmitAdmission {
    fn drop(&mut self) {
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = &mut state.slots[self.slot];
        if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
            return;
        }
        *entry = EMPTY_ARTIFACT_SUBMIT_SLOT;
        state.bytes = state.bytes.checked_sub(self.bytes).expect("artifact submit byte credit underflow");
    }
}

fn artifact_submit_credit(batch: &db_artifact_contract::CommandBatch) -> Result<(usize, u64), DbError> {
    if batch.envelopes.is_empty() || batch.envelopes.len() > ARTIFACT_SUBMIT_BATCH_ITEMS {
        return Err(DbError::LimitExceeded("artifact submit batch item credit"));
    }
    let mut items = batch.envelopes.len();
    let mut bytes = ARTIFACT_SUBMIT_PAGE_BYTES;
    let envelope_owner_bytes = batch.envelopes.capacity().checked_mul(size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    bytes = bytes.checked_add(envelope_owner_bytes as u64).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    for envelope in &batch.envelopes {
        items = items.checked_add(envelope.dependencies.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let dependency_owner_bytes = envelope.dependencies.capacity().checked_mul(size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded("artifact submit dependency owner bytes"))?;
        bytes = bytes
            .checked_add(envelope.mutation_id.0.capacity() as u64)
            .and_then(|value| value.checked_add(envelope.document_id.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.actor.0.capacity() as u64))
            .and_then(|value| value.checked_add(dependency_owner_bytes as u64))
            .and_then(|value| value.checked_add(envelope.diff.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.diff.payload.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.payload.capacity() as u64))
            .ok_or(DbError::LimitExceeded("artifact submit nested byte credit"))?;
        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
        items = items.checked_add(envelope.target.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let target_owner_bytes = envelope.target.capacity().checked_mul(size_of::<String>()).ok_or(DbError::LimitExceeded("artifact submit target owner bytes"))?;
        bytes = bytes
            .checked_add(target_owner_bytes as u64)
            .and_then(|value| value.checked_add(envelope.observed.as_ref().map_or(0, |observed| observed.0.capacity() as u64)))
            .ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        for segment in &envelope.target {
            bytes = bytes.checked_add(segment.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        }
    }
    let pages = bytes.checked_add(ARTIFACT_SUBMIT_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("artifact submit page rounding"))? / ARTIFACT_SUBMIT_PAGE_BYTES;
    let admitted = pages.checked_mul(ARTIFACT_SUBMIT_PAGE_BYTES).ok_or(DbError::LimitExceeded("artifact submit page credit"))?;
    if admitted > ARTIFACT_SUBMIT_OPERATION_BYTES {
        return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
    }
    Ok((items, admitted))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitProgress {
    Admitted,
    Scheduled,
    Waiting,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorSubmitFuture = db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_artifact_contract::CommandReceipt, DbError>>;

pub type ArtifactSubmitOutcome = Result<Result<CommandReceipt, DbError>, DbError>;

enum ArtifactSubmitWorkOwner {
    Request { batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions, submitted_at_ms: u64 },
    Actor(ArtifactActorSubmitFuture),
}

struct ArtifactSubmitState {
    pub pool: WorkerPool,
    pub authority: Mutex<Option<Arc<db_artifact::ArtifactAuthority>>>,
    pub document: protocol::ArtifactId,
    pub generation: u64,
    pub authority_generation: GenerationId,
    pub admission: Mutex<Option<ArtifactSubmitAdmission>>,
    pub work: Mutex<Option<ArtifactSubmitWorkOwner>>,
    pub completion: Mutex<Option<ArtifactSubmitOutcome>>,
    pub terminal_work: Mutex<Option<ArtifactSubmitWorkOwner>>,
    pub terminal_result: Mutex<Option<ArtifactSubmitOutcome>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub progress: std::sync::atomic::AtomicU8,
}

pub struct SubmitFuture {
    pub state: Arc<ArtifactSubmitState>,
    pub resolved: bool,
}

pub struct ArtifactSubmitTerminalJob {
    pub state: Arc<ArtifactSubmitState>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactSubmitTerminalWork {
    pub state: Arc<ArtifactSubmitState>,
    pub owner: Option<ArtifactSubmitWorkOwner>,
}

struct ArtifactSubmitWake {
    pub state: std::sync::Weak<ArtifactSubmitState>,
    pub generation: u64,
}

impl std::task::Wake for ArtifactSubmitWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactSubmitState {
    pub fn set_progress(&self, progress: SubmitProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    pub fn finish(&self) {
        if !self.finished.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.authority.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        }
    }

    pub fn authority(&self) -> Option<Arc<db_artifact::ArtifactAuthority>> {
        self.authority.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn finish_if_terminal_empty(&self) {
        if self.terminal_is_empty() && !self.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire) {
            self.finish();
        }
    }

    pub fn complete(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        self.set_progress(progress);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.wake_waiter();
        }
    }

    fn terminalize_work(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.set_progress(SubmitProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_SUBMIT_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    }
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.complete(Err(DbError::Unavailable(format!("artifact submit WorkerPool submission failed: {kind:?}"))), SubmitProgress::Fault);
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact submit retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact submit retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact submit retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    state.scheduled.store(false, Ordering::Release);
                    state.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), SubmitProgress::Fault);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        let Some(authority) = self.authority() else {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        };
        if authority.generation() != self.authority_generation {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::StaleGeneration { expected: authority.generation(), actual: self.authority_generation }), SubmitProgress::Fault);
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.cancelled.load(Ordering::Acquire) {
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if matches!(work.as_ref(), Some(ArtifactSubmitWorkOwner::Request { .. })) {
            let Some(ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms }) = work.take() else {
                return;
            };
            *work = Some(ArtifactSubmitWorkOwner::Actor(authority.submit_retained(batch, options, submitted_at_ms)));
            drop(work);
            self.schedule();
            return;
        }

        let Some(ArtifactSubmitWorkOwner::Actor(future)) = work.as_mut() else {
            return;
        };
        let waker = std::task::Waker::from(Arc::new(ArtifactSubmitWake { state: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        match std::pin::Pin::new(future).poll(&mut context) {
            std::task::Poll::Pending => {
                self.set_progress(SubmitProgress::Waiting);
            }
            std::task::Poll::Ready(result) => {
                work.take();
                drop(work);
                let result = result.map(|inner| inner.map(|receipt| to_engine_receipt(receipt, self.document.clone())));
                if self.cancelled.load(Ordering::Acquire) {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                    self.complete(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    self.complete(result, SubmitProgress::Completed);
                }
            }
        }
    }

    pub fn close_one(&self) -> bool {
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(work);
            return true;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return true;
        }
        false
    }
}

impl SubmitFuture {
    pub fn submit(handle: &ArtifactHandle, batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions) -> Self {
        let credit = artifact_submit_credit(&batch).and_then(|(items, bytes)| ArtifactSubmitAdmission::try_claim(items, bytes));
        let admission_error = credit.as_ref().err().cloned();
        let generation = credit.as_ref().map_or(0, |admission| admission.generation);
        let request = ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms: handle.pool.now_ms() };
        let (work, terminal_work) = if generation == 0 { (None, Some(request)) } else { (Some(request), None) };
        let state = Arc::new(ArtifactSubmitState {
            pool: handle.pool.as_ref().clone(),
            authority: Mutex::new(Some(handle.authority.clone())),
            document: handle.document.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: Mutex::new(credit.ok()),
            work: Mutex::new(work),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(terminal_work),
            terminal_result: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { SubmitProgress::Fault as u8 } else { SubmitProgress::Admitted as u8 }),
        });
        if generation == 0 {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(admission_error.unwrap_or_else(|| DbError::Unavailable("artifact submit admission exhausted".to_string()))));
        } else {
            state.schedule();
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> SubmitProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => SubmitProgress::Admitted,
            1 => SubmitProgress::Scheduled,
            2 => SubmitProgress::Waiting,
            3 => SubmitProgress::Completed,
            4 => SubmitProgress::Cancelled,
            _ => SubmitProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if matches!(self.progress(), SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault) {
            return;
        }
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactSubmitTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactSubmitTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactSubmitOutcome> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority().and_then(|authority| authority.take_terminal_job())
    }

    pub fn close_step(&self) -> bool {
        let progressed = self.state.close_one() || self.state.authority().is_some_and(|authority| authority.close_step());
        self.state.finish_if_terminal_empty();
        progressed
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority().is_none_or(|authority| authority.terminal_is_empty())
    }
}

impl Future for SubmitFuture {
    type Output = ArtifactSubmitOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for SubmitFuture {
    fn drop(&mut self) {
        if !self.resolved {
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.state.schedule();
        }
        self.state.close_one();
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactSubmitTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact submit job already resolved").0
    }

    pub fn resume(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Scheduled);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        drop(job);
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactSubmitTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact submit work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact submit work already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

pub const ARTIFACT_HISTORY_OPERATION_SLOTS: usize = 8;

const ARTIFACT_HISTORY_PAGE_BYTES: u64 = 16 * 1024;

const ARTIFACT_HISTORY_OPERATION_PAGES: u64 = 2_048;

const ARTIFACT_HISTORY_OPERATION_BYTES: u64 = ARTIFACT_HISTORY_PAGE_BYTES * ARTIFACT_HISTORY_OPERATION_PAGES;

const ARTIFACT_HISTORY_TOTAL_BYTES: u64 = ARTIFACT_HISTORY_OPERATION_BYTES * ARTIFACT_HISTORY_OPERATION_SLOTS as u64;

const ARTIFACT_HISTORY_OPERATION_ITEMS: usize = 20_481;

#[derive(Clone, Copy)]
struct ArtifactHistoryAdmissionSlot {
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
    pub occupied: bool,
}

const EMPTY_ARTIFACT_HISTORY_SLOT: ArtifactHistoryAdmissionSlot = ArtifactHistoryAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactHistoryAdmissionState {
    pub slots: [ArtifactHistoryAdmissionSlot; ARTIFACT_HISTORY_OPERATION_SLOTS],
    pub bytes: u64,
    pub next_generation: u64,
}

pub static ARTIFACT_HISTORY_ADMISSION: Mutex<ArtifactHistoryAdmissionState> = Mutex::new(ArtifactHistoryAdmissionState { slots: [EMPTY_ARTIFACT_HISTORY_SLOT; ARTIFACT_HISTORY_OPERATION_SLOTS], bytes: 0, next_generation: 1 });

pub struct ArtifactHistoryAdmission {
    pub slot: usize,
    pub generation: u64,
    pub reservation: Option<db_artifact_history::HistoryReplayReservation>,
}

pub enum ArtifactHistoryAdmissionError {
    Rejected(DbError),
    Construction { admission: ArtifactHistoryAdmission, fault: db_artifact_history::HistoryReplayReservationConstructionFault },
}

impl std::fmt::Debug for ArtifactHistoryAdmissionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected(error) => formatter.debug_tuple("Rejected").field(error).finish(),
            Self::Construction { fault, .. } => formatter.debug_struct("Construction").field("fault", fault).finish(),
        }
    }
}

impl ArtifactHistoryAdmission {
    pub fn try_claim() -> Result<Self, ArtifactHistoryAdmissionError> {
        let (slot, generation) = {
            let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history item capacity exhausted".to_string())));
            };
            let Some(next_bytes) = state.bytes.checked_add(ARTIFACT_HISTORY_OPERATION_BYTES) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history aggregate bytes")));
            };
            if next_bytes > ARTIFACT_HISTORY_TOTAL_BYTES {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history byte capacity exhausted".to_string())));
            }
            let generation = state.next_generation;
            let Some(next_generation) = state.next_generation.checked_add(1) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history generation")));
            };
            state.next_generation = next_generation;
            state.slots[slot] = ArtifactHistoryAdmissionSlot { generation, bytes: ARTIFACT_HISTORY_OPERATION_BYTES, items: ARTIFACT_HISTORY_OPERATION_ITEMS, occupied: true };
            state.bytes = next_bytes;
            (slot, generation)
        };
        let mut claim = Self { slot, generation, reservation: None };
        match db_artifact_history::HistoryReplayReservation::try_new() {
            Ok(reservation) => {
                claim.reservation = Some(reservation);
                Ok(claim)
            }
            Err(fault) => Err(ArtifactHistoryAdmissionError::Construction { admission: claim, fault }),
        }
    }

    fn take_reservation(&mut self) -> Option<db_artifact_history::HistoryReplayReservation> {
        self.reservation.take()
    }

    pub fn begin_reservation_close(&mut self) -> Option<db_artifact_history::HistoryReplayReservationCloseCursor> {
        self.reservation.take().map(db_artifact_history::HistoryReplayReservationCloseCursor::new)
    }

    fn restore_reservation(&mut self, reservation: db_artifact_history::HistoryReplayReservation) {
        assert!(self.reservation.is_none(), "artifact history reservation restore slot was occupied");
        self.reservation = Some(reservation);
    }
}

impl Drop for ArtifactHistoryAdmission {
    fn drop(&mut self) {
        let live = self.reservation.take().is_some();
        {
            let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let entry = &mut state.slots[self.slot];
            if entry.occupied && entry.generation == self.generation && entry.bytes == ARTIFACT_HISTORY_OPERATION_BYTES && entry.items == ARTIFACT_HISTORY_OPERATION_ITEMS {
                *entry = EMPTY_ARTIFACT_HISTORY_SLOT;
                state.bytes = state.bytes.checked_sub(ARTIFACT_HISTORY_OPERATION_BYTES).expect("artifact history byte credit underflow");
            }
        }
        assert!(!live || std::thread::panicking(), "artifact history admission dropped a live replay reservation");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryProgress {
    Admitted,
    Scheduled,
    Waiting,
    Mapping,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorHistoryFuture = db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_artifact_history::ArtifactHistoryView, DbError>>;

pub type ArtifactHistoryOutcome = Result<HistoryView, DbError>;

enum ArtifactHistoryWorkOwner {
    Request,
    Actor(ArtifactActorHistoryFuture),
}

pub struct ArtifactHistoryState {
    pub pool: WorkerPool,
    pub authority: Arc<db_artifact::ArtifactAuthority>,
    pub generation: u64,
    pub authority_generation: GenerationId,
    pub admission: Mutex<Option<ArtifactHistoryAdmission>>,
    pub work: Mutex<Option<ArtifactHistoryWorkOwner>>,
    pub completion: Mutex<Option<ArtifactHistoryOutcome>>,
    pub terminal_work: Mutex<Option<ArtifactHistoryWorkOwner>>,
    pub terminal_result: Mutex<Option<ArtifactHistoryOutcome>>,
    terminal_reservation: Mutex<Option<db_artifact_history::HistoryReplayReservationCloseCursor>>,
    reservation_checked_out: std::sync::atomic::AtomicBool,
    terminal_construction: Mutex<Option<db_artifact_history::HistoryReplayReservationConstructionFault>>,
    construction_checked_out: std::sync::atomic::AtomicBool,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub progress: std::sync::atomic::AtomicU8,
    #[cfg(test)]
    pub controlled_completion_hook: Mutex<Option<Box<dyn FnOnce(ArtifactHistoryOutcome, HistoryProgress) + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<semio_framework_async::Job>>,
}

pub struct HistoryFuture {
    pub state: Arc<ArtifactHistoryState>,
    pub resolved: bool,
}

pub struct ArtifactHistoryTerminalHandle {
    pub state: Arc<ArtifactHistoryState>,
}

pub struct ArtifactHistoryTerminalJob {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactHistoryTerminalWork {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<ArtifactHistoryWorkOwner>,
}

pub struct ArtifactHistoryTerminalReservation {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<db_artifact_history::HistoryReplayReservationCloseCursor>,
}

pub struct ArtifactHistoryTerminalConstructionFault {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<db_artifact_history::HistoryReplayReservationConstructionFault>,
}

struct ArtifactHistoryWake {
    pub state: std::sync::Weak<ArtifactHistoryState>,
    pub generation: u64,
}

fn artifact_history_registry() -> &'static Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

fn register_artifact_history(state: &Arc<ArtifactHistoryState>) {
    if state.generation == 0 {
        return;
    }
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.iter().any(|slot| slot.as_ref().is_some_and(|registered| registered.generation == state.generation)) {
        return;
    }
    if let Some(slot) = registry.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(state.clone());
    }
}

fn unregister_artifact_history(generation: u64) {
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = registry.iter_mut().find(|slot| slot.as_ref().is_some_and(|state| state.generation == generation)) {
        *slot = None;
    }
}

impl std::task::Wake for ArtifactHistoryWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactHistoryState {
    pub fn set_progress(&self, progress: HistoryProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn wake_waiter(&self) {
        let waker = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn take_terminal_reservation(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalReservation> {
        use std::sync::atomic::Ordering;
        if self.reservation_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.reservation_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalReservation { state: self.clone(), owner })
    }

    fn take_terminal_construction_fault(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalConstructionFault> {
        use std::sync::atomic::Ordering;
        if self.construction_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.construction_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalConstructionFault { state: self.clone(), owner })
    }

    pub fn terminal_roots_are_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.construction_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_roots_are_empty()
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
            && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire)
    }

    fn finish_if_terminal_empty(&self) -> bool {
        use std::sync::atomic::Ordering;
        if !self.terminal_roots_are_empty() || self.scheduled.load(Ordering::Acquire) || self.retry_armed.load(Ordering::Acquire) {
            return false;
        }
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.finished.load(Ordering::Acquire) {
            return false;
        }
        let owner = admission.take();
        assert!(owner.as_ref().is_none_or(|owner| owner.reservation.is_none()), "artifact history admission finished before reservation retirement");
        drop(owner);
        unregister_artifact_history(self.generation);
        self.finished.store(true, Ordering::Release);
        true
    }

    fn begin_unhanded_reservation_close(&self) {
        if self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let cursor = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::begin_reservation_close);
        if let Some(cursor) = cursor {
            *self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(cursor);
        }
    }

    fn resume_unhanded_reservation(&self) -> bool {
        if self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return false;
        }
        let mut terminal = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(cursor) = terminal.as_mut() else {
            return self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.reservation.is_some());
        };
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            return false;
        }
        let Some(reservation) = cursor.resume() else {
            return false;
        };
        let admission = admission.as_mut().expect("artifact history reservation admission disappeared after preflight");
        admission.restore_reservation(reservation);
        terminal.take();
        true
    }

    pub fn complete(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        self.set_progress(progress);
        #[cfg(test)]
        {
            let hook = self.controlled_completion_hook.lock().unwrap().take();
            if let Some(hook) = hook {
                hook(result, progress);
                return;
            }
        }
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            self.wake_waiter();
        }
    }

    fn terminalize_work(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            if matches!(&work, ArtifactHistoryWorkOwner::Request) {
                self.begin_unhanded_reservation_close();
            }
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    fn terminalize_unhanded_request(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) -> bool {
        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !work.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) {
            return false;
        }
        let request = work.take().expect("artifact history request disappeared after retained preflight");
        drop(work);
        self.begin_unhanded_reservation_close();
        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(request);
        self.complete(result, progress);
        true
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire)
            || self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.construction_checked_out.load(Ordering::Acquire)
            || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err()
        {
            return;
        }
        self.set_progress(HistoryProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_SUBMIT_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.terminalize_work(Err(DbError::Unavailable(format!("artifact history WorkerPool submission failed: {kind:?}"))), HistoryProgress::Fault);
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact history retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact history retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact history retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    state.scheduled.store(false, Ordering::Release);
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                    state.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), HistoryProgress::Fault);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.authority.generation() != self.authority_generation {
            self.cancelled.store(true, Ordering::Release);
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::StaleGeneration { expected: self.authority.generation(), actual: self.authority_generation }), HistoryProgress::Fault);
                return;
            }
        }
        if self.cancelled.load(Ordering::Acquire) {
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                return;
            }
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match work.as_mut() {
            Some(ArtifactHistoryWorkOwner::Request) => {
                let reservation = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::take_reservation);
                let Some(reservation) = reservation else {
                    drop(work);
                    self.terminalize_work(Err(DbError::Unavailable("artifact history fixed reservation owner is missing".to_string())), HistoryProgress::Fault);
                    return;
                };
                *work = Some(ArtifactHistoryWorkOwner::Actor(self.authority.history_retained(self.generation, self.cancelled.clone(), reservation)));
                drop(work);
                self.schedule();
            }
            Some(ArtifactHistoryWorkOwner::Actor(future)) => {
                let waker = std::task::Waker::from(Arc::new(ArtifactHistoryWake { state: Arc::downgrade(&self), generation }));
                let mut context = std::task::Context::from_waker(&waker);
                match std::pin::Pin::new(future).poll(&mut context) {
                    std::task::Poll::Pending => self.set_progress(HistoryProgress::Waiting),
                    std::task::Poll::Ready(Ok(Ok(view))) => {
                        work.take();
                        drop(work);
                        let admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                        match admission {
                            Some(admission) => self.complete(Ok(HistoryView::new(view, Some(admission), self.clone())), HistoryProgress::Completed),
                            None => self.complete(Ok(HistoryView::new(view, None, self.clone())), HistoryProgress::Fault),
                        }
                    }
                    std::task::Poll::Ready(Ok(Err(error))) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                    std::task::Poll::Ready(Err(error)) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                }
            }
            None => {}
        }
    }

    pub fn close_one(self: &Arc<Self>) -> bool {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            match work {
                ArtifactHistoryWorkOwner::Request => self.begin_unhanded_reservation_close(),
                ArtifactHistoryWorkOwner::Actor(owner) => {
                    *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                    self.cancelled.store(true, std::sync::atomic::Ordering::Release);
                    self.schedule();
                }
            }
            return true;
        }
        let mut reservation = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = reservation.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                reservation.take();
                return true;
            }
        }
        drop(reservation);
        let mut construction = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = construction.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                construction.take();
                return true;
            }
        }
        drop(construction);
        let mut terminal = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(result) = terminal.as_mut() {
            match result {
                Ok(view) => {
                    if view.close_step() {
                        return true;
                    }
                    terminal.take();
                    return true;
                }
                Err(_) => {
                    terminal.take();
                    return true;
                }
            }
        }
        false
    }
}

impl HistoryFuture {
    pub fn take_completion(&mut self) -> Option<ArtifactHistoryOutcome> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.resolved = true;
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn submit(handle: &ArtifactHandle) -> Self {
        let future = Self::prepare(handle);
        let ready = future.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        if ready {
            future.state.schedule();
        }
        future
    }

    pub fn prepare(handle: &ArtifactHandle) -> Self {
        let (admission, construction_fault, rejected_error, generation) = match ArtifactHistoryAdmission::try_claim() {
            Ok(admission) => {
                let generation = admission.generation;
                (Some(admission), None, None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Construction { admission, fault }) => {
                let generation = admission.generation;
                (Some(admission), Some(fault), None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Rejected(error)) => (None, None, Some(error), 0),
        };
        let ready = construction_fault.is_none() && rejected_error.is_none();
        let state = Arc::new(ArtifactHistoryState {
            pool: handle.pool.as_ref().clone(),
            authority: handle.authority.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: Mutex::new(admission),
            work: Mutex::new(ready.then_some(ArtifactHistoryWorkOwner::Request)),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(None),
            terminal_result: Mutex::new(None),
            terminal_reservation: Mutex::new(None),
            reservation_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_construction: Mutex::new(construction_fault),
            construction_checked_out: std::sync::atomic::AtomicBool::new(false),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { HistoryProgress::Fault as u8 } else { HistoryProgress::Admitted as u8 }),
            #[cfg(test)]
            controlled_completion_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
        });
        register_artifact_history(&state);
        let construction_error = state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(db_artifact_history::HistoryReplayReservationConstructionFault::take_error);
        if let Some(error) = rejected_error.or(construction_error) {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(error));
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn terminal_handle(&self) -> ArtifactHistoryTerminalHandle {
        ArtifactHistoryTerminalHandle { state: self.state.clone() }
    }

    pub fn progress(&self) -> HistoryProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => HistoryProgress::Admitted,
            1 => HistoryProgress::Scheduled,
            2 => HistoryProgress::Waiting,
            3 => HistoryProgress::Mapping,
            4 => HistoryProgress::Completed,
            5 => HistoryProgress::Cancelled,
            _ => HistoryProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if !matches!(self.progress(), HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault) {
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl ArtifactHistoryTerminalHandle {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl Future for HistoryFuture {
    type Output = ArtifactHistoryOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.take_completion() {
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        {
            let hook = self.state.controlled_publication_before_waker_hook.lock().unwrap().take();
            if let Some(hook) = hook {
                hook();
            }
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        self.take_completion().map_or(std::task::Poll::Pending, std::task::Poll::Ready)
    }
}

impl Drop for HistoryFuture {
    fn drop(&mut self) {
        if !self.resolved {
            let mut completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            if let Some(result) = completion.take() {
                *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            }
            drop(completion);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactHistoryTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact history job already resolved").0
    }

    pub fn resume(mut self) {
        let request = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request));
        if request && !self.state.resume_unhanded_reservation() {
            return;
        }
        let (_, job) = self.owner.take().expect("terminal artifact history job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact history job already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactHistoryTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactHistoryTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        if self.owner.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) && !self.state.resume_unhanded_reservation() {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(HistoryProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        match owner {
            ArtifactHistoryWorkOwner::Request => {
                self.state.begin_unhanded_reservation_close();
                self.state.finish_if_terminal_empty();
            }
            ArtifactHistoryWorkOwner::Actor(owner) => {
                *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                self.state.schedule();
            }
        }
    }
}

impl ArtifactHistoryTerminalReservation {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut admission = self.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            drop(admission);
            return Err(self);
        }
        let Some(cursor) = self.owner.as_mut() else {
            drop(admission);
            return Err(self);
        };
        let Some(reservation) = cursor.resume() else {
            drop(admission);
            return Err(self);
        };
        let admission = admission.as_mut().expect("artifact history admission changed after reservation preflight");
        admission.restore_reservation(reservation);
        self.owner = None;
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
            self.state.finish_if_terminal_empty();
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact_history::HistoryReplayReservationCloseCursor::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalReservation {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl ArtifactHistoryTerminalConstructionFault {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut terminal = self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_some() {
            drop(terminal);
            return Err(self);
        }
        *terminal = self.owner.take();
        drop(terminal);
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact_history::HistoryReplayReservationConstructionFault::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalConstructionFault {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl Drop for ArtifactHistoryTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

/// 🎭️ The frozen `ArtifactHandle`: a clone-cheap live handle to one open document.
#[derive(Clone)]
pub struct ArtifactHandle {
    pub authority: Arc<db_artifact::ArtifactAuthority>,
    pub _lease: ArtifactHandleLease,
    pub document: protocol::ArtifactId,
    pub pool: Arc<WorkerPool>,
}

impl ArtifactHandle {
    /// 📓️ Creates the typed Store journal port backed by this document authority's
    /// already-retained WAL writer; no generic command submission or second permit is exposed.
    pub fn durable_group_journal_sink(&self, now_ms: u64) -> Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1> {
        Box::new(ArtifactHandleJournalSinkV1 { sink: self.authority.durable_group_journal_sink(now_ms), _lease: self._lease.clone() })
    }

    /// 🧭 Replays committed fixed-three decisions through this document's existing actor and WAL
    /// owner. The returned owner retains all three Stores if its awaiting caller is cancelled.
    pub fn durable_group_recovery_retained<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> db_artifact_contract::ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        self.authority.durable_group_recovery_retained(db_ids::ArtifactId(self.document.0.clone()), admission)
    }

    pub fn resume_durable_group_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        owner: &mut db_artifact_contract::ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> Result<(), DbError>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        self.authority.resume_durable_group_recovery(owner)
    }

    /// ✍️ The frozen `submit`: commits `batch` through the document's real
    /// `ArtifactAuthority` mailbox. Admission retains the exact request owner, and every I/O-lane
    /// grant advances either request-to-mailbox handoff or one actor-future poll.
    pub fn submit(&self, batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions) -> SubmitFuture {
        SubmitFuture::submit(self, batch, options)
    }

    /// 🧹 Queues one bounded compaction turn behind prior document commands.
    pub fn compact_retained(
        &self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> ArtifactCompactionFuture {
        ArtifactCompactionFuture { ask: self.authority.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled), _lease: self._lease.clone() }
    }

    pub async fn compact(&self, holder: &str, consolidate_snapshots: bool, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<db_compact::CompactionReport, DbError> {
        let holder = db_io_core::DbIoText::try_from_str(holder)?;
        self.compact_retained(holder, consolidate_snapshots, db_compact::CompactionBudget::default(), now_ms().await, cancelled).await?
    }

    /// 🔎️ The frozen `query`. `Consistency::Canonical` reads the document's live state
    /// directly. `AtLeast`/`Exact` read canonical too, then verify the resulting frontier actually
    /// satisfies the request (`DbError::Unavailable` if not — a true wait-for-frontier primitive
    /// would need a `ArtifactMessage` variant `db_artifact`'s mailbox doesn't expose yet).
    /// `Historical`/`Speculative`/`PreviewAugmented` are `DbError::Unimplemented` — see module doc.
    // 🔒️ `consistency`'s by-value signature is the frozen contract API
    // (`ArtifactHandle::query(&self, query: Query, consistency: Consistency)`, contract.md's
    // "Stable API" block) — not changeable even though this revision's body only borrows it.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError> {
        engine_query_maintenance_step()?;
        match &consistency {
            Consistency::Historical(_) | Consistency::PreviewAugmented(_) => {
                return Err(DbError::Unimplemented("historical/preview-augmented query consistency is not yet wired at the db_engine layer (db_query/db_projection integration deferred)"));
            }
            Consistency::Speculative(_) => {
                return Err(DbError::Unimplemented("speculative (preview) query consistency is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"));
            }
            Consistency::Canonical | Consistency::AtLeast(_) | Consistency::Exact(_) => {}
        }

        let paths: Vec<String> = match query {
            Query::Get { path } => vec![path],
            Query::GetMany { paths } => paths,
        };
        if paths.len() > 64 {
            return Err(DbError::LimitExceeded("engine query result entries"));
        }
        let mut results = QueryStream::new();
        if !paths.is_empty() {
            results.retirement = Some(reserve_engine_query_retirement().ok_or_else(|| DbError::Unavailable("engine query retirement pressure refused query".to_string()))?);
        }
        for path in paths {
            let value = match self.authority.query(&path).await {
                Ok(value) => value,
                Err(error) => {
                    drop(results);
                    return Err(error);
                }
            };
            let path = match db_io_core::DbIoText::try_from_str(&path) {
                Ok(path) => path,
                Err(error) => {
                    drop(value);
                    drop(results);
                    return Err(error);
                }
            };
            results.push_preflighted(QueryResultEntry { path, value });
        }

        let frontier = match self.frontier().await {
            Ok(frontier) => frontier,
            Err(error) => {
                drop(results);
                return Err(error);
            }
        };
        match &consistency {
            Consistency::AtLeast(requested) if !frontier.dominates(requested)? => {
                drop(results);
                return Err(DbError::Unavailable("document has not yet reached the requested frontier".to_string()));
            }
            Consistency::Exact(requested) if &frontier != requested => {
                drop(results);
                return Err(DbError::Unavailable("document frontier does not exactly match the requested frontier".to_string()));
            }
            _ => {}
        }
        Ok(results)
    }

    /// 📡️ The frozen `subscribe` — see module doc's `//🎯️ Design choice`: always
    /// `DbError::Unimplemented`, a real (not faked) extension seam pending a `ArtifactMessage`
    /// variant `db_artifact` doesn't expose yet.
    pub async fn subscribe(&self, _spec: LiveQuerySpec) -> Result<LiveQuery, DbError> {
        Err(DbError::Unimplemented("live-query subscription is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// 🧭️ The frozen `frontier`.
    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        let core_frontier = self.authority.frontier().await?;
        Ok(to_engine_frontier(&core_frontier, self.document.clone()))
    }

    /// 📍️ Captures the exact committed frontier and tip identity in one serialized turn.
    pub async fn checkpoint_publication_snapshot(&self) -> Result<CheckpointPublicationSnapshot, DbError> {
        let snapshot = self.authority.checkpoint_publication_snapshot().await?;
        Ok(CheckpointPublicationSnapshot { authority_generation: self.authority.generation().0, frontier: to_engine_frontier(&snapshot.frontier, self.document.clone()), head_edit_id: snapshot.head_edit_id })
    }

    /// 🌫️ The frozen `preview` — see `subscribe`'s doc; same deferral reason.
    pub async fn preview(&self, _base: Frontier) -> Result<PreviewHandle, DbError> {
        Err(DbError::Unimplemented("preview publish/query is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// 📜️ Replays history through the document authority. Each accepted process-pool grant
    /// advances one retained mailbox, WAL, envelope, or result-mapping opportunity.
    pub fn history(&self) -> HistoryFuture {
        HistoryFuture::submit(self)
    }

    pub fn history_terminal(&self, generation: u64) -> Option<ArtifactHistoryTerminalHandle> {
        artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find_map(|slot| slot.as_ref().filter(|state| state.generation == generation).map(|state| ArtifactHistoryTerminalHandle { state: state.clone() }))
    }

    /// 📸️ The frozen `snapshot_now` — see module doc's `//🎯️ Design choice`: always resolves
    /// to `DbError::Unimplemented`, a real extension seam (no full-state enumeration exists yet to
    /// serialize, and `db_snapshot` is not a direct dependency of this crate).
    pub async fn snapshot_now(&self, _kind: SnapshotKind) -> SnapshotFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        reply_tx.send(Err(DbError::Unimplemented("db_engine does not yet build real pack snapshots (no db_snapshot dependency this wave, and DocumentState exposes no full-state enumeration to serialize)")));
        reply_rx
    }

    pub async fn document_id(&self) -> &protocol::ArtifactId {
        &self.document
    }
}
//#endregion 🔖️ArtifactHandle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
