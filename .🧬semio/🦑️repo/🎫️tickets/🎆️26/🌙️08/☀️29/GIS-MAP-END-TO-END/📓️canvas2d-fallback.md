# GIS 2D browser fallback

Browser MapSession keeps one MapHost for tile storage, visible tile revisions, vector and raster commands, overlay picking, theme and interaction. Its canvas presenter admits WebGPU first when the browser exposes it. An absent or rejected adapter leaves the canvas available for Canvas2D. A boot failure after the GPU context bound, a four-second device admission stall, or a WebGPU frame error gets a fresh canvas through React and attaches Canvas2D to the same session. The replacement retains the demand-driven frame scheduler and restores keyboard focus when the outgoing canvas held it. When the browser cannot present through either context, the existing localized retry flow remains available.

Canvas2D replays the same logical scene used by the native map painter. It applies device pixel ratio at the presentation boundary and retains a browser image buffer for each raster used across consecutive frames. This avoids round-tripping commands through JSON and image pixels through base64. Place labels already use shaped glyph paths from the shared painter. The map emits plain compositing and clip operations. Scene commands that require nontrivial isolated compositing or opaque Vello fragments return an error from the browser Canvas2D port.

The renderer component regression cases cover startup failure, hung GPU admission, runtime frame failure, preservation of the session and camera, keyboard focus transfer, and localized retry when neither backend starts. All 53 scoped TiledMapHost tests passed. The browser pixel contract uses Playwright and PNGJS with each browser's independent SVG rasterizer. It compares raster alpha and placement, vector geometry, pin and selection paint at 1×, 1.5× and 2×, verifies that editing an overlay updates the paint and picking, and checks that the browser-side raster copy happens once across frames.

Run the pixel contract with `repo tools exec node -- "<ticket path>/📜️canvas2d.mjs" "<built Surface WASM output directory>" [chromium|firefox|webkit]`. The driver needs the directory containing `framework_surface.js` and its WASM file. It supplies the page and bundle through Playwright route fulfillment.

## Verification

- The managed WASM Cargo check passed for the browser target with the Canvas2D presenter and tiled-map session.
- The final scoped TiledMapHost run passed all 53 tests across three suites, including gesture tests and attach, timeout and frame recovery.
- Built the real Surface component for `wasm32-unknown-unknown` with its owned Cargo manifest and generated web bindings with the lockfile-matched wasm-bindgen 0.2.129.
- Headless Chromium, Firefox, and WebKit each matched all six reference frames with WebGPU disabled. Each browser painted and picked the edited overlay and made one raster copy across the measured frames.
- The browser checks exercise the Surface component and its map session directly. Full GIS shell startup and performance on physical devices remain pending.

## Canvas2D replay optimization

Replay applies a fill or stroke setting when its value changes. Color comparisons use RGBA8 to avoid repeatedly formatting CSS. Stroke comparisons separately cover width, cap, dash array, dash offset, and color. The first stroke following a restored clip reinstates the round join and the required style. The renderer keeps the existing start-cap mapping and uses path stroking for dashed or degenerate rectangles. Single rectangle fills use the native rectangle operation with either winding rule.

Installed Chromium matched the native rectangle operations against the original path operations byte for byte across 20 cases. The comparison covered both fill rules, all three caps with round joins, fractional alpha and placement, clipping, fractional scaling, shear, and reflection. The Rust source passed the whitespace check. The managed WASM rebuild and measured pan replay on the optimized build are tracked separately.
