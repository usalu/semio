#!/usr/bin/env python3
"""🚰 H14 14c P1 (A): kernel-db `🔄️sync` — a retained hello never holds the tail a replica lacks: pass 1 derives the server
frontier hashing every command without decoding any, then the tail streams from the WAL as Commands frames of at most
`DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES` envelopes each, ending on a commit boundary and carrying the exact frontier there
(what a peer's relay carries). Each frame is read by one page future over the segment holding its first command; a frame's
backing is charged while it is out and released when it closes. Idempotent, region-guarded; `--dry-run` reports only."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"
TESTS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {PATH: open(PATH, encoding="utf-8").read(), TESTS: open(TESTS, encoding="utf-8").read()}
problems, states = [], []


def edit(old, new, label, path=PATH):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


def edit_region(start, end, new, label, path=PATH):
    text = files[path]
    if new in text:
        states.append("done")
        return
    if text.count(start) != 1:
        problems.append(f"{label}: start found {text.count(start)}")
        states.append("problem")
        return
    begin = text.index(start)
    stop = text.index(end, begin) + len(end)
    files[path] = text[:begin] + new + text[stop:]
    states.append("replace")


edit("""const DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES: usize = 4 * 1024;
""", """const DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES: usize = 4 * 1024;

/// 📦️ The most envelopes one hello tail frame carries before it ends on the next commit boundary: the tail a replica
/// lacks streams as a sequence of such frames — each with the exact frontier after its last commit, as a peer's relay
/// carries — so no hello holds more than one frame of it, however long the document's history (ticket 26/09/23 session
/// 14c: a 13 201-edit document could no longer be opened, its whole tail was decoded into one frame first).
const DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES: usize = 256;
""", "tail frame constant")

edit_region("""/// 🧾️ What a retained hello takes from the WAL: the server frontier, the id of its last command,""", """    Ok(DatabaseSyncHelloReplay { frontier: Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 }, head_edit_id, floor_head_seq, tail })
}
""", """/// 🧾️ What a retained hello takes from the WAL before it answers: the server frontier, the id of its last command and
/// the snapshot floor — every command is hashed into the chain and counted, none is materialized; the tail a replica lacks
/// streams afterwards ([`DatabaseSyncHelloTail`]).
struct DatabaseSyncHelloReplay {
    frontier: Frontier,
    head_edit_id: String,
    floor_head_seq: u64,
}

async fn replay_sync_state_retained(
    storage: &db_storage::DbBackend,
    document: ArtifactId,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    progress: &std::sync::atomic::AtomicU8,
) -> Result<DatabaseSyncHelloReplay, DbError> {
    progress.store(DatabaseSyncHelloProgress::Replay as u8, std::sync::atomic::Ordering::Release);
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal::replay_committed_document(&wal, &document, control).await?;
    let mut decode_control = db_wal::WalCursorControl::stall_bounded(cancelled.clone(), db_wal::WAL_REPLAY_STEP_STALL_BOUND, db_wal::WAL_REPLAY_STEP_FUEL)?;
    let mut chain = semio_framework_hash::Hasher::new();
    let mut head_seq = 0u64;
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    let mut head_edit_id = String::new();
    let mut turn = db_wal::WalTurn::start();
    let replay = async {
        loop {
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
                        progress.store(DatabaseSyncHelloProgress::Decode as u8, std::sync::atomic::Ordering::Release);
                        chain.update(&bytes.hash());
                        head_edit_id = wal_command_id(bytes, &mut decode_control)?;
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello head sequence"))?;
                    }
                    Ok(db_wal::WalCommittedRecordStep::Record(db_wal::WalRecord::SnapshotPub { frontier, .. })) => floor_head_seq = frontier.head_seq,
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
            commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello commit sequence"))?;
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
    replay.and(close)?;
    ledger.observe(usize::from(head_edit_id.capacity() != 0), head_edit_id.capacity(), "database sync hello server frontier head backing")?;
    let chain_hash = if head_seq == 0 { [0; 32] } else { *chain.finalize().as_bytes() };
    Ok(DatabaseSyncHelloReplay { frontier: Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 }, head_edit_id, floor_head_seq })
}

