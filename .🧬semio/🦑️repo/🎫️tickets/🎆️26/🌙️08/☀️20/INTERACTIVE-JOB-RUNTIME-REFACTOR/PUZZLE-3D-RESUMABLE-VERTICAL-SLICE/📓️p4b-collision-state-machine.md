# P4b Collision State Machine

## Result

The Puzzle 3D overlap path is now a synchronous, persistent collision state machine. The former production `solid_overlap_volume` run-to-completion function was removed.

## Before

- One call computed both world AABBs, scanned every part pair, and ran all Monte-Carlo samples.
- The seeded LCG, sample cursor, inside count, intersection bounds, and early-rejection decision existed only as stack locals.
- Fill scanned every placed collision body.
- No cancellation, deadline, or fuel boundary existed inside collision sampling.

## After

- `CollisionOverlapState` owns the explicit stages `BroadPhaseInit`, `PartPairs`, `SampleInit`, `Sampling`, and `Complete`.
- Persistent state includes ordered part-pair cursors, intersection bounds/volume, total and batched sample counts, seeded LCG state, sample cursor, inside count, last sample, result, and early-rejection state.
- `step` processes one broad-phase/part transition or at most eight fill samples, checking cancellation and yield before each bounded unit and consuming framework fuel after work.
- `semio_framework_job::StepContext` directly implements the owned `CollisionStepContext` seam.
- Numerical behavior is retained: seed `0x9e3779b9`, LCG constants `1664525`/`1013904223`, total-sample denominator, surface-contact threshold, fallback containment point, and exact `overlap_budget + 1.0` early rejection.
- `CollisionSpatialIndex` is an incrementally maintained deterministic spatial hash. Cells and entries use ordered maps; cell members use binary insertion; exact AABB filtering returns lexicographically ordered object ids. Adversarially large cell spans fall back to a bounded ordered-entry scan instead of materializing an unbounded cell range.
- Collision state derives serde encoding so a fill checkpoint can resume the exact pair/RNG/sample position.

## Tests

Focused source tests cover batch sizes 1/7/64, exact checkpoint/resume state, disjoint bodies, coincident bodies, touching surfaces, early budget rejection, cancellation/yield/fuel preservation, deterministic spatial-index upsert/query/remove including adversarial cell spans, and a one-sample 8 ms watchdog.

## Validation

- `rustfmt --edition 2021`: completed.
- Scoped `git diff --check`: clean.
- Static census: zero `async fn`, `.await`, private thread/pool, `block_on`, or `run_blocking` uses in the precompute subtree.
- Isolated `bun nx run @semio-tech/puzzle-plugin:test-quick --skip-nx-cache` uses ticket-local `🧪️target-p4`; final outcome is recorded in the P4a report because the same gate covers both packets.
