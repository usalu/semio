# H50 result: suite runner offload, the real cause of "Original Script deadline refused" on remote runs, thresholds from measured durations

Landed 07:00:03 (slot), locks process + test-file + remote-execution + suites + caching taken and released by `🧰️scripts/H50/land.sh` (only locks it created, dry run of every anchor, all-or-nothing in-memory apply, `apply-process.py` then `apply-product.py`). Landings entry in `landings.md`. Kept scripts `🧰️scripts/H50/` (`suite-driver.ts`, `proof.py`, `measure.py`, `relayer.sh`, `run.sh`, `staged/`).

## 1. Suite runner offload (`repo run <project>:test-<suite>`, the suite scripts)

Design (reuses H44's port, nothing new on the wire except one optional field, see 2):
- `🏃️process/🧪️testing/🟨️bun/📁️files` `runOwnedBunFilesV1` gets an optional `remote` port (`OwnedBunFilesRemoteV1`: `width`, `take(file, index, signal)`, `measure`). Besides the local lanes (unchanged: memory gate, `maximumParallelFiles`) it starts `min(width, files)` remote lanes. A remote lane takes the next file and offers it; a file the port answers (`OwnedBunFileResultV1`) is settled exactly like a local one (`onFileResult`, publish, yield, cohort output bound); a file the port keeps (`undefined`), a port that throws, or a result for another file goes to a retry queue the local lanes drain at once. Local lanes never wait for a file that is still out and finish when the remote lanes are done and the queue is empty. Results stay in file order whatever the width. Without `remote` the code path is the old one.
- `🏃️process/🧪️testing/🗂️suites/🏃️run` `runProcessTestSuiteV1` takes `remote?: ProcessTestRemoteV1` (`width`, `file(relativeFile, {budgetMs,index,total,signal})` answering `{origin,seconds,passed,output}` or `undefined`, `measure`, `environment`). A remote result becomes a synthesized capture (stderr = the output the other machine produced, status 0 or 1). The files of the suite's children get the guard environment (`SEMIO_TEST_FILE_OFFLOAD=never`, so a nested suite inside a test file never offers its own files again; the worker has it already) and the stretch environment (2). The files that failed are listed in suite order, not in completion order (deterministic aggregation).
- Product: `🏗️build/🔬️test-file/🌐️remote/🗂️suite` `createSuiteRemoteV1(checkout, suite, environment)` builds the remote from the very port of `repo test-file` (`createTestFileRemoteV1`): same host and load verdict, same Ready machines of the platform, same replica cut + file SHA-256 pin, same claims (never two files on one machine), same streamed `[i/n origin]` output, same fallback reasons. A file is offered only when it belongs to exactly this suite (one route, mode `suite`, no launcher, runs on this host), because the other machine runs `repo test-file -- <file>` and therefore runs exactly that suite's route; anything else stays local. A file that failed there is the verdict (no re-run); a deadline refusal on the other machine is an infrastructure refusal and the file runs here (H44's rule, now in the same port). Mode `never` returns no remote at all.
- Callers wired: `🏗️build/🗂️suites/📜️script.ts` (every `build-*` suite) and `⚡️caching/📜️script.ts` (the caching suites). NOT wired: `🌐️workspace` suite groups and `🏃️process/📜️script.ts` (the process module may not import the repo product; its suites would need the port injected from outside, a separate small step).
- Tests: process `bun-files` remote 2/0 (remote lane takes some files, local lanes run the rest, order kept, failure = verdict, throwing port = all local; real child processes), test-file `suite-remote` 3/0 (offer rules, fallbacks, cancel, mode never), plus the existing bun-files 2/0, checkout-test 23/0, test-file remote 11/0, contention 15/0 (once red under load 69 on a 2 s wall-time law, green alone).

## 2. "Original Script deadline refused" on remote runs: root cause

H44 hypothesised a double stretch; the arithmetic says otherwise. `checkScriptInvocation` throws this text for `remaining <= 0` (the envelope deadline is used up), not only for `remaining > maximum`. The worker runs `repo test-file` with its own envelope, funded with `declared budget x stretch measured at launch + 120 s slack`. A worker is calm at launch (stretch 1.0) and the test file then loads it. The units file (`⚡️caching/🦀️cargo/📦️units/🧪️tests`, suite `rust-units`, declared 300 s) was measured 427.6 s on a worker (`offload-test-measure.json`): the worker funded 300 x 1.0 + 120 = 420 s and refused at exactly that deadline. On the coordinator the same file is funded 300 x 4-8 (its load) and never hits it. So the budget of an offloaded run was not "stretched twice", it was stretched by the wrong host at the wrong time (launch of an idle executor), while the requester that decided to wait had funded a much longer budget.

