# Fix: FEM2D marquee/lasso overlay and selection on drag

## Symptoms

- Drag with Marquee/Lasso only dimmed shell chrome (paint-stroke transient), no rectangle/lasso on canvas.
- Release did not apply selection.

## Root causes

1. **`Canvas2dHost` dispatched `paintStrokeBegin` / `paintStrokeEnd` on every primary-button pointer gesture** — shell dimming unrelated to FEM selection.
2. **Marquee overlay layers used invalid `layersJson` shape** — flat numeric `segments` and CSS color strings; `Canvas2dHost` only draws path overlays via `drawSceneNode` (`kind`/`to` segments + RGBA `fill`/`stroke`).
3. **`sessionFactory` depended on `layersJson`** — each partial refresh during drag recreated the WASM/canvas session and dropped in-flight pointer state before move/up completed.
4. **Partial body re-render could miss gesture overlay** when `view_state.window_id` was unset; overlay lookup now falls back to the window instance for the body key.
5. **Missing `meta:utility` record** — host could not align utility-aware behavior with the active tool.

## Changes

- `📐️Canvas2dHost`: stable session + `syncLayersJson()` on prop updates; removed paint-stroke dispatch.
- `GraphWasmCanvas`: pointer capture for reliable move/up during drag.
- `canvas-gesture`: draw-compatible overlay paths, `fem2d_finish_canvas_layers_json`, meta utility, render window id resolution.
- Model/results render paths thread `active_utility` and use the shared finish helper.

## Verify

- Launch FEM2D editor, Marquee utility: drag shows blue overlay; release selects enclosed entities.
- Lasso: same with freehand path.
- No shell dimming on selection drag.
