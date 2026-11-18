# Worker g1-m08 (Mac mini 05): Setup and Collaboration With the Coordinator

All worker reports of the protocol live in the coordinator's ticket, folder `26/11/14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m08/` (the protocol allows a worker to write only there). This file is the summary of this ticket.

## State Found (2026-11-15T17:50Z)

- This machine is already joined: origin `g1-m08`, group `g1`, worktree `~/Documents/semio`, store `/Volumes/semio-store` reachable, replica daemon running, code `b4c6513b87ef` (same as g1-m02, g1-m04, g1-m07).
- It was silent since round 18 (04:13Z) because the machine restarted at 04:26Z; the previous agent session and its observer ended with it.
- The completion marker `.cache/f/native-bootstrap.done` was absent: the worker was not ready.
- The coordinator's automatic ready runner (sidecar role `ready`) was already working through its five stages on this machine (`native` and `repo-mcp` done, `dependencies` running).

## Decisions

- Followed `ROUND.md` round 30 (the machine's `next.md` still named round 27).
- Did not start `native-bootstrap.sh` beside the running automatic runner (same stages, one graph command at a time); observed the runner read-only instead and told the coordinator so in `round-30.pre.md`. A manual run follows only on a failed stage or a standstill.
- No build code changed, no git command that changes anything, no process killed.

## Files Written in the Coordinator's Ticket

- `workers/g1-m08/round-30.pre.md`: machine, replica status, what was found running.
- `workers/g1-m08/round-30-observe.sh`: the read-only observer.
- `workers/g1-m08/round-30.stages.txt`, `round-30.samples.txt`, `round-30.p12-auto.txt`: its output.

## Result

Not ready (state 2026-11-15T19:15Z). Full numbers: `workers/g1-m08/round-30.md` in the coordinator's ticket.

- Stages `native`, `repo-mcp`, `dependencies` (about 4.5 min) and `tools` (about 54.5 min) ended without a failure.
- Stage `workspace` (18:48:23Z to 19:11:48Z) was refused after all six cut-smaller steps: `the plan stage verification (during verification) grew to 10230 MB, more than the 6553 MB this machine gives a plan`. Sizes at the cuts: 10180, 8282, 9463, 10038, 9847 MB. The growth is in the verification of input closures, so smaller waves did not lower it. This is the result round 30 asks for and it is the coordinator's to fix; no build code was changed here.
- Slice worker peak: 8021 MB resident, 9366 MB footprint. Memory pressure reached level 4 in 3 of 160 samples.
- The automatic runner was replaced at 19:06Z (new setup digest) and started the stages again by itself; the observer keeps running.

## Round 31 (from 20:51Z)

- The coordinator's round 31 tells every worker session to run no setup command and only observe every 10 minutes. This session never ran one, so nothing had to be stopped.
- `workers/g1-m08/round-31-observe.sh` appends one observation every 10 minutes to `workers/g1-m08/round-31.md` (status row, last 30 log lines, memory pressure, five largest processes). A waiting permission dialog is not measured: the session has no view of the screen.
- Second pass of the runner: `native`, `repo-mcp`, `dependencies`, `tools` (414 s) ended with exit 0; `workspace` runs since 19:55:24Z, not refused after more than 56 minutes (the first run was refused after 23), no exit yet.
- 21:24:46Z: the second `workspace` run ended with exit 75 after 5363 s. It passed verification and was refused in plan stage `inference` (`grew to 10267 MB, more than the 6553 MB this machine gives a plan`). The runner started a third run by itself at 21:25Z. Reported in `workers/g1-m08/round-31.md`.
- 2026-11-16T01:25:21Z: the third `workspace` run ended as `stage-timeout` after 14401 s (peak 2152 MB, no memory refusal). The runner started a fourth run, which was still running at 02:54Z.
- 36 observations in six hours: memory pressure level 1 in all, never ready, `ROUND.md` unchanged at round 31. The observer was started again for six more hours.

## Final State (2026-11-16T09:55Z)

Not ready. The machine is joined, replicating and running the automatic ready runner; four of five setup stages pass. The last stage, `workspace`, cannot finish on this 16 GB machine: every run since 2026-11-15T18:48Z ended as memory refusal (exit 75), stage timeout or stall; `ready.json` stood at `attempts` 15 at 09:55Z. The last refusal read (08:49:57Z): `the plan stage inference (during inference) grew to 10635 MB, more than the 6553 MB this machine gives a plan`.

- 72 observations over twelve hours are in `workers/g1-m08/round-31.md` with the table of the runs; `state.json` lists `workspace:setup` as failed.
- The coordinator posted no round after 31 and no `next.md` for this machine. Observation ended after twice the protocol's six hours; the runner continues by itself and reports to the store.
- Not done: `repo preflight --lang en` (the protocol runs it only when ready). Not measured: waiting permission dialogs, the cut-smaller lines of the runs after the first, which process the runner's peak figure measures.
- What makes it ready is a change to the graph read's memory use (coordinator's code), not anything on this machine.
