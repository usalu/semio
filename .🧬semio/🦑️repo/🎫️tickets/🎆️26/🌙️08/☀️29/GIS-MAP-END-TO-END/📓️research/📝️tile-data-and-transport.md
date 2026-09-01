# GIS Map Tile Data & Transport: End-to-End Analysis

## 1. Map Tile Data Directory Inventory

### Directory Structure
- **Location**: `.🧬semio/🗺️map/`
- **Total Size**: 21 MB
- **Git Status**: Gitignored (see `.gitignore`)

### Subdirectories

#### `osm-tiles/` — Raster Tiles
- **Format**: PNG raster tiles in directory-based store (NOT pmtiles/mbtiles)
- **Structure**: `z/x/y.png` (Web Mercator zoom/column/row)
- **Source**: OpenStreetMap raster tiles cached locally
- **Sample files**: `0/0/0.png`, `2/1/2.png`, `2/3/3.png`, etc.
- **Zoom levels observed**: 0–2
- **Git status**: Gitignored (tile data not committed)

#### `openfreemap-vt/` — Vector Tiles
- **Format**: PBF (Protocol Buffer) vector tiles in directory-based store (NOT pmtiles/mbtiles)
- **Structure**: `z/x/y.pbf` (Web Mercator zoom/column/row)
- **Source**: OpenFreeMap planet MVT tiles cached locally
- **Sample files**: `0/0/0.pbf`, `3/0/1.pbf` through `3/7/7.pbf`
- **Zoom levels observed**: 0–3
- **Git status**: Gitignored (tile data not committed)

### Summary
- **Neither is pmtiles/mbtiles**: Both are simple directory hierarchies
- **Both follow standard Web Mercator tiling scheme**
- **Tile data is gitignored** (likely pre-downloaded by deployment/build process)

---

## 2. Tile URL Templates & Providers (Grep Results)

### Upstream Sources
| Template | Type | File | Line |
|----------|------|------|------|
| `https://tile.openstreetmap.org/{z}/{x}/{y}.png` | Raster (OSM) | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` | ~1165 |
| `https://tiles.openfreemap.org/planet` | Vector (TileJSON endpoint) | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` | ~1052 |
| `https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png` | Raster (DEM/Terrarium) | Frame-worker config | — |

### Tile URL Usage in Code

#### Runtime Code (Rust/WASM)
| Location | Line | Purpose |
|----------|------|---------|
| `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` | 1206 | Format tile URL template with `{z}/{x}/{y}` substitution |
| `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` | (various) | Terrain tile LOD selection |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` | (various) | World terrain style configuration |

#### TypeScript/JavaScript Configuration
| Location | Purpose |
|----------|---------|
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` | Tile proxy config & TileJSON resolution |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟨️frame-worker.js` | Asset specs with tile routes and upstreams |

#### Test Fixtures
| Location | Content |
|----------|---------|
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (GOLDEN_SCENES_JSON) | Hard-coded test scene with `/osm/{z}/{x}/{y}.png` and `/vt/{z}/{x}/{y}.pbf` URLs |

#### Metadata
- **GIS_MAP_OPENFREEMAP_TILEJSON**: `"https://tiles.openfreemap.org/planet"` (TileJSON endpoint, queried to resolve actual tile URL)
- **Tile proxy routes**: `/osm` → raster, `/vt` → vector

---

## 3. Tile Request Transport Chain: Full Trace

### Transport Flow Diagram
```
Rust MapHost (Renderer thread)
    ↓ (render loop calls queue_map_tile_fetch_step)
    ↓ reserve_map_tile_fetch() → URL formatted
    ↓ crate::reserve_renderer_asset_request()
    ↓ [WASM → Renderer Asset Authority (mutex-locked)]
    ↓ WASM: pollAssetRequest() → JavaScript
    ↓ JavaScript: fetch(url) to tile proxy server
    ↓ Node.js tile proxy middleware
    ↓ Cache lookup / disk fetch / upstream proxy
    ↓ HTTP response (PNG/PBF bytes)
    ↓ JavaScript: pushAssetResponsePage() → WASM
    ↓ WASM: sealAssetResponse()
    ↓ WorldAssetFetchOwner → Rust frame loop
    ↓ apply_map_tile_bytes()
    ↓ host.upload_tile() / host.upload_vector_tile()
    ↓ MapHost cache (BTreeMap<key, Arc<RasterImage>|VectorTile>)
    ↓ Render pipeline reads from cache
```

### Each Hop (Sync/Async/Caching)

#### 1. **Rust Render Loop → Tile Request Queue**
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` : ~3083
- **Function**: `queue_map_tile_fetch_step()`
- **Synchronous**: Yes
- **Cancellable**: Via `MapTileRequestCursor` state reset
- **Cache Check**: `host.has_tile(key) / host.has_vector_tile(key)` before requesting
- **Details**: Iterates visible tile cursors, skips cached tiles, calls `reserve_map_tile_fetch()`

