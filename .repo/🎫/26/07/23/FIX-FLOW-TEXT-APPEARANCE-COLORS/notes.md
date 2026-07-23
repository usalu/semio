# Fix Flow Text Appearance Colors

## Problem
Flow text stayed dark on dark appearance (hard to read).

## Root cause
WASM canvas hosts did not reliably push the active board palette into sessions:

1. `FlowGraphCanvasHost` synced theme on appearance mutation but not after `attachCanvas`, so the first sync often ran while `sessionRef` was still null.
2. `WasmEditorSurface` (DSL / generate preview) never synced theme — editor defaulted to `BOARD_LIGHT` dark `label_fill`.
3. `WasmGraphSurface` had the same missing sync.
4. Declarative `interpretUiNode` `text` nodes lacked `text-foreground`.

## Fix
Align with puzzle/note hosts: `syncSessionCanvasTheme` on session ready/attach + `useCanvasAppearanceSync` → sync + `renderFrame()` (+ overlays for graph hosts). Declarative text uses `text-foreground`.
