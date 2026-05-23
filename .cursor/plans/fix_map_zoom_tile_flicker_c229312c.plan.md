---
name: Fix Map Zoom Tile Flicker
overview: "Eliminate GIS map flicker during zoom by adding a tile-pyramid fallback: render retained coarse/fine tiles under the exact-zoom tiles and stop evicting fallback levels when the tile zoom changes."
todos:
  - id: ticket
    content: Open/reopen a repo MCP ticket under the most appropriate goal (read repo://goals first) for the map zoom flicker fix.
    status: cancelled
  - id: pyramid-render
    content: Rewrite append_tiles in gis/map/rs/lib.rs to draw all viewport-intersecting cached tiles sorted coarse-to-fine; add tiles::parse_tile_key and tile_rect_intersects_viewport helpers.
    status: completed
  - id: retention
    content: Update prepare_visible_tiles/retain_tiles_for_keys to retain visible + ancestors + previous-frame visible keys; add last_raster_visible field to MapHost.
    status: completed
  - id: tests
    content: "Extend the existing tests module in lib.rs: parse_tile_key round-trip, ancestor-retained-after-zoom, viewport intersection."
    status: completed
  - id: verify
    content: Build wasm + run cargo tests via the gis/map/play test task, then verify zoom in the dev server shows no flicker; close the ticket with a summary and touched files.
    status: completed
isProject: false
---

# Fix Map Zoom Tile Flicker

## Problem
When zoom crosses a power-of-two boundary, `pick_tile_zoom` jumps to a new tile `z`. The renderer instantly (a) evicts every cached tile outside the new visible set and (b) draws only the new exact-`z` tiles, which haven't been fetched yet (async + 120ms debounce + network). For those frames nothing covers the viewport and `surface_clear` flashes through -> flicker.

All changes are in [gis/map/rs/lib.rs](gis/map/rs/lib.rs). No JS or tile-pipeline changes needed.

## 1. Pyramid render in `append_tiles` (1017-1028)
Replace the exact-`z`-only loop with a pyramid pass:
- Iterate all entries in `self.tile_images`, parse `z/x/y` from the key, compute `projection::tile_world_rect` and cull tiles whose projected screen bbox does not intersect `[0,w] x [0,h]`.
- Collect survivors, sort ascending by tile-`z` (coarse first), then draw each via the existing `tile_raster_affine` + `cavas::raster::draw_image_arc`.

Coarse ancestors paint first and finer tiles (including the exact zoom) paint on top, so every pixel is always covered. No clipping is required: finer tiles overwrite coarser ones and the GPU surface clips overdraw.

Add helpers in the `tiles` module: `parse_tile_key(&str) -> Option<(u32,u32,u32)>` (inverse of `tile_key`), and a `MapHost` method `tile_rect_intersects_viewport(rect) -> bool` using `map_viewport::world_to_screen` on the four corners.

## 2. Keep fallback tiles in `prepare_visible_tiles` (862-877) / `retain_tiles_for_keys` (757-764)
Currently retains only the current visible set, instantly dropping fallbacks. Change the retained key set to the union of:
- current visible keys at `z`
- all ancestor keys of each visible tile down to `z=0` (`(z-1, x>>1, y>>1)`, ...) — guarantees zoom-in fallback
- the previous frame's visible keys — guarantees one-level zoom-out fallback

Add field `last_raster_visible: std::collections::BTreeMap`/`BTreeSet<String>` to `MapHost` (init empty in `Default` at 636-653); set it to the current visible set at the end of `prepare_visible_tiles`.

This union stays well under `MAX_MAP_TILE_CACHE_ENTRIES` (512), so after `retain` the map size never exceeds the cap and the lexicographic `pop_first` LRU never runs (which would otherwise wrongly evict the most-reusable coarse tile `0/0/0`).

## 3. Tests
Extend the existing `#[cfg(test)] mod tests` (1307-1504) in the same file (no new files):
- `parse_tile_key` round-trips `tile_key`.
- After uploading an ancestor tile, setting a deeper-zoom camera, and calling `prepare_visible_tiles`, the ancestor key remains in `host.tile_images` (fallback retained).
- `tile_rect_intersects_viewport` is true for an on-screen tile and false for an off-screen one.

## Scope note
Default play mode is `image` and `combined` also draws raster tiles, so the raster fix covers the reported case. Vector tiles clamp to `z<=5` and transition rarely; left as-is unless flicker is later observed in `vector` mode.

## Verification
- Build wasm + run cargo tests via the existing map task (`gis/map/play` `test`, which runs `cargo test -p gis_map`).
- Run the play dev server (`gis/map/play` `dev`, port 6040) and zoom with the wheel across several LOD boundaries; confirm tiles cross-fade through the pyramid with no background flash. Prefetch tiles first with the `tiles` task if needed.