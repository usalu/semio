# w2-canvas-paint — final report

Only file touched: `framework/renderer/wgpu/rs/lib.rs`. All edits in `//#region Canvas2d`, `//#region Paint2d`, plus the narrowly-permitted `render_ui_image` function (+ adjacent new helpers) inside `pub mod interpreter`. Never touched `dock`, `engine_canvas`, `plugin_bridge`, `shell`, or `scenes::SceneRuntime`/`SceneInput`/`RenderEntry`/`Table`/`GraphTimeline`/`InkCanvas`/`NodeGraph`/`TiledMap`/`IconRender`/`Board2d`/`VirtualFileSystem`/`TextEditor`.

## Canvas2d — draw-record kinds added
Read the full `Canvas2dScene` payload (`layersJson` = generic `CanvasLayerRecord[]`) and `canvas-2d-host.tsx`'s renderer. Extended `CanvasLayer` with `role`, `visible`, `opacity`, `blendMode`, `selected`, `fill` (solid/`linearGradient`/`radialGradient`), `stroke` (color/width/dash), nested `image`, and `text`. Added:
- **Metadata records**: `role === "meta"` (and `visible: false`) now skipped from rendering (`canvas_layer_should_render`), matching React's `layers.filter(role !== "meta")`.
- **Gradients**: `push_linear_gradient_fill`/`push_radial_gradient_fill` approximate linear/radial `CanvasGradientStop[]` as N discrete solid-color bands/concentric rings via `push_triangle_fan` + `push_scissor` (no per-vertex gradient primitive exists in `ui_wgpu::DrawList` without a shared-crate pipeline change, out of scope — documented in doc comments).
- **Blend modes**: `canvas_apply_blend_mode` implements all 16 CSS blend modes (W3C formulas, including proper SetLum/SetSat for hue/saturation/color/luminosity) by pre-blending the resolved color against `theme.canvas_clear` — an approximation for true per-pixel GPU blend state, documented as such.
- **Overlay annotations**: `selected: true` now draws an accent-colored highlight ring (rect or circle) on top of the shape.
- Also added opacity/stroke support to lines/polylines, nested `image`/`text` fields, and reused `draw_ink_rect_outline`/`draw_dashed_line` from sibling (off-limits but callable) regions rather than duplicating.

## Paint2d — navigator overlay + pointer tools
Found `Paint2dScene.compositeViewportJson` + `viewMode: "navigator"`. Ported the exact math from `framework/surface/paint/rs/lib.rs`'s `RasterHost::navigator_fit_camera_json`/`navigator_viewport_overlay_json` (a sibling crate not wired as a dependency of this renderer) using this file's own `Viewport`/`Rect`: `paint2d_navigator_fit_viewport` (fits camera to pixel-layer bounds with 24px padding) and `paint2d_navigator_overlay_rect` (maps the main viewport's world rect into the navigator's fitted screen space). `render_paint_2d` now branches on `view_mode` and draws the "you are here" overlay ring.

**Pointer tools — flag for w2-scene-wiring**: `SceneInput::handle_scene_pointer_button`/`handle_scene_pointer_move` (off-limits) already fully routes Canvas2d (`canvasPointerDown/Move/Up`, wheel-zoom, pan). Paint2d only gets wheel-zoom and middle/right-button pan — left-click/drag paint-tool dispatch is not wired (no `paintPointerDown/Move/Up`-style actions with world coordinates). Did not add this (touching `SceneInput` is off-limits); enriched the existing generic `paint2dClick` hit registration to carry `activeUtility`/`brushSize`/`brushOpacity`. **w2-scene-wiring should add a Paint2d case to `handle_scene_pointer_button`/`_move` mirroring the Canvas2d pattern.**

## render_ui_image (interpreter) — URL/SVG loading
- Made `decode_canvas_image` `pub(crate)` (1-line, within Canvas2d region) to share it.
- Added a `PendingUiImageFetch` queue (`collect_pending_ui_image_fetches`/`apply_ui_image_bytes`), mirroring the existing `PendingMapTileFetch` pattern exactly.
- Added inline `data:image/svg+xml` decoding (base64 and plain/percent-encoded) via a new `rasterize_svg_to_rgba` (reuses `usvg`/`resvg`/`tiny_skia`, already deps — a natural-aspect-ratio sibling to `icon_atlas`'s fixed-24×24 `rasterize_svg`, not directly reusable cross-module).
- Fetched/rasterized bytes are re-encoded as `data:image/png;base64,...` and pushed through the existing `queue_canvas_image_upload` (so the "skip decode when unchanged" digest caching is inherited for free).
- Added `object_contain_rect` (CSS `object-fit: contain` equivalent) applied to all `render_ui_image` paths.

**Flag for shell owner**: the fetch queue is complete and unit-tested but not wired into `poll_pending_assets` (in `shell`, off-limits) — that function needs a stanza calling `collect_pending_ui_image_fetches()`/`apply_ui_image_bytes(...)`, mirroring its existing `map`/`glb` handling. **Wave 3 must wire this or URL/SVG image loading is inert.**

## Build/verify
`cargo check` clean (only pre-existing warnings). `cargo test -p semio-framework-renderer-wgpu --lib`: **99 passed, 1 failed** — the failure (`dock::tests::apply_drop_tab_moves_window_across_stacks`) is in the off-limits `dock` module, unrelated to anything touched here (already tracked, being fixed by `w2-dock-dnd`). Added 22 new tests across `interpreter::render_plan_validator_tests` (SVG/URL image loading, object-contain) and `scenes::raster_frame_cost_tests` (gradients, blend modes, meta filtering, navigator fit/overlay).
