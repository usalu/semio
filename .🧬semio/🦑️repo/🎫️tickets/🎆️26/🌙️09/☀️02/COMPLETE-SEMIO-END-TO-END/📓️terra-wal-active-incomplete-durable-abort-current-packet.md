# Active WAL Transaction Recovery Must Durably Abort

## Current frontier

The neutral corpus already says that a physically committed `Begin(7)` with zero or more body frames in the highest **active** segment is accepted, advances `nextTxId` to `8`, emits no transaction, and requires `recoverAbort: "7"` ([fixture](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️fixtures/🧾️committed-transactions/🔣️.json:237)).  The current opener instead rejects it: `validate_wal_prefix` always calls `WalTransactionGate::advance_segment()` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1980)), which deliberately calls `finish_segment(false)`.

This is recoverable only where the current storage state is the last segment and `Active`.  An incomplete transaction in a sealed segment, or across a segment boundary, remains corruption and must not be repaired.  The existing gate already expresses that distinction: `finish_segment(true)` returns the active transaction id; `finish_segment(false)` rejects it ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1404)).

## Small coherent change

Keep validation and mutation separate.  Refactor `validate_wal_prefix` to return a private result such as:

```rust
struct WalValidatedPrefix {
    records: u64,
    next_tx_id: u64,
    incomplete_active_tx: Option<u64>,
}
```

It must take `writable_highest: bool`, call `gate.finish_segment(writable_highest)` after the complete physical prefix has been chain-verified and logically scanned, and call `gate.advance_segment()` only for a non-highest segment.  It must not decode or retain any body payload to make this decision.  `WalTransactionGate` has only copied `WalRecordFrame` spans, and its `TxAbort` grammar already checks the same id and clears the frames ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1360)).

In the writable branch of `ArtifactWal::open_with_control` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2315)), use this exact order:

1. Verify retained framing, every physical commit and the WAL chain; derive `end`, `tail`, and the optional incomplete id.  A begin/body located only in `span.tail()` is not physically committed and therefore is discarded by the ordinary tail recovery; it must not receive an abort.
2. If an abort is required, preflight `end + wal_frame_bytes(8) + COMMIT_FRAME_LEN <= DB_IO_MAX_READ_BYTES`.  This is 496 KiB, not the generic one-MiB field ceiling ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:63)).  On overflow, return `LimitExceeded` before truncation or append: the transaction cannot legally be moved to a successor because transaction spans across segments are corrupt.
3. Copy exactly `[0, end)` into `SharedBuf` and resume through `SegmentWriter::resume_existing_verified`; this seeds the verified sequence and hash chain and sets `flushed_len = end` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2071)).
4. Take one final `control.grant()` *before* the repair ownership boundary.  After that grant, do not inspect cancellation until the terminal repair either succeeds or fails.
5. If `tail != 0`, call `truncate_tail(document, index, end)` then `sync(..., Fsync)`.  This is safe only because the segment was just proven active; `WalStorage` explicitly prohibits truncating sealed segments ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4616)).
6. If `incomplete_active_tx == Some(tx_id)`, append exactly `WalRecord::TxAbort { tx_id }`, then call `commit_and_flush(storage, Fsync)` once.  Its existing primitive commits the SPR record, appends the exact suffix, checks the returned total length, and fsyncs before clearing pending state ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2172)).  It therefore writes the abort and its physical commit as one hash-chained durable suffix.

Do not `create_segment`, `seal`, rotate, rewrite the header, or alter the segment index.  The open caller's exclusive document-write authority is the only authority needed; `segment_state == Active` at the highest index admits append.  The restored transaction sequence is the gate's checked `next_tx_id`: it was advanced at the already-verified `Begin`, and the synthetic abort neither consumes nor increments an id.  A `Begin(u64::MAX)` is already rejected by the gate before recovery selection ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1370)).

The smallest useful public observation is `WalRecoveryReport::recovered_abort_tx_id: Option<u64>`, set only after the abort's `Fsync` returns `Ok(())`; it maps directly to the neutral `recoverAbort` expectation.  Keep `records_replayed` as the count of pre-existing verified records, not the newly generated abort.

## Failure and owner handling

