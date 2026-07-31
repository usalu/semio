# @semio-tech/sourcing-curate-rs — test suite overhaul report

## Status: Runner migrated to budgeted execution; tests trimmed; wall-clock timing NOT obtained (environment blocker, not a code issue)

## What was found
- `sourcing/curate/rs/script.ts`'s `TestScript.run()` called
  `execFileSync("cargo", ["test", "-p", "sourcing_curate", ...segments], { stdio: "inherit", cwd: this.repoRoot })`
  directly — not routed through any budget helper, so the target was previously unbounded.
- `sourcing/curate/rs/lib.rs` (739 lines) has a single `#[cfg(test)] mod tests` (lines 553-739) with
  18 tests covering: typology tree containment/flattening, geometry-recipe mesh generation (box/frame/
  slab), the pool-table filter predicate (`filtered_stock`), curated-count clamping (`curate_delta`/
  `curate_set`), grid layout placement/scaling, plus several data-consistency/serde-padding tests over
  the crate's static demo catalogue data.

## Classification (18 -> 12 tests)
**DELETE (6, trivial/duplicate):**
- `document_serde_round_trips_with_defaults` - plain-struct serde/JSON round-trip padding on an
  all-defaults document; no branching logic exercised.
- `module_ids_are_unique` - enumerated string-list uniqueness check over a static 3-item list.
- `demo_kind_ids_are_globally_unique` - enumerated string-list uniqueness check over a static
  ~10-item list.
- `every_demo_kind_typology_path_exists_in_its_module_tree` - loop-generated data-consistency check
  that re-exercises the exact same `typology_contains` code path already directly asserted true/false
  in `typology_contains_and_flatten`, just against more static data.
- `slab_recipe_produces_valid_mesh` - `GeometryRecipe::Slab` realizes via the identical private
  `box_mesh_spec` function already validated by `box_recipe_produces_valid_mesh`; near-duplicate of
  the same code path with different literals.
- `every_module_preview_mesh_is_valid` - loops all 10 demo kinds across 3 modules through
  `preview_mesh`, but every kind resolves to either `box_mesh_spec` or `frame_mesh_spec`, both already
  covered by dedicated tests; adds no new code-path coverage over static data.

**KEEP (12, real logic):**
`filtered_stock_matches_query`, `filtered_stock_matches_module`, `filtered_stock_matches_typology_prefix`,
`filtered_stock_matches_min_availability` (multi-dimensional filter predicate),
`curate_delta_clamps_to_availability_and_zero_floor`, `curate_delta_unknown_object_is_noop`,
`curate_set_removes_entry_at_zero` (clamping/removal state logic), `typology_contains_and_flatten`
(recursive tree walk), `box_recipe_produces_valid_mesh`, `frame_recipe_concatenates_four_pieces_into_a_valid_mesh`
(geometry mesh-building algorithms, including the 4-piece mitred-frame composition), `grid_placement_centers_around_origin`,
`grid_scale_normalizes_to_cell_size` (grid layout math).

## What was done
1. Edited `sourcing/curate/rs/lib.rs` in place, deleting the 6 tests above from the existing
   `#region 🔖Tests` block (no new test file created; `sample_document`/`assert_mesh_spec_is_valid`
   helpers kept since still used by the remaining 12 tests). Test count: 18 -> 12.
2. Migrated `sourcing/curate/rs/script.ts`'s `TestScript` from the raw `execFileSync` call to
   `runCargoTestBudgeted(["sourcing_curate"], this.repoRoot, segments)`, imported from
   `../../../repo/lib/js/index.ts` (same relative depth already used for `runWasmPackWebBuild`),
   mirroring the identical already-landed pattern in `animate/core/rs/script.ts` and others. This
   makes the `test` target wall-clock-budgeted (30s hard SIGKILL) going forward, regardless of the
   timing question below - previously it was unbounded.

## Why timing wasn't captured
`cargo build --tests -p sourcing_curate` (warm, un-timed per instructions) was left running for
over 32 minutes and never produced a single fresh `sourcing_curate*.rcgu.o` object file (checked
repeatedly against `target/debug/deps/` - the only cached ones on disk were 4 days stale) - meaning
it never got past compiling its dependency graph (`vcs` -> `rusqlite` with the `bundled` feature,
i.e. the full SQLite C amalgamation, on the non-wasm target; plus `serde`/`serde_json`/`thiserror`/
`semio-framework-core`/`semio-framework-hash`). Over the same window:
- `uptime` showed sustained load averages of 17-28 on a 10-core machine.
- `lsof` on `target/debug/.cargo-build-lock` showed 30+ concurrent `cargo` processes (`ps aux`
  showed 170-190 live `rustc`/`cargo` processes system-wide) - other sessions' builds sharing the
  same workspace `target/` directory, consistent with this repo's documented "Concurrent Cargo
  Workspace Churn" pattern (many simultaneous agent sessions on one machine/workspace).
- No compile error was ever observed at any point - only severe CPU starvation, not a defect in
  `sourcing_curate` or in the `script.ts` change.

This matches the same environment blocker already reported by the sibling
`report-semio-tech-puzzle-3d-rs.md` unit in this same ticket. Rather than continue indefinitely, the
build was left running in the background (not killed) and this report was finalized with the code
changes complete but the numeric before/after seconds unmeasured.

## Result
- Test count: 18 before -> 12 after (6 trivial/duplicate deletions, no coverage of real logic lost -
  every deleted test's code path is still exercised by a kept test).
- Baseline/after wall-clock seconds: not measured (blocked as above).
- Confidence note: `sourcing_curate`'s tests are pure in-memory struct/geometry logic with no I/O,
  no async, no WASM instantiation in the test path itself, over tiny static fixtures (<=10 demo
  kinds) - actual `cargo test` execution time (once compiled) is expected to be well under a second,
  i.e. far inside the 30s budget. This is inferred from the code, not confirmed by a timed run.
- The runner is now budget-enforced (`runCargoTestBudgeted`) so any future run - once the shared
  workspace isn't saturated - will self-report/hard-kill at 30s rather than run unbounded; re-running
  this unit once contention clears would give the actual number.

## Files touched
- `/Users/ueli/Documents/semio/sourcing/curate/rs/lib.rs` (deleted 6 trivial/duplicate tests from the
  existing `#region 🔖Tests` block; 12 tests remain)
- `/Users/ueli/Documents/semio/sourcing/curate/rs/script.ts` (migrated `TestScript` to
  `runCargoTestBudgeted`)
