# Audit: g1-m05 (Windows worker, 64 GB), stage `tools`, 2026-11-18

Read-only audit. No source file, build, test or `repo` command was run; no file was deleted or moved. Evidence is the store `/Users/Shared/semio-store/v1/origins/g1-m05/` plus the summary parts of `result-R146.md`, `result-R150.md`, `result-R183.md`, `result-R187.md`, `result-R189.md` in this ticket folder. Times are UTC unless marked local (local = UTC+1). The store files change while this runs; the state below is as read at about 19:36 UTC.

## Headline

- The machine is still not ready. `ready.json`: `state: running`, `stage: tools`, `stageIndex: 4`, `stageCount: 5`, `attempts: 20`, `ready: false`.
- Since 05:00 UTC there were 21 `tools` runs (run 20 overlaps 05:00). Outcomes: 10 `stalled`, 9 `stage-failed`, 1 `stage-timeout` (4 h), plus the current run.
- The current run is waiting for a holder: pid 12936, a `script.ts component` driver-stage process, which holds the dependency store since 2026-11-18T19:32:51.624Z.
- None of the new text markers asked about (`stage-holder-silent`, `disk-unmeasured`, a typed rename refusal, `step-overdue`) appears in any store file. Only the `waiting for it (N s of at most 21600 s)` format is visible.
- The rename error `status 0xc0000022` is not in the store any more. The newest error text still held in full is the `Resource lease timed out` line in `ready-log-tools-failed.txt` (18:24 UTC).

## 1. Timeline of `tools` runs since 05:00 UTC

Source: `ready-timings.json` (40 runs in total, the table starts at run 20). The digest field is `f0f2aa86e432` for every run, including runs before and after the landings, so it does not show which code ran. Attempt numbers are from the same file.

| Run | Start | End | Duration (s) | Outcome | Exit | Attempt |
|---|---|---|---|---|---|---|
| 20 | 04:54:31 | 05:12:35 | 1084 | stalled | - | 14 |
| 21 | 05:12:36 | 05:25:10 | 754 | stage-failed | 1 | 14 |
| 22 | 05:51:54 | 06:20:06 | 1692 | stage-failed | 1 | 15 |
| 23 | 06:23:42 | 10:23:45 | 14403 | stage-timeout | - | 15 |
| 24 | 10:43:47 | 12:03:32 | 4786 | stage-failed | 1 | 16 |
| 25 | 12:23:44 | 12:35:02 | 679 | stage-failed | 1 | 17 |
| 26 | 12:35:41 | 13:04:29 | 1728 | stalled | 1 | 17 |
| 27 | 13:06:52 | 13:43:06 | 2174 | stalled | - | 17 |
| 28 | 13:43:47 | 14:13:22 | 1775 | stalled | - | 17 |
| 29 | 14:13:54 | 14:25:29 | 695 | stalled | - | 17 |
| 30 | 14:27:44 | 14:42:25 | 881 | stalled | - | 17 |
| 31 | 14:42:56 | 15:13:17 | 1821 | stalled | 1 | 17 |
| 32 | 15:13:49 | 15:31:52 | 1083 | stalled | - | 17 |
| 33 | 15:33:01 | 16:04:52 | 1911 | stalled | 1 | 17 |
| 34 | 16:05:29 | 16:25:34 | 1204 | stalled | 1 | 17 |
| 35 | 16:26:10 | 16:49:28 | 1398 | stalled | - | 17 |
| 36 | 16:50:06 | 16:52:45 | 159 | stage-failed | 1 | 17 |
| 37 | 17:12:47 | 17:27:08 | 861 | stage-failed | 1 | 18 |
| 38 | 17:47:18 | 18:03:15 | 957 | stage-failed | 1 | 19 |
| 39 | 18:23:23 | 18:24:25 | 62 | stage-failed | 1 | 20 |
| 40 | 18:44:29 | 19:32:54 | 2905 | stalled | - | 21 |
| now | 19:34:42 (log header) | running | - | running | - | 21 (ready.json says 20) |

Decisive lines, only where the store still holds them:

- Run 21 (05:25, from `result-R187.md`, the log file has since been overwritten): "Windows relative preservation rename failed (status 0xc0000022; STATUS_ACCESS_DENIED; nt; root C:\semio\.cache\b\d\m" (the log was cut there).
- Run 36 onward: no retained text except the files below. The other runs' logs were overwritten; their decisive lines cannot be quoted from the store.
- Run 39 (`ready-log-tools-failed.txt`, 18:24:25): "Internal error: Resource lease timed out: native-storage:[...] (reference ec1e6ffc)" followed by "repository:command failed with exit code 1".
- Run 40 (`ready-log-tools-stalled.txt`, 19:32:54): "the stage process 892 is gone: it left no exit file and no signal was recorded ... it was last seen alive 0 s ago and this supervisor had sent a stop signal to its tree". `ready.json` reason: `stalled`, "Stage tools showed nothing new for 10 minutes and was stopped; started again at once."

