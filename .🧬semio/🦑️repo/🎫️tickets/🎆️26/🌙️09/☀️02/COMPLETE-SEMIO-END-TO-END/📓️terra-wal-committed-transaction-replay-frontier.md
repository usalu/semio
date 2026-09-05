# WAL Committed-Transaction Replay Frontier

Read-only audit on 2026-09-05. No product files were changed and no build was run.

## Decision

Add one `db_wal` committed-transaction cursor shared by `db_artifact` and `db_sync`. It must not be a `WalReplayCursor` wrapper that accumulates decoded `WalRecord`s in `WalRecordBatch`. It can make logical transactions atomic without unbounded payload ownership only by retaining a fixed set of **verified frame spans**, then decoding one body record at a time after the matching `TxCommit`.

The existing transaction vocabulary is sufficient: `SegmentHeader`, `TxBegin { tx_id }`, `TxCommit { tx_id, record_count }`, `TxAbort { tx_id }`, and the other records are already distinct at [db_wal `🦀️.rs:428`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:428). The missing contract is the grammar and its sole materializing cursor.

## Why decoded `WalRecordBatch` is not safe

`WalRecordBatch` is fixed at 64 slots ([`🦀️.rs:447`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:447)), but its byte-bearing entries each own `DbIoPages`. `WalReplayCursor` reads the source segment into an operation and `wal_decode_page_record` creates each `WalBytes` with `try_reserve_for_operation` ([`🦀️.rs:1125`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1125)). A valid transaction of 64 one-byte commands would therefore hold 64 rounded-up destination pages plus at least one source page: more than the 64-page per-operation maximum ([storage `🦀️.rs:72`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:72)). It fails admission although the on-disk transaction is legal.

The current raw cursor avoids this only because it yields and callers close each record before requesting the next ([`WalReplayCursor::next_step`, `🦀️.rs:1435`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1435)). A transaction gate must preserve that property.

## Exact seam

Keep `replay_document` as the raw forensic/segment cursor. Add a second public constructor, for example:

```rust
pub async fn replay_committed_document<'storage, S: WalStorage>(
    storage: &'storage S,
    document: &ArtifactId,
    control: WalCursorControl,
) -> Result<WalCommittedCursor<'storage, S>, DbError>;

pub enum WalCommittedStep<'cursor> {
    Transaction(WalCommittedTransaction<'cursor>),
    Yield,
    Done,
}
```

Internally factor the raw record reader at [`wal_next_page_record`, `🦀️.rs:1177`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1177) into a verified-frame reader. Its non-owning descriptor is only:

```rust
struct WalRecordFrame {
    kind: u8,
    payload_start: usize,
    payload_end: usize,
}
```

The shared private `WalTransactionGate` holds at most 64 such descriptors, the current transaction id, and its current segment index. It consumes the frame reader only after `WalSegmentChain` has finished verification (the existing whole-segment precondition at [`🦀️.rs:1453`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1453)). It parses only boundary payloads while collecting. It does **not** copy an opaque command/payload/diff/event/outbox/migration body.

On a matching `TxCommit`, return a borrowed `WalCommittedTransaction` over the cursor's still-open source pages and its fixed frame array. Its `next_record_step` decodes exactly one stored frame with the existing `wal_decode_page_record`, and requires that returned `WalRecord` be closed before another record is requested. `finish` is required to consume all frames; dropping an unfinished borrowed transaction poisons the cursor. Thus source pages remain live through consumer decoding, no opaque body is exposed before commit, and both descriptor memory and page ownership remain bounded.

Do not return an owned transaction that lets the outer cursor continue into the same segment while the caller retains the old transaction: that permits the caller to accumulate another set of per-record pages. The borrow/finish shape is the small enforceable ownership boundary.

Use the same `WalTransactionGate` in `ArtifactWal::open_with_control`'s verified-prefix pass, rather than independently duplicating pairing/count/id logic. The current [`validate_wal_prefix`, `🦀️.rs:1542`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1542) only takes the maximum raw marker id. It accepts mismatched/nested markers and a generically committed `Begin + body` without a logical terminator, then resumes the writer. The next `submit` appends a nested `TxBegin`, which the new replay cursor must reject. This is a real open/replay split today.

## Frozen grammar and recovery result

| Input state | Required result |
|---|---|
| Outside a transaction | Only the validated initial `SegmentHeader` of a segment or a `TxBegin` is legal; every business record, `TxCommit`, and `TxAbort` is corruption. Segment header is never exposed by the committed cursor. |
| `TxBegin(id)` | `id != 0`; start one transaction, set high-water to `id + 1` with checked arithmetic, and start an empty fixed frame array. |
| Interior business record | Store one frame descriptor; record 65 is `LimitExceeded("wal transaction records")` before any opaque body allocation. Physical `REC_COMMIT` frames are irrelevant, so a transaction may cross several physical commits within the same segment. |
| `TxCommit { id, record_count }` | Require the open `id` and exact interior count; emit exactly one transaction, including count-zero transactions. |
| `TxAbort { id }` | Require the open `id`; drop only descriptors and emit nothing. It still consumes that id. |
| A second `TxBegin`, a segment header, or EOF while a transaction is open | A new segment header is always corruption: logical transactions may not cross WAL segments. EOF needs the active/sealed recovery rule below. |
| Any id at `u64::MAX` | `LimitExceeded("wal transaction sequence")`; the writer cannot assign a successor. |

