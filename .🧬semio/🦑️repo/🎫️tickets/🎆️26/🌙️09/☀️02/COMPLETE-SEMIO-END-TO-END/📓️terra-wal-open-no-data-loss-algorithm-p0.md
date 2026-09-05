# WAL Open: No-Data-Loss Recovery Algorithm

## Result

`ArtifactWal::open` must be a verify → tail-only repair → exact writer-resume operation. Its current implementation destroys the highest segment and coalesces trusted records into a replacement (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1679-1721`), so a crash after delete/create can erase an otherwise committed anchor. It also relies on `protocol::format::recover` alone and therefore does not enforce the WAL segment-header chain before mutation.

The existing pieces are sufficient for a safe active-segment path:

- `RetainedSprVerification` returns a private-constructor `VerifiedSprSpan { end, sequence, commit_offset, chain }` only after full retained framing/commit verification (`🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🦀️.rs:40-58,96-137`). A valid uncommitted/torn suffix is represented by `end < total`, not by a forged span.
- `WalSegmentChain` checks the stronger WAL meaning: exact document/index segment header, first/genesis or previous-tip relationship, frame CRCs, record counts, commit offsets, and the segment hash chain (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1237-1354`).
- `SprWriter::resume_verified` demands the sink end equal the verified end, seeds the preceding chain and commit offset, advances commit sequence checked, and starts with zero pending records (`🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs:467-487`).
- A resumed `SegmentWriter` must set `flushed_len = span.end`, `pending_records = 0`, and `oldest_pending_at_ms = None`; otherwise its next flush duplicates the retained prefix (`db/📝️wal/🦀️.rs:1530-1590`).

## Required helper boundaries

Keep these helpers private to `db_wal`; none grants storage authority.

```rust
async fn scan_retained_pages(
    pages: &DbIoPages,
    control: &mut WalCursorControl,
) -> Result<VerifiedSprSpan, DbError>;

async fn validate_wal_prefix(
    pages: &DbIoPages,
    span: &VerifiedSprSpan,
    index: u64,
    prior: WalPriorChainTip,
    document: &ArtifactId,
    control: &mut WalCursorControl,
) -> Result<[u8; 32], DbError>;

async fn copy_verified_prefix(
    pages: &DbIoPages,
    end: u64,
    control: &mut WalCursorControl,
) -> Result<SharedBuf, DbError>;
```

`scan_retained_pages` feeds every `DbIoPages::fragments()` slice to one `RetainedSprVerification::new(pages.len(), limits)`. Before each page-bounded `push`, call `WalCursorControl::grant()` and yield after progress; use the scanner's local fuel only to bound that push. Do not flatten pages into a `Vec`. `WalCursorControl` already performs cancellation/deadline/fuel checks (`db/📝️wal/🦀️.rs:173-205`), while the scanner is explicitly incremental (`format/🔎️verification/🦀️.rs:117-137`). Map scanner header/frame/commit diagnostics to `DbError::Corrupt`, capacity to `LimitExceeded`, and cancellation to the corresponding unavailable/cancelled path. A scanner failure must happen before `truncate_tail`.

`validate_wal_prefix` drives `WalSegmentChain::step` to `true` over exactly `span.end`, not the physical length. It requires `chain.tip == Some(*span.chain())` before records are considered usable. The equality is a cheap tripwire that the generic retained verifier and WAL semantic verifier saw the same final commit. It must never call the page-record decoder before the chain is valid.

`copy_verified_prefix` writes only `[0..span.end]` into a fresh `SharedBuf`; `SharedBuf` currently only has `try_new`, `copy_range`, and `read_exact` (`db/📝️wal/🦀️.rs:864-933`), so add a bounded page-fragment copy helper rather than recreating a `Vec`. Construct `SprWriter::resume_verified(buf.clone(), span)` only after that exact copy. Its position check is the final no-overwrite guard.

Add two narrowly named `SegmentWriter` constructors rather than teaching `begin` to hide lifecycle states:

