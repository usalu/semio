# Wave 3b Surface & 2D Recon

## Target A: Surface Family (W3b)

### Paint Component
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs`

- **LOC:** 1838
- **Main Type:** `RasterHost`

#### RasterHost Fields Classification

| Field | Type | Classification | Reasoning |
|-------|------|-----------------|-----------|
| `camera` | `Camera` | (c) Ephemeral view state | Viewport camera position/zoom set by user interaction (wheel, pan) |
| `viewport` | `Viewport` | (c) Ephemeral view state | Display dimensions and DPR for rendering; interaction-driven |
| `document` | `RasterDocument` | (a) Authoritative state | User-loaded/synced document structure persisted from JSON |
| `images` | `RasterImageCache` | (d) Runtime wiring | GPU cache of decoded images keyed by layer ID |
| `buffers` | `RasterLayerBuffers` | (d) Runtime wiring | HashMap of pixel buffers for paint layers (HashMap<String, Vec<u8>>) |
| `active_utility` | `String` | (c) Ephemeral view state | Current tool (paintBrush, paintEraser, selectMarquee); UI state |
| `brush_size` | `f32` | (c) Ephemeral view state | Brush parameter; user-driven painting state |
| `brush_opacity` | `f32` | (c) Ephemeral view state | Brush transparency; user-driven painting state |
| `hovered_id` | `Option<String>` | (c) Ephemeral view state | UI hover feedback for layer IDs |
| `selected_ids` | `Vec<String>` | (c) Ephemeral view state | User selection for painting target; transient |
| `panning` | `bool` | (c) Ephemeral view state | Pan gesture in-progress flag |
| `painting` | `bool` | (c) Ephemeral view state | Paint stroke in-progress flag |
| `last_paint` | `Option<Point>` | (c) Ephemeral view state | Stroke interpolation state; discarded after release |
| `pan_last` | `Option<Point>` | (c) Ephemeral view state | Pan interpolation state; discarded after release |
| `show_selection_chrome` | `bool` | (c) Ephemeral view state | UI rendering flag for selection overlay visibility |
| `theme_clear` | `Color` | (d) Runtime wiring | Computed checkerboard shades; derived from theme |
| `checkerboard_light_cell` | `u8` | (d) Runtime wiring | Checkerboard fill cache |
| `checkerboard_dark_cell` | `u8` | (d) Runtime wiring | Checkerboard fill cache |

#### RasterHost Mutation Methods

```rust
pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfacePaintError>
pub fn set_size(&mut self, width: u32, height: u32, dpr: f64)
pub fn set_show_selection_chrome(&mut self, enabled: bool)
pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64)
pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64)
pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8)
pub fn pointer_move_screen(&mut self, sx: f64, sy: f64)
pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64)
pub fn set_active_utility(&mut self, utility: &str)
pub fn set_brush_size(&mut self, size: f32)
pub fn set_brush_opacity(&mut self, opacity: f32)
pub fn set_hovered_id(&mut self, id: Option<String>)
pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), FrameworkSurfacePaintError>
pub fn sync_document_json(&mut self, json: &str) -> Result<(), FrameworkSurfacePaintError>
pub fn upload_layer_image(&mut self, layer_id: &str, bytes: &[u8]) -> Result<(), FrameworkSurfacePaintError>
pub fn upload_raster_image_key(&mut self, key: &str, bytes: &[u8]) -> Result<(), FrameworkSurfacePaintError>
```

#### External Consumers
- **Usage:** Paint component is primarily internal to framework; no external plugin consumers found in grep results.

---

### Node-Graph Component
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs`

- **LOC:** 1145
- **Main Type:** `GraphHost`

#### GraphHost Fields Classification

| Field | Type | Classification | Reasoning |
|-------|------|-----------------|-----------|
| `dag` | `DagHost` | (d) Runtime wiring | Delegated to infinite_canvas DAG engine; internal layout cache |
| `catalogue_json` | `String` | (c) Ephemeral view state | Transient catalog UI state; not persisted |
| `controls_json` | `String` | (c) Ephemeral view state | Transient control overlay state |
| `capabilities_json` | `String` | (c) Ephemeral view state | Transient capability advertisement |
| `last_payload_signature` | `u64` | (d) Runtime wiring | Diff/change detection cache for payload comparison |

#### GraphHost Mutation Methods

```rust
pub fn sync_from_payload(&mut self, payload: &NodeGraphScenePayload) -> Result<(), NodeGraphError>
pub fn sync_from_scene_json(&mut self, scene_json: &str) -> Result<(), NodeGraphError>
pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), NodeGraphError>
pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64)
pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64, zoom_gesture: bool)
pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool)
pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool)
pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool)
pub fn set_hover(&mut self, node_id: Option<&str>)
pub fn set_hover_channel(&mut self, node_id: Option<&str>, port_id: Option<&str>)
pub fn align_selection(&mut self, mode: &str) -> Result<(), NodeGraphError>
pub fn set_canvas_theme_dark(&mut self, dark: bool)
```