Fix (process area + one optional wire field), "stretch measured once per run tree, never smaller below than above":
- `🎚️budget/⏲️contention` (schema-first: `inheritance.environment` in the policy table and schema): a process funds every deadline with `max(stretch handed down by its parent run, its own launch measurement)`, fixed at its first deadline. `runProcessTestSuiteV1` hands its stretch to every file process (`budgetStretchEnvironmentV1`), so a nested process (a test that starts its own envelope) never funds with less than the run that started it and never measures a second, smaller one. Exports: `inheritedStretchRatioV1`, `launchStretchBasisPointsV1`, `stretchEnvironmentV1`, `budgetStretchEnvironmentV1`.
- `Request.stretchBasisPoints` (optional, 10000..640000, schema `🔬️checkout-test/🧬️schema`): the requester sends the stretch its own run was funded with (`launchStretchBasisPointsV1()` in the test-file port); the worker puts it into the environment of `repo test-file`. The executing host's `repo test-file` process therefore funds `declared x max(requester stretch, own launch measurement) + slack`: the worker never funds less than the requester would have locally, and a busier worker stretches more. The action timeout stays `budgetSeconds + 120 s` (requester's own funded budget).
- Tests: contention `🧬️inherit` 2/0 (parser bounds, environment, child processes started with 10000/20000/40000/80000 and with nothing/garbage fund at least that stretch), checkout-test 23/0 (Request shape with and without the field), real scheduler test unchanged.
- Rollout fact (observed): workers that still run the 06:26 sidecar refuse a `Request` that carries the new field (`request not allowed: /stretchBasisPoints is not allowed`, code 3, instant) and the file runs here with that reason printed; once their sidecars restart on the new code the field is accepted. No hang, no wrong verdict.
- NOT verified live: that the units file now passes on a worker (a 430 s run was not repeated).

## 3. Thresholds from measured durations

Measured pairs (wall seconds of the same file, local = `SEMIO_TEST_FILE_OFFLOAD=never`, remote = `always`, coordinator load 4.6-5.1 per core, workers 0.1-1.0 per core), `wave-1127/🗑️generated/H50/pairs.jsonl` (deleted at the end, numbers below) plus H44's store `offload-test-measure.json`:

| file | local wall | remote wall | remote test s / local test s |
|---|---|---|---|
| resident consent | 2.9 | 4.7 | 1.4 / 2.1 |
| graph edge-refresh | 8.5 | 10.4 | 5.5 / 7.6 |
| remote-execution worker | 9.3 | 6.7 | 4.7 / 8.9 |
| inputs storage | 54.6 | 28.2 (store) | |
| edge-refresh (store, other time) | 15.4 | 4.3 | |
| checkout-test | 8.4 | 9.8 | |
| worker (store) | 7.8 | 9.8 | |
| test-file remote | 7.8 | 8.1 | |

