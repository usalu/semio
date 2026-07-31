# @kernel/3d-scene — Test Suite Overhaul Report

## Unit
- Nx project: `@kernel/3d-scene`
- Root: `kernel/3d/scene`
- Crate: `kernel_3d_scene` (Cargo.toml `[package] name = "kernel_3d_scene"`)

## What ran today
- `kernel/3d/scene/script.ts` `TestScript` called raw `Bun.spawnSync(["cargo", "test", "-p", "kernel_3d_scene"], ...)` directly — no wall-clock kill, and build time folded into the same invocation as test execution.
- `kernel/3d/scene/project.json` `test` target already just called `bun ./📜️script.ts test` (no change needed there).

## Test inventory (in-source `#[cfg(test)] mod tests` in `kernel/3d/scene/rs/lib.rs`)
24 tests existed before this ticket, covering camera/orbit math, `Mat4` inverse round-trips, ray/triangle/AABB picking, frustum culling, screen-space marquee/lasso selection, and LOD-grid banding. All are real geometry/algorithm coverage (matrix inverses, ray-triangle intersection, frustum-plane containment, polygon/segment intersection for marquee selection, LOD banding logic) — none are export-exists checks, getter/identity assertions, CSS-substring checks, serde round-trips, or enumerated string-list comparisons.

### Removed (1 test — near-duplicate)
- `concrete_forest_camera_look_at_inside_frustum_planes` — deleted. It looped over the 6 frustum planes computed from the same `concrete_forest_camera()` fixture and asserted `plane.normal.dot(target) + plane.distance >= -1e-2` for each — this is the exact same computation, over the exact same planes/target, as the loop already inside the kept `concrete_forest_frustum_contains_target_box` test (which uses a tighter `-1e-3` epsilon and additionally asserts an AABB-intersection case). The deleted test added no new code path, only a looser epsilon on an identical check — a near-duplicate loop-generated case per this ticket's delete criteria.

### Kept (23 tests)
Everything else was kept: `orbit_round_trip`, `point_in_square`, `mat4_inverse_round_trips_to_identity`, `mat4_inverse_undoes_view_projection`, `ray_from_screen_center_points_at_target`, `ray_hits_triangle_direct`, `ray_hits_box`, `ray_aabb_misses_offset_box`, `frustum_contains_origin_box`, `frustum_culls_behind_camera_box`, `concrete_forest_frustum_contains_target_box`, `concrete_forest_frustum_culls_off_axis_boxes`, `perspective_maps_depth_to_wgpu_ndc`, `rectangle_marquee_bounds_use_start_and_end_corners`, `projected_aabb_skips_far_instance`, `marquee_is_crossing_follows_drag_direction`, `marquee_is_crossing_from_path_lasso_uses_first_horizontal_step`, `screen_select_instances_window_requires_full_vertex_enclosure`, `lod_from_camera_distance_scales`, `lod_progressive_grid_layers_adds_bands`, `lod_progressive_grid_layer_key_stable_within_band`, `pick_closest_lod_prefers_more_detailed_on_tie`, `floating_origin_rebase_subtracts_anchor` — each exercises real branching/transform/geometry logic (coordinate transforms, ray-triangle/AABB intersection, frustum culling, marquee crossing/lasso direction logic, LOD banding) per the task's explicit keep-category, not padding.

## Runner migration
Mechanical swap, matching the pattern already used elsewhere in the repo (`writer/rs`, `ui/wgpu/rs`, `animate/video/rs`, etc.):

```
- import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
+ import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../repo/lib/js/index.ts";

  class TestScript extends BundleScript {
    run(): void {
-     Bun.spawnSync(["cargo", "test", "-p", "kernel_3d_scene"], {
-       cwd: this.repoRoot,
-       stdin: "inherit",
-       stdout: "inherit",
-       stderr: "inherit",
-     });
+     runCargoTestBudgeted(["kernel_3d_scene"], this.repoRoot);
    }
  }
```

`runCargoTestBudgeted` (from `repo/lib/js/index.ts`) does an un-budgeted `cargo build --tests -p kernel_3d_scene` followed by a budgeted `cargo test -p kernel_3d_scene` (30s wall-clock kill via `SEMIO_TEST_BUDGET_MS`/`runTestBudgeted`). No other part of `script.ts` (`ScriptRouter` registration, `runBundleScriptMain` call, the unused `join` import which was already present and unrelated to this change) was touched.

Nothing about this unit is genuinely e2e/integration (pure math crate, no server/container/browser), so no `test-e2e` target was added — the default `test` target is the right place for this.

## Timing measurement
The shared `target/` directory in this sandbox was under heavy concurrent-session lock contention throughout this ticket (dozens of other sessions' `cargo build`/`cargo test` processes queued on the same `flock`, some stalled for hours — consistent with this repo's documented "Concurrent Cargo Workspace Churn" condition). To get a clean, verifiable number without adding to that shared backlog or waiting indefinitely, I built and ran this crate under an isolated `CARGO_TARGET_DIR` in this ticket's own scratch space (`kernel_3d_scene` only depends on `mathematical_algebra`, so this is a small, self-contained build unaffected by the rest of the workspace).

- Un-timed `cargo build --tests -p kernel_3d_scene`: succeeds in well under a second (`Finished dev profile ... in 0.64s` cold, `0.15-0.44s` on subsequent warm builds).
- **Before** (24 tests, warm `cargo test -p kernel_3d_scene`): `real 0.23s` (`user 0.14s`, `sys 0.07s`); reported `test result: ok. 24 passed; ... finished in 0.00s`.
- **After** (23 tests, warm `cargo test -p kernel_3d_scene`): `real 0.23s` (`user 0.14s`, `sys 0.06s`); reported `test result: ok. 23 passed; ... finished in 0.00s`.

Both before and after are effectively instantaneous — this is a pure in-memory math crate (camera/matrix/ray/polygon arithmetic) with no I/O, no WASM, no external process — so the single near-duplicate test removed had no measurable effect on wall-clock time; it was removed purely for redundant-coverage hygiene per this ticket's classification rules, not to hit the budget (the budget was never at risk).

## Files touched
- `kernel/3d/scene/script.ts` — migrated `TestScript` from raw `Bun.spawnSync(["cargo", "test", ...])` to `runCargoTestBudgeted(["kernel_3d_scene"], this.repoRoot)`; import list updated accordingly.
- `kernel/3d/scene/rs/lib.rs` — removed 1 near-duplicate test (`concrete_forest_camera_look_at_inside_frustum_planes`); 23 tests remain, all kept as real geometry/algorithm coverage. `#region`/`#endregion` structure elsewhere in the file untouched.
- `kernel/3d/scene/project.json` — reviewed only, not modified (already `bun ./📜️script.ts test`, no `test-e2e` needed).
- `.repo/🎫️/26/07/17/OVERHAUL-TEST-SUITES-TO-30S-BUDGET-PER-APP/report-kernel-3d-scene.md` — this report.
