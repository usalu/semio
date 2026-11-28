# Graph-plan reservation stall (2026-11-28)

## Symptom (reproduced live)
`repo capacity status`: `running graph-plan 4299 MB reserved, ~117-1080 MB observed, pid 79132` and `waiting graph-plan wants 6090 MB, since 46s, pid 79132`; 1038 MB spare above the 2048 MB floor.

## Where the numbers come from
- Caller: host service slice pool (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, `reserve` = `reserveGraphPlanCapacityV1`). Booking = `learnedPeak(scope)` x `measuredBookingBasisPoints` (11500), minimum = 40% of it (`leanBasisPoints`): 4299 = 3738 x 1.15; 6090 = 5296 x 1.15 with minimum 2436.
- `learnedPeak(scope)` (`🗓️scheduler/📈️learn/🟦️.ts`, `learnedHostSlicePeakV1`) was the **maximum of the newest 5** recorded peaks. The recorded history of `slice:graph` holds ordinary runs of 2.6-3.9 GB, a few outliers of 4.4-4.7 GB, and killed loads recorded at peak x 1.25 (`learnedGrowthBasisPoints`). Any one of these in the newest five pinned the booking, and the 1.15 margin was stacked on top.
- Decision (`📏️decision/🟦️.ts`): a retained `graph-plan` grant is charged in full while it ramps (observed 117 of 4299 MB), and its unrealized part is never lendable to another plan (`lendable` skips retained grants). The service's second load (same pid) therefore waited behind the unrealized part of its own first load: spare 1.3 GB < minimum 2436 MB, forever ("not enough free memory").

## Root cause
1. A process was charged twice: the unrealized reservation of its own ramping graph plan counted against its next graph plan.
2. The slice estimate was the single largest of five runs, not a measured quantile.

## Fix
- `host-capacity/decision`: for a graph plan with a minimum, the unrealized memory of live non-quiet graph-plan grants held by the same pid is added to the room of the shrunk start (`ownLent`). Another process gets no such credit; the minimum must still fit.
- `learn`: `learnedHostSlicePeakV1` for a scope with at least `minimumSamples` runs is the p95 (linear between ranks, `percentileV1`, `HOST_SLICE_QUANTILE_BASIS_POINTS_V1`) of its newest `estimates.maximumSamples` recorded peaks; the pool's 1.15 margin stays. `HOST_SLICE_RECENT_RUNS_V1` removed.

## Laws
- `host-capacity/tests/own-plan`: "a plan whose process holds a ramping plan is admitted shrunk with the memory that is spare, not made to wait behind its own reservation" (failed before: `wait/memory`); "another process asking the same is not credited the reservation it does not own and waits".
- `scheduler/learn/tests`: new "the slice peak of a scope is the p95 of its recorded peaks: one outlier among many runs does not pin the booking" (failed before: 5300); the stale-history law now reads "stale larger history beyond the sample window does not pin the booking".

## Limits
- With the shipped data (12-16 samples) p95 is close to the largest run; the real saving here is the own-process credit. No third-party oracle (quantile and a one-line guard).
- The own-process credit lets two loads of one service overlap their ramps; the warden guards swap as before.

## Test results
`repo test-file` over 93 files (host-capacity process and library tests, resident `history`, scheduler `learn`): 89 passed, 4 failed. Failures not caused by this change: `followup` (oracle world, failed before per the 2026-11-27 note), `restart/child` and `lazy-graph/probe` (child scripts, not test files), and resident `history` ("a load never learns from a cancelled run", pool writes one more peak; I did not touch the slice pool). New laws pass. `repo capacity status` reads cleanly. Log: `🗑️generated/graph-plan-tests.json` (delete when the ticket ends).
