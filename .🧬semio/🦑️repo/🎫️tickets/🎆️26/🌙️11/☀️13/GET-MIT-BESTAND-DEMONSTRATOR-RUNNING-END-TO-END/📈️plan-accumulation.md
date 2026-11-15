# Plan accumulation over owners (activate-30 stop at ~900 of 1654)

Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.

## What the stop says
- `graph-plan-too-heavy` (`reason:"ceiling"`, 10155 MB tree of the plan worker against a 6553 MB ceiling) is the slice-pool guard on the process TREE (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, `judge`), not the managed-byte plan guard. The worker's own counters at that moment: heap 1620 MB, extra 1152 MB, 8.5M objects (strings 4.99M, Object 1.92M, Array 1.31M, Set 164K, WeakRef 80K, closures 76K). So about 2.8 GB is managed and about 7 GB of the tree is resident pages that the managed counters do not show (allocator retention after each rebuild/release cycle, decompression buffers, children). No ring was kept for the tree and the log has no periodic resident line, so the per-owner slope could only be bounded from the object types, not measured; the post-mortem cannot tell heap from page retention better than this.
- Object types point at row data, not the shared walk state: WeakRef 80K is the live row-table views, Set 164K is the reverse-import sets of the shared walk graph (bounded, released every 8 owners under pressure / 32 otherwise).

## Linear-in-owners components found (code)
1. Plan-wide capture (`👁️observation/🟨️.mjs`, `captureObservedSourceInputsV1`): a `Map` row-id to row OBJECT of every distinct row of every owner (strong references, so the weak row-table views never die). Fixed: with a `spill` row log the capture keeps a bit set over row-table numbers and the log ids (`SpilledCaptureV1`, 4 bytes per distinct row) and returns the log list as `observations`; `🕸️graph/🧭️discovery/🟦️.ts:155` passes its `rowLog`. The final `[...captured.observations]` materialises once at the end of the plan, after the owner memos are released.
2. `derivedFacts` (`🕸️graph/🧭️discovery/🟦️.ts:90,94`): one row object per derived fact for the whole plan (support already in the log). Small per owner, not changed.
3. The row log itself (`🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs`) keeps `byKey`/`keys` strings (full row keys) for every distinct row in memory: linear in distinct rows and the most likely remaining source of the 5M strings. Not changed: `keys()` hands the full key strings to consumers, so compacting them needs the consumer contract changed (digest keys with disk verification).

## Test
`👁️observation/🧪️tests/🗃️spilled-capture/🟦️.ts` (differential: spilled capture equals the in-memory capture row for row, counted, one log row each). `🕸️graph/🧪️tests/🪜️plan-stages` pattern updated for the new call.

## Not verified
Not run end to end: activate-31 started before the change and keeps the old code. If the tree still passes the ceiling, the next step is (3) and returning pages (the 7 GB gap), which needs a tree-resident ring in the slice-pool.

## Owner cost table immune to the thrash samples
`🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts` + schema: table `version` 2 (and the fleet state frame `costs:"2"`); `readOwnerCostFileV1` rejects a v1 file (empty table) and `mergeOwnerCostTablesV1` skips tables whose version is not 2 (fleet-merged v1 samples are ignored). Stopped samples recorded from now on are own growth by construction: the governor stops on growth net of the owner's credit (`🧠️memory/🟨️.mjs growthOf`) and records `takenBytes` net of it. Test in `💰️cost/🧪️tests/🟦️.ts`.
