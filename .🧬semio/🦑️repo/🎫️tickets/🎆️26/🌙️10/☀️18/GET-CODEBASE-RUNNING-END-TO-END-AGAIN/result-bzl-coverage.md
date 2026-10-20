# result-bzl-coverage: how every task kind executes under Bazel, and the staged change that makes tests cached `bazel test` results and servers an explicit interactive class

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. `BZ` = `B/🔧️tools/🏗️bazel`. Provisioned Bazel 8.4.2 (`.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel`), macOS 27 arm64, load average 35-45 throughout. STAGE-ONLY: nothing under `📚️library/`, `package.json`, `bun.lock`, root `📜️script.ts` or any `🔣️taxonomy.json` was written. Everything was verified in a scratch overlay copy.

## Verdict

1. Today every one of the 8,027 statically reconstructed tasks is a Bazel action that re-enters `bun`: `repo_task` (or `repo_launch` for continuous ones), all tagged `no-sandbox` (`host_state`) and `no-remote-exec`; the uncached ones add `no-cache` with a recorded reason. So the answer to "cached hermetic action" is: 0 of 8,027 are hermetic (nothing is sandboxed, the task runs in the real checkout); 3,846 are cache-eligible (Bazel action cache and disk cache work, measured below); 2,870 are `no-cache`; 1,308 are `bazel run` targets; 3 are fail-closed (`cache: true` without `outputs`), plus 3 dangling `dependsOn` declarations the planner refuses.
2. The cache machinery is not the gap. The gap is classification: the neutral contract had no notion of tool family and no `test` or `interactive` disposition, and Bazel tests were never `bazel test` targets, so test results were never in Bazel's test cache.
3. Staged (anchored apply, dry-run verified against the live checkout): a closed neutral schema plus classifier `family x disposition` (`action | test | interactive | uncached | refused`), a `repo_test` rule so a selected, unconsumed, cached `test*` task with `outputs: []` runs as `bazel test` (1,789 of 8,027 tasks, 1,816 test-named, 27 stay actions because another selected task consumes their result), and the `interactive` class for the 1,308 continuous tasks.
4. Real Bazel verification in the overlay, one real tool per family: cold 22 spawns, warm 0 spawns (tests `(cached) PASSED`), after `bazel clean` 22 disk cache hits and 0 executions, deleted outputs restored without a spawn, exact invalidation, early cutoff, no-cache and failing tests re-run every time, interactive runs every time.
5. What is still not a cached action and why, by family, is in "What stays uncached". Most of it is intrinsic (long-running, publishing, host state) or owned by `bzl-rust` (cargo/wasm playground chain). 93 non-Rust tasks are `cache: false` with a verification verb and are listed for their owners in `C/bzl-coverage-owner-candidates.md`.

## How a task executes today (from `BZ/📜️script.ts`, live file)

| Piece | Behaviour |
|---|---|
| Rule | `repo_task`: `ctx.actions.run(bun runner birth-action manifest snapshot-dir dep-snapshots...)`; the runner (captured implementation) executes `bun exec "<command>"` in the real checkout, then copies declared outputs into a tree artifact `<label>.snapshot`; consumers restore dependency snapshots into the checkout before they run |
| Requirements | `no-remote-exec` always; `no-sandbox` for every task and launcher (`host_state`); `no-cache` + required `uncached_reason` when the cache decision is false |
| Cache decision (`bazelCacheDecisionV1`, fail-closed) | continuous, skipped, noop (always cacheable), plugin class other than `deterministic`, `cache: false` -> `declared`, missing `cache` -> `undeclared`, `cache: true` without an `outputs` array -> `unbounded`, else cacheable |
| Continuous | `repo_launch`: the launcher action only copies the bun binary; the task runs through `bazel run //:label` per task, never in the action graph |
| Tests | no special treatment: a test is an ordinary cached or uncached `repo_task`, run by `bazel build`, so no Bazel test result and no `bazel test` |

## Census (static whole-graph reconstruction: 1,466 projects, 8,027 targets incl. 4,217 synthesised playground targets; 3 dangling `dependsOn` pruned as the planner would refuse them)

Family is assigned by the new ordered rule table (target name, project root segment, command, entry-script text for vitest and vite and tsc detection). `cargo` and `wasm` rows are the `bzl-rust` families; the rule table maps every Rust-root project to `cargo` (their `bun` routers call cargo) and playground `prepare-*` to `wasm`. Entry-script text was found for 7,804 of 8,027 tasks; where it is not, `vitest` falls back to `bun-test` and `vite`/`tsc` to `bun-script`, so those three counts are lower bounds.