#### External Consumers
- **Usage:** Graph host is framework-internal; no external plugin consumers identified.

---

### Tiled-Map Component
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`

- **LOC:** 4237
- **Main Type:** `MapHost`

#### MapHost Fields Classification

| Field | Type | Classification | Reasoning |
|-------|------|-----------------|-----------|
| `camera` | `Camera` | (c) Ephemeral view state | User-driven pan/zoom; viewport dependent |
| `viewport` | `Viewport` | (c) Ephemeral view state | Display dimensions |
| `features` | `MapFeatureTables` | (a) Authoritative state | Positions, routes, regions synced from map JSON; user-persisted |
| `tiles` | `MapTileLedger` | (d) Runtime wiring | Cached raster/vector tiles; evictable, bounded LRU |
| `render_mode` | `MapTileMode` | (c) Ephemeral view state | UI toggle: Image/Vector/Combined |
| `vector_style` | `MapVectorStyle` | (c) Ephemeral view state | UI toggle: Colored/Grayscale |
| `forced_lod_id` | `Option<String>` | (c) Ephemeral view state | Pinned LOD band; overrides automatic zoom-based selection |
| `layer_visibility` | `MapLayerVisibility` | (c) Ephemeral view state | Per-layer visibility toggles; UI state |
| `layer_stroke_scale` | `MapLayerStrokeScale` | (c) Ephemeral view state | Per-layer stroke width multipliers; UI styling |
| `events` | `Vec<serde_json::Value>` | (c) Ephemeral view state | Event log drained each frame; interaction feedback |
| `interaction` | `MapInteraction` | (c) Ephemeral view state | Current pan gesture state; transient |
| `theme` | `MapPalette` | (d) Runtime wiring | Color palette; derived from UI theme |
| `selected_positions` | `BTreeSet<String>` | (c) Ephemeral view state | UI selection state for map features |
| `selected_routes` | `BTreeSet<String>` | (c) Ephemeral view state | UI selection state for map features |
| `hovered_kind` | `Option<String>` | (c) Ephemeral view state | Hover feedback (position/route) |
| `hovered_id` | `Option<String>` | (c) Ephemeral view state | Hover feedback entity ID |

#### MapHost Mutation Methods

```rust
pub fn set_map_theme_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn set_size(&mut self, width: u32, height: u32, dpr: f64)
pub fn fit_world_camera(&mut self)
pub fn clamp_camera_to_world(&mut self)
pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64)
pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64)
pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, png_bytes: &[u8]) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn upload_vector_tile(&mut self, z: u32, x: u32, y: u32, pbf_bytes: &[u8]) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn sync_map_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn set_render_mode(&mut self, mode: &str)
pub fn set_vector_style(&mut self, style: &str)
pub fn set_lod_mode(&mut self, mode: &str)
pub fn set_layer_visibility_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn set_layer_stroke_scale_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8)
pub fn pointer_move_screen(&mut self, sx: f64, sy: f64)
pub fn pointer_up_screen(&mut self, sx: f64, sy: f64)
pub fn set_selection_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn set_hover_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError>
pub fn focus_feature(&mut self, kind: &str, id: &str) -> bool
```

#### External Consumers
- **Usage:** Tiled-map is framework-internal; no external plugin consumers identified.

---

## Target B: 2D Drawing Store (Later Wave)

### DrawingStore
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs`

- **LOC:** 1177
- **Architecture:** Content-addressed drawing node engine backed by `EngineCache` (process-local derivative of OS engine host)

#### DrawingStore Fields

| Field | Type | Purpose |
|-------|------|---------|
| `cache` | `EngineCache` | Standalone cache managing OS engine derivations; budgeted at 64MB; holds content-addressed handles |
| `live` | `HashSet<String>` | Set of live handle strings; garbage collection and lifecycle tracking |

#### Key Methods

**Core Operations:**
- `derive_node(&mut self, node: StoredNode) -> Result<DrawingHandle, DrawingError>` – Derives a content-addressed handle via engine, registers in live set
- `register(&mut self, kind: DrawingKind, node: DrawingNode) -> Result<DrawingHandle, DrawingError>` – Registers a new drawing node with identity transform
- `fork(&mut self, source: &DrawingHandle) -> Result<DrawingHandle, DrawingError>` – Creates a copy of a handle (for mutable operations)
- `with_mutated<F>(&mut self, source: &DrawingHandle, mutate: F) -> Result<DrawingHandle, DrawingError>` – Clones, mutates, derives new handle
- `entry(&self, handle: &DrawingHandle) -> Result<StoredNode, DrawingError>` – Reads a stored node from cache

