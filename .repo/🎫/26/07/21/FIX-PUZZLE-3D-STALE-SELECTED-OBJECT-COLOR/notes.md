# Fix Puzzle 3D Stale Selected Object Color

## Symptom
Deselected objects kept the selected mesh color until the mouse hovered them again (same class of stale material update as vortices).

## Fix
- Paint instance `selected`/`hovered` from `selectionJson` (`ids` / `hoveredId`) so mesh paint matches the live selection snapshot.
- `worldMeshMaterialRevision(styleKind)` remounts procedural materials; GLB materials force `needsUpdate` and depend on `revision`.
