# Wave 1125 Audit A7 (read-only)

Snapshot taken 2026-11-25 20:08-20:09 CET. Sources: `/Users/Shared/semio-store/v1/origins/<origin>/` (ready.json, ready-timings.json, newest ready-log-*.txt and ready-log-*-failed.txt). "Since 16:00" counts use ready-timings.json, which retains only the last 40 runs per worker (earliest retained run listed below), so counts can under-report.

## Not-Ready workers

### g1-m02 (macOS, stage workspace, attempt 10)
- Current: stage workspace (5/5), state running, adopted pid 90268. Current attempt startedMs 20:06:54 CET; process log says "still working after 1465 s", last phase door.script. Peak 5492 MB.
- Attempts (since 16:00, 3 retained runs): workspace stage-failed x3 (attempts 8, 9, 10). Last failed run 17:42 to 19:24 CET (exit 1, watchdog nx:source-verification).
- Newest failure (ready-log-workspace-failed.txt, 19:24 CET): "Internal error: Owned Nx invocation failed (1): nx run-many --projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp --targets=build --parallel=2" then "repository:command failed with exit code 1". No E0277 line and no "[nx] failed task" block captured; the din4108-rs build task ran total=1999 s before failing.
- Classification: NEW (no error line captured; cannot confirm F37 E0277 from this log).
- [units] miss lines: none.

### g1-m03 (macOS, stage workspace, attempt 3)
- Current: stage workspace, state running, adopted pid 26595. Current attempt startedMs 20:07:35 CET; "Reading the project graph slice (2942 s)", peak 10473 MB.
- Attempts (since 16:00): workspace stage-failed x2 (attempts 2, 3). Last failed run 18:38 to 19:17 CET.
- Newest failure (19:17 CET): error[E0277]: the trait bound `Din4108Mutation: ArtifactPersistedOperation` is not satisfied (semio-s-artifact-norm-din4108-editor, lib). Nx: "1 task(s) did not succeed", failed task @semio-tech/norm-din4108-rs:build:prototype; OwnedCommandFailure native:owner-command failed (1).
- Classification: din4108 E0277 (F37, in progress).
- [units] miss lines: none.

### g1-m04 (Linux, stage dependencies, waiting)
- Current: stage dependencies (4/5 index 3), state waiting, attempt 22, refusal code disk-collecting: needs 428796 inodes free in /home/deck/semio/.cache, only 162529 free, 266267 missing. Collector closure-layers running. ready.json updated 20:08 CET.
- Attempts: no retained runs since 16:00 (ready-timings last updated 12:53 CET). Last failed run 12:49 to 12:53 CET, attempt 22.
- Newest failure (ready-log-dependencies-failed.txt, 12:53 CET): "error: failed applying patch file: EACCES: Permission denied (open())", "error: failed to apply patchfile (patches/1.patch)", then "Internal error: Fixed Store dependency installer failed (reference 8ef44f48)".
- Classification: NEW (EACCES on patch apply in the dependency installer; not one of the listed classes). Current blocker is the inode shortage, not a listed class.
- [units] miss lines: none.

### g1-m05 (Windows, stage dependencies, attempt 49)
- Current: stage dependencies, state running, adopted pid 33464, attempt 49. Process log: "still working after 1474 s", last phase source.capture (walked 128 files). Peak 7398 MB. ready.json was caught mid-write once (truncated JSON); re-read ok.
- Attempts (since 16:00): dependencies stage-failed x2 (attempts 48, 49), last 18:25 to 19:16 CET.
- Newest failure (ready-log-dependencies-failed.txt, 19:16 CET, stage ended at 18:16:58): "process 24028 stands in the process table with a start time 17 s away from the recorded one ... tolerance 15 s ... judged another process that reused the pid and the stage is ended".
- Classification: NEW (pid reuse / process-identity mismatch on Windows; not a listed class).
- [units] miss lines: none.

### g1-m07 (macOS, stage workspace, attempt 3)
- Current: stage workspace, state running, adopted pid 69756. Current process "[repository:command] still running after 710 s". Peak 10662 MB.
- Attempts (since 16:00): workspace stage-failed x2 (attempts 2, 3). Last failed run 18:20 to 19:20 CET.
- Newest failure (19:20 CET): error[E0277]: the trait bound `Din4108Mutation: ArtifactPersistedOperation` is not satisfied; then native:owner-command failed (1), Internal error: Owned Nx invocation failed (1) for @semio-tech/norm-din4108-rs:build:prototype.
- Classification: din4108 E0277 (F37, in progress).
- [units] miss lines: none.

### g1-m08 (macOS, stage workspace, attempt 24)
- Current: stage workspace, state running, adopted pid 59423. Current process "[repository:command] still running after 2231 s". Peak 10208 MB.
- Attempts (since 16:00): workspace stage-failed x2 (attempts 23, 24). Last failed run 17:31 to 18:49 CET.
- Newest failure (18:49 CET): error[E0277]: the trait bound `Din4108Mutation: ArtifactPersistedOperation` is not satisfied (norm-din4108-editor); native:owner-command failed (1); Internal error (1) with "[nx] failed task @semio-tech/norm-din4108-rs:build:prototype, its last 0 output lines".
- Classification: din4108 E0277 (F37, in progress).
- [units] miss lines: none.

### g1-m09 (no runtime state)
- Only fleet-auth.json (dated 11-14 02:13). No ready.json, no timings, no logs. Cannot audit; worker appears not to be running.

### g1-m12 (macOS, stage workspace, attempt 12)
- Current: stage workspace, state running, adopted pid 542. Process "still working after 590 s; last phase started: source.served". Peak 6323 MB.
- Attempts (since 16:00): workspace stage-failed x8 (attempts 5 to 12). Last failed run 19:18 to 19:39 CET.
- Newest failure (ready-log-workspace-failed.txt, 19:39 CET): error[E0277]: the trait bound `Din4108Mutation: ArtifactPersistedOperation` is not satisfied; then native:owner-command failed (1). The final entry in that file is "Internal error: Owned Nx invocation failed (130) ... --parallel=5" (exit 130 = SIGINT, looks like a supervisor cancel, not a compile error).
- Classification: din4108 E0277 (F37, in progress).
- [units] miss lines: none.

## Ready workers (readyMs, local CET)
- g1-m01: 2026-11-25 20:00:20
- g1-m06: 2026-11-25 18:05:03
- g1-m10: 2026-11-25 18:11:09
- g1-m11: 2026-11-25 18:04:29
- g1-m13: 2026-11-25 18:19:17
- g1-m14: 2026-11-25 17:58:59
- g1-server: no ready.json (not in the requested list; not assessed).

## Summary of the audit
- Known classes seen: din4108 E0277 (F37) on g1-m03, g1-m07, g1-m08, g1-m12.
- NEW classes: g1-m02 (no error line captured, cause unidentified), g1-m04 (EACCES on patch apply, dependencies stage, 12:53 CET), g1-m05 (pid reuse on Windows, dependencies stage, 19:16 CET).
- Not observed in any of the eight workers: crate-refused (F1), ENOENT packages path (F1), partition owner (F22), funding journal (F32), stale closure-graph (F23/F27), syntax "Unexpected }" (17:27), cargo build dir vanished (F34).
- [units] miss ... differs in ... (F40): no such lines in any of the eight workers' logs.
