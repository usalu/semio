# result-show-perf: why `bun repo show projects` (pid 88833) ran 4 h 20 min, and the staged fix

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket. Tools: `C/show-perf-tools/` (`bench.ts`, `ab.sh`, `staged/apply.py|apply.sh|show-perf.diff`, `census-np.mjs`, `classify-np.mjs`, `classify-packs.mjs`, `prof.mjs`, `callers.mjs`, `children.mjs`). Read-only analysis on the live tree; everything else in `/private/tmp/claude-501/...` or `C/🗑️generated/show-perf-*` (removed at the end). No repo file outside `C/` was written. No sub-agents, no git writes.

## 0. Verdict (read this)

1. **The 4 h are not an O(n²) in the closure code.** On an idle-equivalent harness (CPU seconds, 8 workers, same observation scopes, same shared script cache as the real loop) a project costs 0.33-0.45 CPU-s; the whole tree is about 20-30 CPU-min, not 4 h (section 3).
2. **The process was garbage-collection bound under host memory starvation.** Two `sample` runs 18 min apart (main thread, 5-6 s each): 75 percent / 74 percent of the main thread inside JavaScriptCore GC marking or waiting for the GC helper threads, 8 percent in JIT'd JavaScript. 1.5-4.5 GB of its 4.7 GB footprint was in the macOS compressor (top CMPRS, flipping between samples), compressor churn 100-200 MB/s, swap 12.1 of 13.3 GB used. Two runaway shell commands of the fleet hold 43 GB of compressed memory on a 32 GB machine (section 2). The load average was 22 and later 63-75 (10 cores).
3. **Progress was bursty, then zero.** The run rewrote 622 inference records (all of them for the 1,519 projects that own a `📋️project.json`; none of the about 2,140 Cargo-only crates): burst 1 06:38-06:44 (381 records, 63/min), 3 h 05 min of silence, burst 2 09:49-09:53 (241 records, 54/min), then nothing. Counts at 10:29:52 and at 10:54:49 are identical (np json 3677 / 622 new, packs 6148 / 643 new): zero growth in 25 min, so the pack-growth ETA is unbounded.
4. **Decision** (the coordinator already stopped the process; see coordination.md): do NOT relaunch on the current machine state. First kill the two runaway peers, hold the fleet below load 10 for the run, then relaunch once (staged fix optional, section 5). Expected clean-machine duration 25-40 min (section 6).

## 1. What the process did (evidence)

- pid 88833 started 06:36:19, last observed alive 10:54:49 (etime 4:18:31, CPU 246 min = 70 percent of one core); stopped by the coordinator afterwards. stderr (`🗑️generated/chain/show-projects.err`, 29,303 lines: 17,248 `build:command-source-imports`, 455 `build:command-source-packages`, 104 `build:projects`, 0 `build:settle`) stopped growing at 09:53, the same minute as the last np write. For the last 50 minutes not even the throttled (200 ms) progress line was written, so the JavaScript mutator was not reaching `yieldInferenceV1` while the CPU stayed at 70 percent: the process was inside collections.
- Which projects: `classify-np.mjs`/`classify-packs.mjs` (np file name = sha256(root)[:24]): 622 rewritten records = 407 `project.json` + Cargo, 215 `project.json` only, 0 Cargo-only. `emojiProjectJsonNodes` orders its jobs `project.json` projects first, then Cargo-only crates, so the run was at most 41 percent through the first group (622 of 1,519) and had not started the 2,100+ Cargo-only crates. (Persisted records of the other projects are from 10-18 under another implementation revision and are invalid: the key contains the inference implementation revision.)
- Open files at 10:35: 408 retained pack read handles (`observedRecordStoreV1` keeps every store until the writer closes), optional ledger `used_bytes` 529,184,274 of 1,073,741,824 (quota not the limit), `memory_pressure` level 1 (the OS did not signal pressure; the cost is the compressor churn, not a kill).
- `sample` top-of-stack on the main thread: GC leaf addresses (the same addresses the three `Heap Helper Thread`s spend their time in) 54 percent and 38 percent, waiting for helpers (`__psynch_cvwait`) 21 percent and 36 percent, other native 14-18 percent, JIT'd JS 8 percent (`show-perf-tools/gcfrac.py` on `sample` output; bun is stripped, addresses are matched across threads, not symbolicated).

## 2. Host memory starvation (the multiplier)

`top -o mem` at 10:35: `Python` pid 97760 **25 GB** (6 h 46 min, glob over `**/*.json`, parent a `claude` Bash call), `grep -rlF 🚫️async … .` pid 85806 **18 GB** (9 h 29 min, 538 CPU-min, parent an orphaned zsh whose stdout is a task file of this session), `com.apple.Virtualization` (docker VM of the course builder) 8 GB, bun 88833 4.7 GB (3.8 GB compressed). Swap 12.1/13.3 GB. Both runaway commands are read-only scans; killing them is safe and removes 43 GB of compressed pressure.

## 3. Reproduction on a sample (no stop of the process)

