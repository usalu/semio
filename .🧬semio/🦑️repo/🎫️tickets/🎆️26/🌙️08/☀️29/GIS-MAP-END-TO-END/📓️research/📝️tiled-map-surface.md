# Tiled Map Surface Architecture (🗺️tiled-map)

**File**: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` (~4936 lines)

---

## Section Outline

| Region | Lines | Purpose |
|--------|-------|---------|
| 🔖️MapPalette | 24–70 | 12-field color palette (land, water, labels, routes, positions, selection, hover) |
| 🔖️MapLod | 72–276 | 8-level LOD hierarchy; tile zoom picks (GIS_MAP_LOD_TILE_Z: [0,1,2,3,4,5,7,10,18]); span thresholds (GIS_MAP_LOD_MAX_SPAN_DEG) |
| 🔖️Projection | 278–325 | Web Mercator lonlat↔world conversion; tile world rect calc; cover zoom for viewport |
| 🔖️Tiles | 327–464 | VisibleTile/VisibleTileCursor; tile z picker; visible tile enumeration; tile key/ancestor genealogy |
| 🔖️VectorTiles | 466–1464 | MVT (Mapbox Vector Tile) protobuf decoder; geometry decode (rings/lines/points, zigzag VLQ); layer/feature structs; per-LOD visibility gates; style profiles (colored/figureGround/inverted) |
| 🔖️MapExtension | 1466–1485 | CanvasExtension trait impl; EPSG:3857 CRS declaration |
| 🔖️MapContent | 1487–2077 | PositionData, RouteData, RegionData, MapDescriptorJson; render modes (combined/image/vector); vector styles; layer visibility/stroke scale; map interaction (pan gesture); hit testing; polygon/polyline intersection geom |
| 🔖️LabelDeclutter | 2077–2138 | Spatial grid-based collision detection for map labels (grid cells, overlap test, placement attempt) |
| 🔖️WasmSession | 3519–3779 | MapSession WASM interface; MapSessionInner wrapper; gpu_ready/attachCanvas/setSize/renderFrame flow; event draining |
| 🔖️Tests | 3781–4936 | 60+ tests: tile caching, LOD picking, MVT decode, interaction, hit test, label placement, vector styles |

---

## 1. MapHost Structure & Ownership

### Fields

| Field | Type | Ownership | Mutation |
|-------|------|-----------|----------|
| `camera` | `camera::Camera` | (c) Preview/Effect | Pan/zoom gestures; clamped to world bounds |
| `viewport` | `camera::Viewport` | (d) Render wiring | `set_size()` only; DPR, width, height |
| `features` | `MapFeatureTables` (positions/routes/regions) | (a) Elsewhere | Wholesale sync via `sync_map_json()`; no field-by-field mutations; real owner is gis plugin's `GismapSnapshot` |
| `tiles` (MapTileLedger) | `BTreeMap<String, Arc<RasterImage>>` + vector tiles | (d) Ephemeral cache | LRU eviction in `retain_tiles_for_keys()` |
| `render_mode` | `MapTileMode` (Image/Vector/Combined) | (c) Preview/Effect | `set_render_mode()` mirrors plugin's `SetRenderMode` command |
| `vector_style` | `MapVectorStyle` (Colored/FigureGround/InvertedFigure) | (c) Preview/Effect | `set_vector_style()` mirrors `SetVectorStyle` |
| `forced_lod_id` | `Option<String>` | (c) Preview/Effect | Pinned LOD override; mirrors `set_lod_mode()` |
| `layer_visibility` | `MapLayerVisibility` (raster, water, land, roads, buildings, borders, positions, positionLabels, routes, regions, labels) | (c) Preview/Effect | Per-layer toggles mirror `ToggleLayerVisibility` |
| `layer_stroke_scale` | `MapLayerStrokeScale` (per-layer weight multipliers) | (c) Preview/Effect | Per-layer stroke weights mirror `SetLayerStrokeScale` |
| `events` | `Vec<serde_json::Value>` | (c) Preview/Effect | Drained each frame; interaction event log; not persisted |
| `interaction` | `MapInteraction` (None or Pan{origin, start_screen}) | (c) Preview/Effect | Pan gesture state; discarded on release |
| `theme` | `MapPalette` | (d) Runtime wiring | Derived from app UI theme via `set_map_theme_from_json()` |
| `selected_positions` / `selected_routes` | `BTreeSet<String>` | (c) Preview/Effect | Selection state; read from framework's `DomainSelection` via `sync_interaction()` |
| `hovered_kind` / `hovered_id` | `Option<String>` | (c) Preview/Effect | Hover feedback (position/route); UI-only |
| `interaction_revision` | `u64` | (c) Preview/Effect | Monotonic counter for interaction state validation |

### MapHostRetirement
- Retains all ownable types (positions, routes, regions, tiles, events) for gradual per-frame release.
- `close_step()` pops one scalar per call; blocks on terminal-empty in Drop debug_assert.
- Allows fine-grained cleanup without stalling frame.

---

## 2. Tile Pipeline: Camera → Viewport → Visible Tiles → z/x/y

### Entry Point
- **`prepare_visible_tiles()`** (line 2825): Called once per frame before scene build.
  - Calls `pick_raster_tile_zoom()` → `pick_tile_z_target()`.
  - Computes visible raster + vector tiles; retains ancestors for pyramid fallback.

### Web Mercator Projection Math (📐️Projection module, lines 278–325)

```rust
// World coordinates: [-1, 1] on each axis after normalization
pub fn lonlat_to_world(lon: f64, lat: f64) -> Point {
    let x = lon / 180.0 * WORLD_HALF;
    let lat_rad = lat.to_radians();
    let y = (0.5 * tan(π/4 + lat_rad/2).ln() / π) * WORLD_HALF * 2.0;  // log(tan) Mercator formula
    Point::new(x, y)
}

