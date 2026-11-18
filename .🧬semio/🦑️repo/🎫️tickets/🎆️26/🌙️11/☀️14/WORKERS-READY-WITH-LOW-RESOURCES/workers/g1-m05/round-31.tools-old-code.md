# g1-m05 (Windows): The Running `tools` Try Still Has the Code From Before R99

Written 2026-11-17T01:30Z by the agent session on the machine. Observation only (round 31): no setup command, no `repo` command, nothing stopped. It answers the open line of `result-R99.md` ("not yet observed: whether g1-m04 / g1-m05 finish `tools` with the new code").

## State (UTC)

- `ready: false`, stage `tools` (4 of 5), `attempts` 9, reason `adopted`. `workspace` has not started.
- The running try started 2026-11-16T22:31:45Z (`running.json` `startedMs` 1794868305843, wrapper pid 95544, worker `bun.exe` pid 68316). That is 28 minutes before the R99 landing of 23:00Z, so it runs the old read code. It was at 10 288 s at 01:23Z, past the soft cap because its counter grows. With `hardCapMultiplier` 4 it is cut at 02:31:45Z if it does not finish first.
- Rows of `ready-timings.json` since my last report: `stalled` 3 076 s, `stage-timeout` 4 555 s, `stalled` 635 s, `stage-timeout` 4 567 s (ended 21:46:39Z), then the running try.

## Measurement of the Running Try

- Counters in `tools.log`: `native-artifact-input-verification` reached 1 605, then `native-artifact-input-stage` started from 1 and was at 1 857 at 01:23Z.
- 60 s sample ending 01:24:29Z (`Win32_Process`, read-only):

| Process | Resident | CPU s | Read operations | Other operations | Read KB | Written KB |
|---|---|---|---|---|---|---|
| `bun.exe` 68316 (worker) | 1 082 MB | 35.9 | 59 772 | 2 374 848 | 5 580 | 11 |
| `bun.exe` 29736 (door) | 57 MB | 0.3 | 5 466 | 1 086 | 36 | 4 |
| wrapper 95544, `repo.exe` 50392 | 46 MB, 10 MB | 0 | 0 | 0 | 0 | 0 |

- In the same 60 s the counter went from 1 857 to 1 860: one step in 20 s, at about 790 000 other file operations and 1.9 MB read per step.

Reading (inference): this fits section 2 of R99, the old code walks to each file from the root and hashes all roots again per step. The counter (R85 p) works here: no `stalled` row since the try started. At three steps per minute the try gets about 200 steps further before the hard cap; I do not know the number of steps it needs, so I cannot say whether it ends first.

## What Follows for the Coordinator

- The first try on Windows with the R99 code is the next one, expected from about 02:32Z unless this one finishes. Its first counter rate is the number R99 section 6 misses ("Windows speed of the new reads is not measured"). The observer writes the log tail every 10 minutes into `round-31.md`; I will put the rate into a note here once that try has run for some minutes.
- Replica: `health.json` is written again regularly (last 01:12:34Z), `store ok`, head moved 01:12Z. `build-agent` has 12 restarts (last exit 143), failure kinds `blocked` 455, `round` 2 (last 22:45:48Z). Not stuck since the restart of 19:20Z.

## Not Done

- `repo preflight --lang en` (only at `ready: true`), any setup or elevated command.
