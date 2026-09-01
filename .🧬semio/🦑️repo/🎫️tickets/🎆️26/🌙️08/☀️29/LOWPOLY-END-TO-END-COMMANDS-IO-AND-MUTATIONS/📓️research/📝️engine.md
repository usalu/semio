# Lowpoly Engine Exhaustive Mapping

## 1. Session/State Type

**Primary Type: `LowpolyDocument`** (`✏️editor/⚙️engine/🦀️component.rs:79`)

```rust
pub struct LowpolyDocument {
    snapshot: LowpolySnapshot,           // Persisted document projection
    active_object_id: String,            // Current active object identifier
    selection: LowpolySelection,          // Ephemeral vertex/edge/face selection
    meshes: Vec<HalfedgeMesh>,          // Live half-edge mesh kernels (1:1 with objects)
    next_object_serial: u32,             // Serial counter for object ID generation
    mesh_workspace: HashMap<String, String>, // Session-local mesh JSON cache
}
```

**Supporting Types:**
- `LowpolySnapshot` - Persisted schema (🧬️schema/📸️snapshot/🦀️component.rs)
  - `objects: Vec<LowpolyObject>` - array of mesh objects with transforms, paint layers
  - `schema: String` - constant "lowpoly.document"

- `LowpolySelection` (🦀️component.rs:170)
  - `mode: String` - "mesh", "vertex", "edge", or "face"
  - `ids: Vec<u32>` - target element IDs (ephemeral, not persisted)

- `LowpolyObject` (🦀️component.rs:103)
  - `id: String` - unique object identifier
  - `name: String` - display name
  - `transform: LowpolyTransform` - position, rotation, scale
  - `smooth_shading: bool` - shading mode flag
  - `mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>` - content-addressed mesh handle
  - `paint_layers: Vec<LowpolyPaintLayer>` - stacked paint layers

- `LowpolyPaintLayer` (🦀️component.rs:67)
  - `name: String`
  - `visible: bool`
  - `opacity: f32` (0.0 to 1.0)
  - `blend_mode: String` - "normal", "multiply", etc.
  - `pixels: Vec<u8>` - RGBA buffer (1024x1024x4 bytes, base64 persisted)

**Document State Mutation:**
The compute session is ephemeral—never persisted. The flow:
1. Caller creates `LowpolyDocument` from a persisted `LowpolySnapshot` + session-local `mesh_workspace` cache
2. Handler mutates `LowpolyDocument` in place via geometry/paint operations
3. Handler reads back mutated `mesh_workspace` and calls `sync_meshes_to_snapshot()` to re-derive mesh handles
4. Caller commits the mutated `snapshot` as a mutation to the artifact store

---

## 2. Geometry Operations Inventory

### **Session Construction & Lifecycle**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `new` | `fn(LowpolySnapshot, HashMap<String, String>) -> Result<Self, LowpolyCoreError>` | 97 | Construct session from persisted snapshot; fails closed if mesh cache missing/stale |
| `with_context` | `fn(LowpolySnapshot, String, LowpolySelection, HashMap<String, String>) -> Result<Self, LowpolyCoreError>` | 102 | Construct with explicit active object and selection |

### **State Accessors & Queries**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `snapshot` | `fn(&self) -> &LowpolySnapshot` | 117 | Immutable projection access |
| `snapshot_mut` | `fn(&mut self) -> &mut LowpolySnapshot` | 121 | Mutable projection access |
| `mesh_workspace` | `fn(&self) -> &HashMap<String, String>` | 113 | Live mesh JSON cache for caller merge-back |
| `active_object_id` | `fn(&self) -> &str` | 125 | Query current active object ID |
| `selection` | `fn(&self) -> &LowpolySelection` | 129 | Query ephemeral selection |
| `active_index` | `fn(&self) -> Option<usize>` | 190 | Resolve active object to snapshot index |
| `object_index` | `fn(&str) -> Result<usize, LowpolyCoreError>` | 208 | Resolve object ID to snapshot index |
| `mesh_at` | `fn(&self, usize) -> Option<&HalfedgeMesh>` | 204 | Query kernel mesh at snapshot index |

### **Mesh I/O & Serialization**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `reload_meshes` | `fn(&mut self) -> Result<(), LowpolyCoreError>` | 160 | Deserialize all `HalfedgeMesh` from session cache; fails if cache stale |
| `sync_meshes_to_snapshot` | `fn(&mut self) -> Result<(), LowpolyCoreError>` | 181 | Serialize mutated meshes back to cache; re-derive content-addressed child handles |
| `tessellate_transfer_json` | `fn(mesh: &HalfedgeMesh) -> Result<Value, LowpolyCoreError>` | 336 | Export single mesh to WebGL-friendly tessellation (positions, indices, normals, UVs, element IDs) |
| `tessellate_all_json` | `fn(&self) -> Result<String, LowpolyCoreError>` | 352 | Serialize all objects + tessellations as JSON (viewmodel export for 3D viewer) |
| `lowpoly_mesh_from_document` | `fn(&Value, &HashMap<String, String>) -> Result<MeshData, String>` | 402 | Export active object to media-interchange `MeshData` struct |

