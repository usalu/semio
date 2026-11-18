# 🛑️ Who cancelled the whole-repo graph read at 18:59 UTC

Read-only investigation on g1-server, 2026-11-15. Times are UTC unless marked CET (CET = UTC+1, the local clock of the machine). No file was changed except this one.

## Summary

- **Sender of the SIGINT (established from logs, high confidence):** the graph read's own stall watchdog in the build-agent sidecar process (pid 9381 inferred, the process that attached to the read at 18:34:05). It sends SIGINT to the detached read runner when no published wave arrived for 25 minutes. Nobody else sent it.
- **Not the cause:** a Ctrl-C, a sidecar restart, the memory-headroom gate, a landing or its tests. The read runner runs in its own session (`detached: true`), so a restart or terminal signal does not reach it.
- **The read was not 4312 s old.** It started at 17:14:48 (pid 91761 gate) and ran 6259 s until it ended with status 130 at 18:59:07. The 4312 s figure matches only the eval child 8889, which started at 17:47:32.
- **The log tail quoted in the task is gone from disk.** A new read truncated `read.log` at 19:00:41. The surviving evidence is the sidecar log, the per-pid footprint traces and the landings file.
- **Why there was no wave:** the eval child 8889 spent its owner time in the parent's synchronous closure-settle work (`resolve-closure-refs` and inline compute) under host overload (load about 36, swap above ceiling). No generation was published between about 18:34 and 18:59, so the stall rule fired.

## Q1 Who sent SIGINT

Evidence in the sidecar log `.cache/f/sidecars/build-agent.log`:

- `build-agent.log:3678` (18:59:07.618): `graph-server: the graph read ended with status 130 after 6259 s: no wave was published for 25 min; the read is stopped and the next one starts from the waves published so far`
- `build-agent.log:3650` (18:34:05): `attached to the graph read (pid 91761) that has run for 4757 s` (so the read began 17:14:48).
- `build-agent.log:3647` (18:33:45): `left the graph read (pid 91761) running; the next process of the server attaches to it`.
- `build-agent.log:3660-3677`: periodic `made no progress for N s` diagnoses (421 s, 722 s, 1022 s, 1322 s), all from the diagnose hook, which does not signal.

The sender path, from the source tree (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, abbreviated `build/` below):

1. `build/🌐️remote-execution/🛰️sidecar/🕸️graph/🔭️read/🟦️.ts:72-73` `stop()`: `process.kill(record.runner,"SIGINT")`, then SIGKILL after 15 s to runner and read if still alive.
   - Called from the tick at `read/🟦️.ts` (the `verdict` line): `const verdict=watch.verdict(); if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}`.
   - `verdict()` at `build/🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts:34` returns `stalled` when `now-progressAt >= inferStallMilliseconds`, which is 1500000 ms (25 min, `build/🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json:402`). It returns `too-long` at 43200000 ms (line 403). Neither was near at 18:59 (6259 s run).
   - Also called on memory overrun (`read/🟦️.ts` sampling branch, `memoryOverrunV1` at `sidecar/🕸️graph/🔬️stall/🌳️tree/🟦️.ts:84`, factor 2 and 300000 ms silence, schema lines 415-416). Footprint in the traces stayed about 4.4 GB against grants of 10-14 GB, so this did not fire.
   - Also in the `finally` of `inferGraphReadV1` at `read/🟦️.ts:115` (SIGINT to a runner left running when no `directory` was given). Not the path here, since the sidecar passes a directory.
2. `sidecar/🕸️graph/🏃️runner/🟦️.ts:13` `stop()`: `child.kill("SIGINT")`, SIGKILL after 10 s (the "exits after 10 s" wording). Triggers: a SIGINT or SIGTERM received by the runner (line 14), or the orphan rule at the beat timer (watch file not touched for `readOrphanMilliseconds`, 1800000 ms). The orphan rule did not fire, because the attached server touched the watch file every tick.
3. `sidecar/🟦️.ts:84-89` launcher `stop()`: SIGTERM to its own child only. It is triggered by a SIGTERM/SIGINT to the launcher, lease expiry, or a code restart. It does not reach the detached runner, which sits in its own session (`read/🟦️.ts` `startGraphReadV1`, `detached:true`).
4. `sidecar/🚦️ready/🟦️.ts:293-296` `kill()` on stage timeout or a cancel file, and `sidecar/🚦️ready/🏃️wrapper/🟦️.ts:15`: these belong to ready-stage commands, not the graph read. Not the sender.
5. Other senders in the two folders, not traced to a trigger for this event: `build/🧱️job-limits/🟦️.ts:72-91` and `🐕️watchdog/🟦️.ts:57` (memory-limit kills, SIGKILL), `build/🏡️resident/🔄️reload/🟨️.mjs:45` (SIGTERM on code reload of the resident service), `build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs:183-188`, `build/📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts:56` (SIGKILL, a different read), `build/📏️incremental/🏎️bench/🧭️planning/🏃️runner/🟦️.ts:73,128`, `build/🗓️scheduler/⌨️command/🟦️.ts:36-42` (handlers that abort, they do not signal). The `build/🧠️memory/` gate sends no signal (its "refused with exit 75" path is only a message and a return value).
6. The text "SIGINT received ... leases released" and "Cancelled (SIGINT)" is not in the build module or the shared-worktree module. It is printed by a process outside these folders, most likely the repo CLI, which the child 8889/91761 runs on SIGINT. Not checked further (outside the allowed folders).

