# Wave 1125 Audit A8 (read-only)

Snapshot taken 2026-11-26 02:31 CET. Sources: `/Users/Shared/semio-store/v1/origins/<origin>/` (ready.json, ready-timings.json, ready-log-*.txt, ready-log-*-failed.txt). Compared against `audit-A7.md`, the wave-1125 `HANDOVER-1125.md` (landed list F1 to F52), and `landings.md` (F40 notes). No edits, no processes touched.

## 1. NOT Ready workers

Ready now: g1-m01, m03, m06, m07, m08, m10, m11, m12, m13, m14 (10 of 15). g1-server has no ready.json and was not in scope.

### g1-m02 (macOS, stage workspace 5/5)
- State: `backoff`, attempt 11, exit 1. Retry scheduled ~02:36 CET (nextAttemptMs about 5 min away). The current attempt record started 02:29:52 (1.5 min ago). Process 51975 has been in the workspace stage since 18:44 (adopted by supervisor 21006 at 00:59). Peak 5492 MB.
- Last failure: workspace-failed at 01:16:31 CET.
- Exact first error line: `The resident service met an unexpected failure (reference 1c7c6ae4): Error [BUILD_SOURCE_CONFLICT]: Conflicting inference observations: global:["closure-graph","/Users/ueli/Documents/semio",...]` (raised in `observeInferenceComputationV1`).
- No `[nx] failed task` block.
- Classification: NEW. No listed fixed class names `BUILD_SOURCE_CONFLICT`. Closest is F23/F27 (closure-graph stale facts and tombstones), but the conflict is a different error path in the observation layer. Not verified against the new code.
- Older history (not current): dependencies-failed 11-21 ("Cannot find package '@semio-tech/schema-validator-ts'"), tools-failed 11-22 ("Current complete Source owner ready identity required"). Both predate the wave and are not the current blocker.

### g1-m04 (Linux, stage dependencies 3/5)
- State: `waiting`, attempt 22, refusal `disk-collecting`. It needs 428,796 inodes free in /home/deck/semio/.cache, has 38,724, and is missing 390,072. Collectors are running (last tail: "Starting collector compile-caches").
- Timing: the attempt started 312 min ago (about 21:19 CET). ready.json was last updated 304 min ago, while the beat is 120 s. The status file is stale, so the worker may not be beating. Check the process on the host.
- Last failure: dependencies-failed at 12:53 CET on 11-25 (attempt 22). First error line: `error: failed applying patch file: EACCES: Permission denied (open())`, then `error: failed to apply patchfile (patches/1.patch)`, then `Internal error: Fixed Store dependency installer failed (reference 8ef44f48)`. No `[nx] failed task` block.
- Classification: the current refusal is the known inode shortage (user action per HANDOVER: free inodes on the host). The EACCES patch failure is NEW. It is near F3 (bun-cache reflink EACCES), but F3 was about the bun cache, and this is `open()` during patch apply in the dependency installer. Not verified. It is also unknown whether the EACCES recurs after the inode problem clears, because the stage has not run since 12:53.

### g1-m05 (Windows, stage dependencies 3/5)
- State: `running`, attempt 49, adopted pid 31964 (adopted at 18:16 on 11-25). Attempt started 227 min ago (about 22:24 CET). Tail: "still running after 11518 s". Peak 7398 MB.
- Timing: ready.json was last updated 181 min ago, with a 30 s beat. This is stale; the supervisor may be stuck, and the "running" state cannot be trusted until the file updates.
- Last failure: dependencies-failed at 19:16:58 CET on 11-25 (attempt 48). First error line: `process 24028 stands in the process table with a start time 17 s away from the recorded one (... tolerance 15 s ...) ... so it is judged another process that reused the pid and the stage is ended`.
- Classification: matches the pid-identity class already fixed in wave 1125 (F41/F42 pid+start identity, F45 identity tolerance). The failure predates those landings (F45 was after 22:00 CET on 11-25), so it is not evidence against the fix. Not verified on the new code, because attempt 49 has not failed yet. Re-check on the next failure.

