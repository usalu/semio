# Terra WAL Controlled-Open Ownership Frontier

## Decision

Replace the current one-shot `ArtifactWal::open` with one public resumable
`ArtifactWalOpenCursor<'storage, S>`, and give a successful `ArtifactWal`
an explicit terminal close protocol. Do not add a compatibility one-shot open.
A recovery can verify/copy a segment and repair a tail, but the current hidden,
fresh 30-second control cannot honestly make that work caller-cancelable.

```rust
pub struct ArtifactWalOpenCursor<'storage, S: WalStorage> { /* owners */ }
pub enum ArtifactWalOpenStep { Yield, Ready }

impl<'storage, S: WalStorage> ArtifactWalOpenCursor<'storage, S> {
    pub async fn begin(storage: &'storage S, document: ArtifactId,
        policy: GroupCommitPolicy, now_ms: u64, control: WalCursorControl)
        -> Result<Self, DbError>;
    pub fn replenish(&mut self, deadline: Instant, fuel: usize) -> Result<(), DbError>;
    pub async fn next_step(&mut self) -> Result<ArtifactWalOpenStep, DbError>;
    pub fn take_result(&mut self) -> Option<(ArtifactWal, WalRecoveryReport)>;
    pub fn close_step(&mut self) -> Result<bool, DbError>;
    pub fn terminal_is_empty(&self) -> bool;
}
impl ArtifactWal {
    pub fn close_step(&mut self) -> Result<bool, DbError>;
    pub fn terminal_is_empty(&self) -> bool;
}
```

`next_step` spends at most one control grant before a storage action, verified
or copied fragment, or small state change, then returns `Yield`. `close_step`
is progress-only and deliberately does not call `grant()`: cancellation may
stop useful work, never terminal owner retirement. The caller must
`force_flush` before closing a live WAL with pending records; close must not
silently forget a memory-only group commit.

This cursor is the smallest correct public boundary. Passing `&mut
WalCursorControl` into an otherwise one-shot async `open` loses the exact
`DbIoPages`, `DbIoU64List`, and partial `SharedBuf` owners on fuel
exhaustion/cancellation, leaving only deferred Drop cleanup.

## Current Facts

