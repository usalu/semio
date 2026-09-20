# Astra Sol — retained Select and overlay raster parity

## Result

Implemented both assigned renderer slices without changing `LayoutSpec::Overlay` semantics.

1. Retained document `Select` now shares the immediate renderer's collision-resolved popup geometry, painted-height clamp, scroll extent/step, visible row window, chevron hit bands, and clipped option-row hit rectangles. The popup opens a glass foreground route, clips its rows, paints its scroll controls in that route, and balances the scissor/glass stacks on completion, fault, revision cancellation, and caller-owned frame cancellation.
2. Raster images painted while an overlay route is active now enter a dedicated per-layer overlay-raster lane. The lane participates in retained admission and scalar retirement, prepared measurement including texture-key bytes, prepared GPU encoding, immediate test rendering, frame census, clear/reset, and foreground phase classification. The raster keeps the same decoded image key and GPU raster-store lookup as its non-overlay peer.

## Source files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs`

## Tests

Added or extended these focused laws:

- `overlay_rasters_are_collected_separately_without_changing_the_texture_identity`
- `prepared_measurement_retains_scene_and_overlay_raster_cursors`
- `overlay_rasters_are_encoded_in_the_foreground_phase`
- `a_decoded_overlay_image_keeps_its_upload_identity_in_the_overlay_raster_lane`
- `retained_select_popup_is_viewport_clamped_scrolled_and_glass_foreground`, including completed and cancelled route-stack closure plus one shared React scroll step

The language-neutral `semio.ui.retained-select-overlay-raster.v1` fixture carries the 20-option viewport, trigger, clamped menu, initial/scrolled row windows, React scroll step, decoded dimensions, shared raster key, and scene/foreground phase order. The paint, draw, prepared-measurement, and prepared-GPU laws all consume it. Its geometry and ordering oracles are React's `resolveSelectPlacement`, `scrollSelectViewport`, and browser portal paint order; decoded key/dimension identity is asserted across both renderer lanes.

Validation performed:

- `git diff --check` on every changed source and focused test file: passed.
- Neutral fixture JSON parse/schema check through Bun: passed.
- Parent-owned `bun nx run '@semio-tech/ui-rs:check-wgpu-engine-wasm' --skip-nx-cache`: passed in 2m03s after the production source stabilized; four existing warnings, no errors.
- First focused Nx attempt without the cached graph shortcut remained at project-graph calculation for about three minutes and was interrupted. No red test result was observed.
- Cached-graph focused Nx attempts reached `bun ./📜️script.ts test-wgpu-engine …` and then waited silently behind the shared Cargo lock. They were interrupted to preserve the parent's integrated native-suite queue. No compile or test result was observed, and no pass is claimed here.
- Parent owns the integrated `bun nx` wgpu/Cargo gate and wasm activation.

## Resource and cancellation audit

- Overlay raster storage is reserved by the existing retained item grant and charged through the existing item/byte claim before insertion.
- Prepared measurement charges one `UiInstance` plus each texture-key byte, then emits the same bounded raster scalar cursor used by GPU encoding.
- Both raster lanes resolve through the same `RasterTextureTable`; routing does not copy or re-admit decoded pixels.
- `DrawList::clear` resets the overlay-route counter and drops both raster lanes; scalar retirement drains overlay keys, entries, and capacity before the layer retires.
- Popup close clears its retained scroll/popup state. Paint completion and cancellation close the scissor and glass-content route, so a stale frame cannot leave subsequent draws routed into the popup foreground.

## Remaining

- Await the parent's integrated native fixture suites; the production wasm compile is green, but the newly wired `#[cfg(test)]` fixture laws remain queued behind the shared Cargo run.
- Runtime visual scoring and wasm activation remain with the parent packet owner.
