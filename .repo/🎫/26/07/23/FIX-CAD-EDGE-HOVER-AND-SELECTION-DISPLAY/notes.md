# Fix CAD Edge Hover and Selection Display

## Cause
- CAD play hardcoded `selectionMode: "mesh"` and never emitted `hoveredComponent` / `componentIds`.
- `setHover` dropped `mode`/`id`; `worldPick` ignored `granularity: "edge"`.
- World3d edge overlays used face hover fill (`hovered.meshColor`) and depth-tested against coplanar surfaces.

## Fix
- CAD runtime mirrors lowpoly: `hovered_target`, `component_selection`, `active_object_id`, edge targets enabled.
- World3d: edge hover uses `hovered.lineColor`, base edges use instance `style.lineColor`, overlays `depthTest={false}`; component picks include `objectId`.

## Verify
- `cargo test -p cad-plugin --lib edge_` + `world_pick_selects_visible`
- `bun ./script.ts test` in `framework/renderer/react` (edge hover color unit)
