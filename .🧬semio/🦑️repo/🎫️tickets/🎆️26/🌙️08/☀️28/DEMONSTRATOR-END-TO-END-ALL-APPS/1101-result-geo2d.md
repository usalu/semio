STATUS: GREEN native `--lib` tests for 2d-booleans 27/27 (`geo2d-bool3`), 2d-path-stroke 6/6 (`geo2d-stroke2`), 2d-trace 18/18 (`geo2d-trace4`); all rc 0, logs in `🗑️generated/1101/geo2d-*.txt`. Wasm and other crates not rechecked. All failures were stale tests, no code change.

Driver: `1101-tools/geo2d/t.sh <n> -p <crate>` with `MAN=🧰️framework/Cargo.toml`.

## 2d-booleans (3 stale tests)
- `cancellation_covers_every_path_stage...` (`🛤️paths/🧪️tests`): the test positioned the replay job by semantic work, but the Admitting->Flattening hand-off is a zero-semantic-work turn (probe showed t1: work stays 1, phase flattening). The replay now steps the same number of turns as the probe.
- `path_boolean_retirement_composes_child_owners...`: fixture case `booleanClosing` offset 1 is unreachable, because `Boolean` phase takes the child retirement and enters `Complete` in one zero-work step, so no semantic work remains. Fixture offset is now 0 (`🧫️fixtures/🧹️retirement/🔣️.json`; the TS test reads the same fixture and tolerates it).
- `translated_coordinates_preserve_topology` (`🧪️tests/🔬️unit`): origin 1e12 with epsilon 1e-9 is below f64 coordinate precision, which the "Boolean epsilon is below coordinate precision" guard correctly refuses. Added `evaluate_with(..., epsilon)` helper; the test uses epsilon 1e-2 and still asserts one contour of 9 segments.

## 2d-path-stroke (1 stale test)
`large_crossing_strokes...`: the dash-expansion budget error ("Stroke dashing exceeds point budget") arrives after 32787 turns of 4096; the test only allowed 1000. It now iterates up to `turn_bound()`.

## 2d-trace (test build)
Tests used borrowed `&[u8]` masks, but `BitmapTraceJob<M>` needs `M: RetireOwned`. Converted to owned `Vec<u8>` (`.clone()` per use, `foreground` takes `&BitmapTraceInput`, work-test caller adjusted). The drop-witness law uses a hand-written `MaskCursor`/`MaskDrop` retirement because `artifact_retire_leaf!` now requires `Copy`; it still asserts zero drops until the final owned close and exactly one afterwards. Third-party/neutral fixture comparisons untouched.

No `[DEBUG]` text left (grep clean).
