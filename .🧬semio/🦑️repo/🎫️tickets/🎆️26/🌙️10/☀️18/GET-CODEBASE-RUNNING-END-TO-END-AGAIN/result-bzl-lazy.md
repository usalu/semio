# result-bzl-lazy: lazy minimal Bazel work for every neutral operation, `bazel test` for tests, Bazel-native `affected`

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `B` = `L/🏗️build`. `BZ` = `B/🔧️tools/🏗️bazel`. Bazel 8.4.2 (pinned binary), macOS 27 arm64, load average 35-65 during every measurement (wall times are load dependent; counts are not). Stage-only: nothing outside `C/` and `/private/tmp/bzlazy` was written (write pause: dependency sync was running). Everything is under `C/bzl-lazy-tools/`; `staged/` is the apply set, `apply.py` applies it.

## Verdict

1. **Laziness holds by construction and was proven on 16 targets.** A selection generates a workspace that contains only the planner closure of the requested tasks; `bazel cquery 'kind("repo_", deps(//:t))'` equals the planner closure exactly for all 16 (True in every row), `aquery` counts equal the closure size, and in a workspace holding all 8,027 tasks Bazel still executes only the closure (`play-activate`: 108 actions, 107 executed, `aquery` 107, versus 8,027 declared). Analysis and input hashing scale with the declared input files of the closure, never with the repository.
2. **Where the cost really is:** 244 of the 3,845 cacheable tasks (6 %) declare the whole repository (122,167 files) as input, 240 of them in the root `workspace` project; `os-hub-ts:typecheck` declares 63,770 files (134,165 configured Bazel targets). Those tasks cost 5-17 s of analysis, 12+ s of input hashing even on a cache hit, and re-key on any edit. This is a manifest problem, not an adapter problem; owners and numbers below.
3. **Package granularity was measured and rejected** (negative result): moving the source filegroups of a 20 MB BUILD into 16 hash-sharded packages cut load from 4.3 to 2-2.9 s and total cold analysis from 19.8 to 15-18 s (A/B/A/B, same load). Bazel requires a source file to be named in its own package, so every file needs an `exports_files` entry in the root package; the remaining cost is per input-file target. Not worth the complexity; not implemented. No `--experimental_*` flag is used anywhere.
4. **Tests are `bazel test` targets with cached test results** (staged): 1,696 of the 1,817 cacheable test-family tasks (93 %) become `repo_test` (1,816 declare empty `outputs`). Verified with real Bazel and a recording runner: exact birth paths (equal to `bazel info bazel-bin`), `(cached) PASSED` on the second run, rerun under `--nocache_test_results` and on a changed own source, early cutoff when only a dependency's source changes (its snapshot is unchanged), disk-cache restore after `bazel clean` without a rerun, exit code 3 and `FAILED` for a failing test. Larger tests stay cacheable build actions because Bazel links every runfile (measured below).
5. **`affected` is answered by Bazel `rdeps`** and agrees with the neutral rule and the installed Nx's own locators on 4 of 5 realistic change sets (293, 22, 209, 9 projects, 1,466-project graph); the fifth, a lockfile-only change, diverges by design (documented).
6. The neutral operation to Bazel command mapping is a schema, a language-agnostic fixture (19 cases + 5 refusals + 12 classification rows + 5 limit rows), a Python implementation as oracle and Bazel's own binary as oracle (all mapped invocations accepted, test result caching asserted).

## Measured: representative targets (stub runner, real graph reconstruction, real Bazel)

The graph is the static reconstruction of 1,466 projects / 8,027 targets (`bzl-lazy-tools/graph.ts`; no Nx-inferred import edges, so closures are lower bounds). The workspace is generated per target by the real, live adapter (`generateBazelWorkspaceV1`, filegroups, no `.bazelignore`); only the action body is replaced by a `run_shell` stub that writes the snapshot directory (a real run would execute task code in the checkout, forbidden during the write pause), so cache keys, inputs, restore edges and scheduling are the real ones. Phases: cold = fresh output base and empty disk cache; warm = same server; after clean = `bazel clean` keeps the disk cache; after server loss = server and output base removed, disk cache kept. "Input file targets" = configured Bazel targets of the cold analysis (files plus filegroups). "Generate" = captured-implementation snapshot plus workspace files (I/O bound; 0.7-21 s for the snapshot under this load).

