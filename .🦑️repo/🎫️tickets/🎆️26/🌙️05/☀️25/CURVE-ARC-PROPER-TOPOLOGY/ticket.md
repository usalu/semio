# Curve And Cell Geometry (Topologic Model)

**Repo MCP:** unavailable in this session.

## Goal

Separate topologic entities from OCCT-style geometry: curves on `Edge`, surfaces on `Face`, analytic solids on `Cell` (Topologic / `Geom_Curve` / `Geom_Surface` / `BRepPrimAPI` pattern).

## Summary

- **EdgeCurve:** `line`, `arc`, `circle`, `ellipse`, `nurbs` with tessellation + `edgeCurveLength` / `edgeSamplePoints`.
- **FaceSurface:** `plane`, `cylinder`, `sphere`, `cone`, `nurbs` (schema + types).
- **CellSolid:** `box`, `sphere`, `cylinder`, `cone` on `CellRecord`; `cellSolidAabb`; brepjs `sphere`/`cylinder`/`cone` in kernel.
- **Commands:** `curve.arc`, `curve.circle`, `curve.controlPointCurve`, `curve.interpolateCurve`, `solid.sphere`, `solid.cylinder`, `solid.cone`.
- **Schema:** `spatial/schema/json/topology.json` documents `curve`, `surface`, `solid`.
- Interaction commit `fromStates` fixed to `committed` on curve/solid assets.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/schema/json/topology.json`
- `spatial/assets/interactions/curve-arc.interaction.json`
- `spatial/assets/interactions/curve-circle.interaction.json`
- `spatial/assets/interactions/curve-control-point-curve.interaction.json`
- `spatial/assets/interactions/curve-interpolate-curve.interaction.json`
- `spatial/assets/interactions/solid-sphere.interaction.json`
- `spatial/assets/interactions/solid-cylinder.interaction.json`
