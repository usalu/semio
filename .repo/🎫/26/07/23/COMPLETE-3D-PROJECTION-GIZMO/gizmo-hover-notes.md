# Gizmo Hover Effects

## React (`WorldProjectionGizmoViewport`)

- Lifted hover state to viewport level (gumball-style idle / hover / dimmed)
- Hovered heads brighten axis color or switch neutral to `emphasized`
- Non-hovered heads dim to 40% opacity; hovered scales to 1.1×
- Axis shafts brighten when their face is hovered; corner shafts dim when any head is hovered
- Palette syncs on theme change via `useCanvasAppearanceSync`

## wgpu (`paint_world_orbit_view_gizmo`)

- `gizmo_hovered_tip` on `World3dState`; updated on pointer move in gizmo zone
- Hovered tip paints full alpha + larger radius; others dim when any tip is hovered
- Depth fade preserved for back-facing tips

## Tests

- vitest `projectionGizmoHover` (144 passed total)
- `cargo test world_orbit_view_gizmo` passed
