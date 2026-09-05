# Terra WAL Committed-Cursor Retained Decode/Cleanup Packet

Read-only audit on 2026-09-05. No product source was changed and no build was run. This packet narrows the already-recorded committed-transaction frontier to the one-record retained decoder needed before `WalCommittedCursor` can be safe under interruption.

## Decision

Do not let `WalCommittedCursor` call the present `wal_decode_page_record` asynchronously. Factor frame validation from retained materialization, then add one private, resumable `WalRetainedRecordDecoder` that owns *at most one* `DbIoPageWriter` or `DbIoU64List`. It must retain that owner across `WalCursorControl::grant()` interruption, and it must be drained by the owning cursor without another grant.

The outer cursor may advance its logical frame cursor only after either:

1. a non-owning frame descriptor was accepted by the transaction grammar, or
2. this decoder has transferred a complete `WalRecord` to the caller.

This preserves the previous fixed-span design: a committed transaction owns at most 64 address-only `{ kind, payload_start, payload_end }` spans, and the borrowed transaction materializes one span at a time only after its matching logical `TxCommit`. It must not collect decoded `WalRecord`s in `WalRecordBatch`.

## Current executable seam and failure

| Exact current path | Finding |
| --- | --- |
| [`db_wal/🦀️.rs:1139-1189`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1139) | `wal_decode_page_record` owns byte-copy and `IndexCkpt` allocation as stack locals. A `grant()` error after either owner is created returns through `?`, so normal cursor control cannot resume or explicitly close that owner. |
| [`db_wal/🦀️.rs:1088-1102`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1088) | `WalPageReader::bytes` reserves destination pages, advances its local reader as it copies, and awaits `seal_retained`. A fuel/cancel error in the loop drops the writer. A seal failure is converted with `DbIoPageWriterRejected::into_error`, which discards a possible returned writer. |
| [`db_wal/🦀️.rs:1171-1180`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1171) | `IndexCkpt` allocates a `DbIoU64List` locally, then calls `grant` once per id. Interruption after an early id drops a non-terminal list. Its current physical limit is 4,096, matching the storage backing, even though the test-only decoder has a looser historical constant. |
| [`db_wal/🦀️.rs:1191-1227`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1191) | CRC/back-length validation is sound before materialization, but `*offset = frame_end` occurs at line 1219 **before** decoding the retained body at line 1222. Interrupted materialization therefore skips the record after `replenish`. |
| [`db_wal/🦀️.rs:1450-1530`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1450) | `WalReplayCursor` retains source pages and already distinguishes fuel/cancel/deadline from a terminal failure. It has no slot for a partial record decoder, so it cannot close or retry one. Its owner close is correctly no longer gated by `grant`. |
| [`db_storage/🦀️.rs:545-760`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:545) | `DbIoPageWriter::seal_retained_step` is the correct one-opportunity primitive. The consuming async `seal_retained` future should not be used by the cursor because a rejection can own a writer and a dropped future parks it. |
| [`db_storage/🦀️.rs:711-727, 764-788`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:711) | `DbIoPageWriter::close_step` returns one page/shell opportunity. A dropped non-terminal writer is parked, not synchronously retired. |
| [`db_storage/🦀️.rs:1841-1910, 1977-1987`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1841) | `DbIoU64List::close_step` removes one value then backing/handback. Its `Drop` also parks a non-terminal list, so decoder failure must leave it inside the cursor until explicit close. |
| [`db_storage/🦀️.rs:3772-3912`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3772) | Lost-owner retirement is a global best-effort maintenance queue. It is an emergency fallback, not evidence that a cancelled replay is budget-neutral. |

`WalCursorControl::grant` at [`db_wal/🦀️.rs:173-202`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:173) is deliberately irreversible: it checks cancellation/deadline, then spends one fuel unit. `replenish` replaces fuel/deadline. The committed cursor must use the present interruption classification (fuel exhaustion, exact cancelled/deadline `Unavailable`) rather than treating every `LimitExceeded` as resumable; a record-count or payload-shape limit is terminal corruption/limit failure.

## Small private retained decoder

