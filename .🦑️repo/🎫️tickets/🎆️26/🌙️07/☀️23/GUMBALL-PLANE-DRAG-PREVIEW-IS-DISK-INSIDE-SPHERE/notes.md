# Notes

## Bug

`GumballPlanePreviewMesh` scaled a unit square with `gumballPreviewWorldExtent`, so plane hover/drag preview filled the whole viewport.

## Fix

Replace the viewport-scaled `planeGeometry` with a fixed `circleGeometry` of radius `GUMBALL_RING_RADIUS` (disk inside the gumball sphere). Axis line previews still use viewport extent.

## Verify

```text
bunx vitest run -c vitest.config.ts -t "UnifiedGumball math"
# 1 passed | 327 skipped
```