### **Primitive Generation**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `add_primitive` | `fn(&mut self, kind: &str) -> Result<String, LowpolyCoreError>` | 304 | Create & append primitive (box, plane, cylinder, cone, ico_sphere); returns new object ID; auto-activates |

### **Selection Operations**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `apply_selection` | `fn(&mut self, mode: &str, ids: Vec<u32>)` | 241 | Set selection mode + IDs (mode normalized: "object" → "mesh") |
| `normalize_selection_mode` | `fn(mode: &str) -> String` | 233 | Normalize mode strings (object→mesh passthrough) |
| `selected_vertex_ids` | `fn(&self) -> Vec<VertexId>` | 219 | Query vertices in current selection (empty if mode ≠ "vertex") |
| `selected_edge_ids` | `fn(&self) -> Vec<EdgeId>` | 226 | Query edges in current selection (empty if mode ≠ "edge") |
| `selected_face_ids` | `fn(&self) -> Vec<FaceId>` | 212 | Query faces in current selection (empty if mode ≠ "face") |
| `selection_vertex_ids` | `fn(&self) -> Result<Vec<VertexId>, LowpolyCoreError>` | 246 | Convert current selection to vertices (face→vertices, edge→endpoints, vertex→as-is) |
| `selection_transform_pivot` | `fn(&self) -> Result<Vec3, LowpolyCoreError>` | 279 | Compute centroid for transform pivot (all vertices in mesh-mode, selected subset in element modes) |

### **Mesh Access & Queries**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `active_mesh` | `fn(&self) -> Result<&HalfedgeMesh, LowpolyCoreError>` | 199 | Immutable active mesh kernel access |
| `active_mesh_mut` | `fn(&mut self) -> Result<&mut HalfedgeMesh, LowpolyCoreError>` | 194 | Mutable active mesh kernel for editing |

### **Paint Layer Management**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `ensure_all_paint_buffers` | `fn(&mut self)` | 133 | Ensure every object has ≥1 layer with correct pixel buffer size (1024×1024×4) |
| `ensure_paint_layer` | `fn(&mut self, object_id: &str, layer_index: usize) -> Result<(), LowpolyCoreError>` | 325 | Validate paint layer exists and size is correct |
| `layer_pixels` | `fn(&self, object_id: &str, layer_index: usize) -> Result<&[u8], LowpolyCoreError>` | 146 | Immutable RGBA pixel buffer access |
| `layer_pixels_mut` | `fn(&mut self, object_id: &str, layer_index: usize) -> Result<&mut Vec<u8>, LowpolyCoreError>` | 150 | Mutable pixel buffer access |
| `composite_layers` | `fn(&self, object_id: &str) -> Result<Vec<u8>, LowpolyCoreError>` | 370 | Render all layers blended to final RGBA |

### **Paint Operations**

| Function | Signature | File:Line | Description |
|----------|-----------|-----------|-------------|
| `paint_stroke` | `fn(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) -> Result<(), LowpolyCoreError>` | 377 | Stamp soft brush/eraser into layer (delegates to `schema::stamp_brush`) |
| `fill_bucket` | `fn(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, color: [u8; 4]) -> Result<(), LowpolyCoreError>` | 383 | Flood-fill from UV coordinate (delegates to `schema::flood_fill`) |
| `sample_pixel` | `fn(&self, object_id: &str, u: f32, v: f32) -> Result<[u8; 4], LowpolyCoreError>` | 389 | Sample composite color at UV |

---

## 3. Unimplemented / Incomplete Features

| File:Line | Category | Code | Description |
|-----------|----------|------|-------------|
| `✏️editor/🦀️component.rs:1338` | Media Export | `Err(MediaError::NotImplemented)` | Unidentified export path in media handler (context: "default export media case") |
| `✏️editor/🦀️component.rs:1526` | Media Export | `Err(MediaError::NotImplemented)` | Unidentified media query handler case |
| `👁️viewer/.../🌐️model/🦀️component.rs:7` | Viewer Geometry | Comment: "world_meshes_json falls back to while composed-child mesh resolution is unimplemented" | Composed mesh references not fully resolved in viewer (depends on artifact child resolution framework feature) |

---

## 4. Full Module Map from Glue Declarations