| target | area | closure (cached) | input file targets | generate ms (impl+gen) | cold load+analyze ms | warm server wall ms | cold build: actions / executed | warm: executed | after `clean`: executed / hits | after server+output-base loss: executed / hits |
|---|---|---:|---:|---:|---:|---:|---|---:|---|---|
| `@semio-tech/framework-graph:generate` | framework | 1 (1) | 144 | 6243 | 969 | 588 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `@semio-tech/framework-graph:test` | framework | 2 (2) | 148 | 22642 | 2048 | 1742 | 3 / 2 | 0 | 0 / 2 | 0 / 2 |
| `@semio-tech/layout-layout-rs:test-media-adoption-source` | s plugin | 1 (1) | 132 | 16285 | 2670 | 2405 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `@semio-tech/writer-writer-rs:test` | s plugin | 2 (2) | 5263 | 9258 | 8361 | 790 | 3 / 2 | 0 | 0 / 2 | 0 / 2 |
| `@semio-tech/writer-writer-rs:build` | s plugin | 2 (2) | 1402 | 13926 | 2710 | 698 | 3 / 2 | 0 | 0 / 2 | 0 / 2 |
| `@semio-tech/ui-styling:generate` | framework | 2 (2) | 8149 | 13231 | 5444 | 672 | 3 / 2 | 0 | 0 / 2 | 0 / 2 |
| `os-hub-ts:test` | hub | 1 (1) | 13907 | 9317 | 7512 | 808 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `os-hub-ts:typecheck` | hub | 1 (1) | 134165 | 49359 | 41673 | 2415 | 2 / 1 | 1 | 0 / 1 | 1 / 0 |
| `@teaching/architecture-course:test` | teaching | 1 (1) | 6185 | 15792 | 3210 | 502 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `@teaching/architecture-course:typecheck` | teaching | 7 (7) | 6757 | 6923 | 3913 | 640 | 8 / 7 | 0 | 0 / 7 | 0 / 7 |
| `@teaching/architecture-course:build` | teaching | 3 (2) | 1343 | 5022 | 1377 | 175 | 4 / 3 | 0 | 1 / 2 | 1 / 2 |
| `@teaching/proctor:test` | teaching | 1 (1) | 401 | 9978 | 1616 | 407 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `@semio-tech/mit-bestand-bericht:akteursnetz-validate` | mit-bestand | 1 (1) | 1350 | 2722 | 1799 | 264 | 2 / 1 | 0 | 0 / 1 | 0 / 1 |
| `@semio-tech/mit-bestand-demonstrator:build` | mit-bestand | 24 (13) | 2697 | 2510 | 3722 | 562 | 25 / 24 | 0 | 11 / 13 | 11 / 13 |
| `@semio-tech/mit-bestand-demonstrator:activate-prototype` | mit-bestand | 28 (18) | 2709 | 7134 | 4770 | 1040 | 29 / 28 | 0 | 10 / 18 | 10 / 18 |
| `@semio-tech/semio-tech-play:activate-prototype` | semio-tech | 107 (72) | 2914 | 5892 | 3058 | 300 | 108 / 107 | 0 | 35 / 72 | 35 / 72 |

Reading it: warm = 0 executed everywhere; after `clean` and after total server loss every cacheable task is a disk-cache hit and only the by-design uncached tasks (`no-cache`: 11 of 24, 10 of 28, 35 of 107) run again; cold analysis is 1-8 s except the 134k-file typecheck; a warm server answers in 0.2-2.4 s. Exceptions explained: `os-hub-ts:typecheck` rows 'warm: 1' and 'server loss: 1 / 0' are real invalidations, not cache defects: the run printed `Skipping uploading outputs because of concurrent modifications` once, and 134k declared inputs in a checkout that other fleets edit change between runs (an earlier controlled run of the same target hit 0 executed / 1 hit after `clean` and after server loss); `architecture-course:build` itself is `cache: false` (declared), its two dependencies are hits.

Cross-selection cache (separate workspace directories, separate servers and output bases, one shared disk cache): selection `framework-graph:test` (2 tasks) then selection `framework-graph:generate` (its dependency): second selection `2 processes: 1 disk cache hit, 1 internal`, 0 executed. Selections share results through the disk cache only (and will through the remote cache), as designed: nested selections need their own output base or they would deadlock on Bazel's output-base lock.

## Whole repo versus closure (analysis does not scale with the repo)

