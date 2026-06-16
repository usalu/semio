# Build Tiles Cache Fix

`prefetchMapTiles` now pre-filters jobs against `.repo-cache` before any network fetch.

When tiles exist on disk, they are counted as cached and skipped entirely — no batch loop, no 120ms delays.

Log example:
```
[gis/map/play] prefetch 46839 tiles (raster z0-13, vector z0-14) (37064 cached, 9775 to fetch)
```

When fully cached:
```
[gis/map/play] prefetch 46839 tiles (...) (46839 cached, 0 to fetch)
[gis/map/play] prefetch done: downloaded=0 skipped=46839 failed=0
```

## Deploy fix (2026-06-16)

Production `index.html` must only reference `./assets/index-*.js` and CSS — no `__vite-browser-external-*.js` preload.

Cause: vitest-only `node:fs` wasm init leaked into browser bundles; deployed CDN returned HTML 404 for the missing chunk.

Fix: gate wasm init with `import.meta.vitest` (stripped via `playgroundPlayViteDefine`), remove puzzle-2d `node:fs` from production path, skip palette drag controllers for `PUZZLE_PLAY_ENTRY=map`.


- Total required: 46839 (9521 raster z0-13 + 37318 vector z0-14, Switzerland bounds)
- Cached: 46839 / 46839
- Missing: 0
- Empty files: 0
- Paths: `.repo-cache/osm-tiles`, `.repo-cache/openfreemap-vt`
