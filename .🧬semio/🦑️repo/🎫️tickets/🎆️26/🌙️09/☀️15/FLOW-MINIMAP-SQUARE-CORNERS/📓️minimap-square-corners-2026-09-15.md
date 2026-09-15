# Flow Minimap Square Corners

## Report

Semio design language forbids rounded corners and drop shadows on UI chrome (`🎨️.css` zeroes shadow tokens and sets `border-radius: 0` on surfaces).

The wgpu flow minimap navigator (shared `DagHost::paint_minimap_widget`, used by procedural 3D flow and generation2d) painted its panel with `RoundedRect` and `MINIMAP_WIDGET_RADIUS` (6px).

## Fix

- Panel fill/stroke now use `Rect` (square corners).
- Removed unused `MINIMAP_WIDGET_RADIUS` token.
- Regression: `minimap_widget_panel_uses_square_corners` asserts the panel encodes as `["r", …]` in the draw list (fill + stroke).

## Out of scope (follow-up)

- DAG node bodies still use rounded rects at normal LOD (separate design decision).
- Residual `rounded-*` / `shadow-*` Tailwind classes in React hosts (e.g. `NodeGraph`, `ShellHost`) — audit separately.
