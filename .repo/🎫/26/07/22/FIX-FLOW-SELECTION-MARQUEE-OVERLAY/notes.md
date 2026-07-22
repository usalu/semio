# Fix Flow Selection Marquee Overlay

## Root cause

Board rust publishes selection preview points as tuple arrays:

```json
[[x, y], [x, y], ...]
```

(`infinite/board/port/directed/dag` → `selection_preview_points_json`).

`computeDagMarqueeOverlay` only read `{x,y}` objects, so `point.x` / `point.y` were `undefined`, bounds became `NaN`, and `SelectionMarquee` painted an invisible rect. Live selection still worked because hit-testing uses the engine directly.

3D world host does not go through this path — it keeps marquee points in React state as `{x,y}` — so it looked fine.

## Fix

1. Parse both `[[x,y],…]` (rust wire) and `{x,y}` (legacy/tests) in `computeDagMarqueeOverlay`.
2. Wire `selectionPreviewMethod` from flow / node-graph / sequence wasm so lasso draws a polygon, not a bounding rect.
3. Paint marquee at `z-50` above label/pointer overlays.

## Verify

```bash
cd framework/renderer/react && bunx vitest run -t "dag marquee overlay"
```

Rebuild wasm for lasso method (optional for rectangle; method falls back to `"rectangle"`):

```bash
cd flow/core && bun ./script.ts wasm
cd framework/surface/node-graph/rs && bun ./script.ts wasm
cd sequence/core && bun ./script.ts wasm
```
