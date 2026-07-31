# Fix Flow Text Appearance Colors

## Problem
Flow node/port labels stayed dark (black) on dark appearance.

## Real root cause
Node names and port labels are painted on an **HTML Canvas 2D overlay**, not the WASM GPU canvas.
`paintDagLabelOverlays` set `ctx.fillStyle = "var(--color-…)"`. Canvas 2D does **not** resolve CSS variables — invalid values are ignored and text paints as `#000000`.

The earlier WASM `syncSessionCanvasTheme` fix only affected GPU-painted text (notes, previews, sliders). Captions are deliberately delegated to the JS overlay.

## Fix
1. Added `dagOverlayLabelFillHex` — resolves overlay fill expressions via `resolveColorHex` with appearance-aware fallbacks before `fillStyle`.
2. `paintDagLabelOverlays` uses the resolved hex.
3. Color resolve cache keys now include appearance; headless `var(--color-foreground)` flips with `html.dark`.

## Validation
- Overlay / styling unit tests pass.
- Canvas2D ignores `var()` fillStyle (validated in ticket log).