### Before (live adapter, how each task executes)

| family | tasks | cached action | no-cache action | run target | fail-closed |
|---|---:|---:|---:|---:|---:|
| bun-script | 1639 | 828 | 156 | 652 | 3 |
| tsc | 35 | 34 | 1 | 0 | 0 |
| vitest | 139 | 115 | 24 | 0 | 0 |
| bun-test | 557 | 505 | 52 | 0 | 0 |
| vite (build, activate, dev, serve) | 1582 | 322 | 626 | 634 | 0 |
| playwright (browser/e2e) | 13 | 0 | 11 | 2 | 0 |
| go | 27 | 25 | 1 | 1 | 0 |
| python | 12 | 4 | 8 | 0 | 0 |
| dotnet | 4 | 2 | 2 | 0 | 0 |
| latex (print, reports) | 19 | 11 | 2 | 6 | 0 |
| docker | 7 | 0 | 7 | 0 | 0 |
| wasm (bzl-rust) | 963 | 24 | 939 | 0 | 0 |
| cargo (bzl-rust) | 3029 | 1975 | 1041 | 13 | 0 |
| aggregate (noop) | 1 | 1 | 0 | 0 | 0 |
| **total** | **8027** | **3846** | **2870** | **1308** | **3** |

No-cache reasons: `driver-owned` 1560 (playground `prepare-*` and `build-*`), `live` 624 (native `run-*`, `smoke-*`), `side-effecting` 313 (`activate-*-wgpu`, CDN publish), `declared` 373 (`cache: false` in the manifest). Fail-closed: `generate-wgpu`, `generate-scale-fixture`, `generate-distribution` (`cache: true`; their outputs are injected by project-assembly from taxonomy generator contracts, which the static harness does not run, so in the real graph they are cacheable; see `result-uncached-outputs.md`).

### After (adapter with the staged set)

| family | tasks | cached action | cached test (`bazel test`) | interactive (`bazel run`) | uncached | refused |
|---|---:|---:|---:|---:|---:|---:|
| bun-script | 1639 | 828 | 0 | 652 | 156 | 3 |
| tsc | 35 | 34 | 0 | 0 | 1 | 0 |
| vitest | 139 | 2 | 113 | 0 | 24 | 0 |
| bun-test | 557 | 16 | 489 | 0 | 52 | 0 |
| vite | 1582 | 322 | 0 | 634 | 626 | 0 |
| playwright | 13 | 0 | 0 | 2 | 11 | 0 |
| go | 27 | 4 | 21 | 1 | 1 | 0 |
| python | 12 | 2 | 2 | 0 | 8 | 0 |
| dotnet | 4 | 2 | 0 | 0 | 2 | 0 |
| latex | 19 | 11 | 0 | 6 | 2 | 0 |
| docker | 7 | 0 | 0 | 0 | 7 | 0 |
| wasm (bzl-rust) | 963 | 24 | 0 | 0 | 939 | 0 |
| cargo (bzl-rust) | 3029 | 811 | 1164 | 13 | 1041 | 0 |
| aggregate (noop) | 1 | 1 | 0 | 0 | 0 | 0 |
| **total** | **8027** | **2057** | **1789** | **1308** | **2870** | **3** |

Non-Rust-family subtotal (everything except cargo, wasm, aggregate): 4,034 tasks, before: 1,846 cached, 890 no-cache, 1,295 run target, 3 fail-closed; after: 1,221 cached action + 625 cached test, 1,295 interactive, 890 uncached, 3 refused. The 3,846 cached tasks are the same set before and after (2,057 stay build actions, 1,789 become Bazel tests). The tests of Rust projects (1,164) are included because the rule is by name, not by family: `bzl-rust` should confirm that cargo `test` targets may be Bazel tests (they are long and lease-bound; `timeout = "eternal"` is set) before the set is applied, or exclude them with one rule row.

## What stays uncached, per non-Rust family, and the lever

