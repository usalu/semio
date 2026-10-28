# result-cbudget (lane `cbudget`, wave 1027b) — course:test "node exceeded its 300000ms budget"

C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`; R = `/Users/ueli/Documents/semio`. Updated 18:20 (final).

## FOR coordinator:
- **batch `clock` READY** (staged, verified on mirror `cb-m`, base = live at 18:12): `C/cbudget-tools/staged/clock/`
  - check : `python3 -I C/cbudget-tools/staged/clock/stage.py check`
  - apply : `python3 -I C/cbudget-tools/staged/clock/stage.py apply`   (exit 2 = a base file drifted: run `python3 -I C/cbudget-tools/staged/clock/rebuild.py` first, it re-edits at anchors from the CURRENT live files and exits 2 if a peer touched the anchored lines)
  - revert: `python3 -I C/cbudget-tools/staged/clock/stage.py revert`
  - laws : `SEMIO_TEST_ARTIFACT_DIR=R/C/🗑️generated/cbudget sh C/cbudget-tools/staged/clock/verify.sh` (stage check, derived-index law, then `bun test` of the new suite + execution + budget + vitest + protocol: 31 pass / 0 fail on the mirror)
  - product proof: `bun repo run @teaching/architecture-course:test` (one run, then the same again from cache is not needed). Expect: exits 0 although the host is loaded; stderr shows `[budget] Busy host: load … | Ausgelasteter Rechner: Last …` while it counts slower than the wall. Process-module edit: every project that runs a test command re-keys once (the execution module is in the command closure of all test targets), so land it in a group with other process-module edits.
  - a stage.py mode law: `python3 -I C/xplat5-tools/tests/test_stage_mode.py C/cbudget-tools/staged/clock/stage.py` OK.
- **batch `shard` READY (optional, land after `clock` has proven itself; independent of it)**: `C/cbudget-tools/staged/shard/`
  - apply : `python3 -I C/cbudget-tools/staged/shard/stage.py apply` (rebuild: `python3 -I C/cbudget-tools/staged/shard/rebuild.py`); check/revert as above; laws: `sh C/cbudget-tools/staged/shard/verify.sh` (stage check + new course-shard laws + the existing test-classes suite: 10 pass / 0 fail on the mirror; the new laws are RED on the base table: `shards` 1 instead of 3)
  - product proof: `bun repo run @teaching/architecture-course:test` once more; expect Bazel to run 3 shard actions (`shard_count = 3`), `exit 0`; per-shard wall time well under the old single run. This is the first live use of `shard_count` in the table (the rule list was empty), so a red here is cheap to revert (`stage.py revert`).

## Where the 300 s came from
1. `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts` `TEST_LEVEL_BUDGET_MS`: fundamental 15 s, **quick 300 s**, long 900 s, exhaustive 1800 s. `TestScript` of the course (`🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`) calls `resolveTestLevel(segments, "quick")`, so the course suite's floor is `quick` = 300 s.
2. `runVitest` (library `🧪️testing/🧪️vitest`) builds the owner policy (`budgetMs` 300000) and `runVitestV1` -> `runBudgetedTestCommand(…, {budgetMs: policy.budgetMs, throwOnFailure:true})`.
3. `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts` `runBudgetedTestCommand` armed ONE `setTimeout(stop("timeout"), budgetMs)`: pure wall clock. g45-ctest.log line 21014: `[budget] node …/vitest.mjs run --config …/🎚️config/🟦️.ts --testTimeout 300000 … --maxWorkers=4 exceeded 300000ms — killed`, then `error: node exceeded its 300000ms budget`; the Bazel test class of the course is the default `enormous` (3600 s), so Bazel was not the cutter.
4. Context at the cut: host load 40-70 on 10 cores (this host at 18:05: 49.9 / 55.0 / 57.1), i.e. every runnable thread got ~1/5 of a core; a suite that needs 50-260 s on a quiet host needs 5x that on a loaded one.

## Fix (batch `clock`): the budget is work, not wall time
- New module `🎚️budget/⏲️contention/` (policy `🔣️.json` + schema `🧬️schema/🔣️.json`, tests with fixtures and an independent Python oracle):
  - `stretch = clamp(ratio, 1, 8)`; ratio = one-minute load average / `availableParallelism()` (POSIX, Linux/macOS/devcontainer); on Windows (no load average) a smoothed measurement of the scheduler (`spinSlowdownV1`: wall time per CPU millisecond a spinning thread of the process gets, every 10 s, EWMA 0.3). Policy: tick 1 s, max stretch 8.0, announce above 1.25 (step 0.5, repeat every 60 s).
  - budget clock: every wall millisecond is worth `1/stretch` budget milliseconds (credit = ∫ dt/stretch). Consequences (proven): a run needing ≤ budget on an idle host is NEVER stopped on a busy host as long as the stretch it is held to covers the real slowdown (≤ 8×); on an idle host the stop time equals the old `setTimeout` (exactly); a hung process is stopped after at most `8 × budget` wall (quick: 40 min; fundamental 15 s -> 2 min).
  - `runBudgetedTestCommand` now uses `watchTestBudgetV1` (option `contention` for laws/wall-exact callers; default = this host). Failure text `node exceeded its Nms budget` unchanged; the stop additionally prints one `[budget] Budget of N s used up after W s on the wall clock (host stretch S, U s counted). | Budget von …` line; while stretched the run prints (EN | DE) `[budget] Busy host: load L on C cores, so the N s test budget counts at 1/S speed (U s of it used after W s; a hung test is still stopped after at most X s).` = the progress report.
  - Derived schema index (`🗂️schema-document-index.json`) got the one new `$id`; build guide got an EN+DE bullet.
- Not changed (by design): per-test timeouts handed to the tools (`--testTimeout`, `bun test --timeout`, `go -timeout`) stay wall (they are per test, 300 s each, not the failing layer); Bazel's own test timeout (default class `enormous` = `eternal` 3600 s) is an outer wall bound: `quick x 8` = 2400 s fits, `long`/`exhaustive` at the full cap would hit it first (residual, see end).

## Evidence
- Real host, same child (needs 4000 ms of wall) under a 2000 ms budget, load1 ≈ 50 on 10 cores (`C/cbudget-tools/slowhost.ts`):
  - base (live module): `Error: node exceeded its 2000ms budget` after 2553 ms (RED).
  - new (mirror): `survived` after 4507 ms, printed `Busy host: load 49.6 on 10 cores … counts at 1/5.0 speed` (GREEN).
  - new, hung child (`setInterval`), budget 1000 ms: stopped after 5085 ms = 1000 × 5.0 stretch (hung test still killed).
- Laws `⏲️contention/🧪️tests/🟦️.ts` (13 tests, 2408 expects, ~13 s): Ajv (third-party) agrees with the schema and every broken policy; stretch table; clock arithmetic; 9 authored host traces and 400 random traces vs the Python integration (stop time equals the exact integral within one whole-millisecond timer step; always in [budget, 8×budget]; a replay that fits its budget survives every trace whose ratio stays ≤ 8, a run past the bound does not); 120 traces whose load changes between ticks (error ≤ one tick per change); unmeasurable host falls back to the wall; stop cancels the timer; announcements EN+DE (once, again on a 0.5 step or after 60 s, silent when idle); load-average source vs Windows probe EWMA vs the Python model; probe arithmetic on a simulated and on the real host; REAL processes: the 4 s child survives a 2 s budget on a ratio-4 host and is killed on the wall source (same message text as before), a hung process is killed at `3 × 600 ms` and at the cap `8 × 300 ms`.
- Regression set (one `bun test` per file, mirror, failure sets compared before/after with the old runner restored): execution 3/0, vitest 2/0, budget 3/0, protocol 4/0 + 10/0, workers 10/0, cargo order 1/0 identical; pre-existing red on BOTH sides and unchanged: ceiling 2 real-process tests, cargo `🧪️tests` 5, capped-run 1 (they fail the same way on base).
- `tsc` on the new module + runner + test: no error in them (one pre-existing error in `🟨️bun/🚧️operands`); parse scan `C/gate-g-tools/scan.ts` bad 0.

## Residual / notes
- Windows path (probe) is exercised by simulation only (no Windows host here); load average semantics on Linux containers with a CPU quota are not covered by `loadavg` (a quota-throttled devcontainer is not stretched; stays wall).
- The stretch is also applied when the load is our own (a 4-worker vitest on a 4-core CI runner raises load above the cores): the cost is a later stop for a hung test there, never an earlier one.

## Sharding (batch `shard`)
- Rule `course-test-shards` (evidence `declared`) in `🚦️test-classes/🔣️.json`: `{targets:["test"], projects:["@teaching/architecture-course"], shards:3}` (size stays the default class, so Bazel's own timeout stays `eternal`, not `moderate`). Bazel then sets `TEST_SHARD_INDEX/TOTAL_SHARDS`; `bazelTestEnvironmentV1` maps them to `SEMIO_TEST_SHARD=i/3`, and the neutral protocol (`prepareTestCommandV1`, already honoured by the Vitest spawn of `runBudgetedTestCommand`) appends `--shard=i/3` and touches the shard status file. Each shard runs under its own budget clock and its own admitted worker share.
- Third-party oracle: real Vitest 4.1.12 (`run --shard=i/3 -t <matches nothing> --reporter=json` on the course config, 56 s each at load 50) assigned 6 / 6 / 5 files (`mode, sessions, release-tags, course, dashboard/parameters, continuous/manifests` in shard 1) — recorded in the fixture; an independent Python model of Vitest's sequencer (sha1 of the root-relative path, order by digest, consecutive slices) reproduces all three shards exactly; a second law reads the live course config's include list and demands a covering, disjoint, non-empty split with sizes differing by at most one (so a new test file keeps the rule honest; a vanished file fails the law).
- Why optional: sharding does not reduce total CPU (three Vitest startups, 3 x ~1.4 GB), it bounds each action's wall time and lets Bazel overlap them when the capacity governor admits more than one; on a loaded 16 GB host the budget clock (`clock`) is the load-bearing fix. If the course suite grows past ~1000 tests or the gate shows one shard >60 % of the old run, raise `shards` (the partition law adapts).
- Not covered: how long each of the three shards takes (file runtimes unknown; `🧪️deploy` has 1 test of 3.6 KB that may dominate its shard).

## Files (all staged, nothing live was edited)
- `C/cbudget-tools/staged/clock/`: `stage.py`, `rebuild.py`, `paths.json`, `manifest.json`, `files/00-08`, `base/06-08`, `src/*`, `verify.sh`, `indexcheck.py`.
- `C/cbudget-tools/staged/shard/`: same shape (`files/00-03`, `base/03`).
- Helpers: `C/cbudget-tools/mrun.sh` (run in mirror `cb-m` with the lane artifact dir), `suite.sh` (regression set), `slowhost.ts` (real-host witness). Debug outputs under `C/🗑️generated/cbudget/` (`[DEBUG]` prefix: before/after suite lists, vitest shard runs).
- Mirror `cb-m` dropped at the end of the lane.