#### 2. **reserve_map_tile_fetch() → WorldAssetRequestKind → Asset Authority**
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` : 2861
- **Synchronous**: Yes
- **Function**: Formats URL template, calls `reserve_renderer_asset_request()`
- **Error handling**: Returns `WorldAssetFault` on URL overflow or invalid IDs

#### 3. **Renderer Asset Authority (Mutex)**
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` : 141–180
- **Synchronous**: Yes (mutex-locked)
- **Key Functions**:
  - `reserve_renderer_asset_request()`: Queue a new fetch request
  - `take_next_renderer_asset()`: Poll for pending requests
  - `reserve_renderer_asset_response()`: Allocate byte credits
  - `seal_renderer_asset_response()`: Finalize response
- **Cache behavior**: On-disk cache managed by tile proxy middleware (not in MapHost)
- **Concurrency**: Guarded by `OnceLock<Mutex<WorldAssetIoAuthority>>`

#### 4. **WASM Browser Worker ↔ JavaScript**
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️browser_worker.rs` : ~170–230
- **Asynchronous**: Yes (JavaScript fetch is async, WASM polls via `pollAssetRequest()`)
- **Key WASM methods**:
  - `pollAssetRequest()`: Returns JSON with pending URL and byte capacity
  - `pushAssetResponsePage(bytes: &[u8])`: Receives chunked response (bounded to `WORLD_ASSET_RESPONSE_PAGE_BYTES` per chunk)
  - `sealAssetResponse()`: Finalizes, returns ownership to authority
  - `abortAssetResponse()`: Cancels in-flight request
- **Byte credits**: Fixed aggregate and per-page limits prevent OOM
- **Cancellable**: Yes, via `abortAssetResponse()`

#### 5. **JavaScript Fetch (Browser)**
- **Caller**: Browser main thread or worker thread (code not shown but called by frame-worker.js)
- **Asynchronous**: Yes (native fetch API)
- **Transport**: HTTP GET to tile proxy URL
- **Cancellable**: Yes (AbortController supported)
- **Details**: 
  - No hardcoded retry logic visible in tile fetch
  - Responses streamed in pages to respect byte limits
  - On 404/502: response sealed with no data → tile remains uncached

#### 6. **Tile Proxy Middleware (Node.js)**
- **File**: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` : 1234–1277
- **Synchronous**: No (async fetch to upstream)
- **Function**: `createTileProxyMiddleware()`
- **Request pattern**: `^/route/(\d+)/(\d+)/(\d+)\.(\w+)(?:\?.*)?$`
- **Cache behavior**:
  - **Disk cache root**: `.🧬semio/🗺️map/{cache-name}/` (e.g., `osm-tiles`, `openfreemap-vt`)
  - **On hit**: Stream from disk via `createReadStream()` → `res.pipe()`
  - **On miss (dev mode only)**:
    1. Fetch upstream URL with User-Agent header
    2. Check response status; return 404 if failed
    3. Write to disk cache via `mkdir()` + `writeFile()`
    4. Stream cached file to client
- **Error responses**:
  - **404**: If upstream fails or returns non-200
  - **502**: If fetch throws (network error, timeout)
- **Modes**: `"fetch"` (dev), `"bundle"` (production)

#### 7. **Upstream TileJSON Resolution**
- **File**: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` : 1187–1207
- **Function**: `resolveTileProxyUrlTemplate()`
- **Asynchronous**: Yes
- **Logic**:
  1. If upstream already contains `{z}`, use as-is
  2. Otherwise, fetch upstream as TileJSON endpoint
  3. Extract `tiles[0]` URL template
  4. Cache result for 7 days (TTL: `TILE_PROXY_TEMPLATE_TTL_MS`)
- **Used by**: Both dev middleware and prefetch logic

#### 8. **MapHost Upload & Cache**
- **File**: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` : 2455–2468
- **Synchronous**: Yes (in-memory cache insert)
- **Raster tiles**: `host.upload_tile(z, x, y, png_bytes)` → decode PNG → insert into `tile_images` BTreeMap
- **Vector tiles**: `host.upload_vector_tile(z, x, y, pbf_bytes)` → decode MVT protobuf → insert into `vector_tiles` BTreeMap
- **Cache eviction**: LRU when exceeding `MAX_MAP_TILE_CACHE_ENTRIES` (512 entries)
- **Disk cache**: Persists to `.🧬semio/🗺️map/` on dev server; served by middleware on subsequent requests

