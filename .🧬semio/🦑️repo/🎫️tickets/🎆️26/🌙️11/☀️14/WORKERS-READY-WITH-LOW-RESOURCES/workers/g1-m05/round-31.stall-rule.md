# g1-m05 (Windows): The Stall Rule Stops a Working `dependencies` Stage

Written 2026-11-16T07:20Z by the agent session on the machine. Observation only (round 31): no setup command, no `repo` command, nothing under `.cache/f/ready` touched, no process stopped.

## Finding

The 10 minute stall rule of R74 stops the `dependencies` stage on this machine while its worker is busy. The stage has phases of more than 10 minutes in which the worker writes no line at all, only the two heartbeats (`[repository:command] still running after N s`, `[repo] still working after N s`) appear. The rule reads that as a stall, stops the tree and starts the stage again at once. Since 04:50Z every run ended that way, so the stage cannot finish here, whatever the cap is.

## Watched Live (Run of 06:30:50Z, Wrapper Pid 96100, Worker `bun.exe` Pid 88168)

- Last line that is not a heartbeat: `[DATA] Store dependency fences {"version":1,"purpose":"fence-budget","full":4,"skipped":3,"fullSeconds":558}` at about 2 170 s, which is 07:07:00Z. So the fence budget of R70 is live here and this run got further than the earlier ones.
- From then on the log has heartbeats only. I predicted the stop for 07:17:00Z from that line.
- `ready-timings.json`: `startedMs 1794810650672, endedMs 1794813427036, seconds 2776, outcome stalled` = stopped 07:17:07Z. `ready.json`: `reason.code stalled`. A new run started at once (wrapper pid 94060).
- Worker counters (Windows `Win32_Process`, read-only) in the silent 10 minutes:

| Window ending (UTC) | CPU s | Other file operations | Read operations | Read KB | Lines other than heartbeats |
|---|---|---|---|---|---|
| 07:08:09, 30 s | 23.7 | 1 574 716 | 784 | 4 | |
| 07:09:09, 30 s | 25.6 | 1 867 689 | 0 | 0 | |
| 07:11:05, 60 s | 46.6 | 3 326 918 | 1 844 | 9 | 0 |
| 07:12:06, 60 s | 52.0 | 3 477 151 | 1 690 | 9 | 0 |
| 07:13:06, 60 s | 44.5 | 2 981 031 | 1 240 | 5 | 0 |
| 07:14:06, 60 s | 44.5 | 3 215 390 | 1 698 | 9 | 0 |
| 07:15:06, 60 s | 48.3 | 3 184 378 | 1 690 | 10 | 0 |
| 07:16:07, 60 s | 48.5 | 3 491 236 | 1 820 | 9 | 0 |

The worker used about 0.8 of one core and did about 55 000 file operations per second without reading content, up to the minute it was stopped.

## The Earlier Stops Match

`ready-timings.json` rows with `outcome: stalled`, and the nearest 10 s sample of the worker from `round-31.md` inside the last 10 minutes before each stop:

| Run started | Stopped | Seconds | Sample inside the last 10 minutes (other operations in 10 s) |
|---|---|---|---|
| 04:26:12Z | 04:50:44Z | 1 472 | 04:42:34Z: 944 411 |
| 04:50:44Z | 05:37:20Z | 2 795 (adopted at about 05:12Z) | 05:30:58Z: 1 070 490 |
| 05:37:20Z | 05:59:22Z | 1 322 | 05:50:20Z: 1 100 370 |
| 05:59:22Z | 06:30:49Z | 1 886 | 06:29:05Z: 438 421 |
| 06:30:50Z | 07:17:07Z | 2 776 | table above |

The mirror `ready-log-dependencies-stalled.txt` of the 06:30:49Z stop holds heartbeats only, the same picture.

## What It Means for the Fix (My Reading, Not Measured)

- The silent phase is the one with about 1 000 000 "other" operations per 10 s and no reads. In the logs of earlier tries the same counters belong to `dependencies:store-ownership:frontier:N`. That phase printed a line per step batch before; since the batching of R70 (`frontierBatchPaths = 64`) or the fence budget it seems to print nothing for more than 10 minutes. I did not read the code to confirm which change removed the lines.
- Either the phase reports a moving counter at least every few minutes (the rule already treats a moving counter as progress), or the rule needs a second signal on a silent stage. On Windows the process tree's operation counters are such a signal: they are in `Win32_Process`, cost one query, and separate this case (55 000 operations per second) from a sleeping tree (0).
- R74 lists "the stall rule on a worker" and "the Linux/Windows runner of the stall tree stop" as not proven. The tree stop works on Windows: checked after the 07:17:07Z stop, the wrapper (96100), the door (59764) and the worker (88168) were all gone.

## Correction to R70 (Measured Here)

R70 says `store-input-replay` "re-opens, reads and hashes every staged input file". During that phase (06:59:14Z to 06:59:24Z, log lines `dependencies:store-input-replay … elapsedMs 0`) the worker did 35 701 other operations, 202 read operations and read 25 KB in 10 s, on 0.7 of a core. So that loop is bound by opens and metadata calls too, not by reading or hashing content. Defender used 0 s of CPU in every sample of the six hours.

## Smaller Things Seen in the Same Log

- `[DATA] Store dependency fleet … "stage":"skip","detail":"non-relocatable link node_modules/@semio-tech/2d-compute"` for six scopes, then `current-dependency-fleet-seed {"published":0,"present":0,"failed":6}`. This machine takes nothing from the fleet seed, so it installs everything itself.
- Ownership walk rate in this run: 3 960 steps per second at 90 s, 708 per second at 683 s, 576 per second at 1 174 s (`readText`).
- `health.json` counter `blocked` keeps growing: 11 148 at 00:45Z, 18 666 at 07:17Z (about 19 per minute).

## State

- `ready: false`, stage `dependencies` (3 of 5), `attempts` 10, 5 stalled runs since 04:26Z, a sixth run started 07:17:07Z.
- The observer `round-31-observe.ps1` was restarted at 07:00Z for six more hours and keeps appending to `round-31.md`.
- Not done: `repo preflight --lang en` (the round asks for it only at `ready: true`), replica `status`, any elevated command.
