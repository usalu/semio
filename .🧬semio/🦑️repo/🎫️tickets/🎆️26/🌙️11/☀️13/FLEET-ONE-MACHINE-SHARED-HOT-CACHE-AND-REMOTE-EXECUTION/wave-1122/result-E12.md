# Result E12 (fleet-slo)

## What changed (lib `B` = `…/📚️library/🏗️build/`, bench = `📏️incremental/🏎️bench/`)
- Default bench history: `repo bench run` always stores its report in `bench/` of the diagnostics partition of the repository cache (`<ws>/.cache/b/o/m/bench`, budgeted image, machine local, gitignored, not replicated), keeps the newest 64 per host class (`benchHistoryPruneV1`, policy `history.directory|partition|keep` in the schema), compares with the newest earlier report, and bare `repo bench compare` compares the two newest. `--history=<folder>` still overrides. Files: `⌨️command`, `🟦️.ts`, `🧬️schema`, `⚙️operations/🔣️catalog.json` (+ help fixture).
- New probe kind `fleet` with six single-machine scenarios (`🛰️fleet/`: schema, scenarios, pure laws, runner, Python oracle, fixtures, tests). Each runs in a child process on loopback with a second workspace/cache directory; small slices only (6 units, 1 tiny crate chain).
- 6 probes and 11 objectives in `🎯️slo/🔣️.json` (tool `neutral`, 9 blocking): unit-cache and Nx warm-second-workspace hit ratio (goal 1, gate 0.9) and recomputed units (0); twin build in two directories: divergent writes 0; offload under forced pressure: offloaded units 1 and wall p50 (goal 5 s, gate 60 s); scheduler down: local fallback wall max (goal 10 s, gate 60 s); simulated 16 GB host (50/75 % free, critical pressure, real `evaluateHostCapacityV1`) capacity-law breaches 0, reserved memory (gate 12 GiB), simulated 56 GiB-free disk images (gate 90 %).
- Fixed a latent schema bug: `StatsMap` lacked the planning metrics (closures*, swap, available, oom, store), so a real planning report would fail the closed report check.
- Fleet-only selections need no prepared disposable copy. `--slo=kind:fleet` or `probe:<id>` selects them; the MCP `bench_run` `slo` enum only allows `ids|all`, so I ran `repo bench run` from the CLI.

## Proof
- Tests (bun, per file): bench 49/0, fleet 8/0, planning 6/0, acceptance 23/0, operations 15/0. Oracle cross-checks (Python) for warm, twin, memory, disk, host laws; Ajv for the schemas; scenarios themselves run for real.
- Live baseline (load 28-38, host class darwin-arm64-10, load class saturated; 3 runs each), all 11 pass, second run `compare`: 11 same, 0 regressed:
  unit hit 1.0, executed 0; nx hit 1.0, executed 0; divergent writes 0; offloaded 1, offload wall p50 647 ms; fallback max 523 ms; breaches 0; reserved 10.2 GB (9.5 GiB); disk images 52.06 GB (48.5 GiB).
- Stored: `.cache/b/o/m/bench/20261122T000643Z-…json` (first baseline) and `…000703Z…json`.

## Not verified / caveats
- The probes use synthetic units for the cache tiers and one real rustc chain for twin/offload; they measure the mechanism on loopback, not the live fleet server (6094) or live workers. Twin build passes with plain remap flags, not through the E3 wrapper path.
- Fallback probe only drives a dead scheduler endpoint with a live local compile; dead peers tier is not separate.
- Offload/twin/test helpers are imported from `🌐️remote-execution/…/🧪️tests` (layering smell, noted; the cluster harness is the only in-process scheduler+worker).
- E3/E4/E5/E7/E10 fixes were not required for these probes to pass; if their fixes change key or wrapper behaviour re-run `repo bench run --slo=kind:fleet`.
- Probes 1 to 4 do not cover the 1000 live AC divergences; they are a regression guard.

