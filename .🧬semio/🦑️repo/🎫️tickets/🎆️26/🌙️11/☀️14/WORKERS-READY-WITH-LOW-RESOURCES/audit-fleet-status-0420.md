# 🔍️ Audit: Fleet Status at 2026-11-18 04:18 UTC (read-only)

Snapshot: 2026-11-18 04:18 UTC (05:18 CET). Read-only: no source edit, no build, no test, no git write, no network, no sub-agent. Evidence: `/Users/Shared/semio-store/v1/origins/<machine>/`. Times below are UTC unless marked CET (UTC+1). Ages are relative to 04:18 UTC.

Comparison: `audit-linux-windows-and-silent-machines.md` (snapshot 21:01 CET 11-17) and `result-R146.md`, `result-R150.md`, `result-R152.md`.

Limits: the store keeps only the last 16 KB tail of each log, so older error text is not recoverable from it. `ready-timings.json` has outcome and times, but no messages.

## 1. Overview of all origins

| Origin | Platform | `ready` | Stage (index) | Attempts | Newest `ready.json` / newest file | Peak memory | Last reason (short) | Ready time |
|---|---|---|---|---|---|---|---|---|
| g1-m01 | macOS | false | `workspace` (5/5) | 68 | 0 min / 0 min (attest.json 04:18:08) | 9392 MB | `code-stale`: stage ran 30 min while its code changed, restarted at once. Tail: slice read 262 s, "95 of 1800" | none |
| g1-m02 | macOS | false | `workspace` (5/5) | 48 | 1 min / 1 min | 2825 MB | `previous-failed`: retry after back-off. Tail: slice read 33086 s, "taken 1068 of 4455" | none |
| g1-m03 | macOS | **true** | none (done) | 0 (last ok run was try 36) | 1 min / 0 min | 10473 MB | `ready` | 03:58:53 |
| g1-m04 | Linux (Steam Deck) | false | `tools` (4/5) | 38 | 1811 min (30 h) / 1811 min (pool.json 22:07 on 11-16) | 1866 MB | `adopted`: stage was already running, not restarted. Last write 22:07 on 11-16 | none |
| g1-m05 | Windows | false | `tools` (4/5) | 13 (timings say 14) | 0 min / 0 min | n/a (pool rss 723 MB; slice footprint 98 MB) | `code-changed`: tools waited over 10 min for a named holder, restarted | none |
| g1-m06 | macOS | false | `workspace` (5/5) | 2 | 2556 min (42 h) / 2556 min (pool.json.73695.tmp 09:41 on 11-16) | 7539 MB | `stalled`: no new output for 10 min, exit 75. Last write 09:41 on 11-16 | none |
| g1-m07 | macOS | **true** | none (done) | 0 (last ok run was try 91) | 2 min / 0 min | 10662 MB | `ready` | 04:06:22 |
| g1-m08 | macOS | **true** | none (done) | 0 (last ok run was try 65) | 2 min / 0 min | 10208 MB | `ready` | 04:04:35 |
| g1-m09 | unknown | no file | not started | n/a | no `ready.json`, no `pool.json`; only `fleet-auth.json` at 01:13 on 11-14 (4 d 3 h) / same | n/a | not enrolled, no sign of life | none |
| g1-m10 | macOS | false | `workspace` (5/5), state `backoff` | 51 | 2 min / 0 min | 5120 MB | `stage-failed` exit 1, code-stale. Next attempt 04:24:33. Error: `Original nested cache custody refused`, `Owned Nx invocation failed (1): nx run-many ... norm-din4108 ... build` | none |
| g1-server | macOS | no file | pool only | n/a | no `ready.json`; `attest.json` and `pool.json` 04:17 to 04:18 | n/a (pool rss 1461 MB) | pool `yielding` (see section 5) | none |

Summary: 3 of 11 origins are ready (m03, m07, m08). m01, m02, m05, m10 are alive and working. m04, m06 and m09 have had no writes for 30 h, 42 h and 4 days. No origin reports a new refusal code in this snapshot.

## 2. g1-m05 (Windows): stage, step, errors, rate, try time

**Stage and step now.** Stage `tools` (4 of 5), state `running`, reason `code-changed` (it waited more than 10 min for a named holder and was restarted). A new try started at 04:13:51 (pid 26912, `stage started` in `ready-log-tools-containment.txt`). The step is still the dependency store, not the chunk loop. Its tail is a heartbeat:

`Process 30880 (741d306890f3eaea75560cfde56699ec57f6b657c06db92f1e0d550e400c18ef show projects) holds the dependency store since 2026-11-18T03:28:01.557Z; waiting for it (346 s of at most 21600 s)`