`C/show-perf-tools/bench.ts` drives the real code of the live tree: `cacheInternals.projectWithDefaults` per project inside `observeProjectInferenceV1`, under `observeInference` + `settleObservedComputationV1`, one shared `createScriptInputCache`, 8 concurrent workers, scratch record store and scratch input-closure directories (a Bun load plugin redirects `repositoryBuildGraphWriterDirectoryV1` for `build-lock-closures` and `build-input-closures` to the scratch directory, so nothing is written to the checkout). Sample = every k-th `project.json` project plus every k-th Cargo-only crate (50/50).

| N projects | CPU-s | CPU-s/project | heap after full GC | full GC |
|---|---|---|---|---|
| 49 | 14.8 | 0.30 | 331 MB | 366 ms (first), 70-130 ms later |
| 98 | 33.0 | 0.34 | 381 MB | 74-167 ms |
| 120 (project.json only) | 40.7 | 0.34 | 283-343 MB (flat) | 49-167 ms |
| 193 | 54.6 | 0.28 | | |

Heap and full-GC time stay flat in N, per-project CPU does not grow with N: **no quadratic behaviour in the closure/observation code**, no per-file lease claim (`yieldInferenceV1` is throttled to one publish per 200 ms; a forced publish is one stderr write plus `setImmediate`, about 4 per target).

Where the CPU goes at N=193 (`--cpu-prof`, `prof.mjs`, `callers.mjs`, `children.mjs`; total 93.5 s over 8 interleaved workers):
- `targetScriptSources` 26.8 s (29 percent), of which `declaredModuleInputV1` **17.3 s (18.5 percent of everything)**, `existsSync` alone 13.6 s. Cause (inputs-closure, applied 05:22): every target's `inputs` is scanned for literal `.ts` module inputs, and a generated Cargo target repeats the 1,200 shared command inputs verbatim. Counted on 55 projects: 745 `targetScriptSources` calls (13.5 per project), 343,953 input checks (6,254 per project), 245,534 of them probe hits, each hit becoming a closure root. So every call re-probes, re-resolves, re-sorts and re-stringifies (150 KB key) the same 1,200 paths.
- `compactSourceInputs` 10.1 s (10.8 percent): `captureSourceInputInventoryV1` 6.0 s re-validates the same 1,200-file roster for every project.
- First-touch costs shared by all projects (TypeScript parse 4.2 s, `resolve`) are one-off, not per project.
- Persisted records grew: median 4,053 to 8,237 rows, `analysis` rows 290 to 2,007 per project, brotli bytes 97 KB to 232 KB (census-np.mjs), more GC pressure per project but linear.

## 4. Answers to the specific suspects

- Closure recomputed per target instead of memoized per file hash: per FILE it is memoized (`cache.files`), per distinct entry set it is memoized (`cache.closures`) but the key was the 150 KB `JSON.stringify` of the root list and the key plus a fresh `{workspaceRoot}/…` string per file per closure were retained for the whole graph (about 0.9 GB retained for 1,500 project scripts x 2 variants x 1,200 files, estimate). Staged: digest keys and shared path strings.
- Shared closures not shared across projects: every project script is its own entry set, so each project walks the shared 1,100-file library closure again (memo lookup + replay of about 8k observation rows per file set). Measured cost is moderate (14 percent inclusive incl. first touch); a per-file sub-closure memo is the follow-up, not staged (it changes cycle handling).
- Lease claim per file: no (see section 3).

## 5. Staged fix (not applied)

`C/show-perf-tools/staged/apply.sh check|apply [--dry-run]|diff` (anchored, each anchor exactly once, idempotent, refuses on drift, refuses while a graph/dependency command runs). Two files: `L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` (9 edits) and `L/🏗️build/📥️inputs/🟨️.mjs` (2 edits). `show-perf.diff` is the exact diff.
1. `declaredModuleInputsV1`: one observed memo per distinct input list (`memoizedObservedComputationV1`), digest key; every receiving project still replays the same `exists` rows.
2. `memoDigestV1`: closure and `commandDependencies` memo keys are sha256 digests of the ordered parts, never retained path-list strings.
3. `closurePathNameV1`: one shared `{workspaceRoot}/…` string per file per graph, used by the closure result and the escape check.
4. `capturedInventoryV1`: the pure, frozen `captureSourceInputInventoryV1` result is reused for an identical file roster (bounded 512).
Applying it changes the inference implementation revision, so every persisted inference record (including the 622 of the stopped run) is invalid: one cold load, which is what the relaunch is anyway.

### Benchmark before/after (staged fix, same harness, same 139-project sample, CPU seconds, load 58-99)

`C/show-perf-tools/ab.sh 150 <short scratch dir> live fixed live fixed` (the fixed variant applies the exact `staged/apply.py dump-ops` edits to the two modules in memory through the Bun load plugin; the checkout is untouched).

| run | projects | CPU-s | CPU-s/project |
|---|---|---|---|
| live #1 | 139 | 38.2 | 0.275 |
| live #2 | 139 | 37.8 | 0.272 |
| fixed #2 | 139 | 32.8 | 0.236 |
| live, 26-project run | 26 | 11.9 | |
| fixed, 26-project run | 26 | 10.8 | |