| Family | Tasks uncached | Why | Lever |
|---|---|---|---|
| vite (626) | 312 `driver-owned` (`build-<variant>-react`, release builds), 313 `side-effecting` (`activate-<variant>-wgpu`, CDN publish), 1 `declared` | playground chain; outputs live in the leased products store behind a publication protocol and depend on the wasm closure | `bzl-rust` (wasm closure) plus a store-aware restore; not an adapter-only change |
| bun-script (156 declared + 3 refused) | 156 `cache: false` | mostly host-state commands (`setup`, `deps-*`, `clean`, `publish`, `bench`, `backend`, `doctor`, git tooling); 24 have a verification verb | 24 owner candidates in `bzl-coverage-owner-candidates.md`; the 3 refused need the generator-contract output injection (real graph) |
| bun-test (52) and vitest (24) | `cache: false` | 43 + 23 are `test`/`canonical-architecture` targets, mostly tests of the build tool itself (leases, storage, process trees: `framework-process:test-*`, `repo:test-*`, `repo-lib:test-*`) which read or write host state | owners re-declare inputs/outputs, then they run as cached `bazel test` with no further adapter work |
| playwright (11) | `cache: false` | live end-to-end against a running server or a docker stack | stays uncached by nature; a hermetic browser test (fixture below) is cacheable as `bazel test` |
| python (8), dotnet (2), docker (7) | `cache: false` | toolchain installs (`deps`, `oracle-setup`), docker daemon builds and publishes | intrinsic (managed store or daemon state) |
| latex (2), go (1), tsc (1) | `cache: false` | `deps-tectonic`, `deps-tex` toolchain installs; one go and one tsc candidate | owner list |
| wasm (939), cargo (1041) | `driver-owned`, `live`, `declared` | cargo/wasm | `bzl-rust` |

Neutral summary of the uncached non-Rust-family tasks: 625 playground (`bzl-rust` chain), 172 host-state by name, 93 owner candidates (cacheable verb, `cache: false`).

## Design (neutral contract first, schema-first, no compat layer)

- `B/🏃️execution/🧬️schema/🎯️coverage.json` (new, closed): `Family` (14 values), `Disposition` (`action | test | interactive | uncached | refused`), `Reason` (the 11 cache-decision reasons), `Rules` (ordered family rule table as data, last row is the total `bun-script` default), `Policy` (the `test` target pattern; `continuous` maps to `interactive`; `undeclared` and `unbounded` map to `refused`), `Input`, `Report`. The text maps each disposition to its implementation: Nx runs all as `nx run`; Bazel maps `action` to a cached build action, `test` to a cached test result, `interactive` to an uncached `bazel run` target, `uncached` to `no-cache` with its reason, `refused` to fail-closed.
- `B/🏃️execution/🟦️.ts` (new, pure; six types and four functions): `taskFamilyV1`, `taskDispositionV1`, `taskCoverageV1`, `taskCoverageReportV1`. Regular expressions use only syntax shared by JavaScript and Python.
- `BZ/📜️script.ts` (13 anchored edits): `bazelTaskCoverageV1`, `bazelCoverageReportV1`, `BazelActionV1.kind` gains `"test"`; `bazelActionsV1` turns a selected, unconsumed, cached task whose disposition is `test` into `kind: "test"` (a test whose snapshot another action restores stays a `repo_task`, so dependants keep working); `STARLARK_RULES` gains `repo_test` (`test = True`, name ends in `_test`, launcher built by the same birth protocol, runfiles = manifest, fingerprint, runner, implementation, dependency snapshots and declared sources, `testing.ExecutionInfo(no-sandbox, no-remote-exec)`, `timeout = "eternal"`, `args` = runner, implementation, identity, `birth-test`, manifest, dependency snapshots); the BUILD renderer emits it; fingerprints and declared sources include tests; birth grants for the `test` kind use the runfiles paths (`<bin>/<label>.runfiles/_main/<dep>.snapshot`); the owner runs `bazel test` (with `--test_output=all` for one test, `errors` otherwise, `--test_summary=short`, `--enable_runfiles` for Windows) instead of `bazel build` when a selected action is a test; the captured runner accepts `test` as a birth/run kind (a task run without snapshot capture).
- `BZ/🧬️schema/🔣️.json`: `ActionBirth.kind` gains `test` (the contract pin of `ActionBirth` in `📋️contract` is updated: `690de8a1...` -> `d8221916...`).
- The three tests consuming the old shape are updated (actions fixture: `app:test` is now `repo_test`; actions test counts; actions fixture schema kind enum).

## Staged set and how to apply

