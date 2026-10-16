# Rust Scan Batch Independent Current Review

2026-10-15. Source-only review after the latest qualified bounded-metadata appendix in `rust-historical-retirement-timing-current.md`. Inspected metadata helper/schema, CacheWalk and both changed loops, measurement recorder and actual retirement execution. No runtime, canonical measurement, deletion, SDK operation or production edits were performed. Reported BOTH gates/peaks and strict outcome were not independently rerun.

## Findings

No actionable defect found in the inspected bounded-eight epoch.

The helper slices inputs into schema-fixed eight-item batches, wraps each read in a promise continuation (including synchronous throws), and awaits Promise.allSettled for the complete accepted batch before either failure selection or reduction. Rejected results are examined in input order; throwing result.reason retains Error identity and undefined without truthiness tests. Reduction is input ordered and individually awaited. Any read or reducer rejection exits before the next batch. Generic helper cancellation is expressed by the actual read/reducer rejection; it does not invent its own signal or authority.

CacheWalk's actual read admissions check the genuine control before entering its continuation barrier and after awaiting it, and after the filesystem operation. Cancellation during accepted work therefore cannot release that batch before siblings settle. Already-scheduled batch callbacks may reach the checkpoint and refuse without a physical syscall; no next batch starts after the resulting failure. Both changed production loops use the same helper and do not overlap independent helper invocations: discovery child stats are bounded, measurement batches are bounded, and directory enumeration occurs only after those readers settle during ordered reduction. Actual metadata concurrency is therefore at most eight in the inspected walker path, excluding the separately awaited continuation action.

The barrier serializes every 64th checkpoint. It installs the pending promise before executing publication; later admissions wait for it and then count their own checkpoint. A failed publication/continuation sets a permanent explicit failed flag and exact stored reason, including undefined. Pending followers reject with that reason and later callers cannot restart the barrier as a new successful epoch. No fail-fast metadata Promise.all was added.

## Coverage, Reduction And Custody

Discovery preserves each child-name classification, symlink exclusion, named-directory stopping rule, depth rule and pending directory stack insertion order. It does not skip opposite-kind subtrees. Measurement selects pending paths in pop order per batch, then applies the original file-kind, byte sum and maximum timestamp/source tie reduction in that input order. Batched pending selection can visit already-pending siblings before newly discovered descendants; the complete stable-tree coverage and deterministic global maximum/tie result remain the same. It is not an atomic filesystem snapshot, nor a guarantee of identical temporal observation order under concurrent external mutation.

Unit/session provenance retention still occurs after each completed ordered unit/session measurement, with the same budget and deterministic source selection. Stale/working session classification and protected working rows remain. No file recency is replaced by directory timestamps. The diagnostic maximumInFlight counter brackets actual metadata operations after checkpoints; cumulative operation milliseconds can overlap, while stage and total wall times remain elapsed measurements.

Retirement continues to create independent initial and recensus scanner calls. Actual physical profile locks and full rooted descriptor assertions precede recensus; selected rows must still match bytes/recency and be unheld. Deletion still checks guard ownership and the unchanged byte ceiling per row. Preview returns before that deletion boundary. This diagnostic/batch Source grants no permission to delete or bypass the retained image/common-writer/charge/proof guards.

## Limits

The source review does not establish a canonical speedup, a two-census budget fit or a complete runtime result. Existing readdir allocation and traversal bounds are unchanged separate concerns. The supplied temporary-tree/oracle and cancellation gates are meaningful evidence as reported by their owner; this audit did not launch them. Root owns any separately authorized nondeleting canonical measurement.
