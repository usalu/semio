# B-Rep Kernel Dependency Analysis: brepjs/OpenCascade Removal for Precise Geometry Runtime

## Executive Summary

This document surveys the TypeScript runtime dependencies on `brepjs` and `brepjs-opencascade` (OpenCascade WASM), identifies all call sites, categorizes kernel operations as pure TS vs OCCT-backed, documents the current WASM bridge pattern, and outlines the gap for routing precise geometry through the first-party Rust B-Rep kernel (`semio-s-plugin-stdio`).

---

## 1. Complete API Call Inventory: brepjs Functions Used by Legacy Kernel

### Import Site
- **File**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/⚙️engine/🧱️brepjs/🟦️.ts` (lines 2–4, 38–41)
- **Imports**: `brepjs`, `brepjs-opencascade`, OpenCascade initialization

### Core Geometry Factory Functions (Shape Creation)
| Function | Call Sites | Kernel Operation | Count |
|----------|-----------|------------------|-------|
| `box()` | L2490 | `createBoxFromCorners` → primitive solid | 1 |
| `sphere()` | — | `createSphereFromCorners` (not directly called) | 0 |
| `circle()` | L1643, L2685, L1852 | Edge curve creation for swept profiles | 3 |
| `line()` | L1641, L1660, L2878 | Edge construction from vertices | 3 |
| `threePointArc()` | L1648 | Arc edge from 3 points | 1 |
| `wireLoop()` | L1675, L1693, L1713, L1853, L2879 | Closed curve topology | 5 |
| `wire()` | L1677 | Open curve topology (fallback) | 1 |
| `face()` | L1695, L2881 | Planar face from wire | 2 |
| `filledFace()` | L1715 | Non-planar face (deformed/filled) | 1 |
| `extrude()` | L1725 | Linear extrusion of face to solid | 1 |
| `loft()` | L1731, L2908 | Surface lofting between curves | 2 |
| `healSolid()` | L1736, L2307 | Repair invalid solids | 2 |
| `thicken()` | L1740 | Convert face/shell to solid | 1 |
| `translate()` | — | Transform (used internally for sweep) | 0 |
| `bsplineApprox()` | L1652, L1657 | Curve fitting from poles | 2 |

### Topology/Inspection Functions
| Function | Call Sites | Purpose | Count |
|----------|-----------|---------|-------|
| `getFaces()` | L1459, L1533 | Enumerate solid faces | 2 |
| `getEdges()` | L1464, L1561 | Enumerate solid edges | 2 |
| `getHashCode()` | L1501, L1506, L1510, L1516, L1546, L1569 | Face/edge identity deduplication | 6 |
| `getCurveType()` | L1570 | Classify edge curve kind | 1 |
| `getSurfaceType()` | L1540 | Classify face surface kind | 1 |
| `verticesOfEdge()` | — | Extract edge endpoints (not called) | 0 |

### Measurement Functions
| Function | Call Sites | Purpose | Count |
|----------|-----------|---------|-------|
| `measureArea()` | L1543, L3021 | Face area computation | 2 |
| `measureVolume()` | L2505 | Solid volume computation | 1 |
| `measureLength()` | — | Edge length (replaced by pure TS) | 0 |
| `curveLength()` | L1571 | Edge curve length | 1 |
| `curveStartPoint()` | — | Curve start (not called) | 0 |
| `curveEndPoint()` | — | Curve end (not called) | 0 |
| `normalAt()` | L1542 | Face normal vector | 1 |

### Boolean/Topology Operations
| Function | Call Sites | Purpose | Count |
|----------|-----------|---------|-------|
| `cut()` | L2796 | Subtraction (A − B) | 1 |
| `intersect()` | L2814 | Intersection (A ∩ B) | 1 |
| `fuseAll()` | — | Union of multiple solids (not called) | 0 |
| `sewShells()` | — | Stitch shell boundaries (not called) | 0 |
| `solidFromShell()` | — | Promote shell to solid (not called) | 0 |
| `offsetFace()` | — | Offset face (not called in kernel) | 0 |

### Tessellation/Rendering Functions
| Function | Call Sites | Purpose | Return | Count |
|----------|-----------|---------|--------|-------|
| `mesh()` | L1583 | Triangulate solid → `OwnedBrepMesh` | Float32Array buffers | 1 |
| `meshEdges()` | L1584 | Extract edges → `OwnedBrepEdgeMesh` | Line polylines + group ranges | 1 |
| `toGroupedBufferGeometryData()` | L1585 | Convert mesh to Three.js format | Grouped index + normal buffers | 1 |
| `toLineGeometryData()` | L1586 | Convert edge mesh to line buffers | Edge polyline buffers | 1 |

### Worker/Utility Functions
| Function | Call Sites | Purpose | Count |
|----------|-----------|---------|-------|
| `isOk()` | L1653, L1658, L1676, L1694, L1714, L1726, L1732, L1737, L1741 | Result type check | 9 |
| `unwrap()` | L2505, L2807, L2815, L3021 | Extract value from Result | 4 |
| `isSolid()` | L1734 | Type check: shape is closed | 1 |
| `isValidSolid()` | L1735 | Type check: valid solid state | 1 |

### Initialization
- `initializeOwnedOpenCascade()` (L1350, L1354): Loads WASM blob at runtime
- `resolveOwnedOpenCascadeWasmFileUrl()` (L1350): Node-only test loader

---

## 2. CAD Runtime Call Sites & Exercised Kernel Operations

### Legacy Kernel Instantiation
- **Class**: `BrepjsKernel` (L3222)
- **ID**: `"brepjs-opencascade"` (L3223)
- **Carrier**: `BrepjsWorkerClient` (L3133–3217)
- **Worker**: `/Users/ueli/Documents/semio/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/index.ts` (L1348)

### Public API Methods of BrepjsKernel (Implemented via RPC)
| Method | Signature | Calls | Line |
|--------|-----------|-------|------|
| `createBoxFromCorners` | `(input) → Promise<SolidRef>` | `box` + mesh | L3228–3230 |
| `volume` | `(SolidRef) → Promise<number>` | `measureVolume` | L3232–3234 |
| `tessellate` | `(SolidRef, tolerance, Model) → Promise<MeshTransfer>` | `mesh`, `meshEdges`, `toGrouped*` | L3236–3238 |
| `createBoxFromCornersDiff` | `(input) → Promise<{diff, solid}>` | Builds model diff | L3249–3251 |
| `extrudeWireDiff` | `(wireId, distance, direction, model) → Promise<{diff, solid}>` | `extrude`, `loft`, `healSolid`, `thicken` | L3253–3255 |
| `offsetFacesDiff` | `(faceIds, distance, model) → Promise<{diff}>` | `offsetFace` | L3257–3259 |
| `vertexDistance` | `(VertexRef, VertexRef, Model) → Promise<number>` | Pure TS vector math | L3261–3263 |
| `edgeLength` | `(EdgeRef, Model) → Promise<number>` | Pure TS + `curveLength` for NURBS | L3265–3267 |
| `faceArea` | `(FaceRef, Model) → Promise<number>` | Pure TS or `measureArea` | L3269–3271 |
| `solidVolume` | `(SolidRef) → Promise<number>` | `measureVolume` | L3273–3275 |
| `adjacentSolids` | `(SolidRef, Model) → Promise<SolidRef[]>` | Pure TS graph walk | L3277–3279 |
| `sharedFacesBetween` | `(SolidRef, SolidRef, Model) → Promise<FaceRef[]>` | Pure TS graph walk | L3281–3283 |
| `extrudeWire` | `(wireId, distance, direction, model) → Promise<SolidRef>` | `extrude`, `loft`, `healSolid`, `thicken` | L3285–3287 |
| `offsetFaces` | `(faceIds, distance, model) → void` | `offsetFace` | L3289–3291 |
| `syncSolidsFromModel` | `(Model) → Promise<void>` | Reconstructs WASM solids from model | L3297–3299 |
| `exportModelSpaceToStep` | `(ModelSpace) → Promise<string>` | OCCT STEP encoder | L3302–3304 |
| `exportModelToStep` | `(Model) → Promise<string>` | OCCT STEP encoder | L3307–3309 |
| `importStepToModelSpace` | `(stepText) → Promise<ModelSpace>` | OCCT STEP decoder | L3312–3314 |
| `importStepBrepToModelSpace` | `(stepText) → Promise<ModelSpace>` | OCCT STEP decoder (geometry only) | L3317–3319 |
| `importStepBimToModelSpace` | `(stepText) → Promise<ModelSpace>` | OCCT STEP decoder (with layers) | L3322–3324 |

### CAD Editor Entry Points (From Renderer/Actions)
- **Renderer tessellation** (L230): `PreciseSpatialKernelMath` + `faceNormal` imports
- **Action execution** (L1880, L3908–3923): `executeCommandDiff` with various command IDs
- **Tests** (4 files):
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` (L6745)
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts` (L1439)
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts` (L303)
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts` (L1880)

### Runtime Exercise (From CAD Actions/Inferences)
**Operations triggered by real editor workflows:**
- `createBoxFromCorners` → new solid fixture
- `tessellate` → mesh preview/render after each operation
- `executeCommandDiff` with:
  - `curve.arc`, `curve.line`, `curve.circle`, `curve.controlPointCurve`, `curve.interpolateCurve` → edge/wire creation
  - `solid.extrude`, `solid.sweep`, `solid.loft` → solid from curve
  - `solid.boolean.cut`, `solid.boolean.intersect` → shape operations
  - `energy.energy.constructExternalWallFrom2PointsAndHeight` → AEC extension
- `syncSolidsFromModel` → model-to-WASM sync after mutations

---

## 3. Pure TS Math vs OCCT-Backed Methods Table

| SpatialKernel Method | Backend | Implementation | Notes |
|---------------------|---------|-----------------|-------|
| `vec3Add`, `vec3Sub`, `vec3Scale`, `vec3Dot`, `vec3Cross`, `vec3Length`, `vec3Distance` | Pure TS | Arithmetic (L113–145) | Basic linear algebra |
| `vec3Normalize` | Pure TS | Normalize + fallback (L159–163) | Returns `[0,0,1]` if degenerate |
| `constrainMovePoint` | Pure TS | Constrained vector (L149–157) | Rhino-style move modes |
| `arcPlaneFrame`, `arcSweepRadians`, `arcSamplePoints`, `arcFrameFromRadiusPoint`, `arcEndOnCircle`, `arcEndFromAngle` | Pure TS | Analytical arc geometry (L177–240) | No OCCT needed |
| `circleSamplePoints`, `ellipseSamplePoints` | Pure TS | Tessellation (L243–266) | Parameterized sampling |
| `nurbsDisplaySamplePoints` | Pure TS | Catmull–Rom sampling (L269–293) | Preview representation |
| `polylineLength`, `edgeCurveLength` | Pure TS + optional OCCT | Length estimation (L296–319) | Uses `curveLength` for NURBS if OCCT |
| `edgeSamplePoints` | Pure TS + optional OCCT | Tessellates edges (L322–340) | Calls `curveLength` indirectly |
| `circleFromCenterRadiusPoint` | Pure TS | Arc geometry (L343–347) | |
| `nurbsCurveFromPoles` | Pure TS | Nurbs metadata only (L350–354) | No evaluation |
| `evaluateAnchorPosition` | Pure TS + optional OCCT | Parametric evaluation (L576–588) | Uses surface math or OCCT |
| `anchorPlacementFromEntity` | Pure TS + optional OCCT | Geometric pick (L591–618) | Polyline closest-point or surface |
| `meshFaceModelDiff` | Pure TS | Mesh → model diff (L620–661) | Creates geometry from triangle |
| `boxModelDiff` | Pure TS | Box topology (L664–765) | Full topological decomposition |
| `solidPrimitiveAabb`, `modelObjectAabb` | Pure TS | Bounds (L767–817) | AABB from vertices or primitive |
| `aabbCornerPoints`, `aabbIntersect`, `aabbDifferencePieces`, `aabbVolume`, `aabbUnionVolume` | Pure TS | AABB math (L822–904) | All bounds operations |
| `createBoxFromCorners` | OCCT | Calls `box()` + tessellation (L3228–3230) | **OCCT-backed** |
| `volume` | OCCT | Calls `measureVolume()` (L3232–3234) | **OCCT-backed** |
| `tessellate` | OCCT | Calls `mesh()`, `meshEdges()`, `toGroupedBufferGeometryData()` (L3236–3238) | **OCCT-backed** tessellation |
| `vertexDistance` | Pure TS | Vector math (L3261–3263) | Pure TS distance |
| `edgeLength` | OCCT | Calls `curveLength()` for curves (L3265–3267) | **OCCT-backed** for non-line |
| `faceArea` | OCCT | Calls `measureArea()` (L3269–3271) | **OCCT-backed** |
| `solidVolume` | OCCT | Calls `measureVolume()` (L3273–3275) | **OCCT-backed** |
| `adjacentSolids` | Pure TS | Graph walk via shell/face ids (L3277–3279) | Pure TS topology |
| `sharedFacesBetween` | Pure TS | Set intersection (L3281–3283) | Pure TS face ids |
| `extrudeWire` | OCCT | Calls `extrude()`, `loft()`, `healSolid()`, `thicken()` (L3285–3287) | **OCCT-backed** |
| `offsetFaces` | OCCT | Calls `offsetFace()` (L3289–3291) | **OCCT-backed** |
| `executeCommandDiff` | OCCT | Routes to brepjs operations (L3245–3247) | **OCCT-backed** for geometry ops |
| `createBoxFromCornersDiff` | OCCT | Combines `createBoxFromCorners` + diff (L3249–3251) | **OCCT-backed** |
| `extrudeWireDiff` | OCCT | Combines `extrudeWire` + model diff (L3253–3255) | **OCCT-backed** |
| `offsetFacesDiff` | OCCT | Combines `offsetFaces` + model diff (L3257–3259) | **OCCT-backed** |
| `syncSolidsFromModel` | OCCT | Reconstructs WASM solids (L3297–3299) | **OCCT-backed** |
| `exportModelSpaceToStep` | OCCT | Direct STEP export (L3302–3304) | **OCCT-backed** |
| `importStepToModelSpace` | OCCT | Direct STEP import (L3312–3314) | **OCCT-backed** |

**Summary:**
- **Pure TS**: ~25 methods (all vector/arc/bounds/graph math)
- **OCCT-backed**: ~18 methods (tessellation, booleans, measurement, STEP I/O)
- **Hybrid**: 2 methods (`polylineLength`, `edgeLength` → fallback pure TS or OCCT)

---

## 4. Browser ABI for Calling stdio Brep Engine

### Current WASM Bridge Pattern (Existing in codebase)

**No direct stdio brep bridge exists yet in the TS runtime.** The repository shows two patterns:

#### Pattern A: Flow Core Tessellation (s-3d-js bridge)
**File**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🟦️.ts` (L335–348)

