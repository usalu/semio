# Build Triage, Wave 3 (W3-BUILD-TRIAGE)

Source: `🗑️generated/build-runner/runner.log`, `done … exit 1` lines from 14:00 onward. Diagnostics read from `🗑️generated/build-<code>/<n>.txt` (ANSI stripped).

## Run: norm-en1995-rs run 26 (15:01:25, exit 1, dur 176s, k=0)

- Class: infra, not a compile error. No `error[E…]` lines and no `cargo-artifacts-*.log` reached.
- Failure: `build:graph-plan was killed by SIGKILL` (`OwnedCommandFailure`, `owned-execution/🟦️.ts:338`), then `repository:command failed (1)`.
- Context: the run was admitted with "6 of 8 jobs" while swap was fully used (`free -m`: swap 8947 of 8947 MB used, 1264 MB free RAM at 15:0x). Likely OOM or memory-guard kill of the graph-plan process.
- Action: none by triage. Coordinator to decide on rerun after memory clears.

## Run: norm-en1996-rs run 15 (14:24:21, exit 1, dur 387s, k=0)

- Class: infra, NX file watcher limit. No compile errors.
- Failure: `NX Daemon Server … Server stopped because: "Error: watcher lost coverage: watch-limit"`, then `Original detached daemon refused: watcher lost coverage: watch-limit`, then `Owned Nx invocation failed (1): nx run @semio-tech/norm-en1996-rs:build --output-style=static --parallel=7`.
- Host limits at 15:0x: `max_user_watches` 524288, `max_user_instances` 1024. The NX daemon still hit watch-limit, so the cap is most likely per-process or per-daemon, not the sysctl value. Not verified further (read-only triage).
- Action: none by triage. Rerun with the NX daemon disabled, or with fewer parallel jobs, is a coordinator decision.

## Run: norm-en1997-rs run 16 (14:34:21, exit 1, dur 894s, k=0)

- Class: infra, same NX watch-limit failure as en1996 run 15.
- Before it failed, it waited about 100 s on the capacity gate ("not enough free memory", behind en1996 running 7 jobs at 6161 MB).
- Failure: `Server stopped because: "Error: watcher lost coverage: watch-limit"` at 13:32:56 (log timestamp), then `Owned Nx invocation failed (1): nx run @semio-tech/norm-en1997-rs:build --output-style=static --parallel=5`.
- Action: none by triage.

## Summary

| Run | Time | Class | Compile errors |
|---|---|---|---|
| en1995 run 26 | 15:01 | infra: SIGKILL in graph-plan (memory) | 0 |
| en1996 run 15 | 14:24 | infra: NX watch-limit | 0 |
| en1997 run 16 | 14:34 | infra: NX watch-limit | 0 |

No trivial one-line fixes were applied, since no compile errors were reported in these runs. No file was edited, and no Cargo.lock change was made.

## Open for coordinator

- The NX daemon's `watcher lost coverage: watch-limit` failure appeared in two of three failed runs. A reproducible mitigation (daemon off, or lower `--parallel`) is needed before more reruns.
- Swap was fully used at 15:0x. Reruns while swap is saturated are likely to be SIGKILLed again.
- Earlier failed runs with compile output (k>0, e.g. en1995 run 25 at 13:16 with k=2, en1997 run 15 at 12:53 with k=11) were before this wave and are not triaged here. Their cargo-artifacts logs are the next thing to read if the coordinator wants them.