### g1-m09 (no runtime state)
- No ready.json, no logs, no timings. The folder holds only `fleet-auth.json` (dated 11-14). Not provisioned. Replaced by g1-m15. Per HANDOVER, the user must run `prompt-g1-m09.md` on that Mac. Not a runtime failure; nothing to classify.

### g1-m15 (macOS, new Mac, stage workspace 5/5)
- State: `running`, attempt 2, adopted pid 94791. Current record started 02:29:57 (about 1 min ago). The workspace process 13661 has been running since 23:42 on 11-25. Last tail: "Reading the project graph slice (907 s) ... taken 106 of 612". Peak 3342 MB.
- Last failure: workspace-failed at 01:14:05 CET. First error line: `The resident service met an unexpected failure (reference 1c7c6ae4): Error [BUILD_SOURCE_CONFLICT]: Conflicting inference observations: global:["closure-graph","/Users/ueli/Documents/semio",...]`. Same reference id and same message as g1-m02. No `[nx] failed task` block.
- Earlier, native-failed at 21:31 on 11-25: rustup toolchain provisioning failed (`could not remove 'component' file` under `~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin`, then `component download failed for rust-std-wasm32-unknown-unknown: could not rename ... .partial`). Classification: NEW (fresh-machine rustup provisioning race or permissions). The native stage has since passed, so it is not a current blocker. Worth a look if it recurs on another new Mac.
- Classification of the current failure: NEW, same as g1-m02 (BUILD_SOURCE_CONFLICT). Both machines hit the same closure-graph observation conflict with the same reference id, so it looks deterministic across the fleet, not a per-host fault.

### Summary of NEW classes
1. `BUILD_SOURCE_CONFLICT` (closure-graph inference observation conflict, reference 1c7c6ae4): g1-m02 (workspace, attempt 11, backoff), g1-m15 (workspace, attempt 2). NEW.
2. dependency patch apply `EACCES: Permission denied (open())` in the installer: g1-m04 (11-25 12:53). NEW, possibly related to F3.
3. rustup toolchain component remove/rename failure on first provisioning: g1-m15 (native, 11-25 21:31). NEW, not current.
4. Stale ready.json for a "running" or "waiting" worker: g1-m04 (304 min) and g1-m05 (181 min). Status-writing failure, not a classified error. Worth a fleet-status check.

Not NEW: g1-m05 pid identity (F41/F42/F45, predates fix).

## 2. [units] miss lines (F40)

- Result: zero `[units] miss` lines in every origin's ready-log files. A search for `nearest:`, `differs in`, and `miss ... nearest` across all origin files also returned nothing.
- The only `[units]` lines present are per-run summaries (for example `86 restored local, 241 restored fleet, 709 compiled`). These do not carry the F40 miss diagnostic.
- So I cannot name the divergent key component or the top crates from the logs. The likely reasons: (a) the ready logs keep only the newest run per stage, and the F40 diagnostic prints only while a unit compiles on a fleet miss; (b) the F40 round-2 code (`📜️script.ts`, `🛰️fleet/🟦️.ts`, `keyComponentsOfV1`) may not have reached a worker build yet. Per landings, the round-2 change landed after the last worker start for most machines. Next step: after one full worker cycle on the new build, re-grep for `[units] miss`.
- Count by differing component: 0 lines, so 0 per component. Top crates: none.

## 3. Workers with `refreshing: true`

- None. All 14 ready.json files that exist show `"refreshing":false`. (g1-m09 and g1-server have no ready.json.)

## Notes
- Historical: g1-m02 earlier dependency failure of 11-21 cited `@semio-tech/schema-validator-ts` not found. The stage has since passed; not current.
- Ready-log retention: `ready-log-<stage>.txt` keeps only the latest run, so the earlier attempts' errors are not available. The attempt counts above come from ready.json.