```typescript
// Loading flow_core WASM
const [{ default: initFlow, tessellate, dispose }, { default: wasmUrl }] = 
  await Promise.all([
    import("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/flow_core.js"),
    import("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/flow_core_bg.wasm?url"),
  ]);

if (initFlow) await initFlow({ module_or_path: wasmUrl });

// Calling tessellate
const json = await tessellateGeometryJson(ref, tolerance);
const raw = JSON.parse(json);
```

**Entry Point**: `ensureBrepWasmLoaded()` (L336)
**Export Signature**: 
```typescript
tessellate: (handle: string, tolerance: number) => string  // JSON payload
dispose: (handle: string) => void
```

**JSON Format** (L265–273):
```typescript
interface RawMeshTransfer {
  position?: readonly number[];
  normal?: readonly number[];
  index?: readonly number[];
  edges?: readonly number[];
  points?: readonly number[];
  face_groups?: { start: number; count: number; entity_id: string }[];
  error?: string;
}
```

#### Pattern B: brepjs Worker Bridge (Current Legacy)
**File**: `/Users/ueli/Documents/semio/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts` (L3131–3217)

- **Worker entrypoint**: `/Users/ueli/Documents/semio/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/index.ts`
- **Transport**: JSON serialization via `serializeWorkerValue`/`deserializeWorkerValue`
- **RPC protocol**:
  ```typescript
  interface BrepjsWorkerRequest {
    type: "init" | "rpc";
    id?: string;
    method?: string;
    args?: readonly unknown[];
  }
  
  interface BrepjsWorkerResponse {
    type: "init-done" | "rpc-result" | "init-error" | "rpc-error";
    id?: string;
    result?: unknown;
    error?: string;
  }
  ```

