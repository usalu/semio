# 🧟 Stale Process Audit: COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR

Read-only audit taken 2026-11-22 00:14 to 00:16 CET. Nothing was killed or edited. Only this file was written.

Scope: processes whose command line references the ticket folder, its `🗑️generated` folder, or the `node -e` / `bun run-browser-io.ts` wrappers writing `*.log` / `*.exit` / `*.pid` there. Live PIDs were also matched against the `*.pid` files in `🗑️generated` (72 files, 6 live).

## Process table

| PID | PPID | Elapsed | CPU % | RSS | Writes | Last meaningful line (non-heartbeat) | State | Recommendation |
|---|---|---|---|---|---|---|---|---|
| 26630 | 26628 | 05:07:28 | 0.9 to 2.4 | 32 MB | `host.log` (leader) | `reading the owners (round 1) ... 3109 of 3600` at elapsed 10694 s | progressing | keep |
| 26628 | 1 (orphan) | 05:07:28 | 0.0 | 1 MB | `host.log` (bun wrapper) | `still working after 18455 s; last phase started: door.script` | orphaned wrapper, child alive | keep (leader's wrapper) |
| 26629 | 26627 | 05:07:28 | 0.2 | 20 MB | `xlsx-host.log` | `waiting for the load of this checkout ... taken (pass 2) 126 of 1253` at 18509 s | wedged (waiting on leader) | stop |
| 26627 | 1 (orphan) | 05:07:28 | 0.0 | 1 MB | `xlsx-host.log` (bun wrapper) | same as 26629 | orphaned wrapper | stop |
| 26899 | 1 (orphan) | 05:07:17 | 0.0 | 4 MB | `pane-on-ready.log` (last write 2026-11-21 19:06) | `Waiting for managed host 26629 at http://127.0.0.1:6272/` | wedged, no listener on 6272 | stop |
| 26900 | 1 (orphan) | 05:07:17 | 0.1 | 6 MB | `acceptance-de-on-ready.log` (19:06) | `Waiting for managed host 26630 at http://127.0.0.1:6070/` | wedged, no listener on 6070 | stop |
| 26902 | 1 (orphan) | 05:07:17 | 0.0 | 4 MB | `acceptance-on-ready.log` (19:06) | same as 26900 | wedged | stop |
| 26904 | 1 (orphan) | 05:07:17 | 0.1 | 6 MB | `io-matrix-on-ready.log` (19:06) | same as 26900 | wedged | stop |
| 27395 | 27393 | 05:06:40 | 0.3 to 0.8 | 25 to 28 MB | `office-browser-after-checkpoint.log` | `waiting for the load of this checkout ... taken (pass 2) 126 of 1253` at 18476 s | wedged (waiting on leader) | stop |
| 27393 | 1 (orphan) | 05:06:53 | 0.0 | 16 KB | `office-browser-after-checkpoint.log/.exit/.pid` (node wrapper) | (wrapper only) | orphaned wrapper | stop |
| 25382 | 25380 | 04:07:25 | 1.9 to 2.9 | 41 to 58 MB | `xlsx-managed-space.log` | `waiting for host capacity (swap use is above its ceiling)` / `waiting for the load ... running ahead` (5958 + 469 wait lines) | wedged (waits 4 h) | stop |
| 25380 | 1 (orphan) | 04:07:38 | 0.0 | 16 KB | `xlsx-managed-space.log/.pid` (node wrapper) | (wrapper only) | orphaned wrapper | stop |
| 20414 | 20408 | 04:11:10 | 0.2 to 0.9 | 21 MB | `xlsx-managed-renderer-wasm.log` | `checking the file inventory of the tree ... 692 of 600` (beyond expected), then `waiting for run-many: swap use above ceiling` | wedged (waiting, over expected time) | stop |
| 20408 | 1 (orphan) | 04:11:10 | 0.0 | 16 KB | `xlsx-managed-renderer-wasm.log/.pid` (node wrapper) | (wrapper only) | orphaned wrapper | stop |
| 13568 | 13563 | 04:15:35 | 0.9 to 1.3 | 34 to 37 MB | `office-exact-managed-components.log` | `native-build 1320 unit(s) compiled, 0 reused, 1 running ... 38.3 min` | progressing, compile count flat at 1320 for last ~2 min | keep (verify) |
| 13563 | 1 (orphan) | 04:15:49 | 0.0 | 16 KB | `office-exact-managed-components.log/.pid` (node wrapper) | (wrapper only) | orphaned wrapper, child alive | keep until 13568 ends |
| 55560 | 13568 | 38:24 | 0.2 | 16 MB | (child of 13568, bun eval hashing) | n/a | active child tree | keep with 13568 |
| 55956 / 55957 | 55560 chain | 38:48 | 0.0 to 0.4 | 301 MB | (repo exec under 13568) | n/a | active build | keep with 13568 |
| 68450 | 1 (orphan) | 02:15:48 | 0.1 | 10 MB | none (reads all `.log`/`.status`/`.exit` in `🗑️generated`, no file output) | n/a | orphaned observer | stop |
| 97970 | 97813 | 03:13 | 1.6 | 40 MB | none (dev MCP stdio server, `--catalog .../🗑️generated`) | n/a | live, owned by another Claude session (cwd /Users/ueli/Documents/semio) | keep (not ours to stop) |
| 97813 | 97812 | 03:19 | 17.5 | 164 MB | none | n/a | live Claude session | keep (not ours) |

Notes:
- The 26626 to 26630 group was started by `run-browser-io.ts` (`xlsx-host`, `host`, and the on-ready watchers). Watchers 26899 to 26904 take PIDs 26629 and 26630 as arguments; those hosts have no listening socket on 6272 or 6070, so the watchers can never reach ready.
- Unrelated rustc and bun processes under `/Users/ueli/Documents/semio/.cache/...` and `.../semio_s_artifact_stdio_ifc4x3_population` do not reference this ticket and were excluded.

## Project-graph leader

The leader is `host` (PID 26630, log `host.log`). The waiters quote it as `leader: taken (pass 2) N of 1253`, which is what `xlsx-host.log`, `office-browser-after-checkpoint.log` and `xlsx-managed-space.log` (`owners done (pass 2) 1343 of 1343`, older) show.

Recent history from `host.log`:
- Pass 2 starts at elapsed 10081 s with `taken (pass 2) 0 of 1253`.
- Last pass-2 value: `126 of 1253` at elapsed 10613 s.
- After that, the leader reports `reading the owners (round 1) ... 3109 of 3600` at elapsed 10694 s. It has stayed in that phase since about 7593 s and advances about 1 unit per second.
- Pass-2 rate, elapsed 10508 s to 10609 s: 80 to 126, about 0.46 per second. Over 10143 s to 10613 s, the average was about 0.26 per second.

The waiters still quote 126 because their last published view is the leader's last pass-2 checkpoint. The leader itself is no longer printing pass-2 lines. So "126 of 1253" is stale, and the leader is progressing in another phase.

Sampling (log is `host.log`, `leader: taken (pass 2)` / `taken (pass 2) N of 1253`):
- Sample 1: 00:14:32 wall clock, `taken (pass 2) 126 of 1253`, last line `reading the owners (round 1) ... 3109 of 3600` at 10694 s.
- Sample 2: 00:15:33 wall clock, `taken (pass 2) 129 of 1253` (last pass-2 line at elapsed 10724 s), owners round 1 at `3173 of 3600` (elapsed 10756 s), latest 3183 of 3600 at elapsed 10766 s.
- Pass-2 speed fell sharply: 3 units in about 110 s (about 0.03 per second), against about 0.46 per second earlier. Owners round 1 advances about 1 unit per second (3109 to 3173 in 62 s). Phases alternate, so the leader is still advancing, but pass 2 is nearly stalled.

## Recommendations

Stop:
- Orphaned on-ready watchers 26899, 26900, 26902, 26904. They wait on hosts that never became ready.
- Waiting `run-browser-io.ts` wrapper 26627 and its child 26629 (`xlsx-host`), and office-browser-after-checkpoint 27393 and 27395. Their only output is the wait line.
- Waiting xlsx-managed-space 25380 and 25382, and xlsx-managed-renderer-wasm 20408 and 20414. These wait for the leader and for host memory, and have waited four hours or more.
- Observer 68450.

Keep:
- Leader `host` 26630 and its wrapper 26628, which are progressing.
- Office-exact build 13563, 13568 and its child tree, which are compiling. Its count has been flat at 1320 for about two minutes, so re-check before stopping.
- 97970 and 97813, which belong to another live Claude session.