# Round 2

## 1. MCP schema generator
- Cause: `enumOf` in `💻️client/🔌️mcp/🗂️catalog/🐹️.go` turned every `a|b` value name into an enum, so `ids|all` (`--slo`) became the literal enum. Fix: a value set that names a free placeholder (`ids`, `id`, `name(s)`, `path(s)`, `file(s)`, `projects`, `targets`, `n`, `number`, `text`) is no closed set. `--slo` is now a list input like `--scenario`/`--workloads`.
- Sweep of the catalogue (all 100+ flag rows): other `|` sets are genuine closed sets (on|off, en|de, prototype|release, shared|isolated, fresh|current, output styles, sandbox kinds, `--for`). Untyped numeric flags now integers in `🔌️mcp/🖼️assets/🏗️build-surface.json`: `--limit` (fleet_status.limit and others), `--local-workers`, `--max-bytes`, `--port/--grpc-port/--worker-port`, `--load-interval`, `--client-pid/--owner-pid`. `--max-load` (per-core decimal) stays text because the policy types only integers.
- Test `TestEveryFlagIsTypedByWhatItsValueNameMeans` sweeps the whole catalogue (mixed sets are no enum, numeric names are integers, closed sets equal their value name). Not run through the graph: project `repo-mcp` is not in the graph and the tree's Go packages do not compile in place (`sandbox.OriginPath`, `providers.McpClientKind` missing, peers' edits). I ran it in an ASCII scratch module with the real catalogue and policy: pass; with the placeholder rule disabled it fails on `ids|all`. The running MCP server must be restarted to serve the new schemas.

## 2. Live variants (`fleet-live-*`, all advisory until E3/E4/E5/E10 land)
Scenarios `live-unit-cache`, `live-nx-cache`, `live-offload` (child process, cancellable, bounded, small; credentials only inside the child environment, never printed). Endpoints come from the real fleet facts (cache 192.168.178.70:6094, scheduler :8980).
- `fleet-live-unit-*`: a unique tiny crate through the real `wrapRustcV1` in two scratch workspaces with separate caches against the fleet cache; second workspace must run no compiler; new divergences counted from the live `/status`. Measured: hit 1.0, compiler runs 0, new divergent writes 0 (19 s, writes two small entries per run into the live cache).
- `fleet-live-offload-*`: forced local pressure, real scheduler. Measured twice: once offloaded in 0.94 s, then repeatedly **fell back locally**, the client event says `infrastructure: Content store unreachable: http://192.168.178.70:6094 timed out`. This is a concrete cause of rarely used offload (the cache server is slow or unreachable under load); recorded as failing, advisory.
- `fleet-live-nx-*`: `repo run @teaching/proctor:build --tool=nx` twice, the second with an empty `NX_CACHE_DIRECTORY`; remote restores counted from `[remote cache]` lines. Not provable now: the warm-up run does not finish within 300 s (graph slice reads), so the objective is unproven (probe-failed). Run it again when E10 and the graph wait are fixed; whether the repo wrapper honours `NX_CACHE_DIRECTORY` is also unverified. Raw Nx in a scratch workspace is refused by the guarded install ("Original immutable Native snapshot authority required").
- Rerun after E3/E4/E5/E10: `repo bench run --slo=kind:fleet` and then promote the live objectives to blocking.

## 3. No test imports in production probe code
New non-test member `🛰️fleet/🧰️harness` (checkout, pressured host probe, counting rustc, loopback cache+scheduler, worker environment); the runner imports nothing from a `🧪️tests` folder; the test `production fleet probe code never imports from a tests folder` enforces it. The worker risk switch is now scoped to the worker child only.
Bench 49/0, fleet 9/0. Baseline with live objectives: loopback 11 pass; live unit 3 pass; live offload fail (fallback as above); live nx unproven.

# Round 3

