# W3 — GIS plugin (✏️s/🔌️plugins/🌍️gis)

## Extra task — A5 relocation (framework terrain → gis)

**Scope conflict found before making changes, resolved by narrowing the move:**

`🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` is NOT exclusively gis-owned code.
It is also **path-mounted directly** (not via crate dependency, to dodge a surface↔infinite Cargo
cycle) into `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📦️glue.rs`
(`#[path = "...🏔️terrain/🦀️component.rs"] pub mod framework_surface_terrain;`), where
`infinite/🦀️component.rs`'s `World3dState` uses `TerrainSessionCore` (+ its internal `tiles`/
`projection`/elevation-decode/mesh-building helpers) to drive the **generic** `World3d` terrain
layer (DEM tile fetch/decode/mesh/render) — this is shared 3D-rendering engine infrastructure, used
by any `World3d` surface, not gis application logic. `♾️infinite` is squarely in the os product tree
and not in my extra-files allowlist — I cannot edit its `glue.rs`, and deleting the physically-shared
source file would have broken its `cargo check` (dangling `#[path]`) with no way for me to fix it.
Wholesale-moving the file into the gis plugin would also have inverted the dependency direction
(generic os infra depending on a specific plugin) — worse than the original violation, not a fix.

**What was actually gis-specific** (confirmed: not referenced anywhere in `infinite`'s glue-mounted
copy) was the `TerrainDescriptor` region — `TerrainProjectOrigin`, `TerrainPositionData`,
`TerrainDescriptorJson`, `GIS_3D_TERRAIN_TILE_URL_TEMPLATE`, `TerrainSceneStyleJson`,
`build_terrain_scene_json` — whose own doc comment already said "the `.gis.json` shape a `gis/3d`
example is authored in ... Consumed by `gis-plugin`'s `app_3d`." Only this region was relocated:

- Created `✏️s/🔌️plugins/🌍️gis/🔨️modules/🏔️terrain/🦀️component.rs` with that region + its 3 tests,
  now depending on `framework_surface::terrain::tiles` (kept in framework, generic zoom bounds) for
  `build_terrain_scene_json`'s `min_zoom`/`max_zoom`.
- Mounted it from gis's own `📦️glue.rs` as `pub mod modules { pub mod terrain; }` (new
  `//#region 🔨️Modules` block, mirrors the plugin's `🔨️modules/*` convention used by `trinity`/
  `flow`/`cad`).
- Removed that region + its 3 tests from framework's `🏔️terrain/🦀️component.rs`; left `Projection`,
  `Tiles`, `ElevationDecode`, `TerrainTileMesh`, `TerrainSession`, `WasmBindings` regions in place —
  `infinite`'s path-mount and its `World3dState` are unaffected (`Serialize`/`Deserialize` imports at
  the top are still used by the remaining `TerrainTileMeshJson`/`CameraRecord`/`VisibleTileRow`).
- Updated gis's own consumers to import from the new local module instead of
  `framework_surface::terrain`:
  - `🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs` — now
    `use crate::modules::terrain::{build_terrain_scene_json, TerrainDescriptorJson};` +
    `use framework_surface::terrain::projection;` (kept — pure geo math, not gis-specific, no DTOs).
  - `🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — now
    `use crate::modules::terrain::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};`

**Cargo.toml / surface glue-lib wiring:** no `terrain` feature flag exists in
`🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` (only `board-2d`/`session-bindgen`
exist, unconditional otherwise) — nothing to remove there. Surface's `📦️glue.rs` keeps
`pub mod terrain;` unconditionally, since the generic session/tile engine content still lives there
and is still needed (by `infinite` and by whatever gis code still calls `framework_surface::terrain::
{tiles, projection, TerrainSessionCore}` directly). Not touched.

Files touched (all inside my scope or the explicit extra-file allowlist):
- `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` (shrunk — explicit extra file)
- `✏️s/🔌️plugins/🌍️gis/🔨️modules/🏔️terrain/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` (added `modules::terrain` mount only)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs` (import switch)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (import switch)

