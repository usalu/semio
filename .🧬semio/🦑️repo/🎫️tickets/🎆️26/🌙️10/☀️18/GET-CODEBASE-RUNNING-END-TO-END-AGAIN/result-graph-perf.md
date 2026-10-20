# result-graph-perf: faster graph construction, cheaper `bun repo run`, no exclusive walks in a task run

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket, tools = `C/graph-perf-tools/`. Executor `graph-perf`. WRITE PAUSE respected: nothing under `L` or any other repo path was written. Everything is STAGED as an anchored, idempotent set that builds on `C/show-perf-tools/staged` (its ops are included, apply order does not matter). No sub-agents, no git writes, no cargo, no `show projects`, no sync. Scratch: `/private/tmp/gp` (mirrors deleted).

## 0. Verdict

1. **Not measured after the lift** (no "PAUSE LIFTED" reached me). Cold `show projects` <= 5 min, warm <= 30 s and `run` overhead <= 3 s are therefore NOT verified. `C/graph-perf-tools/measure-after-lift.sh TARGET [cold]` produces all of them in one go.
2. **Cold inference CPU -40 percent** against the live code, -23 percent against the show-perf set alone (139-project sample, same harness, CPU seconds, load 45-60): live 35.7 / 35.3, show-perf 28.2, this set 21.2 / 21.7. Persisted result images are byte-identical (sha256 of every result: `8f45ec2b7e29fdaf` live = staged; `351b1687d7c44e3e` live = show-perf, 138 persisted). Extrapolation (estimate, not measured): about 0.11 CPU-s per project marginal, about 6.7 CPU-min for 3,656 projects instead of about 14.
3. **The `bun repo run` cost on the non-selected path was O(everything) and is now O(distinct facts)**: seal verification read, brotli-decompressed and parsed all 3,677 inference records (505 MB, 2 MB uncompressed each, about 95 s CPU) on every run, and the git membership scan hit a hard 10 s deadline, silently disabling graph reuse under load. Both fixed in the staged set (section 3).
4. **Coordinator item (exclusive walks in a task run)**: root cause found and fixed in the staged set with a test (section 4).
5. A worker pool (Bun/Node `worker_threads`, progress + cancellation) exists, is tested, and is used for the seal census. I did NOT parallelize the per-project inference loop across workers: it is not separable (section 6).

## 1. Profile (evidence)

`C/graph-perf-tools/{gp-bench.ts,ab.sh,prof-window.mjs,np-facts-census.mjs,membership-time.ts,pre-phases.ts,zlib-async-micro.mjs}`.

- Per project (steady state, after first-touch): `declaredModuleInputV1`/`targetScriptSources` 27 percent (a generated Cargo target repeats the 1,200 shared command inputs; each became a closure root and was probed/resolved/stat-ed again for every one of 13.5 targets per project), `observeInferenceComputationV1` 27 percent (re-serialising 8k shared observation rows per project: JSON 4.5 s, sha256 3.3 s, brotli 6.3 s per 139 projects), `compactSourceInputs` 11 percent (each closure manifest re-read from disk for verification, inventory recaptured), `modules` set (`resolve` of 1,200 paths per target, 8.6 percent), observation replay 7 percent, TypeScript parse 10 percent (first touch, one-off).
- No quadratic code (confirms result-show-perf). The 4 h run was GC/swap bound; memory per project is the scaling risk: `projectObservationRows` 1.09 M rows and 3.1 GB RSS at 139 projects in the bench.
- Persisted records: 400 of 3,677 sampled: 1.86 M rows, 37,934 distinct (2 percent); decompress+parse 10 s per 400, so about 95 s for all. Records are 2 MB uncompressed (brotli 505 MB total).
- zlib on the real records (80 files): decompress sync 189 ms CPU vs libuv-pool 375 ms (worse), compress q5 sync 955 ms CPU / 1,949 ms wall vs pool 669 ms wall (same CPU, 2.9x less wall). Hence compress goes async, decompress stays sync.
- Owner discovery (pre-project): 12.8 CPU-s, 535 MB heap, not a hotspot.
- `git` membership of the live tree under load 45: tracked 0.3 s, `--deleted` 7.2 s (lstat of every tracked file), `--others` 13 s, total 13.8 s for 122,164 files; the code's per-read deadline is 10 s, so the call is refused and the seal never holds.
- Default `NX_DAEMON` is now "true" (`⚡️caching/🚀️bootstrap/📜️script.ts:243`): a single selected task takes the daemon "selected graph" path (no membership scan, no seal). The seal/membership path serves non-selected runs (`run-many`, workspace-wide, no daemon). `bun repo run x:y` overhead there is bounded by the selected-graph receipt, not by anything in this set; measure with the script.

## 2. Staged changes (`C/graph-perf-tools/staged/`, `apply.sh check|diff|apply [--dry-run]`; 68 steps, all pending, 0 drift; longest target path 147 characters)