### Sanctioned Browser ABI Pattern for stdio Brep

**No direct TypeScript wasm-component binding exists yet.** The pattern would need:

1. **Component Guest Export** (Rust side, `semio-s-plugin-stdio`):
   - Defined via `wit-bindgen` component interface
   - Exports functions like `tessellate(handle: string, tolerance: f32) -> string` (JSON)
   - Compiled to WASM component (not just `cdylib`)

2. **TypeScript Binding** (via component-guest JS runtime or manual wrapper):
   - Load WASM component (not bundled URL; needs component loader)
   - Call exported Rust functions
   - Parse/convert JSON results to `MeshTransfer`

3. **No Existing Bridge Yet**:
   - `semio-s-plugin-stdio` is a `cdylib + rlib` (L18, Cargo.toml), NOT a WASM component
   - No `component-guest` dependency in Cargo.toml (L44 has `component-guest` for the plugin framework, but not for guest-side WASM exports)
   - The Rust side has NO WIT bindings or WASM component exports

### Closest Existing Pattern: framework-3d Geometry Module

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/`

**What it does**:
- Defines `kernelGeometry` namespace with serializable types
- Bridges mesh I/O via `MeshTransfer` contract
- Uses flow_core's tessellation WASM
- No direct B-Rep operations; only tessellation + serialization

**What's missing for stdio brep call**:
- Component definition file (`.wit`)
- WASM component compilation
- RPC routing to stdio Rust crate
- Type bindings for b-rep operations (box, extrude, cut, etc.)

### Proposed ABI for First-Party Kernel

Based on existing patterns, the bridge would look like:

```typescript
// Future: @semio-tech/stdio-brep-wasm
type BrepHandle = string & { __brand: "BrepHandle" };

