# Tail-Only WAL Open: Current Audit

Read-only review of the current recovery implementation on 2026-09-05. I did not run a build or tests. Root reported the two exact native recovery laws green; that result is not independently reverified here.

## Result

The new `open_with_control` has the right primary ordering for normal recovery: it lists and validates dense indices before mutation, validates every sealed segment before reaching the active segment, validates the active prefix with both the retained SPR scanner and `WalSegmentChain`, then copies the verified prefix before truncating only the active tail ([`db_wal/🦀️.rs:1830`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1830)). Exact partial-header comparison also happens before `truncate_tail` ([`:1654-1672`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1654)). The recent `SharedBuf::copy_range` inner write loop now handles a partial destination-page write ([`:897-917`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:897)); the reported large-seed issue should be covered by an exact regression law after the in-flight fix lands.

Two P0 correctness holes remain outside the already-reported resumable-cursor and transaction-gate work. Both are reachable through the current public storage contract, not merely synthetic corruption.

## P0 — Segment capacity is larger than every storage read span

`DEFAULT_MAX_SEGMENT_BYTES` is 512 KiB ([`db_wal/🦀️.rs:1788`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1788)), but the storage implementation caps an individual WAL `append` and `read` at 496 KiB ([`db_storage/🦀️.rs:69`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:69), [`:7300-7317`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7300)). Memory storage also rejects a segment whose total length exceeds that same maximum ([`:5886-5888`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:5886)).

Rotation occurs only *after* the transaction has been written and flushed ([`ArtifactWal::submit`, `db_wal/🦀️.rs:1940-1954`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1940)). Therefore:

* an Fsync transaction whose suffix alone exceeds 496 KiB fails late on `append`, after `SprWriter::commit` has already advanced the in-memory writer ([`SegmentWriter::commit_and_flush`, `:1722-1735`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1722)); and
* on the filesystem backend, smaller individual suffixes can grow the total just past 496 KiB, then rotate and seal that segment. The next `ArtifactWal::open` reads the whole segment at [`:1855-1857`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1855), so the valid sealed history becomes unreadable before validation even begins.

This is backend-divergent: Memory refuses the crossing append; filesystem permits the crossing append but later refuses reopen. The existing forced-rotation test uses a 200-byte threshold and cannot see either boundary ([`:2477-2511`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2477)).

### Smallest correction

Define one public, shared WAL-segment byte ceiling at or below the storage full-read/append ceiling. Preflight a complete `TxBegin + records + TxCommit + physical commit` before changing `SprWriter`: if it does not fit the remaining segment budget, rotate first; if it cannot fit an empty segment, reject the submission before any record is appended. Post-write rotation is retained only as an assertion/normal path, never as the capacity guard. `scan_retained_pages` should use that same ceiling rather than its current 64-page (1 MiB) verifier cap ([`:1533-1550`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1533)).

### Required law

Seed an FS and a Memory WAL to one complete transaction below the ceiling, submit a second transaction that would cross it, reopen, and assert identical replay on both backends: all pre-existing committed bytes remain; the second transaction is either committed in a newly created successor or rejected before any local writer/storage mutation. A one-transaction-over-empty-segment case must reject deterministically rather than poison the writer after `commit()`.

## P0 — `append` length is trusted, so a torn/stale successful append poisons the live writer

`WalStorage::append` promises the segment's new total length ([`db_storage/🦀️.rs:4584-4586`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4584)). `commit_and_flush` stores that returned value directly as `flushed_len`, clears `pending_records`, and returns success without requiring it equal `old_flushed_len + suffix_len` ([`db_wal/🦀️.rs:1727-1735`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1727)).

This matters in the existing test infrastructure: `FaultStorage::torn_write_at` intentionally forwards only a prefix while returning its real shorter total length as an otherwise successful append ([`db_testkit/🦀️.rs:336-357`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:336)). The current live WAL then reports the transaction committed, has an in-memory writer past the complete commit, and has a physical segment ending in a torn prefix. A subsequent flush starts at the shortened `flushed_len` and appends into that corrupt tail rather than requiring recovery first. A stale *larger* returned length is also bad: `pending_bytes` subtracts it without a checked precondition ([`db_wal/🦀️.rs:1707-1710`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1707)), while `copy_range` later uses a saturating subtraction, producing divergent bad states.