The retained first segment may start after compaction, so do not require its first id to be one or contiguous from discarded history. Within retained data, preserve the greatest valid **begin** id plus one; commits and aborts must match it, not independently raise the high-water. This reserves an id that reached a verified physical commit even if its logical transaction later aborts or is incomplete.

For a complete physical prefix that ends after `TxBegin`/body but before its logical terminator:

* on a sealed segment, a non-highest segment, or at a successor header: reject as corruption—rotation only happens after `submit` has terminated its transaction ([`ArtifactWal::submit`, `🦀️.rs:1870`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1870));
* on the sole active highest segment: the shared scanner returns a private `Incomplete { tx_id }` recovery outcome; `open_with_control` appends the matching `TxAbort`, commits and fsyncs it, and only then resumes. It must retain `next_tx_id = tx_id + 1`.

This handles a future writer that physically commits a partial logical transaction without materializing its records. The current writer normally places its one physical commit after `TxCommit`; no replacement of that ordering is needed.

## First call sites

1. [`ArtifactEngine::open_retained`, `db_artifact/🦀️.rs:1248`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1248) currently clears `batch_ids` on raw `TxBegin`, calls `apply_one` for every raw command, and assigns raw `Frontier` before ever seeing `TxCommit` ([`:1261-1285`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1261)). Replace that loop with one committed transaction at a time. Build `batch_ids` from that transaction's commands, apply/decode only within it, consume frontier only within it, then finish/close it. The current error path returns no engine, so the first objective is no durable-record exposure before commit; a later in-memory staging refinement is distinct.
2. [`replay_sync_state`, `db_sync/🦀️.rs:128`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:128) currently appends raw commands and increments `commit_seq` only when a later raw commit arrives ([`:143-152`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:143)). Consume a completed transaction, decode/chain its commands, apply its snapshot frontier marker, then increment `commit_seq` once. An abort must change none of those values.
3. [`replay_sync_state_retained`, `db_sync/🦀️.rs:946`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:946) is the independently implemented database-sync-hello path and has the same raw-command/raw-commit split at [`:980-989`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:980). It must be converted in the same change, preserving its opportunity/replenish ledger.

Follow-on raw semantic consumers should not be silently left as materializers: compaction derives horizons and CAS liveness directly from raw records at [`db_compact/🦀️.rs:734`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:734) and [`:768`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:768), so an aborted `SnapshotPub` or CAS payload is presently treated as live. Give committed transactions their segment index for those two loops. The CLI raw dump may remain raw, but its `verify` command should run the grammar gate. `db_artifact`'s byte-level history token parser recognizes `TxBegin` but not commit/abort at [`🗿️artifact/🦀️.rs:2772`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2772) and [`:2904`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:2904); it needs the same gate before it becomes a source of externally visible history.

## Neutral fixture and executable laws

Add a schema-first corpus beside the existing recovery corpus:

`db_wal/🧪️fixtures/✅️committed-transactions/🧬️.schema.json` and `🔣️.json`.

Use `schema: "wal-committed-transactions-v1"` and a declarative `segments` array. Each segment has `index`, `state` (`active` or `sealed`), `physicalCommitAfter` (record ordinals), a `records` array (`header`, `begin`, `command`, `frontier`, `snapshotPub`, `payloadCas`, `commit`, `abort`), and `expect` (`emit`, `abort`, `recoverAbort`, or `reject`). The test builder writes actual SPR frames, so fixture intent stays language-neutral while the Rust oracle validates its byte representation.

Minimum laws:

1. `commit-gates-visibility`: a two-command/frontier transaction with physical commits after begin and first command emits nothing until its matching logical commit; artifact and sync then agree on two commands, one commit, and the frontier.
2. `abort-hides-everything`: begin, command, snapshot marker, abort emits no transaction; artifact state, sync command chain, `commit_seq`, and floor remain genesis; reopening allocates the successor tx id.
3. `terminal-mismatch-is-corruption`: wrong commit id, wrong `record_count`, commit/abort outside a transaction, nested begin, and business record outside a transaction all return `Corrupt` before any transaction is emitted.
4. `incomplete-active-is-durably-aborted`: a verified physical begin/body prefix in the highest active segment emits nothing; `ArtifactWal::open` appends/fsyncs one matching abort, preserves the begin id as consumed, and a second open is byte-identical. The same prefix in sealed/non-highest or followed by another header is rejected.
5. `sixty-four-tiny-records-stay-bounded`: a legal 64-record transaction succeeds with source pages plus only one decoded record live at a time; record 65 is rejected without a residual page owner. This catches the invalid decoded-`WalRecordBatch` design.
6. `id-high-water-survives-abort-and-tail`: aborted and recovery-aborted ids are never reused; a `u64::MAX` begin rejects recovery/submit rather than wrapping.
7. `segment-boundary-is-not-a-transaction-boundary`: physical commits within one segment are admitted, but a logical transaction crossing segments is rejected even if both segment hash chains are valid.

The existing [`wal_recovery_preserves_neutral_committed_prefixes`, `db_wal/🦀️.rs:2036`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2036), [`open_replays_the_wal_and_reconstructs_state_and_frontier_identically`, `db_artifact/🦀️.rs:4441`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4441), and [`replay_sync_state_derives_frontier_and_ordered_commands`, `db_sync/🦀️.rs:2621`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2621) are the reuse points, but none currently asserts abort/count/id grammar.
