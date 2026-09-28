#!/usr/bin/env python3
"""🚰 H14 14c P1 (A) follow-up on the streamed hello tail: (1) the first page reads from the document's first retained segment
(a compacted WAL has no segment 0) — the cursor's segment is unknown until the first transaction names it; (2) the frame that
emits the tail's last command carries the welcome's server frontier exactly, since transactions without a command after it
(a snapshot marker, a durable group decision) count in the server's commit sequence. Law: a trailing marker transaction.
Idempotent, region-guarded; `--dry-run` reports only."""
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


edit("""/// @emoji 🧭️ Where a hello tail's next frame starts reading: the segment holding the first command it has not emitted, how
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
}""", """/// @emoji 🧭️ Where a hello tail's next frame starts reading: the segment holding the first command it has not emitted (`None`
/// until a page read the document's first retained segment), how many commands and transactions precede that segment, how
/// far the chain has hashed and the replica has received, where the tail ends (the head and commit sequence of the server
/// frontier the welcome announced), and the chain over every hashed command.
struct DatabaseSyncHelloTailCursor {
    segment: Option<u64>,
    segment_head_seq: u64,
    segment_commit_seq: u64,
    hashed_head_seq: u64,
    emitted_head_seq: u64,
    end_head_seq: u64,
    end_commit_seq: u64,
    chain: semio_framework_hash::Hasher,
}""", "cursor struct")

edit("""    let mut records = db_wal::replay_committed_segment(&wal, &document, cursor.segment, control).await?;""", """    let mut records = match cursor.segment {
        Some(segment) => db_wal::replay_committed_segment(&wal, &document, segment, control).await?,
        None => db_wal::replay_committed_document(&wal, &document, control).await?,
    };""", "page open")

edit("""            if transaction.segment_index() != cursor.segment {
                cursor.segment = transaction.segment_index();""", """            if cursor.segment != Some(transaction.segment_index()) {
                cursor.segment = Some(transaction.segment_index());""", "segment track")

edit("""    cursor.emitted_head_seq = head_seq;
    let chain_hash = *cursor.chain.finalize().as_bytes();
    Ok(DatabaseSyncHelloTailPage { cursor, envelopes, frontier: Some((head_seq, head_edit_id, commit_seq, chain_hash)) })""", """    cursor.emitted_head_seq = head_seq;
    let commit_seq = if head_seq == cursor.end_head_seq { cursor.end_commit_seq } else { commit_seq };
    let chain_hash = *cursor.chain.finalize().as_bytes();
    Ok(DatabaseSyncHelloTailPage { cursor, envelopes, frontier: Some((head_seq, head_edit_id, commit_seq, chain_hash)) })""", "last frame commit seq")

edit("""                let cursor = DatabaseSyncHelloTailCursor { segment: 0, segment_head_seq: 0, segment_commit_seq: 0, hashed_head_seq: 0, emitted_head_seq: replica_head, end_head_seq: server.head_seq, chain: semio_framework_hash::Hasher::new() };""", """                let cursor = DatabaseSyncHelloTailCursor { segment: None, segment_head_seq: 0, segment_commit_seq: 0, hashed_head_seq: 0, emitted_head_seq: replica_head, end_head_seq: server.head_seq, end_commit_seq: server.commit_seq, chain: semio_framework_hash::Hasher::new() };""", "cursor init")

edit("""/// ⚖️ A retained hello streams exactly the commands a replica lacks — for a fresh replica, one mid-way and one caught up —
/// in order, and its last frame's frontier is the welcome's server frontier.
#[semio_framework_async_macros::async_test]
async fn a_retained_hello_streams_exactly_the_missing_tail() {
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail-stream".into();
    seed_wal(&memory, &document, 6).await;
    let ordinary = replay_sync_state(&memory, document.clone()).await.unwrap();""", """/// ⚖️ A retained hello streams exactly the commands a replica lacks — for a fresh replica, one mid-way and one caught up —
/// in order, and its last frame's frontier is the welcome's server frontier, commit sequence included although a
/// transaction without a command (a snapshot marker at floor 0) follows the last command.
#[semio_framework_async_macros::async_test]
async fn a_retained_hello_streams_exactly_the_missing_tail() {
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail-stream".into();
    seed_wal(&memory, &document, 6).await;
    publish_snapshot_marker(&memory, &document, 1, Frontier { document: document.clone(), head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 }).await;
    let ordinary = replay_sync_state(&memory, document.clone()).await.unwrap();
    assert_eq!(ordinary.frontier.commit_seq, 7, "the marker transaction counts in the server's commit sequence");""", "trailing marker law", path=TESTS)

edit("""fn database_sync_hello_retire_vec<T>(owner: &mut Vec<T>, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<(), DbError> {
    let capacity = owner.capacity().checked_mul(size_of::<T>()).ok_or(DbError::LimitExceeded("database sync hello retirement capacity"))?;
    if capacity == 0 {
        return Ok(());
    }
    drop(std::mem::take(owner));
    ledger.release(1, capacity)
}

""", "", "remove retire_vec")

edit("""async fn database_sync_hello_close_tail(tail: &mut Vec<protocol::MutationEnvelope>, ledger: &mut DatabaseSyncHelloBackingLedger, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    let mut control_error = None;
    while let Some(envelope) = tail.pop() {
        let mut close = DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) };
        while close.close_one() {
            semio_framework_async::yield_once().await;
            if control_error.is_none() {
                control_error = database_sync_hello_control(cancelled, expired).err();
            }
        }
    }
    database_sync_hello_retire_vec(tail, ledger)?;
    control_error.map_or(Ok(()), Err)
}

""", "", "remove close_tail")

edit("""    database_sync_hello_retire_vec(&mut bytes, &mut ledger).unwrap();""", """    let capacity = bytes.capacity();
    drop(bytes);
    ledger.release(1, capacity).unwrap();""", "predebit law retire", path=TESTS)

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