```rust
async fn resume_existing_verified(/* buf + span */) -> Result<SegmentWriter, DbError>;
async fn initialize_existing_empty(/* already-created index + exact prior */) -> Result<SegmentWriter, DbError>;
```

The second must **not** call `create_segment`; it writes a fresh SPR header only after an active segment has been truncated to zero, then writes and Fsyncs its `WAL_SEGMENT_HEADER` commit. `SegmentWriter::begin` remains the fresh-successor constructor that calls `create_segment` (`1540-1551`).

## Exact open pseudocode

This runs while the caller retains exclusive document/write authority. `WalStorage::segment_state` is observation only, not that authority.

```text
indices = storage.list_segments(document)
if indices is empty:
    close indices; return ArtifactWal::create(...)

first = indices[0]; last = indices.last()
require each index == first.checked_add(ordinal), else Corrupt
prior = if first == 0 { Genesis } else { RetainedBoundary }
next_tx_id = 1
last_clean = None

for index in indices ascending:
    state = storage.segment_state(document, index)
    require index == last || state == Sealed, else Corrupt
    len = storage.segment_len(document, index)
    pages = storage.read(document, index, 0..len)

    outcome = scan_retained_pages(pages, control)

    if index != last or state == Sealed:
        require outcome is span && span.end == len && span.sequence > 0
        tip = validate_wal_prefix(pages, span, index, prior, document, control)
        observe decoded verified WAL records only now; next_tx_id = checked_max_successor(...)
        close pages; prior = Verified(tip); continue

    # sole active highest segment
    resolve_active_highest(outcome, pages, len, index, prior)
```

`checked_max_successor` must use `tx_id.checked_add(1)` for every observed `TxBegin`, `TxCommit`, and `TxAbort`, then take the maximum. The current `saturating_add` at `1703-1705` can leave `next_tx_id == u64::MAX`, and `submit` currently increments unchecked at `1738-1739`; neither can safely allocate a post-maximum id.

`resolve_active_highest` has three safe branches:

```text
A. scan reports a valid span with sequence > 0:
   tip = validate_wal_prefix(... span.end ...)
   observe verified records and tx ids
   if span.end < len: storage.truncate_tail(index, span.end); storage.sync(index, Fsync)
   buf = copy_verified_prefix(pages, span.end)
   close pages
   active = SegmentWriter::resume_existing_verified(index, buf, span)
   # prefix bytes are unchanged; no header, commit, or append is emitted here

B. scan reports a valid span with sequence == 0:
   require span.end == HEADER_SIZE
   require `prior` reveals an exact `None` (genesis) or `Some(tip)`; see compacted caveat below
   if span.end < len: storage.truncate_tail(index, HEADER_SIZE); storage.sync(index, Fsync)
   buf = copy_verified_prefix(pages, HEADER_SIZE)
   close pages
   active = resume_existing_verified(index, buf, span)
   active.append_record(SegmentHeader { document, index, prev: exact_prior })
   active.commit_and_flush(storage, Fsync)

C. active physical length < HEADER_SIZE and scanner says Header:
   require exact prior is known
   storage.truncate_tail(index, 0); storage.sync(index, Fsync)
   close pages
   active = SegmentWriter::initialize_existing_empty(index, exact_prior, now_ms)

Any other scanner diagnostic, a semantic-chain failure, or a complete invalid header/frame/commit:
   close pages; return Corrupt without a storage mutation.
```

Branch B preserves the valid generic SPR header byte-for-byte and removes only non-committed bytes. Branch C is the one exception: no complete SPR header exists, therefore no committed WAL data can exist and zero is the only safe trusted prefix. Both branches make the initial segment-header commit durable before returning. If tail truncation succeeds but a later in-memory allocation/copy fails, the stored prefix remains a valid clean WAL and the next open can resume it.

After a clean active branch, set `next_segment_index = last.checked_add(1)` and return. On a sealed highest branch, first validate it exactly as a sealed segment; then calculate `successor = last.checked_add(1)` **before mutating**, call `SegmentWriter::begin(document, successor, Some(validated_tip), now_ms)`, and set the next index checked. Never append or truncate a sealed highest segment. A crash after `seal` but before successor creation is repaired by this branch; a crash after successor creation but before its first header commit is repaired by B/C on that active successor.

