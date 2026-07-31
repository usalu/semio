# Puzzle 3D Vortex Show Window Options

## Behavior

- Window option **Vortex Show**: `Always` | `Selected` (default `Selected`).
- **Always**: every object's vortices are emitted into `world3d.vorticesJson`.
- **Selected**: vortices emit only when the parent object is hovered or selected, or when any vortex on that object is hovered/selected (vortex-only pick/connect still works).
- Connect-drag: React host retains the source vortex marker if the parent loses hover mid-drag.

## Root

- Runtime field `vortex_show` on `Puzzle3dRuntime`.
- Filter at emit time in `world_vortices_json` via `puzzle3d_object_vortices_visible`.
- Window measure `puzzle3d-play-vortex-show` → action `setVortexShow`.
