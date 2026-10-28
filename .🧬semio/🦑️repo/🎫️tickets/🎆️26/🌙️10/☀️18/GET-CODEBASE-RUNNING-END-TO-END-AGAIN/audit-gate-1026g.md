# audit-gate-1026g (wave 1026g, auditor aud-gate-g, read-only)

Scope: every row of `result-gate-g.md` (rows 1 to 4b and notes), every GREEN claim in `result-repair-g.md`, `result-proof2-g.md` and `result-integ-g.md`.

Method: paths below are relative to this ticket folder (`G` = `🗑️generated`). Logs were streamed with `rg`/`sed`, never loaded whole. Run start, exit, wall, peak RSS and graph captures come from the `product.py` json records (exit = child return code, graph = regex captures of the log). Bazel outcomes come from the log text. Batch apply times come from the `*.applied` marker mtimes. Library file times come from `stat`. No build, no graph run, no edit outside this file.

Caveat on mtimes: the library is edited continuously, so a current mtime only proves that a file changed at least once after that time. It does not prove the content a run used.

## A. Apply time versus run start (gate-g batches)

| batch | applied (marker) | first official run in that batch | run started after apply | run straddles a later batch landing |
|---|---|---|---|---|
| zt-g | 12:23:26 (`G/gate-g/zt-g.applied`) | 12:27:14 (`G/gate-g/zt-g-course.json`) | yes | no |
| xplat-g | 12:45:53 (`G/gate-g/xplat-g.applied`) | 12:53:07 (`G/gate-g/xplat-g-dashboard.json`) | yes | no |
| noop2-g | 13:12:46 (`G/gate-g/noop2-g.applied`) | 13:19:06 (`G/gate-g/noop2-g-print.json`) | yes | no |
| phase-g | 15:05:30 (`G/gate-g/phase-g.applied`) | 15:07:50 (`G/gate-g/phase-g-course.json`) | yes | no |
| preadmit-g | 16:00:23 (`G/gate-g/preadmit-g.applied`; table says 16:03) | none, still queued | n/a | n/a |

Other runs that straddle a batch landing (their GREEN is not a clean post-batch result):
- integ-g cycle c2 dash: 12:22:06 to 12:54:14 (`G/integ-g/cycles.tsv`), xplat-g landed 12:45:53 inside it.
- integ-g cycle c3d dash: 14:21:13 to 15:12:34, phase-g landed 15:05:30 inside it.
- proof2-g course no-op run `course-noop1`: started 15:05:06 (`G/proof2-g/p5/course-noop1.json`), phase-g landed 15:05:30 inside it.

## B. gate-g table (`result-gate-g.md`)

### Row 1, zt-g

| claim | verdict | evidence |
|---|---|---|
| applied 12:23 | CONFIRMED | `G/gate-g/zt-g.applied` 12:23:26 |
| course:build 12:27-12:45, exit 0 | CONFIRMED | `zt-g-course.json` started 12:27:14, wall 1092.5, exit 0 |
| capacity wait 356 s | CONFIRMED | `G/gate-g/zt-g-course.log`, `[capacity] waiting 356s ...` then `admitted run` (about line 65) |
| graph gen 549, 171 projects refreshed in 355 s | CONFIRMED | `zt-g-course.log:3706` "generation 549 published, 171 projects refreshed in 355 s" |
| 170 projects re-inferred | CONFIRMED with note | `zt-g-course.log:2644` "refreshing 170 projects"; the json inferring denominator is 168 |
| 8 actions, 1 internal, 3 local, 4 worker | CONFIRMED | `zt-g-course.log:22764-22765` "8 processes: 1 internal, 3 local, 4 worker", "Build completed successfully, 8 total actions" |
| no cache hits | CONFIRMED | processes line has no cache-hit term |
| laws 8 files, 6 green, 2 red | CONFIRMED | `G/gate-g/zt-g.laws.log`: generation-protocol 45 pass / 2 fail, door startup 4 / 1, the other six 0 fail |
| the 2 reds are NOT caused by the batch | UNSUPPORTED | no log shows the cause of either red; the attribution to repair-g's 11:35 edit is not checked by any log or mtime |
| wall 1092 s, peak 6.0 GB, 6 procs | CONFIRMED | json wall 1092.5, peak_tree_mb 6007, peak_processes 6 |