| workspace | target | BUILD size | cold load ms | cold analyze ms | cold wall ms | configured targets |
|---|---|---:|---:|---:|---:|---:|
| closure workspace | `play-activate` (107 tasks) | 417 KB | 1,372 | 1,686 | 19,697 | 2,914 |
| universe (all 8,027 tasks, generate 60 s, `implementation` 1.7 s) | `play-activate` | 65.7 MB | 7,225 | 928 | 17,181 | 2,910 |
| universe | `//:all` | 65.7 MB | 6,777 | 14,144 | 27,896 (warm repeat 7,975) | 251,023 |
| closure workspace | `framework-graph:generate` | 14 KB | 219 | 750 | 11,920 | 144 |
| universe | `framework-graph:generate` | 65.7 MB | ~12,000 (profile, first run) | 410 | 18,737 | 127 |

Bazel itself is lazy (building one label in the universe executes only its closure), but loading one 65 MB package costs 7-12 s however small the target; the per-selection closure workspace removes that and also the 52 s generation of the universe. Generation of a single small closure: 0.1-3 s plus the captured-implementation snapshot (0.7-3 s unloaded, up to 21 s under load 60; I/O bound: `lstat` of every ancestor of every captured source, twice per file, by design).

Package sharding A/B/A/B on `os-hub-ts:typecheck` (134k configured targets, 20 MB BUILD, same machine state): unsharded cold wall 19.9 s / 19.7 s (load 4.3 s, analyze 7.2-7.4 s); 16 shards plus root `exports_files` 18.4 s / 15.1 s (load 2.9 / 2.0 s, analyze 5.9 / 6.8 s). Rejected, see verdict 3.

## Declared-input census (the real cost driver; owners)

`C/bzl-lazy-tools/inputs-census.ts` expands every cacheable task's inputs with the adapter's own `bazelTaskInputsV1` and source index over the real inventory (`git ls-files -co`: 122,167 files).

| metric | value |
|---|---:|
| cacheable tasks | 3,845 |
| median / p75 / p90 declared files | 248 / 3,201 / 11,710 |
| tasks with >= 1,000 / 10,000 / 50,000 / 100,000 files | 1,431 / 399 / 255 / 244 |
| tasks declaring the whole inventory | 244 (240 in the root `workspace` project) |

Beyond `workspace`: `repo-lib:test-fixture-placement` and `test-mutation-fixture-placement` 121,667; `s-plugin-composition:test` 117,288; `s-dev-composition:typecheck` 105,542 and `generate` 91,035; `hub-stdio-test-oracle` 7 tasks 89,058-99,360; `stdio-plugin:home-io-surface-native-check` 77,114; `os-hub-ts:typecheck` 63,770 (inputs `{workspaceRoot}/🌎️hub/**/*.{ts,tsx,json}`, `🧰️framework/**/*.{ts,tsx,json}`, `✏️s/**/*.{ts,tsx,json}`). Products: `os-hub-ts:test` 7,394; `layout-layout-rs:test` 6,679; `architecture-course:test|typecheck` 3,283; `writer-writer-rs:test` 2,922; `mit-bestand-demonstrator:activate-prototype` 1,267; `semio-tech-play:activate-prototype` 1,252; `proctor:test` 194. Full list: regenerate with `bun C/bzl-lazy-tools/inputs-census.ts` (output `C/🗑️generated/bzl-lazy/inputs-census.json`). Action for owners: narrow these globs to the files the task reads (each is also a remote-execution upload set).

## Operation to Bazel command mapping (neutral contract, schema `BZ/🧭️commands/🧬️schema/🔣️.json`)

| neutral operation | Bazel | evidence |
|---|---|---|
| `run <p:t>` | closure workspace; one `bazel build` or `bazel test` of the selected labels; services (`continuous`) via `bazel run` as before | mapping fixture, 16-target closure proof |
| `run-many` | same, one invocation with all labels; shared closure tasks built once | fixture cases 2, 5, 14; `cquery` equality |
| `affected` | `selectBuildTasksV1` takes the affected projects from `bazelAffectedProjectsV1`: one generated package of project and changed-file nodes, one `bazel query 'rdeps(//:all, set(changed))'`; then as `run-many`. `--dry-run`/`--graph` use the neutral TS rule (no Bazel process); `--all` consults nobody | 5 change sets, 7 extra fixture cases |
| `exec` | one uncached `repo-exec` task per project, one `bazel build` | fixture case 4, `BZO` tests (plan, env, cwd) |
| `--dry-run` / `--graph` | neutral `DryRunPlan` from the planner, no execution; the same graph `cquery` returns | `BZO` tests (12 pass with real Bazel), 16 closure equalities |
| `cache status` / `cache clear` | disk census / server stop plus output bases plus cache removal, no `bazel` verb | `BZO` real-binary tests |
| `watch`, `graph`, `show` | no Bazel | unchanged |

