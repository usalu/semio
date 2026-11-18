# g1-m05 (Windows): `tools` Now Fails After Three Hours With `Immutable input physical work capacity exceeded`; the Short Stalls Are Waits for `show projects`

Written 2026-11-17T13:45Z by the agent session on the machine. Observation only (round 31): no setup command, no `repo` command, nothing stopped. Follow-up of `round-31.tools-new-code.md`.

## Tries Since 07:35Z (UTC, `ready-timings.json`)

| Try start | Seconds | Outcome |
|---|---|---|
| 04:25:43 | 13 355 | `stage-failed` (ended 08:08:18, before the hard cap of 08:25:43) |
| 08:28:38, 08:46:33, 09:14:37, 09:32:59, 09:53:07 | 1 071, 1 684, 1 100, 1 208, 617 | `stalled` |
| 10:03:25 | 11 273 | `stage-failed` (ended 13:11:18), exit 1, `attempts` 12 |
| 13:31:24 | running | `previous-failed`, waiting (see below) |

`ready: false`, stage `tools` (4 of 5). `workspace` has not started.

## Finding 1: The Long Tries End in an Error, Not in the Cap

`ready-log-tools-failed.txt` (13:11:18Z) and the `failure.tail` of `.cache\f\ready\state.json`: after `native-artifact-input-stage:2584` and `still running after 11259 s` the stage prints

```
[DATA] Dependency sync incomplete {"completedBindingsChanged":0,"sourceTransitionMayBeUnresolved":false,"wholeAllocationRetired":false,"sourceDirectory":null,"preservationIntentSha256":null,"completedGenerationRetentions":6,"generationRetentionPhaseCompleted":true,"generationRetentions":[{"directory
Internal error: Immutable input physical work capacity exceeded
repository:command failed with exit code 1
```

- So the slow phase does end on this machine, at about 2 580 steps and 3.1 to 3.7 hours, and what follows is a refusal by a capacity rule. More time does not help: both long tries of today end as `stage-failed`. For the try of 04:25:43 I have only the outcome row, its tail was replaced by the later one; that it ended with the same text is my assumption from the same shape (13 355 s, counter 2 449 at 10 900 s).
- The try that fails pays the whole three hours again on the next start (R99 section 6: no receipt across tries), so the machine loops at about one failure per four hours.

## Finding 2: The Short `stalled` Tries Wait for `repo show projects`

`ready-log-tools-stalled.txt` (10:03:25Z): the last lines of the stopped try are `Process 70836 (… show projects) holds the dependency store since 2026-11-17T08:09:50.356Z; waiting for it (587 s of at most 21600 s)`. The same now: the try of 13:31:24 prints `Process 53380 (… show projects) holds the dependency store since 2026-11-17T13:14:04.232Z; waiting for it (197 s …)`.

- 53380 is alive: `bun.exe --eval`, started 13:09:56Z, child of `bun.exe` 146992, child of `repo.exe` 160232 (`repo show projects`, started 13:03:47Z), child of `bun.exe` 172904. Not started by this session. 20 s sample at 13:38Z: 19.2 s CPU, 41 191 other file operations, 2 106 read operations, 19 KB read, 616 MB resident. So it is a live, working holder (the second case of R99 section 3), a full core busy.
- The wait lines differ only in the number of seconds, which the stall rule normalises away, and the waiting tree uses no processor time, so after 600 s the rule stops the stage ("no output and no processor time for 600 second(s)", `kills.log` 10:03:24Z) and starts it again, where it waits again. Five such rows followed the failure of 08:08. Both times a `show projects` took the store shortly after a long `tools` try had failed and released it: 08:09:50 (failure 08:08:18) and 13:14:04 (failure 13:11:18; that `show projects` had been started at 13:03:47 and waited).

Reading (inference): the two commands take turns on the dependency store, each one slow for the same reason, and `tools` is stopped while it waits correctly. A wait on a named live holder with a fresh heartbeat is not a stall; the stall rule could exempt it (the holder's `silentMs` of R99 is the signal).

## Other

- Store folder `S:\v1\origins\g1-m05\`: 12 `ready-log-tools.txt.<pid>.tmp` of 11-16 and one `ready.json.88548.tmp` beside the `pool.json.<pid>.tmp` leftovers; `pool.json` itself is of 11-16 15:42Z.
- `ready-log-tools.txt`: "The lease store is 93 % full … 7 % free" on every sweep; and `[capacity] the memory footprint of running process trees cannot be read on win32 (Command failed: powershell.exe … Get-CimInstance Win32_PerfFormattedData_PerfProc_Process …)`.
- Replica: `health.json` 13:35Z, head moved 13:29Z. Not stuck.

## Not Done

- `repo preflight --lang en` (only at `ready: true`), any setup or elevated command.
