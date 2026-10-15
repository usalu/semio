# Result: tiled-map surface split, `image` runtime removal, GIS grouping

Crate `semio-framework-surface`. Root `TM/` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/` (was one file, 4187 lines).
Tools and scripts kept in `🪓️monolith-split/tiled-map-tools/` (`split.py`, `widen.py`, `exclude.json`, `widened.json`, `oracle.sh`, `cmp.py`, `lossless.py`, `policy-reader-check.ts`). Logs in `🗑️generated/tiledmap-*`.

## Status in one paragraph

Task 1 (split), task 2 (runtime `image` removed, oracle test added) and task 3 (GIS grouped, follow-up recorded) are done. **Compile is blocked, not passed**: `bun repo cargo ... check -p semio-framework-surface` was run and refused by the source-boundary gate on the dependency `semio-framework-canvas` (`Rust crate Source boundary exceeded before compiler byte allocation`, log `tiledmap-cargo-check.log`). No `cargo check` or `cargo test` of this crate has passed, and no unit test (including the new oracle test) has been run. What passed: the full-resolution rustc differential in all four configurations, losslessness, rustfmt parsing, path and region checks, the readers check, lock refresh and `metadata --locked`.

## Task 1: the split

Pure structural. No logic edit except the PNG decode swap of task 2. All public paths under `tiled_map::` are unchanged (explicit `pub mod` for the three former inline modules, `pub use <child>::*` for the rest, `pub use wasm_session::MapSession`). Mount order in the root matters: `map_host` is mounted before `interaction_and_labels` (see readers).

| File (under `TM/`) | Lines | Active | Content |
|---|---|---|---|
| `🦀️.rs` (root) | 277 | 204 | crate doc, `MapPalette`, neutral camera/zoom clamps, `map_viewport`, FNV tile-revision helpers, neutral consts, mounts, errors, tests mount |
| `🧭️projection/` | 50 | 35 | former inline `pub mod projection` |
| `🧩️tiles/` | 202 | 167 | former inline `pub mod tiles` |
| `🧱️vector-tiles/` | 12 | 5 | facade: `pub use mvt_decoder::*; pub use super::gis_policy::vector_style::*;` |
| `🧱️vector-tiles/📦️mvt-decoder/` | 592 | 527 | MVT protobuf decoder, `decode_mvt`, property helpers, tile-edge geometry helpers; the `include!` of the geometry tests moved here (path now `../../🧪️tests/...`) |
| `🪢️map-state/` | 306 | 255 | `MapExtension`, descriptors, tile mode, layer weights/visibility, `MapFeatureTables`, `MapTileLedger` |
| `🏛️map-host/` | 201 | 156 | `MapHost`, `CachedVectorScene`, `MapHostRetirement`, `enum MapInteraction`, `Default for MapHost` |
| `🏛️map-host/📷️camera-and-picking/` | 753 | 661 | `impl MapHost` part A (camera, queries, uploads, pointer, picking) |
| `🏛️map-host/🖼️host-scene/` | 855 | 773 | `impl MapHost` part B (tile geometry, painting, scene build), `CanvasContent` impl, `VectorTileCoordinates`, `MapFigurePalette` |
| `🖱️interaction-and-labels/` | 228 | 205 | interaction plan, hit-test geometry, `LabelDeclutter` |
| `🌐️wasm-session/` | 299 | 237 | browser `MapSession` bridge (every item keeps its `cfg`) |
| `🌍️gis-policy/` | 198 | 157 | GIS grouping, task 3 |
| `🌍️gis-policy/🎨️vector-style/` | 322 | 272 | GIS grouping, task 3 (plus `🧪️tests/🔬️unit/🦀️.rs`, 23 lines) |

Every child is at most 773 active lines (limit about 2500). Longest path in the folder: 137 bytes (limit 240). All new folder emojis were checked as already used as folder prefixes elsewhere in the repo and are unique among siblings; every new file starts with an emoji `//!` doc whose emoji is unique within this split.

### Deviations from `SEAM-MAP-HUB-REPO-SURFACE.md` section D

