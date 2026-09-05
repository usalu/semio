# History Replay Committed-Gate Migration Packet

Audit date: 2026-09-05. Read-only source review; no build or runtime execution was performed.

## Result

The ordinary reopen, sync, and current opener paths now use `WalCommittedCursor`; history replay is the remaining retained custom parser that can turn a `Command` or `Frontier` into an observable history entry before the enclosing `TxCommit` is seen. It must not store `WalCommittedTransaction<'_>` in `HistoryReplayFuture`: that transaction mutably borrows `WalCommittedCursor`, while both would have to live in fields of the same future. Such a future is self-referential and cannot be made sound by `Pin`; using `unsafe`, leaking the cursor, or holding a live borrow across `poll` is not an acceptable migration.

The smallest coherent route is to extract the **logical transaction grammar** behind the current `WalTransactionGate` into one reusable, owned fixed-span gate. History keeps its current fully owned source-page model and puts only `Copy` body spans in that shared gate. It must separately complete the same header/physical-commit/hash-chain verification before it feeds the gate. A generic gate alone is not an authenticity verifier.

## Current gap and exact locations

| Boundary | Current source | Why it is not committed replay |
| --- | --- | --- |
| Normal raw replay | [`db/📝️wal/🦀️.rs:1710-1869`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1710>) | `WalCommittedCursor` holds an exclusive transaction borrow until each retained record is closed and `finish` releases the gate. `ArtifactEngine::open_retained` uses it at [`db/🗿️artifact/🦀️.rs:1249-1294`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1249>); `db_sync` does too at [`db/🔄️sync/🦀️.rs:128-179`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:128>). |
| History raw frame parser | [`db/🗿️artifact/🦀️.rs:2782-2922`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2782>) | It classifies `TxBegin`, `Command`, and `Frontier`, but maps `TxCommit`, `TxAbort`, header and every other WAL type to `Other` (`:2912-2917`). It checks CRC/back-length and only rejects compression; it does not require exact critical flags, parse terminal payloads, or verify the WAL segment/document/index/chain contract. |
| History effect path | [`db/🗿️artifact/🦀️.rs:3086-3100,3369-3459`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3086>) | A `Command` enters `Envelope`/`CopyMutation` immediately; a `Frontier` enters `Publish` immediately. `ClearPending` is driven by the next `TxBegin`, not a matching terminal. An incomplete active transaction can therefore contribute output before an eventual abort/corrupt tail is discovered. |
| Future ownership | [`db/🗿️artifact/🦀️.rs:3107-3214,3509-3520`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3107>) | `HistoryReplayFuture` retains an `Arc<DbBackend>`, Vec-backed `HistoryPageSet`, a phase, and bounded reservation owners across `poll`; `Drop` asserts that all of them have been retired. It cannot also retain a cursor borrowing its own `WalRef`/pages. |

The direct history parser also accepts non-critical flags whenever they are not compressed (`:2880-2888`), whereas the WAL reader/chain require `FRAME_FLAG_CRITICAL` exactly ([`db/📝️wal/🦀️.rs:1276-1307,1471-1539`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1276>)). It cannot be the proof source for the committed gate.

## Shared, owned gate seam

Extract the current private `WalTransactionGate` ([`db/📝️wal/🦀️.rs:1314-1398`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1314>)) into a crate-visible generic core; do not copy its state machine into `db_artifact`.

```rust
pub(crate) enum WalLogicalFrame<T: Copy> {
    SegmentHeader,
    Begin { tx_id: u64 },
    Body(T),
    Commit { tx_id: u64, record_count: u32 },
    Abort { tx_id: u64 },
}

pub(crate) struct WalCommittedFrameBatch<T: Copy> {
    pub(crate) tx_id: u64,
    frames: [Option<T>; 64],
    len: u8,
}

pub(crate) struct WalLogicalTransactionGate<T: Copy> { /* current header/id/fixed-span state */ }
```

`push` accepts a classified frame and returns either no batch or an **owned** `WalCommittedFrameBatch<T>` after the exact matching commit/count. `Abort` clears its fixed spans and returns no batch. `finish_segment(false)` rejects an in-flight transaction and an unretired batch; `advance_segment` preserves the compacted-first/high-water id state. The current `transaction_seen`/`next_tx_id` semantics must remain: any first retained id is allowed, every subsequent id is exactly contiguous ([`wal:1344-1358`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1344>)). The result is bounded at the existing `WalRecordBatch` maximum of 64; it never owns decoded payloads.

