# Fix Aggregator Window Right Borders

## Symptom

Active/inactive window silhouette borders look fine on left/top, but the right edge is missing or hair-thin (Perspective pane in Aggregator Top|Perspective layout).

## Cause

`windowSilhouettePath` used a **0.5px** inset for a centered 1px (`--stroke-hairline`) SVG stroke. That puts the stroke flush with `x = width` / `y = height`. Overflow clipping on ancestors uses an **exclusive** max edge, so the right/bottom halves of the stroke are clipped away. Left/top sit on the inclusive min edge and stay visible — matching the asymmetric screenshot.

WGPU already paints the right edge inward (`b.w - stroke`); React SVG did not.

## Fix

- Default path inset → `WINDOW_SILHOUETTE_PATH_INSET = 1` so the full hairline stays inside the stack box.
- Mode dock `ResizablePanel` → `overflow-visible` so residual stroke antialiasing is not scissored by the panel.