`C/bzl-coverage-tools/` : `edits.py` (single source: new files and anchored edits), `staged/` (8 new files), `apply.py` / `apply.sh` (`--dry-run`, `--rebase`, `--root`, `status`; refuses on content drift against `bases.json` and on any anchor that does not occur exactly once; idempotent), `bases.json`.
Targets: new `B/🏃️execution/🧬️schema/🎯️coverage.json`, `B/🏃️execution/🟦️.ts`, `B/🏃️execution/🧪️tests/🎯️coverage/{🟦️.ts,🔣️.json,🧬️schema.json}`, `BZ/🧪️tests/📥️coverage/{🟦️.ts,🔣️.json,🧬️schema.json}`; edited `BZ/📜️script.ts`, `BZ/🧬️schema/🔣️.json`, `BZ/🧪️tests/📥️actions/{🔣️.json,🟦️.ts,🧬️schema.json}`, `BZ/🧪️tests/📋️contract/🔣️.json`.
`bash C/bzl-coverage-tools/apply.sh --dry-run` against the live checkout: 13 + 1 + 4 + 1 + 1 + 1 edits and 8 new files, rc 0, no write. After "PAUSE LIFTED": re-run `--dry-run`, then `apply.sh` (use `--rebase` if `bworkers`, `rexec`, `bzl-lazy` or `bzl-rust` edited `BZ/📜️script.ts` meanwhile; the anchors are one-line fragments).

## Verification

Overlay: `C/bzl-coverage-tools/overlay.sh` builds `/private/tmp/bzlcov-ov` (copy of 4187 files, listed by a bundler metafile of the adapter and its tests plus the process, schema and library directories: the adapter import closure, tests, process and schema modules; `ajv`, `typescript`, `minimatch` and `@iarna/toml` with their dependencies copied into the overlay `node_modules`, not linked; its own `git init`). It is outside the ticket tree because the captured Source path limit is 256 units. Bazel ran through the production owner (workspace generation, control channel, action births, captured implementation, snapshot restore) via the existing scratch-host entry, extended with `bazel test` and `bazel run`.

### Unit and fixture tests (overlay, explicit file paths, one runner at a time)

| Suite | Result |
|---|---|
| `🏃️execution/🧪️tests/🎯️coverage` (new) | 5 pass: closed fixture + schema, ordered total rules, 53 authored rows (every family and disposition reachable), independent Python rule evaluator agrees on all rows, SQLite census agrees with the report counts |
| `BZ/🧪️tests/📥️coverage` (new) | 6 pass: closed fixture, kinds per request (consumed test stays an action, skipped cache never a test, server is `launch`), neutral family and disposition of each task, report counts, Python `ast` parse of the rendered BUILD (`repo_test` args, timeout, labels) and of `STARLARK_RULES` (`repo_test` has `test = True`, name ends `_test`), birth kind admitted and `tests` refused |
| `📥️actions` | 11 pass, 1 skip; with the real binary 12 pass |
| `📋️contract`, `📥️sources`, `📥️workspace`, `📥️fingerprint`, `📥️delegation`, `📥️environment`, `📥️inventory` | 2, 8, 4 (5 with the real binary), 2, 1, 1, 2 pass; 0 fail |
| `tsc --noEmit` on the adapter, the classifier and the two new tests | 0 errors in these files (2 existing errors in `B/🕸️graph/🟦️.ts`: the overlay lacks a `.d.mts`) |
| Not run | `📦️snapshot` (needs `fast-glob`), `🔏️implementation`, `🕸️dependencies`, `📥️closure`, `📥️control`, the whole `🧪️tests/🟦️.ts` aggregate, `bun repo run workspace:test-build-tool-bazel` |

### Real Bazel, one real tool per family (fixture projects in the overlay; graph = 11 projects, `cache: true`, declared `outputs`, `runtime` tool-version inputs)

Tools: `bun` 1.3.14, `tsc` 5.9.3, `vitest` 4.1.12, `vite` 7.3.7, `playwright` 1.64.1 with the installed headless chromium 1234 (the version the package wants, 1248, is not installed here), `go` 1.25.0, `python3` 3.9.6 (unittest), `jco transpile` of a hand-assembled empty component for wasm. The latex sample is a stand-in (a Python script writing a PDF): no TeX engine is installed, so only the declared-output restore path is proven for that family. dotnet, docker and cargo were not run.