Make one small shared exact-control decoder alongside it. The DB-page path reads at most 8 bytes for `Begin`/`Abort` or 12 for `Commit`, verifies exact EOF, then emits `WalLogicalFrame<WalRecordFrame>`. History copies the same at-most-12 byte scalar from `HistoryPageSet` into a stack array and calls that exact helper, emitting `WalLogicalFrame<HistoryWalBodySpan>`. `WAL_SEGMENT_HEADER` is a control token only after the segment verifier has authenticated its full payload; every other `WAL_COMMAND..=WAL_MIGRATION` is a body. This removes the present second terminal grammar and ensures the canonical-varint/checksum rules stay in `db_wal`.

```rust
#[derive(Clone, Copy)]
struct HistoryWalBodySpan {
    kind: u8,
    payload_start: u64,
    payload_len: u64,
}
```

That span is a range into the history future's already owned page set; it neither borrows a cursor nor allocates/decodes a WAL body. Its constructor requires the final, exact-critical, uncompressed frame boundary.

## History phase replacement

Add the owned `WalLogicalTransactionGate<HistoryWalBodySpan>` to `HistoryReplayFuture`, plus an owned `Option<WalCommittedFrameBatch<HistoryWalBodySpan>>` in a new `CommittedBodies` phase. Do **not** add `WalCommittedCursor` or `WalCommittedTransaction` fields.

1. After the current page-read phase has retained the whole bounded segment, run the shared verified-segment scanner first. It must use the same SPR committed-prefix check and WAL chain/header proof as `scan_retained_pages` + `WalSegmentChain` + `validate_wal_prefix` ([`db/📝️wal/🦀️.rs:1872-1931`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1872>)); an extracted byte-source adapter is required because history has `HistoryPageSet`, while those current private helpers take `DbIoPages`. The verifier must establish exact document, physical segment index, predecessor chain tip, critical flags, CRC/back-length, physical commit sequence and trusted end **before** it passes a `SegmentHeader` to the gate. A naked shared gate is insufficient here.
2. `HistoryFrameCursor` becomes a strict frame-span cursor over that verified end. It emits `SegmentHeader`, exact terminal controls, physical-commit boundaries, or `HistoryWalBodySpan`; it may no longer map terminal records to `Other`. Incomplete active prefix remains rejected by `finish_segment(false)`—durable recovery/abort stays in the opener.
3. Each decoded frame only advances the shared gate. A body span is never sent to `HistoryEnvelopeCursor`/`HistoryFrontierCursor` before a `Commit` produces an owned batch. `Abort` drops the spans and any per-transaction staging; it makes no history-side effect.
4. `CommittedBodies` processes at most one span per poll from the owned batch. It may use the existing `Envelope`, `CopyMutation`, `Frontier`, and `Publish` subcursors, but each must carry the batch index and return to it, not straight back to unrestricted `Frame`. Maintain `transaction_start` (operation-id length and result-byte length) before the first span; publish only after the final accepted span. On a malformed body/cancel/error, restore or retire all transaction staging before the existing `FaultRetire` path. Do not expose a partially built entry.
5. After the batch's last body closes, move/drop the batch, advance the frame cursor, and only then continue scanning. At segment end require the gate to be drained, then retain pages using the existing one-page-at-a-time `Retire` path.

The existing `HistoryReplayReservation` is already the correct bounded owner for result pages, operation ranges and entries. What changes is visibility timing: it needs a transaction-local pending range rather than using `ClearPending` on a following begin. A transaction containing `Command`, `Frontier`, then `Abort` must leave `result_len`, `operation_ids`, and `entries` exactly as before the begin. A committed batch makes its one entry only after all body parsing succeeds.

## Required extractor boundary

`db_wal` and `db_artifact` are sibling modules in the same crate, so this is not a Cargo dependency cycle. Keep the verifier's source abstraction and transaction grammar in `db_wal` nevertheless: `db_wal` must not name `HistoryPageSet`; `db_artifact` supplies that implementation. The narrow internal interface needs only bounded synchronous byte access:

```rust
pub(crate) trait WalVerifiedByteSource {
    fn len(&self) -> u64;
    fn byte(&self, offset: u64) -> Result<u8, DbError>;
    fn fragment(&self, offset: u64, maximum: u64) -> Result<&[u8], DbError>;
}
```

