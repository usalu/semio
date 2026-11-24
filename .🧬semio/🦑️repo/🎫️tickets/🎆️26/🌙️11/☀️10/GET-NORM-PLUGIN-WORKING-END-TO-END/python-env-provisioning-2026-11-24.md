# Python environment provisioning for test runs (2026-11-24)

## Cause
- `SEMIO_PYTHON` resolution was already correct. `withProvisionedPythonV1` (toolchains `🟦️.ts`) finds the interpreter of the environment `<dependencies>/python/environments/workspace-<digest>/bin/python`. The Cargo test command (`🦀️cargo/🟦️.ts`) and the command configuration both call it. It returns nothing while that directory does not exist.
- Nothing ever created the directory outside `repo run workspace:deps-python`.
  - That target is only a `dependsOn` of the Python test targets.
  - `repo cargo ... test` bypasses the target graph and never runs it.
  - `repo dependencies ensure` and `repo dependencies sync` handled JavaScript only. `--scope` takes a JavaScript dependency-scope directory, so `python` is rejected as an owner.
  - `repo tools setup all` only installs the CPython pins (`python-oracle 3.12.13`, `python-main`), not the locked `uv sync` environment.
  - The dependency gate skipped `ensure` whenever `node_modules` existed.
- `repo run workspace:deps-python` has to read the project graph slice (about 3600 owners) before its one action (`repo <native/📜️script.ts> sync python`) runs. That script only works inside the invocation envelope of the `repo` run engine, so running it directly fails with "Original parent Script process invocation required".

## Why the graph-slice read stalled
- I could not reproduce a full freeze. A fresh `repo run workspace:deps-python` kept progressing under load 10 to 19.
  - It took more than 17 minutes and was still in the owner round (856 of 3600), at about 1 owner per second.
  - The 6200 s run is consistent with repeated inventory rounds, each waiting up to `sliceRestartQuietMaximumMilliseconds` = 600000 ms for the planner source tree to go quiet.
  - The tree is edited by 50+ replicas, so it never goes quiet. The "inventory" label's `boundMilliseconds: 600000` only labels the report. It does not cap the wait.
- Leases were not the cause: `repo leases status` showed 0 waiting and 0 deadlock cycles.
- The graph read is therefore too costly and too fragile to be the only provisioning path. I did not change the graph read itself. Python provisioning no longer depends on it.

## Fix
`repo dependencies` now also provisions the Python environment, through the existing `NativeDependenciesScript` with `python`. The kernel's own invocation envelope is reused, so no graph read happens.
- `repo dependencies ensure` syncs Python when the checkout declares `pyproject.toml` and `uv.lock` and the environment is missing.
- `repo dependencies sync --all` always syncs Python after the JavaScript sync.
- The dependency gate (`dependencyGateV1`) treats a missing Python environment as "not available". Every command that requires dependencies, including `repo cargo`, first runs `repo dependencies ensure`. A fresh clone, or a lock change that yields a new environment key, therefore provisions itself with no manual step.
- `uv`, the CPython pin and the uv cache come from the existing pinned tooling (`pinnedUvEnvironmentV1`, `provisionInterpretersV1`), so the setup works the same on devcontainer, Windows, macOS and Linux.

