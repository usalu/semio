# WAL Committed-Transaction Gate: Current Implementation Packet

Read-only current-source audit on 2026-09-05. I reread the earlier committed-transaction and tail-recovery audits, then rechecked the current WAL, artifact, sync, compaction, and history consumers. No product file was changed and no build was run.

## Current frontier after physical-recovery work

The physical recovery/fail-stop/capacity corrections are present, but they do not provide logical transaction admission:

| Current source | What is now covered | What remains absent |
| --- | --- | --- |
| [`db_wal/🦀️.rs:1746`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1746) | `commit_and_flush` checks the exact append length and poisons the writer after append/sync failure. | It does not know whether a physically committed record sequence has a logical `TxCommit` or `TxAbort`. |
| [`db_wal/🦀️.rs:1993`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1993) | Submission preflights a whole begin/body/commit/physical-commit reservation, rotates first, and emits normal transactions in the right order. | Recovery must still safely handle an active WAL made by a future/failed writer that physically commits `Begin + body` before it writes a logical terminal. |
| [`db_wal/🦀️.rs:1925`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1925) | `open_with_control` verifies physical retained prefix, repairs only an active tail, and resumes the verified writer. | Its `validate_wal_prefix` only finds the maximum id from *every* begin/commit/abort marker. It accepts mismatches, nesting, bodies outside a transaction, and a physically committed unterminated transaction. |

The precise hole is [`validate_wal_prefix`, `db_wal/🦀️.rs:1557-1573`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1557): it decodes each raw record through [`wal_next_page_record`, `:1191-1228`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1191), calls `WalRecord::tx_id` on any boundary, and takes `max(id + 1)`. The source has no committed-transaction cursor, no grammar validation, and no `TxAbort` recovery path. The root recovery report's green receipts are therefore evidence for physical preservation only, not for logical transaction visibility; I did not rerun them.

## Smallest durable seam

Keep `replay_document` and `WalReplayCursor` raw. They are needed by forensic CLI output and segment diagnostics. Add one parallel public materializing cursor in `db_wal`:

```rust
pub async fn replay_committed_document<'storage, S: WalStorage>(
    storage: &'storage S,
    document: &ArtifactId,
    control: WalCursorControl,
) -> Result<WalCommittedCursor<'storage, S>, DbError>;

pub enum WalCommittedStep<'cursor, 'storage, S: WalStorage> {
    Transaction(WalCommittedTransaction<'cursor, 'storage, S>),
    Yield,
    Done,
}
```

`WalCommittedTransaction` must expose `tx_id`, `segment_index`, and the exactly validated business records only. It borrows the cursor; the outer cursor cannot advance to another transaction/segment while it exists. Its record iterator returns one decoded `&mut WalRecord` at a time; `close_record_step` must finish that record before another can be decoded, and `finish` must consume every span. Dropping an unfinished transaction marks the cursor failed; `WalCommittedCursor::close_owner_step` still drains any current record and source pages. This mirrors the current raw cursor's fail-stop/terminal ownership behavior at [`db_wal/🦀️.rs:1450-1530`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1450).

It must **not** use `WalRecordBatch`. That type's 64 retained `WalRecord` slots ([`db_wal/🦀️.rs:447`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:447)) own decoded `WalBytes`; even 64 tiny commands can exceed the page-operation cap when accumulated. Instead factor the framing prefix of `wal_next_page_record` into:

```rust
#[derive(Clone, Copy)]
struct WalRecordFrame {
    kind: u8,
    payload_start: usize,
    payload_end: usize,
}

struct WalTransactionGate {
    transaction_id: Option<u64>,
    frames: [Option<WalRecordFrame>; 64],
    frames_len: u8,
    next_tx_id: u64,
}
```

The factored frame reader validates frame length, flags, CRC, and back-length exactly as the current reader does, skips physical `REC_COMMIT` frames, and returns an address-only span for each `WAL_*` frame. `WalSegmentChain` remains the prior whole-segment chain/header verifier ([`db_wal/🦀️.rs:1240-1368`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1240)); the gate is run only after that verifier has accepted the segment.

For `TxBegin`/`TxCommit`/`TxAbort`, parse only their fixed 8/12/8-byte payloads directly from `WalPageReader`. For a business frame, retain the descriptor only. After a matching commit, decode exactly one descriptor through the existing `wal_decode_page_record` using a bounded `WalPageReader(payload_start..payload_end)`; no opaque command/payload/event bytes are allocated while the transaction is still provisional. A physical commit boundary may occur between any two logical frames, but an open logical transaction cannot cross a segment boundary.