If the existing `u64` `next_segment_index` field cannot represent the post-successor value, return a checked limit error before creating the successor (or model it as `Option<u64>`); do not wrap. The current rotate also increments unchecked (`1776-1778`).

## Lifecycle decision matrix

| Physical state | Required result |
|---|---|
| No segment rows | Close list owner, create segment 0 normally. |
| Missing index inside list | `Corrupt`; dense sequence is already required by `WalReplayCursor::open_segment` (`1403-1421`). Do not fill a gap. |
| Active length 0 / active partial SPR header (`len < 32`) | Only when the exact predecessor tip is known: truncate to 0, sync, initialize the existing segment. This covers crash after `create_segment` before its genesis/successor header write. |
| Active valid SPR header, zero generic commits | Preserve exactly the 32-byte header, truncate only any uncommitted tail, resume its sequence-0 span, append+commit the WAL segment header using the validated predecessor. |
| Active clean committed prefix | Validate generic span plus `WalSegmentChain`; copy exactly `span.end`, `resume_verified`, no storage write. |
| Active torn/uncommitted tail after valid commit | Validate prefix, truncate only `[span.end..len)`, sync, then exact resume. The torn transaction is never decoded/published. |
| Earlier sealed segment or sealed highest with any tail, zero commit, partial header, invalid frame, or wrong semantic chain | `Corrupt`, no truncate/rewrite. This extends the intent of the existing torn non-active law (`2159-2177`) to the sealed-highest rotation gap. |
| Sealed highest, clean | Validate it, leave bytes immutable, create and commit a successor chained to its validated tip. |
| Retained compacted suffix, clean first index > 0 | Start with `WalPriorChainTip::RetainedBoundary`, which requires the preexisting first header to carry some predecessor hash, then verify all retained successors normally (`1357-1362`, `1415-1420`). |
| Compacted suffix whose first retained/highest active segment has no committed header | Refuse without mutation unless open is given a trusted compaction anchor containing the exact preceding tip. `RetainedBoundary` knows only “some hash,” not the value, so fabricating a new `prev_chain_hash` would sever provenance. |

## Test additions and current gaps

The current tests establish useful pieces but do not catch replacement/reopen loss:

- `torn_tail_is_recovered...` checks replay and a clean final parser result, but not byte identity of the verified prefix (`2072-2094`). Change it to retain `before[..span.end]` and assert the stored bytes after open are exactly equal.
- `recovery_resumes_next_tx_id...` covers ordinary ids (`2097-2119`), but needs a max-id denial law to ensure no saturation/wrap.
- rotation/replay test establishes the successor previous hash in a running process (`2121-2156`), but needs a restart test after manually sealing the highest segment. Assert the old segment is byte-identical/sealed and the new header carries the old validated tip.
- the existing sealed old-segment corruption law (`2159-2177`) should use the new state distinction and add sealed-highest corruption.
- add direct Memory-storage fixtures for: pre-created empty 0; 1–31-byte partial header; valid 32-byte zero-commit header; active clean exact-prefix preservation; active torn exact-prefix preservation; and crash after successor `create_segment` before header commit.
- compacted suffix: create a real multi-segment chain, delete the sealed prefix only, then prove a clean retained active suffix resumes and preserves bytes. Add the no-commit first-retained case as a refusal law unless/ until a trusted compaction anchor reaches this API.

`FaultStorage` currently injects failures/torn writes only on `append` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:331-388`). `CrashHarness` deliberately starts at append #2 and explicitly excludes a failed genesis header write (`517-533`). It cannot prove create/seal/successor lifecycle recovery. Keep its existing append-prefix law, but add the above direct lifecycle fixtures; a future fault script that targets create/seal/truncate can make those exhaustive.

## Scope

Read-only audit; no product source, fixtures, runtime, or build command was changed. This algorithm assumes the pending `WalStorage::segment_state` seam and the already-added `SprWriter::resume_verified` constructor, but does not claim either is compiled in this snapshot.