pub fn tile_world_rect(z: u32, x: u32, y: u32) -> Rect {
    let n = 2.0^z;
    let step = (WORLD_HALF * 2.0) / n;
    let min_x = -WORLD_HALF + x as f64 * step;
    let max_y = WORLD_HALF - y as f64 * step;
    Rect::new(min_x, max_y - step, min_x + step, max_y)
}
```

- Max latitude clamped: **MAX_LAT = 85.051°** (Web Mercator standard).
- World span: **WORLD_VISIBLE_SPAN = 2.0** ([-1, 1]).

### Tile Z Computation (lines 182–276)

**`pick_tile_z_target(camera, viewport, forced_lod_id)`**:
1. **Ideal z**: `ideal_tile_z_for_viewport()` (line 169).
   - Viewport longitude span → ideal z = log₂(360 / span × width / 256).
2. **LOD index**: `resolve_map_lod_index_from_span(span_deg)` → threshold lookup in **GIS_MAP_LOD_MAX_SPAN_DEG**: [100°, 35°, 12°, 4°, 1.2°, 0.35°, 0.1°, 0°].
3. **Floor floor**: If forced_lod_id set, clamp z ≥ GIS_MAP_LOD_TILE_Z[lod_idx].
4. **Constraint**: Repeat `pick_zoom()` loop: if visible_tile_cursor remainder > MAX_VISIBLE_TILE_REQUESTS (256), decrement z until ≤ 256 tiles visible.

**GIS_MAP_LOD_TILE_Z** (line 99): Minimum tile z per LOD (coarser levels force coarser tiles):
- Index 0–8: [0, 1, 2, 3, 4, 5, 7, 10, 18] — world to building.

### Visible Tile Enumeration (lines 327–464)

**`visible_tile_cursor(camera, viewport, z)`**:
1. Screen corners → world coords (via `map_viewport::screen_to_world()`).
2. Min/max x,y bounds in world space.
3. Tile indices: x0, x1, y0, y1 = (bounds + WORLD_HALF) / step, clamped to [0, 2^z).
4. Returns **VisibleTileCursor** (stateful iterator): peek/advance, remaining count.

**Ancestry Pyramid Fallback** (lines 441–462):
- `tile_key_ancestors(z, x, y)` → walks up to z=0, halving x,y each step.
- `tile_retention_keys(visible, previous)` → unions visible ancestors + previous visible keys.
- Eviction policy: **BTreeMap.pop_first()** (oldest key) when cache exceeds MAX_MAP_TILE_CACHE_ENTRIES (512).
- **No LRU timestamp**; BTreeMap order is lexicographic (z/x/y key), not access-time. Oldest is smallest key.

---

## 3. Tile Cache & Eviction

### Cache Config
- **MAX_MAP_TILE_CACHE_ENTRIES**: 512 (raster + vector share this limit per type).
- **MAX_VISIBLE_TILE_REQUESTS**: 256 (viewport can request at most 256 tiles; drives z downsampling).

### Eviction Policy (lines 2368–2386)
```rust
fn retain_tiles_for_keys(&mut self, keys: &BTreeSet<String>) {
    self.tiles.tile_images.retain(|k, _| keys.contains(k));
    while self.tiles.tile_images.len() > MAX_MAP_TILE_CACHE_ENTRIES {
        if self.tiles.tile_images.pop_first().is_none() { break; }
    }
}
```

- **Simple retention**: Keep only keys in visible set + ancestors.
- **Eviction**: Truncate oldest (lexicographic) first when cache exceeds 512.
- **NOT LRU**: No access-time tracking; oldest key is lowest lexicographically.
- **Pyramid fallback**: Ancestors retained automatically; parent tiles persist while children load (prevents flicker on zoom).

### Prefetch
- **None explicit**. Visible tiles + ancestors only.
- Neighboring tiles not prefetched; coarser pyramid ancestors serve as visual fallback.

---

## 4. Tile Fetching

### Source & Async

**`upload_tile(z, x, y, png_bytes)`** (line 2455):
- Caller (JavaScript bridge) decodes PNG → RGBA8 → uploads bytes.
- Stores in `BTreeMap<String, Arc<RasterImage>>` keyed by `tile_key(z, x, y)`.
- **Synchronous**: No async inside Rust; async orchestration happens in JS/bridge layer.

**`upload_vector_tile(z, x, y, pbf_bytes)`** (line 2464):
- Decodes MVT protobuf via `vector_tiles::decode_mvt()`.
- Stores in `BTreeMap<String, VectorTile>`.
- **Error handling**: Returns `FrameworkSurfaceTiledMapError::Mvt(MvtDecodeError)`.

### Network Location
- **Not in Rust**: Tile bytes fetched by JS caller (browser fetch, service worker, or HTTP client).
- Rust only receives pre-fetched bytes; no HTTP or disk I/O in component.

### Cancellation
- **Implicit**: Tiles not in `visible_tiles()` set are evicted on next `prepare_visible_tiles()`.
- No explicit request cancellation; eviction replaces in-flight requests.

### Deduplication
- **None explicit**. Caller must deduplicate; Rust accepts any tile key.
- Repeated `upload_tile(z, x, y, ...)` overwrites previous Arc<RasterImage>.

---

## 5. Raster vs Vector: Modes & Styles

### Render Modes (line 1506)
- **Combined** (default): Both raster tiles + vector overlay.
- **Image**: Raster tiles only.
- **Vector**: Vector tiles only; no raster backdrop.

### Vector Styles (line 1518)
- **Colored** (default): Full color palette (water fill, land, roads, borders).
- **FigureGround**: Line/label ink on paper (two-color, high contrast).
- **InvertedFigure**: Inverted polarity (paper ink on line).

### Vector Decoder: MVT/Protobuf (lines 466–1464)

**Owned Protobuf Implementation** (NOT using external crate; lines 650–705):
- **ProtoCursor**: Manages byte offset; reads varint, fixed32/64, length-delimited fields.
- **decode_raw_tile()** → layers.
- **decode_raw_layer()** → keys, values, features, extent.
- **decode_raw_feature()** → id, tags (key/value indices), geometry, geom_type.
- **Wire types**: 0=varint, 1=fixed64, 2=length-delimited, 5=fixed32.

**Geometry Decoding** (lines 809–889):
- Command stream (MoveTo=1, LineTo=2, ClosePath=7) with zigzag-encoded deltas.
- Outputs **rings** (closed polygons), **lines** (polylines), **points**.
- Tile seam artifacts filtered: `mvt_segment_is_tile_seam()`, `mvt_polyline_is_tile_bbox_artifact()`.

**Layer-to-Feature Mapping** (lines 1066–1081):
```rust
for feat in layer.features {
    let geom_type = match RawGeomType::try_from(geom_type) { Point|LineString|Polygon|Unknown }
    let props = decode_properties(&feat.tags, &layer.keys, &layer.values);
    let (rings, lines, points) = decode_geometry(&feat.geometry, geom_type);
    features.push(VectorFeature { id, geom_type, rings, lines, points, properties });
}
```

---

## 6. Labels: Layout & Rendering

### Label Declutter (lines 2077–2138)

**Grid-based Collision Detection**:
```rust
struct LabelDeclutter {
    cell: f64,  // ~12–60 px
    cols, rows: usize,
    mask: Vec<bool>,  // occupancy grid
    count, max_count: usize,
}

