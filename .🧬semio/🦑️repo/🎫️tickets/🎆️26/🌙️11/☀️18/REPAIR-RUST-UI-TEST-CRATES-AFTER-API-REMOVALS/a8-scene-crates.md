# 🎬️ a8: Scene Crate Laws (mesh, modelling, picking, plane)

## Root causes and fixes

### `semio-ui-scene-plane`: `paint2d_scene_splits_into_the_declared_lanes_and_merges_back`
- The diff between the two objects was one field: `editingOptions.filterAmount`. Rust serialized `Number(0.0)` and the fixture held `Number(0)`. `Paint2dEditingOptions::filter_amount` is an `f64`, and `serde_json::Value` treats the float `0.0` and the integer `0` as different numbers. The fixture already writes its other `f64` fields as floats (`brushSize: 4.0`, `brushOpacity: 1.0`).
- Fix: the fixture now has `"filterAmount":0.0` in both the `assembled` and `spine` objects. The TS twins (`🎬️scene/🛤️lanes/🟦️.ts` and `🧪️tests/🧫️fixtures/🟦️.ts`) read the same JSON, and in JS `0.0 === 0`, so they did not need a change.

### `semio-ui-scene-picking`: `screen_select_components_filters_by_active_instance_id` and `screen_select_original_vertices_skips_surface_samples`
- This was a production bug. Every `Mesh3dLease` read calls `try_lock` on the global mesh authority and returns `Mesh3dFault::Busy` when another thread holds it. `screen_select_components` and `screen_select_instances` silently dropped every fault with `let Ok(..) else { continue }`, `.ok()?` or `unwrap_or(index)`. While other threads were writing meshes, a contended read either returned an empty selection or replaced the component id with the raw index. That is why the tests failed only sometimes, under parallel tests.
- Fix: both functions now return `Result<Vec<String>, Mesh3dFault>` and pass every read fault up to the caller. This matches `ray_pick_instance` and `ray_pick_mesh_detail`.
  - The helpers `vertex` and `mesh_triangle` now return `Result`.
  - A new helper, `component_id`, falls back to the index only when the schema declares no id field (`declared == 0`). It no longer falls back on a read error.
  - Callers retry through the existing bounded helpers:
    - `mesh_test_attempt` in picking-unit.
    - `world_test_mesh_query` in the world interaction call sites (gated behind testkit) and in the host marquee laws. The import was added in the host unit file.

### `semio-ui-scene-modelling`: `original_scalar_field_retirement_keeps_every_native_owner_and_physical_receipt` and `original_projection_record_retirement_uses_exact_scalar_owner_receipts`
- The laws' driver was out of date. The fixture grant allows `maximumCopyBytes: 32`, but `ControlledRetirement` now quotes fixed scaffold events above that, such as publishing a pending cursor at 40 bytes. A fixed narrow grant is refused with zero progress on every turn, so the loop never reaches terminal-empty.
- The current contract comes from the value law `original_controlled_narrow_child_keeps_real_semantic_minimum_and_same_grant` (helper `stepped_narrow`). The driver first steps with the narrow grant. If that is refused, it widens the copy grant to `next_demand(copy).copy_bytes` and steps again.
- Fix: a new `narrow_turn` helper in the modelling law follows that contract, and receipts are checked against the grant actually used. Every other assertion is unchanged: zero heap on a denied turn, exact physical receipts, births equal releases, and a terminal drop with heap `(0,0)`.

### `semio-ui-scene-mesh`: `mesh3d_original_component_backings_and_cancelled_hash_index_close_with_exact_receipts`
- There were two causes.
  1. **Production bug.** `authored_step` (`🧱️mesh-owner`) ran `ControlledRetirement::step` through `AuthoringWallet::run_settled`. That opens the effect with `invocation_extent`, which adds the reply-handoff copy bytes, and the owner stepped on that widened extent. With a 1-byte copy grant, a cursor reservation copied 48 bytes, so the progress returned did not fit the caller's grant. A debug run caught this: `progress copied_bytes 48` under `grant maximum_copy_bytes 1`.
     - Fix: the owner now steps on `effect.reborrow(grant, MESH3D_CLOSE_WORK)`, which is the caller's exact grant. This is the same narrowing `invoke_original(.., Some((grant, work)), ..)` does for decode allocation and construction.
  2. **Stale driver.** The abort loop granted `copy = work` (1, 3 or 64) with no widening, so it could never pay the fixed semantic minimum.
     - Fix: if a narrow turn is refused with zero progress, the law re-grants with `work.max(demand.copy_bytes)`, under the same `stepped_narrow` contract as above. It still asserts zero heap on the refusal and `fits(used)`, exact receipts, and `original + born == physical`.

## Files changed
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🎯️picking/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️picking-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🖱️pointer-and-picking/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🔲️cursor-and-marquee-laws/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️world3d-modelling-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️mesh-unit/🦀️.rs`
- Ticket script: `a8-wrap-screen-select.py`, which wraps the screen-selection call sites in a retry helper.

## Incident
The first version of `a8-wrap-screen-select.py` opened its target for writing before reading it. That truncated three files for a few seconds:
- `picking-unit/🦀️.rs`
- `pointer-and-picking/🦀️.rs`
- `cursor-and-marquee-laws/🦀️.rs`

I restored them right away from `HEAD` (`git show HEAD:<path> > <path>`, which only reads git), then applied the edit again.

Evidence that no uncommitted work was lost:
- The picking-unit `HEAD` content matched the failing line numbers (386, 454) in the sweep log, and its diff now holds only my 10 wrapped calls.
- No other `.rs` file in the world module had changed since the last replica commit (`3cde99d`). The world files themselves were last committed on Nov 5 and Nov 10.

## Results
One build covered all four crates (`-p semio-ui-scene-mesh -p semio-ui-scene-modelling -p semio-ui-scene-picking -p semio-ui-scene-plane`):
- semio-ui-scene-mesh: `test result: ok. 18 passed; 0 failed`
- semio-ui-scene-modelling: `test result: ok. 16 passed; 0 failed`
- semio-ui-scene-picking: `test result: ok. 43 passed; 0 failed`
- semio-ui-scene-plane: `test result: ok. 22 passed; 0 failed`

The world host test build compiles (`--no-run --features testkit`, exit 0), and that build includes the changed world interaction call sites. The world tests were not run.

The `a8-*` logs were removed from `🗑️generated`.