The generic scanner may retain offsets/digests but no caller-owned bytes. Implement it first for `DbIoPages` (replacing direct `WalPageReader` use in the existing verifier) and for `HistoryPageSet`; preserve the retained pages' owner in the caller. This is deliberately not a new public arbitrary-byte replay API: `WalCommittedCursor` remains the public record owner for normal callers, while the byte-source scanner is `pub(crate)` solely for the history page owner.

## First executable laws

1. **`history_replay_aborted_command_and_frontier_publish_nothing`** — a valid segment has header, begin, command, frontier, abort and a physical commit. History returns zero entries, zero operation ids and zero result bytes; the gate returns no batch.
2. **`history_replay_command_frontier_commit_publishes_once_after_terminal`** — instrument the phase boundary: before matching `TxCommit`, no `HistoryEnvelopeCursor`, `HistoryFrontierCursor`, result copy or entry allocation occurs. After exact commit, one valid entry/range equals the existing committed fixture projection.
3. **`history_replay_rejects_terminal_mismatch_and_later_id_gap_without_visible_prefix`** — id mismatch, bad count, nested begin, body outside a transaction, first retained 42 then 44, and a non-critical terminal all fail before any history entry. A first 42 then 43 succeeds.
4. **`history_replay_incomplete_active_prefix_is_not_an_implicit_abort`** — header/begin/body plus a physical commit causes corrupt/recovery-required outcome, not a partial view and not a synthetic abort. Close retires each source/result owner under cancellation.
5. **`history_replay_verified_source_rejects_foreign_or_rechained_segment`** — wrong document/index/predecessor hash, bad physical-commit hash/count/sequence, and CRC-valid noncritical frame each fail before the logical gate observes a header. This law closes the distinction between a parser and authenticated replay.

These cases belong in the existing schema-first committed-transaction fixture plus a history projection section/fixture. The native history law must drive the actual actor/future through repeated polls and its existing close protocol; a direct gate unit test is not proof that the retained future leaves no partial result owner.

## Migration order

1. Extract generic grammar/batch and exact control decode from `db_wal`; port `WalCommittedCursor` without changing its external borrow contract. Run its neutral/native suite.
2. Extract the crate-visible verified byte-source scanner from the existing SPR/chain helpers; port current `DbIoPages` callers unchanged and prove output parity.
3. Replace only `HistoryFrameCursor` plus `HistoryReplayPhase` as above. Keep its source-page reservation and `FaultRetire` owner retirement unchanged.
4. Run the five laws, then retain the existing cancellation/panic/partial-owner history laws. Do not claim history replay atomicity merely because normal reopen/sync already use the committed cursor.

No product source was edited and no checks were run for this packet.

## 2026-09-05 Current extraction update — owned spans, no self-borrow

This update is against the subsequently landed `WalCommittedCursor` boundary, not the older raw replay design, and supersedes the earlier type sketch wherever they differ. No product source was edited and no build was run for this review.

### Exact current facts which change the extraction

