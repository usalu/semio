# Infinite Canvas Core Architecture — Research

**Project**: Semio | **Ticket**: GIS-MAP-END-TO-END | **Research Date**: 2026-08-29

---

## 1. Camera & Viewport Model

### Structures
- **Camera** (🖼️canvas/🦀️component.rs:1073–1083)
  - Fields: `x: f64, y: f64, zoom: f64`
  - Represents world-space camera center + zoom scalar
  - Default: x=0, y=0, zoom=1.0

- **Viewport** (🖼️canvas/🦀️component.rs:1086–1110)
  - Fields: `width: u32, height: u32, dpr: f64`
  - Device pixel ratio scales logical viewport to physical GPU surface
  - `set_size()` clamps width/height to ≥1, dpr to ≥1.0
  - `physical_size()` returns (width × dpr, height × dpr) for GPU allocation

### World ↔ Screen Transforms
- **world_to_screen** (1116–1118): `Point { (p.x - cam.x) × zoom + vp.width/2, (p.y - cam.y) × zoom + vp.height/2 }`
- **screen_to_world** (1120–1122): `Point { (s.x - vp.width/2) / zoom + cam.x, (s.y - vp.height/2) / zoom + cam.y }`
- **camera_content_affine** (1124–1127): Builds 2×3 affine for Vello scene rendering: scale by zoom, translate by camera origin

### Zoom Clamping
- **Constants** (1069–1070): `CANVAS_CAMERA_ZOOM_MIN` and `CANVAS_CAMERA_ZOOM_MAX` from `ui_styling::metrics::camera`
- **clamp_zoom** (1112–1114): Clamps zoom to [MIN, MAX] range
- Zoom min/max values are defined in the UI styling module (not inlined here)

---

## 2. Panning Implementation

### Initiation
- Pan mode is **NOT** initiated in the base canvas/board component
- Pan is handled at a higher layer (observed in DAG/directed port components: `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:11896, 11951`)
- Base board component assumes Pan mode is already set via `InteractionMode::Pan`

### Pan Handling During Pointer Move
**Method**: `pointer_move_screen` (🎲️board/🦀️component.rs:959–1073)
- **Case**: `InteractionMode::Pan { start_screen, cam_x, cam_y, zoom }` (963–968)
  ```rust
  let dx = (screen.x - start_screen.x) / zoom;
  let dy = (screen.y - start_screen.y) / zoom;
  self.set_camera(cam_x - dx, cam_y - dy, zoom);
  self.interaction = InteractionMode::Pan { start_screen, cam_x, cam_y, zoom };
  ```
- Screen-space delta is divided by zoom to scale movement in world space
- Camera is updated via `set_camera()` at every move event

### Input Events Handled
**Pointer Events** (🎲️board/🦀️component.rs):
- `pointer_down_screen` (886): screen/world coords, button (0=primary, 2=secondary), shift/ctrl/alt
- `pointer_move_screen` (959): same coords + modifiers
- `pointer_up_screen` (1079): same contract
- Comment (884): *"Primary (button == 0) selects and may start drag / marquee / edge-draw. Secondary (button == 2) only updates selection/hover for context menus — never moves nodes."*
- **No explicit middle-button (button == 1) handling in base board** — Pan initiation deferred to host

**Wheel/Trackpad/Pinch**:
- **wheel_screen** (🖼️canvas/🦀️component.rs:1129–1137)
  - Parameters: camera, viewport, screen coords (sx, sy), delta_y (signed scroll delta)
  - Logic:
    ```rust
    zoom_factor = if delta_y < 0.0 { WHEEL_ZOOM_IN_FACTOR } else { WHEEL_ZOOM_OUT_FACTOR };
    next_zoom = clamp_zoom(camera.zoom * zoom_factor);
    world_before = screen_to_world(camera, viewport, screen);
    camera.x = world_before.x - (sx - vp.width/2) / next_zoom;
    camera.y = world_before.y - (sy - vp.height/2) / next_zoom;
    camera.zoom = next_zoom;
    ```
  - **Cursor-anchored**: World point at cursor stays fixed during zoom
  - Discrete steps: multiplication by zoom factors (not continuous)
  - Constants `WHEEL_ZOOM_IN_FACTOR` and `WHEEL_ZOOM_OUT_FACTOR` from ui_styling
