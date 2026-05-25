# Curve Arc Proper Topology

**Repo MCP:** unavailable in this session.

## Goal

Finalize `curve.arc` as a single circular arc edge (start→end with `curve.kind: arc` + center), not center→start→end polylines. Align sweep sampling with Topologic `Wire.Arc` / interaction preview.

## Summary

- Added `EdgeCurve` (`arc` with `center`) on `EdgeRecord`; arc is one edge from start→end (center is geometry only, not a wire vertex).
- Shared `arcSamplePoints` / `arcPlaneFrame` / `edgeSamplePoints` in core (Topologic-style CCW sweep in the arc plane).
- `curve.arc` kernel command and renderer wireframe tessellate the circular arc instead of chord polylines.
- Fixed `curve-arc.interaction.json` commit `fromStates` to `committed`.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/assets/interactions/curve-arc.interaction.json`
