# Astra Sol UI Integration Repairs

## Result

Repaired the six failures from the 588-test native UI census without changing the retained Select clipping contract or bypassing the production layout/paint ladders.

1. Updated the `ui::wgpu_engine` fixed-slot receipt to the measured `UiSurfaceRegistry` element size of 161832 bytes. Capacity remains 64 and the owner remains 520 bytes.
2. Preserved each completed mounted layout's bottom-up root intrinsic height in the surface owner. `surface_content_height` now answers that viewport-independent measure after accepted publication. Roots with no intrinsic extent, including an engine-surface root, retain their accepted viewport height.
3. Corrected the overlay ladder law to run the layout scheduler after open/close dirties the subtree before driving `frame_into_step`. Exhaustion now reports `paint_stall_census`, which had identified `dirty-layout/subtree-dirty` with no paint frame rather than a paint-phase deadlock.
4. Made the mounted worker routing law use its deterministic test clock. The previous use of the ambient clock could terminate the retained session before `MountedLayoutJob::worker_one` ran, leaving `worker_thread_observed` false without disproving pool routing.
5. Gave the two-row production Select sync law a separate 400×400 document viewport and a control-height trigger at y=100. The former fixture made the Select itself the viewport-sized root, so the collision clamp correctly produced a zero-height popup and zero hit rows. The production implementation and the newer 20-option neutral fixture retain clipped visible rows and zero geometry for rows outside the published window.
6. Authored the engine-surface root fixture as the contract-valid leaf `{ width: "fill", height: "fill" }`. `LeafLayout` requires both sizing fields; no deserialization default or compatibility path was added.

## Files

- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs`

## Validation

- `git diff --check` over the eight changed UI/async source, test, and fixture files: passed.
- Bun JSON parsing and exact receipt assertions for `boxed-fixed-slots` plus schema/item-count assertions for `retained-select-overlay-raster`: passed.
- No Cargo, native, or wasm build was launched from this agent. The parent owns the integrated native census and build slots.
- The parent build sampled the source once and reported Rust E0631 at the intrinsic-height handoff. The call was corrected to use `and_then(|job| job.root_intrinsic_height())`, and the source was handed back as stable for the parent-owned census.

## Contract Notes

The intrinsic height is captured only when the corresponding mounted layout publication reaches `Complete`, so it cannot describe a candidate generation while the tree still exposes the prior accepted geometry. A `f32` value occupies the existing padding between the surface scheduling booleans and revision counters, preserving the measured slot shape expected by the updated receipt.

The overlay failure was not a retained-paint loop. `open_overlay` marks the content root dirty and bubbles `SUBTREE_DIRTY`; the production shell checks `layout_is_dirty`, runs `step_layouts`, and only then enters `frame_into_step`. The repaired law follows that same lifecycle.
# Root Native Follow-Up

The second full native census ran 588 laws: **586 passed, two failed**. The intrinsic-height, mounted-worker, Select geometry and engine-surface fixture laws now pass. The compiler-measured slot size is **161840 bytes**, not 161832: the added intrinsic height increased the element by eight bytes. Root updated the fixture to this measured value; capacity64 and owner520 remain unchanged. The padding assumption below was disproven by runtime measurement.

The overlay law still reported `dirty-layout=false subtree-dirty=true frame=false`. Root traced the production Interpreter's Layout phase: it explicitly calls `request_layout(window_id)` whenever `layout_is_dirty` remains true. The test helper stopped at any global `UiLayoutStep::Idle` and never rearmed a dirty surface after overlay changes. Root corrected the bounded helper to rearm dirty windows, require both Idle and a clean target surface, and report a stall census on exhaustion. No production overlay behavior or paint assertions were weakened. A third full census follows these changes.
# Final Native UI Census — 2026-09-20

Root ran `@semio-tech/ui-rs:test-wgpu-engine --skip-nx-cache -- long --no-fail-fast` after the receipt and scheduler-helper follow-up. **588 tests ran, 588 passed, 0 skipped**; assertion time5.935s, Nx total56.8s. Log: `🗑️generated/astra-runtime/ui-native-census-3.log`. This verifies this UI packet's native laws, including retained popup/raster composition and authored-layout regressions; it does not establish whole-renderer browser parity.
