# Axis colors for gumball + projection gizmo

## Rule
Gumball and view/projection gizmos are exceptions to active/hover chrome:
- **X** → primary
- **Y** → secondary
- **Z** → tertiary

Hover/active only change opacity or size — never swap to emphasized/muted semantics for axis-related parts.

## Changes
- `ui/styling`: `SPATIAL_AXIS_COLOR_REFS` + `resolveSpatialAxisColors()`
- Gumball palette reads those tokens
- World projection gizmo: per-axis shafts/faces; corners/center stay muted
- Scene Gizmo (Z-up label remap): CAD X/Y/Z → primary/secondary/tertiary
- wgpu orbit gizmo: same primary/secondary/tertiary paints
