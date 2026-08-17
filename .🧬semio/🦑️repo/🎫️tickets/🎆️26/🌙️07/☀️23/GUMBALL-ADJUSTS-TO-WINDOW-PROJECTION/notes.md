# Gumball Adjusts to Window Projection

## Approach

- `GumballConfig.plane` (`xy` | `yz` | `xz`) filters handles to the planar subset inside `UnifiedGumball`.
- `worldProjectionGumballPlane(spec)` / `orbitCameraViewGumballPlane(view)` map window projections to that plane.
- World host uses `worldGumballConfigForProjection(mode, projectionSpec)`; CAD panes merge `orbitCameraViewGumballPlane(cameraView)` into the active gumball config.

## Planar subset

| Plane | Move | Rotate | Scale |
| --- | --- | --- | --- |
| `xy` (Top/Plan/Bottom) | X, Y, XY | Z | X, Y, XY, uniform |
| `xz` (Front/Back) | X, Z, XZ | Y | X, Z, XZ, uniform |
| `yz` (Left/Right) | Y, Z, YZ | X | Y, Z, YZ, uniform |

Axonometric / multi-point / curvilinear / perspective keep the full 3D gumball.