### Row 2, xplat-g

| claim | verdict | evidence |
|---|---|---|
| applied 12:45 | CONFIRMED | `G/gate-g/xplat-g.applied` 12:45:53 |
| verify.sh 17/17 single-file tests pass | CONFIRMED | `G/gate-g/xplat-g.verify.log`: 17 `pass` lines, no `fail` line |
| gofmt clean | UNSUPPORTED | no gofmt output anywhere in `xplat-g.verify.log` |
| Go test target not run | CONFIRMED | only a header line names `TestResidentTrustedDirectoryAndParent`, no result line |
| repo-dashboard-rs:build 12:53-13:12, exit 0 | CONFIRMED | json started 12:53:07, wall 1153.7, exit 0 |
| generation 552, 1 project re-inferred in 34 s | CONFIRMED | json published `["552","1","34"]`; `xplat-g-dashboard.log` |
| bazel 208 s, 3 actions | CONFIRMED | `xplat-g-dashboard.log:7501-7503` Elapsed 208.067 s, "3 processes: 1 internal, 2 local", "Build completed successfully, 3 total actions" |
| (not in table) an earlier target ran 2 actions in 67 s | CONFIRMED | `xplat-g-dashboard.log:7291-7293` |
| peak 3.0 GB, 7 procs | CONFIRMED | json peak_tree_mb 3037, peak_processes 7 |

### Row 3, noop2-g batch 2 and watch-g journal

| claim | verdict | evidence |
|---|---|---|
| applied 13:12 | CONFIRMED | `G/gate-g/noop2-g.applied` 13:12:46 |
| changes 9/0 | CONFIRMED | `G/gate-g/noop2-g.laws/1.txt` "9 pass / 0 fail" |
| journal 3/0 | CONFIRMED | `noop2-g.laws/4.txt` 3 pass, 0 fail, 1 skip |
| plan-reuse 4/0 | CONFIRMED | `noop2-g.laws/5.txt` 4 pass, 0 fail |
| snapshot 21/0, mode 1/0, planned 1/0 | CONFIRMED | `noop2-g.laws/8.txt` (21/0), `6.txt` (1/0), `7.txt` (1 pass, 1 skip) |
| fuzz and journal-world skipped | CONFIRMED | `noop2-g.laws/2.txt`, `3.txt` show only skip lines |
| witness 6/0 after a 2-line fix | CONTRADICTED | the only witness results are `noop2-g.laws/9.txt` "5 pass, 1 fail" with `ENOENT: scandir '/Users/ueli/Documents/semio/🔨️modules/🏃️process'` at witness/tests line 111, and `noop2-g.laws.log2` "5 pass 1 fail". No later witness log exists |
| print-rs:test 13:19-13:33, exit 1 | CONFIRMED | json started 13:19:06, wall 850.3, exit 1 |
| tests 39/39 passed | CONFIRMED | `G/gate-g/noop2-g-print.log:3413` "39 tests run: 39 passed" |
| then retainLaneOutputsV1 threw `paths[1] must be string, got undefined` | CONFIRMED | `noop2-g-print.log:3423` TypeError at `.../unit-cache/🧵️lane/📤️retention/🟦️.ts:83:14` |
| the thrown file is not part of a batch | CONFIRMED | no `retention` path in `xplat-g.list`, `zt-g.list` or `noop2-g.list` (only the sibling `🏗️compiler/🛤️lane/🟦️.ts` is in xplat-g) |
| red caused by a peer edit (lane2-g) between 13:12 and 13:26 | UNSUPPORTED | no log gives the edit time; the retention file now has mtime 13:42:31, i.e. it changed after this run ended (13:33), so the 13:12-13:26 window cannot be checked |
| graph gen 555, 88 re-inferred | CONFIRMED | json published `["555","95","130"]`, inferring denominator 88 |
| peak 7.2 GB | CONFIRMED | json peak_tree_mb 7204 |
| rerun p4 13:34, killed by me at 14:01 after 27 min in the queue | CONFIRMED | `noop2-g-print2.json` wall 1613.7, exit -15 (external SIGTERM); `noop2-g-print2.log:296`; `[capacity] waiting 1608s` |
| print-rs:test 14:01-14:19, exit 0 | CONFIRMED | `noop2-g-print3.json` started 14:01:23, wall 1090.2, exit 0 |
| bazel test PASSED in 192 s | CONFIRMED | `noop2-g-print3.log:2938` "//:t_d71aae306395ac0f PASSED in 192.0s"; `2940` "1 test passes" |
| cold Rust lane compiled | UNSUPPORTED | no `Compiling`, `rustc` or `cold` line in the log (0 matches); processes line is "2 action cache hit, 1 disk cache hit" |
| no EBADF | CONFIRMED | 0 matches of EBADF in `noop2-g-print3.log` |
| graph 87 re-inferred | CONFIRMED | json inferring `["87"]`, refreshing 88, published 558 |
| capacity wait about 9 min | CONFIRMED | last `[capacity] waiting 525s` in the log |
| peak 7.0 GB | CONFIRMED | json peak_tree_mb 7011 |

