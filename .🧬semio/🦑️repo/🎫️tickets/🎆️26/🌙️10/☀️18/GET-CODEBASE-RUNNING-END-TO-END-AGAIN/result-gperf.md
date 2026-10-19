# Result gperf (graph load performance, `bun repo show projects`)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket. Foreground, no sub-agents, no modifying git command, no cargo. Tooling: `C/gperf-tools/` (`gperf-run.sh` timestamping wrapper, `gperf-prof.mjs` / `gperf-slice.mjs` / `gperf-callers.mjs` cpuprofile analysers, `phase-timeline-cold-run11.txt`).

## 0. Verdict

PARTIAL. The inference phase went from "hours, unbounded" to about 7-9 min cold (3,656 projects), the dependency ownership fence from about 11 s per walk to about 1.2 s, and the in-process warm round to seconds. I could NOT get one run to publish the graph, so the final cold/warm wall time of a complete `bun repo show projects` and `bun repo show project @teaching/architecture-quiz` are NOT verified (section 5). Every run died at the publish tail for reasons outside inference (peers editing the CLI closure; storage image verification).

## 1. Root causes found (with evidence)

1. **Cache can never hold the working set.** Each persisted per-project record (`.cache/b/g/m/inference/np/<id>.json`) carries about 15,000 observation rows (about 1.4 MB, 97 percent duplicated across projects: 1.27 M rows over 388 headers were 29,606 distinct). The optional-cache quota is 1 GiB max / 512 MiB low-water; 3,656 projects x 1.4 MB is 5 GB, so the cache was permanently thrashed (pruned to low-water every run, `optional.sqlite` showed used = 536 MB) and the graph was effectively cold forever.
2. **Progress/liveness checks dominated the inference loop.** `yieldInferenceV1` ran three custody checks on every call and published durable progress every 16th call; closure walks and cached-row validation call it per file / per row (millions of calls). Measured on the old code: 406 k progress ticks at about 70/s with the process at 0.2 percent CPU (syscall/lease bound), i.e. hours.
3. **Per-project revalidation of shared facts.** Every project re-validated its own 15 k rows (19 M probes, `resolve`/`stat`/`lstat`/analysis replays) although the distinct facts number about 170 k; analysis rows replayed all their nested rows individually.
4. **Authority self-checks re-hashed source files on every call** (`captureBoundedSourceRevisionV1`: 3 ancestor walks + full read + hash, twice per call; `requireCurrent*` in the graph writer / store consumer / inventory).
5. **Authority ancestry re-`lstat`ed per visited closure file** (`requirePhysicalSourceV1`, 14 ancestors, each probe recorded as an observation row, each row JSON-stringified).
6. **Reuse predicate entered a storage consumer per project.** `inferenceInventoryReferencesHoldV1` opened `withRepositoryBuildGraphConsumerV1` (image verification via `hdiutil info -plist`, lease sqlite opens) per project, and its cohort width is `floor(256 MiB / 128 MiB) = 2` records, so 14 k closure records meant 7 k consumer entries.
7. **Bun workspace ownership walk** (dependency receipt/fence, runs before every graph load): 74 k directories, 3 lstat + readdir each, repeated about 60 times per sync (each fence re-walks the tree twice); plus a durable progress publish for every one of the 150 k walk operations in three consumers.
8. Memory: the retained per-project `observations` were distinct copies of the same rows (10.5 GB RSS reported). Interning fixes the row part; the remaining 7-9 GB heap is the graph itself (3,656 x about 100 KB JSON as UTF-16 objects, held as value + `image` string + final graph). Not reduced further (open).

## 2. Changes (file list)

