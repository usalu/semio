# Tiled Map Renderer Path: End-to-End Analysis

## 1. Element Contract & State Transmission

**Props/Schema** (`TiledMapHost:565`):
- `TiledMapScene` via `node.tiledMap`
  - `tileUrlTemplate`, `vectorTileUrlTemplate`: URL templates with `{z}/{x}/{y}` placeholders
  - `renderMode`: "image" | "vector" | "combined"
  - `vectorStyle`: "colored" | "figureGround" | "invertedFigure"
  - `cameraJson`: `{"x": number, "y": number, "zoom": number}` (serialized World Mercator coordinates)
  - `selectionJson`, `hoverJson`: feature IDs as JSON arrays
  - `mapFixtureJson`: positions, routes, regions fixture data

**State Transmission** (`TiledMapHost:275-296`):
- **Rust→React**: MapWasmSession returns JSON strings, NOT full state re-serialization
  - `cameraJson()`: returns `{"x": f64, "y": f64, "zoom": f64}` (line 3744-3747)
  - `visibleTilesJson()`: returns array of `{z, x, y, key}` (line 3749-3752)
  - Tile visibility polled via `session.visibleTilesJson()` only when key changes (line 437)
  - Input: MapRenderer polls tiles every frame via `pollVisibleTilesForRefresh()` (line 433-449)

**Tile Caching** (`MapRenderer:253-256`):
- `tileCache: Map<String, ArrayBuffer>` (raster PNG bytes)
- `vectorTileCache: Map<String, ArrayBuffer>` (PBF protobuf bytes)
- Keys are tile Z/X/Y strings: `"z/x/y"` format (not hierarchical)
- Tiles cached per-session lifetime, cleared only on LOD mode change (line 309-317)
- **NO full map re-serialization per frame** — only visible tile key strings compared (O(1) string comparison)

## 2. wgpu Tile Drawing Architecture

**Tile Upload Path** (`upload_tile:2455-2461`):
```rust
pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, png_bytes: &[u8]) {
    let img = image::load_from_memory(png_bytes)?;  // PNG decode
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let image = RasterImage::rgba8(w, h, Arc::new(rgba.into_raw()));
    self.tiles.tile_images.insert(tiles::tile_key(z, x, y), Arc::new(image));
}
```
- Each tile stored as `Arc<RasterImage>` (RGBA8 buffer)
- Keys inserted as strings like `"3/4/5"`
- No GPU texture atlas — individual CPU-side image objects

**Rendering Path** (`build_render_scene:3496-3505`, `render_frame_gpu:3544-3548`):
```rust
fn render_frame_gpu() {
    self.host.prepare_visible_tiles();
    let scene = self.host.build_render_scene();  // Rebuilds full Vello scene
    self.gpu.render_frame(&scene, canvas::canvas_content::CanvasContent::clear_color(&self.host))
}

fn build_vector_scene() {
    let mut scene = Scene::new();
    match self.render_mode {
        MapTileMode::Image | MapTileMode::Combined => self.append_tiles(&mut scene),
        MapTileMode::Vector => {}
    }
    // ... append vector tiles, regions, routes, positions
}
```

**Tile Rendering** (`append_tiles:3344-3365`):
- Iterate all cached tile images (line 3349)
- Sort by (Z, X, Y) for back-to-front ordering (line 3359)
- For each tile, compute world-space `Rect` via `tile_world_rect(z, x, y)` (line 3353)
- Compute affine transform `tile_raster_affine(rect, img.width, img.height)` → screen-space transform
- Draw via `raster::draw_image_arc(scene, &img, aff)` — **per-tile quad with affine**
- **No texture atlasing**: each tile is separate `RasterImage` object → separate draw call in Vello

**Reuse Strategy**:
- Tiles are Arc-cloned from cache per frame if key exists (line 3357)
- Same Arc reused every frame unless new tile uploaded
- Texture upload happens at raster decode time (`RasterImage::rgba8`), cached in `Arc`

## 3. Input Plumbing

**DOM→Rust Path** (`TiledMapHost:925-1072`, `MapHost:2492-2527`):
- **Wheel**: `applyWheelZoom` → `session.wheelScreen(sx, sy, deltaY)` (line 873)
  - Zoom factor: 1.12× per notch, 2.5× multiplier if Ctrl held (line 871)
  - No throttling: every wheel event calls `wheelScreen()` + `scheduleRefreshTiles()`
  
- **Pointer Down/Move/Up**: `pointerDown/Move/UpScreen` method calls (lines 943, 953, 991)
  - Middle button triggers pan: `pointerDownScreen()` + `beginContinuousInteraction("pan")` (line 946)
  - `pointerMoveScreen()` called per move event (no coalescing) (line 953)
  - **Continuous RAF during pan**: `beginContinuousInteraction()` keeps scheduler running (line 376-382)
  - Pan ends on `pointerUp()` → `endContinuousInteraction()` + trailing 250ms window (line 992, WasmSessionLoader:57)

- **Throttling/Coalescing**:
  - **No per-event throttling** — all events directly call Rust methods
  - **RAF batching** via `createDemandFrameScheduler()` (line 537-543):
    - `invalidate()` schedules next frame, trailing window = 250ms
    - During pan, continuous RAF runs every frame while `beginContinuous()` active
  - Tile refresh debounced 120ms after visible key change (line 64, 406)