| Sample | Family | Executes as | Cold | Warm | After `bazel clean` |
|---|---|---|---|---|---|
| `fx-bun:generate` (`bun gen.ts` -> `out/`) | bun-script | `repo_task` | 1 local | hit | disk cache hit |
| `fx-tsc:typecheck` | tsc | `repo_task` (outputs `[]`) | 1 local | hit | disk cache hit |
| `fx-vitest:test` | vitest | `repo_test` | PASSED (launcher + 2 TestRunner local) | `(cached) PASSED` | disk cache hit, `(cached) PASSED` |
| `fx-buntest:test` (depends on `fx-bun:generate`) | bun-test | `repo_test` with a dependency snapshot in the runfiles | PASSED | `(cached) PASSED` | disk cache hit |
| `fx-vite:build` (`vite build` -> `dist/`) | vite | `repo_task` | 1 local | hit | disk cache hit |
| `fx-pw:test-e2e` (chromium, hermetic page) | playwright | `repo_test` | PASSED | `(cached) PASSED` | disk cache hit |
| `fx-go:test`, `fx-go:build` | go | `repo_test`, `repo_task` (`bin/`) | PASSED, 1 local | hit | disk cache hit |
| `fx-py:generate`, `fx-py:test` | python | `repo_task`, `repo_test` | 1 local, PASSED | hit | disk cache hit |
| `fx-latex:build` (stand-in) | latex | `repo_task` (`out/`) | 1 local | hit | disk cache hit |
| `fx-wasm:wasm` (`jco transpile`) | wasm | `repo_task` (`out/`) | 1 local | hit | disk cache hit |

Phases (`C/bzl-coverage-tools/matrix.sh`, final adapter, all 12 tasks selected together, 4 jobs; Bazel summary lines):
| Phase | Bazel summary | Elapsed | Spawns |
|---|---|---|---|
| p1 cold | `33 processes: 11 internal, 22 local`, 5 tests PASSED | 22.0 s | 22 local (7 tasks + 5 launchers + 10 TestRunner) |
| p2 warm | `1 process: 37 action cache hit, 1 internal`, 5 `(cached) PASSED`, `Executed 0 out of 5 tests` | 0.9 s | 0 |
| p3 outputs deleted (`dist/`, `out/`, jco `out/`) | same as warm; all three directories back | 0.9 s | 0 |
| p4 `bazel clean`, same disk cache | `33 processes: 22 disk cache hit, 11 internal`, 5 `(cached) PASSED` | 1.2 s | 22 disk cache hits, 0 executions |
| p5 whitespace-only change of the generator input | `2 processes: 36 action cache hit, 1 internal, 1 local` | 1.3 s | `fx-bun:generate` only; identical output = early cutoff, `fx-vite:build` and `fx-buntest:test` hit |
| p6 generator input value change | `4 processes: 33 action cache hit, 4 local` | 8.3 s | generate, vite build, bun test (launcher unchanged) |
| p7 Go source change | `3 processes: 34 action cache hit, 3 local` | 5.5 s | `fx-go:build` and `fx-go:test` only |
| p8 Python source change | `3 processes: 34 action cache hit, 3 local` | 2.4 s | `fx-py:generate` and `fx-py:test` only |
| p9 tsc source change | `2 processes: 36 action cache hit, 1 internal, 1 local` | 5.1 s | `fx-tsc:typecheck` only |
| p10 `fx-bun:publish` (`cache: false`) twice | `1 internal, 1 local` each | 1.4 s, 1.0 s | 1 local each; the log shows 2 executions |
| p11 failing test twice | `1 test FAILED`, exit 1 both times | 4.5 s, 1.1 s | re-executed (TestRunner x2) each time; failures are not cached; first run also built the launcher |
| p12 interactive `fx-dev:dev` twice | `bazel run`; `[bazel] task-started` each time | 1.5 s, 0.2 s | launcher built once, the server process starts on every run, never produced |

Undeclared input: a stray file written next to the projects (outside every declared pattern) changed nothing (0 spawns), the same undeclared-input blindness measured in `result-bzl-real.md`.

## Findings (for the other executors, not fixed here)