E10 had already landed `live-nx-cache` (two scratch Nx workspaces through the signed flight) and a new `live-bazel-cache` probe in the same fleet member; I reused them unchanged.

Run (`repo bench run --slo=kind:fleet`, load per core 3.8, saturated), stored in the default history as the new baseline; `repo bench compare`: 18 same, 0 regressed, 1 new, 1 incomparable (renamed offload objective), 1 missing (old name).
- Pass live, now **blocking**: unit cache across two workspaces (hit 1.0, compiler runs 0, new divergent writes 0; hit gate 0.9), Nx restored from the fleet cache with an empty local cache (1.0, 0 re-executed), Bazel after `bazel clean` (1.0, 0 re-executed). Margin: the ratios are exact 1.0 against a 0.9 gate, counts 0 against 0; they passed in every live run since the disk fix (3 of 3 for unit, 2 of 2 for Nx/Bazel in two bench runs).
- Fail, stays **advisory** (`fleet-live-offload-offloaded-mean`, probe now 3 runs, gate 0.66): 2 of 3 runs offloaded; the third stayed local with `offload-declined: no-idle-machine, "No other machine has headroom"` while the scheduler lists workers with `headroom: true` (g1-server shows 60 units offloaded in, so offload works). Earlier round: `Content store unreachable ... 6094 timed out`. Owner: E5 (offload admission against the live load table: the idle check disagrees with the scheduler's headroom flags, probably load reports older than the decision or the requester's own machine being counted) and, for the 6094 timeout case, the remote-cache server latency under load (E16/E4 area). The wall objective is unproven (a failed run is recorded as 600000 ms on a saturated host).
- Display nit: the mean of a count metric prints rounded (0.67 shows as 1); not fixed here.

# Round 4

1. Fixes: counts print their decimals (a mean of 0.67 shows 0.67); a failed offload or fallback run no longer reports 600000 ms: scenarios declare `optional` metrics, so the run yields `offloadedUnits: 0` (or `executedActions` 0) and no time, which makes the count objective fail and the wall-time objective provable from the successful runs. New blocking objective `fleet-offload-fallback-completed-min` (local compile happened with the scheduler down).
2. New objectives (advisory): `fleet-live-run-first-task-p50` and `fleet-live-run-done-p50` (warm single-target `repo run @teaching/proctor:build --tool=nx`: command start to first `> nx run` line and to done; one priming run, 3 measured; goal 3 s/5 s, gate 15 s/30 s, provisional until E14's slices); `fleet-ready-ratio-min` (share of workers whose receipt in `/Users/Shared/semio-store/v1/origins/*/ready.json`, written within 10 minutes, says Ready; goal 1, gate 0.9) and `fleet-ready-time-max` (median time from supervisor start to Ready among the Ready ones; goal 10 min, gate 30 min). Pure laws with Python oracle and fixtures; fleet tests 10/0, bench 49/0.
3. Run (02:15 to 02:30, load per core 5, saturated), stored in the default history, `repo bench compare` against the previous baseline: improved 1, same 3, regressed 0, new 5.
   - Live offload now passes: 3 of 3 offloaded, max 8.1 s (was 2 of 3 with `no-idle-machine`): E5's fix works.
   - Offload under pressure p50 782 ms, fallback max 903 ms, fallback completed 1.
   - `fleet-ready-ratio-min`: **fail, 0 %**: all 12 live receipts say not Ready (stages `workspace`, `tools`, `dependencies`, `backoff`); time to Ready unproven (no Ready worker). Owner: the readiness ticket `WORKERS-READY-WITH-LOW-RESOURCES` (workspace graph slice read is the dominant stage).
   - Warm `repo run` latency: unproven, the command failed 3 of 3: a single cached-target `repo run` did not finish within the probe bound (240 s) at load 50 (graph slice reads); a manual run was still going after 5 minutes. Owner: E14 (closure slices) and the graph read stage. Re-run after E14 lands; calibrate goal/gate from the first real numbers.
