# Lowpoly Editor Command Surface Inventory

**Date:** 2026-08-29 | **Scope:** Complete lowpoly editor command enumeration
**Total Commands:** 48 | **Command Groups:** 13

---

## Part 1: Complete Command Inventory by Group

### ✏️patch-object (1 command)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| patch-object | PatchObject | object_id: String, field: String, value_json: Option<String> | handle() | 🦀️component.rs:24 | Patches scalar field (name/smoothShading) on an object; delegates to RenameObject or ChangeObjectSmoothShading mutations |

### ➕️add-primitive (1 command)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| add-primitive | AddPrimitive | kind: Option<String> | handle() | 🦀️component.rs:20 | Appends new primitive mesh (box/plane/cylinder/cone/ico_sphere) and sets as active object |

### 🌞️sun (4 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| toggle-sun | ToggleSun | (none) | handle() | toggle_sun/🦀️component.rs:29 | Toggles sun visibility via framework's shared WorldSunConfig logic (config-only) |
| set-sun-azimuth | SetSunAzimuth | value: f64 | handle() | set_sun_azimuth/🦀️component.rs:45 | Sets sun azimuth angle (config-only) |
| set-sun-elevation | SetSunElevation | value: f64 | handle() | set_sun_elevation/🦀️component.rs:61 | Sets sun elevation angle (config-only) |
| set-sun-intensity | SetSunIntensity | value: f64 | handle() | set_sun_intensity/🦀️component.rs:77 | Sets sun light intensity (config-only) |

### 🎥️camera (1 command)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| set-camera | SetCamera | position: [f64; 3], target: [f64; 3], fov: f64 | handle() | set_camera/🦀️component.rs:24 | Updates live world-3D camera pose (config-only) |

### 👁️chrome (1 command)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| toggle-show-edges | ToggleShowEdges | (none) | handle() | toggle_show_edges/🦀️component.rs:18 | Toggles edge rendering chrome (config-only) |

### 💬️engagement (2 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| engagement-input | EngagementInput | value: String | handle() | engagement_input/🦀️component.rs:22 | Stages typed engagement input text (config-only); caller must follow with engagement-submit |
| engagement-submit | EngagementSubmit | value: Option<String> | handle() | engagement_submit/🦀️component.rs:38 | Resolves typed token against known command list (extrude/inset/bevel/loopCut/subdivide/triangulate/mirror/decimate/flipFaces/merge/dissolve/snap); dispatches matching command with default args |

### 📄️fixture (2 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| import-snapshot-json | ImportSnapshotJson | json: String | handle() | set_snapshot_json/🦀️component.rs:30 | Whole-projection JSON replacement (outside undo history via Effect::LoadDocument); alias: importSnapshotJson |
| set-fixture-json | SetFixtureJson | json: String | handle() | set_fixture_json/🦀️component.rs:46 | Whole-projection JSON replacement (outside undo history via Effect::LoadDocument); alias: setFixtureJson |

