---
name: Flow Wgpu Rich Rendering Parity
overview: Restore premigration-parity rendering for the Flow node graph (edges, port channels, node chrome, LOD, and labels) by fixing a canvas-theme sync regression and an incorrect label-overlay positioning algorithm introduced during the framework migration.
todos:
 - id: wgpu-theme-host-methods
   content: Add set_canvas_theme_dark to FlowHost (flow/core/rs/lib.rs) and GraphHost (framework/graph/rs/lib.rs)
   status: completed
 - id: wgpu-theme-sync-call
   content: Derive dark/light from ctx.theme in engine_canvas.rs and call set_canvas_theme_dark for both Flow and Dag branches, diff-gated in NodeGraphSyncCache
   status: completed
 - id: react-theme-fix
   content: Replace JSON.stringify({}) stub in flow-graph-canvas-host.tsx with syncSessionCanvasTheme(session) from @semio-tech/ui-styling
   status: completed
 - id: react-label-overlay-rewrite
   content: Rewrite paintDagLabelOverlays/parseDagLabelRows in graph-canvas-overlays.tsx to match premigration dagWorldToScreen + textAlign + shrink-to-fit font clamp + interaction chrome
   status: completed
 - id: wgpu-label-overlay-rewrite
   content: Rewrite paint_label_overlay_row in engine_canvas.rs to use direct anchor + measured text alignment + shrink-to-fit sizing + real hover/selection/preselect chrome from host accessors
   status: completed
 - id: verify-build-and-visual
   content: Run cargo tests, rebuild wgpu wasm, and visually verify in browser (default zoom, zoomed LOD, selection) against React reference for flow and one other DAG playground
   status: completed
isProject: false
---

# Flow Wgpu Rich Rendering Parity

## Root cause (confirmed via live browser test + source diff against `premigration` tag)

I opened the running dev server (`http://127.0.0.1:6118/?plugin=flow`, wgpu renderer) and captured screenshots. Result: node title/port text labels render, but **node rectangles, edges, and port handles are completely invisible**, and selecting/hovering a node is the only thing that shows any chrome. Zoom (LOD) works but reveals nothing extra.

Tracing `mathematical/graph/port/directed/dag/rs/lib.rs::paint_scene` (this file is **byte-identical to `premigration`** — `git diff --stat premigration -- mathematical/graph/port/directed/dag/rs/lib.rs` is empty, so the paint engine itself is not the problem):

- Node fill is only painted when a node is "chrome" (dimmed/selected/highlighted/hovered) — plain nodes are stroke-only by design (`dag_node_paint_fill`, `mathematical/graph/port/directed/dag/rs/lib.rs:1552-1563`).
- All stroke/fill/handle colors come from `self.canvas_theme: CanvasThemePalette` (`mathematical/graph/port/directed/dag/rs/lib.rs:1780`), which defaults to `CanvasThemePalette::from_board_theme(&ui_styling::BOARD_LIGHT)` (`mathematical/graph/port/directed/rs/lib.rs:680-684`) — a **dark-stroke-on-light-canvas** palette.
- The only way to replace this default is `FlowHost::set_canvas_theme_from_json` / `GraphHost`'s wasm `setCanvasThemeJson` (`flow/core/rs/lib.rs:2928`, `framework/graph/rs/lib.rs:478`).
- **`framework/renderer/wgpu/rs/engine_canvas.rs` never calls this at all.** Meanwhile the actual canvas clear color comes from a _different_, correctly dark-mode-aware theme: `vello_clear(ctx.theme)` uses `ui_wgpu::Theme.canvas_clear` (`engine_canvas.rs:17-20`, `321`).
- Net effect: canvas background is dark (correct app theme) but node/edge/handle strokes are the dark BOARD_LIGHT colors meant for a light canvas → near-zero contrast → invisible. Only `node_fill_selected`/`node_stroke_selected` are theme-independent bright red, which is why clicking/selecting is the only thing that shows anything.
- Confirmed against the `premigration` git tag: `flow/react/index.tsx:3021` called `syncSessionCanvasTheme(sessionRef.current)` (from `ui/styling/js/index.ts:386`) which pushes the real, live app-theme-derived `CanvasThemePalette` JSON into the session every time the document theme changes. This call was dropped from the wgpu renderer entirely, and even the ported React host (`framework/renderer/react/components/flow-graph-canvas-host.tsx:352-353`) now stubs it out with `setCanvasThemeJson(JSON.stringify({}))` (a no-op that resets to the same default light palette), so this is a general regression from the migration, not something unique to wgpu — it's just more visible there. There is a matching closed ticket `.repo/🎫/26/06/07/DAG-AND-FLOW-UI-THEME-SYNC` documenting the original correct behavior.

Additionally, my previous session's fix for the node/port **label overlay** (added to make text appear) used an ad-hoc "box corner + padding" anchor model instead of premigration's exact algorithm from `mathematical/graph/port/directed/dag/react/index.tsx:825-984` (`dagWorldToScreen` direct anchor + `ctx.textAlign` + shrink-to-fit font clamping `dagClampLabelFontPx`/`dagClampPortLabelFontPx`, default alignment `center`, and interaction-chrome-driven fill color via `dagElementInteractionChrome`/`dagOverlayLabelFill` fed by separate hover/selection/preselect state). This causes wrong label position/size and no selection/hover emphasis on labels. Both the wgpu port (`framework/renderer/wgpu/rs/engine_canvas.rs`) and the React overlay (`framework/renderer/react/components/graph-canvas-overlays.tsx`) need to be rewritten to match.

