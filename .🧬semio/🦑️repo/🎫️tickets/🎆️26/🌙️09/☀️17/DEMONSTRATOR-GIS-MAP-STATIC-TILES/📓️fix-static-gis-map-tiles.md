# Demonstrator GIS Map Static Tiles

## Symptom

Deployed demonstrator Verfolgen pane: black map, console `GET …/vt/3/1/2.pbf 404`.

## Cause

- GIS map requests `/vt/` and `/osm/` on the static site origin.
- Vite `tile-proxy` middleware only runs under `vite serve` / `preview`.
- Production `build` used `GIS_MAP_TILE_SERVE_MODE=fetch` (default), so `closeBundle` did not copy `.🧬semio/🗺️map` into `dist/site`.
- Local `dist/site` had no `vt/` or `osm/` directories while cache held tiles (e.g. `openfreemap-vt/3/1/2.pbf`).

## Fix

- `demonstratorGisMapTileServeMode`: `bundle` for Vite `build`, `fetch` for dev serve (unless env override).
- `prefetchDemonstratorMapTiles` before site build: prefetch Switzerland through z10 (~400 tiles; not full offline-play z13/z14), fail if canary tiles missing or downloads failed.
- Site build sets `GIS_MAP_TILE_SERVE_MODE=bundle` for the Vite child process.

## Verify

1. `bun nx run @semio-tech/mit-bestand-demonstrator:test`
2. `bun nx run @semio-tech/mit-bestand-demonstrator:build` then confirm `dist/site/vt/3/1/2.pbf` and `dist/site/osm/0/0/0.png` exist.