- `vector_tiles` is a facade over a neutral `📦️mvt-decoder` plus the GIS `vector-style`, but the style child is physically and logically under `🌍️gis-policy/` (it is mounted by `gis_policy` as `pub(super) mod vector_style` and re-exported by the facade). The neutral helpers that sat inside the style region (`property_str/class/u64/flag`, `feature_label`, `feature_label_anchor`, the `mvt_*` tile-edge helpers) went to the decoder instead.
- `impl Default for MapHost` sits in `map-host` (not in the interaction child) because it names private fields.
- `VectorTileCoordinates` and `MapFigurePalette` sit in `host-scene` (their only user), not in the interaction child.
- `FrameworkSurfaceTiledMapError`, `map_viewport`, the camera clamps and the tests mount stay in the root.
- Two unit tests moved: `continent_water_filter_drops_inland_polygons` and `weighted_opaque_fill_keeps_alpha_solid` left `🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs` (they test GIS style items, which are no longer in the decoder module) for the new `🌍️gis-policy/🎨️vector-style/🧪️tests/🔬️unit/🦀️.rs`, mounted from the style child. Bodies unchanged except `super::` now resolves to `vector_style`.
- `pub use canvas::{self, *};` is untouched (explicit re-export is a separate follow-up, not done here).

### Visibility widenings (45 members, list in `tiled-map-tools/widened.json`)