/// 🧭️ Where a hello tail's next frame starts reading: the segment holding the first command it has not emitted, how
/// many commands and transactions precede that segment, how far the chain has hashed and the replica has received, where
/// the tail ends (the server frontier the welcome announced), and the chain over every hashed command.
struct DatabaseSyncHelloTailCursor {
    segment: u64,
    segment_head_seq: u64,
    segment_commit_seq: u64,
    hashed_head_seq: u64,
    emitted_head_seq: u64,
    end_head_seq: u64,
    chain: semio_framework_hash::Hasher,
}

/// 📄️ One read tail frame: its envelopes, the frontier after its last commit (`None` when nothing remained) and the
/// cursor the next frame resumes from.
struct DatabaseSyncHelloTailPage {
    cursor: DatabaseSyncHelloTailCursor,
    envelopes: Vec<protocol::MutationEnvelope>,
    frontier: Option<(u64, String, u64, [u8; 32])>,
}

type DatabaseSyncHelloTailPageFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<DatabaseSyncHelloTailPage, DbError>> + Send>>;

/// 🚰️ The tail a replica lacks, streamed from the WAL one frame at a time: each frame is read by one page future
/// ([`database_sync_hello_tail_page`]) and carries the exact frontier after its last commit; its backing is charged while
/// it is out and released when it closes, so a hello holds at most one frame of the tail however long it is.
struct DatabaseSyncHelloTail {
    storage: Option<std::sync::Arc<db_storage::DbBackend>>,
    document: ArtifactId,
    origin: protocol::ActorId,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    cursor: Option<DatabaseSyncHelloTailCursor>,
    page: Option<DatabaseSyncHelloTailPageFuture>,
    done: bool,
}

impl DatabaseSyncHelloTail {
    /// 🚚️ Starts, polls or finishes the next frame's page read.
    fn drive(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, context: &mut std::task::Context<'_>) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        if self.done {
            return Ok(DatabaseSyncHelloFollowUpStep::Done);
        }
        if self.page.is_none() {
            let cursor = self.cursor.take().ok_or_else(|| DbError::Internal("database sync hello tail cursor missing".to_string()))?;
            if cursor.emitted_head_seq >= cursor.end_head_seq {
                self.cursor = Some(cursor);
                self.done = true;
                return Ok(DatabaseSyncHelloFollowUpStep::Done);
            }
            let storage = self.storage.clone().ok_or_else(|| DbError::Internal("database sync hello tail storage missing".to_string()))?;
            self.page = Some(Box::pin(database_sync_hello_tail_page(storage, self.document.clone(), cursor, self.cancelled.clone(), self.expired.clone())));
        }
        let page = self.page.as_mut().ok_or_else(|| DbError::Internal("database sync hello tail page missing".to_string()))?;
        let std::task::Poll::Ready(read) = std::future::Future::poll(page.as_mut(), context) else { return Ok(DatabaseSyncHelloFollowUpStep::Waiting) };
        self.page = None;
        let DatabaseSyncHelloTailPage { cursor, envelopes, frontier } = read?;
        self.cursor = Some(cursor);
        let Some((head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash)) = frontier else {
            self.done = true;
            return Ok(DatabaseSyncHelloFollowUpStep::Done);
        };
        let frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(self.document.0.clone()), head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
        let frame = protocol::ServerFrame::Commands { envelopes, origin: self.origin.clone(), frontier };
        let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
        ledger.observe(items, bytes, "database sync hello tail frame backing")?;
        Ok(DatabaseSyncHelloFollowUpStep::Frame(frame))
    }

    fn close_one(&mut self) -> bool {
        if self.page.take().is_some() || self.cursor.take().is_some() {
            return true;
        }
        if database_sync_hello_retire_string(&mut self.origin.0) || database_sync_hello_retire_string(&mut self.document.0) {
            return true;
        }
        self.storage.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.page.is_none() && self.cursor.is_none() && self.origin.0.capacity() == 0 && self.document.0.capacity() == 0 && self.storage.is_none()
    }
}