### Summary
| Hop | Sync/Async | Cancellable | Cached | Location |
|-----|-----------|------------|--------|----------|
| Render→Request | Sync | Yes (cursor reset) | Yes (in-memory MapHost) | EngineCanvas/component.rs:3083 |
| Request formatting | Sync | No | Yes (MapHost.has_tile check) | EngineCanvas/component.rs:2861 |
| Asset authority queue | Sync | Yes (abort) | Yes (disk at proxy) | glue.rs:141 |
| WASM polling | Async | Yes (abort) | — | browser_worker.rs:171 |
| JS fetch | Async | Yes (AbortCtrl) | Yes (disk + in-memory) | frame-worker.js |
| Proxy middleware | Async | Yes | Yes (disk at `.🧬semio/🗺️map/`) | vite-elements-assets.ts:1234 |
| Upload to MapHost | Sync | No | Yes (in-memory, LRU evict @ 512) | tiled-map/component.rs:2455 |

---

## 4. Vector Tile (MVT Protobuf) Decoder

### Status: **Fully Hand-Rolled in Rust**

#### Location
- **File**: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` : lines 560–936

#### Implementation Details

**Protobuf Wire Type Decoder**
- Custom `ProtoCursor` struct for byte-level parsing
- Supports all wire types: varint (0), fixed64 (1), length-delimited (2), fixed32 (5)
- Varint decoding with overflow checks (max 10 bytes)
- Key-value tag parsing: `(tag >> 3, tag & 0x07)`

**Tile-Level Decoding**
- `decode_raw_tile()` → parses root protobuf, extracts layers
- `decode_raw_layer()` → parses MVT layer, extracts features, keys, values, extent
- `decode_raw_feature()` → parses geometry type, tags (key-value indices), geometry commands
- `decode_raw_value()` → parses value types: string, float, double, int, uint, bool

**Geometry Command Decoder**
- `decode_geometry()` → processes MoveTo (cmd=1), LineTo (cmd=2), ClosePath (cmd=7)
- Zigzag decoding for signed deltas: `((n >> 1) ^ -((n & 1)))`
- Outputs three separate rings/lines/points based on geometry type
- Handles Polygon, LineString, Point types

**Tag-to-Property Decoding**
- `decode_properties()` → converts tag array (alternating key idx, value idx) to `BTreeMap<String, String>`
- Value conversion to string via `raw_value_string()`: handles all scalar types

**Error Handling**
- `MvtDecodeError` struct with byte offset tracking
- Implements `std::error::Error`
- Bounds checked on all array accesses ("truncated input")
- Wire type validation per field

#### Test Coverage
- `decode_geometry_tests` module with geometry command parsing tests (lines 939–1027)
- Polygon ClosePath test
- LineString multiple segments test
- Point extraction test

#### Specification Compliance
✓ Protobuf encoding per spec (varint, wire types, tags)
✓ MVT 2.x spec (layer name, version, extent, features)
✓ All geometry commands (MoveTo, LineTo, ClosePath)
✓ Tag/key/value dictionaries
✓ Zigzag integer encoding for coordinates

#### External Dependencies
**ZERO external protobuf libraries** — completely hand-rolled.

---

## 5. Raster Tile (PNG/JPEG/WebP) Decoder

### Status: **External `image` Crate Dependency**

#### Location & Usage
- **File**: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` : line 2456
- **Function**: `upload_tile()`
- **Code**:
  ```rust
  let img = image::load_from_memory(png_bytes)?;
  let rgba = img.to_rgba8();
  let (w, h) = rgba.dimensions();
  let image = RasterImage::rgba8(w, h, Arc::new(rgba.into_raw()));
  ```

#### Implementation
- Uses Rust `image` crate for PNG/JPEG/WebP decoding
- Converts to RGBA8 format
- Wraps in `RasterImage` struct with Arc for sharing

#### Compliance Note
**⚠️ VIOLATES CLAUDE.md line 13**: "You MUST NOT create runtime dependencies on external libraries."
- PNG decoder is **externally sourced**, NOT hand-rolled
- Contrast with MVT decoder: hand-rolled, zero external deps
- Exception from CLAUDE.md line 15 ("You SHOULD use existing libraries as possible to test our implementation") does NOT apply to runtime, only to testing

#### Severity
- **Medium**: Raster tiles are used in production (OSM tiles, Terrarium DEM)
- **Mitigation needed**: Implement hand-rolled PNG decoder (like MVT decoder)
- **Current state**: Functional but policy-violating

---

## 6. Test Fixtures for Tiles

### Fixtures Directory
No dedicated tile fixture directory found.

### Inline Test Data (Hard-Coded)

