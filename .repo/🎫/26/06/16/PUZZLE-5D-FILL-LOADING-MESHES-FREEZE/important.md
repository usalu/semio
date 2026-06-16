# Root cause

`FiveD3d` wrapped the R3F canvas in an outer `Suspense` with a "Loading meshes…" fallback. When fill activates, `Puzzle3dFillMeshBridge` loads additional catalog GLBs via `useGLTF`, which suspends. The outer boundary replaces the entire subtree (including the canvas) with the fallback, unmounting the loaders and deadlocking fill prep.

Puzzle 3d has no outer Suspense and preloads fill catalog URLs — so fill works there.

# Fix

1. Remove outer Suspense from `FiveD3d` (match puzzle 3d).
2. Preload fill catalog mesh URLs when fill tool activates in `Puzzle5d3dSurfaceHost`.
3. Defer fill sequence build with captured base model (same pattern as puzzle 3d play).
