# Root causes

1. `registerPuzzle3dPlaySurfaceHosts` always preloaded **nakagin** scene meshes via `useGLTF.preload`, even for concrete-forest locked builds.
2. Vite static build copied kit GLBs to `dist/meshes/` while runtime requests `/mesh/*.glb` (404 on GitHub Pages).
3. Build copied **all** kit GLBs from metabolism + abbau-aufbau roots regardless of locked fixture.

# Fixes

- Preload uses `brushMeshUrlsForFillSession` for the active/locked fixture only.
- Build copies GLBs to `dist/mesh/` and filters by `PLAYGROUND_LOCKED_FIXTURE_ID` when set.
