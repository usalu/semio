# 📋 Audit of wave 1024d GREEN/PASS claims (claims-d, read-only, 10-25)

Scope: GREEN/PASS verdicts in `C/result-*-d.md` of wave 1024d lanes (course-d, cwarm-d, nx-d, dash-d, dtest-d, print-d, snap-d, lockc-d, churn-d, deps-d) and their successors in the same product lanes (dash2-d, dash3-d, dtest2-d, nx2-d, nx3-d, churn2-d, pres-d, pimg-d). Out of scope: 10-25 lanes (crel, cpets, cui, ctest, default, dbgv, depr, cinp, ckey*, plugc, term, lang, rtest, fin), audited only as event sources.

Paths below are relative to `C` (`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`). `G` = `C/🗑️generated`. mtime is local CET as shown by `stat`.

## Shared edits used for the staleness test

| id | time | event (coordination.md) | what it can invalidate |
|---|---|---|---|
| E1 | 23:14 10/24 | pimg-d products-image fix; launcher re-keys every action | verdicts from runs before 23:14 used the old action keys and the old products image path |
| E2 | 03:10 10/25 | dbg-d `[DEBUG]` sweep: 128 files over `🦑️repo/**`, `🏃️process`, `🖱️ui`, course, print, presentation, pets, bericht | every verdict whose closure includes these trees and whose log predates 03:10 tested source that has since changed |
| E3 | 03:30-04:00 10/25 | depr-d: bun.lock rewrites, removed `color` (ui-styling oracle, "RE-VERIFY ui-styling:test"), `npm-run-path`, `terser` | verdicts depending on workspace locks or on ui-styling / os browser-bundle |
| E4 | ~04:00-04:42 10/25 | cinp-d: test-source exclusion added to build/generate/typegen input inference (inputs 17.7k to 10.7k) | every pre-04:00 Bazel/Nx build verdict: action input sets changed |

Result: no GREEN/PASS in these lanes is backed by a log written after 03:10 10/25. The only later runs in these lanes (nx3-d `dtest5` to `dtest12`, 03:35-04:30) are all rc=1 (see the flags). Every row below is therefore stale-by-later-edit, unless noted otherwise.

## Table

Columns: lane / target / claimed (result-file time) / log (mtime) / evidence OK? / stale-by-later-edit?