### Row 3c, hot no-op course:build (three runs)

| claim | verdict | evidence |
|---|---|---|
| run 1 14:19-15:05 never admitted | CONFIRMED | `G/gate-g/b3-course-noop1.log` has 0 "admitted run" lines; json started 14:19:49, wall 2714.5, killed "time" |
| 45 min in the capacity queue | CONFIRMED | json wall 2714.5 s, no admission |
| need 7168 MB | CONFIRMED | 492 occurrences |
| 1-4 jobs ahead | CONTRADICTED in part | 0 jobs ahead occurs 120 times; the range is 0-4 |
| load 29-37 | CONTRADICTED | the load values in the log run from 20.5 to 45.9 |
| peak tree 86 MB | CONFIRMED | json peak_tree_mb 86 |
| runs 2-3 cancelled by me | UNSUPPORTED for run 3 | run 2 is CONFIRMED by `b3-course-noop2.log` (SIGTERM); there is no log or json for run 3 |

### Row 4, phase-g

| claim | verdict | evidence |
|---|---|---|
| applied 15:05 (54 steps) | CONFIRMED | `G/gate-g/phase-g.applied` 15:05:30 |
| verify: oracle equal | CONFIRMED | `G/gate-g/phase-g.verify.log` "the independent oracle agrees with the laws" and oracle pass lines |
| host-capacity 81/0 | CONFIRMED | `phase-g.verify.log:97-98` "81 pass / 0 fail" |
| library 23/0 | CONFIRMED | `phase-g.verify.log:145-146` |
| graph laws 82/0 | CONFIRMED | `phase-g.verify.log:241-242` |
| bazel host-capacity 25/0 | CONFIRMED | `phase-g.verify.log:287-289` |
| tsc: only the known non-phase-g errors | UNSUPPORTED | 21 TS7006 errors, all in `framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧮️system/🟦️.ts`; nothing in `phase-g-tools` or the logs shows they predate phase-g |
| course:build 15:07-16:00, exit 0 | CONFIRMED | `phase-g-course.json` started 15:07:50, wall 3134.9, exit 0 |
| queued 2750 s, ran about 385 s | CONFIRMED | json queued_s 2750.3 (run part 384.6 s) |
| graph {inferring 7, refreshing 7, published 563/7/90, processes 8, actions 8} | CONFIRMED | json graph captures identical; "Build completed successfully, 8 total actions" captured |
| GREEN, peak 5.1 GB | CONFIRMED | json peak_tree_mb 5126 |