Not touched: `♾️infinite`'s `glue.rs`/`🦀️component.rs` — out of scope, and correctly so (still
path-mounts framework's now-slightly-smaller terrain file; unaffected since it never referenced the
relocated DTOs).

## Step A — Schema self-registration

Framework's parked `catalog-integration` feature (`🧬️schema/🦀️component.rs`
`register_all_plugin_app_schema_descriptors()`, lines ~1469-1507) already expects
`semio_s_plugin_gis::apps::gis2d::config::schema::register_app_schema()` and
`::apps::gis3d::config::schema::register_app_schema()` to exist — they didn't yet (only the
`Gis2dConfig`/`Gis3dConfig` `#[derive(ArtifactSchema)]` structs existed at those files).

Added `register_app_schema()` to both, mirroring the already-open `raster` plugin's pattern exactly:
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` — registers `s.gis.gis2d`
  (config = this file's own 5 leaves, presence = `../../👥️presence/🧬️schema/*`'s 5 leaves), matching
  the closed catalog's `s.gis.gis2d` descriptor block (framework schema file, line ~443) verbatim.
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` — registers `s.gis.gis3d`
  likewise, matching the closed catalog's `s.gis.gis3d` block (line ~460).

Wired both calls into `✏️s/🔌️plugins/🌍️gis/🔧️setup/🦀️component.rs`'s `register_gis_exports()`
(the plugin's existing `.setup(...)` hook, called from `plugin()` in the plugin root
`🦀️component.rs`), alongside the pre-existing `register_pilot_languages()` calls — same
established "call your registration from the plugin's setup hook" pattern other already-open
plugins use (e.g. raster calls it from its artifact engine's `register()`, itself invoked at setup).

Framework's closed catalog function (`register_all_app_schema_descriptors()`, `s.gis.gis2d`/
`s.gis.gis3d` blocks) was NOT touched, per instructions (later wave deletes it).

## Step B — Open contribution producers

`grep -rn "Contribution::" ✏️s/🔌️plugins/🌍️gis/` — no matches. gis's `Cargo.toml` also declares no
`contributes`/`consumes` metadata. No `Contribution::<Variant>` producer sites exist in this plugin
or its extensions — Step B is not applicable; skipped entirely, as instructed.

## Verification

- `cargo check -p semio-framework-surface` — **clean**. Finishes in <1s on rebuild, only
  pre-existing warnings (dead-code lints in unrelated `🎨️paint` code, `block v0.1.6` future-incompat
  notice). No errors.
- `cargo check -p semio-s-plugin-gis` — blocked by the known unrelated concurrent-churn error:
  ```
  error: couldn't read `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/./././../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
     --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs:812:13
  ```
  This is the "document" concept another session is mid-threading through plugins (per the
  operational rules) — confirmed pre-existing and unrelated: `git diff` on `glue.rs` shows my only
  change to that file is the new `modules::terrain` mount block; the `pub mod document;` line at 812
  is untouched by me. Because this is a hard file-read error at module-tree-parsing time, `rustc`
  halts before reaching type-checking, so I could not get a full semantic check of my
  `modules::terrain` wiring or the two `register_app_schema()` additions through `cargo check` on
  this crate — I verified them by careful re-read instead (types/paths/`include_str!` targets
  cross-checked against the file tree and against the already-open `raster` plugin's identical
  pattern). Per the task's own guidance this is exactly the expected/acceptable outcome when the only
  remaining error is the known unrelated churn.

## Files touched (full list)

- `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` (modified — shrunk, extra-file)
- `✏️s/🔌️plugins/🌍️gis/🔨️modules/🏔️terrain/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` (modified)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs` (modified)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (modified)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` (modified — added `register_app_schema()`)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` (modified — added `register_app_schema()`)
- `✏️s/🔌️plugins/🌍️gis/🔧️setup/🦀️component.rs` (modified — wired both `register_app_schema()` calls)