The repair must use the existing poisoned-writer retirement discipline: on any append/commit/copy/sync error, set `segment.writer = None`, drive its `SharedBuf` to terminal, then let the outer loop drive `DbIoPages` and `DbIoU64List` to terminal.  This is already the open repair error path ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2375)); it should become one private helper so the abort branch cannot accidentally call ordinary `close_step` with pending records.  Cleanup is deliberately ungated.

The fault outcomes are well-defined:

| Fault point | First open | Next open |
| --- | --- | --- |
| abort `append` errors before storage | error; old committed incomplete prefix unchanged | append exactly one abort |
| append is short/torn | error; old prefix plus uncommitted physical tail | truncate that tail, append exactly one abort |
| `Fsync` errors after append | error; writer is poisoned, but the whole abort commit may already be present | see valid abort, do not append another |

`FaultStorage` already supplies those precise seams: `fail_nth_write`, `torn_write_at`, and `fail_nth_sync` ([testkit](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:258)).  Do not treat `fsync_lies` as proof of restart durability.

## Required native laws

Add the laws beside the existing recovery and fail-stop helpers, not to the generic gate-only oracle.

1. **Neutral active recovery + double reopen.**  For both `active-incomplete-needs-durable-abort` and `active-empty-begin-needs-durable-abort`, build storage with `committed_fixture_storage` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2604)), open, require `recovered_abort_tx_id == Some(7)`, `next_tx_id == 8`, no committed transaction, and exactly one raw `TxAbort(7)`.  Capture bytes; close; reopen; require `recovered_abort_tx_id == None`, byte-for-byte equality, one abort total, and the next submitted transaction id `8`.  Extend the test-local `ReplaySummary` with `Abort(u64)` rather than treating it as `Other` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2557)).
2. **No sealed repair.**  Run `sealed-incomplete-is-corrupt` and `cross-segment-open-transaction-is-corrupt`; assert `Corrupt`, all bytes byte-identical, and every segment state unchanged.  The existing neutral gate law already names these rows ([fixture](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️fixtures/🧾️committed-transactions/🔣️.json:295)).
3. **Abort append fail and torn append.**  Seed a valid active incomplete prefix, then target the next `FaultStorage` append.  An outright append failure leaves identical bytes; a short write leaves an exact prefix plus a tail.  Clear the script and reopen the inner storage: it must yield one abort, no logical transaction, and thereafter a clean byte-identical second reopen.  Follow the existing boundary-counter setup at [WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:3515).
4. **Abort sync failure is idempotent.**  Target the next sync.  The first open returns `Io`; the physical bytes can contain a full abort commit.  A fault-free reopen must retain those exact bytes, report no torn tail, and not append a second abort.  This matches the existing fail-stop sync principle ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:3569)).
5. **Cancellation boundary.**  A cancelled/deadline-exhausted `WalCursorControl` before the final repair grant returns `Unavailable` with no byte or lifecycle mutation.  Once that grant has admitted repair, cancellation must not be polled between truncation and abort fsync.  A test-only delegating `WalStorage` that flips the control's cancellation flag from the abort `append` call can prove the intended atomic boundary: successful open still fsyncs/returns the one abort; a subsequent open is byte-identical.  This needs no production compatibility surface.

No builds were run and no product source was edited for this audit.

## 2026-09-05 implementation review

The currently landed implementation follows the required mutation order.  `validate_wal_prefix` now returns `WalValidatedPrefix`, leaves an incomplete transaction only for the writable highest segment, and preserves the already-checked `next_tx_id` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1982)).  The opener verifies framing and the chain before it preflights `end + abort-frame + commit-frame`, copies only the verified prefix, takes its final grant, then performs `truncate_tail`/Fsync and the synthetic abort/Fsync ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2363)).  The report is assigned only after that complete repair returns ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2403)).  I found no route from raw bytes to verification authority and no duplicate-abort path in the reviewed source.

`retire_poisoned` is the right failure owner: it drops the SPR writer before closing `SharedBuf`, so an append/commit/sync error cannot invoke ordinary close with a pending record ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2080), [WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2243)).  `DbIoPages` is then drained outside the inner outcome regardless of success or repair failure ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2402)).  A complete append followed by failed Fsync remains correctly fail-stop: the next opener sees the committed abort and does not add another.

### Remaining executable gaps (test-strengthening, not a source rejection)