First factor the CRC/back-length checked prefix of `wal_next_page_record` into a non-owning frame step. It returns only `WalRecordFrame { kind, payload_start, payload_end, frame_end }`; physical `REC_COMMIT` frames are skipped after their validated `frame_end`. For a WAL frame the owner keeps `offset == frame_start` while the frame is pending. A grammar collector may advance after it has copied this tiny descriptor; a raw materializer advances only after the decoder returns `Ready`.

The decoder needs no new storage API:

```rust
struct WalRetainedRecordDecoder {
    kind: u8,
    payload_start: usize,
    payload_end: usize,
    state: WalRetainedDecodeState,
}

enum WalRetainedDecodeState {
    Start,
    Bytes {
        record_kind: u8,
        copy_at: usize,
        copy_end: usize,
        writer: Option<DbIoPageWriter>,
    },
    IndexCkpt {
        next: usize,
        count: usize,
        value_at: usize,
        run_ids: Option<DbIoU64List>,
    },
    Ready,
}
```

The exact algorithm per `step(pages, operation, control)` is:

1. **Start/prevalidate, with no retained allocation.** Rebuild a bounded `WalPageReader` from the immutable source span. Decode scalar shapes directly. For opaque bytes, derive the exact `copy_at..copy_end`. For inline payload, parse tag and varint, then require `data_start + len == payload_end` before reserving pages. For `IndexCkpt`, require `count <= 4_096` and `values_start + count * 8 == payload_end` with checked arithmetic before creating the list. Reject every trailing byte before any retained allocation. This corrects the present inline-payload case, which can allocate the declared prefix and only then reject trailing bytes at [`db_wal/🦀️.rs:1155-1159,1184-1187`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1155).
2. **Byte admission/copy.** Call `grant` before `try_reserve_for_operation`; immediately store a successful writer in `Bytes`. Each later step calls `grant`, copies no more than the current source fragment, updates only `copy_at`, and yields. Recreate a reader at that absolute offset rather than retaining a self-borrowing reader. A zero-length opaque field still owns the writer shell exactly as the current `WalBytes` representation does.
3. **Byte sealing.** Call `grant`, then call `DbIoPageWriter::seal_retained_step` once. `None` yields; `Some(pages)` moves into `WalBytes` and then the resulting `WalRecord`. Do not await consuming `seal_retained`. If its one-step method faults, the writer remains in `state` for close.
4. **Checkpoint list.** Store `DbIoU64List` in the state before the first value. Each step calls `grant`, reads exactly one u64 from the immutable span, pushes it, increments `next/value_at`, then yields. On the last value, move the list into `WalRecord::IndexCkpt`; do not keep a duplicate. `DbIoU64List::push` performs its bounded backing allocation at [`db_storage/🦀️.rs:1857-1876`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1857), so the control grant must precede that mutation.
5. **Close without control.** `WalRetainedRecordDecoder::close_owner_step` first drives `writer.close_step()` until terminal, then `run_ids.close_step()` until terminal, clears trivial state, and never calls `grant`. `terminal_is_empty` requires both options absent/terminal. The outer committed/raw cursor invokes this first in its own `close_owner_step`, before closing source `DbIoPages` and its segment list.

All scalar records can finish in `Start`: header, logical terminals, CAS payload, frontier, VCS ref, snapshot marker, and lease. Their present `DbIoText` boundary is 1,024 bytes at [`db_wal/🦀️.rs:1065-1079`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1065), so no unbounded retained source is introduced. The byte-owning variants are Command, inline Payload, Diff, Inverse, Event, Outbox, and Migration.

`DbIoPageWriter::try_reserve_for_operation` currently returns a rejected object without a writer on admission/check-out failure ([`db_storage/🦀️.rs:552-570`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:552)). That is an implementation fact, not a safe reason to use `into_error`: `DbIoPageWriterRejected` expressly has `into_parts`/`into_writer` ([`:797-813`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:797)). The decoder must retain any returned writer in its error/closing state, then report the error. Using direct `seal_retained_step` avoids the only normal sealing rejection path entirely.

## Cursor ownership integration