### Row 4b, preadmit-g

| claim | verdict | evidence |
|---|---|---|
| applied 16:03 | CONTRADICTED | `G/gate-g/preadmit-g.applied` mtime 16:00:23; verify log written until 16:01 |
| 15 files + 8 new = 23 paths | CONFIRMED | `G/gate-g/preadmit-g.list` 23 lines |
| upgrade/phases oracles equal | CONFIRMED | oracle pass lines in `preadmit-g.verify.log` (about 87-104) |
| host-capacity 108/0 | CONFIRMED | `preadmit-g.verify.log:133-134` |
| library 41/0 | CONFIRMED | `preadmit-g.verify.log:209-210` |
| bazel 3 files green | CONFIRMED | `preadmit-g.verify.log:240-242` 18 pass, 0 fail, 3 files |
| repo-dashboard-rs:build (16:05, running) | CONTRADICTED | `G/gate-g/preadmit-g-dashboard.log` (mtime 16:06) has only `[capacity] waiting 357s ... 4 running, 1 ahead`; 0 Analyzing, Build or elapsed lines, so the run is queued, not running |
| verdict pending | CONFIRMED | consistent with the log |

## C. repair-g GREEN claims (`result-repair-g.md`)

Run times come from the pid files (`G/repair-g/dash1.pid` 11:53:26, `course1.pid` 12:01:28, `print1.pid` 12:08:09).

| claim | verdict | evidence |
|---|---|---|
| line 1 "PRODUCTS GREEN 12:21" | CONFIRMED for the three runs below | see the three rows |
| dashboard: generation 545 published, 4019 projects refreshed in 174 s | CONFIRMED | `G/repair-g/dash1.log:1625` |
| dashboard: "66 withheld owners healed in ONE build" | UNSUPPORTED | the log shows "66 projects planned by a child process" (`:1631`), no healed count |
| dashboard: 3 actions, 191 s, completed | CONFIRMED | `dash1.log:6659-6661` Elapsed 191.448 s, "3 processes: 1 internal, 2 worker", "Build completed successfully, 3 total actions" (a 2-action target at `:6591-6593` also completed) |
| dashboard run started after the repair edits | CONFIRMED for the edits with mtime before 11:53 | `🕰️generations/🧪️tests/🚧️unfounded-withheld/🟦️.ts` 11:47:56, `📥️inputs/🧮️digest/📥️read/🟨️.mjs` 11:40:18, `project-nodes/🟨️.mjs` 11:46:16 |
| dashboard run covers the final repair edits | UNSUPPORTED | edits the result lists as repair-g's changed later: `🧭️selection/🟦️.ts` 13:26:45 (unknownBuildTaskV1), `👁️observation/🟨️.mjs` 12:56:13, `📥️inference/🧭️script-inputs/🟨️.mjs` 13:11:58, `🕰️generations/🟨️.mjs` 13:43:37; the log does not say which version ran |
| course: generation 546 SEALED, 169 projects, 85 s | CONFIRMED | `G/repair-g/course1.log:2885` |
| course: 8 actions (1 disk-cache hit, 3 local, 3 worker), completed | CONFIRMED | `course1.log:22311-22312` |
| course: "before: never sealed, 170 re-inferred on every run" | UNSUPPORTED | history claim, not in these logs |
| print: generation 547, 353 projects, 148 s prepare | CONFIRMED | `G/repair-g/print1.log:4216` |
| print: 1 test PASSED in 307.7 s, completed | CONFIRMED | `print1.log:4605` "Build completed successfully, 8 total actions", `:4607` "PASSED in 307.7s", "1 test passes" |

## D. proof2-g GREEN claims (`result-proof2-g.md`)