/// 🚰️ Reads one frame of a hello tail: replays the WAL from `cursor`'s segment, skips the commands the chain already
/// covers, hashes the rest, decodes those the replica lacks and stops on the first commit boundary after
/// [`DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES`] envelopes or at the tail's end. The cursor it answers resumes in the segment
/// of the last transaction it read, so a frame reads at most that segment again.
async fn database_sync_hello_tail_page(
    storage: std::sync::Arc<db_storage::DbBackend>,
    document: ArtifactId,
    mut cursor: DatabaseSyncHelloTailCursor,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<DatabaseSyncHelloTailPage, DbError> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal::replay_committed_segment(&wal, &document, cursor.segment, control).await?;
    let mut decode_control = db_wal::WalCursorControl::stall_bounded(cancelled.clone(), db_wal::WAL_REPLAY_STEP_STALL_BOUND, db_wal::WAL_REPLAY_STEP_FUEL)?;
    let mut envelopes: Vec<protocol::MutationEnvelope> = Vec::new();
    let mut head_seq = cursor.segment_head_seq;
    let mut commit_seq = cursor.segment_commit_seq;
    let mut head_edit_id = String::new();
    let mut turn = db_wal::WalTurn::start();
    let read = async {
        while head_seq < cursor.end_head_seq && envelopes.len() < DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES {
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
            if transaction.segment_index() != cursor.segment {
                cursor.segment = transaction.segment_index();
                cursor.segment_head_seq = head_seq;
                cursor.segment_commit_seq = commit_seq;
            }
            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal::WalCommittedRecordStep::Record(db_wal::WalRecord::Command(bytes))) => {
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail head sequence"))?;
                        if head_seq > cursor.hashed_head_seq {
                            cursor.chain.update(&bytes.hash());
                            cursor.hashed_head_seq = head_seq;
                            if head_seq > cursor.emitted_head_seq {
                                let envelope = decode_wal_command(bytes, &mut decode_control)?;
                                head_edit_id.clone_from(&envelope.mutation_id.0);
                                envelopes.push(envelope);
                            }
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
    read.and(close)?;
    if envelopes.is_empty() {
        cursor.emitted_head_seq = cursor.end_head_seq;
        return Ok(DatabaseSyncHelloTailPage { cursor, envelopes, frontier: None });
    }
    cursor.emitted_head_seq = head_seq;
    let chain_hash = *cursor.chain.finalize().as_bytes();
    Ok(DatabaseSyncHelloTailPage { cursor, envelopes, frontier: Some((head_seq, head_edit_id, commit_seq, chain_hash)) })
}
""", "pass 1 + tail stream")

edit("""enum DatabaseSyncHelloFollowUp {
    None,
    Tail { envelopes: Option<Vec<protocol::MutationEnvelope>>, closing: Option<DatabaseSyncHelloEnvelopeClose>, origin: Option<protocol::ActorId>, frontier: Option<protocol::RuntimeFrontierSummary> },""", """enum DatabaseSyncHelloFollowUp {
    None,
    Tail(Box<DatabaseSyncHelloTail>),""", "follow-up enum")

edit("""impl DatabaseSyncHelloFollowUp {
    fn drive_one(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<Option<Option<protocol::ServerFrame>>, DbError> {
        let mut grant = DatabaseSyncHelloGrant::fresh()?;
        self.drive_one_with_grant(ledger, cancelled, expired, &mut grant)
    }

    fn drive_one_with_grant(
        &mut self,
        ledger: &mut DatabaseSyncHelloBackingLedger,
        cancelled: &std::sync::atomic::AtomicBool,
        expired: &std::sync::atomic::AtomicBool,
        grant: &mut DatabaseSyncHelloGrant,
    ) -> Result<Option<Option<protocol::ServerFrame>>, DbError> {
        grant.check(cancelled, expired)?;
        match self {
            Self::None => Ok(Some(None)),
            Self::Tail { envelopes, closing, origin, frontier } => {
                if closing.is_some() {
                    return Err(DbError::Closed);
                }
                grant.check(cancelled, expired)?;
                let Some(envelopes) = envelopes.take() else { return Ok(Some(None)) };
                let origin = origin.take().ok_or_else(|| DbError::Internal("database sync hello tail origin missing".to_string()))?;
                let frontier = frontier.take().ok_or_else(|| DbError::Internal("database sync hello tail frontier missing".to_string()))?;
                Ok(Some(Some(protocol::ServerFrame::Commands { envelopes, origin, frontier })))
            }
            Self::Snapshot { pages, chunk_bytes, offset, page, page_offset, seq, chunk, done } => {
                if *done {
                    return Ok(Some(None));
                }""", """/// 🪜️ What one drive of a hello follow-up produced: a frame, the end of the stream, progress toward a frame (drive
/// again), or a read in flight whose completion wakes the hello.
enum DatabaseSyncHelloFollowUpStep {
    Frame(protocol::ServerFrame),
    Done,
    Progress,
    Waiting,
}

impl DatabaseSyncHelloFollowUp {
    fn drive_one(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool, context: &mut std::task::Context<'_>) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        let mut grant = DatabaseSyncHelloGrant::fresh()?;
        self.drive_one_with_grant(ledger, cancelled, expired, &mut grant, context)
    }

    fn drive_one_with_grant(
        &mut self,
        ledger: &mut DatabaseSyncHelloBackingLedger,
        cancelled: &std::sync::atomic::AtomicBool,
        expired: &std::sync::atomic::AtomicBool,
        grant: &mut DatabaseSyncHelloGrant,
        context: &mut std::task::Context<'_>,
    ) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        grant.check(cancelled, expired)?;
        match self {
            Self::None => Ok(DatabaseSyncHelloFollowUpStep::Done),
            Self::Tail(tail) => tail.drive(ledger, context),
            Self::Snapshot { pages, chunk_bytes, offset, page, page_offset, seq, chunk, done } => {
                if *done {
                    return Ok(DatabaseSyncHelloFollowUpStep::Done);
                }""", "drive_one head")

edit("""                    *done = true;
                    return Ok(Some(Some(protocol::ServerFrame::SnapshotDone { seq_count: *seq })));""", """                    *done = true;
                    return Ok(DatabaseSyncHelloFollowUpStep::Frame(protocol::ServerFrame::SnapshotDone { seq_count: *seq }));""", "snapshot done frame")

edit("""                    let frame = protocol::ServerFrame::SnapshotChunk { seq: *seq, bytes };
                    *seq = next_seq;
                    return Ok(Some(Some(frame)));
                }
                Ok(None)
            }
        }
    }""", """                    let frame = protocol::ServerFrame::SnapshotChunk { seq: *seq, bytes };
                    *seq = next_seq;
                    return Ok(DatabaseSyncHelloFollowUpStep::Frame(frame));
                }
                Ok(DatabaseSyncHelloFollowUpStep::Progress)
            }
        }
    }""", "snapshot chunk frame")

edit_region("""            Self::Tail { envelopes, closing, origin, frontier } => {
                if let Some(cursor) = closing.as_mut() {""", """                Ok(false)
            }
            Self::Snapshot { pages, chunk, .. } => {""", """            Self::Tail(tail) => Ok(tail.close_one()),
            Self::Snapshot { pages, chunk, .. } => {""", "follow-up close")

edit("""            Self::Tail { envelopes, closing, origin, frontier } => envelopes.as_ref().is_none_or(|owners| owners.is_empty() && owners.capacity() == 0) && closing.is_none() && origin.is_none() && frontier.is_none(),""", """            Self::Tail(tail) => tail.terminal_is_empty(),""", "follow-up terminal")

edit("""        let DatabaseSyncHelloReplay { frontier: mut server, head_edit_id, floor_head_seq, tail: mut envelopes } = replay_sync_state_retained(owners.storage()?, document, replica_head, cancelled.clone(), expired.clone(), &mut ledger, &progress).await?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let replica = owners.hello_frontier.as_ref();
        if let Some(replica) = replica.as_ref() {
            if replica.document_id.0 != server.document.0 {
                database_sync_hello_close_tail(&mut envelopes, &mut ledger, &cancelled, &expired).await?;
                return Err(DbError::InvalidArgument("database sync hello frontier document mismatch".to_string()));
            }
            if replica.head_edit_ordinal > server.head_seq {
                database_sync_hello_close_tail(&mut envelopes, &mut ledger, &cancelled, &expired).await?;
                return Err(DbError::InvalidArgument("database sync hello frontier ahead of server".to_string()));
            }
        }
        progress.store(DatabaseSyncHelloProgress::Bootstrap as u8, std::sync::atomic::Ordering::Release);
        let (bootstrap, follow_up) = if replica_head >= floor_head_seq {
            if envelopes.is_empty() {
                (protocol::Bootstrap::None, DatabaseSyncHelloFollowUp::None)
            } else {
                let frontier_document = database_sync_hello_clone_string(&server.document.0, &mut ledger, "database sync hello tail frontier document backing")?;
                let frontier_head = database_sync_hello_clone_string(&head_edit_id, &mut ledger, "database sync hello tail frontier head backing")?;
                let frontier = protocol::RuntimeFrontierSummary {
                    document_id: protocol::ArtifactId(frontier_document),
                    head_edit_ordinal: server.head_seq,
                    head_edit_id: frontier_head,
                    last_commit_seq: server.commit_seq,
                    chain_hash: server.chain_hash,
                };
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail { envelopes: Some(envelopes), closing: None, origin: Some(origin), frontier: Some(frontier) })
            }
        } else {
            database_sync_hello_close_tail(&mut envelopes, &mut ledger, &cancelled, &expired).await?;
            let snapshots""", """        let DatabaseSyncHelloReplay { frontier: mut server, head_edit_id, floor_head_seq } = replay_sync_state_retained(owners.storage()?, document, cancelled.clone(), expired.clone(), &mut ledger, &progress).await?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let replica = owners.hello_frontier.as_ref();
        if let Some(replica) = replica.as_ref() {
            if replica.document_id.0 != server.document.0 {
                return Err(DbError::InvalidArgument("database sync hello frontier document mismatch".to_string()));
            }
            if replica.head_edit_ordinal > server.head_seq {
                return Err(DbError::InvalidArgument("database sync hello frontier ahead of server".to_string()));
            }
        }
        progress.store(DatabaseSyncHelloProgress::Bootstrap as u8, std::sync::atomic::Ordering::Release);
        let (bootstrap, follow_up) = if replica_head >= floor_head_seq {
            if replica_head == server.head_seq {
                (protocol::Bootstrap::None, DatabaseSyncHelloFollowUp::None)
            } else {
                let storage = owners.storage.clone().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))?;
                let document = ArtifactId(database_sync_hello_clone_string(&server.document.0, &mut ledger, "database sync hello tail document backing")?);
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                let cursor = DatabaseSyncHelloTailCursor { segment: 0, segment_head_seq: 0, segment_commit_seq: 0, hashed_head_seq: 0, emitted_head_seq: replica_head, end_head_seq: server.head_seq, chain: semio_framework_hash::Hasher::new() };
                let tail = DatabaseSyncHelloTail { storage: Some(storage), document, origin, cancelled: cancelled.clone(), expired: expired.clone(), cursor: Some(cursor), page: None, done: false };
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail(Box::new(tail)))
            }
        } else {
            let snapshots""", "execute bootstrap")

edit("""        let frame = match (&mut execution.prepared, &mut execution.ledger) {
            (Ok(prepared), Some(ledger)) => prepared.follow_up.drive_one(ledger, &self.cancelled, &self.expired),
            (Ok(_), None) => Err(DbError::Internal("database sync hello backing ledger missing".to_string())),
            (Err(_), _) => Ok(Some(None)),
        };
        match frame {
            Ok(Some(frame)) => {
                let terminal = frame.is_none();
                core.frame = Some(Ok(frame));
                if terminal {
                    self.progress.store(DatabaseSyncHelloProgress::Completed as u8, std::sync::atomic::Ordering::Release);
                }
            }
            Ok(None) => self.demand.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => core.frame = Some(Err(error)),
        }
        drop(core);
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        self.demand.load(std::sync::atomic::Ordering::Acquire)
    }""", """        let waker = std::task::Waker::from(self.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let step = match (&mut execution.prepared, &mut execution.ledger) {
            (Ok(prepared), Some(ledger)) => prepared.follow_up.drive_one(ledger, &self.cancelled, &self.expired, &mut context),
            (Ok(_), None) => Err(DbError::Internal("database sync hello backing ledger missing".to_string())),
            (Err(_), _) => Ok(DatabaseSyncHelloFollowUpStep::Done),
        };
        let waiting = matches!(step, Ok(DatabaseSyncHelloFollowUpStep::Waiting));
        match step {
            Ok(DatabaseSyncHelloFollowUpStep::Frame(frame)) => core.frame = Some(Ok(Some(frame))),
            Ok(DatabaseSyncHelloFollowUpStep::Done) => {
                core.frame = Some(Ok(None));
                self.progress.store(DatabaseSyncHelloProgress::Completed as u8, std::sync::atomic::Ordering::Release);
            }
            Ok(DatabaseSyncHelloFollowUpStep::Progress | DatabaseSyncHelloFollowUpStep::Waiting) => self.demand.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => core.frame = Some(Err(error)),
        }
        drop(core);
        if waiting {
            return false;
        }
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        self.demand.load(std::sync::atomic::Ordering::Acquire)
    }""", "poll_one follow-up")

edit("""fn database_sync_hello_returned_frame_credit(frame: &protocol::ServerFrame) -> Result<(usize, usize), DbError> {
    match frame {
        protocol::ServerFrame::SnapshotChunk { bytes, .. } => Ok((usize::from(bytes.backing_bytes() != 0), bytes.backing_bytes())),
        _ => Ok((0, 0)),
    }
}""", """fn database_sync_hello_returned_frame_credit(frame: &protocol::ServerFrame) -> Result<(usize, usize), DbError> {
    match frame {
        protocol::ServerFrame::SnapshotChunk { bytes, .. } => Ok((usize::from(bytes.backing_bytes() != 0), bytes.backing_bytes())),
        protocol::ServerFrame::Commands { envelopes, origin, frontier } => {
            let label = "database sync hello tail frame backing";
            let shell = envelopes.capacity().checked_mul(size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded(label))?;
            let mut items = 0usize;
            let mut bytes = 0usize;
            for capacity in [shell, origin.0.capacity(), frontier.document_id.0.capacity(), frontier.head_edit_id.capacity()] {
                if capacity != 0 {
                    items = items.checked_add(1).ok_or(DbError::LimitExceeded(label))?;
                    bytes = bytes.checked_add(capacity).ok_or(DbError::LimitExceeded(label))?;
                }
            }
            for envelope in envelopes {
                let (envelope_items, envelope_bytes) = database_sync_hello_envelope_credit(envelope)?;
                items = items.checked_add(envelope_items).ok_or(DbError::LimitExceeded(label))?;
                bytes = bytes.checked_add(envelope_bytes).ok_or(DbError::LimitExceeded(label))?;
            }
            Ok((items, bytes))
        }
        _ => Ok((0, 0)),
    }
}""", "frame credit")

edit("""/// 🧮️ The heap backing one decoded command holds, in the ledger's units: one item per non-empty
/// owner, its capacity in bytes.
fn database_sync_hello_envelope_credit(envelope: &protocol::MutationEnvelope) -> Result<(usize, usize), DbError> {
    let label = "database sync hello cumulative envelope backing";""", """/// 🧮️ The heap backing one decoded command holds, in the ledger's units: one item per non-empty
/// owner, its capacity in bytes.
fn database_sync_hello_envelope_credit(envelope: &protocol::MutationEnvelope) -> Result<(usize, usize), DbError> {
    let label = "database sync hello envelope backing";""", "envelope credit label")

edit("""/// counted and hashed into the same frontier the whole-WAL replay derives, and the last command's
/// id is the server head whether or not it was decoded.
#[semio_framework_async_macros::async_test]
async fn retained_hello_replay_decodes_only_the_replicas_missing_tail() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail".into();
    seed_wal(&storage, &document, 6).await;
    let ordinary = replay_sync_state(&storage, document.clone()).await.unwrap();
    let storage = db_storage::DbBackend::Memory(storage);
    for replica_head in [0u64, 4, 6] {
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress = std::sync::atomic::AtomicU8::new(0);
        let mut replay = replay_sync_state_retained(&storage, document.clone(), replica_head, cancelled.clone(), expired.clone(), &mut ledger, &progress).await.unwrap();
        assert_eq!(replay.frontier, ordinary.frontier);
        assert_eq!(replay.head_edit_id, "op-5");
        assert_eq!(replay.tail, ordinary.commands[replica_head as usize..].to_vec());
        database_sync_hello_close_tail(&mut replay.tail, &mut ledger, &cancelled, &expired).await.unwrap();
    }
}""", """/// counted and hashed into the same frontier the whole-WAL replay derives, and the last command's
/// id is the server head, while no command is materialized.
#[semio_framework_async_macros::async_test]
async fn retained_hello_replay_derives_the_server_frontier_without_decoding_the_tail() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail".into();
    seed_wal(&storage, &document, 6).await;
    let ordinary = replay_sync_state(&storage, document.clone()).await.unwrap();
    let storage = db_storage::DbBackend::Memory(storage);
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let progress = std::sync::atomic::AtomicU8::new(0);
    let replay = replay_sync_state_retained(&storage, document.clone(), cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert_eq!(replay.frontier, ordinary.frontier);
    assert_eq!(replay.head_edit_id, "op-5");
    assert_eq!((ledger.items, ledger.bytes), (1, replay.head_edit_id.capacity()), "only the server head id is held");
}

/// 🚰️ Drives one retained hello to its end and answers the welcome's server frontier with every tail frame's
/// envelopes and frontier, acknowledging each frame, plus the largest backing the hello held while a frame was out.
async fn stream_hello_tail(storage: std::sync::Arc<db_storage::DbBackend>, document: ArtifactId, replica: Option<protocol::RuntimeFrontierSummary>) -> (protocol::RuntimeFrontierSummary, Vec<(Vec<protocol::MutationEnvelope>, protocol::RuntimeFrontierSummary)>, usize, usize) {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, replica, String::from("tail-stream-session"), protocol::ActorId(String::from("tail-stream-origin")), 64 * 1024).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    let server = match welcome.frame().unwrap() {
        protocol::ServerFrame::Welcome { server_frontier, .. } => server_frontier.clone(),
        other => panic!("expected Welcome, got {other:?}"),
    };
    welcome.acknowledge().unwrap();
    let (mut frames, mut peak_items, mut peak_bytes) = (Vec::new(), 0usize, 0usize);
    while let Some(frame) = session.next_frame().await.unwrap() {
        {
            let state = session.state.as_ref().unwrap();
            let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let ledger = core.execution.as_ref().and_then(|execution| execution.ledger.as_ref()).unwrap();
            peak_items = peak_items.max(ledger.items);
            peak_bytes = peak_bytes.max(ledger.bytes);
        }
        match frame.frame().unwrap() {
            protocol::ServerFrame::Commands { envelopes, frontier, .. } => frames.push((envelopes.clone(), frontier.clone())),
            other => panic!("expected a tail Commands frame, got {other:?}"),
        }
        frame.acknowledge().unwrap();
    }
    (server, frames, peak_items, peak_bytes)
}

/// ⚖️ A retained hello streams exactly the commands a replica lacks — for a fresh replica, one mid-way and one caught up —
/// in order, and its last frame's frontier is the welcome's server frontier.
#[semio_framework_async_macros::async_test]
async fn a_retained_hello_streams_exactly_the_missing_tail() {
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail-stream".into();
    seed_wal(&memory, &document, 6).await;
    let ordinary = replay_sync_state(&memory, document.clone()).await.unwrap();
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    for replica_head in [0u64, 4, 6] {
        let replica = (replica_head > 0).then(|| protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: replica_head, head_edit_id: format!("op-{}", replica_head - 1), last_commit_seq: replica_head, chain_hash: [0; 32] });
        let (server, frames, _, _) = stream_hello_tail(storage.clone(), document.clone(), replica).await;
        let streamed: Vec<protocol::MutationEnvelope> = frames.iter().flat_map(|(envelopes, _)| envelopes.clone()).collect();
        assert_eq!(streamed, ordinary.commands[replica_head as usize..].to_vec(), "replica head {replica_head}");
        if let Some((_, last)) = frames.last() {
            assert_eq!(*last, server);
        } else {
            assert_eq!(replica_head, 6);
        }
    }
}

/// ⚖️ A document of 50 000 single-command commits opens for a fresh replica: its tail streams as 196 frames of at most
/// `DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES` envelopes on commit boundaries, each frontier exact (head and commit counts)
/// and the last equal to the welcome's, while the hello never holds more than about one frame — the old hello decoded the
/// whole tail first and refused at ~8 000 envelopes (`cumulative envelope backing`). A replica at 40 000 receives exactly
/// the last 10 000.
#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn a_fifty_thousand_edit_document_streams_to_a_fresh_replica_within_one_frame_of_backing() {
    const EDITS: u64 = 50_000;
    let dir = std::env::temp_dir().join(format!("db_sync_fifty_thousand_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let fs = db_storage::FsStorage::open(db_storage::db_io_test_pool(), &dir).await.unwrap();
    let document: ArtifactId = "doc-fifty-thousand".into();
    let mut wal = db_actor::block_on(ArtifactWal::create(&fs, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    for i in 0..EDITS {
        let envelope = sample_envelope(&format!("op-{i}"), i).await;
        let mut records = db_wal::WalRecordBatch::new();
        assert!(records.push(command_record(&envelope).await).is_ok());
        wal.submit(&fs, &[], &records, DurabilityClass::Os, i).await.unwrap();
        while records.close_step().unwrap() {}
    }
    wal.close().await.unwrap();
    let storage = std::sync::Arc::new(db_storage::DbBackend::Fs(fs));
    let (server, frames, peak_items, peak_bytes) = stream_hello_tail(storage.clone(), document.clone(), None).await;
    assert_eq!(server.head_edit_ordinal, EDITS);
    assert_eq!(frames.len(), EDITS.div_ceil(DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES as u64) as usize);
    let mut expected = 0u64;
    for (envelopes, frontier) in &frames {
        assert!(!envelopes.is_empty() && envelopes.len() <= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES);
        for envelope in envelopes {
            assert_eq!(envelope.mutation_id.0, format!("op-{expected}"));
            expected += 1;
        }
        assert_eq!((frontier.head_edit_ordinal, frontier.last_commit_seq), (expected, expected), "each frame ends on the commit of its last command");
        assert_eq!(frontier.head_edit_id, format!("op-{}", expected - 1));
    }
    assert_eq!(expected, EDITS);
    assert_eq!(frames.last().unwrap().1, server, "the last frame reaches the welcome's frontier, chain hash included");
    assert!(peak_items <= 16 * DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES, "the hello held {peak_items} items while a frame was out");
    assert!(peak_bytes <= 4 * 1024 * 1024, "the hello held {peak_bytes} bytes while a frame was out");
    let replica = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 40_000, head_edit_id: String::from("op-39999"), last_commit_seq: 40_000, chain_hash: [0; 32] };
    let (_, frames, _, _) = stream_hello_tail(storage.clone(), document, Some(replica)).await;
    let streamed: Vec<String> = frames.iter().flat_map(|(envelopes, _)| envelopes.iter().map(|envelope| envelope.mutation_id.0.clone())).collect();
    assert_eq!(streamed, (40_000..EDITS).map(|i| format!("op-{i}")).collect::<Vec<_>>());
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}""", "tail replay law", path=TESTS)

edit("""    let mut retained = replay_sync_state_retained(&storage, document, 0, cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert!(retained.tail.is_empty());
    assert_eq!(retained.floor_head_seq, 0);
    assert_eq!(retained.frontier, ordinary.frontier);
    database_sync_hello_retire_vec(&mut retained.tail, &mut ledger).unwrap();
    assert_eq!(ledger.items, 0);
    assert_eq!(ledger.bytes, 0);""", """    let retained = replay_sync_state_retained(&storage, document, cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert_eq!(retained.floor_head_seq, 0);
    assert_eq!(retained.frontier, ordinary.frontier);
    assert_eq!(ledger.items, 0);
    assert_eq!(ledger.bytes, 0);""", "neutral replay law", path=TESTS)

edit("""    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_copy), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));""", """    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_copy, &mut std::task::Context::from_waker(std::task::Waker::noop())), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));""", "grant law 1", path=TESTS)

edit("""    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_publication), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));""", """    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_publication, &mut std::task::Context::from_waker(std::task::Waker::noop())), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));""", "grant law 2", path=TESTS)

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
