# Projection pane + gumball stack

## Layout
- Projection pane default anchor: `bottom-right` (was `bottom-middle`)
- Navigation cube (`resolveSceneGizmoViewportPlacement`) sits above folded pane chrome (`h-medium` + spacing gap)
- Unfolded pane grows upward over the cube (DOM PaneHost overlay on canvas)

## Files
- `ui/js/react/index.tsx` — gizmo margin Y lifts above folded chrome
- `framework/renderer/react/index.tsx` — World3dHost projection pane default
- `cad/renderer/js/index.tsx` — CAD projection pane default (same stack)
- `infinite/world/r3f/index.tsx` — docstring
