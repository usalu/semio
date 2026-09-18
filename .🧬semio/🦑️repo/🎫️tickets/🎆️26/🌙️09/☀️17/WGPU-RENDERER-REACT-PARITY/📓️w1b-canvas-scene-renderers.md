# 🖌️ W1b — canvas-like component-scene kinds (Canvas2d, Paint2d, InkCanvas, TextEditor, IconRender)

Packet W1b of 26/09/17/WGPU-RENDERER-REACT-PARITY. Goal: the five canvas-like `SurfaceKind`s paint
real content and dispatch React's action payloads in production, not a blank `theme.panel` rounded
rect.

## 1. What was wrong

`scenes::render_component_scene_step` phase 4 handled `World3d` directly and delegated everything
else to `engine_canvas::sync_engine_scene`, whose match covered only `NodeGraph`/`TiledMap`/`Board2d`
with `_ => false`. A `false` there makes the step `cursor.finish()` immediately — so phases 5–8 never
run and the window body keeps only the panel rect pushed in phase 3. Every one of the five kinds in
this packet painted a blank panel.

The implementations were not deleted on 2026-09-08, they were quarantined into `#[cfg(test)]`-only
include files (`🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`) when the phased cursor replaced the old
`match scene.component_kind` dispatch (`render_component_scene`, visible at
`git show 860e015bf6:"<Scenes wgpu path>"` ~1914).

## 2. Two paint models, deliberately

| Kind | Model | Why |
| --- | --- | --- |
| `Paint2d` | engine surface → vello texture | React drives the wasm `RasterSession`, which wraps `paint::RasterHost`. That host is a plain Rust type in `semio-framework-surface`, already a dependency of the renderer crate — so the wgpu build uses the SAME compositor React does rather than a second layer walker. |
| `TextEditor` | engine surface → vello texture | `framework_editor::EditorHost` already had a `build_scene()` and was already the pointer target; it just had no constructor and no paint. |
| `Canvas2d` | direct retained draw | A canvas-2d document IS a draw list — React's `drawSceneNode` walks the same records into `CanvasRenderingContext2D`. Routing it through a vello texture would be a pointless copy. |
| `InkCanvas` | direct retained draw | Same: strokes/cards/grid/marquee are draw-list primitives. |
| `IconRender` | direct retained draw + world-3d pass | The GLB shot delegates to the same `infinite_world::world::render_world_3d` a `World3d` surface uses; only the frame chrome is local. |

## 3. What moved where

### 3.1 `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (production)

Un-gated in place (the `#[cfg(test)]` attribute removed, code unchanged): `Viewport::from_json` /
`::from_typed` / `::world_to_screen`; `CanvasFillJson`, `CanvasGradientStopJson`, `CanvasStrokeJson`,
`CanvasImageFieldJson`, `CanvasTextFieldJson`, `CanvasLayer`, `Canvas2dPacketText`,
`Canvas2dPacketItem`; `CANVAS2D_SELECTION_GLOW` / `_RING`; `CANVAS_GRADIENT_BANDS`,
`CANVAS_CIRCLE_SEGMENTS`, `canvas_circle_points`; `INK_RESIZE_HANDLES`; and the `use` lines those
need (`base64::Engine`, `DragAxis`/`KeyAction`, `draw_text`/`draw_text_wrapped`/`render_widget`/
`HitKind`/`HitTarget`/`Theme`/`WidgetNode`, `Rgba`, `UiPresence`).

Moved out of `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs` into the production file's own regions
(22 + 9 + 1 items, ≈720 lines):

- `//#region Canvas2d`: `canvas2d_packet_text_size`, `canvas_layer_should_render`,
  `decode_canvas_image_source`, `decode_canvas_image_bytes`, `queue_canvas_image_upload_sized`,
  `queue_canvas_image_upload`, `draw_checkerboard`, `draw_canvas_infinite_grid`, `draw_dashed_line`,
  `canvas_color_channels`, `canvas_mix_rgba`, `canvas_gradient_color_at`, `canvas_blend_channel`,
  `canvas_apply_blend_mode`, `push_shape_fill`, `push_circle_outline`, `push_shape_outline`,
  `push_linear_gradient_fill`, `push_radial_gradient_fill`, `render_canvas_shape_fill`,
  `render_canvas2d_packet_item`, **`render_canvas_2d`**.
