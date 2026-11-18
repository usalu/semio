# R113: pool children that do almost no work

## Evidence (g1-server, 2026-11-17 03:55 to 03:58 UTC, load 60)
- The pool (pid 7339) had started 24 children in 25 minutes and held 10 to 14 of them; `published 0`, `claimed 3`, `claimAttempts 20..28`, `target 0`, `reason pressure`.
- Every live child is in state `RNs`: runnable at nice 10, 14 to 27 s of CPU in 2 to 7 minutes (3 to 6 percent). They are starved by the machine (load 60, one 4.3 GB worker at 130 to 180 percent), not waiting on the server and not slow to start.
- Start of one child of my own against a copy of the wave (4681 units, `units.json` 25 MB, 21 MB of it the `json` member that only `ownerJobV1` reads): 0.58 s wall warm, 7.1 s cold; `JSON.parse` of the list 93 ms, `openMilliseconds` 320 to 500. So the start is not the cost; claiming round trips are 2 per child and minute at most.
- Why they end: the pool itself. With `target 0` the loop drains the newest child on every look (2 s), whether it is old or young or has claimed anything; a drained child abandons its prefetched claims (claimed 3, published 0). A target that flips between 0 and a few makes the pool start two children per look and drain them again. Nothing recorded the reason: `onExit` set `lastError` only for unexpected codes, and `lastError` stayed `None` on g1-server and five workers over 24 minutes, which also shows the R108 watchdog did not end healthy children there (a watchdog exit would have set `childEnded exit 78`). Not proven by a recorded exit, only by that absence.
- Worker evidence (coordinator, 04:05): 18 and 23 children under `reason pressure`. Cause: the start test compared `have` (children not draining) with the target, so draining children that live on (starved, owner in flight) and survivors of earlier pools were not counted, and the pool started more.

## Change (landed 2026-11-17 05:33:32 UTC, entry R113 in landings.md)
- `🚪️exits`: every child exit gets a record (reason: watchdog, stopped, revision, open-failed, recycled, drained, finished, unexpected; code; age; owners done; EN and DE line). `pool.json` gets `exits` (counts by reason) and `lastExits` (newest 8 with `text.en`, `text.de`); each exit is also said in the pool log.
- `🐕️watchdog`: a dead or replaced parent still ends the child after 3 s; a stale lease, another pool's lease or a lease without the wave ends it only after 120 s without one holding verdict (`ownerPoolChildFateV1`); lease stale limit 20 s to 60 s; it writes its verdict to `child-<n>.exit.json` before it ends the process so the pool records the reason.
- `🧮️lifecycle`: a start only when live children (draining ones and survivors included) are below the target, no own child is idle and younger than 90 looks, none is still loading (one at a time), and 8 looks after the last start; a surplus is drained only from children at least 90 looks (180 s) old, except under `pressure`, idle ones first, then the newest. The pool no longer starts two children per look.
- Not changed: unit list slimming (not the cost, measured above), nice 10 (outside the pool).

## Tests
- `🧪️tests/🚪️exits` 4, `🧪️tests/🧮️lifecycle` 3 (including a 600-look simulation with failing starts, draining children, survivors and a target falling to 0: live never above target, no start at target 0), `🧪️tests/🪦️reaper` 7 (was 5; new pure fate test and a real child that survives a lease starved for a while and records `lease-stale` when it stays lost). Red first: the new test files fail on the unchanged tree (missing members); green on the staged copy. Landing battery under the lock: owner-pool 30, reaper 7, exits 4, lifecycle 4, all other owner-pool tests and both standing preflights unchanged and green. Two earlier attempts (04:31, 05:00) were restored on red: tests that assumed an immediate fan-out (fixed with a `ramp` option, off in the test rigs) and a lease removed by an orderly pool end (now short grace). The lock overran 5 minutes.

## Owners per minute per child
- Before (pool.json, ownersPerMinute / children): g1-server 0 / 10 to 14; workers 0.2 to 1.4 per machine / 6 to 24 children, 0.01 to 0.07 per child per minute.
- After: not measured (needs hours of fleet run after landing).

## Unverified
- The pool on this Mac and the workers still run the code from before the landing until their pools restart; no `lastExits` seen yet.
- That no exit was a watchdog kill (only inferred); the recorded `lastExits` after landing will show it.
- Whether the fewer children raise owners per minute on loaded machines.
- `invalid build.closure.provider-entry-stale {"port":"layers"}` on g1-m03 03:14: pool children publish owner records and closure travel entries to the fleet cache but replace nothing in a running reader's own closure store; I found no path from the pool to that error, not proven either way, not touched.