### Smallest correction

Compute `expected_len = flushed_len.checked_add(suffix_len)` before the append and require `new_len == expected_len`. On any mismatch, do not clear `pending_records`, do not issue a success receipt, and poison the in-memory `SegmentWriter` so no later `submit`/`force_flush` can append behind an uncertain tail; the caller must drop and reopen, which is the only safe recovery boundary. This is also the right place to reject a stale/larger backend result without unsigned underflow.

If `append` returns the exact expected length but `sync` fails, update `flushed_len`/pending bookkeeping before returning that error (the bytes are already visible to the storage API), or likewise poison and demand reopen. The present code changes neither state before a `sync` error, so a retry can re-append the already written suffix.

### Required law

Use the existing `FaultStorage` torn-write hook on a non-genesis append. The operation must return an error—not `WalAppendReceipt { committed: true }`—and all further writes on that `ArtifactWal` must return `Closed`/poisoned. Drop it, reopen the inner storage, and assert the last complete prefix is exactly preserved and the torn tail alone is removed. Add a minimal sync-after-success failure shim too: retry/reopen must not duplicate the committed frames.

## Untested but bounded recovery branches

| Priority | Current path | Missing exact case | Expected result |
|---|---|---|---|
| P1 | Invalid partial active header, [`initialize_existing_empty`, `db_wal/🦀️.rs:1654`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1654) | Highest active segment contains one byte that differs from the exact new-format header. Existing `successor-partial` covers an exact prefix, not a mismatch ([`:2171-2173`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2171)). | `Corrupt`; byte-for-byte segment and `Active` state unchanged. This guards the present validation-before-truncate ordering. |
| P1 | Partial header mutation failure | Exact partial header, then inject failure of `truncate_tail` or of the following `sync`. | Reopen may reconstruct only the uncommitted header; it must never discard a verified record, leak the constructed `SharedBuf`, or call append before successful validation. |
| P1 | Forced rotation after sealing, [`rotate`, `db_wal/🦀️.rs:1969`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1969) | `seal` succeeds but successor creation/header append fails; separately, an all-sealed highest segment is reopened. | The old sealed bytes remain unchanged; the failed in-memory handle is unusable; reopen creates exactly `last + 1` with the sealed tip and restores the next tx id. The all-sealed steady recovery is covered in outline by `highest-sealed`, but not this post-seal failure boundary. |
| P1 | Tail truncate after all validation, [`open_with_control`, `:1875-1897`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1875) | Cancel/fuel exhaustion during scan/copy and immediately before mutation; then an injected truncate or sync failure. | Before mutation, exact original bytes; after an error following successful truncation, only the non-committed tail may be gone and reopen is idempotent. Cleanup must reach terminal ownership without consulting cancellation. |
| P2 | Newly explicit close ownership, [`ArtifactWal::close_step`, `:1984-1990`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1984) | Repeated close is tested, but normal `db_artifact` owner shutdown does not currently call it. | Wire the already-added close protocol through the document authority; otherwise ordinary `drop` parks writer pages for deferred maintenance rather than proving bounded prompt retirement. This is the caller-integration gap noted in the earlier controlled-open audit, not a defect in the new close implementation itself. |

## Non-findings in the reviewed slice

* A sealed non-highest segment is rejected on any tail or zero physical commit before an active segment is mutated ([`db_wal/🦀️.rs:1866-1874`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1866)).
* The `last + 1` and transaction-id arithmetic is checked before the relevant rotate/open mutation paths ([`:1840`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1840), [`:1935-1937`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1935), [`:1970-1972`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1970)).
* The compacted-boundary authentication question and transaction pairing/abort grammar remain deliberately outside this audit; they are covered by the prior recovery and committed-transaction reports.
