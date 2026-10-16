# Result: tiled-map GIS policy relocation (slice B, `relo-tiledmap`)

Executor: slice B of `📓️RELOCATION-BRIEF.md`, 2026-10-15. Tools: `relo-tiledmap-tools/` (`relo.py` differential driver, `typeck_probe.py`). Logs: `🗑️generated/relo-tiledmap-*`. Pristine copies of every edited file: `🗑️generated/relo-tiledmap-baseline/`.

## Verdict

Done, with one placement change. The framework no longer names GIS anywhere in production tiled-map code. The GIS level-of-detail bands, tile pyramid and OpenMapTiles cartography now live in the GIS plugin. They reach the framework through a neutral `MapPolicy` port on `MapExtension`, and the framework ships a neutral default (one band, all layer gates open, no source-layer roles).

- Nothing was compiled or tested for real. The sanctioned launcher failed before reaching the compiler (see Verification 6).
- What did pass:
  - the full-resolution differential for both crates in every configuration;
  - a new analysis-only type-check probe over the whole production tiled-map tree plus the GIS policy, including a seeded type error and seeded privacy regressions;
  - rustfmt, region, path-length and emoji checks;
  - the mercantile Python oracle;
  - third-party JSON Schema validation of the new policy schema and fixtures (Python `jsonschema` and `ajv`).
- While I was verifying, a peer (crate-carving) started splitting the tiled-map tree into producer crates. That carve keeps my design (details under "Concurrent peer carve").
- **Updated after the follow-up (see "Follow-up: remaining GIS leaks"):** all six recorded leaks are resolved. Layer toggles, overlay layers and vector style presets are now declared by the policy, the cartography tokens moved out of `ui_styling`, the ui-scene lane doc is neutral, and the os story paints a small neutral test policy. Nothing beyond the two TiledMapHost vitest suites was executed. The cargo gate still blocks real Rust compilation, and the gismap `Cargo.lock` refresh failed in the launcher. The remaining residuals are listed in the follow-up section.

## Design decisions (owner evidence)

1. **The policy is data that crosses the plugin/renderer boundary, behind a neutral trait.** The brief's design (`MapPolicy` trait, `MapHost::with_extension`) alone would have broken GIS rendering. The hosts that actually draw GIS maps are os-layer renderers that build `MapHost::new()` themselves:
   - the wgpu engine surface: `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach` (`sync_tiled_map_scene`);
   - the React `🧭️TiledMapHost`, through the framework browser `MapSession`.

   These hosts cannot depend on the plugin. Both already feed `TiledMapScene.map_descriptor_json` into `MapHost::sync_map_json`, and that field rides an unbounded scene lane (`🖱️ui/🎬️scene` `TiledMapSceneLane::MapDescriptor`). So:
   - the policy became a schema-first, serialisable `MapPolicyDescriptor`;
   - the plugin ships it as the `policy` member of that descriptor;
   - `sync_map_json` installs it when it differs from the current one.

   No renderer, ui-scene or TS change was needed. `MapPolicy` is still a trait: every rule has a default that reads the descriptor, and an implementation may override any of them (the new test overrides `tile_z_target` and `vector_zoom_cap`).