## Fix 1 (critical): sync canvas theme in wgpu — restores edges, channels, node rectangles, LOD chrome

- Add `pub fn set_canvas_theme_dark(&mut self, dark: bool)` to `FlowHost` (`flow/core/rs/lib.rs`, near `set_canvas_theme_from_json` at line 2928) and to `GraphHost` (`framework/graph/rs/lib.rs`, near line 388), each doing `self.dag.canvas_theme = dag::CanvasThemePalette::from_board_theme(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });`. Both crates already depend on `ui_styling` and already publicly re-export `dag` (`pub use mathematical_graph_port_directed_dag as dag;`), so no new Cargo dependencies are needed.
- In `framework/renderer/wgpu/rs/engine_canvas.rs`, add an `is_dark: Option<bool>` field to `NodeGraphSyncCache`, derive dark/light from `ctx.theme.canvas_clear` luminance (same pattern as the existing `board_icon_paint_colors` helper in `mathematical/graph/port/directed/rs/lib.rs:671-676`), and call `.set_canvas_theme_dark(is_dark)` on the Flow/Dag host from `paint_node_graph` (diff-gated so it's cheap every frame) for **both** the `flow` and non-flow (Dag/GraphHost) branches. This fixes every node-graph-based playground (flow, puzzle-style boards, etc.), not just flow.

## Fix 2: restore real theme sync in React (parity + consistency)

- `framework/renderer/react/components/flow-graph-canvas-host.tsx:352-356` currently does `sessionRef.current?.setCanvasThemeJson?.(JSON.stringify({}))`. Replace with `syncSessionCanvasTheme(sessionRef.current)` imported from `@semio-tech/ui-styling` (same call premigration used), matching the existing `useCanvasThemeSync` mutation-observer hook already in place.

## Fix 3: rewrite label overlay to match premigration exactly

Port the exact algorithm from `mathematical/graph/port/directed/dag/react/index.tsx:825-984` (`dagWorldToScreen`, `dagClampLabelFontPx`, `dagClampPortLabelFontPx`, `dagElementInteractionChrome`, `dagOverlayLabelFill`) into both renderers:

- `framework/renderer/react/components/graph-canvas-overlays.tsx`: rewrite `paintDagLabelOverlays`/`parseDagLabelRows` to anchor text directly at the projected world point with `ctx.textAlign` (default `center`, not `left`), shrink-to-fit font sizing bounded by `nodeW`/`nodeH` in screen space, and vertical-layout rotation via `ctx.rotate(-Math.PI/2)` around the anchor (not an offset box corner).
- Thread real interaction state (hoveredNodeId, selectedNodeIds, preselect ids) into `paintDagLabelOverlays`'s call site in `framework/renderer/react/components/flow-graph-canvas-host.tsx` (currently not passed at all) so label color responds to hover/selection like premigration.
- `framework/renderer/wgpu/rs/engine_canvas.rs::paint_label_overlay_row`: replace the box-offset math with direct-anchor + `ctx.textAlign`-equivalent (compute offset from `measure_text_width` for left/right/center, default `center` when `align` is absent) + shrink-to-fit sizing using `ui_wgpu`'s existing `measure_text`. Feed real hover/selected/preselect/dimmed state from the host's existing accessors (`hovered_node_id()`, `selected_node_ids()`, `preselect_widget_ids()`, `preselect_removed_widget_ids()` in `mathematical/graph/port/directed/dag/rs/lib.rs:2153-2292`) into the fill-color resolution.
- Known, explicitly scoped-out limitation: `ui_wgpu`'s `draw_text`/`DrawList::push_glyph` only supports axis-aligned glyph quads (`ui/wgpu/rs/widgets.rs:1548`, `ui/wgpu/rs/draw.rs:352`), so true 90°-rotated vertical labels (used only for IO-widget titles at the default LOD when controls aren't shown) will keep rendering horizontally in wgpu as an interim approximation rather than adding glyph-rotation support to the renderer in this pass.

## Verification (mandatory — no claiming "fixed" without it)

- `cargo test` for `flow_core`, `framework_graph`, `mathematical-graph-port-directed(-dag)`.
- `cargo build -p semio-framework-renderer-wgpu --target wasm32-unknown-unknown` plus the actual dev-server wasm rebuild pipeline (already auto-rebuilds on save per the running `bun run dev:procedural:3d` terminal).
- Reload `http://127.0.0.1:6118/?plugin=flow` (wgpu) in the browser and screenshot: default zoom (expect visible node outlines, edges, port dots/labels), zoomed in to a higher LOD (expect handles), a node selected (red highlight, as before, still correct), and confirm this matches the equivalent React flow screenshot for the same fixture.
- Spot-check one non-flow DAG-based playground (e.g. puzzle 2d) in wgpu to confirm the shared theme-sync fix doesn't regress it.