Refit at 07:28 with the 14 complete pairs now in the store (the suite runs and other lanes added six; local seconds were measured at coordinator load 45-69 on 10 cores, i.e. 4.5-6.9 per core): `remote wall = 5.2 s + 0.20 x local wall` (13 pairs with local >= 1 s: 5.6 + 0.19). Break-even (remote = local) at 6.5-6.9 s; remote is shorter than 0.8 x local in 9 of 13 pairs: in 8 of the 9 files that take >= 10 s here and in 1 of the 5 that take < 10 s. The fixed cost of leaving is 2.0-5 s (hand-over, replica check, start), the test itself ran 0.35-0.72 of the local time (median remote/local 0.35).
- `remoteWorthRatio` 0.8: stays. Remote/local = 0.20 + 5.2/L is below 0.8 from L = 8.7-9.2 s, about 2.2 s above break-even, which is the spread of the repeated pairs; the 8 of 9 / 1 of 5 split above is the ratio doing its job. Derived, not changed.
- `perCoreAlways` 1.5: removed. Its rule ("at high load always leave, measure ignored") contradicts the data: at 4.7 per core the 2.9 s and 8.5 s files lost by leaving (1.6x, 1.2x) and 4 of the 5 files under 10 s lose in the refit. The measured pair now decides at every load (both fresh and remote >= 0.8 x local: stay); a file without a pair still leaves to learn its remote duration (that is how the pairs fill: every offloaded suite file records a remote duration, every file that stays records a local one).
- `perCoreMeasured` 0.75 -> 0.5: the local slowdown against an idle worker is s = 1/0.20 = 5 at about 5.5 per core now (1/0.64 = 1.6 at 4.7 per core in the first six pairs; the loaded coordinator is not linear), so take s = 1 + 0.65 x load/core as the conservative reading. A file of L seconds is worth leaving when `L (1 - 1/s) > 5.2`: 100 s files from 0.08 per core, 30 s files from 0.33, 10 s files from 1.7. 0.5 per core is where a 30 s file (the files whose relief matters) certainly pays; shorter files are held back by their own measured pair, not by the load.
- Confidence: medium for the ratio and the removal of `perCoreAlways` (14 pairs, one clear regime), low for `perCoreMeasured` (every pair is from coordinator load 4.5-6.9 per core, none from a calm one, so the left end of the curve is extrapolated). The store refills with every suite run; refit `a`, `b` and the slope from it in the next slot once a few calm-host pairs exist.
- Vectors updated: checkout-test `🔣️.json` (policy, "a file measured as worthless stays at any load", 0.4 per core stays) and the independent Python oracle `🐍️.py` (no always branch); test 23/0 (plus the worker begin test now asserts the stretch environment).

## 4. Live proof



Prerequisite: a worker that still ran the 06:26 sidecar refused a Request with the new field (`request not allowed: /stretchBasisPoints is not allowed`, instant, file ran here with the reason printed: 07:06-07:25, the fallback works). From 07:26 the workers accepted it (first file: `ran on g1-m13`).

Both runs go through `🧰️scripts/H50/suite-driver.ts` (`repo suite-driver.ts <suite>`), which runs ONE suite of the build route table exactly as `🗂️suites/📜️script.ts` does (same `runProcessTestSuiteV1` call with `createSuiteRemoteV1`, same envelope arguments as `repo run repo:test-<suite>`), without reading the project graph. The real Nx path `repo run repo:test-build-source-tree` was started at 06:44 as the local baseline and sat in "Reading the project graph slice" for 43 minutes (2579 s, round 1, 718 of 3600) without reaching the suite; I killed it (my own pids). So the real target was NOT proven; the suite scripts of `🗂️suites` and `⚡️caching` carry the same three lines the driver has.

| suite | local (`never`) | offloaded (`always`) | totals | machines |
|---|---|---|---|---|
| `build-source-tree` (4 files) | 43 s, 7 pass 0 fail, 4 files, 265 expect | 57.5 s, 7 pass 0 fail, 4 files, 265 expect | identical | g1-m11 x2 (2 files, one after the other), g1-m13 x1, 1 file here (`no-idle-machine`) |
| `build-native-storage` (5 files, one file fails in the tree: `👁️existing`) | 82.5 s, 4 pass 2 fail 1 skip, 7 tests, 5 files, 13 expect, `suite-files-failed` naming `👁️existing` | 26.8 s, same totals, same `suite-files-failed` naming `👁️existing` | identical, same failed list | g1-m01, g1-m11 x2, g1-m13; `👁️existing` ran here |

`proof.py` (strips the `[i/n origin]` prefix, sums `pass/fail/skip/expect/Ran N tests across M files`, requires >= 2 machines): identical true, 3 machines on `build-native-storage`, 2 on `build-source-tree`. The first suite is slower offloaded (its 32 s file sits on a loaded worker while the other files are short); the second is 3.1x faster. Single sample each, load was not controlled.

## Not verified / open
- The real Nx suite target (graph read never finished, see above); `🌐️workspace` suite groups and `🏃️process/📜️script.ts` are not wired.
- Units file on a worker under the stretch floor (the 430 s case) was not rerun.
- Windows/Linux workers did not take a file.
- `perCoreMeasured` 0.5 is extrapolated on the calm side (see 3).