- Total CPU -13.5 percent at 139 projects; marginal cost per project (slope between the 26- and 139-project runs) 0.231 s live vs 0.195 s fixed (-16 percent). One fixed attempt aborted with `Build graph implementation authority changed during inference` on a Cargo.toml a peer edited mid-run (the harness reads the manifest before the observation scope; the real run has the same exposure while the fleet edits manifests).
- Extrapolated whole tree (3,656 projects, estimate): live 11.9 + 0.231 x 3,630 = 850 CPU-s = **14.2 CPU-min**; fixed 10.8 + 0.195 x 3,630 = 719 CPU-s = **12.0 CPU-min**. The fix saves about 2 CPU-min. It is a small win, not the explanation of 4 h.
- Equivalence (language-agnostic cross-check): for the 138 projects persisted by both variants, the sha256 of every persisted result image and of every persisted observation-row key set (`kind` + `path`) is identical (`digests.json` compared per root; the 139th project was persisted by the fixed run only because the live run hit the racy-timestamp window). Live #1 and live #2 digests are identical, so the harness is deterministic.
- Memory effect of the fix (digest keys, shared path strings: estimated 0.9 GB of the 4.7 GB heap for 1,500 project scripts) is NOT measured: the `SERIES` runs were aborted because swap reached 20.5 of 21.5 GB and my harness (1-2 GB) would have added to it.


## 6. ETA and recommendation

**Process state:** pid 88833 was stopped by the coordinator after 4 h 18 min (`coordination.md`). 622 of 3,656 inference records were rewritten; 3,034 projects (about 897 `project.json` projects, then about 2,140 Cargo-only crates, plus bookkeeping) are still to do. Those 622 records stay valid for the relaunch as long as no file of the inference implementation or of a project's closure changes (the key holds the implementation revision).

**ETA from pack growth (asked for):** `.records` packs and np json were counted at 10:29:52 (np json 3677 / 622 new, packs 6148 / 643 new) and at 10:54:49 (identical) and the same count held across the 3 h 05 min between the two bursts. Average over the whole run 622 / 258 min = 2.4 records/min, which would need 21 h for the remaining 3,034; at the burst rate (54-63/min) it would need 48-56 min. Neither is a forecast: the rate is set by host memory, not by the code. Do not read an ETA from stderr either: it carries no counter (`yieldInferenceV1` receives the project index but prints only the phase name).

**ETA on a clean machine (estimate, harness-based):** pre-project phases 2.4 min (start 06:36:19, first record 06:38:41) + projects 12-14 CPU-min (3,034 x 0.2-0.23 s harness figure, real loop up to 1 s/project at load 22 = 52 min worst case) + post-project phases 2-3 min (result-gperf) + publish tail (not measured; earlier runs died there, see result-gperf section 5). Total 25-60 min with a full core and no swap. Peak RSS 12-17 GB (result-gperf), so it needs at least 20 GB of free RAM.

**Recommendation (coordinator decision):**
1. Now: kill the two runaway peers, `kill 85806 97760` (grep 19 GB and Python 27 GB, both read-only scans, 10 h and 7 h 18 min old; both belong to this session's tool calls). Swap is at 95 percent (20.5 of 21.5 GB) and still growing, `node` 98019 (10 GB) and `bun` 10057 (5 GB) appeared since; the machine will start killing applications. I did not kill anything.
2. Do not relaunch at load 60-100 (a single-threaded job gets about a tenth of a core). Hold the cargo wave and the agent fleet below load 10 for roughly 45 minutes (queue the cargo window behind the show).
3. Relaunch `bun repo show projects` once, WITHOUT applying the staged fix first: it keeps the 622 valid records and the fix is worth only 2 CPU-min. Apply the staged fix with the next change that forces a cold load anyway (any inference-module edit does); `apply.sh check` then `apply.sh apply`.
4. If the relaunch is again silent for 10 minutes: `sample <pid> 5` twice; GC-bound again (more than 60 percent of the main thread in the unsymbolicated helper-thread addresses) means memory pressure, not code.
5. Follow-ups (not staged): print the project index in the progress line (ETA); drop `projectImages[].image` (a second JSON copy of every project, about 0.8 GB at 3,656 projects, only needed when the same process re-infers); per-file sub-closure memo so projects share the library closure walk.


## 7. Not verified / limits

- The GC-bound diagnosis comes from two unsymbolicated `sample` runs (bun is stripped) plus footprint/compressor numbers; I could not attach a JS profiler to the live process. The harness shows no CPU-side explanation, which is why the host starvation is the working cause.
- The harness has no real invocation (`yieldInferenceV1` returns immediately), no graph-writer lease or optional-ledger publication, no pre-project phases (owner discovery, playground catalogs, cargo targets: 3.4 GB of the 4.7 GB heap per C/result-gperf.md), and native lock publication is redirected to a scratch directory.
- Load average 22-75 during all measurements: CPU seconds are quoted, wall seconds are not comparable.
- The coordinator stopped pid 88833 during my work; I did not stop, signal or kill anything (the two `sample` runs suspend a process for milliseconds only).
