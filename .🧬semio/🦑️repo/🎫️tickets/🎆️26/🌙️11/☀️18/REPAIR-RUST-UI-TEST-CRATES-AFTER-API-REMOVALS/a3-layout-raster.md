# 🧱️ A3: layout laws and raster test crates repaired

## semio-ui-wgpu-layout-laws

- **Before:** the lib tests did not compile. `tree-row-rects` stepped `MountedLayoutJob` through `InteractiveJob::step(&mut job, &mut cx)` without the return claim, under a bare `StepContext::new`. No law owned a `UiArenaFixture`.
- **Root of the port:**
  - `MountedLayoutJob::worker_one` is `pub(super)`.
  - Outside the crate, the only path left was `InteractiveJob::step` with a real `OriginalJobReturnClaim`, which needs an origin plus a decoder.
  - The existing testkit driver `layout_tree_now` already drives the real stage ladder under its own original fixture caller, but only with the down/LTR flow.
  - I added the flow-aware sibling `layout_tree_flowing_now(tree, root, theme, w, h, block_reversed, inline)`, and `layout_tree_now` now delegates to it. It is testkit-only and introduces no new path.
- **Laws:**
  - `tree-row-rects` now lays out through `layout_tree_flowing_now`. The hand-rolled admit/step/publish loop and its now-unused imports are gone.
  - Every law that builds a `UiTree` opens `UiArenaFixture::new()` as its first statement, in both `tree-row-rects` and `table-row-grid`. The dev-dependency `semio-ui-contract-values` (feature `testkit`) was added.
- **Law fix:** `a_touch_tree_lays_every_row_out_at_least_44_pixels_tall…` asserted the tree ROOT height ≥ 132.
  - The mounted root is pinned to the viewport (`measure_one` → `set_root_box(0, viewport)`, 96 px in the fixture), so that assertion could never hold.
  - It is the same root the sibling law pins to `treeHeight`.
  - The law now asserts the stacking where it lives: the `generations` section height ≥ 3 × `TREE_ROW_TOUCH_MIN_PX`.
- **After:** `test result: ok. 16 passed; 0 failed` (doc-tests 0/0). The intermediate run had 15 passed and 1 failed: the touch root assertion.

## semio-ui-wgpu-raster

- **Defect:** this is a manifest bug in production, not a removed API.
  - `RasterUploadPixels::Contiguous` (`cfg(any(test, feature = "testkit"))`) calls `RasterContentIdentity::test_pixels`, which is gated by `semio-ui-raster-owner`'s own `testkit` feature.
  - The raster crate's `testkit = []` never forwarded it, and its test build never enabled it.
- **Fix:**
  - `testkit = ["semio-ui-raster-owner/testkit"]`.
  - A `semio-ui-raster-owner` dev-dependency with `features = ["testkit"]`.
- **After:** `test result: ok. 32 passed; 0 failed` (doc-tests 0/0).

## Files changed

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🏁️collect-publish-and-close/🦀️.rs`: `layout_tree_flowing_now`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs`: re-export
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📊️table-row-grid/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧱️layout-stack/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/📦️packages/🦀️rust/Cargo.toml`

## Unresolved

None. Both crates compile and every test passes.

Neither crate drives a `Ui`, so the coordinator's `UiFixture::settle_layout` was not needed here.
