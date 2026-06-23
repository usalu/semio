# Puzzle 2d Node Kind Handle Ring T

Fix node-kind handle template angles to use compose connector normalized ring `t` instead of CAD point → angle conversion.

## Summary

- `puzzle2dNodeKindHandlesFromKitConnectors` prefers connector `t` via `puzzle2dHandleAngleFromRingT`.
- Palette seed nodes respect `NodeKind.shape`.
- One-off migration rebakes `puzzle/2d/fixture/nakagin-capsule-tower.2d.json` catalog + instance handle angles from `nakagin-capsule-tower.filtered.kit.compose.json`.

## Files

- `puzzle/2d/react/index.tsx`
- `puzzle/2d/fixture/nakagin-capsule-tower.2d.json`
- `.repo/🎫/26/06/03/PUZZLE-2D-NODE-KIND-HANDLE-RING-T/migrate-nakagin-2d-handle-angles.mts`
