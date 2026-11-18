# g1-m01 Round 30

Rounds 28, 29 and 30 were posted after the round 27 runner ended (try 12, 11:26:59Z, `native_rc=75`) and the agent session that hosted it had ended; they were first read at 17:36Z by a new session. No separate round 28 or 29 run exists for this machine; no round 27 report file was written (its per-try files `round-27.*` are in this folder).

## Before The Run (2026-11-15T17:45Z)

- Machine: Mac mini, 16 GB (`hw.memsize` 17179869184), macOS Darwin 27.2.0 arm64, 60 GiB free of 228 GiB, memory pressure level 1.
- Replica `status` (exit 0): synchronization on, daemon running (pid 63024), 298924 files tracked, 0 incoming queued, 0 events behind every origin except g1-m02 (2), g1-m04 (1), g1-m07 (1). Code `203e01b3594f` (health file: `203e01b3594f1a510964bfe95d4031e2f7925aba3bdb457016bd480173d76efd`), the same as g1-server. Verdict of g1-m01: `ok`; `The fleet is not in agreement (10 origins; …)` (g1-server and five workers `lagging (not settled …)`, g1-m06 `stale`). The first try was not held back for it, as in round 17.
- Sidecar `build-agent`: running (process 40526, 0 restarts). `build-scheduler` and `build-cache`: off on this machine.
- Git state: `shared`, stage tree `bb58893207bab4930c7772b9e165f8a862e67a27`.
- Marker `.cache/f/native-bootstrap.done`: absent.
- Step 0 (`repo tools setup all`): ended with exit 0 in round 17 (1895 s); not run again.
- Other work on this machine at the start: a `volumes-check.ts` process of another session (563 MB resident), not a graph command.

## Runner

`run-round-30.sh` (this folder) = the round 27 runner with: tries 5 minutes apart, at most 40 (rounds 28 and 29 set no limit; 40 bounds an unattended run), and per try in `round-30.steps.txt` every `cut smaller` line, the highest `step k of 6`, the largest `waiting for the fleet: x of N`, the slice worker's peak resident size (5 s samples) and the last lines of the bootstrap log. Hard stop unchanged: a process of the run above 8 GB resident.