### Changed files
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts`: added `pythonEnvironmentMissingV1`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📦️dependencies/🟦️.ts`: `ensure` and `sync --all` run the Python sync.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🥾️provision/🟦️.ts`: the gate's `available` also requires the Python environment.

## Verification
- Killed only the stuck `repo run workspace:deps-python` processes (pid 54294, then my own re-run).
- `repo dependencies ensure` completed. uv installed the locked packages into `.cache/b/d/m/python/environments/workspace-3b97251b6599d245`.
- The interpreter there is `bin/python` 3.14.4 with numpy 2.5.0, scipy 1.18.0 and shapely 2.1.2 (the scene oracle's libraries).
- Oracle test: `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1997-scene --offline numerics_match` (log `🗑️generated/oracle-test.log`).
  - The command must run from inside the crate directory (`📦️packages/🦀️rust`); from the repo root, cargo reports the package as unknown (first log).
  - The second run (`🗑️generated/oracle-test2.log`, cwd = the crate) passed planning and was still queued at "waiting for compiler image headroom": other agents' compiles had filled the compiler image, and nothing was reclaimable. Its result is not recorded here.
  - Re-run it once headroom frees up.

## Follow-ups
- Make the inventory settle wait of the graph read cap its total time and fail with an actionable message.
- Add a law for `pythonEnvironmentMissingV1` and for the gate (the toolchains tests set `SEMIO_PYTHON`; the gate tests inject ports).

## Correction of the stall diagnosis
The first diagnosis named `settleInventoryRoundsV1` and `awaitPlannerQuietV1`. Neither is called anywhere outside tests, so they cannot cause the stall.

The wait that does exist is in the resident slice pool (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`).
- A slice worker pins the build tool's own source when it loads. When a peer edits that source, the worker ends with restart code 75.
- The pool then waits for the drifted files to be quiet for 60 s, at most 600 s, before it restarts the worker (`awaitImplementationQuietV1`).
- Under continuous edits by 50+ writers that wait runs to the maximum on every restart. The cap is 120 restarts and the same-cause guard only triggers on an identical cause, so a read can stall for hours: 10 minutes times many restarts, with the frozen counters of the symptom in between.

## Fix of the stall (root cause)
The quiet wait is now budgeted per load instead of per restart.
- New policy values in `🏡️resident/🧬️schema/🔣️.json`: `sliceRestartQuietTotalMaximumMilliseconds` = 600000 and `sliceRestartQuietFloorMilliseconds` = 3000 (typed in `📨️protocol/🎚️policy/🟨️.d.mts`).
- New `quietBudgetV1` in `🔕️quiet/🟨️.mjs`. While the budget lasts, each wait is capped at what is left. Afterwards only the 3 s floor is waited. The slice pool tracks `quietSpent` per load and applies this policy.
- The waits of one load therefore sum to at most about 10 minutes plus 3 s per further restart, however many edits arrive.
- Correctness is unchanged. Each worker pins the implementation it loaded. A later edit ends it with another drift restart, so only a worker whose pin held answers. A slice never mixes observations from before and after an edit. The shorter wait only risks a restart that has to restart again, which the existing same-cause (3) and total (120) bounds still cap.

## Tests added or changed
- `🏡️resident/🧪️tests/🔕️restart-quiet/🟦️.ts`: a new test with edits that never stop and four drift restarts. The first wait is the maximum, the second is what is left of the budget, and later ones are the floor only. The total stays below 800 ms, where unbudgeted waits would total about 1000 ms. The whole file passes (7 tests).
- `🧰️toolchains/🧪️tests/🟦️.ts`: new test for `pythonEnvironmentMissingV1` (no manifest, manifest and lock without interpreter, interpreter present, lock changed). Passes with the file (27 pass, 4 skip).
- `🎛️commands/🗂️registry/🧪️tests/🟦️.ts`: the gate's real `available` port is false for a checkout with `node_modules`, `pyproject.toml` and `uv.lock` but no interpreter, and true once the interpreter exists. The file passes (49 tests).
- `🏗️build/⌨️command/📦️dependencies/🧪️tests/🐍️python-sync/`: a language-agnostic table (`🔣️.json`) and its test for `pythonSyncDueV1` (`ensure` syncs Python only while it is missing, `sync --all` always, scoped sync and lock never). Passes (7 tests). `🟦️.ts` of the command now routes both branches through `pythonSyncDueV1`.
- There is no third-party oracle for these rules: they decide when to run `uv sync`, so there is no numeric output to cross-check.
- Not tested: the real `uv sync` through `NativeDependenciesScript`. It was exercised once by hand with `repo dependencies ensure`.

## En1997 scene oracle run
`🗑️generated/oracle-test2.log` was still waiting for compiler image headroom after about 470 s, so there is no pass or fail yet. Its status is not recorded here.