### 🔷️mesh-edit (13 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| extrude | Extrude | extrude_distance: Option<f32> | handle() | extrude/🦀️component.rs:28 | Extrudes selected faces along normal; reads utility param extrudeDistance (default 0.25) |
| inset | Inset | inset_amount: Option<f32> | handle() | inset/🦀️component.rs:54 | Inserts inset region on selected faces; reads utility param insetAmount (default 0.1) |
| bevel | Bevel | bevel_amount: Option<f32>, bevel_segments: Option<u32> | handle() | bevel/🦀️component.rs:78 | Bevels selected edges; reads utility params bevelAmount (0.05) and bevelSegments (1) |
| loop-cut | LoopCut | loop_cuts: Option<u32> | handle() | loop_cut/🦀️component.rs:102 | Adds loop cuts along selected edges; reads utility param loopCuts (default 1) |
| subdivide | Subdivide | (none) | handle() | subdivide/🦀️component.rs:123 | Subdivides selected faces (Catmull-Clark) |
| triangulate | Triangulate | (none) | handle() | triangulate/🦀️component.rs:141 | Converts all faces to triangles |
| mirror | Mirror | axis: Option<String> | handle() | mirror/🦀️component.rs:160 | Mirrors active mesh on axis (X/Y/Z); reads utility param mirrorAxis |
| decimate | Decimate | decimate_ratio: Option<f32> | handle() | decimate/🦀️component.rs:189 | Reduces face count by ratio (0..1); reads utility param decimateRatio (default 0.5) |
| flip-faces | FlipFaces | face_ids: Vec<u32> | handle() | flip_faces/🦀️component.rs:211 | Flips normals on specified faces; falls back to selected_face_ids() or full selection.ids |
| merge | Merge | (none) | handle() | merge/🦀️component.rs:236 | Welds selected vertices (center mode, 0.001 threshold) |
| dissolve | Dissolve | (none) | handle() | dissolve/🦀️component.rs:254 | Removes selected edges and merges adjacent faces |
| snap | Snap | (none) | handle() | snap/🦀️component.rs:272 | Snaps selected vertices to grid; reads utility param snapGrid (default 0.25) |
| toggle-smooth | ToggleSmooth | (none) | handle() | toggle_smooth/🦀️component.rs:293 | Toggles active object's smooth shading flag and recomputes normals |

### 🖌️paint (10 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| paint-stroke-begin | PaintStrokeBegin | (none) | handle() | paint_stroke_begin/🦀️component.rs:48 | Marks start of paint stroke drag sequence (no mutations) |
| paint-stroke | PaintStroke | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | paint_stroke/🦀️component.rs:83 | Paint tick during stroke drag (uses u/v UV coords or x/y canvas pixels); no intermediate mutations |
| paint-at | PaintAt | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | paint_at/🦀️component.rs:103 | Single paint tick (identical to paint-stroke, distinct wire keyword) |
| canvas-pointer-down | CanvasPointerDown | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | canvas_pointer_down/🦀️component.rs:123 | Paint at canvas pointer down event; no intermediate mutations |
| canvas-pointer-move | CanvasPointerMove | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | canvas_pointer_move/🦀️component.rs:143 | Paint during pointer move (only if stroke_drag_active); no intermediate mutations |
| paint-stroke-end | PaintStrokeEnd | (none) | handle() | paint_stroke_end/🦀️component.rs:64 | Marks end of paint stroke drag; emits single coalesced PaintStroke mutation |
| paint-fill | PaintFill | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | paint_fill/🦀️component.rs:166 | Single-shot flood-fill from UV point |
| fill-bucket | FillBucket | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | fill_bucket/🦀️component.rs:188 | Alias for paint-fill (identical implementation) |
| paint-sample | PaintSample | object_id: Option<String>, u/v/x/y: Option<f32> | handle() | paint_sample/🦀️component.rs:210 | Eyedropper: samples color from UV point and updates paint color config (config-only, no mutations) |
| add-paint-layer | AddPaintLayer | object_id: Option<String>, name: Option<String> | handle() | add_paint_layer/🦀️component.rs:232 | Appends new paint layer to object and emits InsertPaintLayer mutation |

### 🗂️selection (2 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| set-active-object | SetActiveObject | object_id: String | handle() | set_active_object/🦀️component.rs:27 | Sets active object (view state, config-only, no mutations); validates existence |
| set-active-paint-layer | SetActivePaintLayer | layer_index: u32 | handle() | set_active_paint_layer/🦀️component.rs:47 | Sets active paint layer index (view state, config-only, no mutations) |

