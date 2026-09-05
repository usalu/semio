# WAL Tail-Only Recovery

## Current Result

The registered source gate is GREEN39: an AJV-validated neutral corpus contains 18 exact physical cuts, 17 lifecycle cases and four independently CRC-checked aligned/misaligned multi-page copies. Its independent CRC-32C/LEB128/hash-chain grammar reproduces the three committed ends (129, 258, 387) and every cut's trusted/repaired boundary and next transaction id. It was first observed RED on the old opener's `delete_segment` call, after the independent oracle had passed. Scoped staged and unstaged whitespace checks passed.

Native receipt `wal-recovery-exact/exact-cargo-laws-iNHATx/00` built in 2m07 and passed both root laws. The third law failed before cancellation while preparing a multi-page WAL: `SharedBuf::copy_range` incorrectly required a source fragment to fit in one destination page remainder. Both that loop and prefix-copy are corrected and backed by four neutral CRC cases.

The execution agent reran the expanded gate as `wal-recovery-exact/exact-cargo-laws-BuR4F8/00`: all five exact native laws PASS. These cover the corrected root prefix/lifecycle cases, cancellation-stays-set replay close, 18 repeated WAL open/close cycles beyond the 16-writer page-pool ratio, and rejection of normal close with pending writes or later use of a closed handle. The executable SHA-256 is `dd5345f6881e1b55d296a6bbef6200879e8cad108e4ef43713a2d64b6f72084f`.

## Implementation

The opener no longer deletes or rewrites retained committed bytes. It checks dense segment indices, explicit active/sealed state, full retained SPR framing/CRC/commit/hash chains, and the stronger WAL document/index/predecessor semantics before any active-tail mutation. It decodes verified records to restore the maximum checked transaction successor across all retained segments. Optional WAL header flags are denied.

Only the active highest segment may lose an uncommitted suffix. The original verified prefix is copied pagewise into the buffered writer, then consumed by `SprWriter::resume_verified` with `flushed_len` at the existing end and no pending records. A clean reopen performs no storage append/commit. Repair truncation is followed by Fsync. A valid sequence-zero header is retained and gains a fresh WAL header commit; an exact partial profile header is initialized without deleting or recreating its segment. The highest sealed segment remains immutable and gains a checked-index successor. Empty or sequence-zero compacted boundaries are refused because no trusted predecessor hash is available.

Transaction and segment successor arithmetic is checked before submission/rotation mutations. Read-page and list owners are drained rather than taking only one close step; error-side buffer retirement is explicit around copy, resume and repair.

## Remaining Ownership and Integration Work

`open_with_control` currently bounds scan/copy progress and checks cancellation before storage work, with uninterrupted truncate/Fsync repair epochs and ungated cleanup. It is still a one-shot operation, not the resumable owner cursor requested by the ownership audit. The existing convenience `open` still supplies a hidden default control. Replace these with a single retained open cursor and migrate callers, then wire the now-tested `ArtifactWal` terminal-close lifecycle through document authority shutdown. Do not claim cancellation can interrupt an in-flight backend I/O future.

The artifact and sync consumers separately expose commands before checking TxCommit and ignore TxAbort. Their transaction-atomic replay repair is not part of this byte-preservation claim. The durable three-member Map journal and full end-to-end system are still unfinished.

Related read-only decisions: `📓️terra-wal-open-no-data-loss-algorithm-p0.md` and `📓️terra-wal-controlled-open-ownership-p0.md`.

## Subsequent Expansion

After the five-law receipt, root added a no-mutation invalid-partial-header lifecycle case and the execution agent added short/failed append, failed sync, and failed post-seal successor cases. Source is GREEN44. The expanded exact native receipt `wal-recovery-exact/exact-cargo-laws-pQEcDQ/00` is GREEN18, including eight existing group-commit/rotation/recovery regressions. Its executable SHA-256 is `a3f2bc3d24dc99467d9f1cfbcd4ec9f2289fddd563a2dacefd54b196fafaccdb`, also used by the passed capacity law. The earlier torn-tail regression used a complete CRC-invalid frame while intending to simulate a truncated write; its input is now genuinely partial, and it additionally asserts exact original byte preservation. Complete invalid CRCs remain hard failures in the neutral corpus.