`WalSegmentChain` first authenticates the entire immutable segment (format header, document/index/predecessor header, exact critical flags, CRC/back-length, physical commit count/length/offset/chain) at [`db/📝️wal/🦀️.rs:1436`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1436>). Only after that does [`wal_next_verified_page_frame`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1281>) rescan a bounded span. It returns a source-independent `WalRecordFrame { kind, payload_start, payload_end, frame_end }`, advances the caller offset only for a physical `REC_COMMIT`, and otherwise leaves advance to its caller ([`:1284-1319`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1284>)). `WalCommittedCursor` therefore validates before it calls the logical gate and advances to `frame_end` only after the gate has admitted the frame ([`:1754-1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1754>).

The existing `WalTransactionGate` remains private and couples three separate concerns: exact scalar/control decoding from `DbIoPages`, the logical state machine, and a `ready` transaction held inside the gate ([`:1321-1395`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1321>)). History cannot retain the current borrowed `WalCommittedTransaction`: its `HistoryReplayFuture` already owns `HistoryPageSet` and its phase across polls ([`db/🗿️artifact/🦀️.rs:3102-3139`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3102>), while `WalCommittedTransaction` mutably borrows its cursor until `finish` ([`db/📝️wal/🦀️.rs:1730-1734,1815-1872`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1730>)). Storing either inside the other would be self-referential.

The good news is that `WalRecordFrame` already is the required owned, `Copy`, source-neutral body span. Its three offsets and kind do not borrow `DbIoPages` ([`db/📝️wal/🦀️.rs:1164-1170`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1164>)). `HistoryPageSet` has the same necessary immutable, page-crossing operations: `byte`, `page_slice`, and stack-array copying ([`db/🗿️artifact/🦀️.rs:2702-2786`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2702>)). It needs no `Vec` copy and no decoded WAL record owner.

### Minimal shared code seam

Keep the current public `WalCommittedCursor`/borrowed-record API intact. Extract only these crate-visible types in `db_wal`, close to the current gate:

```rust
pub(crate) const WAL_TRANSACTION_BODY_SPANS: usize = 64;

#[derive(Clone, Copy, Debug)]
pub(crate) struct WalPayloadSpan {
    pub(crate) kind: u8,
    pub(crate) payload_start: usize,
    pub(crate) payload_end: usize,
    pub(crate) frame_end: usize,
}

pub(crate) trait WalImmutableByteSource {
    fn byte_len(&self) -> usize;
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError>;
}

pub(crate) struct WalCommittedSpanBatch<T: Copy> {
    tx_id: u64,
    spans: [Option<T>; WAL_TRANSACTION_BODY_SPANS],
    len: u8,
}

pub(crate) struct WalLogicalTransactionGate<T: Copy> {
    transaction_id: Option<u64>,
    spans: [Option<T>; WAL_TRANSACTION_BODY_SPANS],
    len: u8,
    next_tx_id: u64,
    header_seen: bool,
    transaction_seen: bool,
}
```

`WalPayloadSpan` is the promoted/renamed current `WalRecordFrame`, so both DB replay and History use an identical *owned numeric range*. `WalImmutableByteSource::fragment_at` must return a non-empty sub-slice beginning exactly at `offset`, clipped to the absolute exclusive `limit`; `offset == limit` is not a fragment request. `DbIoPages` implements it by walking `fragments()`. `HistoryPageSet` implements it with `page_slice(offset as u64, (limit - offset) as u64)` after checked conversions. A shared `WalByteReader<'source, S>` then owns only `&S`, `position`, and `limit`; its `array::<8/12>()` preserves the present exact-EOF check. This is deliberately crate-visible, not a new public arbitrary-byte WAL API.

The gate must own the existing logical grammar, including scalar shape, rather than making History identify `Begin`/`Commit`/`Abort` itself:

```rust
impl<T: Copy + WalPayloadSpanLike> WalLogicalTransactionGate<T> {
    pub(crate) fn push<S: WalImmutableByteSource>(
        &mut self,
        source: &S,
        frame: T,
    ) -> Result<Option<WalCommittedSpanBatch<T>>, DbError>;
}
```

`WalPayloadSpanLike` needs only `kind`, `payload_start`, and `payload_end`; it is implemented by `WalPayloadSpan`. Alternatively, omit the trait and make `WalPayloadSpan` the one concrete generic payload, because History uses the same offsets. In either spelling, `push` uses the one shared scalar reader and implements these exact transitions:

| Input after full segment admission | Gate state/result |
| --- | --- |
| first `WAL_SEGMENT_HEADER` | require no current transaction; set `header_seen`; never retain it |
| `WAL_TX_BEGIN` | require exact 8 bytes, no active transaction, first id arbitrary but later id exactly `next_tx_id`; open span collection |
| `WAL_COMMAND..=WAL_MIGRATION` | require active transaction; append only its `Copy` span, fail at 64 |
| `WAL_TX_COMMIT` | require exact 12 bytes, matching id and matching span count; `mem::replace` the fixed array into an **owned** batch and clear active state |
| `WAL_TX_ABORT` | require exact 8 bytes and matching id; clear only the active spans; yield no batch |
| physical `REC_COMMIT` | never enters logical gate; frame walker advances it |
| segment end | reject absent header; reject a live transaction when sealed; return its id only for the active-highest recovery decision; caller must also have no unprocessed owned batch |

Do not retain `ready` in this generic gate. `push` moves a batch out on logical commit. `WalCommittedCursor` gains `batch: Option<WalCommittedSpanBatch<WalPayloadSpan>>`; it places the returned batch there, and its existing borrowed `WalCommittedTransaction` indexes that batch/keeps its source record owner. `finish` takes the batch only after all records are closed. This preserves the current public borrow contract while making the grammar single-sourced. `ArtifactWal::open`/`validate_wal_prefix` immediately drops a returned batch after proving it exists; no decoded body or allocation is introduced there.

### Verification boundary and History state machine

Promoting spans and the gate alone is not enough: History's `HistoryFrameCursor` currently accepts every non-compressed flag and maps every terminal to `Other` ([`db/🗿️artifact/🦀️.rs:2807-2930`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2807>)). It must not feed a `WalPayloadSpan` to the gate until the exact same segment has passed `WalSegmentChain`. The correct order for one retained history segment is:

```
pages fully admitted -> WalSegmentChain proves complete immutable segment
 -> shared verified-frame walker yields owned spans / physical commits
 -> shared owned gate yields a committed batch only at exact TxCommit
 -> History processes one span at a time from that owned batch
 -> drop batch -> retire source pages
```

The verifier must remain source-owning rather than making an `async` future borrow `HistoryReplayFuture.pages`: `WalSegmentChain::new/step` currently await protocol header/commit helpers ([`db/📝️wal/🦀️.rs:1436-1535`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1436>)). A generic source adapter by itself does **not** make that callable from `HistoryReplayFuture::poll`. The smallest safe extraction is therefore a poll-step verifier whose owned state contains offsets, hashers, CRC and copied fixed commit/header scalars, while each call receives `&S` only for that call:

```rust
pub(crate) struct WalSegmentChain { /* current state; no S or &S */ }
pub(crate) enum WalSegmentChainStep { Yield, Verified { tip: [u8; 32] } }

pub(crate) fn step<S: WalImmutableByteSource>(
    &mut self,
    source: &S,
    trusted_len: usize,
    document: &ArtifactId,
    control: &mut WalCursorControl,
) -> Result<WalSegmentChainStep, DbError>;
```

This means moving the pure protocol header/commit field validation into a synchronous shared, schema-owned helper first (not duplicating it in `db_artifact`); current `read_header` and `parse_commit_payload` are syntactically async despite their fixed-byte computation. A boxed verifier future that captures `&HistoryPageSet` is specifically not an alternative: it would borrow another field of `HistoryReplayFuture` across `poll`. Moving the pages into such a future also breaks the current explicit one-page `FaultRetire` accounting ([`db/🗿️artifact/🦀️.rs:3164-3204,3470-3485`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3164>)).

With that source-free verifier state, replace `HistoryReplayPhase::Frame` with these owned phases/fields:

```rust
// on HistoryReplayFuture
chain: Option<db_wal::WalSegmentChain>,
gate: db_wal::WalLogicalTransactionGate<db_wal::WalPayloadSpan>,
batch: Option<db_wal::WalCommittedSpanBatch<db_wal::WalPayloadSpan>>,

Verify { index, trusted_len },
GateFrame { index, offset, trusted_len },
CommittedBody { index, offset_after_batch, batch_index, transaction_start },
```

`Verify` repeatedly passes `&this.pages` to the source-free chain state; only after `Verified` does it enter `GateFrame`. `GateFrame` calls the shared walker and gate, retaining `offset` in the phase rather than a borrow. On a returned batch it captures `transaction_start = (result_len, operation_ids.len(), entries.len())`, puts the batch in the future field, and enters `CommittedBody`. `CommittedBody` dispatches its current span's `kind`: only `WAL_COMMAND` may create `HistoryEnvelopeCursor`; only `WAL_FRONTIER` may create `HistoryFrontierCursor`; other valid transaction bodies are skipped. Every subcursor returns to this batch phase, not to unrestricted frame scanning. After the last span, require/perform one publication using the already accumulated Frontier and then drop/take the batch before resuming `GateFrame` at `offset_after_batch`.

`ClearPending` must be removed. It is currently keyed to a later begin, not the matching terminal ([`db/🗿️artifact/🦀️.rs:3377-3448`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3377>)). On abort the gate supplies no batch, so neither the envelope nor frontier parser runs. On a malformed committed body/cancel, restore the captured transaction counts/length before the existing `FaultRetire` sequence; no entry can have been published because publication is batch-terminal. `Retire` remains the sole source-page retirement owner.

### First three real-history native laws

These are `db_artifact` native laws that drive the actual `ArtifactEngine::history_replay` future through repeated polls and its existing close protocol—not direct gate unit tests. Reuse the real `MemoryStorage`/`SegmentWriter` committed fixture infrastructure at [`db/📝️wal/🦀️.rs:2516-2545`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2516>) and attach to the current History replay tests near [`db/🗿️artifact/🦀️.rs:4991`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4991>).

1. `artifact_history_replay_committed_command_frontier_is_visible_once` — append one real encoded mutation `WAL_COMMAND` and its real `WAL_FRONTIER` in a matching begin/commit plus physical commit. The completed `HistoryReplayFuture` returns exactly one entry and operation id; repeated poll/close leaves `terminal_is_empty()`.
2. `artifact_history_replay_valid_aborted_command_frontier_has_no_visible_projection` — write the same valid bodies under begin/abort and seal with a valid physical commit. Assert no envelope/frontier/copy phase is entered, zero result length/ids/entries, and all retained source/result pages retire. This is the direct regression for present early visibility.
3. `artifact_history_replay_rejects_bad_terminal_before_history_projection` — use a fully CRC-valid/physical-chain-valid segment whose logical commit has wrong id or count (and a separate non-critical terminal frame). The future faults before a history body phase or entry allocation; close completes its existing one-owner-at-a-time protocol. The neutral committed-transaction fixture already contains terminal/id/count categories at [`db/📝️wal/🧪️fixtures/🧾️committed-transactions/🔣️.json`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️fixtures/🧾️committed-transactions/🔣️.json); do not make a parallel logical grammar fixture.

This remains an implementation packet only. No test, build, or source edit was performed.

## 2026-09-05 Document-Binding and Ordering Fixture Packet

This is a read-only current-source packet for the next History document-binding laws.  No build
or runtime execution was performed.

### Use the real writer; do not synthesize SPR frames

The smallest fixture seam is the existing `db_artifact` test module's retained encoder:
[`retained_wal_envelope`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4304>) produces a real
`protocol::encode_envelope` payload owned by `WalBytes`.  Create the target engine with
[`ArtifactEngine::create_retained`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1183>), assemble a
`WalRecordBatch`, then call that same engine's private `wal.submit` against its private
`storage.wal()` facet.  Tests nested in this module can legally use those private fields.  The
submitter supplies the segment header, exact critical SPR frames, CRC/back-length, physical commit,
cross-segment chain and matching `TxBegin`/`TxCommit` count; `DurabilityClass::Fsync` forces the
physical commit ([`ArtifactWal::submit`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2415>)).
It is therefore a valid-physical-chain, logically committed fixture with only its intended semantic
field varied.

The test must retire every retained owner after `submit`:

```rust
let mut records = db_wal::WalRecordBatch::new();
records.push(db_wal::WalRecord::Command(retained_wal_envelope(&command).await)).unwrap();
// add the selected Frontier(s) here
let wal_facet = engine.storage.wal().await;
engine.wal.submit(&wal_facet, &records, DurabilityClass::Fsync, 1).await.unwrap();
drop(wal_facet);
while records.close_step().unwrap() { semio_framework_async::yield_once().await; }
```

Use `Frontier { document: ArtifactId(foreign_or_owned.0.clone()), head_seq: 1, commit_seq: 1,
chain_hash: [0; 32], epoch: 0 }`.  `db_durability::Frontier.document` is the core `ArtifactId`,
while the inner command uses `protocol::ArtifactId` ([`db/💾️durability/🦀️.rs:57`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/💾️durability/🦀️.rs:57>)).
Set `command.document_id` to a distinct `protocol::ArtifactId` *after* using the normal test
`envelope` helper; do not call `engine.submit`, which correctly rejects that input at
[`db/🗿️artifact/🦀️.rs:1374`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1374>).

This is preferable to `db_wal::tests::committed_fixture_storage`: that neutral helper's
`"command"` is one ordinal byte, not a decodable mutation envelope
([`db/📝️wal/🦀️.rs:2610`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2610>)).  It remains useful for generic
transaction-gate cases, but cannot prove History's inner document checks.

### Four native History laws

Attach these beside the current real projection law at
[`db/🗿️artifact/🦀️.rs:4125`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4125>).  Each drives
`engine.history_replay` to its terminal error, then asserts `terminal_is_empty()`; the returned
view must never be exposed on a rejected transaction.

| Law | Batch body records (all wrapped by real `ArtifactWal::submit`) | Required result |
| --- | --- | --- |
| `artifact_history_replay_rejects_foreign_inner_command_document` | `Command(encoded foreign-document envelope)`, `Frontier(owned document)` | Corrupt before the mutation ID is copied into the result owner. |
| `artifact_history_replay_rejects_foreign_frontier_document` | `Command(encoded owned-document envelope)`, `Frontier(foreign document)` | Corrupt; no returned entry. Temporary staged operation bytes must retire through the existing fault path. |
| `artifact_history_replay_rejects_duplicate_frontier_in_committed_batch` | `Command(owned)`, `Frontier(owned)`, `Frontier(owned)` | Corrupt rather than replacing `pending_frontier` with the second marker. |
| `artifact_history_replay_rejects_frontier_before_command_in_committed_batch` | `Frontier(owned)`, `Command(owned)` | Corrupt before publication; a history entry may not be anchored before its command. |

The direct companion opener laws are warranted too.  `ArtifactEngine::open_retained` decodes each
committed command and calls `apply_one` without again comparing its `envelope.document_id` to the
engine's `protocol_document`, and assigns any decoded frontier directly
([`db/🗿️artifact/🦀️.rs:1269-1286`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1269>)).  The same two
foreign-document fixtures should make open fail before cross-document state or frontier becomes
observable.  The authenticated segment header only proves the WAL is for the outer document; it
does not bind the payload's embedded document field.

### Exact current bug and bounded semantic fix

Both History inner cursors merely validate that the document text is UTF-8 and discard it:
`HistoryEnvelopeCursor::Document` at
[`db/🗿️artifact/🦀️.rs:2848`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2848>) and
`HistoryFrontierCursor::Document` at
[`db/🗿️artifact/🦀️.rs:2935`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2935>).  Pass the expected
core `ArtifactId` into both constructors.  In the envelope cursor, compare the decoded
`protocol::ArtifactId.0` bytes/text to that core ID before advancing to Actor; in the frontier
cursor compare the decoded `ArtifactId` before Head.  A string equality is lossless at this local
bridge (`to_core_document_id` is only `ArtifactId(id.0.clone())` at
[`db/🗿️artifact/🦀️.rs:62`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:62>)); do not introduce a second
identity codec.

After the shared committed batch is admitted, make History's *artifact-projection* state machine
carry `seen_command` and `seen_frontier` for that batch:

* `WAL_COMMAND` after a frontier is corrupt.
* A frontier before any command, or a second frontier, is corrupt.
* A command-bearing batch must end with exactly one frontier; current missing-frontier rejection
  at [`db/🗿️artifact/🦀️.rs:3342`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3342>) stays.
* Non-`Command`/`Frontier` WAL body kinds remain ignored in all states.  They must not turn into
  an ordering failure merely by following a frontier.

Today the second frontier silently replaces the first at
[`db/🗿️artifact/🦀️.rs:3386`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3386>), and the following
`Publish` uses that replacement.  The above check belongs after generic `WalTransactionGate`
commit admission: the generic WAL grammar intentionally permits generic record ordering and must
not learn Artifact history semantics.

### Production ordering basis

The sole production `ArtifactEngine::submit` builder initializes one batch, appends
`Command` followed by `Outbox` for each accepted envelope
([`db/🗿️artifact/🦀️.rs:1385-1436`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1385>)), then appends exactly
one `Frontier` and immediately submits the batch
([`db/🗿️artifact/🦀️.rs:1484-1492`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1484>).  It never emits a
later `Command` or `Frontier`.  `snapshot_now` uses `SnapshotManager` storage directly and does
not append a WAL transaction ([`db/🗿️artifact/🦀️.rs:1606`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1606>).

`Compactor` likewise has no production `ArtifactWal::submit` call: its only submit helper is in
the test section at [`db/🗜️compact/🦀️.rs:2396`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:2396>); the production pass
publishes snapshot storage, not a WAL record.  Its duplicate-frontier sequence at
[`db/🗜️compact/🦀️.rs:2911-2915`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:2911>) is a raw test for segment
horizons, not an `ArtifactWal` transaction or a history source.  Thus neither requested
production path relies on `Command`/`Frontier` after its first frontier.

Outside that scope, `db_cluster::replicate_document` deliberately writes one `Command` per WAL
transaction and no frontier ([`db/🌐️cluster/🦀️.rs:225-244`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:225>)).  This confirms
the ordering rule must stay in `ArtifactHistory` rather than the generic WAL gate; it also means
History is not currently a universal WAL projection unless cluster tail writes are later made to
carry canonical Artifact frontiers.

## 2026-09-05 Current real-projection and registered-law review

This is a subsequent read-only review of the newly landed owned-source History path, its
`historyProjection` fixture section, and the registered `wal-committed-transactions-check` list.
No build or runtime execution was performed.

### What the new law now proves

[`artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4125)
uses the actual `ArtifactEngine::submit` path, then feeds the resulting durable WAL back through
`HistoryReplayFuture`.  It correctly compares the one returned entry's operation IDs, head and
commit sequence, chain hash, and epoch to the submit receipt before explicitly retiring the
returned view.  This is material evidence that the historical projection sees a real two-command
committed group, not only abstract `WalRecordFrame` grammar.

The new implementation correctly owns `WalAuthenticatedSource<HistoryPageSet>` rather than a
self-borrowing committed cursor ([`artifact:2993-3039`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2993)); the source is authenticated before a frame can enter the gate
([`artifact:3287-3322`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3287)).

### Material remaining test gaps

1. The cancellation loop stops at `Verify` or immediately on entry to `CommittedBody`
   ([`artifact:4146-4169`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4146)).  At both cut points the law explicitly expects no entries, so it
   does **not** exercise retirement after a command ID has been copied to result pages, or after a
   history entry has been published.  Add two deterministic stops in the same law:

   - after `reservation.operation_ids.len() == 1` and `result_len > 0`, before the transaction
     reaches its frontier; and
   - after `reservation.entries.len() == 1` but before the next verified frame/segment retires.

   Cancel each, drive the future to `Err(DbError::Closed)`, and assert
   `terminal_is_empty()`.  This reaches the `CopyMutation` and post-`Publish` owner graphs which
   the current before-body checkpoints do not cover.

2. The schema's `historyProjection.commands` bounds the list to two but does not require distinct
   command IDs ([`committed schema:8-24`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️fixtures/🧾️committed-transactions/🧬️.schema.json:8)).
   A duplicate ID is not a valid real committed-batch witness and can make the asserted projection
   depend on engine dedupe behavior rather than the history grammar.  Standard JSON Schema cannot
   compare two sibling property values, so add the exact two-ID inequality to the independent
   oracle in the registered script; do not require distinct paths, since same-path edits are
   legitimate.

3. The History cursors validate that the command/frontier document fields are UTF-8 but discard
   their values ([`HistoryEnvelopeCursor::Document`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2848) and
   [`HistoryFrontierCursor::Document`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2936)).  Segment-header admission binds the segment to the requested
   document, but it does not establish that every embedded command/frontier repeats that document.
   Add the expected `ArtifactId` to both cursor constructors and reject an unequal embedded value.
   A fully chain-valid segment whose header says document A but whose committed command/frontier
   says B must fault before copying an operation ID.  The current real-submit fixture cannot
   expose this because it only constructs coherent writer output.

4. `HistoryReplayPhase::Frontier` overwrites `pending_frontier` without refusing a second
   `WAL_FRONTIER` ([`artifact:3386-3395`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3386)); it also accepts a frontier before later commands in the same committed group.
   The writer emits commands/outbox records followed by exactly one frontier
   ([`artifact:1431-1436`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1431),
   [`:1484-1491`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1484)).  Require one final frontier per projected transaction; reject duplicate or
   nonterminal frontier ordering.  Add hostile real/encoded vectors for both orders so history
   cannot choose a last-seen frontier silently.

The `matches!(replay.phase, ...)` check in the cancellation fixture at
[`artifact:4151`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4151) should be written as `matches!(replay.phase.as_ref(), ...)`.
The non-binding pattern is expected to inspect only the discriminant, but the explicit borrow
removes an avoidable dependency on match-place behavior for a non-`Copy` phase that contains
boxed futures and is mutably polled immediately afterward.

### Registered exact-law audit

The 17 selectors in
[`packages/rust script:94-108`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:94)
each have exactly one `fn` declaration under the DB source tree.  This includes the only qualified
selector, `db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects`,
whose sole declaration is at
[`db compact:2954`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:2954).
The History selectors resolve once at
[`artifact:4096`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4096),
[`artifact:4125`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4125), and
[`artifact:5040`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:5040).
The runner itself rejects any selector that resolves zero or more than once in the executable
[`runExactCargoLaws:1982-1997`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1982).

No missing, duplicate, or incompatible source-level selector was found.  This does not claim
that the native executable has built or that these laws have passed.
