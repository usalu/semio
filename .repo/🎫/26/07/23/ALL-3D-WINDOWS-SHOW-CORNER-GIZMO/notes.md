# Notes

## Root cause

`World3dHost` (shared host for every `world-3d` window: puzzle3d, procedural, fem3d, gis3d, lowpoly, shooting, cad play, …) mounts `WorldOrbitGated` + `WorldOrbitViewSnapGateProvider` but never mounts `WorldOrbitViewControls`, so the bottom-right orbit view gizmo is missing.

CAD’s standalone `InteractionSpatialView` already defaults `showOrbitViewGizmo = true`.

## Fix

1. **React** — Mount `WorldOrbitViewControls` inside `World3dHost`’s snap-gate provider; mark hosts with `data-orbit-view-gizmo`.
2. **wgpu** — Paint a screen-space XYZ orientation gizmo in `render_world_3d` using the same bottom-right placement math (`world_orbit_view_gizmo_placement`).

Also removed a duplicate re-export of silhouette helpers in `ui/js/react/index.tsx` that blocked the vitest suite (already `export function`’d earlier in the file).