Options for the one execution invocation (after the opaque `base` list owned by the cache contract: lockfile, strict env, disk cache and, after rcache applies, remote flags): `--jobs=N`, `--keep_going` (not with `--bail`), `--subcommands` (verbose), `--build_event_json_file=`; for a batch with a test: `--test_output=errors|all`, `--cache_test_results=auto` or `--nocache_test_results` (skip-cache), `--notest_keep_going` (bail), and `--build_tests_only` only when every selected label is a test (otherwise the non-test tasks would not be built). Sharding: `unsupported` (constant in the schema, asserted): the neutral contract defines no shard protocol, no target declares shards, and `shard_count` is never emitted. Download mode is not touched: `bazelCacheFlagsV1` (rcache) owns `--remote_download_*`; tests need top-level outputs (test logs), which `toplevel` provides, and rcache already refuses `minimal`.

## `repo_test`: what it is and where it stops

A task action becomes a Bazel test iff all hold (`bazelTestActionV1`): cacheable, name `test` or `test-*`, `outputs` declared as an empty array, no other planned task depends on it, and (recorded at generation in `kinds.json`) at most 10,000 declared source files. The rule writes a launcher that computes `bin=$(dirname "$TEST_SRCDIR")` and execs the same `birth-action` argv as the build rule with the exact absolute `bin/<label>.snapshot` paths, so the control-channel grants need no change; `timeout = "eternal"`; execution requirements as the task rule (`no-remote-exec`, `no-sandbox`); results are cacheable in the local and disk/remote cache.

Why the 10,000-file limit (measured with the stub, real rules): every declared source is a runfile; Bazel links them before every test run, cache hits included. `os-hub-ts:typecheck` (134k configured targets) as a test: cold execution 31.6 s (build rule 25.0 s), after `clean` (pure disk-cache hit) 29.5 s versus 12.3 s: about 0.13 ms per runfile, 17 s extra. `architecture-course:test` (6,185): 0.8 s versus 0.4 s on a cache hit. 120 of the 1,816 eligible tests (7 %) exceed the limit and stay cacheable build actions.

## `affected` cross-check (1,466 projects; change sets frozen from the working tree against HEAD by `change-sets.py`)

| set | files | touched directly | neutral | Bazel `rdeps` | Nx locators | equal |
|---|---:|---:|---:|---:|---:|---|
| framework build library | 913 | 178 | 293 | 293 | 293 | yes |
| layout + writer plugins | 81 | 16 | 22 | 22 | 22 | yes |
| root manifests, teaching, mit-bestand (no lockfile) | 34 | 152 | 209 | 209 | 209 | yes |
| one hub file | 1 | 3 | 9 | 9 | 9 | yes |
| `bun.lock` only (known divergence) | 1 | 9 | 21 | 21 | 1,466 | neutral = Bazel, Nx differs |

Timing: Bazel generate 2-23 ms, query on a warm server 54-160 ms (cold JVM 7.9 s once); neutral 8-632 ms; Nx 0.6-43 s. The Nx oracle is the installed Nx's own `filterAffected` with JSON-aware `jsonDiff` file changes (`git show HEAD:file` versus the working tree), run in a scratch workspace where changed paths exist (otherwise its project-glob locator reads them as deleted and returns every project). Divergence explained: for a lockfile change Nx diffs the resolved dependencies against its external npm nodes; the oracle graph has none, so Nx falls back to "all"; the neutral rule and Bazel know only path ownership. Recommendation (not implemented, shared rule): treat a lockfile or root `package.json` dependency change as affecting every project whose target inputs declare `externalDependencies` (or all). Bazel actions already re-key on the lock through their declared bundled inputs, so execution stays correct; only the affected selection under-approximates.

## Changes

