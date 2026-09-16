# Raster window dark appearance

## Symptom

Composite and Navigator window bodies stay on the light checkerboard / cream canvas clear while the shell chrome is in dark appearance.

## Cause

`Paint2dHost` wires `useCanvasAppearanceSync` → `syncSessionCanvasTheme`, but that hook runs on mount before `createRasterSession()` resolves. `onSessionReady` only called `syncAll()` and never pushed the active palette into the wasm session. `Board2dHost` already calls `syncSessionCanvasTheme` after `attach_canvas`.

## Fix

- `onSessionReady`: `syncSessionCanvasTheme(session)` before `syncAll()`.
- `Paint2dWasmCanvas`: `syncSessionCanvasTheme(session)` after successful `attachCanvas`.

## Test

`engine-contract`: `syncs raster canvas theme when the wasm session attaches` — expects `setCanvasThemeJson` at least twice (ready + attach).