Landings in the same window (from the R-reports): R183 at 05:00:00 to 05:01:58 UTC; R187 at 06:00:00 to 06:00:07 UTC, with run 22 adopted under the old code (per R187); R189 at 07:00:08 to 07:01:39 UTC. Run 23 (06:23 to 10:23, `stage-timeout`) is the first run that started after R187 and it ran into the 4 h hard cap (R183 names the cap as 4 times the soft cap). Its log is not in the store, so what it waited for is unknown. The stalls in runs 26 to 35 and 40 came after the R183 landing; the store does not show whether they were the holder-silent case R183 describes.

## 2. The current attempt

- Step: `build:source-admission` (done), `build:implementation-source` (done), then `repository:command` running, last phase `source.served`. The `tools` stage is index 4 of 5.
- Start: the log header is `2026-11-18T19:34:42.616Z tools`. `ready.json` has `startedMs` 19:22:46 UTC and pid 17604. The two disagree; the store cannot say which is the true start of this attempt. Holder lease entry `30844 stage` shows 19:33:45.554Z. The current log shows the wait since 19:32:51 (holder, see below).
- What it waits for: pid 12936, command `C:\semio\...\driver-stages\📜️script.ts component --manifest 🌎️hub/🧩️compositions/🏗️fem/🧩️compon` (cut in the log; the full command line is not in the store). It holds the dependency store since 2026-11-18T19:32:51.624Z. The wait line is `waiting for it (84 s of at most 21600 s)` in the last log line before this audit's latest read (19:36:05 UTC), seven such lines in total.
- Holder start: the containment log lists 12936 as `not-ours` started 2026-11-18T19:22:50Z, cwd `C:\semio\.cache\b\c`.
- New behaviour visible in the text: the `waiting for it (N s of at most …)` wait line with the 21600 s cap (R183). Not visible: `stage-holder-silent` (0 matches), `disk-unmeasured` (0), a typed rename refusal naming source, target and NT status (0 matches for `0xc0000022`, `STATUS_ACCESS`, `preservation`), `step-overdue` (0). Also not visible: the `preservation rename waiting` line that R187 describes.
- Last stall: the previous attempt ended with the generic reason `stalled`, not `stage-holder-silent`, so the R183 typed refusal did not appear in the store for the 19:32 stall.

## 3. Newest error text (full)

The newest error text in the store is in `ready-log-tools-failed.txt`, written 2026-11-18T18:24:25.480Z, and this is the full line:

`Internal error: Resource lease timed out: native-storage:["win32",4130025757,844424936912036,179,"{\"kind\":\"directory-partition\",\"anchor\":[4130025757,844424936912036,179],\"volume\":{\"dev\":\"4130025757\",\"ino\":\"9288674232317024\",\"mode\":16822}}"] (reference ec1e6ffc)`

The rename error quoted in the task (`status 0xc0000022`) is not in any file of the store. The only copy is the one R187 quotes from 05:25 UTC. The rename-error text R187 says the wrapped text was cut at 240 characters in the old log, so that copy is also incomplete.

## 4. Real developer command holding resources

- The current holder 12936 is a `script.ts component` driver-stage process under `🏗️build/🧩️driver-stages`. Its lease is `not-ours`. The containment summary reads `windows 32, person none`, so no person window is registered for it. The store cannot tell whether it is a human's command or a fleet build. The stored command is cut.
- It is not `run @semio-tech/fem-3d-rs:check`. That command appears only in `result-R183.md` (holder since 2026-11-18T03:55:14.747Z, per R183), not in the current store.
- Other `not-ours` entries in the containment log: 2692 (`dependencies`, since 2026-11-16 05:37), 28448 (`show`, since 2026-11-15 12:20), 9036 (`show`, since 2026-11-18 01:31), 7616 (`tools`, since 2026-11-17 04:15). Their current state (alive or not) is not stated per entry.

## 5. The pool

`pool.json` (19:34:43 UTC, pid 28772, role `puller`, state `pulling`):

- `claimed` 16, `published` 7, `stored` 9, `held` 0, `failed` 0, `running` 5, `slots` 12, `children` 8 of target 12.
- Grant: `grantMegabytes` 19634 of `totalMegabytes` 65447; `rssMegabytes` 3346; `childMegabytes` 768.
- Waves 8, `claimAttempts` 32, `ownersPerMinute` 1.4, `failures` empty, `lastError` null, `reason` `starting`, next child may start in 156 s.