- **No smoothing/inertia/animation in core** — host may layer this

**Keyboard**: Not handled in canvas/board core (would be host responsibility)

---

## 3. Zoom Implementation

### Discrete vs Continuous
- **wheel_screen**: Discrete steps via factors (see above)
- No continuous zoom via trackpad delta magnitude observed in core
- No easing/spring animations in core (host may implement)

### Cursor Anchoring
- **wheel_screen** anchors zoom center to cursor by:
  1. Map screen cursor to world before zoom
  2. After zoom, adjust camera x/y so same world point remains under cursor
  3. Formula: `cam.{x,y} = world_before.{x,y} - (s{x,y} - vp.{width,height}/2) / next_zoom`

---

## 4. LOD (Level of Detail) Module

**Location**: 🖼️canvas/🦀️component.rs:1141–1191

### Struct Definitions
```rust
pub struct Lod {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub max_zoom: f64,  // Threshold: LOD selected when camera.zoom < max_zoom
}

pub struct LodScale {
    pub lods: &'static [Lod],  // Sorted by zoom threshold
}
```

### LOD Selection
- **resolve_index** (1157–1164): Binary search through lods; returns first index where `zoom < max_zoom`
  - Falls back to last LOD if zoom exceeds all thresholds
- **resolve** (1166–1168): Returns `&Lod` for given zoom
- **index_of** (1170–1172): Look up LOD by id string