1. The exact-capacity law stops after opening the repaired 496 KiB segment ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2821)).  Extend its `extra == 0` branch with one small `submit(Fsync)`: it must first seal segment 0, create segment 1 with a predecessor equal to the repaired abort tip, place the command only in segment 1, and reopen with one abort total.  The source appears to perform that reservation-first rotation ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2441)); this law protects that required usable-open result.
2. The cancellation wrapper flips only at `append`, so it proves that the abort Fsync ignores cancellation but not the earlier tail-repair half of the same admitted boundary ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2764)).  Add a wrapper which flips cancellation from `truncate_tail`, seed an incomplete transaction plus a torn tail, and require: tail Fsync, abort append/Fsync, one abort, and a byte-identical second reopen.  This pins the intended no-poll interval from the final grant through both mutations.
3. The fault law exercises abort append/sync against a clean physical prefix ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2711)).  Add one case with a torn tail and an injected abort-append failure, plus one with tail-truncation Fsync failure.  First open must fail closed; fault-free reopening may discard only the tail and append exactly one abort.  This covers the only intermediate durable state (`tail` already truncated) not exercised by the four current laws.

These are acceptance holes only.  The reviewed source already has correct final-grant placement, checked 496 KiB preflight, transaction-id retention, poisonous-writer retirement, and post-repair at-capacity rotation logic.  No build was run and no product source was edited in this review.

## Filesystem Double-Reopen Native Law Packet

The existing filesystem-capacity law supplies the correct native scratch convention: root at `${SEMIO_TEST_ARTIFACT_DIR}/wal-active-abort-<pid>-<nonce>-<row>` when the environment variable is set, otherwise under `std::env::temp_dir()`, opened with `FsStorage::open(db_io_test_pool(), &root)` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:3255)).  The physical active segment is exactly `root/wal/<safe-document>/segment-00000000000000000000.bin`; absence of its sibling `.sealed` is the active lifecycle ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6576), [storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6653)).  `FsStorage::close` actually retires its typed backend rather than merely dropping a convenience handle ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7272)).

Add one `#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]` async native law beside the current four active-abort laws.  Extract the test-only `committed_fixture_storage` frame loop into `write_committed_fixture(storage: &impl WalStorage, row, document)` so it can seed the existing neutral rows `active-incomplete-needs-durable-abort` and `active-empty-begin-needs-durable-abort` through the real `SegmentWriter`/`WalStorage` path, closing every temporary `WalRecord` and the writer after its Fsync.

For each row, use three non-overlapping lexical scopes:

1. Open the seed `FsStorage`; write the exact physically committed incomplete row; assert `segment_state(0) == Active`; explicitly `close().await` it; then leave scope.  No `ArtifactWal`, `SegmentWriter`, record, pages, or backend from the seed scope may survive.
2. Open a new `FsStorage` at the same root; call `ArtifactWal::open`; require `recovered_abort_tx_id == Some(7)`, no committed transaction, and exactly one raw abort.  Capture segment bytes, drive `wal.close_step()` to terminal, explicitly close storage, then leave scope.
3. Open a third independent `FsStorage` at that root; call `ArtifactWal::open`; require `recovered_abort_tx_id == None`, byte equality with phase 2, active state, and one abort total.  Submit a small Fsync command and require `tx_id == 8`; drain the WAL and close storage.

Do not remove the scratch root during the law; the registered runner owns `SEMIO_TEST_ARTIFACT_DIR`, matching the current capacity convention.  The law proves an actual `sync_all()` boundary because the filesystem backend executes `sync_all` for Fsync ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6918)); it does not overclaim a simulated memory Fsync.

### Static selector audit

The current `wal-committed-transactions-check --native` list contains **22** selectors ([script](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:102)).  A source-only exact-definition scan finds each selector once: 12 in `db_wal`, 6 in `db_artifact`, 2 in `db_sync`, 1 in `db_cli`, and 1 in `db_compact`.  The sole qualified compaction selector is valid because the crate declares `pub mod db_compact` from the compaction source ([crate](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/🦀️.rs:98)); the unqualified entries are unique test-function names.  This is registration/source evidence only: no native law or source script was run.

## 2026-09-05 active production and filesystem follow-up

### The new filesystem law

`wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens` is registered as the **23rd** `wal-committed-transactions-check --native` selector ([script](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:102)).  A source-only exact-name scan resolves that definition once ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2719)).  It is correctly scoped as a real-backend *reopen* law: both accepted neutral rows are seeded via `write_committed_fixture`, then run through three non-overlapping `FsStorage` instances; phase 2 closes its `ArtifactWal` before `FsStorage::close`, and phase 3 proves byte identity, active lifecycle, one abort, and a new `tx_id == 8` submission.  No retained pages, records, or a prior backend survive the instance boundary.

It must not be described as a power-loss proof.  `WalSync(Fsync)` calls `sync_all()` on the segment file ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6918)), but `WalCreate` and `WalSeal` do not fsync the containing directory ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6884)).  Thus it correctly proves an explicit close/reopen on the same filesystem, not crash persistence of the seed segment's directory entry or a successor's `.sealed` marker.  That limitation is not a defect in this abort law: recovery appends to an already-existing active segment.  Any general `FsStorage` claim that `Fsync` makes **creation/rotation** crash-durable needs a separate parent-directory-durability contract and law.

The branch ownership is otherwise sound.  The fault law covers clean and torn-tail routes through abort append, torn append, abort sync, and tail-sync failure ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2764)); the cancellation wrapper proves that the admitted final-repair boundary ignores cancellation both after `truncate_tail` and after abort append ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2847)); and the exact-capacity branch now forces successor rotation after the 496 KiB repair ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2890)).  I found no duplicate-abort, cancellation, capacity, or file-handle error in those reviewed paths.

### Blocking correctness issue: the documented writer is not an authority

`ArtifactWal::open` says that recovery occurs under the caller's “exclusive write authority”, but its actual capability is only `&impl WalStorage` ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2317)).  `ArtifactEngine::create_retained`/`open_retained` directly call `ArtifactWal::{create,open}` with `storage.wal()` ([artifact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1198), [artifact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1242)); `db_cluster::replicate_document` directly opens and submits to the follower too ([cluster](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:217)).  Neither owns a document writer guard.

This is a reachable corruption sequence, not merely a missing race test:

1. Openers A and B validate the same active physical `Begin(7)` prefix and both construct a resumed writer at the old `end`.
2. A appends and fsyncs `Abort(7)` plus the SPR physical commit.
3. B appends its stale complete abort suffix **before** `SegmentWriter::commit_and_flush` compares returned length with its old expected end ([WAL](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2200)).  It is then poisoned because the length differs, but the bytes already exist.
4. A subsequent opener accepts A's abort and then sees B's terminal record outside any transaction.  `WalTransactionGate` rejects that orphan `TxAbort(7)` as corrupt.  The WAL has become permanently unrecoverable.

`FsStorage` makes the race especially direct: every `WalAppend` opens an append-mode file for its current retained-page fragment, writes it, and closes it ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6894)).  There is no file lock.  Its existing lease mutex is only inside each `FsDbIoExecutor`; two `FsStorage::open` calls, let alone two processes, use different mutexes, and no WAL task carries a `LeaseStorage` fence ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7083)).  The new sequential filesystem law intentionally cannot expose this concurrent-writer failure.

### Smallest coherent writer-authority refactor

Do not try to cure this with an open-time `LeaseStorage::current`, a TTL alone, or an expected-length check after writing.  None prevents a stale owner from entering a later append.  Replace the raw mutable `WalStorage` surface with a storage-owned, non-`Clone` document writer permit:

```rust
pub struct WalWriterPermit { /* backend control + opaque slot/generation + exact document; fields private */ }

pub trait WalStorage: Send + Sync {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError>;
    async fn release_writer(&self, writer: WalWriterPermit) -> Result<(), DbError>;
    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError>;
    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError>;
    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError>;
    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
    // read/list/length/state remain document-only observations.
}
```

Every mutating `DbIoTask` (`WalCreate`, `WalAppend`, `WalSync`, `WalSeal`, `WalTruncate`, `WalDelete`) must carry the opaque writer slot/generation.  The target backend atomically checks that it owns the exact document *before* touching bytes or a lifecycle marker; `WalRef` must forward that permit and reject a permit from another backend.  This is the narrow common choke point: all current raw mutators are already confined to these six task variants ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2023)).

