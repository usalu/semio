# Nested capacity deadlock (2026-11-26)

## Root cause
`repo capacity run --memory=4500 -- repo run ...` holds a 4500 MB grant. The nested `repo run` reads its graph slice through the per-host graph service, a separate long-lived process. The service books the slice worker through `reserveGraphPlanCapacityV1`, which deletes `SEMIO_HOST_CAPACITY_GRANT` and queues as a brand-new request (learned peak x 1.15 = 6553 MB, minimum equal to the full amount on the learned path). The service never knew the requester's grant, so the nested request competed with its own parent at the host gate and the parent could not release while the child waited. The gate treated the outer grant as "ours", so the starvation refusal never fired: an endless wait.

## Sizing (item 2)
6553 MB is not a corrupted value: it is the learned slice peak (capped by the host ceiling) times the 11500 bp booking margin. On the learned path the first ask uses minimum == full by design (fair queueing for the real need); the reduced start (`elasticity.minimumMegabytes`) is only tried after a refusal. With the fixes below the refusal now happens at once under a parent grant, so the reduced start is reached immediately. No sizing change was needed.

## Fix
- Schema-first: `PlanRequest.grant` (resident protocol) carries the requester's `SEMIO_HOST_CAPACITY_GRANT`; client -> service frame -> planner port -> slice pool request -> `reserve({parentGrant})`. Job and share keys include the grant so a nested request never joins a load queued independently of its parent.
- `reserveGraphPlanCapacityV1` with a live non-quiet parent grant charges the plan inside the grant (`reserveWithinParentGrantV1`): no second ledger row, worker attached to the parent's heartbeat, in-grant bookings tracked and released. What does not fit (below the plan's minimum) is refused at once with the bilingual `nestedExceedsGrant` error and declared to the parent via `declareHostCapacityNeedV1` so its class learns the need. A smaller `minimumMegabytes` is admitted reduced (the pool already narrows the wave).
- Gate deadlock detection (generic): a waiting request whose ancestors (named `parentGrant`, the inherited grant id, or grants held by processes in its parent chain) hold grants is checked with the pure `hostAncestorDeadlockV1`: refused with `ancestorHolds` (en/de) when the budget less the ancestors' holdings cannot cover its minimum, or when nothing but ancestors runs and it waited the override time.

## Tests
- `process/.../host-capacity/tests/ancestors` (language-agnostic `fixtures` JSON + gate tests): 9 pass.
- `library/.../host-capacity/tests/nested` (+ JSON fixture): 3 pass.
- Regression: graph-plan, shrink, slice-share, booking, quiet, need all pass.
- `resident/tests/protocol` has 2 failures unrelated to `grant`: a fixture expects `arguments: [""]` invalid while the schema accepts it (not touched by this change).
- typecheck: only 2 pre-existing errors in `host-capacity/disk/tests`.