2. **Placement: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy/`, not `✏️editor/🗺️maphost/`.** The GIS viewer's map window (`👁️viewer/…/🗺️map`) must ship the same policy, and `policyViewerPurityBreaches` forbids `::editor::` under a viewer. The artifact-level `🗺️tiles` module already owns the GIS tile pyramid: its TS twin `🗺️tiles/🟦️.ts` declares `GIS_MAP_OSM_TILE_MAX_Z = 19` and `GIS_MAP_VECTOR_TILE_MAX_Z = 14`, the same two leaks the root and decoder carried. The editor map host and both map windows consume it. Rust path: `crate::tile_coordinates::gis_policy`.
3. **Neutral items stay in the framework under neutral names:**
   - `MAP_LOD_MODE_AUTOMATIC` (MapHost interprets it);
   - camera limits `map_camera_limits_json[_for_viewport]` (pure projection maths);
   - `vector_tiles_available_at_camera_zoom`;
   - the colour helpers `color_with_alpha` and `weighted_opaque_fill`, used by the host painter.

   The `gis_map_*` names exist only in the plugin.
4. **`set_lod_mode` keeps the requested band id even when the current policy does not declare it.** The id pins only while the policy declares it (`MapHost::pinned_band_id`). The React host calls `setLodMode` before `syncMapJson` (boot order), so validating the id at set time would drop a pinned band before its policy arrives. Behaviour change: an unknown id now acts as automatic instead of keeping the previous pin.

## Moved items (from `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🌍️gis-policy/` to `🗺️gismap/🗺️tiles/🌍️gis-policy/`)

| Former item | New home |
|---|---|
| `GIS_MAP_LODS`, `GIS_MAP_LOD_TILE_Z`, `GIS_MAP_LOD_MAX_SPAN_DEG`, `representative_viewport_span_for_lod` | GIS consts `GIS_MAP_LODS` / `GIS_MAP_LOD_TILE_Z` / `GIS_MAP_LOD_MAX_SPAN_DEG` / `GIS_MAP_LOD_REPRESENTATIVE_SPAN_DEG` become `MapLodBand` rows (`gis_map_band`) |
| `GIS_MAP_LOD_SCALE`, `resolve_map_lod_index_from_span`, `resolve_detail_lod_index`, `map_lod_band`, `current_map_lod`, `active_map_lod`, `forced_lod_tile_z` | neutral `MapPolicy::band_for_span(span, pinned)`, `band`, `band_index` |
| `pick_tile_z_target` (world/continent coarse rule) | `MapPolicy::tile_z_target` + GIS `coarse_band_count = 2` |
| `max_tile_z_for_span`, `MAP_VECTOR_TILE_MAX_Z` (decoder), `MAP_RASTER_TILE_Z_MAX` (root) | `vector_zoom_steps` / `vector_tile_z_max = 14` / `raster_tile_z_max = 19` (GIS); `MapPolicy::vector_zoom_cap`, `vector_tile_z_target` |
| `gis_map_lod_scale_json`, `gis_map_layer_weight_slider_ids_json`, `map_layer_weight_slider_keys_at_lod` | GIS `gis_map_lod_scale_json()`, `gis_map_layer_weight_slider_ids_json()` over neutral `map_lod_scale_json(policy)`, `map_layer_weight_slider_ids_json(policy, …)`, `MapPolicy::slider_keys` |
| `gis_map_camera_limits_json[_for_viewport]` | framework root `map_camera_limits_json[_for_viewport]` (neutral) |
| `GIS_MAP_LOD_MODE_AUTOMATIC` | framework `MAP_LOD_MODE_AUTOMATIC` |
| `vector_style::layer_draw_rank` + host match arms on OpenMapTiles layer names | `vector_style::gis_map_source_layers()` rows (name, `MapLayerRole`, draw rank, label rank) |
| `VectorDetailProfile`, `vector_detail_profile[_for_lod]`, `boundary_visible[_for_lod]` | `gis_map_band_gates`, `gis_map_band_buildings`, `GIS_MAP_BOUNDARY_ADMIN_MIN`; `MapPolicy::layer_gates`, `boundary_visible` |
| `transportation_visible`, `transportation_stroke_lod_scale`, `transportation_stroke_width`, `boundary_stroke_width`, `coastline_stroke_width`, `vector_line_scale` | `gis_map_road_spans` / `gis_map_band_road_spans` / `gis_map_band_road_stroke_scale` / `gis_map_road_widths` / `gis_map_boundary_widths` / `gis_map_coastline_width` / `gis_map_line_scale`; `MapPolicy::road_visible`, `stroke_width`, `line_scale` |
| `place_label_visible`, `transportation_name_visible`, `poi_label_visible`, `place_label_rank`, `transportation_name_rank`, `vector_label_px[_for_lod]` | `gis_map_label_rules`, `gis_map_band_label_px`; `MapPolicy::label_gate_and_rank`, `label_px` |
| `water_polygon_visible_for_lod`, `waterway_visible_for_lod`, `country_polygon_holes_visible_for_lod`, colored `lod_idx <= 2` / `== 1` checks | `gis_map_band_water_area_classes` / `_waterways` / `_admin_area_holes` / `_land_wash` / `_land_cover_over_admin_areas` |
| `property_class` ("class"), `feature_label` key list (decoder) | `gis_map_feature_keys()`; decoder `feature_label(properties, keys)` |
| style unit tests (`continent_water_filter_drops_inland_polygons`) | `🗺️tiles/🌍️gis-policy/🎨️vector-style/🧪️tests/🔬️unit/🦀️.rs` (+ draw-order test) |

Float semantics are kept bit-for-bit:
- damping uses `base + slope·t` with explicit `Linear` / `SquareRoot` curves, not `from→to` interpolation;
- the path label limit `caps[6]·0.85` is computed once, the same product as before;
- the road table override in the region band falls back to `0.0` ("never");
- bounds use `max(min).min(max)`, which matches `clamp` on finite values without panicking on bad data.

## Registration point

- **Framework:**
  - `🗺️tiled-map/🎛️map-policy/🦀️.rs`: `MapPolicy` (`descriptor()` plus descriptor-driven rules), `DescribedMapPolicy`, `MapViewFacts`, `MapFeatureFacts`, the JSON surfaces.
  - `🎛️map-policy/📋️descriptor/🦀️.rs`: `MapPolicyDescriptor` + row types + `MapPolicyDescriptor::neutral()`.
  - Schema: `🎛️map-policy/🧬️schema/🔣️.json` (draft-07).
  - Language-agnostic fixture: `🎛️map-policy/🧪️tests/🧫️fixtures/🔣️.json` (the neutral descriptor, a two-band sample policy, and band, tile-z and vector tile-z vectors).
- **`MapExtension` (`🪢️map-state`):**
  - gains `fn policy(&self) -> &dyn MapPolicy`;
  - `DefaultMapExtension` now carries a `DescribedMapPolicy` (`with_policy`), with id `MAP_EXTENSION_DEFAULT_ID = "surface.tiled-map/default"` (was `gis.map/default`);
  - `MapSyncJson` = feature tables (`#[serde(flatten)]`) + optional `policy`.