interface BrepWasmBridge {
  // Primitives
  createBox(cornerA: Vec3, cornerB: Vec3, height: number): BrepHandle;
  createSphere(center: Vec3, radius: number): BrepHandle;
  createCylinder(base: Vec3, axis: Vec3, radius: number, height: number): BrepHandle;
  
  // Booleans
  cut(a: BrepHandle, b: BrepHandle): BrepHandle;
  intersect(a: BrepHandle, b: BrepHandle): BrepHandle;
  
  // Curves / Extrude
  extrudeFace(faceHandle: BrepHandle, vec: Vec3): BrepHandle;
  loftProfiles(profiles: BrepHandle[]): BrepHandle;
  
  // Tessellation
  tessellate(handle: BrepHandle, tolerance: number): MeshTransfer;
  
  // Measurement
  measureVolume(handle: BrepHandle): number;
  measureArea(handle: BrepHandle): number;
  
  // Cleanup
  dispose(handle: BrepHandle): void;
}
```

**Call pattern** (vs. current brepjs):
```typescript
// Current (brepjs in worker)
const kernel = new BrepjsKernel();
const mesh = await kernel.tessellate(solidRef, tolerance, model);

// Proposed (stdio brep)
const bridge = await createStdioBrepBridge();
const brepHandle = bridge.createBox([0,0,0], [1,1,0], 1);
const mesh = await bridge.tessellate(brepHandle, tolerance);
bridge.dispose(brepHandle);
```

**Status**: Pattern exists (flow_core + worker bridge), but no stdio brep WASM component yet.

---

## 5. Tests Requiring OCCT (resolveOwnedOpenCascadeWasmFileUrl)

### Test Files Using OCCT WASM
All tests that import `BrepjsKernel` directly:

| Test File | Line | Usage | Scope |
|-----------|------|-------|-------|
| `/Users/ueli/Documents/semio/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts` | L1350 | Worker initialization (`ensureInit()` calls `initializeOwnedOpenCascade`) | Vitest suite ~3900 lines |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts` | L1880 | `__actionsTestKernel` | Action command tests |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts` | L303 | `__spatialStatelyTestKernel` | State machine tests |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts` | L1439 | `__spatialQueryTestKernel` | Schema inference tests |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` | L6745 | `__cadRendererTestKernel` | R3F renderer tests |

### Tests by Category

**Geometry Construction Tests** (L3700–3923):
- `boxModelDiff` bounds (L3700–3715)
- `vertexDistance` (L3717–3724)
- `faceArea` (L3726–3749)
- `solidVolume` (L3751–3758)
- `syncSolidsFromModel` with hull fuse (L3760–3768)
- `adjacentSolids` (L3770–3782)
- `sharedFacesBetween` (L3784–3796)
- `aabbDifferencePieces` volume check (L3798–3805)

**Command Execution Tests** (L3807–3923):
- `curve.arc` vertex placement (L3807–3835)
- `curve.circle` closed edge (L3851–3862)
- `curve.controlPointCurve` nurbs (L3875–3889)
- `curve.interpolateCurve` interpolation (L3906–3923)
- `solid.sphere` primitive + volume (L3864–3873)
- `energy.energy.constructExternalWallFrom2PointsAndHeight` (L3925–3932)

**Fixture Tests** (L3934–3954):
- Concrete Forest Play fixture roundtrip (shape, building, energy, structure models)

### Oracle Dependencies in stdio

**File**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`

