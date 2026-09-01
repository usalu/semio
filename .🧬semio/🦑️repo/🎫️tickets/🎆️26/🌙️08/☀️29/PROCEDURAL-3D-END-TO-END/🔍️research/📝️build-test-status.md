# Procedural 3D Build & Test Status Report

## 1. Launch Configuration Names

### VSCode (.vscode/launch.json) - Procedural/Flow/S-Dev/Plugin Related:
- `🛠️dev🔧️procedural🩻️2d⚛️react` (port 6021, React)
- `🛠️dev🔧️procedural🩻️2d🧊️wgpu🌐️wasm` (port 6121, WASM)
- `🛠️dev🔧️procedural🩻️2d🧊️wgpu🖥️native` (native)
- `🛠️dev🔧️procedural🏙️3d⚛️react` (port 6018, React) **PROCEDURAL 3D**
- `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm` (port 6118, WASM) **PROCEDURAL 3D**
- `🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native` (native) **PROCEDURAL 3D**
- `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column⚛️react` (port 6018)
- `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🌐️wasm` (port 6118)
- `🛠️dev🌊️flow⚛️react` (port 6016)
- `🛠️dev🌊️flow🧊️wgpu🌐️wasm` (port 6116)

### Claude Launch (.claude/launch.json):
- `s-react` (port 6070) **S DEV APP** - command: `bun ./📜️script.ts dev s`

## 2. NX Targets for Procedural Plugin

**Location:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📋️project.json`

**Crate Name:** `semio-s-plugin-procedural`

**Available Targets:**
- `test` - standard tests
- `test-quick` - quick test suite
- `test-long` - extended tests
- `test-exhaustive` - exhaustive test suite
- `describe` - builds `wasm32-wasip2` component and re-emits descriptor files

**WASM Target:** `wasm32-wasip2`

## 3. S Product Dev Server

**Command:** `bun ./📜️script.ts dev s`

**Port:** 6070 (default S_OS_PORT, can be overridden)

**Renderer Options:** 
- React (default): `SEMIO_RENDERER=react`
- WGPU WebAssembly: `SEMIO_RENDERER=wgpu`

**Router Function:** `runFrameworkOsPlaygroundDev("s", segments)`

## 4. Native Cargo Test Status

**Command:** `cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust" && cargo test --offline 2>&1`

Status: **RUNNING (background task)**
- Process started at 19:50 GMT
- Output file still empty at time of investigation
- Note: `--offline` mode used; will retry without it if needed

## 5. WASM32-WASIP2 Target Build Status

**Command:** `cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust" && cargo build --target wasm32-wasip2 2>&1`

Status: **RUNNING (background task)**
- Process started at 19:53 GMT
- Output file still empty at time of investigation
- Target configured in Cargo.toml with `crate-type = ["cdylib", "rlib"]`
- Component entry point controlled by `plugin-entry` feature (default-on)

## 6. Procedural 3D Tests

### Test Files Found:
Multiple test suites in `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/`:

**Mutation Tests** (15+ test scenarios):
- `update-synapse` - repoints wire connections
- `update-camera` - frames graph at different zoom levels
- `rename-generation` - retitles generation via new name
- `create-generation` - appends generation and moves selection
- `create-widget` - inserts node at index
- `disconnect-synapse` - cuts wire, leaving nodes
- `update-widget` - retunes knob slider value
- `delete-generation` - removes selected generation
- `delete-widget` - removes node, leaves wire dangling
- `connect-synapse` - wires node connections
- `move-widget` - repositions node in graph
- `change-schema` - restamps fixture schema ID
- `change-generation-value` - raises storeys answer in generation

**Preview & Tessellation:**
- Reference in component.rs line 860: "Pure per-render tessellation: bounded-cost, safe to call fresh on every render call"
- Function `tessellate_geometry()` at line 876
- Function `pending_preview_tessellate_handles()` at line 881
- Function `preview_tessellate_effects()` at line 911
- Preview widget tessellation is an async host capability

**Hover State:**
- `hoveredNodeId: null` appears in all test diff snapshots
- Indicates hover state tracking in editor

### Schema Components Tested:
- Editor component at `✏️editor/🦀️component.rs`
- Viewer component at `👁️viewer/🦀️component.rs`
- Schema component with tessellation logic at `🧬️schema/🦀️component.rs`

## Commands for Manual Testing

```bash
# Run procedural plugin tests
nx run @semio-tech/procedural-plugin:test

# Run quick test suite
nx run @semio-tech/procedural-plugin:test-quick

# Build WASM component
nx run @semio-tech/procedural-plugin:describe

# Start procedural 3D dev server (React)
bun ./📜️script.ts dev procedural 3d

# Start procedural 3D dev server (WGPU)
SEMIO_RENDERER=wgpu bun ./📜️script.ts dev procedural 3d

# Start S product dev app
bun ./📜️script.ts dev s
```

---

**Report Generated:** 2026-08-29 19:55 UTC
**Status:** Cargo test and WASM build tasks still executing in background