So the pool is contributing claimed and published records. The store does not show which whole-workspace wave these belong to beyond its own wave id `1bca9b...`, nor whether the workspace-wide wave counts them.

## 6. What must happen for `tools` to pass, and what cannot be known

What the log text says must happen:

1. Holder 12936 must release the dependency store. The stage is designed to wait for it up to 21600 s (`waiting for it (N s of at most 21600 s)`); the log does not say when it will release.
2. The stage must then get past the dependency store without `Resource lease timed out` (the 18:24 error) recurring. The current log does not show one.
3. The rename of the old dependency generation must succeed or end in the typed refusal. Neither outcome appears in the store.

What cannot be known from the store:

- The contents of the logs of runs 22 to 38 and of run 40's full text; they were overwritten.
- Whether the new code (R183, R187, R189) actually runs: `digest` in `ready-timings.json` is the same for every run, and `ready.json.digest` (`ce0ca918...`) differs from it; neither proves a revision.
- The host log of m05 (R183 says it cannot be read); the reason for each stall and the `stage-timeout` cause.
- Whether 12936 is a human's command or fleet work.
- Whether the rename error (`0xc0000022`) recurred after 05:25 UTC.
- Whether `disk-unmeasured` or `step-overdue` ever fired, since no line of the store shows either.
- The store state after 19:36 UTC.

## Store file inventory

Mtimes in local time (UTC+1), as read. Sizes in bytes.

| File | Size | mtime (local) |
|---|---|---|
| attest.json | 1994 | Nov 18 20:32:04 |
| fleet-auth.json | 562 | Nov 18 13:58:45 |
| head.json | 90 | Nov 18 20:32:49 |
| home.json | 67 | Nov 14 04:22:12 |
| pool.json | 773 | Nov 18 20:34:43 |
| pool.json.*.tmp (67 files) | 566 to 2977 each | Nov 16 16:50 to Nov 18 08:03 |
| ready-log-dependencies-failed.txt | 1110 | Nov 16 05:06:12 |
| ready-log-dependencies-stalled.txt | 1111 | Nov 16 13:31:52 |
| ready-log-dependencies.txt | 16422 | Nov 16 14:35:23 |
| ready-log-native.txt | 1839 | Nov 15 19:58:40 |
| ready-log-repo-mcp.txt | 16415 | Nov 15 20:17:23 |
| ready-log-tools-containment.txt | 1330 | Nov 18 20:34:37 |
| ready-log-tools-failed.txt | 1228 | Nov 18 19:24:25 |
| ready-log-tools-stalled.txt | 1365 | Nov 18 20:32:54 |
| ready-log-tools.txt | 16266 at read, growing | Nov 18 20:34:42 |
| ready-log-tools.txt.*.tmp (11 files) | 628 to 16415 each | Nov 16 17:43 to Nov 16 21:39 |
| ready-timings.json | 7393 | Nov 18 20:32:54 |
| ready.json | 971 at read, changing | Nov 18 20:34:42 |
| ready.json.30716.tmp | 635 | Nov 18 09:46:57 |
| ready.json.88548.tmp | 3188 | Nov 16 16:51:51 |
| refused.json | 258 | Nov 14 11:23:05 |
| segments/ (dir) | 128 | Nov 18 12:51 |
| segments/000000000001.jsonl | 1449425 | Nov 18 12:51 |
| segments/000000000002.jsonl | 273243 at read, growing | Nov 18 20:32 |
| slice-memory.json | 3758 | Nov 18 20:34:42 |
| slice-memory.json.37188.tmp | 3758 | Nov 18 14:24:07 |
| view.txt | 79592 | Nov 18 20:32:03 |

Not present in the store: `ready-log-tools-*.previous`, any `service-failure*.txt`.

`view.txt` is not a worker view: its head is a git-status listing of the shared worktree (`D ✏️s/🔌️plugins/...`, `M .🧬semio/...`), so it does not describe `tools`.

`ready.json` at read: `state: running`, `pid 17604`, `reason.code: stalled` (describing the previous attempt), `attempts 20`, `startedMs 1795029766748` (19:22:46 UTC), `updatedMs 1795030561840` (19:36:01 UTC), `tail` now a lease-sweep line.

`head.json` / `attest.json`: segment 2 of the event log, cursor for g1-m05 at 270666 bytes in `attest.json`; `refused.json` holds one watchdog refusal (digest `77ac3a6c...`, 2026-11-14).