The gate and materializing cursor share the same private state machine. `ArtifactWal::open_with_control` calls its verifier-only mode; `WalCommittedCursor` calls its emitting mode. Do not duplicate marker/count/id rules in an opener-specific function.

## Frozen grammar and active-tail recovery

| Gate state/input | Required result |
| --- | --- |
| Segment start | `WalSegmentChain` has already accepted exactly one first header. The committed cursor never emits that header. Only `TxBegin` may then start a logical transaction. |
| Outside a transaction | Any business record, commit, abort, second header, or unknown kind is `Corrupt`. |
| `TxBegin { id }` | `id != 0`; checked `id + 1` is reserved as the high-water successor; initialise zero frame spans. The retained first id need not be one because compaction may have removed earlier history. |
| Interior business frame | Store only one descriptor. Record 65 is `LimitExceeded("wal transaction records")` before opaque-body allocation. |
| `TxCommit { id, record_count }` | Require the same open id and exact descriptor count; emit one transaction, including count-zero. |
| `TxAbort { id }` | Require the same open id; clear descriptors and emit nothing. The id stays consumed. |
| Nested begin, terminal mismatch, header/EOF while open | `Corrupt`, except the specifically writable active-highest recovery case below. |
| Any begin at `u64::MAX` | `LimitExceeded("wal transaction sequence")` before mutation; a recovery abort cannot make a usable successor. |

High-water derives only from valid begins, then takes the greatest checked successor across segments. Commit/abort ids merely match an open begin; they never independently advance it. This corrects the present `WalRecord::tx_id` maximum logic at [`db_wal/🦀️.rs:501-510`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:501).

For a fully physical-verified prefix that ends `Begin + business frames` without a logical terminal:

1. A sealed segment, any non-highest segment, or a successor header is corruption. Rotation seals only after `submit` has emitted the terminal ([`db_wal/🦀️.rs:2009-2013`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2009)).
2. Only `index == last && state == Active` may return private `Incomplete { tx_id, next_tx_id, record_count }` from the gate.
3. In `open_with_control`, first finish validation/copy, then remove a physical torn tail (if any) and Fsync it exactly as now ([`db_wal/🦀️.rs:1934-1941`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1934)). Resume the verified prefix, append the matching `TxAbort`, and `commit_and_flush(...Fsync)` before returning an `ArtifactWal`. Thus durable begin/body bytes remain but can never materialize, and a second open is byte-identical.
4. After appending the recovery abort, take the returned segment's new tip, not the old `span.chain()`, for the next prior-tip value. Preserve `next_tx_id = tx_id + 1`.

If the incomplete begin/body is itself after `span.end`, it was never physically committed and ordinary tail truncation removes it; no synthetic abort is written. All validation and `u64::MAX` checks happen before truncate/append mutation. Repair/close remain ungated by cancellation; scanning and descriptor collection use the caller's existing `WalCursorControl` grants/yields.

## Coherent consumer migration

This cannot land as a `db_wal`-only API: an active-tail recovery abort makes the existing raw materializers observably unsafe. The first coherent change set is five production paths, plus the two forms of compaction:

| Owner/current raw site | Migration using committed transactions |
| --- | --- |
| [`ArtifactEngine::open_retained`, `db_artifact/🦀️.rs:1248-1285`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1248) | Replace the raw loop. At each returned transaction, clear `batch_ids`, process records in order, and finish it before requesting another. `apply_one` explicitly wants ids seen *earlier in the same batch* ([`:1321-1334`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1321), so no two-pass/all-envelopes allocation is needed. The engine is not returned on a decode/apply error, so post-terminal sequential application is sufficient for the first slice. Close every record before the next one; do not retain the present `continue` path at `:1268-1270`, which bypasses explicit record closing. |
| [`replay_sync_state`, `db_sync/🦀️.rs:128-163`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:128) | Decode/hash commands and take `SnapshotPub` only from an emitted transaction. Increment `commit_seq` once after `transaction.finish`, not from a later raw marker. An abort changes no command, hash, floor, or commit sequence. |
| [`replay_sync_state_retained`, `db_sync/🦀️.rs:946-1015`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:946) | Same semantics in the database-sync-hello worker, preserving its deadline/fuel replenishment, owner ledger, cancellation checks, and error-side envelope retirement. The borrowed transaction needs `replenish`, since the outer cursor is unavailable while it is borrowed. |
| Ordinary compaction horizon/payload scans, [`db_compact/🦀️.rs:734-793`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:734) | Consume only emitted `Frontier`/`SnapshotPub` and CAS payloads. Use `transaction.segment_index` instead of raw segment headers. Otherwise an aborted snapshot can delete a segment and an aborted CAS reference can keep/delete payloads incorrectly. |
| Retained compaction's identical scans, [`db_compact/🦀️.rs:1162-1237`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1162) | Make the same change while retaining its explicit page owners, cancellation opportunities, and fixed backing ledgers. Leaving this sibling raw produces backend-dependent logical visibility. |

The byte-level history endpoint is a required follow-on before claiming *all* user-visible replay is transaction-safe: `HistoryReplayFuture` is entered from `ArtifactAuthority::history_retained` ([`db_artifact/🦀️.rs:3990`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3990)), but its independent parser treats begin as “clear pending” ([`:3368`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3368)) and classifies both logical terminals as `Other` ([`:2903-2908`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2903)). It can publish command/frontier effects before a later abort. Replace that custom frame admission with the shared gate/cursor (or do not expose history until migrated); adding `TxAbort` special-cases to the custom parser would duplicate the grammar and still miss count/id checks.

CLI `wal-inspect`/`replay` may stay explicitly raw diagnostics. `verify_document` at [`db_cli/🦀️.rs:801-821`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:801) must call a verifier-only committed grammar pass, however, so “verified” does not bless a logically invalid physical prefix.

## Neutral corpus and executable laws

Add `db_wal/🧪️fixtures/🧾️committed-transactions/🧬️.schema.json` and `🔣️.json`, separate from the existing recovery/capacity/fail-stop fixtures. Schema `wal-committed-transactions-v1` should carry physical commit cuts and declarative logical frames (`header`, `begin`, business kind, `commit`, `abort`), each segment's active/sealed state, and an expected `emit`/`recoverAbort`/`reject` result. The test writer creates actual SPR bytes; the neutral corpus never contains implementation-owned page offsets.

1. **Commit gates visibility:** physical commits after begin and first body do not expose anything; matching logical commit exposes exactly two commands/frontier once to artifact and both sync paths.
2. **Abort has no semantic effect:** begin, command, snapshot publication, CAS payload, abort yields genesis sync state and no artifact command; compaction sees neither the snapshot horizon nor payload liveness; next open never reuses the aborted id.
3. **Grammar rejects before emission/allocation:** body outside, nested begin, wrong terminal id/count, terminal outside, header inside, 65th body, and cross-segment open transaction are rejected without an emitted transaction or opaque owner.
4. **Active committed incomplete is durably aborted:** a highest active prefix ending after begin/body receives exactly one matching Fsync abort on open; the bytes/next id are identical after second open. The sealed, non-highest, successor-header, and `u64::MAX` versions reject without mutation. A body only in an uncommitted physical tail is truncated without an abort.
5. **Bounded/cancellable ownership:** 64 tiny bodies are admitted with source pages plus at most one decoded record live; after cancellation/fuel exhaust the borrowed transaction resumes only after replenish, and close reaches terminal ownership while cancellation stays asserted.

Reuse and extend the current physical recovery/capacity/fail-stop harnesses at [`db_wal/🦀️.rs:2209`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2209) and [`:2704`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2704), plus the existing artifact open and sync replay laws at [`db_artifact/🦀️.rs:4441`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4441) and [`db_sync/🦀️.rs:2621`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2621). Add a compaction regression beside [`sweep_payloads_deletes_orphaned_candidates_but_keeps_hashes_still_referenced_elsewhere`, `db_compact/🦀️.rs:2903`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:2903) to prove aborted CAS/snapshot frames are invisible.

## Implementation order

1. Factor non-owning frame parsing; implement the private `WalTransactionGate`; replace `validate_wal_prefix` with the gate's verifier-only scan and active incomplete outcome.
2. Add `WalCommittedCursor`/borrowed transaction on the same gate, including cancellation, fail-stop, and terminal-close mechanics; add the neutral grammar corpus.
3. Wire active incomplete recovery abort into `ArtifactWal::open_with_control`, including tip/high-water update and idempotence tests.
4. Migrate artifact, both sync paths, and both compaction forms in one change; then replace `HistoryReplayFuture`'s raw logical-frame policy and wire CLI verification to verifier-only grammar.

This is the smallest retained implementation that makes a logical terminal—not a later record consumers happen to see—the one admission point for durable WAL effects.
