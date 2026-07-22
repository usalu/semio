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
2. Infer rectangle vs lasso when `selectionPreviewMethod` is unavailable (four AA corners → rect, else path → lasso).
3. Expose `selectionPreviewMethod` on flow / node-graph / sequence wasm bindings for an explicit method source after rebuild.
4. Paint marquee at `z-50` above label/pointer overlays.

## Verify

```bash
cd framework/renderer/react && bunx vitest run -t "dag marquee overlay"
```

All 10 overlay tests pass. Rectangle and lasso overlays work without a wasm rebuild.
