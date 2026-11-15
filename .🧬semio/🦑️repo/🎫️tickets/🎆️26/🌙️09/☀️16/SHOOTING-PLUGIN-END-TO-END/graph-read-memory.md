# Graph read memory: why the inference stage grew past 8 GB

## Symptom
`repo show project ...` on a 16 GB host ended with exit 75: "the plan stage inference grew to 8212 MB (8770 MB), more than the 6553 MB this machine gives a plan".

## Measurement (before)
- `SEMIO_GRAPH_PHASES=1 repo show projects` plus a process sampler (`generated/tree.sh`): the loader (`bun --eval`, parent of all owners) alone reached 9190 MB resident, 6.3 GB within 25 s in an earlier peer read; no farm worker was running (the host had no headroom, so owners ran one at a time in the parent).
- Per-owner learned peaks (`owner-peaks.json`): 1.6 to 5.3 GB per owner for every kind (typescript, rust, cargo), while the stored record of an owner is about 10 KB. So the growth was not the product of the owner but memory it pinned.
- Forced-collection probes (`Bun.gc(true)` between steps) showed the live heap grow monotonically across owners (437, 737, 809, 2034 MB) although the caches that `releaseOwnerStateV1` clears were nearly empty at each release.
- A heap snapshot at 800 MB live heap: 2.85 M strings = 624 MB, of which 533 MB were redundant copies (2.45 M duplicates of 400 k distinct strings). 1.0 M `{kind,fact,path}` rows with their own absolute-path strings; 97 % of them were reachable as `bundle.layers` of the in-process closure bundle cache.

## Root cause
`closureTravelV1.export` remembers up to 512 portable closure bundles (`closureBundlesV1`, used to export project records to the fleet). `store.bundle(id)` parsed every shared layer with a plain `JSON.parse`, so each bundle held its own copy of every row object and path string, although the rows of one layer are shared by thousands of entries. The cache charged 256 bytes per row (a row really costs 600 bytes or more as an unshared copy) against a 128 MiB budget, so the budget admitted several times its real size, and nothing evicted it before the process tree passed the plan ceiling.

## Fix (schema first, no legacy)
- `store.bundle` hands the parsed layers through `shareObservationRowsV1` (the weak row pool already used by `load`), so a row met by several bundles or by a live entry is one object.
- `closureBundlesV1` is charged like the entries and layers it holds (`closureEntryBaseBytes`, `closureEntryRowBytes` per row, `closureLayerRowBytes` per layer row) and bounded by the new declared limit `closureBundleCacheBytes` (32 MiB) in `inference/schema` (en and de descriptions).
- Test: `inference/tests/bundle-rows` (three entries sharing layers; the independent Python model counts held versus distinct rows; the shared store must return one live object per distinct row; fails without the fix, passes with it). Run: `repo tools exec bun -- test ./<file>`.

## Measurement (after, same owners, same moment of the same read)
| owner | live heap (collected) before | after | rss before | after |
|---|---|---|---|---|
| wfc-grid2d | 713 MB | 348 MB | 3868 MB | 2317 MB |
| gis-js / vcs-js | 727 MB | 358 MB | 3731 MB | 2318 MB |
| heap snapshot strings | 624 MB (533 MB duplicates) | 211 MB (90 MB duplicates) | | |

## End-to-end result (honest)
`repo show project @semio-tech/shooting-js` was NOT verified: after the fix a cold read (empty generation, 4500 owners, host shared with other agents) still ran 11 minutes without finishing and its process tree peaked at about 8.0 GB resident (before: 9.2 GB loader alone, 8.2 to 8.8 GB in the refused runs), with owners still stopped as too heavy (`mit-bestand-demonstrator` 3949 MB, `wfc-2d` 3042 MB, `flow-extension-brep-rust` 4403 MB). The duplicate-row defect is fixed and measured (about half of the live heap and 1.5 GB of resident memory at equal progress), but it is not the whole growth: the per-target closure walks below remain the dominant source and need their own fix.

## Not fixed here (open)
- Single owners still show a transient, real growth of 300 to 400 MB per target inside `projectWithDefaults` (command closure walks `targetClosureSome`, `nativeCommandInputs`): the live heap after collection climbs from about 0.35 GB to 1.6 GB over the 10 targets of one cargo/TypeScript owner and the owner is stopped as "too heavy" at 1756 MB. The snapshot accounts for only a third of it (the rest is JSC external memory: strings, buffers). Another agent is changing `releaseOwnerStateV1` (`sharedBytes`, shrink interval) in the same area at the same time.
- Tried and discarded: raising `rowStoreRows`, `rowStoreCharacters` and `maximumInternedCharacters` (no effect on the snapshot; reverted).
- Debug instrumentation used for the profile (heap preload, per-step collection, snapshot writer) was removed from all sources.

## Continuation: per-target growth in projectWithDefaults (not resolved)
Measured with temporary `[DEBUG]` probes (removed; `grep` for `[DEBUG]`, `dbgV1`, `dbgProbeV1` in the touched sources is empty):
- Mid-owner (at 900+ MB collected heap) the probe cleared, one step at a time with a full collection after each: every `scripts` cache (files, closures, commandDependencies, declared, paths, walkNodes, toolGroups, modules, reachState), `closureEntriesV1`, the walk-fact caches, `closureBundlesV1`, in-flight/failure maps, and the observation maps (settledReads, fileRecordImages, verifiedBlocks, sourceAnalysisImages, projectImages, fieldDigests, record stores, blocks, registry owners). Together they released only about 70 MB of 1250 MB heap (+1040 MB external). So the per-target growth is NOT memoized closures or path strings held by these caches; those are already released or tiny. The live mass sits in something I could not name from the snapshot (it shows about 470 MB of objects; the rest is JSC external memory).
- Resident versus managed: the front-door loader runs without `BUN_JSC_forceRAMSize`, the slice-pool worker sets 512 MB. At the same live heap (1.2 GB heap + 1.0 GB external) resident memory was 6.0 GB without the cap, 6.4 GB with `MIMALLOC_PURGE_DELAY=0` alone, and 2.8 GB with the cap. The cap does not change the "grew by" owner figures (cargo/rust owners still report 3.0 to 3.4 GB growth), only how much freed memory stays resident.
- Cold `repo show project @semio-tech/shooting-js` with the cap in the environment (`BUN_JSC_forceRAMSize=536870912 MIMALLOC_PURGE_DELAY=0`): the loader peaked at 5170 MB resident (uncapped runs: 6.5 to 9.2 GB), under the 6553 MB ceiling, but the run was still in `command-source-imports` after the hard 1500 s timeout and was killed. It did NOT complete. No code change enforces the cap on the front-door path yet: that needs a decision (the sealed trampoline spawns every door; capping them all affects all commands), so I did not make it.
- `repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype --lang=en --output=stream` (once, hard timeout 1500 s): still "Reading the project graph slice" at 1435 s, killed by the timeout, exit code 124 (`generated/activate-final.txt`). It never reached the plan ceiling refusal (no exit 75), but also never finished.

Open: (1) name the holder of the roughly 2 GB live managed memory of the owner phase (try `Bun.generateHeapSnapshot("jsc")` or per-module `heapStats` deltas around `targetScriptClosure`); (2) decide the cap for the front-door loader; (3) a cold 4500-owner build needs more than 25 minutes on this saturated host, so completion should be verified on a warm generation.
