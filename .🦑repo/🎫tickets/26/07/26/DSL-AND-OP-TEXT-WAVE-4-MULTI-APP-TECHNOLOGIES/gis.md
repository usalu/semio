# gis (Wave 4) — scratch notes

## Cross-reference check
grep across whole repo (excluding node_modules/target/.repo/.claude) for the two JSON fixture filenames:
- `reuse.map.gis.json`: only hit outside gis/ is `.cursor/plans/gis-map-reuse-pins_a017207e.plan.md` (a plan doc, not code/include_str) — safe to delete.
- `reuse.terrain.gis.json`: no hits outside gis/ at all — safe to delete.
Both JSON files deleted after conversion.

## Design
- `gis/plugin/rs/lib.rs` domain module: added `//#region 🔖Dsl` + `//#region 🔖OpText` right after each
  app's existing `//#region 🔖DocumentVcs` (map's around old line ~327, terrain's around old line ~870).
- GisMapDocument (`.gismap`): `MapFeature::data` is opaque `serde_json::Value` -> reused the
  raster_text/mathematical_text/shooting Word/Str/`{key=value}`/`[ ]` lexer convention (NOT mindmap's
  colon-JSON grammar) for consistency with other plugin crates. Grammar: `position "<id>" <value>` /
  `route ...` / `region ...`, one per line, value = generic object/array/string/number/bool/null literal.
  Ops: `add-position <index> "<id>" <value>`, `remove-position "<id>"`, `move-position "<id>" <to_index>`,
  `patch-position "<id>" <value|->`, same for `-route`/`-region`, plus `set-document <feature-lines>`.
- Gis3dTerrainDocument (`.gisterrain`): trivial single field (`exaggeration: f64`). DSL = one line
  `gisterrain exaggeration=<number>`; op = `set-exaggeration exaggeration=<number>`.
- Tension resolved: the OLD `reuse.terrain.gis.json` fed BOTH the document's exaggeration seed AND
  `app_3d`'s rendering-only scenery (project origin + pins), but `Gis3dTerrainDocument` only ever
  carries `exaggeration`. Fix: the new `.gisterrain` fixture file has an extra `origin lon=.. lat=..`
  line and `position id=.. lon=.. lat=.. label=".." icon=..` lines AFTER the `gisterrain exaggeration=..`
  header. `Gis3dTerrainDocument::parse_dsl` (domain::gis_terrain_text) reads ONLY the first line and
  ignores the rest. A new `app_3d::terrain_fixture_text` module (hand-rolled, separate from the vcs
  DocumentDsl trait — TerrainDescriptorJson/TerrainProjectOrigin/TerrainPositionData are foreign types
  from framework_surface_terrain, so this could never be a DocumentDsl impl anyway) reads the
  `origin`/`position` lines for rendering. Both parsers read the SAME single converted file — still only
  one fixture file per app, per the "no new example files" constraint.
- Renamed `REUSE_MAP_EXAMPLE_JSON`/`REUSE_TERRAIN_EXAMPLE_JSON` -> `..._TEXT`; runtime field
  `terrain_fixture_json` -> `terrain_fixture_text` (greenfield rename, no back-compat needed).
- `gis_map_document_from_descriptor_json`/`gis_map_descriptor_json` (JSON <-> GisMapDocument) are
  UNTOUCHED — still used for `patchPositions` UI-JSON-payload ingestion, DWG import, and MapHost
  scene sync (framework_surface_tiled_map's wire format), unrelated to fixture loading.

## Fixture conversion
One-off Python script `convert_gis_fixtures.py` in this ticket folder (implements the same
quote()/print_value() grammar as the Rust printer) converted:
- `gis/2d/example/reuse.map.gis.json` (152 positions, 149 routes, 0 regions) -> `reuse.map.gismap`
- `gis/3d/example/reuse.terrain.gis.json` -> `reuse.terrain.gisterrain`
Old JSON files deleted. Script kept here as scratch, not part of the build.

## Status
- Code changes complete.
- First `cargo check -p gis-plugin --lib` attempt used `CARGO_TARGET_DIR=.../scratchpad/gis-cargo-target`
  and failed/was interrupted — a sibling agent flagged that multiple parallel wave-4 agents pointing
  CARGO_TARGET_DIR at generic-looking paths can lock each other out. Switched to a tech-qualified unique
  path: `CARGO_TARGET_DIR=/private/tmp/claude-501-gis-wave4-cargo-target`. Retrying `cargo check` there —
  see next update for pass/fail + `cargo test` + wasm32 check results.