**NOTE:** Selection domain (vertex/edge/face targeting) is NOW owned by framework via injected `interactionSelect`/`setInteractionGranularity` verbs (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Deleted commands: SetSelection, ToggleSelectionKind, ToggleSelectionTarget, SetSelectionMethod, SetSelectionModeDefault.

### 🧰️utility (2 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| set-utility-param | SetUtilityParam | key: String, value_json: String | handle() | set_utility_param/🦀️component.rs:25 | Updates per-utility parameter (e.g., extrudeDistance, bevelAmount); merged into utility_params JSON object (config-only) |
| set-active-utility | SetActiveUtility | utility_id: String | handle() | set_active_utility/🦀️component.rs:50 | Switches active transform/paint utility, clears mid-gesture scratch, updates config (config-only); also sets paint_utility if paint tool |

### 🧲️transform (5 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| transform-begin | TransformBegin | (none) | handle() | transform_begin/🦀️component.rs:21 | Marks start of gumball drag (no mutations) |
| translate-selection | TranslateSelection | mode: Option<String>, ids: Option<Vec<u32>>, dx/dy/dz: f32 | handle() | translate_selection/🦀️component.rs:56 | Translates by delta (mode='mesh' default); no intermediate mutations, coalesces on transformEnd |
| rotate-selection | RotateSelection | mode: Option<String>, ids: Option<Vec<u32>>, ax/ay/az/angle: f32 | handle() | rotate_selection/🦀️component.rs:79 | Rotates around axis by angle (mode='mesh' default); no intermediate mutations |
| scale-selection | ScaleSelection | mode: Option<String>, ids: Option<Vec<u32>>, sx/sy/sz: f32 | handle() | scale_selection/🦀️component.rs:101 | Scales by factors (mode='mesh' default); no intermediate mutations |
| transform-end | TransformEnd | (none) | handle() | transform_end/🦀️component.rs:36 | Marks end of gumball drag; emits single coalesced Objects(Patch) mutation |

### 🧵️uv (3 commands)

| Command | Struct | Payload Fields | Handler | File:Line | Semantics |
|---------|--------|-----------------|---------|-----------|-----------|
| unwrap-active | UnwrapActive | (none) | handle() | unwrap_active/🦀️component.rs:19 | Unwraps UVs on active mesh; resyncs mesh JSON |
| mark-uv-seam | MarkUvSeam | seam: Option<bool>, edge_ids: Option<Vec<u32>> | handle() | mark_uv_seam/🦀️component.rs:39 | Marks (or unmarks if seam=false) UV seams on edges; falls back to current_selection().ids if edge_ids omitted |
| clear-seam | ClearSeam | (none) | handle() | clear_seam/🦀️component.rs:62 | Removes all UV seams from selected edges (delegates to mark_uv_seam with seam=false) |

---

## Part 2: Implementation Status

### Fully Implemented Commands: 48/48

All 48 commands are fully implemented with no `unimplemented!()` / `todo!()` stubs. Each handler:
- Either emits artifact mutations (mesh-edit, paint, fixture commands)
- Or emits config mutations only (chrome, sun, camera, utility, selection, transform begin/ticks, paint begin/ticks, engagement input)
- Or coalesces to single commit mutation on drag-end (paint stroke, transform drag)
- Gracefully degrades to no-op `Emit::default()` on validation failure (missing selection, invalid JSON, etc.)

---

## Part 3: Command Declaration and Dispatch Mechanism

### Macro: `app_commands!` — Line 269, Editor Component

**Location:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:269–321`

**Syntax Pattern:**
```rust
semio_framework_plugin::app_commands! {
    pub enum LowpolyCommand for LowpolySnapshot, LowpolyMutation, LowpolyConfig, LowpolyConfigMutation, ctx = LowpolyScratch {
        "camelCaseActionId" as "dsl-keyword" => module::PayloadStruct,
        // ... repeat for each command
    }
}

// Then import each payload module by its inner type name:
use camera::set_camera;
use chrome::toggle_show_edges;
use engagement::{engagement_input, engagement_submit};
// ... etc.
```

**Row Order:** Ordinal variant index (appending safe, reordering breaks wire format).

**Declaration Sites:**
1. **Enum variants:** Lines 274–321 in 🦀️editor/component.rs
2. **Payload struct imports:** Lines 328–338 in 🦀️editor/component.rs
3. **Payload struct definitions:** Each group's `🦀️component.rs` file (e.g., `extrude/🦀️component.rs:19`)
4. **Handler functions:** Each struct's `pub fn handle(payload: &Struct, doc: &ArtifactView, cfg: &ConfigView, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>` (same file)

**To Add a New Command (Mechanical Recipe):**

1. **Create payload struct** in a new or existing `🎮️commands/📁group/inner_module.rs`:
   ```rust
   pub mod my_new_command {
       #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
       #[dsl(keyword = "my-keyword")]
       pub struct MyNewCommand {
           pub field_a: String,
           pub field_b: Option<f32>,
       }
       
       pub fn handle(payload: &MyNewCommand, doc: &ArtifactView<'_, LowpolySnapshot>, 
                     cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) 
           -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>
       {
           // implementation
           Ok(Emit::mutations(vec![...]))
       }
   }
   ```

2. **Add variant to enum** in 🦀️editor/component.rs, line ~315:
   ```rust
   "myNewCommandActionId" as "my-keyword" => my_new_command::MyNewCommand,
   ```

3. **Import payload module** in 🦀️editor/component.rs, line ~338:
   ```rust
   use my_group::{my_new_command};
   ```

4. **Wire keyword must be distinct** (macro invariant — enforced by lint).

---

## Part 4: Unimplemented Commands

**Count:** 0 / 48

All commands are production-ready. No stubs, errors, or TODOs detected.

---

## Part 5: Missing Commands (Full-Featured Mesh Editor Gaps)

Based on Blender/Maya/Houdini lowpoly editing conventions and the present topology types (vertices, edges, faces, half-edge mesh kernel), the following are notably absent:

### Core Topology Operations (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Extrude Edges** (via edges, not faces) | Only extrude_faces() exposed in kernel; edge extrude requires face-ring walk | Edges can only be beveled, not directly extruded to create new geometry |
| **Inset Faces (Individual)** | Only collective inset on multi-face selection | Cannot inset single face independently in one step |
| **Knife Tool / Loop Knife** | No interactive edge-drawing mode; would require retained stroke state | Cannot cut arbitrary topology into faces |
| **Separate / Join Objects** | No multi-object merge or single-object split command | All ops on single active mesh; no object grouping |
| **Weld (General)** | Only merge_vertices(); no edge-based or proximity-based weld variant | Limited vertex consolidation |
| **Duplicate (Shift-D style)** | No in-place duplication+selection command | Must add-primitive + model manually |
| **Delete Elements** | No vertex/edge/face deletion (only dissolve edges) | Cannot remove orphaned topology |

### Selection and View Operations (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Select By** (type/color/random/similar) | Framework-owned selection; no filter predicates | Must select via click/box/lasso only |
| **Undo / Redo** | Framework-owned; not an app command (store-level) | Works via handle_action("undo"/"redo"), not dispatch |
| **Select All / None / Invert** | Selection now framework-owned (interactionSelect) | Missing convenience shortcuts |

### Material & Shading (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Assign Material / Slots** | No material system modeled | Paint-only texturing; no material library |
| **Flatten / Smooth Shading** (per-face mode) | toggle-smooth is object-wide; no per-face smooth flags | Coarse shading control |
| **Normal Editing / Flip By Normal Angle** | flip-faces only; no weighted or threshold-based flip | Manual per-face normal control |

### Transform & Precision (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Constrain Axis (X/Y/Z lock during drag)** | Gumball args override, no live constraint toggle | Users must pass explicit per-axis transforms |
| **Snap to Vertex / Edge / Face** | Snap only to grid; no target-snapping | Limited precision alignment |
| **Align (distribute/match center)** | No multi-object hierarchy | Single-object model only |

### Deformation & Detail (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Proportional Edit** | No falloff-weighted edit; all verts in selection equally | All-or-nothing transforms |
| **Smooth (Geometry Blur)** | No Laplacian smoothing; subdivide only | Cannot soften hard edges gradually |
| **Lattice Deform / Cage Edit** | No non-uniform deformer | Only uniform mesh-level transforms |
| **Bend / Taper / Twist** | No per-axis curvature deformers | Gumball only |

### UV & Texture (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **UV Pack** | Only unwrap; no packing algorithm | UVs may overlap; no atlas optimization |
| **UV Straighten / Orient** | No per-island transform | Seams and island rotation manual |
| **UV Checker Map** | No diagnostic texture | Cannot visually verify UV distortion |
| **Bake Texture / AO** | No baking engine | Cannot project detail to paint layer |

### Export & Documentation (Expected but Missing)

| Operation | Reason Missing | Impact |
|-----------|-----------------|--------|
| **Export Mesh (OBJ/FBX/GLTF)** | Media I/O only, no format export command | Cannot ship finished mesh to game engine |
| **Naming / Metadata** | Only object.name patch; no custom properties | Limited annotation |

---

## Part 6: Mode and Structural Gaps

### Edit Mode (`🎭️edit`)
- **Empty folders (framework slots, unused):**
  - 🎚️config/ — no mode-specific config beyond artifact/app-level
  - 👥️presence/ — no mode-scoped presence state
  - 🎮️commands/ — all commands in top-level aggregate; no edit-mode-specific verbs
  - 🫧️transient/ — no transient mutable state exclusive to edit mode
  
- **Window: 🌐️model**
  - 🎚️config/ — (empty)
  - 👥️presence/ — (empty)
  - 🎬️actions/ — (empty)
  - 🎚️options/ — (empty)
  - 🪛️utilities/ — (empty)
  - 🫧️transient/ — (empty)

### Paint Mode (`🎭️paint`)
- **Empty folders:**
  - 🎚️config/ — no mode-specific config
  - 👥️presence/ — no mode-scoped presence
  - 🎮️commands/ — all commands in top-level aggregate
  - 🫧️transient/ — no transient mutable state exclusive to paint mode

- **Window: 🖼️uv**
  - 🎚️config/ — (empty)
  - 👥️presence/ — (empty)
  - 🎬️actions/ — (empty)
  - 🎚️options/ — (empty)
  - 🪛️utilities/ — (empty)
  - 🫧️transient/ — (empty)

**Interpretation:** Modes and windows are rendered/UI separations only; all command/state machinery is centralized at app level. No mode-specific validation, context, or transient scratch. This simplifies multi-mode consistency but constrains mode-scoped feature isolation.

---

## Part 7: Summary Metrics

| Metric | Count |
|--------|-------|
| **Total Commands** | 48 |
| **Command Groups** | 13 |
| **Artifact Mutations** | 21 (add-primitive, patch-object, mesh-edit, paint strokes, paint layers, fixture load) |
| **Config-Only Commands** | 19 (sun, camera, chrome, utility, selection, transform ticks, paint ticks, engagement input) |
| **Coalesced Drag Commands** | 6 (paint: begin/ticks/end; transform: begin/ticks/end) |
| **Delegation/Aliases** | 2 (engagement-submit → resolve + dispatch; clear-seam → mark-uv-seam) |
| **Fully Implemented** | 48 / 48 |
| **Unimplemented Stubs** | 0 |
| **Empty Mode Folders** | 20 (across edit/paint modes and their windows) |
| **Missing Blender-Style Operations** | ~25+ (see Part 5) |

---

## Appendix: Payload Field Patterns

### By Optionality
- **Required fields:** object_id (paint), position/target/fov (camera), field/value (patch), etc.
- **Optional numeric args:** extrude_distance, inset_amount, bevel_amount, decimate_ratio, etc. (all fall back to utility params)
- **Optional UV/canvas coords:** paint commands (u/v for 3D pick, x/y for UV canvas; auto-resolves to active object)

### By Type
- **String (JSON-backed):** value_json (patch-object), json (fixture), value_json (paint sample), utility_params (config)
- **Numeric ranges:** f32 (distances, ratios, grid), f64 (camera, sun angles/intensity), u32 (segments, layer index)
- **Vector types:** [f64; 3] for camera position/target, Vec3 for transform deltas, Vec<u32> for face/edge IDs
- **Enums (as strings):** axis (X/Y/Z for mirror), mode (mesh for transform), utility_id, keyword matches (engagement)

---

**Report compiled:** 2026-08-29 | **Schema version:** lowpoly.v1