## 4. Renderer Backend Selection

**Canvas Attachment** (`MapSession.attach_canvas:3571-3592`):
```rust
let (render_ctx, renderer, surface) = 
    canvas::gpu_session::CanvasGpuSession::create_canvas_surface(
        canvas.clone(), pw, ph).await?;
```
- Async GPU surface creation via `CanvasGpuSession::create_canvas_surface()`
- Calls wgpu backend directly (path: `infinite/canvas/gpu_session/CanvasGpuSession`)
- **No canvas2d fallback** in current code
- Dev server uses wgpu only: `GraphWasmCanvas` (react-renderer line 80-218) runs rAF loop with `session.renderFrame()`

## 5. Text/Label Rendering Path

**Font Assets** (`text:496`, `usvg_options_map_labels:844-851`):
- TTF: `MapLabelSans.ttf` embedded at `infinite/canvas/assets/🔤️MapLabelSans.ttf` (line 496)
- Loaded via `usvg::fontdb` (line 846)
- No Noto Color Emoji in current map rendering (only MapLabelSans)

**Label Rendering** (`append_label:980-1014`):
```rust
pub fn append_label(scene: &mut Scene, label: &str, origin: Point, px: f64, 
                    fill: Color, halo: Color) {
    let svg = format!(r##"<svg ...><text font-family="{family}" font-size="{px}" 
                             stroke="{halo}" paint-order="stroke">{text}</text></svg>"##);
    let tree = usvg::Tree::from_str(&svg, usvg_options_map_labels())?;
    render_svg_tree_literal(&mut label_scene, &tree);
    scene.append(&label_scene, Some(aff));
}
```
- **Per-label SVG generation** at render time (line 989-1001)
- Parses SVG string → usvg tree (line 1002)
- Renders tree to Vello scene with halo stroke effect (line 1011-1012)
- Label size/scale determined by LOD (line 3441)
- **Per-position per frame**: called in `append_positions()` loop (line 3473)

**Glyph Rasterization**:
- Vello handles glyph rasterization from usvg tree
- No separate glyph atlas; glyphs rendered as SVG paths

## 6. Performance Hazards

### Critical Issues

1. **Full Scene Rebuild Every Frame** (`build_render_scene:3496`)
   - `Scene::new()` called every `renderFrame()` (line 3480, 3546)
   - All tiles, vector geometry, routes, positions re-appended each frame
   - Vello will re-tessellate all paths/strokes each frame

2. **Tile Cache Linear Scan** (`append_tiles:3349`)
   - Loop over ALL cached tiles, then filter visible (line 3349)
   - No spatial index: O(n_cached_tiles) per frame
   - Max cache: 512 entries (line 94), all scanned every frame

3. **Text Label SVG Parsing Per-Position Per Frame** (`append_positions:3442-3476`)
   - `text::append_label()` called in loop over `features.positions` (line 3442)
   - **SVG generation + usvg parsing inside render loop** (line 989-1002)
   - Regex escaping for XML attributes (line 854)
   - Charset iteration in label_extent (line 874) — O(str_len) per label
   - **No memoization**: same label string parsed fresh every frame

4. **Tile Key Parsing Every Poll** (`pollVisibleTilesForRefresh:433-449`)
   - `parseVisibleTilesJson()` called every frame (line 85-90)
   - JSON.parse() per frame, even if key unchanged (compared after parse)

5. **No Texture Reuse / Atlas**
   - Each tile is separate `Arc<RasterImage>` → separate Vello image object
   - No GPU texture reuse between frames
   - 256×256 tile = 262KB RGBA8 per tile, × 512 cache entries = 128MB peak CPU RAM for tile decompressed data

### Moderate Issues

6. **React State Updates on Every pointerMove**
   - Line 954: `mirrorSessionCameraToReact()` called on every `pointerMove` event
   - Dispatches camera action → React state update → potential layout thrash
   - Could be batched to end of pan only (currently done on pointerUp, line 994)

7. **useMemo Coverage Gaps**
   - `positionMetaById` memoized (line 592) ✓
   - `hoveredFeature` memoized (line 593) ✓
   - But `selectionMethod`, `dispatch` not memoized — recreated on every render
   - Pointers in event listeners capture stale versions (refs hold current, lines 679-682)

8. **Missing Memoization on Callbacks**
   - `clampCamera`, `mirrorSessionCameraToReact`, etc. not wrapped in useCallback
   - Passed as dependencies to useEffect (line 791) — recreated on every render

---

## Summary

**State Path**: TiledMapScene (React) → `MapWasmSession` (WASM) → `MapHost` (Rust), with **no full re-serialization per frame** — only visible tile keys compared.

**Rendering**: Vello-based vector scene building, per-tile raster quads via affine transforms, **NO texture atlas**. Each frame **rebuilds entire Scene**, parse-draws all cached tiles, SVG-renders all label texts.

**Input**: RAF-batched via demand scheduler; continuous RAF during pans; no event throttling.

**Top Perf Bottleneck** (#1): Full Scene rebuild + all-tile linear scan + label SVG parsing, every frame. Tile cache keys are not hierarchical (misses mipmap reuse). Text rendering has no glyph cache.