The holder pid 30880 has held that store for about 50 minutes (since 03:28:01). Each try waits behind it, then the stall rule ends the try.

**Last three tries.** The timings file has the outcome and times. Message text exists only for the stalled try and the current try.

| Try | Window (UTC) | Length | Outcome | Decisive text |
|---|---|---|---|---|
| T1 | 03:52:02 to 04:02:23 | 621 s | stalled, exit null | not retained (rolled out of the store) |
| T2 | 04:03:24 to 04:13:45 | 621 s | stalled, exit null | `[repository:command] still running after 582 s`; `Process 30880 (... show projects) holds the dependency store since 2026-11-18T03:28:01.557Z; waiting for it (575 s of at most 21600 s)`; `[repo] still working after 590 s; last phase started: source.served` |
| T3 | 04:13:51 onward (current) | running, 4.5 min at 04:18 | running | `waiting for it (346 s of at most 21600 s)` (last line, about 04:17 UTC) |

No line in any m05 file contains an error text now. The `Immutable input physical work capacity exceeded` string from the earlier audit is absent from all current logs (`grep` over all m05 files). Nothing reads as a fresh error. The `tools` try ends with the stall rule, not an error.

**Did the error change since the earlier audit?** Yes. Earlier (13:11 UTC, 11-17), the last explicit error was `Internal error: Immutable input physical work capacity exceeded`. In the 28 tries after R150 (landed 20:55 UTC 11-17), the outcome is `stalled` every time (no `stage-failed`). The 12 tries before R150 include 3 `stage-failed` (exit 1). The current blocker is a wait on a holder, not a counter cap.

**Rate of progress and R150.** The current logs show no `native-artifact-input-stage:N` counter. The last counters I can find are 1836 to 1841 in `ready-log-tools-failed.txt` (11-17 20:14 UTC, before R150). R146 cites 3 to 4 s per step there. In the post-R150 logs, there are no chunk or step counters: only heartbeats every ~10 s (`still running after N s`) and the holder wait. So R150's larger steps are not observed on m05 yet. The try never reaches the input staging while pid 30880 holds the store. This cannot confirm or refute R150's effect.

**Time per try.** Post-R150: 28 tries, all `stalled`, median 647 s, min 611 s, max 2111 s. The 600 s no-progress rule ends each try, and it restarts at once. Pre-R150: 12 tries, median 1100 s, longest 13355 s (exit 1, `stage-failed`).

Side note: `lease store is 91 % full` appears in every m05 lease sweep. It has not been freed by the sweeps (`0.0 MiB freed`).

## 3. g1-m04 (Linux), g1-m06, g1-m09: sign of life since the earlier audit

- **g1-m04 (Linux):** no sign of life. Newest file is `pool.json` and `ready.json` at 22:06 to 22:07 UTC on 11-16 (30 h). The earlier audit already had this (23:07 CET). The `tools` stage still reads `adopted` at try 38. Its last explicit error (from the earlier audit) remains `Internal error: unable to open database file`.
- **g1-m06:** no sign of life. Newest file is `pool.json.73695.tmp` at 09:41 UTC on 11-16 (42 h). Unchanged since the earlier audit (10:41 CET).
- **g1-m09:** no sign of life. Only `fleet-auth.json` at 01:13 UTC on 11-14 (4 d 3 h). No `ready.json`, no `pool.json`. Unchanged; still not enrolled.

## 4. g1-m02: one old read, and whether the service restarted since 18:00 UTC 11-17

**The read.** `ready-log-workspace.txt` ends with `Reading the project graph slice (33086 s): taken 1068 of 4455.` (last line, written 04:14 UTC). Its elapsed clock runs continuously across the file window (178 counter lines, from 32487 s to 33098 s) with no reset. The same clock appears in `ready-log-workspace-stalled.txt` at 26836 to 26873 s with "taken 4403 of 4455". So the elapsed clock started about 19:00 UTC on 11-17. The progress count fell from 4403 to 1068 between the two windows. That is a reset of the inner count, not of the clock; the gap between the windows is not in the store.

Also present: `[repo] still working after 33712 s; last phase started: door.script`. Its clock is about 10 minutes ahead of the read counter.

**Restart since 18:00 UTC 11-17?** The files disagree, so the answer is "probably yes, for the supervisor and pool; no evidence for the read clock":

