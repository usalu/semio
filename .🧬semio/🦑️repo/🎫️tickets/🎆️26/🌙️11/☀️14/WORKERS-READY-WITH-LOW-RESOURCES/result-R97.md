# R97: one owner of 4.4 to 5.3 GB in a worker process: the fleet intake of closure entries held twelve 37 MB entries unpacked at once

Scripts: `T/🧰️scripts/R97/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `warm-child.mjs` (the warm job of one owner in a fresh process, the way `👷️service` runs it), `sample.py` (footprint of the process tree from outside with macOS `footprint`, every second; no heap snapshot, no profiler in the numbers below), `overlay.ts` (runs staged files against the live tree), `land.py`/`restore.py`/`runall.sh`/`window.sh`, the two new tests, `lane.v4.ts`. Scratch copies of the closure store were APFS clones (`cp -c`), removed at the end.

## 1. What holds the memory (measured)

Owner `@semio-tech/stdio-ifc-rs`, warm job (`warmProjectRecordV1`: 18 closure recipes, the record is 7 KB / 150 KB unpacked), load average 19 to 30 on g1-server, fleet cache live, closure store empty or a clone of this machine's.

Steps ruled out by measurement:
- `ownerJobV1` (assembly authority, 494 cargo manifests): 4 to 8 s, 260 MB.
- reading the stored record: 150 KB.
- stale-entry checks (`closureEntryHoldsAsyncV1`, 117 checks): heap stays 270 to 560 MB.
- the TypeScript derivation of a stale entry: 10 to 16 derivations of `repo-mcp-rs` in 12 s at 840 MB with the fleet lane off (`SEMIO_CLOSURE_FLEET=off`).

The cause is `prefetchClosureEntriesV1` -> `lane.prefetch` (`graph-state/project-records/entry-lane`): for the recipes the store lacks it fetches **up to `fetchConcurrency` = 8 entries at once**, and `intake` of each entry unpacks ALL its documents (`unpackBundleV1(JSON.parse(...))`), localizes ALL of them (`localBundleV1`) and only then imports. One entry is 12 to 14 documents, 37 to 42 MB of JSON; the expanded bundles are about 330 000 objects each. Logged: 12 intakes of 37 MB; the JS heap grows 75 -> 397 -> 1102 -> 2873 -> 5862 -> 6315 MB and 24 to 29 million objects; reply `peakMegabytes` 7493 and 9491. The publishing side has the same shape (`offersInFlight` 4: four `documentsOf` + `JSON.stringify` of whole documents beside each other).

## 2. Numbers

| run (stdio-ifc-rs warm job) | peak footprint (tree) | peak rss | wall | cpu | note |
|---|---|---|---|---|---|
| before, store clone, fetch width 8 | 4495 MB | 7758 MB | 163 s | 145 s | the 4477 MB of `slice-memory.json` |
| before, empty store, fetch width 8 | 12288 MB (macOS rounds footprints above 10 GB to whole GB) | 9714 MB | 87 s | 104 s | heap 75 -> 6315 MB |
| width 1, still whole entry at once | 2685 MB | 3659 MB | 48 s | 44 s | |
| after (serial intake, one bundle at a time, width 2, offers 1) | **708 MB (770 with walk children)**, 120 to 200 MB while the entries come in | 3937 MB (freed pages the allocator keeps) | 72 s | 72 s | the 450 to 700 MB are the last derivations with 4 walk children |

Not reproduced after the fix: `repo-mcp-rs` and `procedural-generation3d-rs` (their records are not in the wave of 14:26 / vanished from `np` while I measured). A run of `repo-mcp-rs` with the first serial version (forced GC after each document) reached 5 to 16 GB footprint in two runs (one with 16 derivations and offers, one with the sampling profiler on, whose traces are 35 million objects and are not a measure); the forced GC was the top CPU frame in a profile and was removed (the version above has none).

## 3. Fix

1. `entry-lane/🟦️.ts`: the intake of a lane is serial (`serial`), takes the keys of all documents of an entry first (one parse each, dropped at once), then parses, localizes and imports ONE document at a time (`travel.import` gets one bundle). `🧬️schema`: `fetchConcurrency` 8 -> 2, `offersInFlight` 4 -> 1.
2. Admission (`project-farm/🏊️pool`): `createFarmV1({limitMegabytes})` (the plan limit of the lean slice, `lean.limitMegabytes`); a job carries `knownMegabytes` (the cost table, set in `warmOwners` `describe`) or the farm has measured its footprint on this machine (`learned`); a job whose known footprint plus the running ones passes the limit waits until they ended (a heavy owner runs alone), no extra process is started for it; jobs of unknown cost are admitted as before.

## 4. Laws (red before, green after)

- `entry-lane/🧪️tests/🌊️streamed-intake`: (a) six entries prefetched with a fetch width above 1: widest overlap of imports 6 -> 1, one bundle per import, every entry arrives; (b) six entries offered together: documents built at once 4 -> 1. Red on the old lane (6 and 4), green after.
- `project-farm/🧪️tests/🧮️admission`: owners of 600 MB known beside 1000 MB never overlap (3 overlaps before), 400 MB do, unknown cost is admitted as before, a job of 5000 MB runs alone, an owner measured at 700 MB counts with it afterwards.
- A law that bounds the memory of an owner is not stated: footprints depend on the machine and on what the fleet holds.

## 5. Landing

LANDED 22:34 to 22:37 UTC (lock `landing.lock`, released), all 11 test files green, see `landings.md` entry R97. After the landing the stored records of the wave were gone from `np` (the revision moved), so the numbers above are from the identical staged files (`lane.v4.ts` through the overlay), not from a post-landing run. The first window at 22:30 was RED on `🛑️hold` ("a hold that lifts while jobs wait lets the farm run side by side again": overlap 3, expected 2): it fails 2 of 3 runs on the UNCHANGED tree too under load (timing slack of 25 ms), the script restored. `runall.sh` now repeats a red file up to three times and reports the attempt.

## 6. Not verified

- `repo-mcp-rs` and `procedural-generation3d-rs` after the fix (see above); native memory of a derivation with offers (rss 7 to 11 GB with a 400 MB JS heap in one run) is not explained.
- A 16 GB / 8 GB worker end to end (first try after a restart); the lean slice limit that feeds `limitMegabytes` is only set where the plan has a bound.
- `prepare` jobs (cold inference) carry no `knownMegabytes`; only warm jobs and the learned footprint feed the admission.
