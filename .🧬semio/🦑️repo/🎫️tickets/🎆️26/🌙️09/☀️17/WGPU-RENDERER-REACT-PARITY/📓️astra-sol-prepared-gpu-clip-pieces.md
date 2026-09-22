# Prepared GPU Clip Pieces

## Contract

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖥️prepared-gpu-clip-pieces/🔣️.json` defines one language-neutral clip contract for ordinary UI, vector/raster/UI overlays, glass, and World color scalars. A color scalar advances through one exact `layer scissor ∩ silhouette piece` per opportunity. World color additionally intersects the pass viewport. Empty intersections do no GPU work. World shadow commands execute once. Glass snapshots once and composites every nonempty piece from that snapshot. Overlapping silhouette pieces are refused before retained ownership because drawing their intersection twice changes alpha.

The independent oracle is `tiny-skia` 0.11.4, already locked as a dev-only dependency. It rasterizes each admitted union and checks painted and unpainted samples, including the gap between two disjoint pieces.

## Fail first

UI19 was compile-only and is not behavioral evidence: two comparisons used `Vec<serde_json::Value>` against `serde_json::Value`. Those test types were corrected without changing the expected contract.

UI20 executed both laws and failed both intended assertions: the prepared presenter had no clip-piece watchdog cursor, and overlapping pieces were accepted as `Ok((4, 240))` instead of `Err(LimitExceeded)`. Receipt: `🗑️generated/astra-runtime/ui20-clip-red/run.log`; 0/2 passed, 665 outside the filter, 0.128 s test time, Nx 33.2 s.

## Production repair

- `ClipRegion` rejects pairwise-overlapping nonempty physical pieces before it installs a retained clip layer.
- `PreparedGpuPresentCursor` owns `clip_piece`; close and abandonment transfer/reset it, and the host presentation watchdog includes it.
- `prepared_command_clip_piece` is the first-party behavioral resolver used by both the test and the GPU ladder. It resolves the exact command layer, one bounded piece, the layer scissor, and the World pass viewport without collecting or searching sibling commands.
- Normal UI, vector, raster, World mesh/material/textured/grid/line color families receive the resolved scissor. Shadow begin/caster commands remain one-shot.
- Glass scans empty pieces without snapshot work, snapshots once on the first nonempty piece, then composites all remaining pieces without refreshing the snapshot.
- The stall-watch neutral fixture now carries `(phase, command, clip piece, blur mip)` and its native parser uses the same four-field shape.

## Verification

All edited Rust sources parse through `rustfmt --emit stdout`. JSON fixtures parse through Python's standard JSON decoder. UI full verification is pending root's serial Cargo lane.

## UI22 focused receipt

UI22 executed both clip laws. The behavioral prepared-GPU progression and TinySkia oracle law passed. The overlap refusal law still failed because `claim_retained_output(usize::MAX, usize::MAX)` overflowed the aggregate prepared counter before it reached the active grant, returning `false` without setting `grant.faulted`. The overflow path now faults the active retained grant before returning, matching the existing bounded refusal contract used by every draw emitter. Receipt: `🗑️generated/astra-runtime/ui-engine22-clip-filtered/run.log`; 1/2 passed, 665 outside the filter.

## UI23 full receipt

UI23 ran the full UI target: 667 tests executed, 666 passed, zero skipped, 4.937 s. Both clip laws passed. The sole failure was the exact `UiSurfaceRegistry` byte census: the measured slot grew from 164128 to 164224 bytes after the accepted compact Tree metrics/cursor fields. The committed fixed-slot fixture now records the measured 164224-byte element while preserving capacity 64, owner size 520 bytes, and the existing bounded stack ceiling. Receipt: `🗑️generated/astra-runtime/ui-engine23-compact-clip-full/run.log`.
## Native145 source-law repair

Native145 (`renderer-native145-compact-ownership-ax/run.log`) failed only the older three-argument source assertion in `glass_snapshots_the_accumulated_composite_at_its_authored_command`. The live command phase now resolves `prepared_command_clip_piece` and calls `encode_prepared_draw_scalar` with an explicit `None` or `Some(scissor)` fourth argument. The law now pins both paths, the per-piece resolver, and the Glass-to-`SnapshotBackdrop` transition. This retains the authored snapshot invariant while covering the clip-piece protocol that UI22 and UI25 exercised behaviorally.

No GPU production code changed in this repair. Native rerun remains root-owned.