**Lifecycle:**
- `dispose_sync(&mut self, handle: &DrawingHandle)` – Removes handle from live set
- `retain_sync(&mut self, live: &HashSet<String>)` – Garbage-collects unreferenced handles

**Export:**
- `flatten_scene_sync(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError>` – Recursively flattens groups into scene nodes
- `export_svg_sync(&self, handle: &DrawingHandle) -> Result<String, DrawingError>`
- `export_pdf_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError>`
- `export_dwg_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError>`
- `import_dwg_sync(&mut self, data: &[u8]) -> Result<DrawingHandle, DrawingError>`

#### How fork/derive_node/register Work

1. **`register`** creates a `StoredNode` with identity transform (Affine2D::identity(), no fill/stroke/clip, opacity=1.0) and calls `derive_node`
2. **`derive_node`** serializes the StoredNode to JSON bytes, calls `cache.derive()` with the engine ID ("s.2d.drawing"), receives a content-addressed key from the OS engine, converts to hex string handle, and tracks in `live` set
3. **`fork`** retrieves the entry, clones it, and calls `derive_node` again (resulting in a new handle for the same content if unchanged, same handle if content is identical)
4. **`with_mutated`** retrieves, clones, applies a closure mutation, then calls `derive_node` – enables chaining (e.g., translate then set_fill)

#### Relationship to EngineCache

- `DrawingStore` holds a **standalone `EngineCache`** budgeted at 64MB
- Registers `DrawingEngine` (with ENGINE_ID = "s.2d.drawing") once at construction
- `DrawingEngine` is stateless; its `compute()` merely round-trips: serializes StoredNode → deserializes (validates format)
- The cache is content-addressed: same StoredNode bytes always derive the same handle
- **No OS EngineHost integration yet** – the store is a process-local stand-in until host injection

---

### DrawingEngine
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs` (lines 8-18)

```rust
pub struct DrawingEngine;

impl Engine for DrawingEngine {
    const ENGINE_ID: &'static str = "s.2d.drawing";
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> { ... }
}
```

- **Stateless compute:** accepts serialized `StoredNode` (JSON), validates decode, returns as-is (derivation happens at cache layer)
- **No kernel trait implementation** in this file – that lives in `engine/component.rs` (`DrawingKernel` trait)

---

### External Consumers of DrawingStore/DrawingEngine

| Path | Usage |
|------|-------|
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` | Holds static `DRAWING_KERNEL: LazyLock<Mutex<DrawingStore>>`; provides `with_drawing_kernel()` fn for flow evaluation |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` | Flow module draw operators; calls kernel methods (rect, circle, set_fill, export_svg, etc.) |

Both are in the **flow** subsystem for runtime drawing evaluation.

---

### Value Types Requiring Schema Migration

#### Defined in `◻2d/⚙️engine/🦀️component.rs`

| Type | Refs Outside ◻2d | Used By (Sample) |
|------|------------------|------------------|
| `PathSegment` | Yes (~50+) | Draw plugin schemas, PDF/SVG engines, CAD plugin, Studio artifacts |
| `FillStyle` | Yes (~40+) | Draw plugin, PDF/SVG/DXF export, PDF/docx schemas |
| `StrokeStyle` | Yes (~40+) | Draw plugin, PDF/SVG export, CAD interaction spec |
| `GradientStop` | Yes (~10+) | Draw plugin, PDF/SVG export |
| `Affine2D` | Yes (~60+) | Mesh, infinite canvas, draw plugin, layout, CAD, studio artifacts |
| `DrawingNode` | Yes (~20+) | Draw plugin schemas, PDF/SVG engines, studio draw artifact schema |
| `DrawingHandle` | Yes (~30+) | Draw plugin, flow draw extension, studio artifact schemas |
| `DrawingKind` | Yes (~15+) | Draw plugin, studio artifact schemas |
| `DrawingScene` | Yes (~10+) | Draw plugin, studio artifact schemas |
| `DrawingError` | Yes (~15+) | Flow draw extension, studio artifact schemas |
| `LineCap`, `LineJoin` | Yes (~10+) | Draw plugin, PDF/SVG export |
| `Vec2` | Yes (~100+, but generic) | Mesh, draw, CAD, layout, animations |

**Conclusion:** These are **heavily distributed** across plugin artifact schemas (draw, PDF, SVG, DXF, studio semio), flow operators, and framework modules (mesh, infinite canvas). **Dead code risk: LOW** – all are actively consumed.

---

### Re-exports in glue.rs

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/📦️glue.rs`

