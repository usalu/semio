# Worker g1-m05 (Windows Workstation): Setup and Collaboration With the Coordinator

All worker reports of the protocol live in the coordinator's ticket, folder `26/11/14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m05/` (the protocol allows a worker to write only there). This file is the summary of this ticket.

## State Found (2026-11-16T00:45Z)

- The machine is already joined: origin `g1-m05`, group `g1`, worktree `C:\semio`, store `S:\` reachable, replica task `semio-fleet-replica` running, code `b4c6513b87ef`. Long paths are on, the clock is synchronised.
- The completion marker `.cache\f\native-bootstrap.done` is absent: the worker is not ready.
- The coordinator's automatic ready runner works on this machine by itself: `native` and `repo-mcp` are done, `dependencies` failed six times with exit 1 and twice with `stage-timeout` (3 600 s); try 9 started at 00:39:29Z.
- The machine had no `workers/g1-m05` folder in the coordinator's ticket.
- The session was opened in `C:\git\semio`. That folder is an old git checkout that is not part of the fleet (no `.cache\f`, launcher fails there with a registry text error). The `repo` MCP of the session is bound to it, so this ticket was written by hand into the shared worktree `C:\semio` instead of through `ticket_open`.

## Decisions

- Followed `ROUND.md` round 31: no setup command, nothing under `.cache\f\ready` touched, no process stopped, no build code changed, no git command that changes anything, no elevated command.
- No `repo` command at all beside the running stage (not even `preflight` or `status`); the state was read from files and process counters.

## Files Written in the Coordinator's Ticket

- `workers/g1-m05/round-31.pre.md`: machine, replica, runner state and one 30 s measurement of the running `dependencies` try.
- `workers/g1-m05/round-31-observe.ps1`: the read-only observer (10 minute interval, ends on `ready`, a new round or a changed `next.md`).
- `workers/g1-m05/round-31.md`: its observations.
- `workers/g1-m05/state.json`.

## Contribution to the Open Question (Q2 of `pool-failures-and-windows-timeout.md`)

The worker process of `dependencies sync --all` does about 6 800 file operations per second while it reads 0.2 MB and writes nothing in 30 s, on a third of one core, with Defender idle. The stage is bound by single-threaded metadata calls on NTFS, not by a lock held elsewhere, not by content I/O and not by the virus scanner at that moment. The split into read, write and other operations is in every observation of `round-31.md`.

## Check That Reports Arrive

`S:\v1\origins\g1-m05\head.json` had not moved since 2026-11-15T15:41Z. It was replaced at 2026-11-16T00:50:51Z, right after the three files were written (3 134 to 3 137 events), so this machine publishes.

## Second Contribution (2026-11-16T07:20Z): The Stall Rule Stops a Working Stage

File `workers/g1-m05/round-31.stall-rule.md`. After six hours of observation the worker was still not ready. Since 04:50Z every `dependencies` run on this machine ended with `outcome: stalled`: the coordinator's new 10 minute stall rule (R74) stops the stage during a phase in which the worker writes no log line but does about 55 000 file operations per second. Predicted from the last log line for 07:17:00Z, seen at 07:17:07Z. The five stops and the worker's counters before each are in the file, with a proposal (a moving counter in that phase, or the process tree's operation counters as a second signal) and one correction to R70 (`store-input-replay` is metadata-bound too, it reads 25 KB in 10 s).

## Third Contribution (2026-11-16T13:10Z): Publication Rate Against the Cap

File `workers/g1-m05/round-31.publish-rate.md`. The coordinator found the same cause in its result R77 (the publication of about 1 500 bindings printed nothing) and landed a progress counter (R77b). The run of 12:31:53Z is the first one that is not stopped as stalled, after 15 stalled runs. Measured here: 6.3 s per binding, so the one-hour cap of the stage cuts the run at about binding 450; the file gives the projection (about four runs, or one run with the cap raised to four hours).

## Fourth Contribution (2026-11-16T19:20Z): `dependencies` Done, `tools` Shows the Same Pattern

File `workers/g1-m05/round-31.tools-stage.md`. Stage `dependencies` finished at 13:35:23Z (my four-run projection was wrong, about 320 bindings were left). Stage `tools` (4 of 5) failed seven times within seconds and was then stopped eleven times as stalled; a sample shows its worker at about 35 000 file operations per second while it prints only identical lines without a counter. Proposal sent: a counter on those two phases, as the coordinator did for `dependencies`.

## Replica Stuck and Restarted (2026-11-16T19:20Z)

File `workers/g1-m05/round-31.replica-stuck.md`. From 18:48Z the replica daemon published and applied nothing although its heartbeat was alive; the build agent and the `ready` child left ten minutes later and were not started again. Restarting the scheduled task did not replace the daemon, so I ended its two processes (93524, 51764); the task started a fresh daemon, which applied 485 events and published 3, and the `ready` child came back and adopted the running `tools` stage. The cause of the stuck round is not known. This is the only action of the session that is not observation.

## Result at 07:20Z (Superseded by the Sections Above)

- Not ready: stage `dependencies` (3 of 5), `attempts` 10, five stalled runs, a sixth running. `tools` and `workspace` have not started. `repo preflight --lang en` was not run, because the round asks for it only at `ready: true`.
- The cause is in the build code that the coordinator owns (stall rule against a silent phase, slow first sync on Windows); a worker may not change it in this round.
- The observer was restarted at 07:00Z for six more hours; it lives only as long as the session.
- The ticket stays open until the worker is ready or the coordinator says stop.

## Fifth Contribution (2026-11-17T01:30Z): The Running `tools` Try Predates the Coordinator's Fix

File `workers/g1-m05/round-31.tools-old-code.md`. The coordinator added the counter I proposed and rewrote the slow reads (its result R99, landed 23:00Z). The try running here started at 22:31:45Z with the old code: no stall stop any more, but one counter step per 20 s at about 790 000 file operations per step; it is cut at 02:31:45Z unless it finishes. The next try is the first Windows run of the new code. Still not ready, stage 4 of 5. The replica has not been stuck again. Observer restarted 01:30Z for six hours.

## Sixth Contribution (2026-11-17T07:35Z): The Coordinator's Fix Does Not Speed Up `tools` on Windows

File `workers/g1-m05/round-31.tools-new-code.md`. The old-code try was cut at the hard cap (14 424 s), seven tries were then stopped as stalled, and the try of 04:25:43Z (the first started after the R99 landing) costs about 770 000 file operations per counter step, the same as before. It is cut at 08:25:43Z unless it finishes. Still not ready, stage 4 of 5. Replica not stuck. Observer restarted 07:35Z for six hours.

## Seventh Contribution (2026-11-17T13:45Z): `tools` Ends in a Capacity Error; Short Stalls Are Waits for `show projects`

File `workers/g1-m05/round-31.tools-capacity-error.md`. The slow phase does end here after more than three hours, and then the stage fails with `Internal error: Immutable input physical work capacity exceeded` (the try of 10:03:25Z after 11 273 s; the try of 04:25:43Z has the same outcome row). The short `stalled` tries in between are the stage waiting for a live `repo show projects` that holds the dependency store; the stall rule stops that wait every 600 s. Still not ready, stage 4 of 5, `attempts` 12. Replica not stuck. Observer restarted 13:45Z for six hours.

## Terminal Window Flood Stopped (2026-11-17T20:50Z, on the Owner's Order)

File `workers/g1-m05/round-31.console-windows.md`. After the reboot of 20:13Z the build agent's worker pool opened terminal windows in bursts of eight (45 console hosts within five minutes): processes started detached have no console on Windows, and their children were started without `windowsHide`. Added `windowsHide: true` at 13 spawn sites of the pool, the farms, the sidecar and the resident host, restarted the agent's processes, and measured no new console in 270 s. The scheduled tasks' windows are fixed in the task definition (`conhost.exe --headless`, proven with two temporary tasks) but not applied here: re-registering needs an elevated shell (`headless-task.ps1` in this folder does it for one task). Scripts of the probes are in this folder.
