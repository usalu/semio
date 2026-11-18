# g1-m05 (Windows): First `tools` Try Started After the R99 Landing Is as Slow as the Old One

Written 2026-11-17T07:35Z by the agent session on the machine. Observation only (round 31): no setup command, no `repo` command, nothing stopped. Follow-up of `round-31.tools-old-code.md`; it gives the Windows number that `result-R99.md` section 6 misses.

## What Happened Since 01:30Z (UTC, `ready-timings.json`)

| Try start | Seconds | Outcome |
|---|---|---|
| 11-16 22:31:45 (old code) | 14 424 | `stage-timeout` (hard cap, as predicted for 02:31:45) |
| 02:52:10 | 625 | `stalled` |
| 03:02:36 | 1 523 | `stalled` |
| 03:28:26 | 860 | `stalled` |
| 03:42:48, 03:53:37, 04:04:16, 04:14:59 | 647, 638, 642, 643 | `stalled` |
| 04:25:43 | running, 10 900 s at 07:27 | adopted, `attempts` 10 |

- The seven `stalled` rows are stops for "no output and no processor time for 600 second(s)" (`kills.log`, last entry 04:25:40Z). I did not find out which phase was silent: `tools.log` holds no closing line of those tries.
- `ready: false`, stage `tools` (4 of 5). `workspace` has not started.

## The Running Try (Started 04:25:43Z, After the Landings of 23:00Z and 23:30Z)

Counter `native-artifact-input-verification` ran to 1 605, then `native-artifact-input-stage` continued the same count. Value at the heartbeat nearest to each quarter hour of the try:

| Try second | 920 | 1 822 | 2 722 | 3 625 | 4 528 | 5 428 | 6 328 | 7 228 | 8 128 | 9 028 | 9 928 | 10 900 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Counter | 1 619 | 1 658 | 1 779 | 1 832 | 1 864 | 1 928 | 2 014 | 2 119 | 2 220 | 2 313 | 2 380 | 2 449 |

60 s sample ending 07:27:21Z (`Win32_Process`, read-only): worker `bun.exe` 95744, 1 322 MB resident, 44.8 s CPU, 76 098 read operations, 3 088 829 other operations, 7 257 KB read, 2 KB written; counter 2 445 to 2 449.

Comparison with the old-code try (sample of 01:24Z in `round-31.tools-old-code.md`):

| | Old code (try 22:31:45) | Try 04:25:43 |
|---|---|---|
| Counter at about 10 250 s | 1 857 | about 2 400 |
| Steps per minute in the sample | 3 | 4 |
| Other file operations per step | about 790 000 | about 770 000 |
| KB read per step | about 1 860 | about 1 810 |

Reading (inference): on this machine the cost of one step did not change with R99. About 770 000 metadata calls and under 2 MB of content per step means the time is still spent in walks, not in hashing, so the saving of R99 (fewer hash passes) does not reach the part that is slow on NTFS. That the try runs the new code I conclude only from its start time; I cannot see the loaded code. The try is cut at 08:25:43Z (4 x 3 600 s) unless it finishes; at 4 to 7 steps per minute it gains 250 to 400 steps until then, and I do not know the number it needs.

## Other

- Replica: `health.json` 07:22Z, head moved 07:19Z, `build-agent` running since 06:46:38Z with 0 restarts (so the daemon itself started again about then); new failure kind `git-broken-config-unreadable` once at 06:46:04Z. Not stuck.
- `S:\v1\origins\g1-m05\` still holds many `pool.json.<pid>.tmp` leftovers (R99 section 3 mentions 21).

## Not Done

- `repo preflight --lang en` (only at `ready: true`), any setup or elevated command.
