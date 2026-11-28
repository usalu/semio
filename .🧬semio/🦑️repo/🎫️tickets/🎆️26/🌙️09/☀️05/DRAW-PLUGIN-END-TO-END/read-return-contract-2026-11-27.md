# Source read return contract for the Draw scene cleanup, 2026-11-27

## Decision: the early return is sound for the mounted scene cleanup

- Private plan owners cannot depend on the source snapshot. `DocumentVectorOwners` (and every child it holds: `DocumentSceneCursor`, `DocumentTraceJob`, `DocumentTextJob`, `DocumentBooleanJob`, `DocumentScenePlan`, `ScenePlanCloseJob`, `RasterSceneAsset { Arc<RasterImage> }`) has no lifetime parameter, no `NonNull`/raw pointer and no `&DrawingSnapshot`; only `DocumentVectorSource<'a>` borrows or owns the snapshot, and it lives beside the owners in `DocumentVectorRetirement`, not inside them. The retirement steps (`ControlledRetirement::step(body, effect)`) take no document argument, so draining never dereferences the source. `DocumentSceneCursor` keeps source positions only for `advance`, which is never called on a retiring job.
- Store semantics: `SnapshotRead::return_to_registry*` moves the `Arc<DrawingSnapshot>` into the registry's own retirement cursor; plan data is deep-copied (`String`, `Vec`, `Arc<RasterImage>`), so nothing in the plan aliases the returned root. A lease has no timer; only its holder releases it, so holding it while draining was the stall (26-40 s of ~270 ms maintenance turns).
- Contract: a returned read is a lease on the capture, and the cleanup owner must return it independent of how large its private plan is. The mounted cleanup now returns it within a constant number of steps (one take step, then the `returned` retirement), and drains the private owners afterwards.

## Scope: strict for export, early only for the mounted cleanup

The PNG/export path keeps its documented contract ("holds the real source through encoding and physical close"): `DocumentVectorRetirement::take_snapshot_read` and `DocumentRasterRetirement::take_snapshot_read` stay strict (None until private owners are empty). Soundness would also allow early return there, but export has no latency problem, and its laws and the `ExportProducer::close_step` ordering (output, read box, then source hand-back) depend on the strict order, so it is not widened.

## Changes

- `scene/📋️prepare/🦀️.rs`: `take_snapshot_read` restored strict; new `release_snapshot_read` (early hand-back) and `source_pending` on `DocumentVectorRetirement`; docstrings state the split.
- `scene/🧵️mounted/🦀️.rs`: `State::close_step` returns the read first through `release_snapshot_read`, then drains owners; `State::demands` quotes `returned` first and a depth-1 demand while the source is pending, mirroring the step order (before, `returned` was quoted only after `cleanup` emptied, which would mis-size grants now that both coexist).
- Laws: the five failing prepare laws stay unchanged (strict contract restored); new `vector_retirement_releases_the_read_early_only_through_the_scoped_mounted_hand_back` (prepare) and `mounted_cleanup_hands_the_source_read_back_within_bounded_steps_regardless_of_private_owner_count` (mounted; steps 1/12/80/400, at most 2 maintenance steps, equal counts, private owners still draining afterwards).

## Verification

- `semio-s-artifact-draw-drawing-scene-tests`: 49 passed, 0 failed (includes the five formerly failing laws and the new prepare law). Log: `🗑️generated/r27-scene-tests.txt`.
- `semio-s-artifact-draw-drawing-scene-mounted` (where `mounted/🦀️.rs` and its laws are compiled): all 15 laws pass, including the new bounded hand-back law. Log: `🗑️generated/r27-scene-mounted.txt`.
- Two mounted laws were adjusted for the new order: the painted-scene law no longer asserts that `returned` stays empty during paint cleanup (it now asserts the lease is handed back by the second cleanup unit and was renamed `..._returns_the_source_before_retiring`), and the failed-slot law's drain bound grew from 1000 to 100000 units (the private drain of a failed job measures 6795 one-item units, which is why element-wise draining stalled the lease for seconds; the drain itself is unchanged here).
- Not run: browser Demo to Blank probe (needs a rebuilt component, excluded by instructions).
