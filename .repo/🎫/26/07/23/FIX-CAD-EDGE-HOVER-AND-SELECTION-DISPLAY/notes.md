# Fix CAD Edge Hover and Selection Display

## Cause
1. CAD play hardcoded `selectionMode: "mesh"` and never emitted `hoveredComponent` / `componentIds`.
2. `setHover` dropped `mode`/`id`; `worldPick` ignored `granularity: "edge"`.
3. World3d edge overlays used face hover fill (`hovered.meshColor`) and depth-tested against coplanar surfaces.
4. Structure-classic beams/columns/walls are **curve-only** centerlines (`indices=[]`, `edgePositions` set). World3d only mesh-picked shaded solids, so centerline clicks always became edge-component picks and never selected the model-definition object.

## Fix
- CAD runtime mirrors lowpoly: `hovered_target`, `component_selection`, `active_object_id`, edge targets enabled.
- World3d: edge hover uses `hovered.lineColor`, base edges use instance `style.lineColor`, overlays `depthTest={false}`; component picks include `objectId`.
- Curve-only meshes pick/hover as **whole instances**; CAD promotes curve-primitive edge picks/hovers to mesh selection; marquee bounds use `edgePositions` when positions are empty; wider line raycast threshold.

## Verify
- `cargo test -p cad-plugin --lib edge_` + `world_pick_selects_visible` + `world_pick_curve_centerline`
- `bun ./script.ts test` in `framework/renderer/react` (edge hover color + curve-only helpers)