`WalCommittedCursor` holds `pending_frame: Option<WalRecordFrame>` and `decoder: Option<WalRetainedRecordDecoder>`. A returned `WalCommittedTransaction` borrows that cursor; its `next_record_step` installs one decoder for the next descriptor, and no second descriptor may start while a record/decoder is live. The borrowed transaction forwards `replenish`, because its exclusive borrow prevents the parent cursor from doing so. `close_record_step` must empty the delivered `WalRecord` before `next_record_step` advances.

On an interruption, leave `pending_frame`, decoder, source pages, descriptor index, and parent offset unchanged apart from successfully copied bytes/list entries held by the decoder. On a structural or allocation error, mark the cursor failed **but retain that decoder**; subsequent `next_step` fails closed and only close can drain it. Dropping an unfinished borrowed transaction likewise poisons the cursor, while its parent remains closable. This is the same fail-stop/terminal-close separation already used by `WalReplayCursor` at [`db_wal/🦀️.rs:1450-1456,1501-1530`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1450).

The budget stays within the existing storage contract. A readable segment is at most 496 KiB ([`db_storage/🦀️.rs:69-85`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:69)), i.e. at most 31 16-KiB pages; one same-operation copied record can add at most 31 pages. This is below the 64-page per-operation cap at [`db_storage/🦀️.rs:71-82`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:71). The transaction's 64 descriptors contain no byte owners. A decoded 64-record batch would not have this property.

## First executable laws

Add these beside the planned `wal-committed-transactions-v1` neutral corpus and the current retained-replay laws at [`db_wal/🦀️.rs:2813-2871`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2813). The fixture declares logical frames and interruption cut points; the test builder writes ordinary SPR bytes.

1. **Copy interruption neither skips nor parks.** A committed transaction contains a two-page `Command`. Exhaust fuel after its destination writer exists and after its first fragment. `next_record_step` returns the recognised interruption; after `replenish`, it emits exactly the original bytes once. The parent cannot advance to the next span while the record is live. Cancel, close with the cancel flag still set, and prove terminal empty before a fresh replay succeeds.
2. **Seal interruption owns the writer.** Exhaust fuel after the final copied fragment but before every required `seal_retained_step` opportunity. Replenish until the record is ready. No record is observable before sealing, and closing the cursor without replenishing drains the writer/source pages without relying on global `db_io_maintenance_step`.
3. **Checkpoint interruption resumes exact index.** An `IndexCkpt` with 4,096 ids interrupts after the backing is allocated and after a nonzero prefix. Replenish and emit one ordered 4,096-id record—no duplicate/omitted ids—then close terminally with cancellation asserted. A declared count of 4,097 rejects before list backing allocation; a short or trailing body rejects before a list is created.
4. **Malformed retained body never advances its materializer.** An inline payload whose declared length leaves a trailing byte, and an opaque frame with a bad CRC/back length, both fail with no materialized record. Raw replay retains its pending frame at the offending source offset; committed replay retains its current descriptor index even though the grammar already consumed the address-only span. After close, repeat an exact-page writer admission/replay without global-maintenance assistance. This catches line 1219's old early offset assignment and byte-prefix allocation before trailing validation.
5. **64 spans remain fixed-memory.** A valid 64-command transaction with tiny fields, plus a two-page final field, emits after logical commit and is consumed one record at a time; a 65th descriptor rejects before opaque allocation. Repeated fuel interruptions at every descriptor still permit a terminal close and a new replay. This is the direct guard against replacing spans with `WalRecordBatch`.

## Implementation order

1. Introduce private frame descriptor/reader, keeping physical CRC/back-length behaviour and source-offset commit skipping; use it in the raw replay path too so raw replay no longer skips interrupted retained records.
2. Add private `WalRetainedRecordDecoder` with step/close/terminal APIs, and replace the current async materializer. Keep all retained owner fields in cursor state before an operation that can fail.
3. Build `WalTransactionGate` over frame descriptors; then put the decoder behind its borrowed committed transaction. The gate continues to defer opaque materialization until matching `TxCommit`.
4. Add the five laws above, then migrate artifact/sync/compaction consumers from the already-recorded transaction packet.

This is deliberately a small internal slice: no storage compatibility API and no bulk decoded owner are necessary. The essential invariant is that interruption preserves a decoder state the caller can either replenish or terminally close.
