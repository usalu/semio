# Findings

## Symptom

After ramping the puzzle 3d fill slider up, sliding down no longer removes objects, and sliding back up cannot add new ones.

## Root cause

`setFillCount` materializes fill objects into the document while keeping an in-memory fill plan (`fill.base` + `sequence`). Any later incidental action (`setHover`, pick, mesh sync, …) called `sync_precompute_session` → `set_scene` with that applied projection. `set_scene` treated it as a brand-new scene, ran `rebuild_queue()`, and baked the filled objects into a fresh `fill.base`. After that the slider could only plan *on top of* the already-filled scene — it could neither remove the baked objects nor replan their tail.

Two integration details still made a repaired fill session appear stuck during rapid interaction:

- The shared slider treated its `ready` progress extent as a hard input maximum, so the user could not retain a requested count while the replacement tail was still being planned.
- The async measure dispatcher retained only the latest pending value. During a fast up-down-up gesture it could discard the downward turning point, even though that point is the command that truncates and restarts the worker plan.

## Fix

In `puzzle/3d` `set_scene`: if the incoming fixture is the active fill plan's base plus applied fill objects (compared by id sets, tolerant of attraction-rederive pose drift), update non-fixture scene fields and keep the fill session instead of rebuilding.

The fill engine now also tracks its applied prefix independently from its ready sequence. Reducing the count truncates the discarded tail, rebuilds the fixture from the retained prefix, and requeues fill work without rewinding the random state, so the next increase produces a different result.

The slider's fixed range remains fully interactive while `ready` is display-only progress. Puzzle 3d retains an above-ready requested count and `fillBuildTick` dispatches a coalesced `setFillCount` as replacements become available. The renderer serializes slider actions directionally: it coalesces movement in one direction but preserves every reversal.

## Regression coverage

- The renderer dispatcher test proves that `20 → 18 → 12 → 8 → 14 → 20` is serialized as `20 → 8 → 20` while the first action is in flight.
- The shared slider test proves ArrowRight can advance from `55` to `56` when `ready=55` and `max=100`.
- The Puzzle 3d integration test proves ready planning is non-mutating, down removes objects immediately, an immediate up request remains visible and non-blocking, worker ticks materialize it without another gesture, and the regenerated fill ids differ from the discarded tail.