- Resident service failures after 18:00 UTC: `service-failure-17306fe5.txt` at 00:59:59 UTC (`SEMIO_LEASE_ORDER`, "Refusing to wait ... shared ... unavailable for 17 s while this process holds ... exclusively"), and `service-failure-b1c96dc7.txt` at 03:24:21 UTC (`Original Script process terminated`). `service-failure.txt` duplicates the 03:24:21 entry.
- `ready.json` `startedMs` = 04:08:13 UTC (pid 80439). `pool.json` `startedMs` = 04:08:28 UTC (pid 80426). So the ready loop and pool were started at 04:08 UTC, after the 03:24 failure (`previous-failed` retry).
- `refused.json` lists a daemon that "was not ready after 900 s" at 04:05:58 UTC.
- The slice-memory writer pid 72076 reports `uptimeSeconds` 21461 at 04:19 UTC, so that process started about 22:20 UTC on 11-17. It is a process restart after 18:00 UTC, but I cannot tell from the store whether it is the resident service.
- The read clock itself (about 19:00 UTC start) does not restart inside the windows I can read. The 04:08 supervisor start and the continuous read clock contradict each other. I did not resolve this.

The replica daemon is alive: `head.json` hlc 04:18:28 UTC, `attest.json` 04:18:13 UTC, segment 71.

## 5. g1-server: ready loop or pool only

Only a **pool**. There is no `ready.json` and no `ready-log-*` file. Its role is `puller` (`pool.json` at 04:17:53 UTC): state `pulling`, reason `yielding` ("0 of 10 cores idle, 10 booked by the machine's own work, 2 children run"). Claimed 2, published 0, stored 210, held 1, running 1, slots 3, children 2, target 3, RSS 1461 MB, waves 8.

## 6. Ready Macs: g1-m03, g1-m07, g1-m08 (last successful workspace run)

Timings are from `ready-timings.json` (the last `ok` run) and the nx summary in `ready-log-workspace.txt`.

| | g1-m03 | g1-m07 | g1-m08 |
|---|---|---|---|
| Last ok run (UTC) | 03:24:35 to 03:58:53 | 03:32:48 to 04:06:22 | 03:21:15 to 04:04:35 |
| Run length (try) | 2058 s (try 36) | 2014 s (try 91) | 2600 s (try 65) |
| nx run duration | 9m 22s | 22m 44s | 24m 43s |
| Cache hits | 0/7 (0%) | 0/7 (0%) | 0/7 (0%) |
| Critical path | 9m 14s (2 tasks) | 22m 37s (2 tasks) | 24m 34s (2 tasks) |
| Slowest task | `@semio-tech/norm-din4108-rs:build:prototype` 8m 42s | `@semio-tech/norm-din4108-rs:build:prototype` 22m 13s | `@semio-tech/norm-din4108-rs:build:prototype` 23m 36s |
| Slice read max, in this run's log | no `Reading the project graph slice` line (window 186 lines) | none (window 201 lines) | none (window 201 lines) |
| Slice read max, earlier tries | 864 s (stalled file); 11381 s (failed file, earlier) | none | 801 s (stalled file) |
| Phase lines in this run | `cache:entry · Preparing build tools` (8), `Verifying build tools` (11), `build:implementation-source · Reading the sources` (3); last phase `source.served` at 1490 s | `build:implementation-source · Reading the sources` (1); last phase `source.served` at 1792 s | `build:implementation-source · Reading the sources` (1); last phase `source.served` at 1310 s |
| Ready time | 03:58:53 | 04:06:22 | 04:04:35 |

Note: the successful runs take about 9 to 25 minutes of nx time, and every run has 0 cache hits. The "Recoverable time" is under 8 s, so cache reuse would matter more than task splitting here.

## Side note: g1-m01 and g1-m10

- g1-m01: 68 attempts, peak 9392 MB, `code-stale` restart at 04:17:58. Its last pool lines say "next child needs 1282 MB, only 915 MB free above the 2048 MB floor".
- g1-m10: backoff until 04:24:33. Last error chain: `Graph readers and namespace release failed`, then `Original process admission closed - first failure: Original nested cache custody refused`, then `Internal error: Owned Nx invocation failed (1): nx run-many --projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp --targets=build --parallel=3`.

## Files read (all read-only)

- `/Users/Shared/semio-store/v1/origins/g1-m05/` (`ready.json`, `ready-timings.json`, `ready-log-tools*.txt`, `slice-memory.json`, `pool.json`)
- `/Users/Shared/semio-store/v1/origins/g1-m02/` (`ready.json`, `pool.json`, `ready-log-workspace*.txt`, `service-failure*.txt`, `refused.json`, `slice-memory.json`, `attest.json`, `head.json`)
- `/Users/Shared/semio-store/v1/origins/g1-m03/`, `g1-m07/`, `g1-m08/` (`ready.json`, `ready-timings.json`, `ready-log-workspace*.txt`)
- `/Users/Shared/semio-store/v1/origins/g1-server/pool.json`, plus `ready.json` and `pool.json` of the other origins
- Ticket `audit-linux-windows-and-silent-machines.md`, `result-R146.md`, `result-R150.md`
