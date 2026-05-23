# Board canvas interaction stack (ticket notes)

## Rust (`elements_board` host)

- `pointer_down_screen` / `pointer_move_screen` / `pointer_up_screen` drive `Interaction::Pan` on **middle button** (`button == 1`); primary button selects/drags nodes.
- `wheel_screen` updates camera zoom + pan pivot (`set_camera`); emits hover updates when idle.
- No DOM: hit tests are world-space from WASM scene.

## JS (`BoardRenderer`)

- Listeners: `contextmenu` + `pointerleave` + `lostpointercapture` on **event surface**; pointer move/up/down + wheel either on **surface (bubble)** or **`window` (capture)** when `boardRendererUsesWindowPointerCaptureBridge()` is true (playwright/Vite dev), else surface only (Vitest `MODE=test`).
- `wasmSessionCallBlockedForReentry()` short-circuits input while `wasmSessionBorrowDepth` or `wasmGpuFrameDepth` > 0; depths use `try/finally` (no unbounded growth from normal paths).
- `dispose` → `detachCanvasListeners` before `session.free()` so window listeners are not leaked.

## React (`BoardCanvas`)

- `eventSurfaceRef` wraps GPU canvas + text overlay (`pointer-events: none` on overlay).
- `BoardHostSubtree` uses a reconciler rooted on `BoardRenderer` (no extra DOM over the canvas from host markers).

## Memory

- One `BoardRenderer` per pane; each registers/removes its own `window` capture handlers in `attachCanvasListeners` / `detachCanvasListeners`.
- `queueMicrotask` dispose defers teardown to avoid sync re-entry during React commit; listeners are cleared in `dispose` before WASM `free`.

## Debugging

- Canvas exposes `data-board-pointer-bridge` = `window` | `surface` after listeners attach.