- `//#region InkCanvasRender` (was EMPTY): `ink_effective_bounds`, `flatten_ink_items`,
  `ink_selection_bounds`, `ink_text_plain`, `ink_world_to_screen`, `ink_resize_handle_screen_pos`,
  `draw_ink_table`, `draw_ink_image`, `draw_ink_item`, plus `ink_item_visible`, `positive_mod_f32`,
  `draw_ink_rect_outline`, `draw_ink_grid`, `draw_ink_selection_chrome` and **`render_ink_canvas`**
  recovered from `860e015bf6`.
- `//#region IconRender`: the real `IconRenderRequestFields` (the production stub carried only
  `width`/`height`) plus `IconRenderCameraFields`, `IconRenderLightsFields`,
  `IconRenderMaterialFields`, `icon_render_default_zoom`, `icon_render_camera_json`,
  `icon_render_environment_json`, `render_icon_render_empty`, `paint_icon_render_chrome` and
  **`render_icon_render`**.
- `//#region RenderEntry`: `render_placeholder`.

New in `//#region RenderEntry`:

- `ENGINE_PHASE: u16 = 4` — the first cursor phase past the id/metrics/panel preamble.
- `render_component_scene_step` now intercepts `phase() >= ENGINE_PHASE` and routes `World3d`,
  `Canvas2d`, `InkCanvas`, `IconRender` to their own step functions; everything else falls through to
  the unchanged engine-texture ladder (phases 4–8). The old inline `if component_kind == World3d`
  check inside arm `4` is gone (it is the same dispatch, hoisted).
- `render_canvas_2d_step`, `render_ink_canvas_step`, `render_icon_render_step` — each reserves its
  retained-item grant (`canvas_2d_reserve_items`, `INK_CANVAS_RESERVE_ITEMS`,
  `ICON_RENDER_RESERVE_ITEMS`) then paints, then `cursor.finish()`.
- `engine_surface_clear` gained `SurfaceKind::Paint2d => theme.canvas_clear` (it composites over the
  canvas ground, not the panel).

`render_icon_render` was adapted: it takes `&mut SceneEngineHosts<'_>` instead of a private
`HashMap<String, World3dState>`, so the icon's synthetic `World3dScene` attaches against the SAME
`world3d_states` map every other 3-D surface uses, and its `assetUrl` now goes through the new Rust
twin of `meshAssetTransportUrl` (§3.4).

Removed (dead once Paint2d has a real route): the `SurfaceKind::Paint2d` arms of
`passive_scene_wheel`, `passive_scene_pointer_button` and `passive_scene_pointer_move`. Paint2d's
camera is the `RasterHost`'s, exactly as it is React's.

### 3.2 `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`

- `EngineSurface` gained `raster_host: Option<RasterHost>`, `raster_sync_cache: RasterSyncCache`,
  `editor_sync_cache: EditorSyncCache`.
- `RasterSyncCache` / `EditorSyncCache` + `raster_sync_terminal` / `editor_sync_terminal` — the wgpu
  twins of `Paint2dCanvasSurface`'s `documentSyncRef`/`assetsRef` guards.
- `EngineSurfaceRetirement` gained `raster_source`/`raster_sync_cache`/`editor_sync_cache` fields, the
  `EditorSync` / `Raster` / `RasterSync` close phases, `close_editor_sync` / `close_raster_sync`, and
  the matching witness + `terminal_nonopaque_is_empty` clauses.
- `engine_canvas_theme_json(theme)` — the canvas-theme document both hosts read, the twin of
  `syncSessionCanvasTheme`.
- `sync_raster_engine`, `paint2d_asset_bytes`, `sync_paint2d_scene`, `sync_text_editor_scene`.
- `sync_engine_scene` gained `Paint2d` and `TextEditor` arms; `stage_engine_scene_paint` gained
  `Paint2d => raster_host.build_vector_scene()` and `TextEditor => editor.build_scene()`.
- `//#region Paint2d` (new): `PAINT2D_INTERACTION_DOMAIN`/`PAINT2D_LAYER_GRANULARITY`,
  `with_raster_host_mut`, `paint2d_pick_layer`, `interaction_targets_json`, `write_interaction_hover`,
  `write_interaction_select`, `write_surface_camera`, `paint2d_selection_utility`,
  `paint2d_merge_mode`, `paint2d_merge_ids`, `paint2d_selection`, and the three input entries
  `paint2d_pointer_button_into` / `paint2d_pointer_move_into` / `paint2d_wheel_into`.

