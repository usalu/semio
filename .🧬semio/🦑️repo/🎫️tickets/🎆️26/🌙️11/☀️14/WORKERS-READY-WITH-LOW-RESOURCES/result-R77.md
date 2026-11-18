# R77: source-closure capacity (g1-m04 `tools`) and the Windows `dependencies` stall

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, AU = `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/`, CL = `LIB/🏗️build/📥️source/🔏️closure/`. Scripts: `T/🧰️scripts/R77/` (landing 1, 06:19 CET = 05:19 UTC) and `T/🧰️scripts/R77b/` (landing 2, ~12:30 UTC). Both under `landing.lock`, tests inside, files restored on red, entries in `T/landings.md`.

## Part 1: the capacity error (landing 1)

- Throw site: `AU/🟦️.ts`, function `traverseNxAuthorizerSourcesV1` (`capture`): `buildSourceCapacityV1(files, bytes)` against `CL/🧬️schema/🔣️.json` `definitions.Policy` (2048 files, 20 MiB sources, 32 MiB snapshot). Not a count of "authorizers" but the whole import closure of a command kernel (`repo` runs from one captured snapshot of it). Failing entry on g1-m04: `LIB/🏗️build/⌨️command/⚙️runtime/🟦️.ts` (chain `…/🏃️task/🤝️shared/🟦️.ts` <- runtime kernel; `🤝️shared` is a new file of the day).
- Why a fixed number existed: the closure is passed as one JSON snapshot (stdin / auxiliary pipe, base64 of every file) and validated against closed schemas, so every array carried a `maxItems`; the file count was a copy of a byte-era guess (2048 files at about 10 KB), repeated in 12+ places.
- Why Macs did not fail: they run from a sealed snapshot / source memo (`replayCapturedV1` serves a remembered capture without the capacity check) so nothing re-captures while the closure is unchanged. I reproduced the failure on the Mac tree: a fresh `captureNxAuthorizerSourcesV1` of the runtime kernel gave `files=2049/2048, bytes=18555805/20971520`; census of the same entry: 2051 files, 18.56 MB, snapshot 26.4 MB. **Yes, Macs would have failed on their next promotion or memo miss** (the kernel canary tests `📏️kernels` were already red).
- R69's stale-generation diagnostic line does not appear in the `tools` logs of g1-m04 (the stage never got that far).

### What changed (one proportional family)
Policy 2048 files / 20 MiB / 32 MiB -> 16384 files / 64 MiB / 96 MiB (one file per 4096 source bytes, snapshot 1.5x), file 4 MiB unchanged; compiler snapshot policy (`AU/📦️snapshot/🔣️policy.json`) same, compiler output 8 -> 32 MiB; census 8192/64 MiB -> 65536/256 MiB; data discovery 1024 -> 8192 (+ 64 MiB); memo file 4 -> 32 MiB; compiler closure law 2400/18 MiB -> 12288/48 MiB (75 % of the policy); diagnostics schema maxima follow. Refusal text now bilingual and actionable (`nxAuthorizerCapacityTextV1`: counts, import chain, "drop the import or raise maximumFiles, maximumSourceBytes and maximumSnapshotBytes together"); same hint in `closure.capacity-exceeded` and the sealed `compilerClosureBounded`. Kernel canaries now fire at half of the policy, not 32 files before it.

### Sibling ceilings, current / limit on the tree at 05:00 UTC (Mac tree)
| ceiling | current | old limit | new limit |
|---|---|---|---|
| closure files, runtime kernel | 2051 | 2048 (over) | 16384 |
| closure source bytes | 18.56 MB | 20.97 MB (88 %) | 67.1 MB |
| closure snapshot bytes | 26.4 MB | 33.55 MB (79 %) | 100.7 MB |
| compiler (`📜️script.ts`) closure files | 1727 | 2048 (84 %) | 16384 |
| compiler bundle output (raw minified 5.6 MB, embedded images unmeasured) | >=5.6 MB | 8.39 MB (>=67 %) | 33.5 MB |
| data discovery entries / dirs / results / bytes | 824 / 643 / 561 / 3.6 MB | 1024 / 1024 / 1024 / 16.8 MB (80 % / 63 % / 55 % / 21 %) | 8192 x3 / 67 MB |
| memo file | 2.54 MB | 4.19 MB (61 %) | 33.5 MB |
| compiler closure law (files / bytes) | 1727 / 14.8 MB | 2400 / 18.9 MB | 12288 / 50.3 MB |
| census | 2051 files | 8192 (25 %) | 65536 |
| captured physical sources / images (revision schema) | 2051 | 2048 (over) | 16384 (landed separately as R78 after this missed it) |
| bazel scope traces `modules/reads/physical` | not measured | 2048 | 16384 (R77b) |

### Laws
`AU/🧪️tests/📈️growth` (new): proportional family; closures of 2049 and 4097 files are captured and equal an esbuild bundle's input set (third-party oracle); a closure of `maximumFiles+1` is refused in English and German with chain and way out. Before (old tree): load error / red; after: 5 pass. Updated `📏️capacity` (bounds, byte refusal now 17 x 4 MB), `📦️snapshot` corpus (17 x 3.95 MB), kernel and sealed canaries, fixtures.

### Not verified in landing 1
`CL/🧪️tests/📥️stream` (96 MiB boundary payload) and `AU/🧪️tests/🟦️.ts` (the 16384-file boundary with fixtures, esbuild oracle) need the repo process envelope ("Original parent Script process invocation required" under bare `bun test`); changed fixtures there were not run. This is how the revision ceiling was missed.

## Part 2: R77b (this resume)