1. Bun's runtime transpiler cache is keyed by file content, not by the module's root: identical content under a second root (a scratch overlay, a second checkout, a devcontainer mount) resolves its relative imports to the FIRST root's absolute paths, and the captured loader then refuses them (`Uncaptured implementation module refused: /Users/ueli/Documents/semio/...`). Reproduced deterministically with `BUN_RUNTIME_TRANSPILER_CACHE_PATH` unset and cured with `=0`. Bazel actions get an empty or strict env, so the harness passes `--action_env`/`--test_env=BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` and the overlay sets `use_default_shell_env` on the launcher rules. For `rexec`: a hermetic Bazel action environment should set this variable itself (or `repo_task`/`repo_launch` an `env` attribute), otherwise the first run from any other root poisons the cache.
2. Task actions cannot run in a root without the native storage images: `runCapturedBuildSourceV1` opens `withRepositoryCacheStoreV1(root, "graph", ...)` unconditionally. The overlay replaces it with a pass-through (documented harness deviation); a real storage-free entry (`runBazelTasksOnHostV1`, open item 2 of `result-bzl-real.md`) would remove the deviation.
3. `repo_launch` and the new `repo_test` launcher action both copy the bun binary through the birth protocol; the launcher action takes an empty environment unless `use_default_shell_env` is set, which the staged `repo_test` sets.
4. The static census reports 3 fail-closed generators that are cacheable in the real graph: `generate-wgpu`, `generate-scale-fixture`, `generate-distribution` get outputs from taxonomy generator contracts in the real graph; re-run `census.ts` with the real graph in place of the reconstruction (`census.ts` reads `🗑️generated/bzl-coverage/graph.json`) once `show projects` is green.
5. By the `runViteBuild` docstring ("complete artifact publication" through `buildViteArtifact`) a site build publishes into the artifact store, not into a declared output directory: the course `build` target is `cache: false` with `outputs: []`, the demonstrator and play `build` targets are `cache: false` with one declared output. A cached Bazel `build` needs either a declared output directory that the publication step consumes or a store-aware restore; that is a design item shared with `bzl-rust`.
6. Windows: the test grants depend on a runfiles tree, which Windows does not create unless `--enable_runfiles` is on; the staged owner passes it. Not run on Windows or Linux.

## Open items

1. Apply after "PAUSE LIFTED" (`apply.sh`), then run the live suites (`📥️coverage`, `📥️actions`, `📋️contract`) in the real tree and `bun repo run workspace:test-build-tool-bazel`; confirm the taxonomy gate admits the two new directory names `🎯️coverage` (execution tests) and `📥️coverage` (Bazel tests) and the schema file `🎯️coverage.json`.
2. `bzl-rust`: decide whether cargo `test` targets (1,164) become `bazel test`.
3. Production end-to-end `bun repo --tool=bazel run X:test` with the storage lease (blocked while a dependency sync holds the image); `recordBazelBuildMetricsV1` was not exercised with test events.
4. `bzl-lazy`: expose the report (`bazelCoverageReportV1`) in `--dry-run` and the dashboard, and use the `interactive` disposition for the launch plan.
5. Owners: `C/bzl-coverage-owner-candidates.md` (93 tasks).
6. TeX, dotnet, docker and cargo families were not run.

## Changes (all under `C/`, no library edit)

`C/bzl-coverage-tools/`: `graph.ts` (static graph, copy of the bzl-breadth reconstruction writing to `🗑️generated/bzl-coverage`), `census.ts`, `edits.py`, `apply.py`, `apply.sh`, `bases.json`, `overlay.sh`, `harness.sh`, `nodemods.py`, `fixtures.py`, `scratch-host.part.ts`, `scratch-run.ts`, `go.sh`, `bz.sh`, `exec-summary.py`, `matrix.sh`, `test.sh`, `staged/*` (8 files). `C/bzl-coverage-owner-candidates.md`. This report.

Reproduce: `bun C/bzl-coverage-tools/graph.ts`; `C/bzl-coverage-tools/overlay.sh --rebuild`; `C/bzl-coverage-tools/harness.sh`; `BZL_COVERAGE_MODE=before|after BZL_COVERAGE_ADAPTER=<adapter> BZL_COVERAGE_CLASSIFIER=<coverage.ts> bun C/bzl-coverage-tools/census.ts <out.json>`; `C/bzl-coverage-tools/test.sh [real-bazel] <test path in the overlay>`; `C/bzl-coverage-tools/matrix.sh`.

## Cleanup

All Bazel servers shut down (`ps` shows none for this output base); `/private/tmp/bzlcov*` (overlay, output base, install base, disk cache, logs, probes), `C/🗑️generated/bzl-coverage*` and the Python bytecode caches removed. No file under the checkout outside `C/` was written; nothing was committed.