New (destination under `BZ/`, staged flat names in `C/bzl-lazy-tools/staged/`):
- `commands.ts`, `commands-schema.json`, `commands-test.ts`, `commands-fixture.json` -> `🧭️commands/{🟦️.ts, 🧬️schema/🔣️.json, 🧪️tests/🟦️.ts, 🧪️tests/🧫️fixtures/🔣️.json}`: `bazelCommandPlanV1`, `bazelTestActionV1`, `bazelTestTargetV1`, `bazelTestWithinLimitV1`, `bazelTestTimeoutV1`.
- `affected.ts`, `affected-schema.json`, `affected-test.ts`, `affected-fixture.json` -> `🎯️affected/…`: package renderer, `rdeps` expression, output parser.
- `testrule-test.ts` -> `🧪️tests/📥️test-actions/🟦️.ts`.

Modified live files (full-file copies built by anchored edits from the CURRENT live text, `build-staged.py` + `patches.py`, base hashes in `staged/bases.json`):
- `BZ/📜️script.ts` (`script.ts`): `BazelActionV1.kind` gains `"test"`; `bazelActionsV1(…, tests?)`; `repo_test` rule and `timeout`/reason in `renderBazelBuildV1`; `kinds.json` written by `generateBazelWorkspaceV1` and read by the runner (`readBazelTestsV1`); the run command goes through `bazelCommandPlanV1` (`bazel test` when a test is selected, labels of services excluded); `bazelStartupOptionsV1` shared by run and affected; `bazelAffectedProjectsV1`; `bazelStarlarkRulesV1`.
- `BZ/🧬️schema/🔣️.json` (`schema.json`): `Actions.kinds.test`, `ActionPolicy` kind, `Kinds` definition.
- `B/🧭️selection/🟦️.ts` (`selection.ts`): `affectedTouchesV1` (the rule's first half, keeps touching files), `affectedBuildProjectsV1` (closure over dependents, same answer), `selectBuildTasksV1(…, affectedProjects?)` with `--all` not consulting it.
- `B/📜️script.ts` (`neutral.ts`): the Bazel branch passes `bazelAffectedProjectsV1` for executions and the neutral rule for `--dry-run`/`--graph`.
- `B/⚙️operations/🔣️catalog.json` (`catalog.json`): two notes (run, affected).
- Existing tests whose expectations changed deliberately: `BZ/🧪️tests/📥️actions/{🔣️.json,🧬️schema.json,🟦️.ts}` (`app:test` is now `repo_test`; kind enum; rule counts), `BZ/🧪️tests/📤️arguments/🟦️.ts` (build call label `"bazel:"+batch.verb`), `B/🧭️selection/🧪️tests/🟦️.ts` (function list gains `affectedTouchesV1`).

Tools (kept): `graph.ts`, `gen.ts`, `gen-all.ts`, `closures.ts`, `run.ts`, `measure.py`, `matrix.sh`, `matrix-all.sh`, `table-md.py`, `proto-test.py`, `xsel.py`, `shard-build.py`, `inputs-census.ts`, `change-sets.py`, `affected-xcheck.ts` (+ `-lib.ts`), `affected-bisect.ts`, `prep-run.py`, `prep-suites.py`, `prep-selection.py`, `run-suites.sh`, `build-staged.py`, `patches.py`, `apply.py`.

## Apply (after PAUSE LIFTED)

`python3 C/bzl-lazy-tools/apply.py --dry-run`, then `python3 C/bzl-lazy-tools/apply.py` (refuses on drift against `staged/bases.json` and while a dependency sync/lock runs; after drift run `python3 C/bzl-lazy-tools/build-staged.py` first, it re-applies every anchor to the current text and fails if one disappeared). Then, explicit paths only:
`SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel bun test ./BZ/🧭️commands/🧪️tests/🟦️.ts ./BZ/🎯️affected/🧪️tests/🟦️.ts ./BZ/🧪️tests/📥️test-actions/🟦️.ts ./BZ/⚙️operations/🧪️tests/🟦️.ts`, `bun test ./B/🧭️selection/🧪️tests/🟦️.ts`, and the adapter suites under a minted envelope (`C/bzl-lazy-tools/run-suites.sh` shows the mint).

Integration note for `integrate` (it already folds this set via `apply.py` against a base copy): `bzl-coverage` stages the same idea (`repo_test` for the `test` disposition). The merge you wrote (`semantic.py`) keeps their classifier and my input limit, command plan and affected; the lines those patches anchor on are unchanged in the final set (`dependents` map, `const base=continuous?…`, `bazelTestWithinLimitV1(count)` statement, startup helper, affected startup line). Re-run `runset.py bzl-lazy` once: the staged files changed since 14:33 (`--all` handling, dry-run in `neutral.ts`, five test/fixture patches).

## Verification (exact)

Staged copies are runnable through `prep-run.py` (specifiers re-pointed under `C/bzl-lazy-tools/run/`, deleted at the end) and `prep-suites.py`/`prep-selection.py` (copies under `/private/tmp/bzlazy`, deleted).
- `bun test ./C/bzl-lazy-tools/staged/commands-test.ts` with `SEMIO_TEST_BAZEL_BINARY`: 5 pass (Ajv, Python oracle, real Bazel accepts all 19 mapped invocations; second test run `(cached) PASSED`, not cached under skip-cache).
- `… staged/affected-test.ts`: 6 pass (Ajv; neutral rule on 17 cases (10 from the Bazel-operations fixture plus 7 new); Python walk over the generated package; installed Nx's `filterAffected`; real Bazel `rdeps`; injected selection callback).
- `… staged/testrule-test.ts`: 3 pass (planner kinds, rendered BUILD parsed by Python `ast`, real Bazel run described in verdict 4).
- `bun C/bzl-lazy-tools/affected-xcheck.ts`: the table above. `bun …/inputs-census.ts`, `matrix-all.sh`, `table-md.py`, `xsel.py`, `shard-build.py`, `proto-test.py`: the measurements above.
- Existing adapter suites against the STAGED adapter with real Bazel (`run-suites.sh staged real-bazel …`): `📥️actions` 12 pass, `📥️sources` 10, `📥️workspace` 5, `📥️inventory` 2, `📥️delegation` 1, `📦️snapshot` 4, `📤️arguments` 3, `📥️closure` 9, `📥️fingerprint` 2 (`📥️control` fails identically against the live adapter copy: the relocated copy cannot resolve its relative process imports; not a regression). Selection suite against the staged selection: 6 pass.
- Live baseline, untouched tree: `SEMIO_TEST_BAZEL_BINARY=… bun test --timeout 400000 ./BZ/⚙️operations/🧪️tests/🟦️.ts`: 12 pass (real cache census/clear, dry-run and exec parity with Nx, affected versus `rdeps`).
- `tsc -p` (repo options, strict, types bun+node) over staged adapter, selection, neutral script, commands, affected: no error in a touched file (one pre-existing error in `📥️caller/📤️child/🟦️.ts`).
- `bun build --target=bun` of the staged adapter and neutral script: ok. `apply.py --dry-run`: 19 writes, no refusal.

## Not run / open items

1. Not run: production `bun repo --tool=bazel run|affected` (sync/lease), a real task body inside `repo_test` (births proven with a recording runner and exact paths only), `bazelAffectedProjectsV1` end to end through storage and leases (type-checked; its query, parser and package are tested), sandboxed or remote execution, Windows and Linux (the launcher is `#!/bin/sh`; bzl-coverage notes the runfiles requirement on Windows), the aggregate `BZ/🧪️tests/🟦️.ts` and `🕸️dependencies` (needs `tools setup nx`).
2. All executions in the matrix use a stub action body; real task bodies write the checkout. Repeat `matrix.sh` with the real runner once the freeze lifts (`bzl-real-tools/go.sh` shows the scratch host).
3. Generation fixed cost: the captured-implementation snapshot is rebuilt on every run (0.7-21 s, I/O bound, security-hardened by design). A stat-signature memo of the ~1,000 captured files would make warm runs generation-free; left to the owner of `📥️source/🔏️closure` because it changes an anti-TOCTOU module.
4. Lockfile semantics of `affected` (above); static graph lacks Nx import-inferred edges (rerun `graph.ts` replacement with the real `show projects` output).
5. Owners: the 244 whole-repo tasks and the 255 tasks over 50,000 files (list above).
6. Sharding of tests: define a neutral protocol first (index/total environment names and a runner contract); until then none.
7. rcache coordination: nothing in this set touches remote or download flags; with rcache applied `base` carries them unchanged, and `--remote_download_toplevel` (their default) keeps test logs available.

## Cleanup

Bazel servers shut down; `/private/tmp/bzlazy` (workspaces, output bases, disk caches, copies, the `node_modules` link) and `C/🗑️generated/bzl-lazy` removed; `C/bzl-lazy-tools/run/` removed (regenerate with `prep-run.py`).