Edits of existing files (`staged/gpops.py`, `staged/bootstrap_ops.py`; the show-perf set first):
- `L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: digest memo keys, shared closure path names, per-list declared-module memo (show-perf); NEW: a target input that names a module the entry closure already compiled is skipped (exactly: only modules in the closure's compiled set, a resource-only file is still followed, tested), `generatedTargetsV1` derives the absolute module set once per distinct closure.
- `L/🏗️build/📥️inputs/🟨️.mjs`: inventory memo (show-perf); NEW: a content-addressed closure whose stamp equals the last verified one is not read again.
- `L/🏗️build/👁️observation/🟨️.mjs`: row JSON text cached per interned row (header = concatenation, byte-identical, tested), async brotli compress, project image as Buffer (about -365 MB at full scale). (A stamp shortcut for analysis facts was tried and REMOVED: the law "same-stamp byte changes cannot bypass current parser admission" failed.)
- `L/🏗️build/🚀️bootstrap/🟦️.ts` (section 4), `L/⚡️caching/🕸️graph/🔏️seal/{🟦️.ts,🧬️schema/🔣️.json,📥️membership/🟦️.ts}` (section 3).
New modules: `🏗️build/🧵️workers/` (section 6), `⚡️caching/🕸️graph/🔏️seal/{📦️facts,🧮️census,📥️membership/🧬️schema}`.
New/updated tests (all language-agnostic JSON fixtures + Ajv closure): `🧵️workers/🧪️tests`, `📦️facts/🧪️tests`, `🧮️covered-inputs`, `🧾️record-header`, `🚀️bootstrap/🧪️tests`; replaced `seal/🧪️tests/{🟦️.ts,🧫️fixtures/🔣️.json}` (records written as brotli, new scenario `inference-record-rewritten`).

## 3. Graph reuse policy (nx cached graph is reused only when the seal holds)

Seal format 3: the seal no longer re-reads every inference record. `captureSealFactsV1` (once per graph build, on the worker pool, with cancellation) reads each record header named by the receipts, requires its sha256 to equal the receipt's, deduplicates all rows (and the receipts' own rows) into one brotli table `semio-graph-facts.<sha256>.json.br` (size 3-5 MB instead of 505 MB), and stamps the records. `sealFactsHoldV1` verifies: table integrity (sha256), every record stamp (lstat), then `observationsHold` over the DISTINCT facts. Invalidation is exact: any changed observed file, record (rewritten = new inode/stamp), table byte, receipt, graph file, Nx identity, membership or input stamp makes the seal not hold. Table files are retired except the current and previous seal's.
Membership: the 10 s per-git-read deadline (and the 64 MiB cap) moved into `📥️membership/🧬️schema/🔣️.json` (180 s, still cancellable by the caller's signal).
NOT done: an incremental membership frontier (directory-stamp receipt replacing the 13 s git scan on non-selected runs). Design: stamp every directory that contains a member plus `.git/index`, all ignore files and `.git/info/exclude`; reuse the list when all stamps hold, else fall back to git. It carries correctness risk for an "exact membership" authority and needs the real Nx SDK fixture to verify, so I only measured and documented it.

## 4. Coordinator item: why a `repository:command` takes the exclusive dependency image (evidence + fix)

Evidence: pid 8903 (`run-many ... --skip-cache`) log `DASHBOARD-LAUNCH-COCKPIT/🗑️generated/e2e-process-current.log` (91 MB): `build:dependencies stage running` at line 598 is published only when `ensureBuildEnvironmentV1` got `material-required`; it is followed by `dependencies:store-ownership` (exclusive), `retire-obsolete` (265 s deleting 6 generations), `store-stage`, `store-input-replay`, `workspace-edges`, i.e. the full `prepareJavascriptDependencies("sync")` that `dependencies sync --all` runs. `material-required` comes from `inspect(false)` when (a) no generation receipt matches (`fast.status !== "match"`, which is every task run after a manifest batch until someone publishes one) so it walks the whole tree in the shared hold, and (b) the parser witness/walk throws while the tree or `node_modules` generation is in flux (the catch also maps a transient failure to "material-required"). The default installer then takes the exclusive image and syncs, queueing the coordinator's own sync behind it. The adoption path `inspect(true)` repeats the same full walk up to three times under exclusive (owner, `confirmed`, `settled`).
Fix (`bootstrap_ops.py`, in `🏗️build/🚀️bootstrap/🟦️.ts`): (1) double-checked install: `installUnlessReadyV1` re-checks readiness under the exclusive hold it finally obtained with a cheap, receipt-only `recheck` (generation receipt must match, no walk) and skips the install when a peer already did it, so runs queued behind one sync install at most once; (2) adoption without walks: the shared walk's evidence is handed to the exclusive phase and published through the existing publish-with-proof (`publishJavascriptDependencyGenerationReceiptV1` replays every observation against the live tree and checks `walk.sha256`); only when it is stale/deferred/failed (`adoptedCandidateV1`) does the exclusive phase walk again; `confirmed`/`settled` become receipt verifications. The structural shape that the existing AST law (`🚀️bootstrap/📦️parsers/🧪️tests`) extracts (two catch returns, the `!publishReceipt` branch, the tail) is preserved; its port list gets `recheck`.
Not changed: a run still installs when material truly needs a new generation (zero-touch); it just does so once. The first shared walk after a manifest batch remains one full walk per concurrent run until the first adoption publishes a receipt (the next lever: serialize that walk on a shared lease).

## 5. Verification (mirror = real copy of `L` with symlinks elsewhere, `graph-perf-tools/{mirror.py,rebuild-mirror.sh,run-tests.sh}`)

| check | result |
|---|---|
| `workspace-closure` (inputs-closure suite, esbuild + `bun build --metafile` + Python hashlib oracles) on patched mirror | 7 pass (control 7 pass) |
| `covered-inputs` (new; naive reference, bundler containment, shared cache) | 3 pass |
| worker pool suite (order, Python hashlib cross-check, thread bound, progress, pool + per-task cancel, failure, crash replacement) | 7 pass |
| seal fact table suite (serial set, Python distinct count, 7 mutation scenarios, 2 refusals, cancel, retire) | 5 pass |
| `record-header` (canonical JSON, key order, tracks file) | 2 pass |
| bootstrap helpers (3 install scenarios + adoption table) | 3 pass |
| bootstrap parsers suite | 2 pass / 2 fail, identical to the unpatched control (the two failures are environment: cwd, Nx SDK) |
| observation suite, `-t semantic` | 13 pass, 0 fail (the full file did not finish in 28 min under load: realize loop re-runs it per symlink; the earlier full run of the patched mirror showed 50 pass and the 2 failures caused by the removed stamp shortcut; control not completed) |
| observation `reuse` suite | 12 pass (control 12) |
| seal fixture suite (`seal/🧪️tests`) | NOT runnable in a mirror (control also fails: "Exact physical cache ancestor required"); `seal/🟦️.ts`, bootstrap and membership bundle with `bun build --target=bun`; membership returns the right list on a scratch git repo |
| red/green | new helper and worker/facts/header suites have no counterpart in the control; `covered-inputs` is an equivalence law and passes on both |

Benchmarks: `ab.sh N DIR live|showperf|gp` (CPU seconds are the metric; wall is load-dominated). 98 projects: live 26.7, show-perf 24.3, staged 21.4 CPU-s; 139 projects: 35.7/35.3, 28.2, 21.2/21.7. Disk-warm per project about 30 ms CPU with the show-perf code (unchanged by this set; the fact-table record format below would address it).

## 6. Worker pool (`🏗️build/🧵️workers/`) and why the project loop is not parallelized

`createWorkerPoolV1({workers, signal, onProgress, onTaskProgress})` -> `run/map/close`; tasks are `{module: file URL, export}` pure functions; messages are the closed plain-data protocol of `🧬️schema/🔣️.json`; pool signal cancels all (workers terminated), `run(..., {signal})` cancels one task; a crashing worker rejects only its task and is replaced; no dependency beyond `node:worker_threads` (Node and Bun). Used for the seal census (95 s serial to about a tenth with 8 workers, estimate).
The per-project loop cannot be split across workers without a redesign: projects share the async-local observation scopes, the script-input cache (closures memoized across projects), the record stores with graph-writer custody, and pre-project state; cloning them per worker costs more than the work. The honest route to <= 5 min cold is the CPU cut above plus the follow-ups below.

## 7. Open items / follow-ups (ordered by value)

1. Run `graph-perf-tools/measure-after-lift.sh <cheap-target> cold` after the lift and fill: cold show (target <= 300 s), warm show (<= 30 s), `run` x3 (<= 3 s overhead), and the `[nx] Graph observation selected=... sealed=...` lines. Apply: `staged/apply.sh apply --dry-run` then `apply` (needs the Nx tooling re-materialized: the seal/membership modules are part of the immutable SDK identity, so run `bun repo tools setup nx` afterwards, as the chain does).
2. np record fact-table format (header = ids into one shared digest-addressed table): cuts the 2 MB header per project (stringify + sha + brotli about 18 percent of cold CPU, parse about 60 percent of disk-warm) and the retained-row memory. Requires the observation module core (`observeInferenceComputationV1`, `inferenceHeaderLine`, receipts) and a broad test run; not staged blind.
3. Membership frontier (section 3).
4. Serialize the first shared walk across concurrent task runs on a shared lease (section 4).
5. Enable `yieldInferenceV1` to return non-promises on its throttled path (14k awaits per project).

Files: tools `C/graph-perf-tools/**` (49 files, 264 KB, no generated output left); staged set `C/graph-perf-tools/staged/{apply.py,apply.sh,gpops.py,bootstrap_ops.py,new/**}`.