Paint2d and TextEditor are deliberately NOT registered through `register_engine_surface`: that
registry exists only to mirror surfaces into the shell's bespoke pointer maps
(`node_graph_states`/`tiled_map_states`/`board2d_states`/`world3d_status`), and these two travel the
`SceneInteractionIntent` path instead. No `EngineSurfaceKindDetail` variant was added, so no Shell
edit was needed.

### 3.3 `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`

- `process_scene_interaction` gained a `SurfaceKind::Paint2d` arm routing pointer/wheel to the three
  `paint2d_*_into` entries.
- `//#region ✍️TextEditorFocus` (new): `FocusedTextEditor` + `FOCUSED_TEXT_EDITOR` thread-local,
  `focus_text_editor` (called from the TextEditor `PointerDown` arm), and
  `apply_focused_text_editor_key` — the first and only production caller of
  `engine_canvas::text_editor_apply_key_into`, which previously had none at all.

### 3.4 Other files

- `🔨️modules/🖼️assets/🥽️mesh/🦀️.rs`: new `mesh_asset_transport_url` — the Rust twin of
  `meshAssetTransportUrl` in the sibling `🟦️.ts`. Unknown ids pass through unchanged so the loader
  reports the miss rather than the resolver inventing a filename.
- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`: `mod mesh_assets` is no longer `cfg(not(wasm32))` (the catalog
  is `include_str!`-embedded and the icon host needs it on the browser build too), and `handle_key`
  now offers every key to `interpreter::apply_focused_text_editor_key` before the shell chord table.
- `🔨️modules/🗺️surface/🎨️paint/🦀️.rs`: new `//#region 🔖️Retirement` with `RasterHostRetirement`
  (`new`/`close_step`/`terminal_is_empty`/`Drop`), the raster twin of `MapHostRetirement` — the layer
  tree, decoded-image cache and pixel scratch buffers each drain ONE entry per step.
- `🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`: `RasterImageCache::close_step` /
  `::is_empty`, the bounded drain that retirement walks.

## 4. Action payloads vs React

| Kind | Gesture | wgpu action | args | React source |
| --- | --- | --- | --- | --- |
| Paint2d | move under a selection utility | `interactionHover` | `domainId:"layers"`, `channel:"pointer"`, `targets:"[{\"granularity\":\"layer\",\"id\":…}]"` | `Paint2dHost` `onHoverFocus` → `world3dHoverActionArgs(PAINT2D_INTERACTION_DOMAIN, PAINT2D_LAYER_GRANULARITY, id)` |
| Paint2d | release under a selection utility | `interactionSelect` | `domainId:"layers"`, `targets`, `merge`, `method:"pick"` | `onSelectTarget`/`commitMarqueeSelection` → `world3dSelectionActionArgs(...)` |
| Paint2d | brush release / move / wheel | `setCamera` | `surfaceId`, `camera:{x,y,zoom}` | `dispatch("setCamera", { camera: next })` |
| TextEditor | any printable key, Backspace, Delete, cmd/ctrl+A | `textSelect` then `textEdit` | `surfaceId` + `selectionJson:"{\"start\":n,\"end\":n}"`; `surfaceId` + `document` | `TextEditor` — the same ordered pair, whole document, not a delta |
| Canvas2d | pointer/wheel | `canvasPointerDown`/`Up`/`Move`, `paintStrokeBegin`/`End`, `setCamera` | unchanged | already correct before this packet |
| InkCanvas | pointer/wheel | ink events tagged `operation`, `setSelection`, `setHover`, `setCamera` | unchanged | already correct before this packet |
| IconRender | — | none | — | React's `IconRenderHost` is render-only (74 lines, no handlers) |

`paint2d_merge_mode` mirrors `marqueeModeFromModifiers` (shift → `add`, ctrl/meta → `subtract`, else
`replace`) and `paint2d_merge_ids` mirrors `selectionMergeIds`.

## 5. Verification — PARTIAL, read this carefully

The tree was churned all night by 14 sibling W1 packets and the machine ran at load 20–57 with a
full agent fleet. **The packet's required verification did not complete.** What was actually
observed, in order (logs under `🗑️generated/w1b-*.txt`):