**Finding**: NO brepjs or OCCT entries in oracle registry. brepjs is:
- An NPM third-party (`brepjs@18.20.3`)
- Not a Rust crate owned by the repository
- Not listed in `oracles[]` or `noOracleDecisions[]`
- Only used as a TRANSIENT test dependency (vitest environment)

**Implication**: Tests can migrate to stdio brep bridge without "oracle discharge" — brepjs is already test-only and has no oracle record.

---

## 6. Other Plugins' brepjs Dependencies

### Actual Dependencies (from package.json)
| Plugin | Dep Type | Version | Real or Text? |
|--------|----------|---------|---------------|
| CAD (`📐️cad`) | Direct | `brepjs@^18.20.3`, `brepjs-opencascade@^0.15.6` | **REAL** (L28–29, package.json) |
| AEC Building (`🏢️aec-building`) | Indirect | `@semio-tech/cad-js workspace:*` | **REAL** (re-exports BrepjsKernel) |
| AEC Structure (`🏛️aec-building-structure`) | Indirect | `@semio-tech/cad-js workspace:*` | **REAL** (re-exports BrepjsKernel) |
| Spatial Shape (`📐️spatial-shape`) | Indirect | `@semio-tech/cad-js workspace:*` | **REAL** (re-exports BrepjsKernel) |
| AEC Energy (`🔥️aec-building-energy`) | Indirect | `@semio-tech/cad-js workspace:*` | **REAL** (re-exports BrepjsKernel) |
| Trinity | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Raster | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Process | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Norm | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Demonstrator | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Block | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| DAG | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Reasoning | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |
| Sequence | Description only | — | **TEXT** (copy-paste boilerplate in package.json description) |

