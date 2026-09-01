# 🗺️ Verfolgen (`s.gis.gismap@1/*#editor`) — fixture / window / interactivity / tiles

Plugin: `✏️s/🔌️plugins/🌍️gis`, artifact `🗿️artifacts/🗺️gismap`. Default example: `reuse-map`.

## 1. Editor and default window

`✏️editor/🦀️component.rs` — `impl ArtifactEditor for Gis2dPlayApp` (:572), manifest `create_gis2d_app()` (:841),
controller `s.gis.gismap@1/*#editor` (:600 test proof). Single mode `edit`
(`🎭️modes/✏️edit/🦀️component.rs:7-15`):

```rust
pub fn layout() -> WindowLayout {
    create_default_layout(&[map::GIS2D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Map".into()]))
}
```

Exactly **one** default window: `gis2d-main`, label "Map"/"Karte",
`surface_kind: SurfaceKind::TiledMap`, `body_key: "gis2d.play.composite"`
(`🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs:20-37`).

## 2. `setActiveExample` — real content, but the id is never matched

`✏️editor/🎮️commands/🎨️example/🦀️component.rs`:

```rust
let next = if payload.example_id.is_empty() { GisMapSnapshot::default() } else { default_document() };
let mut artifact_mutations = positions_operations(&document.positions, &next.positions);
artifact_mutations.extend(routes_operations(&document.routes, &next.routes));
artifact_mutations.extend(regions_operations(&document.regions, &next.regions));
```

Only two effective arms: empty id clears; **any** non-empty id (including `"reuse-map"`, the manifest's only
declared `ActionArgOption` at `🦀️component.rs:919-923`) loads the same `default_document()`. The id string
itself is never branched on.

`default_document()` (`🧬️schema/🦀️component.rs:301-303`) parses `REUSE_MAP_EXAMPLE_TEXT`,
`include_str!` from `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`
(`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:12`). Decoded content is real and non-empty: 2 positions
("Institut de Botanique", "Lycee Block 3000", Liège, lat≈50.60 / lon≈5.58-5.59), 2 routes
("Holz Fassade Botanique", "Stahl Mehrere Lycee Profiles Canopy"), 0 regions.

Loading is done as **diffed create operations with true inverses** (undo-safe), not a snapshot swap.
`SetActiveExample` also fits the camera (`map_host_from(...).fit_world_camera()`) and is declared
`ActionKind::Mutation` under registry kind-discipline (test at :74-84).

## 3. Document → surface

`ArtifactEditor::render` (`🦀️component.rs:812-822`) routes `body_key == "gis2d.play.composite"` to
`map::render` (`🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs:80-94`):

```rust
let mut scene = TiledMapScene::base(gis_map_descriptor_json(document), cfg.camera_json.clone());
scene.render_mode = cfg.render_mode.clone();
apply_gis_map_tile_base_url(&mut scene);
scene_surface(GIS2D_PLAY_SURFACE, ContractSurfaceKind::TiledMap, &scene)
```

`gis_map_descriptor_json` (`🧬️schema/🦀️component.rs:289-297`) serializes `{positions, routes, regions}`
from the opaque per-feature DSL payloads back to JSON — the same shape `map:out` media export uses.
Frontend host: `TiledMapHost`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TiledMapHost/🟦️component.tsx`),
backed by `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` (4936 lines of real geometry /
MVT-decode / LOD logic; no `todo!()`/`unimplemented!()`).

## 4. Map tiles in dev

Sources from `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:37-56`:
`/osm` → `https://tile.openstreetmap.org/{z}/{x}/{y}.png` (cache `osm-tiles`);
`/vt` → `https://tiles.openfreemap.org/planet` (cache `openfreemap-vt`); `/dem` → terrarium (out of scope).

Demonstrator `⚙️vite.config.ts:93` calls
`resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)`
(`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts:960-962`):

```ts
export function resolveGisMapTileServeMode(value?: string): GisMapTileServeMode {
  return value === "bundle" ? "bundle" : "fetch";
}
```

Default is **`"fetch"`**: a 7-day disk cache under `.🧬semio/🗺️map/{osm-tiles,openfreemap-vt}`, but a cache
miss fetches live from the internet (`createTileProxyMiddleware`, `fetchTileProxyTileToCache`, :1209-1276).
`GIS_MAP_TILE_SERVE_MODE=bundle` serves cache-only and 404s on a miss.

**Failure behaviour:** `refreshRasterTiles`/`refreshVectorTiles` (`TiledMapHost 🟦️component.tsx:451-511`)
add failed keys to a `tileMiss`/`vectorTileMiss` set and skip them — no error UI, no placeholder. Vector
features from the document render independently, so **the fixture markers/routes still show with zero
network; only the basemap is blank.**

## 5. Interactivity

**Working:** pan (middle-drag) and wheel zoom via wasm `pointerDownScreen`/`pointerMoveScreen`/`wheelScreen`;
hover hit-testing `session.hitTestFeatureJson` (:650); layer visibility toggle, stroke-scale sliders,
render-mode / vector-style / LOD selects (`toggle_layer_visibility`, `set_layer_stroke_scale`,
`set_render_mode`, `set_vector_style`, `set_lod_mode` in `👁️view/🦀️component.rs`);
`focusFeature`/`fitWorld`/`setCamera`/`openSource`; right-click context menu
(`gis2d_context_menu_items`, `🦀️component.rs:549-570`).

**BUG — selection and hover are dispatched through retired action ids.** `TiledMapHost.tsx:903-910,956`:

```ts
dispatch("setFeatureSelection", { positions: [...hits.positions], routes: [...hits.routes], mode, crossing });
dispatch("setHover", { hover: nextHover });
```

Those ids were **deleted** (not renamed) by ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W1 in
favour of the generic `"features"` interaction domain (`interactionSelect`/`interactionHover`) —
`🛂️manifest/🦀️component.rs:557-566`. `Gis2dPlayApp::command_from_action` has no matching arm and its
catch-all (`🦀️component.rs:788-792`) returns a `Fault`. **Net effect: clicking a feature to select it and
hovering for the popup silently fail.** `TiledMapHost` is shared across TiledMap-surface apps — check other
consumers before changing the ids.

## 6. Stub sweep

No `todo!()`/`unimplemented!()`/TODO/FIXME/"placeholder" in the gismap editor tree or in the tiled-map
surface module. The only "placeholder" hits in `🌍️gis` are in the unrelated `🏔️gisterrain` artifact.

## 7. Verdict

A user sees a full-width Map window booting with the real Liège reuse-map fixture (2 positions, 2 routes)
over OSM raster + OpenFreeMap vector tiles (network needed in dev; degrades to blank basemap with fixture
features still visible). Pan, zoom, camera fit, layer controls, render/style/LOD switches and "open source"
all work.

Gaps to close:
1. **Feature click-select and hover-popup are broken** — update `TiledMapHost.tsx` to dispatch the generic
   `interactionSelect`/`interactionHover` domain actions the Rust side already expects.
2. `setActiveExample` ignores `example_id` beyond empty-vs-non-empty, so there is effectively one example
   rather than a selectable catalogue.