| Concern | Current source | Finding |
| --- | --- | --- |
| Caller control | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1659-1721` | Open creates its own cancel flag/deadline/fuel at 1696-1697. An authority cannot cancel, bound, or resume recovery. |
| Incomplete retirement | `…/📝️wal/🦀️.rs:1660-1677,1679-1720`; `…/🗄️storage/🦀️.rs:1604-1625,1891-1909` | Success takes only one close step. Sealed-tail, scanner, decode, append, or sync errors return with pages/list still owned. |
| Drop is emergency-only | `…/🗄️storage/🦀️.rs:1628-1651,3772-3913,4314-4332` | Dropped pages park for later global maintenance. That is not terminal ownership and can retain pages/slots until unrelated I/O occurs. |
| Successful WAL cost | `…/📝️wal/🦀️.rs:875-880,1636-1643`; `…/🗄️storage/🦀️.rs:71-74` | Every `SharedBuf` reserves 64 pages (1 MiB), from a 1,024-page process pool. `ArtifactWal` has no close API. |
| Replay close is cancel-blocked | `…/📝️wal/🦀️.rs:1487-1516` | `close_owner_step` calls `control.grant()` before release. A canceled cursor cannot clean up. Existing test resets its cancel flag before close at 2249-2255, masking this. |
| Storage action control | `…/🗄️storage/🦀️.rs:4579-4624` | `read`, `truncate_tail`, `sync`, `create_segment`, and `append` have no cancellation parameter. A cursor gates action start; it cannot promise interruption while backend I/O is in flight. |

`DbIoPages::close_step` retires a page, then shell/result handback
(`…/🗄️storage/🦀️.rs:1604-1625`). `DbIoU64List::close_step` decrements the
bounded list and releases backing/handback (`1841-1909`). Those exact owners
must remain inside the open cursor until terminal empty; normal cancellation
must not rely on Drop.

## Required Ordering

1. `begin` retains `list_segments`; every scan action uses caller control.
   Validate density and feed each segment through existing `WalSegmentChain`
   before accepting records or repair. Its structural verifier is
   `…/📝️wal/🦀️.rs:1237-1354`; `WalReplayCursor` already models
   fragment-bounded `Yield` at `1445-1473`.
2. Scanning and prefix copy use a grant per fragment. Copy the exact verified
   source prefix into admitted `SharedBuf`, never a `Vec`. Current
   `SharedBuf::copy_range` has page-sized yielding but no control check
   (`875-917`); add an open-only `copy_from_pages_prefix(..., control)`.
3. On a torn active tail, grant immediately before a mutation epoch. From
   `truncate_tail(trusted_end)` through `sync(Fsync)` and
   `SprWriter::resume_verified`, do not observe cancellation between durable
   state changes. Report cancellation before that epoch or after it completes,
   not after truncate but before the fence.
4. Before `Ready`, drain source pages and segment list completely. The result
   owns only the active WAL. `take_result` is one-shot.
5. On error/cancellation, enter `Closing`; each `close_step` drains one
   source page/list/writer opportunity without `grant()`. `next_step` may
   report its error while the cursor remains caller-owned, but no result may
   be taken and the caller must drive it to `terminal_is_empty`. Never repair
   sealed data after an interrupted scan.

Make the same narrow change to existing `WalReplayCursor::close_owner_step`:
remove its leading grant, retain grants on `next_step`, and prove cancellation
followed by close reaches terminal empty. This restores its existing lifecycle;
it is not a second API family.

## Active Result Ownership

`SharedBuf` is `Arc<Mutex<DbIoPageWriter>>`
(`…/📝️wal/🦀️.rs:863-880`). Both `SegmentWriter.buf` and
`SprWriter<SharedBuf>` own clones (`1530-1547`). Therefore
`ArtifactWal::close_step` must first relinquish the `SprWriter`, then take
the unique `SharedBuf` and drive `DbIoPageWriter::close_step`. Hold the
releasable writer/buffer as `Option` owners during closure; retiring the
buffer while the writer still owns its clone is an ownership error.

Current callers that must adopt this lifecycle:

| Caller | Current line | Required outcome |
| --- | --- | --- |
| Document authority | `…/🗿️artifact/🦀️.rs:1198-1295`; spawn `…/⚙️engine/🦀️.rs:7322` | Thread one caller control through WAL open and replay. On authority shutdown: force-flush then terminal-close its WAL. It currently creates unrelated uncancelable controls. |
| Cluster tail apply | `…/🌐️cluster/🦀️.rs:217-233` | Its submits are Fsync; terminal-close when replication finishes. |
| CLI repair/migrate | `…/⌨️cli/🦀️.rs:1024-1033,1227-1239` | Repair has no pending records and migration is Fsync; both drain before storage release. |
| Snapshot marker helper | `…/🔄️sync/🦀️.rs:2598-2602` | Fsync marker then terminal-close. |

## Transaction Replay Constraint

Open must preserve verified bytes and resume `next_tx_id` from every
transaction marker, but it cannot be claimed as transaction-atomic
materialization. `ArtifactEngine::open_retained` clears `batch_ids` on
`TxBegin` and applies every `Command` immediately
(`…/🗿️artifact/🦀️.rs:1251-1286`). `db_sync::replay_sync_state` accumulates
commands before merely counting `TxCommit` (`…/🔄️sync/🦀️.rs:131-151`).
Neither matches transaction id/record count nor discards `TxAbort`. Thus a
valid committed generic SPR segment containing `TxBegin, Command, TxAbort`
is materialized and synced as committed. The current writer only emits
`TxBegin ... TxCommit` (`…/📝️wal/🦀️.rs:1738-1746`), explaining the gap in
happy-path tests, not validating the public record vocabulary.

## First Executable Laws

1. **Cancellation owns no pages.** Begin a multi-page valid active segment with
   tiny fuel/cancel during prefix scan; leave cancellation set, drain
   `close_step`, assert terminal empty and original bytes unchanged, then
   prove a fresh full-control open succeeds.
2. **No cancel inside repair.** For a torn tail, cancel before repair and prove
   no truncate; cancel after the mutation epoch and prove exactly
   `trusted_end` bytes plus Fsync. Extend the current torn-tail fixture at
   `…/📝️wal/🦀️.rs:2071-2094` with observational truncate/sync storage.
3. **Repeated open/close is budget-neutral.** Repeat more than
   `DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES == 16` clean opens, force-flush
   if needed, terminal-close each without global maintenance, then admit and
   append again. Add a test-only ledger witness beside the existing private
   `…/🗄️storage/🦀️.rs:7874` if exact credits are asserted; a pass dependent
   on `db_io_maintenance_step` is not ownership proof.
4. **Canceled replay closes.** Cancel a `WalReplayCursor` with source pages
   open; without resetting its flag drain close and assert terminal empty. It
   fails currently at the grant on 1488.
5. **Abort is invisible.** Handcraft a valid, committed segment with
   `SegmentHeader, TxBegin(7), Command(valid envelope), TxAbort(7)`.
   Materialization and sync must expose no command; a later submission gets a
   tx id greater than 7.

## Scope

Read-only audit; no build or runtime launch was performed. The recommendation
does not invent a storage cancellation trait: the narrow correct boundary is a
caller-owned cursor around existing storage futures, control checks before
actions, a non-interruptible small repair epoch, and deterministic retirement
after them.