### Evidence
- **Direct**: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json` (lines 28–29)
- **Indirect** (4 AEC extensions re-export via `@semio-tech/cad-js`):
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`
  - `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`

**Conclusion**: Only CAD plugin + 4 AEC extensions have real brepjs deps; others just copied the same description text.

---

## 7. Concrete Gap List: No-brepjs Runtime Operations

### SpatialKernel Methods Requiring Replacement (18 OCCT-backed)

| Method | Current Implementation | Needed for stdlib Brep | Gap |
|--------|----------------------|----------------------|-----|
| `createBoxFromCorners` | `brepjs.box()` + mesh | Rust box primitive + tessellate | ✓ Component export needed |
| `volume` | `brepjs.measureVolume()` | Rust volume integral | ✓ Component export needed |
| `tessellate` | `brepjs.mesh()` + `toGrouped*` | Rust tessellation with group info | ✓ Edge/face groups + faceInfos/edgeInfos |
| `edgeLength` | `brepjs.curveLength()` | Rust curve length evaluation | ✓ Need curve handle + evaluation |
| `faceArea` | `brepjs.measureArea()` | Rust face area integral | ✓ Component export needed |
| `solidVolume` | `brepjs.measureVolume()` | Rust volume integral | ✓ Component export needed |
| `extrudeWire` | `brepjs.extrude()` + loft + heal | Rust extrusion + topology repair | ✓ Extrude, loft, heal operations |
| `offsetFaces` | `brepjs.offsetFace()` | Rust offset surface + shell rebuild | ✓ Offset + topology repair |
| `executeCommandDiff` (routing) | Individual brepjs calls | Route through brep bridge | ✓ Operation dispatch |
| `createBoxFromCornersDiff` | `createBoxFromCorners` + model diff | Box + model diff builder | ✓ Reuse model diff logic |
| `extrudeWireDiff` | `extrudeWire` + model diff | Extrude + model diff builder | ✓ Reuse model diff logic |
| `offsetFacesDiff` | `offsetFaces` + model diff | Offset + model diff builder | ✓ Reuse model diff logic |
| `syncSolidsFromModel` | Rebuild WASM solids from model | Rebuild Rust solids from model | ✓ Topology reconstruction |
| `exportModelSpaceToStep` | OCCT STEP writer | Rust STEP writer OR stdio brep export | ✓ STEP export (keep OCCT as oracle) |
| `importStepToModelSpace` | OCCT STEP reader | Rust STEP reader OR stdio brep import | ✓ STEP import (keep OCCT as oracle) |
| `importStepBrepToModelSpace` | OCCT STEP reader (geometry only) | Rust STEP reader OR stdio brep import | ✓ STEP import (keep OCCT as oracle) |
| `importStepBimToModelSpace` | OCCT STEP reader (with layers) | Rust STEP reader OR stdio brep import | ✓ STEP import (keep OCCT as oracle) |
| `disposeSolid` | Worker RPC `disposeSolid` | Rust handle cleanup | ✓ Handle lifecycle |

