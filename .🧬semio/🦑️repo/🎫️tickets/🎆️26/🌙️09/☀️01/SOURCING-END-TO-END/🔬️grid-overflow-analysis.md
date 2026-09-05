# Grid Window Overflow Analysis

## Executive Summary

The `ui.fixed-capacity: fixed UI admission failed at mesh-window.scene` error in the sourcing plugin's Grid window is caused by **error collapse in MeshWindowKit::render**, which discards the true failure details. The actual failure is likely one of three SurfaceEncodeError variants, NOT a payload byte overflow (18,606 bytes fits in 32 KiB).

## 1. Error Mapping & Collapse

### File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26203`

```rust
let props = semio_framework_ui_scene::encode(SurfaceKind::World3d, &scene).map_err(|_| ui_assembly_error("mesh-window.scene"))?;
```

The `.map_err(|_| ...)` **discards all error details** from the encode function and replaces them with a generic error. This is the root cause of the non-diagnostic error message.

### Fallback error handler: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:372-374`

```rust
fn ui_assembly_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}"))
}
```

All UI assembly errors are lumped under `"ui.fixed-capacity"` code regardless of true cause.

## 2. SurfaceEncodeError Variants

**File:** `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs:39-51`

```rust
pub enum SurfaceEncodeError {
    Pack(PackError),                    // line 40: encoding codec failure
    Payload(Vec<u8>),                   // line 41: bytes exceed capacity
    Schema(&'static str),               // line 42: schema string exceeds capacity
}

impl std::fmt::Display for SurfaceEncodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(error) => write!(formatter, "surface payload encode failed: {error}"),
            Self::Payload(bytes) => write!(formatter, "surface payload exceeds fixed capacity with {} bytes", bytes.len()),
            Self::Schema(schema) => write!(formatter, "surface schema exceeds fixed capacity: {schema}"),
        }
    }
}
```

All three variants are mapped to the same `"ui.fixed-capacity"` error code via the collapse at line 26203.

## 3. Capacity Constants

### `UI_FIXED_BYTES` (byte cap on encoded payload)
- **Value:** 32 * 1024 = 32,768 bytes
- **File:Line:** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:20`
- **Applies to:** Total byte size of the packed `World3dScene` after `encode_pack()` via varint-compressed pack codec
- **Check location:** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:582-590` in `UiFixedBytes::try_from_vec()`

```rust
pub const UI_FIXED_BYTES: usize = 32 * 1_024;  // line 20

pub fn try_from_vec(value: Vec<u8>) -> Result<Self, Vec<u8>> {
    if value.len() > UI_FIXED_BYTES {
        return Err(value);
    }
    // ...
}
```

### `UI_TEXT_MAX_BYTES` (cap on schema string)
- **Value:** 512 bytes
- **File:Line:** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:18`
- **Applies to:** The schema identifier string (e.g., `"world-3d@1"`)
- **Check location:** `🧰️framework/🔨️modules/🖱️ui/🧬️scene/📦️packages/🦀️rust/🌉️surface.rs:66` in `encode()`

```rust
let doc_schema = ui_contract::UiText::try_from_str(T::SCHEMA).ok_or(SurfaceEncodeError::Schema(T::SCHEMA))?;
```

## 4. Measurement vs. Reality

### Measurement Script Finding
- **Location:** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/measure-grid-scene.py`
- **Result:** meshes_json + instances_json = 18,606 bytes (57% of 32 KiB cap)
- **Conclusion:** NOT a direct byte overflow

### Why measurement is insufficient:
1. **Not measuring the full World3dScene payload:** The grid construct only measures `meshes_json` + `instances_json`, but the full scene also includes:
   - `camera_json` (from `world3d_default_camera()`)
   - `selection_json` (from `world3d_selection_json("rectangle", &[], None)`)
   - `environment_json` (from `world3d_environment_json(sun)`)

2. **Pack encoding adds overhead:** The measurement script measures raw JSON strings, but `World3dScene::encode_pack()` runs through the varint-compressed pack codec at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📦️pack.rs`, which may not be a simple pass-through.

3. **Pack codec constraints:** The pack module defines no hard size limit, but may have encoding issues with certain data patterns or fail on unsupported features.

## 5. Sourcing Grid Window Construction

**File:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs:38-60`