| # | lane | target | claimed | log | evidence OK? | stale-by-later-edit? |
|---|---|---|---|---|---|---|
| 1 | course-d | `course:e2e` test-e2e, 76/76 (MILESTONE 00:13) | 00:12 | `G/e2e-orch/e2e-1024-course-d6/course/e2e-1-test-e2e.log` (00:12); also `G/e2e-course-e2e-1024-course-d6-course-test-e2e.log`; `G/course-d/orch-d6.log` "PASS rc=0 45m33s" | YES: `[dev] 76 passed (39.1m)`, 0 failed | YES: E2 (course), E4 |
| 2 | course-d | cold: proctor-build, typecheck, test, build, check, proctor-test | 22:42 | `G/course-d/orch-d2.log` lines 45-120 (21:03-22:09) | YES, six `PASS rc=0`; the file's overall rc is 1 because warm steps failed | YES: E2, E4 |
| 3 | course-d | warm | not claimed GREEN | `G/course-d/orch-d2.log` 22:14-22:44 `FAIL` (1 and 7 spawns) | n/a (honest FAIL, owned by cwarm-d) | n/a |
| 4 | dash-d | Bazel `repo-dashboard-rs:build` (1a) | 16:28 | `G/dash-d/bz2-1.log` (16:27), `G/dash-d/bz2.done` = "ok on try 1" | YES: `Build completed successfully, 3 total actions`; no explicit rc line, done marker only | YES: E2 (repo/**) |
| 5 | dash-d | CLI smoke EN+DE (`daemon start`, status, `stop ghost` negative) | 16:34 | `G/dash-d/task-e2e.out` (16:34), final `rc=0`; line 431 `rc=2` is the intended unknown-task refusal | YES | YES: E2 |
| 6 | dash-d | launcher nested rebuild pf1 | 17:12 | `G/dash-d/pf1-1.log` (17:12) | YES: two `Build completed successfully`; no rc line | YES: E2 |
| 7 | dash2-d | SMOKE GREEN EN+DE, daemon, tasks | 20:40 | `G/dash2-d/task-e2e.out` (20:36), `rc=0` | YES | YES: E2 |
| 8 | dash2-d | "two Rust journeys `a_task_speaks_the_language_*` PASS in official Bazel test-journeys" | 22:55 | `G/dash2-d/jt9-1.log`, `jt10-1.log`: those two tests `ok`; target: `jt10-1.log` `test result: FAILED. 28 passed; 7 failed`, rc=1 (22:48) | PARTIAL: per-test ok; the official target was RED | YES: E2; superseded by row 10 |
| 9 | dash3-d | official `repo-dashboard-rs:test-journeys` GREEN (jt2) | 00:05 | `G/dash3-d/jt2-1.log` (23:46): `test result: ok. 35 passed; 0 failed`, `Build completed successfully, 3 total actions`; `jt2.done` ok; `jt1-1.log` (23:40, done ok) | YES (the bun part 15/15 is not separately checked) | YES: E2 |
| 10 | dash3-d | dashboard `:build` (b1) | 00:05 area | `G/dash3-d/b1-1.log` (23:50), `b1.done` ok, `Build completed successfully` x2 | YES | YES: E2 |
| 11 | dtest-d | 16 dashboard cargo crates GREEN, graph-free | 17:06 | `G/dtest-d/t1.sum` (17:06), `rc=0` lines | YES for the lines seen (per-crate lines not all read) | YES: E2 |
| 12 | dtest-d | 23 other `repo-*-rs` + os-native-runtime-reader GREEN, "ALL 40 CARGO TEST CRATES GREEN" (17:35) | 17:35 | `G/dtest-d/g1.sum` rc=0; `G/dtest-d/g3.sum` rc=0; **`G/dtest-d/g2.sum` rc=1** (17:20): `6 passed; 6 failed` (export_snapshot_content_hash, payload_golden_encoding, store_* tests). No later g2 run in the folder | **NO for g2**: the "ALL 40 GREEN" claim is contradicted by its own log | YES: E2 |
| 13 | dtest-d | `repo-dashboard-rs:test-source-freshness` GREEN (o1) | 17:48 | `G/dtest-d/o1.log` (17:46): `//:t_47c3d... PASSED in 7.2s`, `1 test passes` | YES | YES: E2 |
| 14 | dtest-d | `test-unit` PASSED on Bazel 141 s (o3) | 18:30 | `G/dtest-d/o3.log` (18:23): `t_fab884... PASSED in 141.3s` (test-unit); `t_376488... FAILED in 7.0s` (dispatch) | YES for test-unit; dispatch was RED in o3 and PASSED in o4 (`G/dtest-d/o4.log` 18:48, `t_376488... PASSED in 7.0s`; mapping inferred from timings) | YES: E2 |
| 15 | dtest-d | `test-authority` 58/58 | 19:20 | `G/dtest-d/auth-direct2.log` (19:10): `58 pass / 0 fail`, `Ran 58 tests`, `rc=0`. Earlier `auth-direct1.log` (18:51) rc=1, 15 fail (superseded) | YES (direct `bun test`, not the Bazel target) | YES: E2 |
| 16 | print-d | `print-rs:test` Bazel | 16:32 | `G/print-d/printrsbz1.raw` (16:32): `PASSED in 359.2s`, `rc=0` | YES | YES: E1, E2 |
| 17 | print-d | `print:check` Bazel | 17:00 | `G/print-d/pcheckbz1.raw` (17:00): `Build completed successfully`, `rc=0` | YES | YES: E1, E2 |
| 18 | print-d | `print:build` incl. flyer, five documents | 18:01 | `G/print-d/pbuildbz1.raw` (18:01): `Build completed successfully, 5 total actions`, `rc=0` (line 7450/7455). Mid-run `ConcurrentEditErrorV1` at line 1309 (env:BUN_JSC_forceRAMSize, 5 snapshot attempts) before the final success | YES (final rc=0; mid-run error noted, not a failure of the final result) | YES: E1, E2 |
| 19 | print-d | `print:test` Bazel | 21:37 | `G/print-d/ptestbz5.raw` (21:37): `PASSED in 64.9s`, `rc=0` | YES | YES: E1, E2 |
| 20 | print-d | bericht `test` | 23:29 | `G/print-d/btestbz8.raw` (23:29): `PASSED in 4.3s`, `rc=0` | YES | YES: E2 |
| 21 | print-d | bericht `check-generated` | 22:12 | `G/print-d/bcgbz3.raw` (22:12), `rc=0` | YES | YES: E1, E2 |
| 22 | print-d | bericht `akteursnetz-validate` | 22:22 | `G/print-d/banbz3.raw` (22:22), `rc=0` | YES | YES: E1, E2 |
| 23 | print-d | bericht `build-kompaktbericht` | 22:30 | `G/print-d/bkbz3.raw` (22:30), `rc=0` | YES | YES: E1, E2 |
| 24 | print-d | bericht `build-zwischenbericht` | 22:40 | `G/print-d/bzbz3.raw` (22:40), `rc=0` | YES | YES: E1, E2 |
| 25 | print-d | bericht `build-forschungsbericht` | 22:49 | `G/print-d/bfbz3.raw` (22:49), `rc=0` | YES | YES: E1, E2 |
| 26 | print-d / nx2-d | `print:test` on default (Nx) | RED 23:34 (`ptestdef1`); GREEN 01:35 (nx2-d print27) | `G/print-d/ptestdef1.raw` rc=1 (Nx steady-publication fault, superseded); `G/nx2-d/print27.log` (01:35): `Successfully ran target test for project @semio-tech/print and 13 tasks`, `rc=0` | YES | YES: E2 (print closure, log 01:35 < 03:10) |
| 27 | pres-d | `presentation:test` Bazel | 17:58 | `G/pres-d/ptbz1.raw` (17:58): `PASSED in 7.0s`, `rc=0` | YES | YES: E2 |
| 28 | pres-d | `presentation-react:test` Bazel | 18:21 | `G/pres-d/prbz2.raw` (18:21): `PASSED in 17.2s`, `rc=0` (prbz1 rc=1 superseded) | YES | YES: E2 |
| 29 | pres-d | `mit-bestand-praesentation-projektetage:build` | 18:30 | `G/pres-d/pjbbz2.raw` (18:30), `rc=0` | YES | YES: E2 |
| 30 | pres-d | `mit-bestand-praesentation-projektetage:test` | 19:32 | `G/pres-d/pjtbz3.raw` (19:32): `PASSED in 8.7s`, `rc=0` | YES | YES: E2 |
| 31 | pres-d | dev smoke 6050 (projektetage) | 18:02-18:07 | `G/pres-d/smoke2/verdict.json` (18:07): `pass: true`, `consoleErrors: 0`; `console.txt` has no error lines | YES | YES: E2 |
| 32 | nx2-d | course `typecheck` on Nx | 21:23 (course2) | `G/nx2-d/course19.log` (00:02): `Successfully ran target typecheck ... 6 tasks`, `rc=0`; `course20.log` (00:09) same, `rc=0`. `course2.log` (21:24) exists, not read | YES for course19/20 | YES: E2 (course), E4 |
| 33 | nx2-d | `presentation:test` on Nx | 00:20 | `G/nx2-d/pres21.log` (00:11): `Tests 56 passed (56)`, `Successfully ran target test`, `rc=0` | YES | YES: E2 |
| 34 | nx3-d | `repo-dashboard-rs:build` on Nx (#2) | 02:19 | `G/nx3-d/dash5.log` (02:18): `Successfully ran target build ... and 1 task it depends on`, `rc=0` | YES | YES: E2 (repo/**) |
| 35 | nx3-d | `architecture-course:build` on Nx (#3) | 02:40 | `G/nx3-d/course2.log` (02:39): `Successfully ran target build ... 6 tasks`, `rc=0` | YES | YES: E2, E4 |
| 36 | nx3-d | dashboard test-execution / dtest series on Nx (after 03:35) | no GREEN claim; 04:20 "daemon-slot race fixed" | `G/nx3-d/dtest5.log` to `dtest12.log` (03:35-04:30): all `rc=1`. dtest5/9: `graph: 66 projects planned by a child process that has exited`; dtest12: `Exact original descendant creator identity required` | RED (no claim; current state is red) | n/a |
| 37 | nx-d | Nx faults K-O, 5 fixed | no GREEN claim | `G/nx-d/*` | no PASS claim, nothing to audit | n/a |
| 38 | cwarm-d | warm = cache hits | "STOPPED SHORT" (18:57), no GREEN claim | `G/cwarm-d/pairs.json` only | no GREEN claim | n/a (see cinp-d, out of scope) |
| 39 | snap-d | framework law `sqlite_snapshot_codec_default_receiving_validators_allocate_only_through_the_original_receiving_port` GREEN | 16:20 | `G/snap-d/kernel2.raw` (16:20): `test ... ok`; suite `FAILED. 55 passed; 5 failed` (5 pre-existing reds, listed in result) | PARTIAL: the named law is ok; suite is 55/60 | UNKNOWN: snap-d tree (framework/os store) is not in the E2 list; no later edit cited |
| 40 | snap-d | "other sqlite-snapshot plugins' tests: fallout fixed" | 21:45 (FINAL) | `G/snap-d/verdicts.tsv` (21:39): only codec-registry `passed=2`; the other 15 crates `passed=0`, `errorLines` 2-3 (compile errors) | NO: not verified; result says "no plugin artifact crate compiles at HEAD" | n/a |
| 41 | lockc-d | contended custody proof with real `bun repo cargo` | 16:30 | `G/lockc-d/contended1.log` (16:23): `custody released`, `cargo rc=0 wall=52s`; `c1.cargo.log` | YES | YES: E2 (framework/process) |
| 42 | lockc-d | `resource-leases` GREEN (after cwd fix) | 17:55 | `G/lockc-d/s2-resource-leases.log` (17:18) `rc=0`; `s1-resource-leases.log` (16:59) `rc=1` superseded | YES | YES: E2 |
| 43 | lockc-d | `cargo-build-custody` GREEN | 17:55 / 18:30 | `G/lockc-d/s1-cargo-build-custody.log` (17:12) `rc=0`; `s6-cargo-build-custody.log` (18:25) `rc=0` | YES | YES: E2 |
| 44 | lockc-d | `cargo-build-lease` rc 0 | 16:45 (s1); **18:30 (FINAL)** | `G/lockc-d/s1-cargo-build-lease.log` (16:40) `rc=0` (pre-lease-hoist); `G/lockc-d/s6-cargo-build-lease.log` (18:26) `rc=143`, terminated by SIGTERM | YES for 16:40 only; **the 18:30 "rc 0" has no passing log** | YES: E2; the 16:40 log predates the lease hoist (17:10-18:30) |
| 45 | lockc-d | `cargo-overlap` rc 0 (s5) | 18:30 | `G/lockc-d/s5-cargo-overlap.log` (17:51) `rc=0`; s2/s3/s4 `rc=1` superseded | YES | YES: E2 |
| 46 | churn-d | live proof pf10 (print `repo-exec` flyer under peer edit) | 19:25 | `G/churn-d/pf10.run.log` (19:18): `[semio] executed @semio-tech/print:repo-exec:prototype (1/1)`, `[script] ... complete`; `G/churn-d/pf10.rc` = `rc=0` | YES | YES: E2 (print/flyer, process) |
| 47 | churn2-d | live proof A pf12 (print `repo-exec` viz-api, saturated fleet) | 21:42 | `G/churn2-d/pf12.run.log` (21:41): `[semio] executed ...` 1 match, `complete`; `pf12.rc` = `rc=0` | YES | YES: E2 |
| 48 | churn2-d | live proof B pb1 (dashboard `:build` under two peer edits) | 22:09-22:22 | `G/churn2-d/pb1.run.log` (22:22): `Build completed successfully` x2; `pb1.rc` = `rc=0` | YES | YES: E2 |
| 49 | deps-d | `presentation:test` on Bazel (lock sync, run4) | 17:58 | `G/deps-d/run4.log` (17:49): `PASSED in 7.8s`, `Executed 1 out of 1 test: 1 test passes` | YES | YES: E3 (lock rewrites), E2 |
| 50 | deps-d | run5 Bazel build after stale-lock preflight | 18:02 | `G/deps-d/run5.log` (18:01): `Build completed successfully, 3 total actions` | YES | YES: E3, E2 |
| 51 | pimg-d | `presentation:test` after products-image fix (LIVE 23:14) | 23:25 | `G/pimg-d/verify2.log` (23:25): `PASSED in 5.0s`, `1 test passes`; `verify3.log` (23:41): `Build completed successfully, 3 total actions` | YES | YES: E2 (presentation), E4 |

## Claims without a log, or contradicted by the log

1. **dtest-d "ALL 40 cargo test crates GREEN" (17:35)**: `G/dtest-d/g2.sum` at 17:20 is rc=1 with 6 failing `store_*`/golden tests, and no rerun exists in the folder. The claim is unsupported as written.
2. **dtest-d "dispatch FAILED once" then fixed**: o3 shows dispatch FAILED (row 14); the fix is shown only by o4 (18:48) timing. Acceptable, but the mapping is inferred.
3. **lockc-d FINAL 18:30 "cargo-build-lease rc 0"**: no log at or after 18:30 passes. The only later log, `s6-cargo-build-lease.log`, is rc=143 (killed). The rc 0 is from 16:40, before the lease hoist.
4. **course-d "76/76 deterministic, no retries" (00:13 MILESTONE)**: one run passed 76/76 (row 1). The same result file's OPEN 1 says the four "200% text" specs failed 2 of 4 in each of d8/d10/d12. Determinism is not shown; cpets-d owns the open item.
5. **dtest2-d "14 projects PASS in one official run (`fin`)"**: `G/dtest2-d/fin.a1.log` (00:12) has 13/14 PASSED with `t_1d2f87b387be47e0` (repo-tree-rs) FAILED in 105.9 s (`fin.rc` = 1). The 14th passed only in the re-run `fin2` (00:18, `fin2.rc` = 0). Two runs, not one. Both are before E2.
6. **dash2-d "two Rust journeys PASS in official Bazel test-journeys"**: the two named tests are `ok` in jt9/jt10, but the official target was red (`jt10-1.log`, 7 failed, rc=1). The later green official run is dash3-d (row 9).
7. **nx3-d dtest series (03:35-04:30)**: all eight runs rc=1 (rows 36). No GREEN is claimed, but the Nx dashboard test lane has no passing log after the sweep.
8. **snap-d plugin fallout (row 40)**: not verified; 15 of 16 plugin crates fail to compile at HEAD per `verdicts.tsv`.

## Staleness summary

- Every GREEN in rows 1-51 has a log older than 03:10 10/25 (E2), so each needs a re-run on the current tree before it counts as a current verdict. Rows whose closure includes `🏃️process` or `🦑️repo/**` (lockc-d, churn-d, dtest-d, dash-d/dash2-d/dash3-d) are the most exposed.
- Rows touching the lock files or ui-styling (deps-d rows 49-50; any `ui-styling:test`) also need E3 re-verification.
- Rows from before 23:14 10/24 (print-d rows 16-25, dash-d row 4-6, dtest-d rows 11-15) ran with the pre-fix action keys (E1). Their evidence is for the old keys.
- E4 (cinp-d input exclusion) invalidates all pre-04:00 build/Nx verdicts (rows 1, 2, 32, 35, 51) for the input sets they were built with.

## Not audited

- Out-of-scope 10-25 lanes (crel, cpets, cui, ctest, default, dbgv, depr, cinp, ckey*, plugc, term, lang, rtest, fin); depr-d `G/depr-d/ratchet-final2.log` (04:13) was not read; cinp-d's warm "zero re-execution" proof (`wl1`) was not read.
- `G/course-d/focus-d*.log` and `G/dtest-d/t1-*.log` individual crate logs were not read line by line; only the summary files and rc lines.
- No builds, no `bun repo`, no git, no ticket tools. Read-only except this report.