| claim | verdict | evidence |
|---|---|---|
| proof 1 churn-g env-digest GREEN 7/7 | CONFIRMED | `G/proof2-g/p1/rows.txt` five runs rc=0; test counts `t1`..`t5` = 1, 1, 1, 1, 3 pass, 0 fail (sum 7) |
| gen 553 served | CONFIRMED with note | `p1/t1.log`, `p1/t5.log`: "generation 553 without verification (graph policy current)" |
| 5 official execs, wall 297 s | CONFIRMED | `p1/tree.json` wall 297, done true; rows sum 294 s |
| 2.2 GB tree, 5 procs | CONFIRMED | `p1/tree.json` peakMB 2173, maxProcs 5 |
| run started after batch 3 (13:12:46) | CONFIRMED | `p1/env.try1.log` at 13:13, `p1/t1.log` from 13:16 |
| batch-snapshot 2/2 with staged patch | UNSUPPORTED for the 2/2 count | the settle text "was edited ... ms ago" is present (`p1/t3.log` 8 matches, `t1`, `t2`, `t4` 1 each), but no per-law count is separable |
| (not in table) earlier tries env.try1 / env.try2 failed | CONFIRMED | `p1/env.try1.log:198`, `p1/env.try2.log:244`: "Build invocation failed (1): workspace:repo-exec:prototype"; not reported in the result file |
| proof 2 rust-g BLOCKED, dashboard build A exit 1 | CONFIRMED | `G/proof2-g/p5/dash-a.json` exit 1, started 13:43:58, wall 1277 |
| EBADF RED in dashboard build A | CONFIRMED | `p5/dash-a.log:5348`, `:5796`, `:6361` "failed to run rustc: EBADF" |
| about 1100 s capacity wait | CONFIRMED approximately | last `[capacity] waiting 1029s` in `p5/dash-a.log` |
| nx inputs ENOMEM | CONFIRMED | `p2/inputs-a.log` 2 ENOMEM matches |
| print-rs:test GREEN in gate-g 14:01-14:19 (same wrapper) | CONFIRMED | `G/gate-g/noop2-g-print3.log:2938` |
| proof 5 chain-g "QUEUED since 14:57, 1650 s+ at 15:32" | CONTRADICTED on start | `p5/course-noop1.json` started 15:05:06; admitted only after 2254 s (`p5/course-noop1.log:386`); killed by time at 2713 s, exit 1; no no-op timing recorded |
| (not in table) `p5/course-noop2.log` still running at 16:08 | CONFIRMED | file mtime 16:08, progress lines present; the result file does not list it |

## E. integ-g GREEN claims (`result-integ-g.md`)

Label caveat for all "test-unit" lines: the log never prints the label `test-unit` for the 1-test Bazel target `//:t_fab884dd6b374ba6`. The label appears only in the run-many argument line. That target's block also carries the failing journeys output and "Build did NOT complete successfully", so the label mapping is not evidenced.