```rust
pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> UiAssemblyResult<BuiltNode> {
    let filtered = filtered_stock(document, &cfg.filters);
    let mut seen_mesh_ids = HashSet::new();
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for (index, kind) in filtered.iter().enumerate() {
        if seen_mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));  // line 45
        }
        let (x, z) = grid_placement(filtered.len(), index, SOURCING_CURATION_GRID_CELL);
        let scale = grid_scale(&kind.geometry, SOURCING_CURATION_GRID_CELL * 0.8);
        instances.push(instance_json(kind, [x, 0.0, z], scale, false));  // line 52
    }
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_default_camera(),
        meshes_json: dsl::json::to_json_string(&dsl::DslValue::Array(meshes)),  // line 56
        instances_json: dsl::json::to_json_string(&dsl::DslValue::Array(instances)),  // line 57
        selection_json: world3d_selection_json("rectangle", &[], None),  // line 58
    })
}
```

### Key observations:
- **Mesh construction:** Each stock kind generates ONE mesh via `kind_mesh_json(kind)`, which calls `mesh_from_indexed()` with the geometry spec (lines 237-241 of schema/🦀️.rs)
- **Instance construction:** Each filtered stock kind generates ONE instance via `instance_json(kind, position, scale, selected)` (lines 245-256 of schema/🦀️.rs)
- **NO paging:** Grid does NOT use `World3dSnapshotLease` for paging—it embeds all meshes and instances directly in the scene JSON
- **Demo stock:** All 10 stock kinds (beams, windows, slabs, etc.) are included unless filtered

## 6. Comparison: Procedural Plugin (Paging)

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`

The procedural plugin also renders World3d scenes but uses `World3dSnapshotLease` for paging in cases with large payloads:

**Search results:** `grep -rn "World3dSnapshotLease" --include="*.rs"` shows:
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌍️world3d_snapshot.rs:12:pub const WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY: usize = 16 * 1024;`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs:2136`

The snapshot mechanism allows paging large scenes across multiple 16 KiB pages instead of embedding everything in one 32 KiB payload.

## 7. Root Cause Analysis

### Most Likely Failure: Pack Codec Error (not byte overflow)

Given:
1. Measurement shows 18.6 KiB of just meshes_json + instances_json
2. Full scene adds camera_json + selection_json + environment_json (~100 bytes each)
3. Total JSON strings likely still < 30 KiB
4. Error message collapses all three SurfaceEncodeError variants

**Hypothesis:** The Pack encoding fails on a `SurfaceEncodeError::Pack(PackError)` variant, likely:
- **Invalid UTF-8 in JSON:** A JSON string contains non-UTF-8 data (unlikely but possible if `dsl::json::to_json_string` has a bug)
- **Unsupported serde feature:** The pack codec explicitly rejects `serialize_map` and enum variants (line 12 of pack.rs); if the packed data structure uses these, it will fail
- **Truncation on decode:** A roundtrip pack/unpack fails to preserve the data exactly

### Secondary: Byte Overflow After Full Scene Assembly

If the full World3dScene (including camera + selection + environment + pack encoding overhead) exceeds 32 KiB, the error is `SurfaceEncodeError::Payload`, which would show: `"surface payload exceeds fixed capacity with X bytes"` (but this is masked by the collapse).

## Recommended Fix

**Change:** Propagate the actual `SurfaceEncodeError` details from `MeshWindowKit::render()` instead of collapsing them.

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26203`

**Before:**
```rust
let props = semio_framework_ui_scene::encode(SurfaceKind::World3d, &scene).map_err(|_| ui_assembly_error("mesh-window.scene"))?;
```

**After:**
```rust
let props = semio_framework_ui_scene::encode(SurfaceKind::World3d, &scene).map_err(|error| {
    let stage = match &error {
        semio_framework_ui_scene::SurfaceEncodeError::Pack(_) => "mesh-window.scene-pack",
        semio_framework_ui_scene::SurfaceEncodeError::Payload(_) => "mesh-window.scene-payload",
        semio_framework_ui_scene::SurfaceEncodeError::Schema(_) => "mesh-window.scene-schema",
    };
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}: {error}"))
})?;
```

This will reveal whether the failure is:
- **Pack error:** Indicates a codec bug or unsupported serde feature
- **Payload error + byte count:** Indicates grid is exceeding 32 KiB and needs `World3dSnapshotLease` paging
- **Schema error:** Indicates schema string > 512 bytes (unlikely for `"world-3d@1"`)

Once the true error is visible, the fix is either:
1. Fix the pack encoding issue
2. Implement `World3dSnapshotLease` paging in the sourcing grid (like the fem plugin does)
3. Reduce grid complexity (fewer meshes, shared instances, LOD)
