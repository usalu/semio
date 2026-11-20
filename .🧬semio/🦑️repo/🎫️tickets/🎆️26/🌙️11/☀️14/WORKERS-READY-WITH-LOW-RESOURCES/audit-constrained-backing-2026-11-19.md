# Constrained Run Backing Audit — 2026-11-19

Read-only inspection; no collection, detach, compaction, deep scan, staging, worker or external access. The previous 20:40 coordinator ledger reported 56.46 GiB allocated versus 39.91 GiB used, zero eviction against a 198.31 GiB budget and approximately 40 GiB host free (`audit-current-constrained-lab.md`). That aggregate 16.55 GiB difference is not an executable reclamation guarantee.

## Current image measurements

A fresh metadata-only `stat`/`statfs` sample of declared `.cache/b` fixed images, with no traversal or lease probe, gave the following GiB. Used means filesystem blocks used inside the mounted image; allocated means host blocks of `fixed.dmg`. Values change concurrently. Slack estimate subtracts the image-slack policy’s 64 MiB filesystem allowance. Mounts were present for all rows; this sample does not establish that any image is idle or that its exclusive lease is obtainable.

| Partition | Host allocated | Inside used | Estimated slack |
|---|---:|---:|---:|
| rust | 22.573 | 10.489 | 12.021 |
| graph | 9.058 | 5.439 | 3.557 |
| dependencies | 13.778 | 13.384 | 0.332 |
| tools | 3.527 | 3.255 | 0.210 |
| nx | 0.810 | 0.736 | 0.012 |
| products | 0.059 | 0.058 | 0 |
| diagnostics | 0.063 | 0.008 | 0 |
| leases | 0.062 | 0.005 | 0 |

These estimates total about 16.13 GiB and are not a measured return of physical blocks. Rust and graph account for approximately 15.58 GiB. The older `result-R172.md` measurements are historical and its conclusion that raw images could not be compacted predates the current in-place UNMAP implementation; do not use its 43.8 GiB replica-store plan as a current reclaimable balance or add it to cache slack. That plan required a deep report and a server-wide action and was not remeasured here.

## Current command and eligibility

`repo cache gc --help` describes area eviction and returning idle image blocks. Current `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts:203` first runs ledger GC. An unfiltered successful GC then tries `rust`, `dependencies`, `graph`, `products`, `tools`, `diagnostics` and `nx`; `--area` bypasses this image loop. Dry-run merely reports the bloat predicate, not lease availability or the actual bytes returned.

`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts:160` checks the platform, no retirement pending, current bloat and a non-waiting exclusive `native-fixed-image` lease. If admitted it detaches, attaches through an UNMAP-capable stack, waits for allocation to settle and detaches again; the next opener reattaches. It does not recreate image contents, but it cycles the mount and skips held images. Idle status cannot be inferred from statfs or the ledger; specifically the ongoing graph read makes assuming graph quiescence unjustified.

The current bloat predicate (`🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts:13`) requires allocated bytes at least 300% of inside used and unused allocation at least the policy floor. Below 48 GiB host free, the floor is reduced to one eighth: rust and most images 1 GiB, graph 0.25 GiB, products 0.75 GiB. On the above sample rust is 2.15 times used and graph 1.67 times; neither meets 3x. Diagnostics meets the ratio but has only about 0.055 GiB before overhead, below its 1 GiB floor. The others also fail the ratio or floor. Thus this sample identifies **no image due** under the public GC reclaim predicate, regardless of lease availability. The separate disk-recovery image-slack policy has a 1 GiB / 10% selection threshold, but its in-place reclaim calls the same predicate; elastic retirement is a distinct fallback for graph, products and rust, requires exclusive quiescence, and discards the recomputable image. It is not evidence that an idle image can be reclaimed nondisruptively now.

## Admission implication

At about 40 GiB free, preserving the documented 12 GiB recovery threshold leaves about 28 GiB before ongoing writes and image overhead, below the lab’s 40 GiB free-space preflight and without a reserved backing budget. Even the entire estimated 16.13 GiB slack, if all could be returned, would yield only about 44.13 GiB above that threshold before ongoing writes; actual reclaimability and timing are unproven and the current public reclaim predicate selects none. This evidence does not establish a nondisruptive route to close the constrained-run gap while the coordinator is active. CPU and memory reservation remain separate requirements. No GC, reclaim or lab was run.