| claim | verdict | evidence |
|---|---|---|
| c3d (14:21-15:12, wall 3081, rc 1) | CONFIRMED | `G/integ-g/cycles.tsv` |
| c3d build GREEN, 2 actions, 305 s | CONFIRMED | `G/integ-g/c3d.dash.log:5553-5555` Elapsed 305.263 s, "Build completed successfully, 2 total actions" |
| c3d authority 59/59 | CONFIRMED | `c3d.dash.log:6003-6006` "59 pass / 0 fail" |
| c3d dispatch 15/15 | CONFIRMED | `c3d.dash.log:6020-6021` "15 pass / 0 fail" |
| c3d test-unit PASSED in 494.8 s, 7 processes (2 cache hit, 3 internal, 4 local) | UNSUPPORTED (label) | numbers match `c3d.dash.log:7842-7845` for `//:t_fab884dd6b374ba6`, but the target is not named test-unit, and the same block says "ERROR: Build did NOT complete successfully" and "All tests passed but there were other errors during the build" |
| c3d test-journeys 33/35 RED | CONFIRMED | `c3d.dash.log:7695`, `:8031` "33 passed; 2 failed"; failing `maximum_native_document_stays_responsive` and `native_files_browse_edit_unicode...` at `:7964-7965`; `test-journeys:prototype` failed (Exit 1) at `:5942` |
| EBADF gone | CONFIRMED | 0 EBADF matches in `c3d.dash.log` and `c2.dash.log` |
| c2 (12:22-12:54, rc 1) build GREEN, 2 actions, 83 s | CONFIRMED | `G/integ-g/c2.dash.log:8249-8251` Elapsed 83.301 s, "Build completed successfully, 2 total actions"; the run began before batch 2 landed (12:45:53) |
| c2 test-unit GREEN (PASSED 220 s, 8 processes, 561 s) | UNSUPPORTED (label) | `c2.dash.log:8621-8625` numbers match for `t_fab884`, same label caveat; the block also has "Build did NOT complete successfully" |
| c2 test-journeys RED | CONFIRMED | `c2.dash.log:8484`, `:8558`, `:8632` test-journeys failed |
| c2 authority 57/2, withPlannedTestWorkersV1 not defined | CONFIRMED | `c2.dash.log:8536-8537` "57 pass / 2 fail"; the ReferenceError text occurs 4 times |
| course:dev headless ready, rc 0, wall 1026 s | CONFIRMED | `G/integ-g/c2.devcheck.out`; `c2.course-dev.log:3309-3318` task-started, "proctor listening on http://127.0.0.1:8791", "ready after 757 ms" |
| course:dev build 7 actions (1 disk cache hit, 3 internal, 3 local) | CONFIRMED | `c2.course-dev.log:3296-3297` |
| no `stale build.graph.released-read` | CONFIRMED | 0 matches in `c2.course-dev.log` |
| dashboard run EN smoke, 19 s, 0 churn, 0 active tasks | CONFIRMED | `G/integ-g/c2b.dashrun.out` "smoke en wanted-hits=2 churn-lines=0 wall=19s ... 0 active tasks" |
| dashboard run DE smoke, 23 s, 0 active tasks | CONFIRMED | `c2b.dashrun.out` "smoke de ... wall=23s ... 0 aktive Aufgaben" |
| EN boot screen `semio / dashboard / Tasks` | UNSUPPORTED | string not found in `c2b.run-en.boot.txt`; the smoke matched "New task" (2 hits) |
| daemon stop | UNSUPPORTED | `c2b.dashrun.out` shows status only, no stop output |
| projektetage dev 6050: title 33. Projektetage, 0 console errors, root mounted | CONFIRMED | `G/integ-g/c2.projektetage.verdict` pass true, consoleErrors 0, rootChildren 1 |
| projektetage: port freed | UNSUPPORTED | not in the verdict file |
| dev routes "current tree, gate-g batches 1-2 applied" | CONTRADICTED | batch 3 (noop2-g) landed 13:12:46 (`G/gate-g/noop2-g.applied`), before the smoke runs at 13:14 and 13:16; the first smoke attempt failed with churn from that landing (`c2.dashrun.out`) |

## F. Summary

- gate-g rows: 4 batch rows (zt-g, xplat-g, noop2-g, phase-g) and their GREEN verdicts CONFIRMED for build outcome, exit, wall, peak and graph; 3c and 4b have contradictions (load and queue depth, applied time, and "running" that is still queued); noop2-g witness 6/0 CONTRADICTED; several attributions and gofmt / tsc "known" claims UNSUPPORTED.
- repair-g: the three product runs are CONFIRMED against their logs; the final-edit coverage claim is UNSUPPORTED because several listed edits post-date the runs.
- proof2-g: proof 1 CONFIRMED; proof 5 start time CONTRADICTED and its outcome is missing from the table.
- integ-g: build, authority, dispatch, journeys RED and dev routes CONFIRMED; every "test-unit GREEN" is UNSUPPORTED because the log does not name that target; the dev-route tree claim is CONTRADICTED (batch 3 had landed); c2 and c3d straddle batch landings.
- Recommended before any gate verdict relies on these: name the Bazel targets in the cycle logs (or map t_fab884 explicitly), re-run the witness law after the fix and record it, and stop the cycles from spanning a batch landing.