Rule: a private member defined in a child and used outside it gets `pub(super)` (or `pub(in crate::tiled_map)` when the users are outside the parent's subtree). Decided by a name-based definition scan (`widen.py`) over all tree files plus the unit tests; fields were decided by `.field` access evidence, then hand-reviewed (false positives such as same-named fields of other structs excluded in `exclude.json`). Widened: `gis_policy` 8 (four LOD consts, three resolvers, `pick_tile_z_target`, `active_map_lod`), `map_state` 8 (`MapTileLedger`, its five fields, `touch_raster/vector`), `map_host` 10 (nine `MapHost` fields used by the unit tests, `MapInteraction`), `camera_and_picking` 2 (`retain_*_for_keys`), `host_scene` 2 (`tile_rect_intersects_viewport`, `tile_raster_affine`, both used by tests), `interaction_and_labels` 15 (six `MapInteractionPlan` fields, six helper fns, `LabelDeclutter` plus two methods). A re-run of `widen.py` after the last edit reports 0 further candidates. Methods and fields are checked by name, not by the compiler (see Limits).

## Task 2: removing the runtime `image` dependency

- Replaced: `upload_tile` (`📷️camera-and-picking`) now calls `semio_framework_pixels::decode_png` and builds `RasterImage::rgba8(decoded.width, decoded.height, Arc::new(decoded.pixels))`; `impl From<image::ImageError>` became `impl From<semio_framework_pixels::RasterError>` (error enum shape and message prefix source unchanged: `Image(String)`).
- The same crate's `🎨️paint/🦀️.rs` also used `image` at runtime (`upload_layer_image`, `upload_raster_image_key`, one `From` impl). Without fixing it `image` could not leave `[dependencies]`, so it got the same swap (decoded `RasterImage` is inserted as is). Paint had not been touched since Oct 8, no peer edit conflict.
- `Cargo.toml` of `📦️packages/🦀️rust`: `image` moved from `[dependencies]` to `[dev-dependencies]` (same version and features), comment updated. Remaining `image::` uses in the crate are test code only (tiled-map tests, paint tests).
- Oracle test added in `🧪️tests/🔬️unit/🦀️.rs`: `upload_tile_png_decode_matches_image_crate_oracle_for_every_8_bit_color_type`. It encodes a 19x13 image as Rgba8, Rgb8, L8 and La8 PNG with the `image` encoder, then asserts that `semio_framework_pixels::decode_png` equals `image::load_from_memory(..).to_rgba8()` (extent and every pixel), and that `MapHost::upload_tile` stores the same extent. **Written, not run** (compile gate). 16-bit and palette PNGs are deliberately not in the oracle: the owned decoder maps 16 to 8 bit by `>> 8`, which is not verified to equal the `image` crate's rounding.
- Behavior notes: `decode_png` caps input at 64 MiB and 16,777,216 pixels (`image` had no such cap); only PNG is accepted, as before (the `image` feature set was `png` only).
- Debt records: the `productionDebt` entries of the stdio oracle registries (`bmp`, `jpg`, `tiff`, hub `compositions/stdio`, 8 entries) listed `tiled-map/🦀️.rs` or the surface `Cargo.toml` as production-reachable `image` users. Those paths and the plan texts were shrunk; owner now the animate plugin (or intrinsic-size for the Cargo.toml record). JSON validated by parse. `🔒️dependencies.json` (freeze baseline) was **not** regenerated: its ratchet only fails on new dependencies, and none were added.

### `verify dependencies literal-external` before/after (each about 10 minutes, 900 s cap)

| | Before | After |
|---|---|---|
| rust raw / literal-external | 183 | 183 |
| all-ecosystem literal-external | 2543 | 2543 |
| oracle conflicts | 3 | 3 |
| gate | red (target 0) | red (target 0) |

The delta is one manifest: the surface `Cargo.toml` no longer appears among the declarers in the `oracle-conflict rust:image` line (the other change in that line is an unrelated peer manifest, `repo/🔗️graphql/🗄️fs-context`). The crate-level counts do not move because `image` stays a literal-external crate through other runtime declarers (canvas, intrinsic-size, os infinite, wgpu renderer) and `serde`/`serde_json` remain runtime dependencies of this crate (not touched). Logs: `tiledmap-verify-before.log`, `tiledmap-verify-after.log`.

## Task 3: GIS content in a framework module (grouped, not relocated)

Everything GIS-specific that is cohesive now lives under `🌍️gis-policy/`:

- `🌍️gis-policy/🦀️.rs`: `GIS_MAP_LODS`, `GIS_MAP_LOD_TILE_Z`, `GIS_MAP_LOD_MAX_SPAN_DEG`, `GIS_MAP_LOD_SCALE`, `GIS_MAP_LOD_MODE_AUTOMATIC`, `gis_map_lod_scale_json`, `gis_map_camera_limits_json`, `gis_map_camera_limits_json_for_viewport`, `gis_map_layer_weight_slider_ids_json`, `map_layer_weight_slider_keys_at_lod`, `resolve_map_lod_index_from_span`, `map_lod_band`, `resolve_detail_lod_index`, `representative_viewport_span_for_lod`, `current_map_lod`, `forced_lod_tile_z`, `pick_tile_z_target`, `active_map_lod`, `vector_tiles_available_at_camera_zoom`.
- `🌍️gis-policy/🎨️vector-style/🦀️.rs`: `layer_draw_rank`, `VectorDetailProfile`, `max_tile_z_for_span`, `vector_detail_profile`, `boundary_visible`, `transportation_visible`, `transportation_name_visible`, `place_label_visible`, `poi_label_visible`, `place_label_rank`, `transportation_name_rank`, `water_polygon_visible_for_lod`, `waterway_visible_for_lod`, `country_polygon_holes_visible_for_lod`, stroke-width and line-scale rules, label px bands, `color_with_alpha`, `weighted_opaque_fill`.

### What the follow-up must do

Move the whole `🌍️gis-policy/` folder (both files and the style unit tests) into the GIS plugin's map-host code (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🗺️maphost/`). Its only external consumers are inside that plugin: `…/🪟️windows/🗺️map/☑️options/🔽️lod/🦀️.rs` (+ its test), `…/☑️options/📏️layer-weights/🦀️.rs`, `…/🎚️config/🦀️.rs` (+ test) and `…/✏️editor/🦀️.rs` (symbols `gis_map_lod_scale_json`, `gis_map_layer_weight_slider_ids_json`, `GIS_MAP_LOD_MODE_AUTOMATIC`); plus the framework tests `🧪️tests/🔬️unit`, `🕸️web-mercator-tile-oracle` and `🧪️tests/🗺️tiled-map-mercator-oracle`, which read `map_lod_band` and must move or be re-pointed with it. After the move the `gis_map_*` names are correct in the plugin; the framework side gets neutral names.

Neutral extension point the framework should expose (extend the existing `MapExtension` trait in `🪢️map-state`, which already is the domain-specific extension seam and has `DefaultMapExtension`):

- `MapExtension::policy(&self) -> &dyn MapPolicy`, and `MapHost::with_extension(Arc<dyn MapExtension>)` (today `MapHost` holds no extension).
- `MapPolicy` (neutral): `bands() -> &[MapLodBand]` (id, name, description, max zoom, max span deg, raster tile z), `band_for_span(span_deg, pinned_id)`, `tile_z_target(view_facts, pinned_id)`, `vector_zoom_cap(span_deg)`, `layer_gates(band, span_deg, tile_z) -> MapLayerGates` (neutral booleans keyed by layer key), `draw_rank(layer)`, `label_gate_and_rank(layer, class, span_deg)`, `stroke_width(layer, class, line_scale)`, `slider_keys(band, MapTileMode)`.
- The framework keeps a neutral default (one band, everything visible) so the crate works without a plugin; JSON surfaces become `map_lod_scale_json(policy)` style functions.

Remaining GIS leaks outside `gis-policy` that the same follow-up must also resolve (left in place on purpose):

- root: `MAP_RASTER_TILE_Z_MAX = 19` (OSM raster pyramid), `MAX_MAP_TILE_CACHE_ENTRIES`; crate doc naming `GismapSnapshot` and `crate::artifacts::gismap` (doc only).
- decoder: `MAP_VECTOR_TILE_MAX_Z = 14` (OpenFreeMap), `property_class` ("class" key), `feature_label` key list (`name:en`, ...).
- `map-state`: `DefaultMapExtension::extension_id() == "gis.map/default"`, `MapLayerStrokeScale` and `MapLayerVisibility` field vocabulary (raster, water, land, roads, buildings, borders, labels, positions, position_labels, routes, regions), `MapFeatureTables` (positions/routes/regions) and `MapVectorStyle` presets.
- `host-scene`: `match` arms on OpenMapTiles layer names (`water`, `landcover`, `landuse`, `park`, `transportation`, `building`, `boundary`, `waterway`, `aeroway`, `place`, `poi`, `water_name`, `transportation_name`, `countries`, `geolines`, `centroids`); should become `MapPolicy` paint roles.
- `camera-and-picking`: `pick_raster_tile_zoom`, `pick_vector_tile_zoom`, `set_lod_mode`, `current_lod_json` call the policy functions directly.

## Verification (commands run and results)

1. Baseline before the first edit: `oracle.sh` on a pristine source copy, `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_framework_surface --emit=metadata -o /dev/null [--test|--target wasm32-unknown-unknown|--target wasm32-wasip2] <real lib root>`. Errors: native 691, `--test` 876, wasm32-unknown-unknown 824, wasm32-wasip2 691 (external-crate resolution only).
2. After the split (final tree): native 689, test 878, wasm32 822, wasip2 689. `cmp.py` (multiset of error head plus primary label, line numbers and files stripped) shows exactly one class of difference per configuration: `image` unresolved-crate errors removed, `semio_framework_pixels` unresolved-crate errors added (the intentional task 2 edit; `--test` also gains one `unresolved import image` from the new test). **No new E0412, E0425, E0422, E0423, E0603, E0616, E0624 or E0432.** (The first runs on the unwidened split showed E0425/E0433/E0422 missing-name errors (private child items) and a decode-geometry test mismatch; both fixed, see widenings and moved tests.) Limit: with external crates unresolved, typeck does not run, so field and method privacy are covered by the definition scan only.
3. Definition scan (`widen.py`, dry run on the final tree): 0 candidates left.
4. Losslessness (`lossless.py`, line multiset, whitespace-insensitive, tests excluded): every original line is present in the new tree except the 3 inline `pub mod` openers, 2 net closing braces (3 inline closers minus 1 new `impl` closer), the 45 widened declarations (present with a `pub(...)` prefix), the `include!` path, and the image lines replaced by task 2. Added lines are only docs, mounts, region markers, `use` glue and the new `impl MapHost {` opener.
5. rustfmt parse (`rustfmt +nightly-2026-07-20 --emit stdout --config skip_children=true,unstable_features=true`) on all 16 Rust files: exit 0 each (also paint).
6. Paths: longest 137 bytes. Regions: balanced in every file. Emojis: first-line emoji unique per file.
7. Readers: `📜️script.ts` `runInteractivityP3mn` now reads the tiled-map module tree (`policyReadModuleTreeSource`); the tiled-map root was added to `POLICY_SPLIT_SOURCE_ROOTS` (so the surface-lifetime self-test's `read(...)` also gets the tree). `tiled-map-tools/policy-reader-check.ts`: 13 files in the tree, the `MapHostRetirement` ... `enum MapInteraction` slice is non-empty (75955..79710) and contains every needle the gate requires. The gate itself (`bun ./📜️script.ts verify interactivity p3mn`) could **not** be run to completion: it fails earlier on a peer breakage (`conditional external module native_socket_sqlite_tests is not canonical` in the wgpu renderer sqlite tests), unrelated to this crate. The oracle declarations that name `tiled-map/🦀️.rs` as a path still point at the unchanged root file.
8. Locks: `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` for the root workspace (twice; each run also added peer-created members, not mine: 7 and then 1 packages) and with `--manifest-path /Users/ueli/Documents/semio/🧰️framework/Cargo.toml` (the surface crate's own workspace: 0 packages locked, existing diffs are peers). `-- metadata --offline --locked --format-version 1 --no-deps` exit 0 for the root workspace and for `🧰️framework/Cargo.toml`. In `🧰️framework/Cargo.lock` the surface package still lists `image` (as dev-dependency) and `semio-framework-pixels`.
9. Real compile: `-- check -p semio-framework-surface --message-format=short` refused by the gate on `semio-framework-canvas`. `test -p` not attempted (same closure).
10. Compiler custody: no scratch cargo workspace, manifest or target dir was created, no `RUSTC` set. The rustc oracle used a source-only copy of the crate under the session scratchpad (output to `/dev/null`); it was deleted after the last run (`scratchpad/sandbox`). No `[DEBUG]` logs were added.

## Limits and risks

- Nothing in this crate has been compiled or tested for real; the new oracle test and the whole split rest on the rustc resolution differential, the definition scan and review.
- Possible real-compile issues the oracle cannot see: a field or method used by name through an unusual path that the scan missed (would surface as E0616/E0624), and glob ambiguity between child re-exports and the `canvas::*` glob (checked by name: none of the 138 child item names exists as a `pub` item in the canvas crate sources).
- `FrameworkSurfaceTiledMapError::Image` message text now comes from `RasterError` (`raster codec error: ...`) instead of the `image` crate wording; no test asserts the old text (checked by grep).

## Hardening (patched oracle)

Wave 2, Sonnet executor, 2026-10-15. Full write-up: `RESULT-HARDEN-WAVE2.md`. No repository source file was edited; no regression was found.

**Method and baseline.** `rustc ... --crate-name semio_framework_surface --emit=metadata -o /dev/null --error-format=json` from `🗺️surface/📦️packages/🦀️rust/🦀️.rs`, sandbox with all module files copied and patched identically (extern crates 0; external globs 2: `pub use canvas::{self, *}` has its `*` dropped, plus the wasm bridge). The lib root has to be copied into the sandbox as well: `📦️packages` is otherwise a symlink and `../../🗺️tiled-map` resolves into the real repo (my first baseline was silently the split tree, module count 29 / 29; the fixed baseline has 16 modules against 29). Baseline = `HEAD` tiled-map root with the deliberate `image` to `semio_framework_pixels` change applied (`From<semio_framework_pixels::RasterError>`, `upload_tile` through `decode_png`, 4,185 lines) and the `HEAD` decode-geometry test file (the two GIS tests the split moved still inside; the split compiles them in `🌍️gis-policy/🎨️vector-style/🧪️tests/🔬️unit`). Lossless: only the three inline `pub mod` openers, two braces and the `include!` path are missing from the split.

| Config | Baseline / split errors | Non-extern | Differing keys |
|---|---|---|---|
| native | 689 / 689 | 265 / 265 | 0 |
| `--test` | 878 / 878 | 308 / 308 | 0 |
| wasm32-unknown-unknown | 820 / 820 | 303 / 303 | 0 |
| wasm32-wasip2 | 689 / 689 | 265 / 265 | 0 |

The earlier "691 vs 689" difference is gone: it was exactly the `image` unresolved-crate lines. Unpatched totals are the same (689, 878): the patch is a no-op here except for the two replaced globs. The canvas glob makes every canvas name unresolved on both sides, so a child without `use super::*` would hide behind it; static check: every child using canvas names starts with `use super::*;` or `use super::{..}`, `📦️mvt-decoder` uses none (its former `use super::Color` lives in the style child), and none of the 111 `pub` names in the tiled-map tree equals one of the 190 `pub` names of the canvas crate (no E0659 candidate).

**Fixes.** None needed.

**Definition scan.** 15 family files: fields 75 private / 20 narrowed, members 55 / 8, top-level items 60 / 18. Uses counted inside the original scope only (`tiles` and `projection` as their own module, `vector_tiles` as the facade + decoder + style + geometry-test group, everything else the whole `tiled_map` subtree). 81 name-collision rows, every one read: `VisibleTileCursor` x/y/z, `RawTile`, `RawLayer`, `RawFeature`, `ProtoCursor::is_empty`, `DecodedGeometry` (all named only in their own file), `MapHostRetirement` (constructed only by its own `new`), `VectorTileCoordinates`, `LabelDeclutter.width/height`, and the two `pub(in crate::tiled_map)` methods used by the unit tests (inside their scope). Real candidates: 0; item scan 0; tuple-constructor scan 0. Seeded: stripping 8 `pub(super)` items in `🌍️gis-policy` adds 7 oracle errors (689 to 696); stripping 3 items and 5 fields in `🪢️map-state` raises the member rows from 81 to 99 and the item scan flags `MapTileLedger`.

**Residual.** Trait-in-scope and typeck-level privacy are not produced by rustc without the linked crates; the `pub use canvas::{self, *}` follow-up in this file's "Task 3" stays. Tools: `harden2-tools/` (`ho.py` target `tm`, `tm-base.rs`, `tm-geom-head.rs`).