- **`MapHost`:**
  - new fields `extension: Arc<dyn MapExtension>` and `policy_revision`, also added to the exhaustive `MapHostRetirement` destructure;
  - new methods `with_extension`, `set_extension`, `extension`, `policy`, `pinned_band_id`, `viewport_span_degrees`;
  - `sync_map_json` installs a shipped policy;
  - the vector-scene cache key mixes in `policy_revision`.
- **GIS:**
  - `gis_map_policy_descriptor()`, `gis_map_policy()` (`OnceLock`), `GisMapExtension` (id `gis.map/openmaptiles`), `gis_map_extension()`;
  - `gis_map_scene_descriptor_json(document)` returns the document descriptor + `policy`.
- **Tests of the extension point:**
  - `a_non_default_policy_is_honoured_by_the_map_host` (code-level `FixedTileZoomPolicy`: raster z 3, vector z 2, visible rows all at z 3);
  - `a_policy_shipped_in_the_map_descriptor_replaces_the_neutral_policy` (renderer path, a band pinned before the policy arrives, no reinstall of an equal policy, a malformed policy is a sync error);
  - `a_described_policy_honours_its_band_floors_in_the_host`.

  The peer has since moved these three to `🎛️map-policy/🧪️tests/🏛️host/🦀️.rs`.

  GIS side: `the_scene_descriptor_ships_the_policy_to_a_neutral_host` (a neutral `MapHost` synced with the GIS scene descriptor picks the same raster and vector z as the GIS host) and `the_openmaptiles_paint_roles_come_from_the_gis_policy`.

## Callers, fixtures and readers updated