| Run | Command | Result |
| --- | --- | --- |
| baseline | `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **0 errors**, 50 warnings — the crate was green before this packet's edits (P0 had just landed). |
| mid-work | same | blocked by peer breakage in `semio-framework-ui` (`set_surface_scale` missing, `ICON_INLINE_UI_SPACING` missing, `*const ()` not `Send` via taffy) — none of it in files this packet touches. |
| round 1 | same, `-j 1` after 5 OOM kills (`exit=137`) | **12 errors, all mine, all one family**: `canvas_lum` / `canvas_sat` / `canvas_set_lum` / `canvas_set_sat` still `#[cfg(test)]` while the now-production `canvas_blend_channel` calls them. Every other symbol this packet un-gated or moved resolved. Fixed by un-gating those five helpers (`canvas_clip_color` too). |
| round 2 / 3 | same | never reported a verdict: `exit=137` (jetsam) ×9 and `exit=143` (SIGTERM) once, each after 10–40 min queued behind up to 34 concurrent sibling `cargo check` runs on the shared build dir. |

**Not verified, therefore claimed only as intent, not as fact:**

- the native `cargo check` is NOT known to be green after the round-1 fix;
- `cargo check --target wasm32-unknown-unknown` was never run at all;
- `cargo test -p semio-framework-os-renderer-wgpu --lib paint2d_engine_tests` was never run — the
  seven new tests in `⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs` are **written, not executed**;
- no existing test was re-run, so "all existing tests still pass" is NOT asserted here.

What IS verified about the final state: every one of the nine edited files is brace-balanced under a
string/char/raw-string/comment-aware scan, so nothing is left half-applied or non-parsing. The
round-1 log is the only compiler evidence, and it says the move itself resolves.

**For the integrator**, the residual risk after the round-1 fix is concentrated in edits made AFTER
that log: the `Option::as_slice` / `slice::from_ref` narrowing in `paint2d_pointer_button_into` and
`paint2d_pointer_move_into`; the `NodeId` path in `apply_focused_text_editor_key`; the `#[cfg(test)]`
gating of `Paint2dCameraFields`/`Paint2dDocSyncJson`/`paint2d_default_one`; the dedupe of
`CANVAS_GRADIENT_BANDS`/`CANVAS_CIRCLE_SEGMENTS`/`canvas_circle_points` against the pre-existing
`Canvas2dShapes` copy; and the three new close phases in `EngineSurfaceRetirement`. Expect
`unused`-family warnings on the fixture-side Paint2d helpers before anything else.

## 6. Remaining gaps

1. **Bounded paint granularity.** The three direct-paint kinds each paint in ONE cursor step
   (reserving their item grant up front) rather than paging layers/blocks across steps. Paging needs a
   retained per-surface parse cache keyed by scene revision — parsing `layers_json` once per step
   would be O(n²). This matches what phases 7/8 (`paint_node_graph_labels`/`_overlays`) already do in
   production today, so it is not a regression, but it is not the fully paged model either.
2. **Paint2d marquee/lasso overlay.** `RasterHost::marquee_hits_json` exists and the merge helpers
   are wired, but the wgpu side dispatches a single-point pick on release; the drag-rectangle overlay
   and its `marqueeHitsJson` commit are not yet driven (React's `marqueeRef`/`SelectionMarquee`).
3. **Surface context menus.** The wgpu renderer has no `openSurfaceContextMenu` equivalent at all —
   for ANY scene kind, not just these five. `scene.menu` is never read. That is a separate packet.
4. **Paint2d navigator pane.** `navigator_fit_camera_json` is applied so the overview renders fitted,
   but `navigator_viewport_overlay_json`'s "you are here" rect and the navigator's own pan/wheel
   (which dispatch the CONTENT pane's camera) are not wired; the navigator refuses pointer input.
5. **TextEditor popups.** Completions, the rename input and the context menu are still test-only
   helpers (`text_editor_completions`, `render_text_editor_completions`,
   `render_text_editor_context_menu`, `render_text_editor_rename_input`, `text_editor_menu_hit`) —
   the buffer, gutter, selection, carets and token highlighting all paint through `EditorHost`, but
   the overlay chrome above the texture has no production caller yet.
6. **IconRender is synchronous.** React renders the shot offscreen through `iconRenderPort` and shows
   a "Rendering…" state; the wgpu twin paints the GLB straight into the frame each paint. Visually
   equivalent once assets are resident, but there is no pending/error state.
7. **Verification is incomplete** — see §5. Native check unproven after the last fix, wasm check and
   the new tests never run.
