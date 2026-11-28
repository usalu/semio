# H53: g1-m04 stuck in stage tools (4/5)

## Timeline (CET, evidence S/origins/g1-m04: ready.json, memory.json, metrics rings, pool.json)
- 00:44-01:14 dependencies ok (H8/H23/H28: scoped install, 116 GB free now, 432k inodes free: disk is no longer a blocker).
- 01:14 tools starts. Since then `build.tasks.running 3 / queued 5-6` is constant: the stage's nested `repo tools setup all` (2048 MB, batch) waits in the capacity gate ("an earlier request is still waiting", 5600 s at 08:11).
- 06:57-07:50 one cargo tree at 100 % of 12 cores; 07:02 pool churn (children started and killed after 7 s).
- 08:00 landed pool + warden-tree; 08:05 first process tree arrived in memory.json (see below).

## Root cause (proven by the new warden `tree` in memory.json, 08:05)
The requests ahead of the stage are NOT tool provisioning. They are agent-started `repo capacity run --memory=N -- timeout <h> bash <ticket 11/22 GET-PUZZLE-PLUGIN-WORKING-END-TO-END>/current-native…|lguest-build.sh|lweb-browse…` commands (2800, 3000, 4000, 5200, 8000 MB, 5-11 h timeouts, started 05:37-06:40 on m04, which is also a dev workstation). `capacity run --memory` makes an exact `retainReservation` request, priority batch, held for the whole script: two running ones hold about 6 GB unresident (the "6.4 GB reserved but not yet resident"), three wait with 4-8 GB that can never fit beside them, and the stage command (2048 MB, minimum 1024) queues behind them (queue-order, FIFO within batch). The 12-core cargo is that puzzle work, not the tool setup. Spare is 1.9 GB against 2.05 GB needed, so the stage is within 150 MB of running and moves only when the puzzle jobs shrink.

## Fixes landed (before the 08:15 seal)
1. Diagnosability (needed to see the above): warden state `tree` (schema `TreeRow`, `wardenTreeV1`, Python oracle test 2/0): heaviest repository processes with pid, parent, age, cpu, memory, shortened command; subjects classified by the warden (orphaned cargo) are roots. Any worker's hang is now readable in `S/origins/<o>/memory.json`.
2. Owner pool flap (found on the way): `ownerPoolSizingV1` stopped children whenever the machine's own work waited and no core was idle, although the wait was a memory wait; with one child the child itself used the last core, so it was started, drained and SIGKILLed 3 s later in a loop (claimed 19, published 0). Now a wait does not stop children when the waiting request does not fit even now, or while a retained reservation of the own read is unrealized. Fixture cases + oracle updated; types 0 errors. NOTE this is `graph/inference` code, landed 08:00, i.e. before the quiet window: coordinator please be aware.

## Not done (needs the 09:00 slot, area host-capacity/ready, not graph)
- The real fix for m04: readiness stage commands must not queue behind foreground dev reservations. Proposal: the ready launcher exports `SEMIO_HOST_CAPACITY_PRIORITY=interactive` into the stage environment (inherited by nested `repo` commands); `sizeHostRequestV1` (`📏️decision`, line 378) and the door request (`🌡️host-capacity/🟦️.ts` 260) honour it, so the stage command ranks before batch waiters and, with minimum 1024 MB, runs shrunk. Test: decision vector (interactive 2048/min 1024 behind batch 8000 with 1.9 GB spare is admitted) against the Python oracle of `📏️decision`.
- Alternative without code: end or shrink the puzzle lane jobs on m04 (not allowed for me: they are another agent's work).
- Workspace stage not reached.

## Unverified
- m04 had not reached Ready when this was written; whether it moves once the puzzle jobs' unresident reservation falls below 2048 MB of spare is open.
- Owner-pool suite: 33 pass / 2 fail, both pre-existing (fixture case 40 vs a peer's swapActive 0.6 -> 10 change; a hook timeout).
