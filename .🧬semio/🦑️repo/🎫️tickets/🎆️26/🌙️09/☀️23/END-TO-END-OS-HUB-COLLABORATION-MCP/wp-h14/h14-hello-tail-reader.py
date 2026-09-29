#!/usr/bin/env python3
"""🚰 H14 14c P1 (A2), on top of the streamed hello tail: (1) ONE tail reader walks the WAL once — it keeps its cursor across
frames and hands each frame over through a one-page slot (waiting while the slot is full), instead of a page future per frame
that re-read its segment from the start (a 50 000-edit tail re-read every segment ~7×, the law timed out); (2) a streaming
hello's deadline is a STALL bound: every produced frame renews it, so a long tail that keeps moving is never cut, and a
stalled one still expires after `DATABASE_SYNC_HELLO_DEADLINE_MS` without progress. Idempotent; `--dry-run` reports only."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems, states = [], []


def edit(old, new, label):
    global text
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    text = text.replace(old, new)
    states.append("replace")


def edit_region(start, end, new, label):
    global text
    if new in text:
        states.append("done")
        return
    if text.count(start) != 1:
        problems.append(f"{label}: start found {text.count(start)}")
        states.append("problem")
        return
    begin = text.index(start)
    stop = text.index(end, begin) + len(end)
    text = text[:begin] + new + text[stop:]
    states.append("replace")


edit_region("""/// 🧭️ Where a hello tail's next frame starts reading:""", """    let chain_hash = *cursor.chain.finalize().as_bytes();
    Ok(DatabaseSyncHelloTailPage { cursor, envelopes, frontier: Some((head_seq, head_edit_id, commit_seq, chain_hash)) })
}
""", """/// 📄️ One tail frame the reader handed over: its envelopes and the frontier after its last commit (head, head id, commit
/// sequence, chain hash).
type DatabaseSyncHelloTailPage = (Vec<protocol::MutationEnvelope>, (u64, String, u64, [u8; 32]));

/// 🤝️ The one-page slot between the tail reader and the hello driver: the reader parks here while the previous frame is still
/// in the slot, and the driver wakes it when it takes that frame.
#[derive(Default)]
struct DatabaseSyncHelloTailHandoff {
    page: Option<DatabaseSyncHelloTailPage>,
    reader: Option<std::task::Waker>,
}

/// ⏳️ Resolves once the tail slot is empty.
struct DatabaseSyncHelloTailSlotFree<'slot>(&'slot std::sync::Mutex<DatabaseSyncHelloTailHandoff>);

impl std::future::Future for DatabaseSyncHelloTailSlotFree<'_> {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let mut handoff = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if handoff.page.is_none() {
            return std::task::Poll::Ready(());
        }
        handoff.reader = Some(context.waker().clone());
        std::task::Poll::Pending
    }
}

type DatabaseSyncHelloTailReaderFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbError>> + Send>>;

/// 🚰️ The tail a replica lacks, streamed from the WAL one frame at a time: ONE reader ([`database_sync_hello_tail_reader`])
/// walks the WAL once and hands each frame over through a one-page slot; each frame carries the exact frontier after its last
/// commit and its backing is charged while it is out and released when it closes, so a hello holds at most about two frames of
/// the tail however long it is.
struct DatabaseSyncHelloTail {
    document: ArtifactId,
    origin: protocol::ActorId,
    handoff: std::sync::Arc<std::sync::Mutex<DatabaseSyncHelloTailHandoff>>,
    reader: Option<DatabaseSyncHelloTailReaderFuture>,
    done: bool,
}

impl DatabaseSyncHelloTail {
    fn new(storage: std::sync::Arc<db_storage::DbBackend>, document: ArtifactId, origin: protocol::ActorId, replica_head: u64, server: &Frontier, cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, expired: std::sync::Arc<std::sync::atomic::AtomicBool>) -> Self {
        let handoff = std::sync::Arc::new(std::sync::Mutex::new(DatabaseSyncHelloTailHandoff::default()));
        let reader = Box::pin(database_sync_hello_tail_reader(storage, document.clone(), replica_head, server.head_seq, server.commit_seq, handoff.clone(), cancelled, expired));
        Self { document, origin, handoff, reader: Some(reader), done: false }
    }

    fn take_page(&self) -> Option<DatabaseSyncHelloTailPage> {
        let mut handoff = self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let page = handoff.page.take()?;
        if let Some(reader) = handoff.reader.take() {
            reader.wake();
        }
        Some(page)
    }

