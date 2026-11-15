# Baseline health before the authoring wave (2026-11-14)

Scripts: `health-run.sh NAME TARGET` (plain `repo run`, `time -l`), `health-direct.sh` (direct vitest, unused: needs `SEMIO_VITEST_POLICY`), `health-uptodate.sh` (the whitespace experiment). Logs: `🗑️generated/health/` (delete when done). The coordinator announced mid-task that the staffage schema (`volume` to `volumes`), kernel (`scenes`) and pipeline change; everything below is "before the change" and no volumes-related failure was touched.

## 1. Targets, plain `repo run`, one at a time

| # | command (verbatim) | exit | wall | result |
|---|---|---|---|---|
| 1 | `repo run @semio-tech/framework-3d-js:test` | 0 | 1587 s (vitest itself: 13.7 s) | `Test Files  40 passed (40)`, `Tests  673 passed \| 1 skipped (674)` (the skip is the GPU case) |
| 2 | `repo run @teaching/staffage-react:test` | 1, 75, 75, 1 (four attempts) | 454 s, 935 s, 727 s, 1640 s | never reached the tests, see below |
| 3 | `repo run @teaching/staffage:test` | not run | | withheld on the coordinator's instruction (suite goes red for the volumes transition) and because step 2 never got past planning |
| 4 | `repo run @teaching/staffage-site:test` | not run | | same graph-service blocker as 2 |

Why 1 took 1587 s: 26 minutes behind another agent's `tools setup all` (exclusive `native-fixed-image` lease for about 25 minutes). Not a defect of the target.

Attempts at 2 (logs `staffage-react-1/-2.log`, `staffage-react.log`, `ut-run.log`):
- attempt 1: `Internal error: ENOENT: no such file or directory, lstat '.../🦑️repo/🔨️modules/⚡️caching'` after `tasks: 1 stale, 0 up to date`. The directory now lives at `📚️library/⚡️caching`; a path observed before another agent's move went stale. Not reproduced on retry; the tracked tree has no reference to the old path.
- attempts 2 and 3: exit 75, "The project graph could not be read on this machine: it can infer only 0 of the stale projects ... this machine can give about 0 MB per project". Host spare memory was 300 to 800 MB above the floor, because a dozen authoring agents ran bun/python at the same time. The refusal is the documented memory governor, not a tree error.
- attempt 4 and the experiment run: "The graph service under the launcher was denied access; it is started again", then "The graph service could not answer (timeout)" after 1640 s; a retry sat in `Planning 1 targets` for more than 20 minutes without slice progress.

Direct vitest runs as evidence: not obtained. A direct run needs `SEMIO_VITEST_POLICY` (JSON owner policy) which only `repo run` composes; I did not forge it. The kernel suite evidence is the plain run above.

## 2. Items (a), (b), (c): state of the tree

- (a) Re-verification of task keys: present. `🧮️derived-nodes/🟨️.mjs` has `verify({signal})` (stat every file the nodes read, re-digest where the stamp moved, invalidate), and `🏡️resident/🔌️ports/🧭️plan/🟨️.mjs` calls `state.graph.verify?.()` before `taskKeysV1`. It had no test. Added:
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧪️tests/🎯️task-state/🟦️.ts`, new test "an edit nobody announced, even a whitespace one, changes the key after the graph re-verifies its files, and an unchanged tree stays up to date" (real disk world and stat index; no dirty set given; the edited task and the task above change key, an unrelated one keeps it). Passes: `bun test ./<file> -t "edit nobody announced"`: 1 pass, 6 expects. The other two tests of the file were not re-run.
  Whitespace experiment on the real tree (`health-uptodate.sh`: run react test, `--dry-run` plan, append a newline to `🎯️targets/⚛️react/🧪️tests/⏱️clock-determinism/🟦️.tsx`, `--dry-run` plan, restore): NOT completed, because the first step needs one successful plain react run and the graph service never planned (section 1). The script is still running in the background while this is written; if it finishes, `🗑️generated/health/ut-plan-{1,2,3}.log` hold the `tasks: N stale` lines (expected 0, 1, then 1 stale-or-0 after restore). The test file is only edited by that script after a successful run; it restores it from `ut-original.tsx`. Unproven on the real tree.
- (b) Producer digest in the `export-models` resume key: present (`🧍️staffage/🔨️modules/📤️export/📦️batch/🔏️producer/🟦️.ts`, `BatchSettings.producer`, compared in `sameSettings`, test in `🧪️tests/🧊️model-batch/🟦️.ts`). Left alone: the pipeline agent owns the batch exporter code now. Note for it: `producerDigest` roots are the staffage modules, the staffage schema and `🧊️3d/🗿️model`; the new `volumes`/`scenes` code lives under those roots, so it is covered, but a new root outside them would not be.
- (c) Nx cache service exiting during long silent runs: present. `🏗️build/⌨️command/🌐️host/🟦️.ts` `withHostedRemoteCacheV1` hosts the cache inside the graph service and feeds `noteScriptProcessProgressV1` every 30 s while a capacity grant is younger than 120 s (against the 10 minute stall watchdog `🐕️watchdog`), and the idle shutdown (30 min) asks `runInFlightV1` first (`🏡️resident/🏛️service/🌐️host/🟨️.mjs`, tests in `🏡️resident/🧪️tests/🌐️host/🟦️.ts` lines 488 to 495). The heartbeat itself is a non-exported closure with no test. Not proven by a full plain staffage run (withheld, see 1.3); the earlier failure signature (`Connection refused` on 127.0.0.1:6090 after a silent run) was not seen in any run today, but no run was long and silent.

## 3. Catalog checks (152 assets, model guard, silhouette guard)

Not run. They live inside `@teaching/staffage:test` and depend on the volumes transition that others are rewriting; no plain run was possible.

## 4. Open, stated plainly

1. No plain run of the staffage, staffage-react or staffage-site targets exists for this baseline; only the kernel target is green (40 files, 673 passed, 1 skipped).
2. The `repo` graph service cannot plan under the present load (about 12 agents). Concrete symptoms: governor refusal exit 75, a stale-path ENOENT, a service that does not answer, multi-minute planning stalls. Root cause for the stalls is not established; `repo service host status` itself only answered after minutes. This needs a quiet machine or a dedicated one for baselines.
3. (a) has a unit test but no real-tree proof; (c) has no proof; (b) belongs to the pipeline agent.
4. The `ENOENT lstat ...🔨️modules/⚡️caching` race (a moved directory still observed by the graph service) is a candidate robustness bug in the graph's file observation (a vanished path should invalidate, not abort the plan); not investigated further.