#### SVG/HTML/JavaScript Test Scenes
| File | Test Data |
|------|-----------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` | GOLDEN_SCENES_JSON: Array with tiled-map scene fixtures including `tileUrlTemplate: "/osm/{z}/{x}/{y}.png"` and `vectorTileUrlTemplate: "/vt/{z}/{x}/{y}.pbf"` |

#### PNG Byte Fixtures
| Function | File | Line | Purpose |
|----------|------|------|---------|
| `test_png_1x1()` | `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` | 3789 | Generates 1×1 PNG byte array for upload tests |
| `solid_terrarium_png()` | `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` | 491 | Generates solid-color Terrarium RGB PNG for elevation testing |
| `gradient_terrarium_png()` | `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` | 616 | Generates gradient Terrarium PNG for elevation testing |

#### Test Usage
- `upload_tile_invalid_png_bytes_returns_err()` → tests PNG decode error handling
- `has_tile_and_has_vector_tile_reflect_uploads()` → tests cache population
- Terrain elevation decoding tests use generated Terrarium PNGs

### Summary
- **No external tile fixture files** (`.pbf`, `.png` in fixtures/)
- **Inline generated fixtures only** (mostly for unit tests)
- **Real tile data** at `.🧬semio/🗺️map/` (osm-tiles, openfreemap-vt) used for integration tests
- **Test scene fixtures** embedded in GOLDEN_SCENES_JSON constant

---

## 7. Offline Behavior & Error Handling

### Tile Fetch Failure Scenarios

#### Missing Tile in Cache (First Visit)
1. `queue_map_tile_fetch_step()` checks `host.has_tile(key)` → miss
2. Calls `reserve_map_tile_fetch()` → queues HTTP request
3. Browser fetches tile proxy: `GET /osm/{z}/{x}/{y}.png`

#### Tile Proxy Cache Miss (Dev Mode)
1. Middleware checks disk cache → miss
2. Fetches upstream (OSM/OpenFreeMap) → `GET https://tile.openstreetmap.org/{z}/{x}/{y}.png`
3. On success: writes to disk, returns to browser
4. On failure (status 404/502 or network error): returns HTTP 404/502

#### JavaScript Receives 404/502
- No retry logic visible in codebase
- Response body empty or error message
- JavaScript calls `sealAssetResponse()` with zero bytes

#### MapHost Upload Failure
- `upload_tile(png_bytes)` fails if PNG decode errors
- Error returned to browser worker via Result type
- Tile remains uncached
- Next frame, render loop checks `has_tile(key)` → still miss → re-requests

#### Rendering Missing Tile
- Render loop calls `visible_tile_cursor()` → lists visible tiles
- For each tile, checks `has_tile()` → false for missing tiles
- **Tile simply not drawn** → blank area on map
- No placeholder image, no gray box, no error marker
- User sees incomplete map

### Retry Behavior
| Scenario | Retry? | Mechanism | Details |
|----------|--------|-----------|---------|
| Network timeout | No | — | Single attempt, if fails → blank |
| 404 upstream | No | — | Cached as "not available" via HTTP response |
| 502 proxy error | No | — | Returned to browser, no recovery |
| PNG decode error | No (implicit) | Lazy re-request | Next pan/zoom, tile re-requested from disk cache or upstream |
| Disk write failure | No explicit | Upstream fetch retried on next request | If disk full, upstream refetched & may fail again |

### Offline (No Network) Behavior
1. If tile exists on disk cache (`.🧬semio/🗺️map/`): served normally
2. If tile not cached:
   - Dev mode: upstream fetch fails → HTTP 502 → blank tile
   - Production (bundle mode): tile not in dist/ → HTTP 404 → blank tile
3. **No fallback, placeholder, or retry** in any mode
4. **User sees blank/incomplete map**

### Summary
- **Failure mode**: Silent blank (no error message, placeholder, or UI feedback)
- **Retry**: None at tile level; implicit retry via user pan/zoom action re-requesting cursor
- **Caching**: Disk cache persists across sessions (dev); baked into dist/ (prod)
- **Infinite loop**: No — single attempt per frame; map simply shows gaps

---

## Architectural Summary

| Aspect | Detail |
|--------|--------|
| **Tile sources** | OSM (raster), OpenFreeMap (vector), AWS Terrarium (DEM) |
| **Tile storage** | Directory-based (z/x/y), not pmtiles/mbtiles |
| **Transport** | HTTP via Vite dev middleware or production asset server |
| **Caching tiers** | Disk (`.🧬semio/🗺️map/`), in-memory MapHost (512-entry LRU) |
| **Async boundary** | WASM ↔ JavaScript, authority mutex-guarded |
| **MVT decoder** | Hand-rolled protobuf, complete |
| **PNG decoder** | External `image` crate (policy violation) |
| **Error handling** | No retry, silent blank tiles |
| **Cancellation** | Via cursor reset or `abortAssetResponse()` |