    /// 🚚️ Advances the reader and hands out the next frame it produced.
    fn drive(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, context: &mut std::task::Context<'_>) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        if self.done {
            return Ok(DatabaseSyncHelloFollowUpStep::Done);
        }
        let finished = match self.reader.as_mut() {
            Some(reader) => match std::future::Future::poll(reader.as_mut(), context) {
                std::task::Poll::Pending => false,
                std::task::Poll::Ready(read) => {
                    self.reader = None;
                    read?;
                    true
                }
            },
            None => true,
        };
        let Some((envelopes, (head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash))) = self.take_page() else {
            if finished {
                self.done = true;
                return Ok(DatabaseSyncHelloFollowUpStep::Done);
            }
            return Ok(DatabaseSyncHelloFollowUpStep::Waiting);
        };
        let frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(self.document.0.clone()), head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
        let frame = protocol::ServerFrame::Commands { envelopes, origin: self.origin.clone(), frontier };
        let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
        ledger.observe(items, bytes, "database sync hello tail frame backing")?;
        Ok(DatabaseSyncHelloFollowUpStep::Frame(frame))
    }

    fn close_one(&mut self) -> bool {
        if self.reader.take().is_some() || self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page.take().is_some() {
            return true;
        }
        database_sync_hello_retire_string(&mut self.origin.0) || database_sync_hello_retire_string(&mut self.document.0)
    }

    fn terminal_is_empty(&self) -> bool {
        self.reader.is_none() && self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page.is_none() && self.origin.0.capacity() == 0 && self.document.0.capacity() == 0
    }
}