fn try_place(&mut self, label: &str, origin: Point, px: f64) -> bool {
    // Estimate bounding box
    let (x, y, w, h) = Self::estimate_box(label, px, origin);
    // Clamp to viewport; round to grid cells
    let cx0..=cx1, cy0..=cy1 = ...
    // Check all cells occupied?
    for cy in cy0..=cy1 { for cx in cx0..=cx1 { if mask[...] return false; } }
    // Mark cells as occupied
    for cy in cy0..=cy1 { for cx in cx0..=cx1 { mask[...] = true; } }
    count += 1; true
}
```

- **Cell size**: `max(viewport.height / max_count, 12.0)`.
- **Max labels**: Configurable; default depends on LOD.
- **No priority**: First-fit (traversal order determines placement).

### Label Rendering (lines 3100–3340)

**`append_vector_tile_labels()`**:
- Iterates draw set (visible vector tiles at highest z).
- Per tile: per layer: per feature with `feature_label()`.
- `LabelDeclutter::try_place()` → if success, `text::append_label(scene, label, anchor, px, fill, halo)`.
- Halo color varies by style (colored→label_halo; figureGround→surface_clear; inverted→label_fill).

**Label Font** (fonts `🔤️MapLabelSans.ttf`):
- Referenced in text module (not in this file); embedded or shipped separately.

---

## 7. Draw Path: Frame → Scene → GPU

### Frame Render (line 3544)

```rust
fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
    self.host.prepare_visible_tiles();  // Compute visible; evict cache
    let scene = self.host.build_render_scene();  // Emit Scene
    self.gpu.render_frame(&scene, clear_color)  // Send to GPU
}
```

### Scene Build (lines 3479–3505)

**`build_vector_scene()`**:
1. **Raster**: `append_tiles()` if Image|Combined mode.
   - Iter cache; cull off-viewport; draw affine-transformed images.
2. **Vector**: `append_vector_tiles()` if Vector|Combined mode.
   - Render colored/figureGround/inverted style.
   - Iterate layers by draw rank; apply per-LOD visibility gates.
   - Append fills (water, land, buildings, countries, parks) + strokes (roads, boundaries, waterways, coastlines).
   - Append labels (if layer_visibility.labels).
3. **Regions**: `append_regions()`.
   - Convert lonlat ring → world → screen; stroke+fill polygon.
4. **Routes**: `append_routes()`.
   - Polyline with optional halo (hover/select).
5. **Positions**: `append_positions()`.
   - Circle marker + stroke + optional label.

### Output
- **Scene**: Collection of fill/stroke commands (Kurve/Skia-style).
- **Emits**: Affine-transformed fill/stroke paths; raster images; text quads (via text module).
- **No vertex buffers** visible here; Scene is retained intermediate representation passed to GPU layer.

### DPR Scaling (lines 3496–3505)
- If DPR ≠ 1.0, wrap scene in scale transform.
- Logical coordinates scaled to physical pixels.

---

## 8. Error Handling & Stubs

### Errors (lines 2138–2179)
```rust
pub enum FrameworkSurfaceTiledMapError {
    Json(serde_json::Error),
    Image(image::ImageError),
    Mvt(vector_tiles::MvtDecodeError),
}
```

- **JSON**: `sync_map_json()`, theme parsing.
- **Image**: PNG decode via `image` crate.
- **MVT**: Protobuf parsing; bounded offset + message.

### No Unimplemented / TODO / Stubbed Functions
- No `todo!()`, `unimplemented!()`, or `unreachable!()` in hot paths.
- All `unwrap()` in tests (safe contexts) or safe `try_into()` casts (fixed sizes).
- No `FIXME` or `HACK` comments.

---

## 9. Performance Hazards

### Per-Frame Allocations
- **Scene**: Created fresh each frame (`Scene::new()`); no pooling.
- **Draw vec**: Filtered visible tiles collected into temp Vec; sorted.
- **Geometry**: No re-tessellation; MVT geometry decoded once (on upload), stored in cache.

### Cloning
- **Arc<RasterImage>**: Cheap (ref count +1). Line 3357 clones Arc in draw vec (acceptable).
- **VectorTile**: Cloned into cache on upload; stored in BTreeMap (one copy per tile).
- **Strings**: ID/label strings cloned in collections; no batching.

### Cache Misses
- **No prefetch**: Only visible + ancestors loaded.
- **Pyramid fallback**: Coarser tiles mask loading delay; smooth visual continuity.
- **No streaming**: Entire tile uploaded atomically; no progressive decode.

### Blocking I/O
- **None in Rust**: Tile bytes supplied by caller; no disk/network wait.
- JS caller orchestrates async fetch; Rust `upload_tile()` is synchronous append.

### Unbounded Growth
- **Tile cache**: Capped at 512 per type (raster/vector).
- **Position/route/region**: Synchronized wholesale via `sync_map_json()`; no incremental append.
- **Events**: Drained each frame; not persisted.

### Collision Detection
- **LabelDeclutter**: Grid-based O(cells²) per label; max_count limits total.
- **Hit testing**: Brute-force intersection tests over visible positions/routes/regions; O(n) per query.

---

## 10. Notable Features

### Selection & Hover
- Synced from framework's **DomainSelection** via `sync_interaction(granularity, selected_ids, hovered_id)`.
- Positions/routes rendered with selection/hover halo (2.4× stroke).
- `selected_positions`, `selected_routes`, `hovered_kind`, `hovered_id` fields.

### Interaction (Pan Gesture)
- **PointerDown**: Start pan from screen origin.
- **PointerMove**: Delta screen → delta world; pan camera.
- **PointerUp**: Commit if drag > 6 px.
- Camera always clamped to world bounds (no outscroll).

### Hit Testing
- **`hit_test_feature_json(sx, sy)`** (line 2519): Exact point hit.
- **`features_in_rect_json(x0, y0, x1, y1, crossing)`** (line 2589): Rect containment.
- **`features_in_polygon_json(points, crossing)`** (line 2608): Polygon intersection (point-in-polygon + segment tests).

### Layer Visibility & Stroke Scale
- Per-layer toggles (raster, water, land, roads, buildings, borders, positions, routes, regions, labels).
- Per-layer stroke multipliers; clamped to [MAP_LAYER_WEIGHT_MIN, MAP_LAYER_WEIGHT_MAX].
- Synced from plugin via `set_layer_visibility_json()`, `set_layer_stroke_scale_json()`.

### Camera Limits
- **Min zoom**: max(cover_zoom(viewport), MAP_CAMERA_ZOOM_MIN=8.0) — world fits on screen.
- **Max zoom**: MAP_CAMERA_ZOOM_MAX=100M — ~1.4e-3° span (street scale).
- Always re-clamped on viewport resize.

---

## Key Paths

**Tile computation**: `pick_tile_z_target()` (182) → `viewport_lon_span_degrees()` (119) → `GIS_MAP_LOD_MAX_SPAN_DEG` lookup.

**Tile picking**: `ideal_tile_z_for_viewport()` (169) → log₂ formula.

**Visible tiles**: `visible_tile_cursor()` (368) → stateful iteration; `tile_retention_keys()` (456) → ancestors.

**Eviction**: `retain_tiles_for_keys()` (2368) → `pop_first()` if > 512.

**MVT decode**: `upload_vector_tile()` (2464) → `decode_mvt()` (1074) → `ProtoCursor` parsing.

**Render**: `prepare_visible_tiles()` (2825) → `build_vector_scene()` (3479) → `append_vector_tiles_colored()` (3154) / `append_vector_tiles_figure()` (3266).

**Label placement**: `append_vector_tile_labels()` (3095) → `LabelDeclutter::try_place()` (2098).

---

## Summary

**Tiled Map Surface** is a **cached, hierarchical, Web Mercator tile renderer** combining:
- **Raster tiles** (PNG, RGBA8) in BTreeMap cache with automatic ancestor retention.
- **Vector tiles** (MVT/protobuf, custom decoder) with per-LOD layer visibility.
- **User features** (positions, routes, regions) synced from plugin; rendered with selection/hover.
- **Label declutter** via spatial grid; no A* priority, just first-fit placement.
- **Interactive pan** with camera clamp to world bounds.
- **DPR-aware scene build** scaling logical to physical pixels.

No async I/O, streaming, or prefetch in Rust; all orchestrated by JS caller. Pyramid fallback (ancestor retention) prevents flicker on zoom. Cache is simple FIFO eviction (not LRU), but ancestor pinning provides effective multi-resolution fallback. Performance is bounded: 512 tile cache, 256 visible limit, no per-frame re-tessellation.

