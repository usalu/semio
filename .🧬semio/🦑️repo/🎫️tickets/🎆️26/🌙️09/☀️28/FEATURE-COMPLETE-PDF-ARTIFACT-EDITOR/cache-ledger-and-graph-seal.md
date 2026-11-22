# Cache Ledger Refusal and Graph Seal

## Diagnosis
- Refusal: "area caps leave less free space than the soft reclaim threshold of bazel-workspaces".
- Check (ledger `🟦️.ts`, `readCacheLedgerV1`): products capacity minus image overhead (256 MiB) minus explicit caps (256 MiB + share of the 24 GiB logical limit) must stay at or above the 3 GiB soft threshold.
- Cause: a policy inconsistency from an in-flight edit, not disk space (49 GiB free). A new 1000 bp `fleet-cache-index` sibling was added to `products` before the `bazel-actions` (4000 to 3300) and `semio-actions` (2000 to 1700) shares were reduced. Shares then summed to 9400 bp and left about 1.3 GiB.
- Now: shares sum to 8400 bp, leaving about 3.34 GiB (at least 3 GiB). `cacheLedgerV1()` validates (67 areas); no space reclaim was needed, no cache gc run, no sources touched.
- The validator already derives the check from the authored capacity, so no code change was needed.

## Regression test
Added to `ledger/🧪️tests/📏️needs/🟦️.ts`: the products shares leave at least the soft threshold, and raising `semio-actions` by 1000 bp is refused with the bazel-workspaces message. Not executed (no bun on this host); it needs `repo test` once the graph is sealed.

## Graph
`repo graph --graph-policy fresh --language en` passed the ledger check and is inferring 4822 projects (fleet cache unreachable, all inferred here, 8 in parallel). At 71 of 4822 after about 40 minutes (about 3 per minute, still speeding up) it is still running in the background; output in `🗑️generated/graph-try2.txt`. The seal is not yet written, so `repo show project` targets could not be listed.