```rust
#[path = "../../⚙️engine/🦀️component.rs"]
pub mod engine;
pub use engine::*;  // Re-exports ALL engine types (Vec2, PathSegment, FillStyle, DrawingKernel, etc.)

#[path = "../../🗄️store/🦀️component.rs"]
mod store;
pub use store::{DrawingEngine, DrawingStore};  // Only DrawingEngine/DrawingStore, not StoredNode
```

**Key finding:** The `engine::*` re-export **publishes all value types** (PathSegment, FillStyle, Affine2D, GradientStop, etc.) to consumers of `semio_framework_2d` crate.

---

## Summary

### Surface Family (Paint, Graph, Map)
- **All three are ephemeral view-state holders** with runtime wiring for GPU/canvas resources
- **No external plugin consumers** – framework-internal only
- **State is overwhelmingly (c) or (d)** – view state and rendering caches, not persistent user data
- **Single authoritative source** in tiled-map (`features`) is synced from JSON; other surfaces load documents once

### 2D Drawing Store
- **Parallel non-artifact store** with **content-addressed derivation** via OS engine cache
- **Would be deleted by event-sourcing:** kernel operations become events, fork/derive_node/register become event handlers, StoredNode becomes event-derived state
- **Heavy downstream dependence:** Value types are re-exported to 50+ files across plugins and framework modules
- **No dead code:** Every type is actively used in schemas, engines, and operators

### Migration Risk
- **Value types (PathSegment, FillStyle, etc.) CANNOT be deleted locally** – must move to shared schema layer before ◻2d module dissolves
- **DrawingStore itself is the only true "local artifact"** – the engine is a compute stub; the store is the kernel implementation


---

# ⚠️ COORDINATOR CORRECTIONS (appended after independent re-measurement)

The per-field doctrine classification above is the valuable part of this report and is retained.
**Two factual claims in it are WRONG and are corrected here.** Do not act on the originals.

## CORRECTION 1 — "No external consumers" is false for all three surface modules

The report states Paint, Node-Graph and Tiled-Map each have "no external consumers". Re-measured
per type, excluding `🗺️surface/` itself and `node_modules`:

| Type | External hits | Consumers (full paths) |
|---|---|---|
| `RasterHost` | 3 | `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs` ×2 · `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🦀️component.rs` |
| `RasterSession` | 1 | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` |
| `GraphHost` | 7 | `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` ×6 · `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🌉️wasm/🦀️component.rs` |
| `MapHost` | **18** | `📺️renderer/…/EngineCanvas/🧊️component.rs` ×7 · `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🗺️maphost/🦀️component.rs` ×7 · `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/⚙️engine/🦀️component.rs` ×2 · `📺️renderer/…/Scenes/🧊️component.rs` · `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs` |

**Consequence: W3b is real dissolution work, NOT a deletion.** This matters because the intended W2
exemplar (`🖥️platform`) *was* genuinely consumer-free and was dropped for that reason — a false
"no consumers" reading here would have wrongly deleted live, plugin-facing code. Every one of these
modules is reached from `📺️renderer`'s engine elements and from a real plugin app.

## CORRECTION 2 — the `◻2d` consumer surface is narrower AND wider than reported, in different places

- **`DrawingStore`/`DrawingEngine` have only TWO external consumers**, not a broad surface:
  `💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` and `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`.
  **Good news for the deletion** — the parallel store's blast radius is two files.
- **The value types are the wide surface**, and the report is right that they must move first:
  `PathSegment` 27 files · `FillStyle` 12 · `StrokeStyle` 11 · `DrawingNode` 1 · `Affine2D` **0**
  (all counts outside `🔨️modules/◻2d/`). `Affine2D` having zero external users contradicts its
  inclusion in the "must migrate" list — verify before moving it.
- ⚠️ **`PathSegment`'s 27 files are NOT necessarily one type.** UCAS's design doc records "3
  path-segment vocabularies" as a known duplication. Treat 27 as a *search* result, not a census,
  and disambiguate by definition site before planning any migration. (Third instance of this trap
  today: `CollectionMutation` ×2, `Vec3` ×3.)
- The report's claim that framework `📦️glue.rs` re-exports all `◻2d` engine types via
  `pub use engine::*` **could not be confirmed** — a grep of `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
  for 2d/drawing symbols returns only dwg conversion functions at :53. Any such re-export is likely
  in `◻2d`'s OWN package glue, not the framework's. **Verify before relying on it.**

## Standing lesson

This is the second recon in this ticket whose factual claims needed independent re-measurement
(the brep recon proposed replaying `OpRecorder`, which is provenance, not a command log). The
per-field/architectural reasoning in both was good; the *counts and consumer claims* were not.
**Treat a scout's judgment as input and its measurements as hypotheses.** Re-run any number you are
about to plan against.