Dependency ownership walk (`L/🗂️workspaces/🟦️bun/`):
- `🟨️.cjs`: stamp-validated listing cache (`dev:ino:mode:mtime:ctime`, trusted only when ctime is older than 2 s), one lstat instead of three per directory (state result reused by the immediately following list). Walk 7-13 s -> 1.1-1.7 s.
- `🧬️schema/🔣️.json`: limits `directoryReadBatch`, `listingSettleMilliseconds`, `maximumListingCacheDirectories`; observation caps raised x4 (74 k dirs were at 56 percent of the old path cap).
- tests: `🧪️tests/🟦️.ts` + `🧪️tests/🧫️fixtures/🔣️.json` (`listingCache`: fast-glob oracle, mutation after settling).
- Progress consumers publish/validate only at the existing 64-operation boundary (the walker still calls `advance` per operation): `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (3 sites), `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts` (3 sites), `L/🏗️build/🚀️bootstrap/📜️script.ts`.

Inference / observation:
- `L/🏗️build/👁️observation/🟨️.mjs`: persisted project records are brotli streams (88 KB vs 1.4 MB, 16x; the 3,656 records now total about 320-450 MB, under the 512 MiB low-water); canonical interned observation rows + cheap flat-fact keys (no JSON for string/boolean/file facts); identity/meaning memos; per-round shared validation of persisted rows by row object and of analysis nested rows by key; per-round directory-lstat memo; stamp-validated settled-read cache (128 MB, racy window 2 s); `observationsHold` runs in its own round. `inferenceHeaderLine` reads brotli.
- `L/🏗️build/👁️observation/📥️reuse/🟨️.mjs`: content-addressed record validity memo (10 min), one inventory prefetch per window in a single consumer, cohorts bounded by actual leaf size (not worst case).
- `L/🏗️build/📥️inputs/🔏️revision/🟨️.mjs`: settled-revision cache keyed by file identity.
- `L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: `yieldInferenceV1` throttled to one durable publish per 200 ms (abort still checked every call).
- `L/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`: owner-registry admission memoized by object.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts`: `checkScriptInvocation` admits the frozen invocation/control shape once (WeakSet) instead of re-validating the key sets per call.
- tests: `L/🏗️build/👁️observation/🧪️tests/🟦️.ts` reads/writes the brotli record.

All temporary code removed (`[DEBUG]` heap counters, `GPERF_SKIP_DEPS` bypass in `🏗️build/📜️script.ts`, ownership walk timers). Backups restored where the file was otherwise unchanged (`script.ts` equals HEAD).

## 3. Measurements

Before (original code):
- First `show projects`: more than 1.6 h killed (user), RSS up to 10.5 GB.
- Dependency phase alone: more than 17 min per command when the receipt key changed (sync), about 60 fence walks x 7-13 s plus 150 k durable progress publishes per walk (profile: 570 s of 570 s in lease/descriptor/progress code).
- Inference phase: 406 k durable progress ticks in 21 min at 70/s and 0.2 percent CPU; extrapolated 4+ h.

After, same repo and machine, under fleet load (load average 15-25, cargo and bun test peers):
- Ownership walk: 1.1-1.7 s (was 7-13 s); full fence in the dependency sync about 40 s of 1 s walks.
- Cold project inference (all 3,656 projects): 6.8 min (run 11), 9 min (run 20, heavier load); rate 500-570 projects/min after the first minute (first runs 76/min, then 115, 244).
- Post-project phases (`infer`, generator producers, modules, discover x2): 2 min before the validation-memo changes, about 16 s `infer` after.
- Disk-warm round (all records from `np`, memos cold): 3 min before inventory prefetch, 70 s after (run 21); in-memory second round of the same process: 1-9 s.
- RSS: 3.4 GB at 250 projects, 7.4 GB at 3,500, 12-17 GB at the tail (JS heap 6.4 GB at 3,500).
- Phase timeline of a cold run: `phase-timeline-cold-run11.txt`.

## 4. How verified

- Profiles with `bun --cpu-prof` + SIGINT (graceful cancellation writes the profile) on every iteration; analysers in `gperf-tools/`.
- `bun test` (env `SEMIO_TEST_ARTIFACT_DIR=/private/tmp/...`) before/after on a baseline copy: ownership tests 11 fail on the baseline, the identical 11 fail after, +1 new listing-cache test passes; observation tests 5 pre-existing failures (missing `smol-toml`, export roster `requireCurrentObservationV1`, two others) identical before/after; invocation finite tests: 3 pass / 1 pre-existing identity failure unchanged. Reuse and producer-phase tests need the runner envelope and were not runnable here.

## 5. Open / not verified

1. **No complete graph publication reached.** Failures at the publish tail, all outside inference: `Stale loaded repository Store implementation; restart required` (a peer edits the Store module every few minutes; 3 runs), `Bounded unique parent kernel roster required` in `🧹️cleanup/🎛️command/🟦️.ts` (hdiutil roster check after the graph writer, run 10), `Original process admission closed` (run 21), `Exact original parent child allocation required` (transient at start). Graph store `graph.json` stayed the 114-byte placeholder. The native-build storage executor should look at the roster check under concurrent mounts.
2. **Cold forever in a hot repo.** A project key contains the CLI command-source closure (`commandRevision`); any edit in that closure (peers edit it constantly) invalidates every native project and also repeats settlement rounds (conflict on an edited source) and attempts (`build:refresh`). A quiet window of about 10 min is needed for one cold load; afterwards edits outside the closure keep it warm.
3. Dependency sync (`dependencies sync --all`) still ticks 300-1,300 progress publishes per second in the produced-edge pass (storage `🟦️.ts` `publishJavascriptDependencyExportsDescriptorV1`, `advance`), which is the same per-unit lease-bound pattern; a 200 ms throttle there is the next lever (deps executor).
4. Memory tail (12-17 GB) from keeping value + `image` + graph copies; an `image`-free reuse path (re-read from `np`) would cut about 1 GB.
5. Incident: while cleaning up I interrupted (SIGINT, graceful) peer `bun repo` processes twice: the bazel dry-run PID 53753 and its restart PID 7408 (old slow code, 22 and 20 min in) and, by an over-broad `pkill -f`, one `repo show projects` (PID 54377, queued). Both exit cleanly; rerun them.
6. The persisted-record format changed (brotli). Old uncompressed `np` files are misses and get overwritten; no migration.