### Text Scaling Within LOD Band
- **band_label_screen_px** (1176–1178): Fixed screen px for a band; lookup or fallback
- **band_floor_zoom** (1181–1183): Lower zoom bound for a band (previous band's max_zoom or zoom_min)
- **lod_band_label_screen_px** (1186–1190):
  ```rust
  z = zoom.max(LOD_ZOOM_FLOOR);
  floor = band_floor_zoom.max(LOD_ZOOM_FLOOR);
  base_screen_px * z / floor
  ```
  - Scales label size proportionally with zoom inside a LOD band
  - Clamps to `LOD_ZOOM_FLOOR` to avoid division by near-zero
  - **Pattern**: Discrete LOD bands; within each band, continuous label scaling

### No Tile/Grid Integration
- LOD module is orthogonal: selects detail level, does not manage tile loading or culling
- Host/MapHost layer would use LOD to decide detail level for rendering

---

## 5. Render Loop & Frame Scheduling

### Render Entry Point
**CanvasContent trait** (🖼️canvas/🦀️component.rs:1248–1251):
```rust
pub trait CanvasContent {
    fn build_scene(&self) -> Scene;  // Rebuild Vello scene on demand
    fn clear_color(&self) -> Color;   // Raster clear color
}
```

### GPU Rendering (WASM target)
**CanvasGpuSession** (🖼️canvas/🦀️component.rs:1264–1345):
- `create_canvas_surface()`: Async WebGPU bring-up
- `render_frame()` (1311–1343):
  - Renders Vello scene to texture via `renderer.render_to_texture()`
  - Blits to wgpu surface; presents
  - Retry loop (3 attempts) handles surface stale/timeout; returns Ok() gracefully
  - **No dirty tracking**: Called on demand by host (likely every frame or on invalidation)

### Dirty Tracking
- **None in core**: Canvas does not track which regions changed
- **CanvasContent.build_scene()** is called fresh each frame (or host optimizes)
- **Scene.retirement_step()** (254–271): Incremental deallocate (pop from scene encoding vectors)
  - Not a dirty-tracking optimization; just gradual resource cleanup

### Frame Scheduling
- **No internal frame loop in core**: Canvas is passive
- **Host responsibility**: Call render_frame(), manage refresh rate, input dispatch
- Render-on-demand model: No scheduled redraw; host drives via CanvasGpuSession

---

## 6. Tile & Spatial-Index Machinery

### Quadtree / Tile Grid
- **None found in core** canvas/board modules
- Search results: No `quadtree`, `tile_grid`, `spatial_index`, `culling`, or related keywords

### Bounding Box Utilities (Selection-Only)
**canvas::geom_sel** module (🎲️board/🦀️component.rs:261–264):
- `WorldBox`: world-space bounding rect with min_x, min_y, max_x, max_y
- Marquee selection tests: `polygon_contains_world_box()`, `world_boxes_overlap()`, etc.
- **Not** used for render culling; only for hit-testing

### Caching
**RasterImageCache** (🖼️canvas/🦀️component.rs:1209–1223):
- HashMap<String, Arc<RasterImage>>
- Simple keyed cache; get() and insert()
- No eviction policy, no TTL (unbounded size hazard)

### Render Culling
- **None in core**: Vello scene is rendered in full each frame
- Host/MapHost layer would implement culling (e.g., only upload tiles for visible zoom/region)

---

## 7. Public API Surface (Traits & Host Integration)

### CanvasExtension Trait
**Location**: 🖼️canvas/🦀️component.rs:1778–1780
```rust
pub trait CanvasExtension: Send + Sync {
    fn extension_id(&self) -> &str;
}
```
- Minimal trait: only identity required
- Extensibility: domain logic lives in concrete implementations

### CanvasEngine<E>
**Location**: 🖼️canvas/🦀️component.rs:1783–1791
```rust
pub struct CanvasEngine<E: CanvasExtension> {
    pub extension: E,
}
impl<E: CanvasExtension> CanvasEngine<E> {
    pub fn new(extension: E) -> Self { Self { extension } }
}
```
- Generic wrapper; no required methods
- **Semantic**: Hosts implement CanvasExtension to plug domain-specific behavior

### GraphExtension (Board)
**Location**: 🎲️board/🦀️component.rs:94
```rust
pub trait GraphExtension: canvas::CanvasExtension {}
```
- Marker trait inheriting CanvasExtension
- No additional methods in base board

### GraphEngine<P, D> (Core Retained State)
**Location**: 🎲️board/🦀️component.rs:485–612
```rust
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub nodes: BTreeMap<NodeId, Node>,
    pub edges: BTreeMap<EdgeId, GraphEdge<P::Endpoint>>,
    pub handles: BTreeMap<HandleId, Handle>,
    pub selection: Selection,
    pub preselect: Selection,
    pub events: Vec<BoardEvent>,
    pub interaction: InteractionMode,
    pub hover: Option<u64>,
    // + internals: drag_start_positions, area_points, proximity_connection, …
}
```

**Required/Provided Methods** (excerpt):
- `set_camera(x, y, zoom)`: Setter
- `create_node(id, x, y, radius, draggable)`: Add circle node
- `create_rect_node(id, x, y, width, height, draggable)`: Add rectangle node
- `pointer_down_screen(sx, sy, wx, wy, button, shift, ctrl, alt)`: Mouse/touch down
- `pointer_move_screen(...)`: Mouse/touch move
- `pointer_up_screen(...)`: Mouse/touch up
- `render_snapshot()`: Export edges + node positions + pending edge for host rendering
- `drain_events()`: Pull queued BoardEvents (HoverChanged, NodeMoved, SelectionChanged, …)

### Board Events
**Location**: 🎲️board/🦀️component.rs:130–137
```rust
pub enum BoardEvent {
    HoverChanged { id: Option<u64> },
    NodeMoved { id: NodeId, x: f64, y: f64 },
    EdgeConnected { id: EdgeId, source: HandleId, target: HandleId },
    EdgeRemoved { id: EdgeId },
    SelectionChanged { edge_ids, handle_ids, node_ids },
    PreselectChanged { edge_ids, handle_ids, node_ids, removed_* },
}
```
- Events queued per interaction; host drains via `drain_events()`

### Host Responsibilities (Inferred)
1. **Input Dispatch**: Call pointer_*_screen() with screen/world coords
   - Host converts pointer events to world space (requires camera + viewport)
2. **Rendering**: Call render_snapshot(), rebuild scene, render via Vello
3. **Pan/Zoom Control**: Initiate Pan mode and call wheel_screen()
4. **Event Handling**: Drain events, update UI/model accordingly
5. **Spatial Queries**: e.g., hit-testing beyond board's simple hit_test()

---

## 8. Known Performance Hazards

### Per-Frame Allocations
- **RenderSnapshot** (🎲️board/🦀️component.rs:1148–1166):
  ```rust
  pub fn render_snapshot(&self) -> RenderSnapshot {
      let mut snapshot = RenderSnapshot::default();
      for node in self.nodes.values() { /* clone nodes */ }
      for edge in self.edges.values() { /* compute curves */ }
      snapshot  // Returned; allocates Vec for every edge/node
  }
  ```
  - Allocates new Vec for edges, handles, nodes on every call
  - No pooling or incremental update mechanism
  - Hazard: O(n) allocation churn per frame for large graphs

### Unbounded Caches
- **RasterImageCache** (🖼️canvas/🦀️component.rs:1209–1223):
  - No size limit; HashMap grows without bound
  - Hazard: Long-lived canvases with many unique raster images → memory leak

### O(n) Scans
- **Selection marquee tests** (🎲️board/🦀️component.rs:410–462):
  - `selection_contains_edge_curve()` samples edge at fixed 24 points, then tests all against polygon
  - Hazard: 100 edges × 24 samples × O(polygon) = heavy per-frame cost during marquee drag
- **Node proximity snapping** (inferred): Tests all nodes against anchor during edge draw
  - Hazard: O(nodes) per pointer move

### String Formatting in Hot Paths
- **Icon resolution** (🖼️canvas/🦀️component.rs:1448–1505):
  - `decode_icon()` parses strings (shortcodes, URLs, SVG, math notation)
  - Called on demand (not known if cached)
  - Hazard: Frequent icon updates without caching

### Blocking I/O on Render Thread
- **SVG Parsing** (🖼️canvas/🦀️component.rs:792–801, 922–929):
  - `usvg::Tree::from_str()` runs synchronously in pointer move / marquee handling
  - Hazard: Large SVG icons or labels block render thread

### No LOD Culling
- **Vello scene rendered in full**: No tile-based culling
- Hazard: Rendering 1M-node graphs at zoomed-out scale → full rasterization cost

### SceneRetirement (Soft Hazard)
- **OpaqueSceneRetirementRegistry** (🖼️canvas/🦀️component.rs:195–248):
  - Fixed 1024-slot capacity; wrapped indices
  - Hazard: If > 1024 concurrent scenes queued for retirement, registry faults (returns None)
  - Comment (245–248): `opaque_scene_retirement_status()` monitors; host must check

---

## Summary

| Aspect | Finding |
|--------|---------|
| **Camera Model** | Simple struct: x, y, zoom (world + scalar) |
| **Viewport** | Logical + DPR → physical GPU surface size |
| **Zoom** | Cursor-anchored, discrete steps via factors; no inertia |
| **Pan** | Initiated by host (not in base board); handled in pointer_move |
| **LOD** | Discrete bands; within-band continuous label scaling |
| **Render Loop** | Host-driven (no internal scheduler); CanvasContent trait pulls scene/color |
| **Spatial Index** | None; bounding boxes only for selection |
| **Caching** | Unbounded HashMap for rasters; no render caching |
| **API Surface** | CanvasExtension + GraphEngine traits; events queued for host |
| **Perf Hazards** | Per-frame allocations, O(n) scans, unbounded caches, blocking SVG parse, no culling |

---

**Next Steps for GIS-MAP-END-TO-END**:
- Implement MapHost as CanvasExtension (plug styling, LOD rules, tile loading)
- Cache render output per LOD band to reduce per-frame scene rebuild
- Add spatial index (quadtree) for efficient node/edge culling at zoom-out
- Offload SVG/label parsing to worker thread or cache parsed trees
- Implement tile/image streaming pipeline with viewport-relative fetching