Other sidecar events near 18:59:
- `build-agent.log:3679` (18:59:07.647, 29 ms after the status line): `graph code changed, restarting (deferred 872 s, serve in flight is done; changed: build/🧠️memory/...footprint...)`. Inferred: the restart waited for the read to end (the 872 s deferral and the "serve in flight is done" condition both released at that point). The restart is a consequence of the cancel, not the cause. The changed files match the landing "R57" (`landings.md` section "Package R57 landing (2026-11-15 19:56)", which is 18:56 UTC, touching `build/🧠️memory/🦶️footprint/`).
- `landings.md`: no entry mentions SIGINT, SIGTERM, kill or cancel, so no landing script or test signalled the read. Landings near the time: R57 (19:56 CET = 18:56 UTC), R55 (19:00 CET), R53 wave2 (18:59 CET = 17:59 UTC, the closure walk farm, which changes child spawning but does not signal).

## Q2 Why owners ran for up to 2875 s

Eval child 8889 (`/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-footprint/8889.log`). Owner lines (`owner <name> wall=… farm=… inline=… observe=… refs=…`):

- `18:11:19` async-sync-rs wall 16.9 s, farm 16.3 s
- `18:12:42` semio-framework-pixels wall 99.6 s, farm 35.7 s, inline 61.2 s
- `18:13:56` repo-graph-maintenance wall 131.7 s, observe 106.6 s
- `18:15:00` repo-vite-storage wall 221 s, observe 158 s
- `18:15:58` ui-contract-operations-rs wall 296 s, observe 232 s
- `18:21:59` repo-test-runner-go wall 657 s, refs 360 s, observe 260 s
- `18:31:52` repo-schema-commands wall 1227 s, inline 513 s, refs 687 s
- `18:33:09` semio-plugin-reactor-staged-forms wall 1263 s, queue 238 s, observe 991 s
- `18:55:05` repo-policy wall 2643 s, inline 901 s, refs 1691 s (`slow resolve-closure-refs repo-policy 1691260 ms`, line 651)

Reading: most of the wall time was in the parent (inline compute, `resolve-closure-refs`, and `observe`), not in farm children. Farm times were small (16-70 s) against owner walls of 100-2600 s. The parent main thread was repeatedly blocked for 1.5-22 s (`starved sample step graph.plan.settle` / `graph.build.compute`, lines 14-17, 85, 254-314, 651+). The sampled stacks are synchronous closure probes: `record@🟨️.mjs:1085 < probe@🟨️.mjs:1101 < existsSync` and `getPackageJsonInfo@typescript.js`. The top frame `footprintPulseV1@build/🧠️memory/🦶️footprint/🟨️.mjs:391` is the sampler's call site, not the cost (inferred). The parent was busy, not idle. The read process 91761 was idle: its heartbeats show `phase=source.served`, rss about 45-60 MB, loop lag under 5 ms (`91761.log` tail).

Host load: the capacity log of the new read says `load 36.19`, `swap use is above its ceiling` (`read.log`, 19:00 CET). Sidecar log at 18:41-18:56: physical footprint 7.9-11.7 GB, 1-2 processes paging (`build-agent.log:3661, 3667, 3672, 3675`).

Note: the 2875 s and 2850 s owners (framework-os-kernel, repo) were still running at the stop, so they never finished before the stall.

## Q3 New read and current state

- New runner pid 80989, read pid 80992, `read-record.json` at `.cache/f/graph-server/` (`startedMs` 1794769241266 = 19:00:41 UTC, reason `request`).
- Gate: `80992.log:1` `gate installed pid=80992 grant=8572 MB` (19:01:02). `read.log`: `admitted show with 8572 of 10462 MB`.
- At 19:03:13 UTC: heartbeat `phase=source.served footprint=128 MB`, `read.log` last line `[repository:command] still running after 110 s`. Sidecar at 19:02:52: `no progress for 121 s`.
- Code: the server restarted at 18:59:08 with the changed `build/🧠️memory/` files, so the new read runs the R57 footprint code.
- Reuse of the cancelled read:
  - `read-record.json`, `read.log`, `read-watch` were recreated (`startGraphReadV1` removes the record, the runner opens the log with `"w"`). The old `read.log` is gone, so the quoted tail cannot be re-read from disk.
  - There is no `pending.json` in `.cache/f/graph-server/` (the expected file does not exist).
  - Persisted and possibly reused (inferred, not verified): `.cache/f/graph-costs/owner-costs.json` (written 18:55 UTC, the repo-policy sample) and `priority.json` (19:00:56). Waves published before 18:34 (generation 239 at 16:55 UTC per `wave-upload.json`) are the "waves published so far" the stall message says the next read starts from.
  - `serve-state.json` (saved 19:01:50 UTC) holds the held request.

## Evidence files

- Sidecar log: `/Users/ueli/Documents/semio/.cache/f/sidecars/build-agent.log` (lines 3647-3680)
- Read files: `/Users/ueli/Documents/semio/.cache/f/graph-server/` (`read-record.json`, `read.log`, `serve-state.json`, `priority.json`)
- Traces: `/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-footprint/8889.log`, `91761.log`, `80992.log`
- Policy: `build/🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json:402-422`

## What would be needed next time

- The read's stall stop should write a line to the read log before it signals (for example `stopping: no wave for N s, SIGINT to runner <pid>`), and the runner should append `SIGINT from <sender>` to its log. Signals cannot report the sender, so the sender must log at the send site (`read/🟦️.ts` `stop()`).
- Keep the cancelled read's log: rename `read.log` to `read.<startedMs>.log` instead of truncating it.
- Count waves as progress only where they are published; the 25 min window here spanned long parent-only phases (`resolve-closure-refs` of 1691 s), which the stall rule treats as no progress.
