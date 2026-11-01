STATUS: EDITS DONE, UNVERIFIED (wasm checks queued; logs `C/🗑️generated/1101/plugunwrap-{1..6}.txt`, markers `.rc`)

Rows came from `aud-policy-unwrap-rows.txt` (28 rows under `✏️s/`). Many line numbers were stale; I fixed every unwrap/expect in the listed files whose function already returns a typed failure.

## Fixed (typed failure through the existing path)
- procedural/generation3d `preview-eval/tessellation` row 67: `unwrap_or_else(|_|unreachable!())` and the `?` that sat directly in a `RetainedWorkOutcome` closure; now an inner closure returning `Result<_,ValueError>` with `ControlledRetirement::new(..).map_err(|(error,_)|error)?`.
- fem `engine/material/patch` row 36: two `texts[..].unwrap()` -> `ok_or_else(invalid)?`.
- fem `mesh/mesh-job-run` rows 383/384 were already fixed; remaining 7 `expect`s in `run` -> `Self::fail(..)` (typed `MeshRun` failure), 1 in `retire_delivered` -> `ValueError::invariant`.
- energy `commands/action` row 60: 5x `.result?.unwrap()`, `wire`, `receiver` unwraps -> `ok_or_else(invalid)?`. Assumes `begin_granted(..).result?` is an `Option` (the compiler decides).
- Out of the strict pane scope but listed rows (layout/draw are not in the demonstrator project.json); edited since cheap, also checked: layout `media-export-job` rows 35 (all 6 unwrap/expect), `patch-page .../source` (all unwraps), `export-tool-job/retained` (all unwraps); draw `clipboard/job` row 273 only.

## Left, needs a signature change (later)
- generation2d/3d `dsl-mirror` rows 49/50: `print_dsl(&self) -> String` is the framework `ArtifactDsl` trait; `from_envelope_id(..).expect` cannot fail typed.
- generation3d `compute` row 199 (`step_with_fuel` -> `WidgetStep`, feature `funded-turns` test driver) and `modules/host` row 180 (`generation3d_retire_displaced_cold`, `cfg(test|testkit)` returning `()`).
- fem `2d .../cold-laws` rows 27-66: `assert_cold_mutation_*` are assertion helpers, panics are their contract.
- draw `scene/prepare/testkit` rows 10/14: test macros. draw `clipboard/job` has ~15 more unwraps (lines 164-303) not on the row list.
- stdio rows (ifc field_edit/scalar_edit/tessellation/pack, jpg snapshot, tiff diff, semio sqlite native drop): not demonstrator pane crates, untouched. `drop()` and `embedded_catalog` cannot return a failure without a signature change.
- generation3d tessellation has more unwraps (lines 29, 56-60) beyond row 67; they sit in `Result` functions and are next.