/// 🚰️ Walks the document's WAL once from its first retained segment: hashes every command into the chain, decodes those past
/// `replica_head` and, on the first commit boundary after [`DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES`] envelopes (or at the
/// tail's end, `end_head_seq`), hands the frame over — waiting while the previous one is still in the slot. The frame that
/// emits the last command carries the server's commit sequence `end_commit_seq`: transactions without a command after it
/// count in the frontier the welcome announced.
#[allow(clippy::too_many_arguments)]
async fn database_sync_hello_tail_reader(
    storage: std::sync::Arc<db_storage::DbBackend>,
    document: ArtifactId,
    replica_head: u64,
    end_head_seq: u64,
    end_commit_seq: u64,
    handoff: std::sync::Arc<std::sync::Mutex<DatabaseSyncHelloTailHandoff>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<(), DbError> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal::replay_committed_document(&wal, &document, control).await?;
    let mut decode_control = db_wal::WalCursorControl::stall_bounded(cancelled.clone(), db_wal::WAL_REPLAY_STEP_STALL_BOUND, db_wal::WAL_REPLAY_STEP_FUEL)?;
    let mut chain = semio_framework_hash::Hasher::new();
    let mut envelopes: Vec<protocol::MutationEnvelope> = Vec::new();
    let mut head_seq = 0u64;
    let mut commit_seq = 0u64;
    let mut head_edit_id = String::new();
    let mut turn = db_wal::WalTurn::start();
    let read = async {
        while head_seq < end_head_seq {
            records.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            database_sync_hello_control(&cancelled, &expired)?;
            let mut transaction = match records.next_transaction_step().await {
                Ok(db_wal::WalCommittedStep::Transaction(transaction)) => transaction,
                Ok(db_wal::WalCommittedStep::Yield) => {
                    turn.step().await;
                    continue;
                }
                Ok(db_wal::WalCommittedStep::Done) => break,
                Err(error) if database_sync_hello_turn_exhausted(&error) => {
                    database_sync_hello_opportunity(&cancelled, &expired).await?;
                    continue;
                }
                Err(error) => return Err(error),
            };
            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal::WalCommittedRecordStep::Record(db_wal::WalRecord::Command(bytes))) => {
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail head sequence"))?;
                        chain.update(&bytes.hash());
                        if head_seq > replica_head {
                            let envelope = decode_wal_command(bytes, &mut decode_control)?;
                            head_edit_id.clone_from(&envelope.mutation_id.0);
                            envelopes.push(envelope);
                        }
                    }
                    Ok(db_wal::WalCommittedRecordStep::Record(_)) => {}
                    Ok(db_wal::WalCommittedRecordStep::Yield) => {
                        turn.step().await;
                        continue;
                    }
                    Ok(db_wal::WalCommittedRecordStep::Done) => break,
                    Err(error) if database_sync_hello_turn_exhausted(&error) => {
                        database_sync_hello_opportunity(&cancelled, &expired).await?;
                        continue;
                    }
                    Err(error) => return Err(error),
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail commit sequence"))?;
            if envelopes.len() >= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES || (head_seq >= end_head_seq && !envelopes.is_empty()) {
                let frame_commit_seq = if head_seq >= end_head_seq { end_commit_seq } else { commit_seq };
                let frontier = (head_seq, std::mem::take(&mut head_edit_id), frame_commit_seq, *chain.finalize().as_bytes());
                DatabaseSyncHelloTailSlotFree(&handoff).await;
                database_sync_hello_control(&cancelled, &expired)?;
                handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((std::mem::take(&mut envelopes), frontier));
            }
            turn.step().await;
        }
        database_sync_hello_control(&cancelled, &expired)?;
        Ok::<(), DbError>(())
    }
    .await;
    let close = async {
        while records.close_owner_step()? {
            semio_framework_async::yield_once().await;
        }
        Ok::<(), DbError>(())
    }
    .await;
    read.and(close)
}
""", "reader")

edit("""                let storage = owners.storage.clone().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))?;
                let document = ArtifactId(database_sync_hello_clone_string(&server.document.0, &mut ledger, "database sync hello tail document backing")?);
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                let cursor = DatabaseSyncHelloTailCursor { segment: None, segment_head_seq: 0, segment_commit_seq: 0, hashed_head_seq: 0, emitted_head_seq: replica_head, end_head_seq: server.head_seq, end_commit_seq: server.commit_seq, chain: semio_framework_hash::Hasher::new() };
                let tail = DatabaseSyncHelloTail { storage: Some(storage), document, origin, cancelled: cancelled.clone(), expired: expired.clone(), cursor: Some(cursor), page: None, done: false };
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail(Box::new(tail)))""", """                let storage = owners.storage.clone().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))?;
                let document = ArtifactId(database_sync_hello_clone_string(&server.document.0, &mut ledger, "database sync hello tail document backing")?);
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                let tail = DatabaseSyncHelloTail::new(storage, document, origin, replica_head, &server, cancelled.clone(), expired.clone());
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail(Box::new(tail)))""", "tail construction")

# stall-bounded deadline: every produced frame renews it
edit("""    deadline_ms: u64,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
    wake_requested: std::sync::atomic::AtomicBool,""", """    progress_deadline_ms: std::sync::atomic::AtomicU64,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
    wake_requested: std::sync::atomic::AtomicBool,""", "state field")

edit("""            deadline_ms,
            progress,
            wake_requested: std::sync::atomic::AtomicBool::new(false),""", """            progress_deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,
            wake_requested: std::sync::atomic::AtomicBool::new(false),""", "state init")

edit("""    fn deadline_callback(self: &std::sync::Arc<Self>) {
        if self.current() && !matches!(self.progress(), DatabaseSyncHelloProgress::Completed | DatabaseSyncHelloProgress::Cancelled | DatabaseSyncHelloProgress::Fault) {""", """    fn deadline_callback(self: &std::sync::Arc<Self>) {
        let renewed = self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if renewed > self.pool.now_ms() {
            let deadline = std::sync::Arc::downgrade(self);
            self.pool.callback_at(renewed, move || {
                if let Some(state) = deadline.upgrade() {
                    state.deadline_callback();
                }
            });
            return;
        }
        if self.current() && !matches!(self.progress(), DatabaseSyncHelloProgress::Completed | DatabaseSyncHelloProgress::Cancelled | DatabaseSyncHelloProgress::Fault) {""", "deadline renew")

edit("""        match step {
            Ok(DatabaseSyncHelloFollowUpStep::Frame(frame)) => core.frame = Some(Ok(Some(frame))),""", """        match step {
            Ok(DatabaseSyncHelloFollowUpStep::Frame(frame)) => {
                self.progress_deadline_ms.store(self.pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS), std::sync::atomic::Ordering::Release);
                core.frame = Some(Ok(Some(frame)));
            }""", "frame renews deadline")

edit("""    /// ⏳️ Expires one hello that is still running at its deadline. The `WorkerPool` timer holds the
""", """    /// ⏳️ Expires one hello that made no progress for `DATABASE_SYNC_HELLO_DEADLINE_MS`: every frame it produced renewed its
    /// deadline, so a long streamed tail that keeps moving is never cut and the callback re-arms at the renewed one. The
    /// `WorkerPool` timer holds the
""", "deadline doc")

edit("""        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.deadline_ms || attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT {""", """        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire) || attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT {""", "retry uses the progress deadline")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    open(PATH, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
