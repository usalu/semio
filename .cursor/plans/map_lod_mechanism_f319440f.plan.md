---
name: Map LOD Mechanism
overview: Fix the map-play browser crash by making the neutral canvas LOD mechanism a compile-time, data-driven list of LODs (id/name/description/threshold) that each canvas content declares, route puzzle 2d's six tiers through it as the example, and give the map its own named tile-zoom bands that bound tile fetching.
todos:
 - id: cavas-lod
   content: Generalize infinite/cavas lod module into compile-time Lod + LodScale; remove DrawLod/LodThresholds/resolve_draw_lod; update crate test
   status: completed
 - id: puzzle2d-lod
   content: Declare PUZZLE_2D_LODS const + LodScale in puzzle 2d rust; keep local DrawLod enum; remove runtime threshold override; expose lod_scale_json
   status: completed
 - id: puzzle2d-react
   content: Update puzzle 2d react/play to source LOD labels/thresholds from compile-time table; drop lodZoomThresholds prop and DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS
   status: completed
 - id: map-rust
   content: Add GIS_MAP_LODS bands + tile-z mapping; replace pick_zoom with band resolution; fix default world camera; add visible_tiles_json/current_lod_json wasm methods
   status: completed
 - id: map-react
   content: Make JS map renderer consume Rust visible_tiles_json; remove duplicated JS tile math; initialize camera from Rust
   status: completed
 - id: tests-build
   content: Update/extend Rust + JS tests, rebuild wasm for both crates, run cargo + vitest, verify map play no longer crashes
   status: completed
isProject: false
---

# Fix Map Play Crash With Compile-Time LOD Mechanism

## Root cause

- `MapCanvas` defaults to `zoom: 390`; `pickTileZoom` returns `round(log2(390)+4)=13` -> `visibleTileKeys` enumerates ~67M tiles -> per-frame `fetch`/`uploadTile` flood crashes the tab. See `gis/map/react/index.tsx` lines 123-162, 270-273.
- JS duplicates Rust tile math (`pickTileZoom`/`visibleTileKeys`/`screenToWorld`) and the two diverge; Rust `tiles::pick_zoom` has the same `+4` miscalibration (`gis/map/rs/lib.rs` lines 55-58).
- LOD is a fixed 6-variant enum in the neutral canvas crate, consumed only by puzzle 2d; the map has no LOD.

## 1. Generalize `infinite/cavas` LOD into a compile-time, data-driven scale

In the `#region Lod` of [infinite/cavas/vello/lib.rs](infinite/cavas/vello/lib.rs) (lines 809-871), replace the puzzle-specific enum with a neutral, compile-time mechanism:

- `pub struct Lod { pub id: &'static str, pub name: &'static str, pub description: &'static str, pub max_zoom: f64 }` (last/finest band uses `f64::INFINITY`).
- `pub struct LodScale { pub lods: &'static [Lod] }` with `resolve_index(zoom)->usize`, `resolve(zoom)->&'static Lod`, `index_of(id)->Option<usize>`.
- Remove `DrawLod`, `LodThresholds`, `resolve_draw_lod` from the crate (those move to puzzle 2d). Update the crate test `lod_thresholds_default` (lines 1051, 1065-1070) to exercise `LodScale::resolve` instead.

## 2. Puzzle 2d declares its six LODs via the mechanism (the example)

In [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs):

- Add `const PUZZLE_2D_LODS: &[Lod; 6]` (minimap, overview, compact, normal, detail, micro) carrying id/name/description and the current default `max_zoom` values (lines 243-247) and a `LodScale` over it.
- Keep the local `DrawLod` enum for the extensive pattern-matching (it now lives in puzzle 2d, not cavas), derived from the resolved LOD `id`/index. `current_draw_lod` (lines 827-834) calls `LodScale::resolve`.
- Per "all LODs declared at compile time", remove the runtime threshold override: delete `set_lod_zoom_thresholds_from_json` (lines 860-885) and its wasm binding `setLodZoomThresholdsJson` (lines 5532-5534). Keep automatic/forced selection (`set_automatic_lod`, `set_forced_draw_lod_label`).
- Add `lod_scale_json()` exposing the compile-time table (id/name/description/max_zoom) so JS can resolve labels from a single source.

In [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx):

- Drop `DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS` + the `lodZoomThresholds` prop/field/effect (lines 1153-1168, 3532-3581, 4823-4834, 9630, 10656, 11107, 11162-11163) and source LOD labels/thresholds from `lod_scale_json()` (replacing `resolvePuzzle2dLodLabelFromThresholds`). Keep `automaticLod`/forced-LOD plumbing.
- Update [puzzle/2d/play/index.ts](puzzle/2d/play/index.ts) `PUZZLE_2D_PLAY_LOD_TIERS` and labels to read from the exposed table.

## 3. Map declares its own fixed LOD bands and becomes tile SSOT

In [gis/map/rs/lib.rs](gis/map/rs/lib.rs):

- Add `const GIS_MAP_LODS: &[Lod]` of named bands (World, Continent, Country, Region, City, District, Street, Building) with name/description and `max_zoom` camera thresholds, plus an index-aligned `const GIS_MAP_LOD_TILE_Z: &[u32]` mapping each band to an OSM tile zoom (tile z is map-specific, kept beside the neutral `Lod`).
- Replace `tiles::pick_zoom` (lines 55-58) with band resolution: `tile_z = GIS_MAP_LOD_TILE_Z[LodScale::resolve_index(camera.zoom)]`, bounding tile counts.
- Fix default camera: in `set_size` (lines 201-206) always fit the world view for the map (do not skip when overlays exist), so the initial frame is a sane whole-world zoom.
- Add `MapSession::visible_tiles_json()` (z/x/y list from host camera+viewport+band) and `MapSession::current_lod_json()` (active id/name/description) wasm methods in the `WasmSession` region.

## 4. JS map renderer consumes Rust tiles (removes duplication/crash surface)

In [gis/map/react/index.tsx](gis/map/react/index.tsx):

- Delete `pickTileZoom`, `visibleTileKeys`, `screenToWorld` (lines 122-170).
- `MapRenderer.refreshTiles` (lines 209-233) calls `session.visibleTilesJson()` for the key list, then fetch+`uploadTile`. Cap concurrent fetches.
- Initialize `MapCanvas` camera from Rust (`current camera`/world-fit) instead of the hardcoded `zoom: 390` (line 273); keep wheel/pan event sync.

## 5. Tests, build, ticket

- Rust: update cavas `lod` test; extend puzzle 2d LOD tests (`overlay_paint_state_json`, forced/automatic) for the table; add gis map tests for band resolution + bounded `visible_tiles_json`.
- JS: keep `gis/map/react` vitest; add a `visible_tiles_json` SSOT assertion; adjust puzzle 2d tests that referenced removed thresholds.
- Rebuild wasm for both crates via existing `script.ts wasm` targets (`gis/map/rs`, `puzzle/2d/rs`); run `nx`/vitest + `cargo test -p gis_map -p puzzle_2d`. Verify map play (`gis/map/play` dev, port 6040) loads without crashing and logs bounded tile counts.
- Do the work inside a repo MCP ticket (read `repo://goals`, reopen/open as appropriate), keeping any temp logs under the ticket folder.

## Notes / decisions

- Removing puzzle 2d runtime threshold reconfiguration is a deliberate consequence of "all LODs declared at compile time"; thresholds now live in the const tables.
- `Lod` stays domain-neutral (no tile_z); map-specific tile mapping is an adjacent const in `gis/map`.
