# R129: where the memory of a farm child goes, and what a child keeps after an owner (2026-11-17)

Scripts: `T/🧰️scripts/R129/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `measure.ts` + `run.sh` (one child of my own with the real farm environment, the real service `open`/`run`, a sequence of real owners; RSS, footprint, heapUsed, external, arrayBuffers, `bun:jsc` type counts, bounded caches, observation collections, optional `vmmap`), `pool-run.ts` + `pool-run.sh` (the real pool and the real worker process over a sequence, with `stats()`), `trace-ts.ts`, `overlay.ts` + `run-staged.sh` + `stage-tests.mjs` (run any test against staged files), `battery.sh`, `compare.py`, `land.mjs`, `restore.mjs`, `window.sh`, `at.sh` (landing). Owners come from a copy of a real wave (`.cache/f/owner-pool-waves`, 12 units cut to `slim/`), env of the child exactly as the live children carry it (read from `ps eww` of a live child, no secrets printed): `BUN_JSC_forceRAMSize` 1152 MB, `MIMALLOC_PURGE_DELAY=0`, `BUN_JSC_numberOfGCMarkers=1`, `BUN_JSC_useConcurrentGC=0`, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. The host ran at load 25 to 56 the whole time (swapping and compression), so seconds and single footprint readings are noisy; read the direction.

## 1. Where a child's memory goes (measured)

Idle child right after start (module graph and session loaded, `service.open` done): **RSS 122 MB, physical footprint 40 MB** (peak 48 MB), managed memory (heap + external) 29 MB. The TypeScript compiler is NOT loaded at start (`require.cache` holds no typescript until the first owner); it is loaded during the first owner of any kind (also a Rust crate), 66 MB RSS against 19 MB for a bare process when loaded alone, +8 MB heap, +17 MB when it preprocesses a large file. Not a top holder; I did not find the one caller (`nxBuildEngineReferencesV1` in `📚️library/🟨️.mjs` creates the require; not confirmed as the one that runs).

A child that served one Rust owner (`norm-en1991-rs`, 10 owners in a row, same child, before the change), all MB:

| holder | MB | evidence |
|---|---|---|
| resident set (what `ps` shows, what the recycle rule read) | 885 to 1593 (1.25 to 2.5 GB in earlier runs) | grows by 100 to 400 MB per Rust owner and never falls back |
| of that: freed pages the allocator keeps and the host has not taken back (clean resident pages of the mimalloc heap, `vmmap` tag 1: 772 MB resident, 192 MB dirty after a TypeScript owner) | 50 to 70 percent of RSS | not in the footprint; `Bun.gc(true)` and `Bun.shrink()` after the owner change RSS by 0 to +40 MB; `MIMALLOC_PURGE_DELAY=0` is already set |
| physical footprint after the owner | 530 to 647 (96 to 300 MB when the host compresses the child) | `footprint -p`, peak 1003 MB over 10 owners |
| managed memory after the owner (heap + external) | 323 after owner 1, 483 after owner 10 (heap 334 + external 149) | idle 29: **about 300 MB retained from the first Rust owner, 20 to 40 MB per later owner** |
| the 14 named bounded caches (`boundedCacheStatsV1`) | 1 to 4 (closure entries 3 of 96, sidecars 0) | the 96 MiB closure cache is NOT what a child holds; it never filled |
| observation row table and row pool (`observationCollectionsV1`) | 13.5 + 5.5 | 43 000 rows interned, 36 000 pooled |
| rest of the retained JS (strings 1.3 M, objects 565 K, arrays 215 K, WeakRef 306 K that grows 60 K per Rust owner, closures) | about 250 MB | not in any named cache; `releaseInferenceMemoryV1()` + `shrinkBoundedCachesV1(0)` drops it (below) |
| native: JIT, module code, allocator fragmentation | about 190 to 260 | footprint minus managed minus idle |

Peak of the first Rust owner: footprint 700 to 930 MB (transient intake documents); a TypeScript owner peaks at 300 MB. A lower collector target `BUN_JSC_forceRAMSize` 384 MB (the pool sets 75 percent of the reservation, 1152 MB at 1536 MB) cut the peak footprint of two Rust owners from 894 to 586 MB (-34 percent), CPU of the first owner +11 percent (31.5 s to 35 s); not landed (one noisy pair of runs).

So the numbers in the task (children at 498 to 1772 MB, 3 to 6 percent CPU) are RSS: freed pages the host has not taken back. The physical need of a child is 40 MB idle, 300 to 650 MB working, up to 930 MB at the peak of a cold Rust owner.

## 2. What caused the recycling and what was changed

1. **The pool recycled children by RSS** (`recycleReasonV1` read `message.rssMegabytes` of the reply), so a child with 300 MB of footprint and 2.1 GB of freed-but-resident pages was replaced "to return memory" after one to three owners, each replacement paying the cold boot again (7 to 10 s of module loading, then a cold JIT). Real pool, 8 owners, reservation 1536 MB, old code: 1 recycle at the 7th owner (`reason memory`, 2076 MB RSS at a footprint peak of 807 MB), 2 starts. Staged code: 0 recycles, 1 start, peak RSS 1873 MB.
   Fix (`🏊️pool/🟨️.mjs`): after a derivation (weight at least 1), after the first reply of a child (its baseline) and whenever the RSS passes the ceiling, the parent reads the physical footprint of the child (`footprintOf`, macOS `footprint`, Linux `smaps_rollup`, 40 to 80 ms measured at load 40) and judges with that; the result is settled after the judgement, the slot is `settling` meanwhile (neither idle nor busy), `close()` waits for the judgements; no pid or an unreadable footprint falls back to the RSS. Warm jobs (weight under 1) read nothing. The parameter of `recycleReasonV1` is now `heldMegabytes` (fixtures and the independent Python oracle renamed with it).
2. **Nothing retained beyond a budget** (`👷️service/🟨️.mjs`, `👷️worker/🟨️.mjs`, pool schema): the parent tells the child its reservation (`Start.megabytes`, schema first); the child's budget is `retainedBasisPoints` (10 percent) of it, at least `retainedMinimumMegabytes` (64 MB). After each job the worker already collected once (`Bun.gc(true)` per owner, unchanged); `trimRetainedV1()` then reads heap + external and, above the budget, calls `releaseInferenceMemoryV1()` (interned rows, verified blocks, images, publications, settled reads, registry rows) and `shrinkBoundedCachesV1(0)` and collects once more. No new forced collection when under budget.
3. Measured on the same child: managed memory after two Rust owners 290 MB (no trim) -> 152 MB (trim), RSS 1613 -> 1048 MB.

## 3. Proof

- `🧪️tests/🦶️footprint-recycle` (3 tests; against the base tree 2 fail, against the staged tree 3 pass): a child with 2000 MB resident and 300 MB footprint keeps serving; a child whose footprint passes the budget is replaced after the owner that grew it; an unreadable footprint falls back to the resident set. The existing recycle laws (`♻️retirement`, `🔱️warm-jobs`, the model comparison in `🧪️tests/🟦️.ts` with the Python oracle) pass with the renamed parameter.
- `🧪️tests/🧽️retained` (2 tests, a real child process: the budget formula; with no budget the child keeps 32 -> 81 MB of managed memory over 6 owners of 60 000 rows and the interned rows grow, with the budget it keeps 10 MB after every owner and zero interned rows; against the base tree it fails on the missing export, against the staged tree it passes). The bound is measured with `process.memoryUsage()` in the child after the normal per-owner collection.
- Battery against the unchanged tree (26 files of the farm, the memory gate and the cooperative farm): none worse except the flaky `🛑️hold` overlap count (1 failure in 6 staged runs, 0 in 12 base runs; the test counts windows that overlap by more than 25 ms, a job stretched by host load overlaps its two neighbours: 3 instead of 2 without three jobs running).

## 4. Before and after, same 10 owners (`norm-en1991-rs, norm-din18599, cad-cad-rs, energy-oracle-py, forms-forms-rs, norm-en1990, bim-model-rs, ui-styling-py, reasoning-wires, norm-en1997`; one child; both runs together at load 40)

| after owner | RSS before | RSS after | footprint before | footprint after | heap+external before | heap+external after |
|---|---|---|---|---|---|---|
| (idle) | 123 | 110 | 40 | 43 | 42 | 27 |
| norm-en1991-rs | 885 | 886 | 544 | 541 | 323 | 276 |
| cad-cad-rs | 1215 | 1309 | 624 | 559 | 280 | 315 |
| forms-forms-rs | 1428 | 1483 | 546 | 468 | 318 | 203 |
| bim-model-rs | 1552 | 1524 | 538 | 450 | 349 | 246 |
| norm-en1997 (10th) | 1593 | 1981 | 647 | 476 | 483 | 289 |
| peak footprint of the run | 1003 | 934 | | | | |

Idle is under 150 MB (it was already: 122 MB RSS, 40 MB footprint). A working child holds 450 to 560 MB of footprint after the change (530 to 647 before) and its managed memory stops growing (289 MB after ten owners against 483). The first cold Rust owner still peaks near 930 MB; 500 MB is reached after the owner, not at its peak. The RSS column is not a target any more (it is freed pages).

R125 price after: `askWorkerMegabytesV1` books the 75th percentile of the footprints children report plus 15 percent (floor 768 MB): from the per-owner footprints of the 8-owner pool runs (old: 398, 334, 503, 473, 438, 684, 807, 194; new: 267, 101, 89, 90, 90, 1006, 1107, 920, the last three after a 327 s owner that fragmented the heap of a child that was no longer recycled) the 75th percentile is about 550 MB (old) against 940 MB (new) -> price 768 MB (floor) against about 1080 MB, so on this sequence the price did not fall (the 327 s owner ran at load 50; the same owner took 9 s in the in-process run and left 450 MB, so read this as a risk, not a measurement of the fleet): a child that is no longer recycled at 2 GB RSS keeps its fragmentation up to the reservation. Where the recycling by RSS happened to cap a fragmented child, the footprint rule lets it live until `max(reservation, 1.5 x its first footprint)`. To lower the price, the next step is a retained-footprint recycle (replace a child whose footprint after a job and after the trim stays above its baseline by more than X) priced against one boot of 7 to 10 s; not done, not measured.

## 5. Landing and what is not verified

Landed 2026-11-17 13:30:00 to 13:32:22 UTC (everything above) and 13:33:24 to 13:34:51 UTC (the fixture schema of the farm tests still named the old key: one law was red for 2.5 minutes), lock `landing.lock` owner claude-R129, harness tests green under the lock (`tests/🟦️.ts` 30 pass 0 fail after the follow-up); attempts at 12:30 and 13:00 lost the lock to R130 and R125. Entry in `landings.md`. The live reads and farm workers run old code until their parents restart. Process note: at 11:51 UTC I edited the live `🏊️pool/🟨️.mjs` for about 3 minutes before I had the lock (a slip: the scratch copy was meant); I restored the exact original text (27 699 bytes, same size, no peer wrote in between: mtime check) and moved the new test file out of the tree; nothing ran against it.
- NOT verified: the effect on a real read of 473 owners (the live reads run old code); a Linux child (`smaps_rollup` path of `footprintOf` is existing code, the MADV_FREE behaviour of the allocator differs there); the 8 GB machine case (no 8 GB host here); the per-owner price in the fleet after the change; the CPU cost of re-deriving rows after a trim (not visible in 10 owners: 36.6 vs 37 s, 4.8 vs 10.4 s for the second Rust owner, load noise); the `forceRAMSize` reduction; which module loads TypeScript for a Rust owner; the Windows path of the footprint read.