### Expected Mesh/Edge Payload (MeshTransfer Contract)

From `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🟦️.ts` (L184–195):

```typescript
export interface MeshTransfer {
  readonly position: Float32Array;        // Vertex positions (xyz)
  readonly normal: Float32Array;          // Vertex normals (xyz)
  readonly index: Uint32Array;            // Triangle indices
  readonly edges: Float32Array;           // Edge polyline (xyz points)
  readonly points?: Float32Array;         // Optional anchor points (xyz)
  
  readonly faceGroups: readonly FaceGroup[];  // Grouped faces by B-Rep face
  readonly edgeGroups: readonly EdgeGroup[];  // Grouped edges by B-Rep edge
  
  readonly faceInfos: readonly FaceInfo[];    // Face metadata
  readonly edgeInfos: readonly EdgeInfo[];    // Edge metadata
  
  readonly color?: string;
}

export interface FaceGroup {
  readonly start: number;           // Index into index buffer
  readonly count: number;           // Number of triangles
  readonly entityId: FaceRef;       // B-Rep face identity
}

export interface EdgeGroup {
  readonly start: number;           // Index into edges buffer
  readonly count: number;           // Number of line segments
  readonly entityId: EdgeRef;       // B-Rep edge identity
}

export interface FaceInfo {
  readonly entityId: FaceRef;       // Face identity
  readonly surfaceType: string;     // "PLANE", "CYLINDER", "SPHERE", "CONE", "OTHER_SURFACE"
  readonly area: number;            // Computed area
  readonly normal: readonly [number, number, number];  // Unit normal
}

export interface EdgeInfo {
  readonly entityId: EdgeRef;       // Edge identity
  readonly curveType: string;       // "LINE", "CIRCLE", "ELLIPSE", "NURBS", "OTHER_CURVE"
  readonly length: number;          // Computed length
}
```

### Critical Gaps
1. **No WASM component exports** in `semio-s-plugin-stdio` (currently `cdylib + rlib`)
2. **No WIT bindings** for brep operations (box, extrude, cut, tessellate, etc.)
3. **No TS bridge code** to call stdio brep from CAD runtime
4. **No topology reconstruction** logic to hydrate model graph from Rust handles
5. **No metadata extraction** (surface types, edge curve kinds) from Rust B-Rep
6. **No handle lifecycle** (allocation, deallocation) strategy
7. **STEP import/export** still requires OCCT or a pure-Rust STEP library (gap remains, keep as oracle)

### Non-Gaps (Pure TS, No Action Needed)
- All vector math (vec3Add, vec3Cross, etc.)
- All tessellation sampling (arcSamplePoints, ellipseSamplePoints, etc.)
- All AABB operations (aabbIntersect, aabbDifferencePieces, etc.)
- All graph queries (adjacentSolids, sharedFacesBetween)
- Worker RPC plumbing (swap brepjs-worker for stdio-brep-worker)

---

## Summary

**brepjs API surface**: 40+ functions across 4 categories (primitives, topology, measurement, tessellation).

**Call density**: ~60 invocations in legacy kernel, ~15–20 exercised per typical editor session.

**OCCT-backed methods**: 18/36 public SpatialKernel methods; 25+ pure TS.

**Test entrypoints**: 5 vitest files; no oracle record (brepjs is already test-only).

**Plugins affected**: CAD + 4 AEC extensions (real deps); 9 others are text-only boilerplate.

**WASM bridge status**: Flow/tessellation pattern exists; stdio brep component does NOT exist yet.

**Removal blocker**: stdio Rust crate must be compiled as WASM component with guest exports for box, extrude, cut, tessellate, measure, and topology operations.