**File:** `📦️packages/🦀️rust/📦️glue.rs` (actually `lib.rs` renamed in taxonomy)

### **Declared Paths (All Verified Existing)**

#### Artifact Schema Tree:
- `🗿️artifacts/💠️lowpoly/🦀️component.rs` ✓
  - `standards/v1/subsets/any/schema/🦀️component.rs` ✓
    - `snapshot/🦀️component.rs` ✓
    - `snapshot/binary/🦀️component.rs` ✓
    - `snapshot/text/🦀️component.rs` ✓
    - `inferences/🦀️component.rs` ✓
    - `inferences/binary/🦀️component.rs` ✓
    - `inferences/text/🦀️component.rs` ✓
    - `inferences/bounds/🦀️component.rs` ✓
    - `diff/🦀️component.rs` ✓
    - `diff/text/🦀️component.rs` ✓
    - `diff/binary/🦀️component.rs` ✓
    - `mutations/🦀️component.rs` ✓
    - `mutations/binary/🦀️component.rs` ✓
    - `mutations/text/🦀️component.rs` ✓
    - **Mutations (16 types):** create-object, delete-object, reorder-objects, rename-object, change-object-smooth-shading, move-object, rotate-object, scale-object, create-mesh, delete-mesh, insert-paint-layer, remove-paint-layer, rename-paint-layer, change-paint-layer-visible, change-paint-layer-opacity, change-paint-layer-blend-mode, edit-paint-layer (each with diff/inverse/test)

#### I/O Codec Tree (Import/Export):
- `io/🦀️component.rs` ✓
  - **Import:** las, ply, txt, png, json, dwg, stl, gltf, obj (9 formats)
  - **Export:** las, ply, txt, png, json, dwg, stl, gltf, obj (9 formats)

#### Editor Subsystems:
- `✏️editor/🦀️component.rs` ✓ (134 KB - main app)
  - `config/🦀️component.rs` ✓
  - `config/schema/🦀️component.rs` ✓
  - `presence/🦀️component.rs` ✓
  - `presence/schema/🦀️component.rs` ✓
  - `engine/🦀️component.rs` ✓ (THIS FILE - compute session)
  - `session/🦀️component.rs` ✓ (55 KB - session state & handlers)
  - `terminology/🦀️component.rs` ✓
  - `view/🦀️component.rs` ✓
  - **Commands (12):** add-primitive, camera, chrome, engagement, fixture, mesh-edit, paint, patch-object, selection, sun, transform, utility, uv
  - **Modes:** edit, paint (each with windows/submodules)
  - **Options (6):** paint-params-brush, paint-params-eraser, select, show-edges, snap, sun
  - **Panels (4):** catalogue, document, inspection, layers

#### Viewer:
- `👁️viewer/🦀️component.rs` ✓
  - `modes/view/🦀️component.rs` ✓
  - `modes/view/windows/model/🦀️component.rs` ✓

### **Dangling Files / Undeclared Modules**

**None found.** All `🦀️component.rs` files on disk are declared in glue.rs #[path] attributes. All declared paths resolve.

---

## 5. Exported WASM/Glue Surface

**File:** `📦️packages/🦀️rust/📦️glue.rs` (781 lines)

The plugin exports via: `semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::LowpolyApps)`

This macro auto-generates the WIT bindings from:
- `artifacts::lowpoly` schema definitions (mutations, codecs)
- `editor::lowpoly::LowpolyPlayApp` - the main app struct
- `viewer::lowpoly::LowpolyViewerApp` - the readonly viewer

No explicit function exports; all surface is via:
1. **Mutations:** `LowpolyMutation` enum (18 variants from 🧬️mutations/)
2. **Schema Codecs:** Text (DSL) and Binary (WIT) serialization
3. **App Commands:** `LowpolyPlayApp::handle(cmd, state) -> Result<Emit<Mutation, ConfigMutation>, Fault>`
4. **Media Export:** via `artifact_kind()` declaration in 🦀️component.rs:298

**No direct function exports** — all I/O is schema-driven (document codecs, mutation apply/inverse, diff computation).

---

## Summary Statistics

- **Session Type:** 1 (`LowpolyDocument`)
- **Geometry Operations:** 31 public methods
- **Mutation Types:** 17 (create/delete/modify object, mesh, paint layers; paint pixels)
- **IO Codecs:** 9 import + 9 export formats = 18 total
- **Editor Commands:** 12 command handler modules
- **Subsystems:** Config, Presence, Session, Terminology, View (5 + engine + 12 commands)
- **Unimplemented:** 2 export paths + 1 viewer resolution feature
- **Module Declarations:** ~100+ paths, all verified ✓
- **No Dangling Files:** Complete 1:1 mapping between disk and declarations