### 1. One declared family, enforced by a law (landed ~12:33 UTC, `T/landings.md` "R77b")
- `AU/🧬️schema/🔣️.json` `$defs.SourceClosureFamily` declares every file-count ceiling of the family: 13 members that must equal `closure.Policy.maximumFiles` (+1 for the diagnostic maximum), and 13 exempt rows (hand-authored lists, cargo crate sources, package rosters, request batches) each with an English and German reason. JSON Schema cannot compute a `maxItems` from another file, so the single source is the Policy value and the law is what ties the copies to it.
- Law `AU/🧪️tests/📈️growth` "every file-count ceiling ... is declared once": walks every `🧬️schema` JSON under `🏗️build` and `⚡️caching` for properties named like the family (`sources, files, images, modules, reads, physical, sourceCount, closure, paths`) with a `maxItems`/`maximum` >= 64 and below the bound; each must be a declared member or exempt with a reason; members must equal the bound; the bound and the source bytes must be >= 1.25 x the real runtime-kernel closure (census, 2051 files / 18.6 MB today). A new schema ceiling of 2048 anywhere in the family now fails here, with its path.
- Fixed with it: bazel scope traces `modules/reads/physical` (2048 -> 16384, the next ceiling of the same family, found by the walk), revision test fixture (2048 -> 16384).
- Fail before / pass after: both new test files red on the old tree (family law: `SourceClosureFamily` missing; producer progress law: counters missing), 1+4 pass after, plus `📏️capacity`, `🌲️process-tree` and both standing preflights green.
- NOT run through the proper runner: `repo test` / `repo tools exec bun -- test` could not give the process envelope (`tools exec` ran the file with the same 3 "Original parent Script process invocation required" failures as bare bun; `repo show projects` sat 40 minutes in `build:source-admission` on this Mac and was stopped). So the revision tests (6 of 9 run, 3 need the envelope), the bazel scope tests, the stream test and the big authorizer test are unverified; only the schema/fixture laws above cover those edits.

### 2. Windows g1-m05 `dependencies`
- Not a capacity error. `ready-timings.json`: ten stalled tries (07:49 to 11:48 UTC) of 1250 to 2800 s, `outcome: stalled`; so R74's rule did cut it, every time, and it did restart at once (`reason stalled`), on new code (R70/R71 fence budget visible: `[DATA] Store dependency fences full 3 skipped 3 fullSeconds 177`). The `ready` child is alive (ready.json updated seconds ago).
- Cause: after the final fence the producer publishes the Source endpoint of each binding (about 1 500, one durable link swap per binding) and printed NOTHING per binding; the only repeated line before it (`dependencies:store-input-replay`, elapsedMs 0) was identical, so ten quiet minutes = "stalled". The phase is slow on Windows (new counters show about 6 s per binding), not hung. Each cut threw away derive + walks + fences (about 8 to 10 minutes) before getting back to the same silent phase, so it never finished. Bindings already published are durable and are skipped by the next try, but a try cut at 10 minutes only advanced a few dozen.
- Fix (R77b): the input replay publishes `dependencies:store-input-replay:<work units>`, the publication loop publishes `dependencies:publish-binding:<n>` every 8 bindings (`DP/🟦️.ts`, 2 edits). Law `DP/🧪️tests/📶️progress`: source law plus the real stall tracker (`readyStallTrackerV1`): a moving counter is progress, a repeated line is not; red before, green after.
- Observed after the landing (the running try picked up the new code, no restart needed): replay counter reached 6080 units in 770 s, then `publish-binding` 112 at 1420 s and 192 at about 1900 s, i.e. the phase now moves and is no longer cut.
- Not fixed / open: ~6 s per binding is the real cost (candidate causes, not measured: one `FlushFileBuffers` per sync on NTFS, the relative-handle layer, Defender). At that rate a full pass is about 2.5 hours but it is resumable per binding. "Seconds when unchanged" still depends on the first completed run publishing the sync receipt. Next measurement: time one binding with a trace of its operations on the Windows host.

### 3. g1-m04 `tools` after R78 + R77b
- Try 36 started 12:30 UTC and went past the former failure point (`build:source-admission`, `Closed compiler-captured physical Source context required`) at once; from 12:31 UTC it was in `Checking declared package providers`, then `dependencies:ownership`, then `javascript-selected-payload` at 44 312 units after 1 222 s (counter still moving). The earlier failed tries lasted 3 to 750 s. Result of the stage and of `workspace`: not yet known at the end of my 30-minute watch; read `/Users/Shared/semio-store/v1/origins/g1-m04/ready.json` and `ready-timings.json`.
- EDQUOT on pool wave files: R75 already added the fallback (`writeOwnerPoolWaveFilesV1`: temp dir first, then `.cache/f/owner-pool-waves/<id>` on EDQUOT/ENOSPC). Where the quota is: on g1-m04 (Linux, user `deck`) it is a per-user or per-project disk quota on the directory `os.tmpdir()` points to; I cannot read that host, so it is inferred from the error code. After R75 the pool log shows `lastError null`, 10 children, 12 slots.

### Files
Landing 1: closure/authorizer/snapshot/texts/sealed schemas and tests (see `T/landings.md` "R77"). Landing 2: `AU/🧬️schema/🔣️.json`, `LIB/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧬️schema/🔣️.json`, `LIB/🏗️build/📥️inputs/🔏️revision/🧪️tests/🔣️.json`, `AU/🧪️tests/📈️growth/🟦️.ts`, `LIB/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`, new `…/💾️storage/🧪️tests/📶️progress/🟦️.ts`.