`ArtifactWal::{create,open,open_with_control}` must acquire before the initial list/read, retain the permit privately, and release it only from a new cancellation-immune async terminal close.  `submit`, `force_flush`, `rotate`, `SegmentWriter::{begin,initialize_existing_empty,commit_and_flush}`, and the active-abort repair consume that retained permit rather than accepting an unfenced mutable storage operation.  On *any* open failure after acquire (including a cancellation before the final repair grant), drain pages/list/writer then release; on a writer I/O failure, poison the segment but retain/release the permit during terminal close.  The permit's lifetime therefore spans recovery, all normal submits, and rotation—not just the synthetic abort.

`ArtifactEngine` owns the resulting `ArtifactWal` permit for its document lifetime; `db_cluster` obtains one for its whole tail-apply loop; CLI repair holds one through open/inspection/close; and compaction's `delete_segment` must obtain the **same** writer resource instead of its independent `compact:` lease ([compact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:178)).  Tests and fixture writers acquire/release a short permit around fixture mutation.  There is deliberately no compatibility raw-mutation path.

### Backend-owned guards

| Implementor | Correct writer guard | Mutation check |
| --- | --- | --- |
| `MemoryStorage` | Executor-owned `(document, slot, generation)` table under its existing state mutex. | Every mutable task checks the live slot and document; second owner gets `Conflict`; `Drop` of the storage releases all slots. |
| `FsStorage` | Retain an open `<root>/wal/<safe-document>/.writer.lock` `File` in the executor's permit slot, acquired with stable `std::fs::File::try_lock()`. | Open the sidecar read+write+create, convert `TryLockError::WouldBlock` to `Conflict`, and keep the file until `release_writer`/backend retirement.  Rust has supplied this cross-process lock API since 1.89; it releases when the owning process/file handle dies.  The existing process-local `lease_lock` is insufficient. |
| `SqliteStorage` | Disk SQLite: the same stable sidecar-file permit adjacent to the database, retained in `SqliteDbIoExecutor`; in-memory: its executor-local slot table. | Every `DbIoTask` validates the slot before its SQL transaction.  SQLite's existing `lease` table cannot be only an open-time TTL check. |
| `PostgresStorage` | Retain a dedicated pool connection for the permit and take a session-level PostgreSQL advisory lock derived from the exact document (use two deterministic advisory keys if avoiding a single-hash collision). | Route every mutable task bearing the slot through that retained connection.  The server releases the session lock on connection/process loss; every raw mutation then fails rather than allowing the stale `ArtifactWal` to append.  The current per-call `SELECT ... FOR UPDATE`/commit is not lifetime exclusion. |
| `Neo4jStorage` | There is no current session-owned lock or retained writer state in this backend. | Do **not** claim this backend supports concurrent WAL writers until it obtains a real server/session writer primitive with auto-release on connection loss and routes every mutable Cypher write through it.  The present per-call Neo4j transactions at `append`/`truncate_tail` ([neo4j](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:415)) cannot implement lifetime exclusion; a TTL record or post-write length check would repeat the same flaw. |
| `FaultStorage` and test-local `AbortCancellationStorage` | Forward the opaque permit to `inner`; scripts operate after permit validation. | Preserve fault ordering but add no bypass. |

The dispatch surface that must change is `WalRef` ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4894)), the native implementations in storage core, SQLite, Postgres, Neo4j, testkit, and the cancellation wrapper.  This is a single schema/trait break, not an adapter layer.

### First native laws for the authority

1. **Active-abort contender.**  Create two writer-acquisition attempts for one active incomplete fixture.  A holds the permit; B gets `Conflict` before it can list/repair.  A recovers once; after A's terminal release B reopens cleanly and sees exactly one `Abort(7)`.
2. **Stale permit cannot mutate.**  Retire/release A, acquire B, then attempt A's `append`, `truncate_tail`, `seal`, and `delete_segment`; every one returns `Fenced`/`Closed` with byte and marker identity.  This is the exploit's decisive negative case.
3. **Real Fs cross-instance contender.**  Two separately opened `FsStorage`s at one root contend for the same document; the second fails `Conflict`, then succeeds after the first WAL's explicit async terminal close.  Assert the `.writer.lock` is not a WAL segment and the subsequent reopen has one abort.  Use the existing three-scope filesystem fixture as the seed, not memory.
4. **Failure/cancellation release.**  Inject abort append and tail-sync errors, and cancel before the final grant.  In each case all retained owners and the permit are released; a fresh holder can recover exactly once.  Cancellation after the admitted final grant still completes exactly one abort before release.

No build was run and no product source was edited for this audit.