**Framework:**
- tiled-map root: crate doc made neutral; `MAP_RASTER_TILE_Z_MAX` removed; `ideal_tile_z_for_viewport` takes the policy's raster maximum; new `map_view_facts`.
- `🧩️tiles` (`pick_zoom(camera, viewport, policy, pinned)`).
- `🧱️vector-tiles` facade: GIS re-export removed.
- `📦️mvt-decoder`: `MAP_VECTOR_TILE_MAX_Z` and `property_class` removed; `feature_label` takes keys.
- `🪢️map-state`, `🏛️map-host`.
- `📷️camera-and-picking` (`current_lod_json`, `set_lod_mode`, `pick_raster_tile_zoom` / `pick_vector_tile_zoom`, `focus_feature` limits).
- `🖼️host-scene`: OpenMapTiles match arms replaced by `MapLayerRole` arms; labels, backdrop, widths, gates and positions read the policy.
- `🌐️wasm-session` (`lodScaleJson` / `layerWeightSliderIdsJson` use the session's own policy; `cameraLimitsJson` uses the neutral limits).
- Unit tests `🧪️tests/🔬️unit`:
  - 22 GIS tests moved to the plugin;
  - the rest rewritten against neutral or test policies (`land_cover_extension`, `fixture_band_policy`);
  - new `vector_scene_cache_invalidates_on_policy_change` and `weighted_opaque_fill_keeps_alpha_solid`.
- Web-Mercator case (`🦀️.rs` subject builds a `DescribedMapPolicy` from the fixture's `lodConstants`; `🥒️.feature`, `🐍️.py`, fixture `_comment`, `🔮️oracles/🔣️.json` rationale).
- `🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs` doc.

Pre-existing bugs fixed along the way: the fixture path in `🗺️tiled-map-mercator-oracle/🦀️.rs` and in `🐍️.py` `main()` (both pointed at a nonexistent `🗺️tiled-map/🧫️fixtures`).

**GIS:**
- `📦️packages/🦀️rust/Cargo.toml` (+`ui_styling` path dependency, downward).
- `🗺️tiles/🦀️.rs` (mounts `gis_policy`).
- `🚪️io/📝️text/📸️snapshot` (`gis_map_descriptor_value`, which `gis_map_descriptor_json` now wraps; the I/O descriptor stays policy-free).
- `✏️editor/🗺️maphost` (`MapHost::with_extension(gis_map_extension())`).
- Editor map window and viewer map window (`gis_map_scene_descriptor_json`).
- `☑️options/🔽️lod` + test, `☑️options/📏️layer-weights`, `🎚️config` comment + test.
- `✏️editor/🦀️.rs` (`setLodMode` default).
- `🗺️tiles/🟦️.ts` (comment).
- New GIS tests and fixture `🗺️tiles/🌍️gis-policy/🧪️tests/🧫️fixtures/🔣️.json`: the GIS band vectors, moved from the framework fixture's semantics.

**Readers:**
- `📜️script.ts` `runInteractivityP3mn`: re-checked on the current tree with `tiled-map-tools/policy-reader-check.ts`. The `MapHostRetirement`…`enum MapInteraction` slice is non-empty (10835..14648) and all four needles are present.
- `POLICY_SPLIT_SOURCE_ROOTS` unchanged; the tiled-map root is still registered.
- No TS twin or wasm-bindgen name changed: `lodScaleJson`, `layerWeightSliderIdsJson`, `cameraLimitsJson` and `currentLodJson` keep their JS names, so `WasmSessionLoader` and `TiledMapHost` are untouched.

## GIS leaks listed in RESULT-TILED-MAP "Task 3"

Resolved:
- root `MAP_RASTER_TILE_Z_MAX` and the GIS crate doc;
- decoder `MAP_VECTOR_TILE_MAX_Z`, `property_class`, `feature_label` key list;
- `DefaultMapExtension` id;
- host-scene OpenMapTiles layer-name arms (now `MapLayerRole`);
- camera-and-picking direct policy calls.

`MAX_MAP_TILE_CACHE_ENTRIES` (512) is a neutral cache budget and stays.

Recorded with a plan (each needs edits in peer-owned ui, os renderer or TS-twin files, or a cross-plugin vocabulary rename that cannot be done correctly here):
1. **Host layer vocabulary**: `MapLayerVisibility` / `MapLayerStrokeScale` fields (raster, water, land, roads, buildings, borders, labels, positions, positionLabels, routes, regions) and the matching `slider_keys` strings.
   - Plan: make the toggle keys policy-declared (`MapPolicyDescriptor.layer_toggles: Vec<{key, roles}>`), store visibility and weights as maps keyed by those keys, and move the GIS key list into `gis_map_policy_descriptor`.
   - Files to update in the same pass: GIS `MapWindowConfig`, `GIS_MAP_LAYER_IDS`, ui-scene `tiled_map_default_layer_*_json` (ui crate, peer-owned), the wgpu and React sync paths.
2. **`MapFeatureTables` / `PositionData` / `RouteData` / `RegionData`** (positions, routes, regions; `kind == "donor"` colouring in `append_positions`).
   - Plan: generic overlay layers (`points` / `lines` / `areas` with a style key) declared by the descriptor.
   - Touches the ui-scene doc, `TiledMapHost.parsePositionMeta`, hit-test JSON shapes and the GIS descriptor writer.
3. **`MapVectorStyle` presets** (colored / figureGround / invertedFigure).
   - Plan: policy-declared style presets mapping roles to palette slots.
4. **`ui_styling` `MAP_ROAD_*`, `MAP_BOUNDARY_*`, `MAP_COASTLINE_*`, `MAP_LINE_SCALE_*`, `LABEL_PX_BANDS`** are GIS cartography tokens in the framework ui crate. The GIS policy now reads them through its new `ui_styling` dependency.
   - Plan: move them into the GIS policy as literals, or keep a theme token set the policy names, then drop them from ui-styling (ui crate, peer-owned).
5. **ui-scene doc**: `TiledMapSceneLane::MapDescriptor` still says the lane carries `{ positions, routes, regions }`; it now also carries `policy` (ui crate, peer-owned doc).
6. **The os `🧭️TiledMapHost` Storybook story** sends a descriptor without a policy, so it renders the neutral policy (no vector cartography).
   - Plan: give the story a small test policy descriptor (os level, not GIS).

## Verification (commands and real results)

1. **Patched full-resolution differential.**
   - Command: `HO_WORK=<scratch> python3 relo-tiledmap-tools/relo.py snap <tm|gm> <base|split>` then `cmp`. It reuses `harden2-tools` neutral/ho and runs `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json`.
   - Baseline = the current repository with every slice file restored from the pristine copies, so peer edits elsewhere, such as the surface lib-root crate split, appear on both sides. A patch places deleted files into the sandbox, which `mksbx` alone skipped.
   - Logs: `relo-tiledmap-oracle-surface.log`, `relo-tiledmap-oracle-gismap.log`.

   | Crate / config | Base / split errors | Only-split classes |
   |---|---|---|
   | `semio_framework_surface` native | 401 / 451 | `serde` / `serde_json` unresolved (new descriptor, map-policy, map-state `MapSyncJson`), `Color` / `camera` from the neutralised canvas glob in the new root helpers |
   | `--test` | 478 / 554 | same + `serde_json` in the new map-policy tests, `crate::tiled_map::CanvasExtension` and `super::Color` (canvas glob) |
   | wasm32-unknown-unknown / wasm32-wasip2 | 475 / 525, 401 / 451 | same as native |
   | `semio_s_artifact_gis_gismap` native / `--test` / wasip2 | 2491 / 2512, 4538 / 4571, 2487 / 2508 | `semio_framework_surface`, `serde_json`, `ui_styling` unresolved in the new GIS files only |

   **No new E0412, E0422, E0423, E0425, E0432, E0603, E0616 or E0624 on first-party names.** The internal paths `crate::standards::…::gis_map_descriptor_value` and `crate::tile_coordinates::gis_policy` resolve. "Base-only" rows are the deleted framework `🌍️gis-policy` files.
   - Limit: rustc stops after resolution here, because every external crate is unresolved, so typeck never runs. Hence item 2.
2. **Type and privacy probe.** `relo-tiledmap-tools/typeck_probe.py` copies the whole production tiled-map tree plus the GIS `🌍️gis-policy` (no tests, no browser wasm session), swaps the external crates (canvas, serde, serde_json, ui_styling, value, pixels, the GIS document types) for signature-only stand-ins, and runs `rustc --emit=metadata -o /dev/null` with no artifact. Run on the slice snapshot taken before the peer carve (`TM_SRC=<nsbx-tm-split>`):
   - native: 0 errors; wasm32-wasip2: 0 errors.
   - Seeded type error (`road_lod_scale = band_row.road_spans`): E0277 caught on both targets.
   - Seeded privacy (`gis_map_band_gates` without `pub(super)`, `pinned_band_id` without `pub`): E0603 ×1 + E0624 ×3 on both targets.
   - Log: `relo-tiledmap-typeck-probe.log`. This also serves as the definition scan for field, method and item privacy over the moved code.
3. **rustfmt, regions, paths, emoji.**
   - `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true,unstable_features=true` on all 30 touched Rust files: all parse.
   - `#region` / `#endregion` balanced in every file.
   - Longest touched path 239 bytes (pre-existing lod test file); new paths at most 151 bytes.
   - New folder emojis (🎛️ 📋️ 🧬️ 🧪️ 🔬️ 🧫️ 🌍️ 🎨️) pass the `canonicalTaxonomyEmoji` rule and are unique among their siblings.
4. **Direction.**
   - `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/…/🗺️gismap/🗺️tiles/🟦️.ts --config …/🟨️.cjs --output-type err`: 259 errors after, all transitive through `🖱️ui/🧬️contract` and pre-existing. The only TS edit is a doc comment, so the import graph is unchanged and there is no new violation.
   - Cargo: the only manifest change is the gismap `ui_styling` path dependency (plugin → framework, downward).
   - Rust sources: no `#[path]` / `include*` / `use` from framework into plugin code; the GIS tests `include_bytes!` three framework vector-tile fixtures (plugin → framework, downward).
   - `/usr/bin/grep` of the production tiled-map tree for GIS / OpenMapTiles / OSM vocabulary: nothing left except the host layer key `"water"` (recorded leak 1).
5. **TS / Python.**
   - `.venv/bin/python3 🧰️framework/…/🕸️web-mercator-tile-oracle/🐍️.py` → `PASS: mercantile 1.2.1 agrees with all 47 frozen fixture vectors`.
   - `jsonschema` 4.26.0 `Draft7Validator`: schema valid; `neutral` and `samplePolicy` valid; an empty band table is rejected.
   - `ajv`: same results, plus an unknown top-level key is rejected.
   - `bun ✏️s/…/🗺️tiles/🧪️tests/🟦️.ts` imports cleanly (exit 0; the file only exports its test function).
   - No scoped `tsc` run, because no TS logic changed.
6. **Launcher.** These three each failed with `error: Finite original maintenance authority required` in the cache-maintenance step (`⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:43`), before reaching the compiler or the source-boundary gate:
   - `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-gis-gismap --manifest-path ✏️s/…/🗺️gismap/Cargo.toml --message-format=short`;
   - the same for `-p semio-framework-surface`;
   - `… -- update --workspace --offline --manifest-path ✏️s/…/🗺️gismap/Cargo.toml`.

   Logs: `relo-tiledmap-cargo-check-gismap.log`, `relo-tiledmap-cargo-check-surface.log`, `relo-tiledmap-cargo-update-gismap.log`.
   - **The gismap `Cargo.lock` was not refreshed.** The `semio-s-artifact-gis-gismap` entry lacks `semio-framework-ui-styling` (already in that lock as a transitive package), so `--locked` builds fail until the next refresh.
   - No cargo check or test of either crate has passed. Every new test is written, not run.

## Concurrent peer carve

From 15:37 a peer split the tiled-map tree into producer crates:
- `semio-framework-surface-tiled-map-model` (`🧮️model/`, which mounts my `🎛️map-policy`, `🪢️map-state`, `🧭️projection` and `🧩️tiles` by path, plus `📐️camera` with the root camera and helper functions);
- `semio-framework-surface-tiled-map-mvt` (decoder);
- a host crate manifest under `🗺️tiled-map/📦️packages/🦀️rust`.

The tiled-map root now re-exports the model names, including every new policy name, so GIS consumers keep `semio_framework_surface::tiled_map::{MapPolicy, MapHost, …}`. The peer also moved the host-level policy tests to `🎛️map-policy/🧪️tests/🏛️host/🦀️.rs` and edited the root and host unit tests and the map-policy doc line.

My differential and probe evidence is for the slice state before that carve. Rerunning it on the multi-crate layout needs new crate roots and specs, and belongs to the carving peer. I did not edit any file after the carve started.

## Residual risks and follow-ups

- Real-compile risk:
  - stand-in signatures in the probe can differ from the real canvas / serde_json APIs, although the moved code only uses calls the old code already made;
  - the tests were checked only by the resolution differential.
- **Behaviour change**: an unknown `setLodMode` id now means automatic instead of keeping the previous pin; it pins once a policy declares the band.
- Until a scene descriptor carrying a policy arrives, renderers draw the first frames with the neutral policy (one band, no vector roles). The GIS editor and viewer always ship it.
- The gismap lock needs a refresh; recorded leaks 1–6 stay open.

## Files

Created:
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy/{🦀️.rs, 📋️descriptor/🦀️.rs, 🧬️schema/🔣️.json, 🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🧫️fixtures/🔣️.json}`;
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy/{🦀️.rs, 🎨️vector-style/🦀️.rs, 🎨️vector-style/🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🧫️fixtures/🔣️.json}`;
- `relo-tiledmap-tools/{relo.py, typeck_probe.py}`.

Deleted: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🌍️gis-policy/` (3 files).

Edited:
- framework: tiled-map `🦀️.rs`, `🧩️tiles`, `🧱️vector-tiles`, `📦️mvt-decoder`, `🪢️map-state`, `🏛️map-host` (+ `📷️camera-and-picking`, `🖼️host-scene`), `🌐️wasm-session`, `🧪️tests/🔬️unit`, `🧪️tests/🕸️web-mercator-tile-oracle/{🦀️.rs, 🥒️.feature, 🐍️.py}`, `🧪️tests/🧫️fixtures/🕸️web-mercator-tile-oracle/🔣️.json`, `🔮️oracles/🔣️.json`, `🗺️surface/🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs`;
- GIS: the files listed under "Callers, fixtures and readers updated".

## Follow-up: remaining GIS leaks

Second pass, same day, after the wave-4 SURFACE carve (`semio-framework-surface-tiled-map` host, `-tiled-map-model`, `-tiled-map-mvt`, thin facade) and the UI-SCENE carve (`TiledMapScene` in `🎬️scene/🎬️scenes/🖼️plane/🦀️.rs`). Every edit was placed where the carves put the code. No peer move was reverted. Pristine copies are in `🗑️generated/relo-tiledmap-baseline-fu/`. Tools are `relo-tiledmap-tools/` (`typeck_probe.py` is now multi-crate and has a `PROBE_TESTS=1` mode; `relo.py` has follow-up specs).

### The six leaks

1. **Layer toggles are declared by the policy.**
   - New descriptor types: `MapLayerToggle { key, targets }` and `MapLayerTarget = raster | role | overlay | overlayLabels`.
   - The GIS field vocabulary is gone: `MapLayerVisibility` and `MapLayerStrokeScale` became `MapLayerSwitches { visibility, weights }`, keyed by the policy's toggle keys. A missing key means visible at weight 1.0.
   - `slider_keys` now returns the declared keys whose targets the band/mode paints.
   - GIS: `GIS_MAP_LAYER_KEYS` (11 keys) feed `gis_map_policy_descriptor`. The editor's `GIS_MAP_LAYER_IDS` takes its ids from `GIS_MAP_LAYER_KEYS[i]`, and a new test pins it to the policy's toggle list.
   - `MapWindowConfig` already persists string-keyed maps, so its persisted shape is unchanged. The ui-scene layer JSON defaults were already `"{}"`.
   - wgpu and React feed the same JSON documents through the same host methods.
2. **Overlay layers are declared by the policy.**
   - `MapFeatureTables`, `PositionData`, `RouteData` and `RegionData` are gone.
   - New descriptor types: `MapOverlayLayer` (id, point/line/area geometry, optional pickable `granularity`, pixel dimensions, palette slots, `styleFills`) and `MapOverlayFeature { id, coordinates, label, style, width, icon, link }`.
   - The descriptor is now `{ overlays: { <layer id>: [features] }, policy }`.
   - `kind == "donor"` became a GIS `styleFills` row.
   - Hit, rect, polygon and selection JSON is keyed by pickable layer id, with `kind` set to the granularity. GIS declares overlays `positions`/`routes` with granularities `position`/`route`, so every wire document GIS sees is byte-compatible.
   - GIS writer: `gis_map_overlays_value(document)` and `gis_map_scene_descriptor_json`.
   - Callers updated:
     - editor `map_host_from` syncs `{ overlays }`;
     - the shell command reads `overlay_feature("positions", id).link`;
     - React `TiledMapHost`: `parsePositionMeta` became `parseMapOverlayIndex`, which reads `policy.overlays` and `overlays`. The popup shows for any point layer. Selection, marquee and context-menu groups are driven by the declared layers;
     - wgpu: `parse_map_feature_hit` is flat, context-menu groups come from `pickable_overlays()`, and the dead `map_merge_selection` was removed.
   - The resolve-granularity rule now lives once, in the framework: `MapHost::sync_interaction_json`. The first declared pickable layer with ids wins, then the hover's granularity. The wgpu twin is deleted, and React `resolveMapInteractionSync(selection, hover, layers)` mirrors it. Both have tests.
3. **Vector style presets are declared by the policy.** `MapStylePreset { id, technique: colored|figureGround, ink, paper, labelFill, labelHalo }`. The window's `vector_style` is a preset id. An unknown id paints the first preset. Neutral has one preset, `default`; GIS declares `colored`, `figureGround` and `invertedFigure`. The React `MapVectorStyle` union became `string`.
4. **Cartography tokens moved out of `ui_styling`.**
   - Removed from `🎨️styling/🔣️.json` and `🌓️theme/🔣️.json`: `mapRouteDefault`, `mapRegionMult`, `mapPositionMult`, `mapRoad*` (including `*Mult` and `Lod*`), `mapBoundary*`, `mapCoastline*`, `mapLineScaleCap/Raw`, `mapPositionMarker`, `mapLabelAnchorOffset`, `mapRegion/RouteStrokeAlpha`, `mapRegionFillAlpha`, `metrics.map.labelPxBands` and `lineScaleDamp{Min,Max}`.
   - The generator was run through `generateStylingArtifacts()` and `checkStylingArtifacts()`, both green. I called them directly because the launcher target failed in graph and native-image preflight.
   - The values the GIS cartography uses are now private constants in `🎨️vector-style` (values identical). Overlay dimensions live in the GIS policy.
   - The gismap `ui_styling` dependency is removed.
5. **ui-scene `TiledMapSceneLane` doc** now describes `{ overlays, policy }`. The lane fixture `🚚️tiledmap-scene-lanes` got a neutral sample descriptor with recomputed bytes and hash (67, `b698f1dbf9bd8a82`); the hash function reproduced the old pair 78/`89d18b1418fd712b` first.
6. **Neutral test policy for the os story.** New file `🧭️TiledMapHost/🧪️tests/🧫️fixtures/🗺️overlay-descriptor/🔣️.json`: the neutral bands plus pickable `paths` (line) and `pins` (point), with selection, hover and expectations.
   - The story and the wgpu `wgpu-engine-surfaces` tests both read it. The engine-surfaces fixture keeps only `cameraJson`.
   - The pinch test and repaint fixture are neutral too.

Also: the P3mn needle in the root `📜️script.ts` and its TS self-test now read `self.overlays.is_empty()`. The framework host and model unit tests import `MapPolicy`; the trait-method calls there did not resolve without it, and the probe found this.

### Verification (real results)

- **Typeck probe**, multi-crate, analysis-only:
  - production tree, native and wasip2: 0 errors;
  - `PROBE_TESTS=1` (all framework tiled-map tests plus the GIS policy tests, with fixtures): 0 errors;
  - `--seed` reaches typeck (1 error) and `--seed-privacy` gives 15 privacy errors, in both modes.
  - Not covered: the GIS editor and os renderer files.
- **Full-resolution differential**, base = baseline-fu restored:
  - model, host, gismap, ui-scene and ui-styling: native, test and wasip2 all base == split, with 0 split-only diagnostics. ui-styling compiles clean in both trees.
  - os renderer wgpu: native base == split (6155/6155, 0 split-only). The test config did not finish within 600 s even with `-Zthreads=8` under fleet load, so it was **not run**.
- **rustfmt parse** of all 28 touched Rust files passed. `#region` blocks are balanced in every touched file (the root `📜️script.ts` imbalance of 101/102 was already present at HEAD). The only `[DEBUG]` lines are pre-existing ones in untouched tests, and every path is at most 240 bytes (the longest new one is 195).
- **TypeScript:**
  - scoped `tsc --noEmit`: no errors in any touched file; the remaining errors are in unrelated peer files, plus `WasmSessionLoader` lines 49–50 (an unbuilt `framework-os-node-graph-rs` package, already failing before this pass);
  - vitest for `🧭️TiledMapHost` component and pinch-gesture: 2 files and 26 tests pass;
  - P3mn needles verified against `policyReadRustPolicySource` of the map host. The full lifetime self-test aborts on an unrelated peer source.
- **dependency-cruiser** before/after on the touched TS: 2049 violations both times. The touched-file rows are identical apart from terminal byte mangling, so there is no new violation; the extra module is the story's JSON fixture.
- **JSON Schema** (`jsonschema` 4.25 and `ajv` 8.20):
  - map-policy schema valid;
  - `neutral`, `samplePolicy`, `sampleDescriptor` (as `MapSyncJson`) and the new overlay descriptor are all valid;
  - negative cases are rejected by both validators: bad geometry, bad toggle target, short coordinate, missing `stylePresets`, unknown technique, plus an unknown top-level key (checked with ajv);
  - the premade theme validates against the theme contract with the geometry sub-schema.
- **Leak sweep:** no GIS, OSM, OpenMapTiles, donor or positions/routes/regions vocabulary remains in production code of the framework tiled-map, React `TiledMapHost` or the wgpu map paths.
- **Not run:** any Rust `cargo` build or test (compiler ingress gate); the GIS editor and os Rust edits were checked only by resolution-level differential and parse.
- **Gismap `Cargo.lock`:** `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline --manifest-path …/gismap/Cargo.toml` failed in the launcher with "Finite original maintenance authority required". The lock's gismap block still lists `semio-framework-ui-styling`, and the next unlocked cargo run will drop it.

### Residuals

- `MapPaletteSlot` names mirror the `MapPalette` fields, which are a theme contract shared with the os renderers and `ui_styling::MapPalette`: `regionFill`, `routeStroke`, `positionFill` and similar. Renaming them is a palette-contract change across styling and both renderers.
- ui-scene defaults: `vector_style = "colored"` and the `/osm/…` and `/vt/…` tile templates. They are wire defaults that the typed catalog and lane fixtures pin, and the GIS window relies on the templates. Neutralising them needs GIS to always set its templates. I left them alone to keep the peer-active ui edits minimal.
- The shared React context-menu title table names `position`/`route` among the other apps' domains. It is not tiled-map code.
