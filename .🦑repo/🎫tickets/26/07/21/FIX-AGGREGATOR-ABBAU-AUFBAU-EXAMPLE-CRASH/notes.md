# Fix Aggregator Abbau Aufbau Example Crash

## Root cause

`WorldLodGridHelper` sized `GridHelper` divisions as the next power of two of `frustumCoverage / stepWorld` with **no upper bound**.

When the orbit camera grazes the grid plane (common while orbiting Abbau Aufbau / after example reload), the frustum∩plane footprint becomes huge → millions of divisions → Chrome tab OOM (**error code 5**). That also made the default example look missing when the GPU/tab was already dying.

## Fix

- `WORLD_LOD_GRID_MAX_DIVISIONS = 2048`
- `lodGridGeometrySpec` clamps divisions and **coarsens** `stepWorld` so coverage still holds
- Grazing-camera unit test

## Verify

```bash
cd infinite/world/r3f && bun ./script.ts test -t "lodGridGeometrySpec|cameraLodGridLayout"
bun .repo/🎫/26/07/21/FIX-AGGREGATOR-ABBAU-AUFBAU-EXAMPLE-CRASH/verify-runtime.ts
```
