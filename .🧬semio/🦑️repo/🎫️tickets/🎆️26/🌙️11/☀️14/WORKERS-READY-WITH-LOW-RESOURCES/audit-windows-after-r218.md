# Audit: g1-m05 (Windows, 64 GB) after R218

Read-only audit. Evidence: `/Users/Shared/semio-store/v1/origins/g1-m05/`. All times UTC (the folder's local clock is UTC+1). Evidence is current to the last write at 22:47:50 UTC (`ready.json`). The audit ran at about 22:53 UTC.

## 1. Every tools attempt since 20:30 UTC

Source: `ready-timings.json` (completed runs), `ready-log-*.txt` (logs), `ready.json` (current).

| # | Start (UTC) | End (UTC) | Duration | Outcome | Notes |
|---|---|---|---|---|---|
| A | 20:07:26 | 21:08:14 | 3649 s | exit 1, `stage-timeout`, attempts 21 | Started before the fix and ended after it. Its containment dump is at 20:54:25. No per-run log survives. |
| B | 21:29:45 | 22:03:29 | 2025 s | exit 1, `stage-failed`, attempts 22 | Started by supervisor 34460 and adopted by supervisor 9372 at 21:58:33. This is the "previous attempt on old code". Log: `ready-log-tools-failed.txt`. |
| C | after the back-off following B, about 22:25 (derived) | still running at 22:47:50 | at least 22 min | `state: running`, `ready: false` | Supervisor pid 9372, reason `previous-failed`. Log: `ready-log-tools.txt`. Its start is not in `ready-timings.json` because the run has not ended. |

Decisive lines:

- A: `ready-log-tools-containment.txt` header `2026-11-18T20:54:25.541Z tools-containment` (written during A).
- B: `Internal error: Closed pinned dependency recipe required (reference ed58d12c)` and then `repository:command failed with exit code 1`. The stage is `failed`, not `refused`.
- C: last log line `[repository:command] still running after 1359 s`. `ready.json` tail: `[repository:command] still running after 1369 s`.

Before 20:30 UTC, for context only: a run stalled from 19:33:28 to 20:06:14 (`ready-log-tools-stalled.txt`, "the stage process 32336 is gone ... SIGKILL ..."). That run is pre-fix.

## 2. Are the new behaviours present?

- **Store not waited for, shared holders: yes, in B and C.**
  - B (21:29 to 22:03): `The dependency store is verified current without exclusive access; 1 other process(es) hold it shared (first: process 32184 (...script.ts component --manifest ...) since 2026-11-18T21:08:13.763Z); nobody is waited for`
  - C (current): `The dependency store is verified current without exclusive access; 1 other process(es) hold it shared (first: process 22108 (...script.ts component --manifest ...) since 2026-11-18T22:24:04.486Z); nobody is waited for`
- **Bounded maintenance attempt: yes, once, in B.**
  - `[repo leases] The free space of the lease store could not be read; entries idle for 60 s are retired now.`
  - `[repo leases] Lease store swept: 17 idle databases, 13 empty queues, 5 settled claims and 0 abandoned nested stores retired (0.2 MiB freed, ? % free).`
  - No sweep line appears in C.
- **Typed lease refusal naming a holder: not seen.**
  - `ready.json` has `"refusal": null` and `"containment": null`.
  - No `refus` line appears in any tools log.
  - The 20:54 containment dump is a typed registry listing with pid, state, start, mode and command hash. It is not a refusal. Example line: `24944 stage started 2026-11-18T20:09:09.982Z directory C:\semio\.cache\b\c command 66f36ff... tools`.
  - No lease timeout has occurred under the new code yet, so this cannot be confirmed either way.
- **Memory admission (not part of R218, but it delayed B):** `[capacity] waiting 111s for command: memory pressure is raised (need 2270 MB; ... behind the host itself (nothing of ours)`. The win32 memory counters cannot be read (`powershell ... Win32_Process ...` failed), so the host is treated as under raised pressure.

## 3. Current state and what it waits for

- `ready.json` at 22:47:50: `state: running`, `stage: tools`, `stageIndex 4 of 5`, `attempts 22`, `exit null`, `ready: false`, `reason: previous-failed` ("retrying after the back-off").
- The current log shows `[repository:command] still running` for at least 1369 s. The command is `build:implementation-source` (owner `repository:build`), and `[repo] still working ... last phase started: source.served` has not changed since the start of the log. The log contains no `waiting` line for C, so the log does not show this command waiting on a lease.
- The only holder the log names is shared-store holder pid 22108. Its command is `C:\semio\...\🔌️plugin\🏗️build\🧩️driver-stages\📜️script.ts component --manifest 🌎️hub/🧩️compositions/🏗️fem/...`, and it has held the store since 22:24:04.486 UTC (about 23.8 min old at 22:47:50). The log says nobody is waited for on it.
- Is it a real developer command? The evidence does not establish who started it. It is a `driver-stages` build script, not an interactive shell or editor. B had a holder with the same command since 21:08:13, so this looks like a recurring automated build job rather than a one-off person. The logs cannot prove either way.

## 4. Has `ready` become true?

No. `ready: false` in `ready.json`. No `readyMs` exists. `ready-timings.json` only records `tools` runs (A and B above). No stage timings for a ready run exist.

## 5. What from the evidence still stands between this machine and `ready: true`

1. Run C has not finished. The `build:implementation-source` step has been in phase `source.served` for at least 22 min with no progress lines. The evidence does not say why. Run B failed at a later point with `Closed pinned dependency recipe required (reference ed58d12c)`. Nothing in the evidence shows whether C will pass that point.
2. The `Closed pinned dependency recipe` failure in B is unexplained by the logs. Whether it is fixed by C's progress is unknown.
3. Memory admission is blind on win32 (`the win32 memory counters could not be read (The operation was aborted)`). Admission uses declared sizes and treats the host as under pressure. This delayed B by about 111 s or more.
4. Fleet and replica state is degraded:
   - `pool.json` (updated 22:46:59): `state: pulling`, `lastError: owner pool: the fleet cache did not answer`, `failures: no-answer 2, breaker-open 2`, `children 8 of 11`.
   - `attest.json` (22:38): `view.stale: true`, `failures.blocked` count 173904 (last 22:37:10), `stage-objects-missing` last at 22:38:03, `queue.incoming 1918`.
5. Evidence gaps: the folder has no `ready-log-tools-*.previous` file and no `service-failure*.txt` file, as the brief expected. Those checks could not be made.

## Current state (one line)

At 22:47:50 UTC the tools stage is running on attempt 22, has not finished after at least 22 min in `build:implementation-source`, and has not reached `ready: true`.
