# One slice worker, seven graph-plan grants

Ticket 26/11/25/FIX-CAPACITY-ADMISSION-STARVATION-AND-RESERVATION-OVERCOMMIT (reopened for lane CAP-GP).

## Cause

Not one grant per changed project, per retry or leaked on cancellation: it was the **grant growth of the slice pool**.
`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` (`grow`) books the memory a running slice worker needs beyond its grant by calling `reserve` again, and `reserveGraphPlanCapacityV1` opened a **new, separate `graph-plan` grant row** for each growth (up to `sliceGrantGrowth.maximum` = 6). 1 base + 6 growths = the seven grants of pid 93404.

Consequences seen in `repo capacity status`:
- every growth grant was never `attach`ed to the worker, so `graphPlanObserverV1` counted 0 MB for it: 292 to 606 MB reserved, 0 MB observed, held until the worker ended (404 to 1116 s), and 0 MB observed never lets a reservation settle;
- the growth rows counted as extra grants of the session (cap, fair order), 5.3 GB reserved for one 3.9 GB worker.

## Fix (at the cause)

- Gate (`host-capacity/🎫️gate/🟦️.ts`): the handle of a held grant gets `grow({megabytes,minimumMegabytes,signal})` (`HostGrantGrowV1`). It rewrites the reservation of the very grant in place through the existing upgrade path (`upgradeHostGrantV1`): fair queue, only the difference has to fit, ramp restarted, cancellable by signal, calls of one grant serialised. No second row, no second waiter identity.
- Product (`🌡️host-capacity/🟦️.ts`): `reserveGraphPlanCapacityV1` returns `GraphPlanReservationV1` with `grow`; gate-off grants at once; a nested plan grows inside the parent grant's free reservation.
- Slice pool: growth calls `reservation.grow(...)` and adds the answer to what the worker is granted; the `extras` list of stacked grants is gone. The worker tree is observed by the one grant, so the observed memory is real.

Not changed: floor, interactive reserve, budget, session cap (the stacked rows no longer exist, so they no longer inflate it). No row schema change: processes on old code keep their old behaviour until they restart.

## Tests

- `🌡️host-capacity/🧪️tests/🧮️graph-plan/🟦️.ts` + fixture `🔣️one-grant.json`: six growths of a plan leave exactly one `graph-plan` row with the summed reservation (4792 MB), a competing request queues behind it and starts at release, a released grant grows by nothing; a growth waiting for memory ends with its signal leaving no waiter and no second grant.
- `🏡️resident/🧪️tests/📈️grant-growth/🟦️.ts`: fake reserve now gives `grow`; asserts growth rides on the one grant (only it is released).

## Verification

- `repo test-file` over the 157 capacity and resident test files: 150 pass. graph-plan (15/15 incl. 2 new) and grant-growth (8/8) are green.
- 7 red files, none touching grants of the plan or growth: `🔁️restart/📜️child` and `🪶️lazy-graph/🧲️probe` are helper scripts picked up by the glob, `📨️protocol` (frame schema oracle), `🌤️followup` (quiet oracle), `🔬️footprint` (macOS footprint), `🧾️blockers` (blockers oracle) and `🧷️history` (a cancelled load learns 3000 MB twice: learner behaviour, not growth). Decision, sampling and learner code are untouched by this change and are being edited concurrently by others.
- Live: the host service (pid 93404 era) runs the old implementation pinned until it is replaced, and the host had no `graph-plan` grant at 01:5x (`repo capacity status` shows only 3 command grants and 4 waiters), so the stacked state could not be re-produced live; the new behaviour shows once the service restarts onto this code (one `graph-plan` row per worker, growing in place, the observed MB following the worker).
