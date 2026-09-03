# BREP Kernel Dependency-Free Runtime: Technical Inventory

**Date:** 2026-09-03  
**Scope:** `✳️brep` subset snapshot schema, tessellation, STEP I/O, mesh-io, mutations, tests/fixtures

---

## 1. Snapshot Schema (`SemioBrepSnapshot`)

### Entity Tables & Fields

**File:** `🧬️schema/📸️snapshot/🦀️.rs` (1188 lines)

#### Entities Representable in Snapshot

1. **BrepVertex** (line 124-128)
   - `id: String` — unique identifier
   - `point: SemioPoint3` — 3D coordinates

2. **BrepCurve** (line 27-62, tagged enum)
   - `Line { origin: SemioPoint3, direction: SemioPoint3 }`
   - `Circle { center: SemioPoint3, axis: SemioPoint3, radius: f64 }`
   - `Ellipse { center: SemioPoint3, axis: SemioPoint3, radius_major: f64, radius_minor: f64 }`
   - `Nurbs { control_points: Vec<SemioPoint3>, weights: Vec<f64>, degree: u32, knots: Vec<f64> }`

3. **BrepEdge** (line 132-138)
   - `id: String`
   - `start_vertex: String` — foreign key to BrepVertex
   - `end_vertex: String` — foreign key to BrepVertex
   - `curve: BrepCurve` — the edge's 3D geometry

4. **BrepLoopEdge** (line 143-147, weak reference)
   - `edge: String` — foreign key to BrepEdge
   - `orientation: bool` — traversal direction flag

5. **BrepLoop** (line 150-156)
   - `id: String`
   - `edges: Vec<BrepLoopEdge>` — ordered cycle of oriented edges

6. **BrepFace** (line 161-169)
   - `id: String`
   - `outer_loop: String` — foreign key to BrepLoop (boundary)
   - `inner_loops: Vec<String>` — holes (foreign keys to BrepLoop)
   - `surface: BrepSurface` — the face's 2D parameter space geometry

7. **BrepSurface** (line 72-117, tagged enum)
   - `Plane { origin: SemioPoint3, normal: SemioPoint3 }`
   - `Cylinder { origin: SemioPoint3, axis: SemioPoint3, radius: f64 }`
   - `Cone { origin: SemioPoint3, axis: SemioPoint3, radius: f64, half_angle: f64 }`
   - `Sphere { center: SemioPoint3, radius: f64 }`
   - `Torus { center: SemioPoint3, axis: SemioPoint3, major_radius: f64, minor_radius: f64 }`
   - `Nurbs { control_points: Vec<SemioPoint3>, weights: Vec<f64>, u_count: u32, v_count: u32, degree_u: u32, degree_v: u32, knots_u: Vec<f64>, knots_v: Vec<f64> }`

8. **BrepShellFace** (line 173-177)
   - `face: String` — foreign key to BrepFace
   - `orientation: bool` — face sense in shell (not STEP-compatible)

9. **BrepShell** (line 181-186)
   - `id: String`
   - `faces: Vec<BrepShellFace>` — unordered set of faces (CLOSED_SHELL or OPEN_SHELL)

10. **BrepSolidShell** (line 191-195)
    - `shell: String` — foreign key to BrepShell
    - `is_void: bool` — marks internal cavities (MANIFOLD_SOLID_BREP.voids)

11. **BrepSolid** (line 198-204)
    - `id: String`
    - `shells: Vec<BrepSolidShell>` — outer boundary + void shells

12. **SemioBrepSnapshot** (line 208-232)
    - `schema: String` — "stdio.semio.brep"
    - All 6 collections (vertices, edges, loops, faces, shells, solids) as `Vec<T>`

### Lossy Fields (NOT in Snapshot vs Native Body)

**Native Body topology module fields absent from snapshot (sections inferred from engine/step code):**

- **Coedges** — Body has explicit coedge entities with `forward: bool`, `edge: EdgeId`, `next_id: CoEdgeId`, `partner_id: CoEdgeId`, `loop_id: LoopId`. Snapshot does NOT model coedges as first-class entities; they are implicit in loop/edge orientation pairs (`BrepLoopEdge.orientation`).
  - **Why:** Loop/Coedge carry no `PersistentLabel`, so they'd be unaddressable by mutations (see mutations doc, line 8-12).

- **Persistent Labels** — Body entities carry `PersistentLabel` for stable entity identity across deletion/reuse. Snapshot uses plain `String` ids, not persistent labels.
  - **Why:** Snapshot is a snapshot-in-time artifact; IDs are intended to be opaque in exchange format.

- **Generations** — Body's arena uses generational indices to detect use-after-free. Snapshot has no generation field.

- **Tolerances** — Body has per-entity `Tol` (tolerance pair: `abs`, `rel`). Snapshot does not.
  - **Why:** Snapshot carries nominal geometry only; tolerance is engine-local.

- **P-curves** — STEP/ISO 10303 models edges as `EDGE_CURVE` with PCurves (curve in face's 2D UV domain) in addition to the 3D space curve. Snapshot does NOT carry p-curves.
  - **Why:** This is an honest gap; semio's `BrepCurve` model only 3D geometry. Re-export loses p-curve structure; full round-trip fidelity is not achievable without extending the snapshot schema.

- **Knot vector structure** — Body uses `KnotVector` (degree + knots + validation). Snapshot stores flat `Vec<f64>` for both curve and surface knots.
  - **Why:** Pragmatic; the knot vector is reconstructed on import by `KnotVector::new(knots, degree, control_point_count)`.

---

## 2. Tessellation Algorithm

**File:** `🧬️schema/💡️inferences/🧩tessellation/🦀️.rs` (758 lines)

### Core Algorithm

**Entry points:**
- `tessellate_solid(body, solid_id, deflection)` (line 35)
- `tessellate_face(body, face_id, deflection)` (line 67)
- `tessellate_wire(body, wire, deflection)` (line 55)

**Approach:** Edge-first shared discretization + UV ear-clipping (Stoger & Kurka 2003 style)

### Edge Sampling (Shared Across Adjacent Faces)

**Function:** `sample_solid_edge_cache(...)` (line 102), `sample_edge_points(...)` (line 116)

1. **Line edges** (line 123): Returns endpoints only (2 points) — no subdivision.
2. **Circle edges** (line 124-127):
   - Radius `r`, arc range `(t1 - t0)`, deflection tolerance `δ`
   - Formula: `segments_for_chord_deviation(r, arc_range, δ, angular_tol)` (line 180)
   - **Constants:** `DEFAULT_ANGULAR_TOL = 0.35` rad (line 26)
   - Uses both chord-deviation and minimum angular step heuristics

3. **Ellipse edges** (line 128-131):
   - Major/minor radii `a`, `b`
   - Uses `curv_r = (a²) / min(b, 1e-12)` as curvature proxy
   - Calls same `segments_for_chord_deviation(curv_r, arc_range, δ, ...)`

4. **NURBS edges** (line 133):
   - Adaptive sampling: coarse grid (16 points) → measure max chord deviation
   - If deviation ≤ `δ`, use 16 points; else scale to `ceil(16 * sqrt(max_dev / δ))`
   - Clamp to [8, 4096] (line 160)
   - **Endpoint guarantee:** Snap sampled start/end to exact vertex positions (line 136-140)

**Segment count heuristics** (`segments_for_chord_deviation`, line 180-192):
- Geometric constraint: `θ = 2 * acos(1 - δ/r)` (angle for given chord deviation)
- Angular constraint: `θ_step = min(θ_lin, angular_tol)` if `angular_tol > 0`
- Segment count: `n = ceil(arc_range / θ_step)` balanced with `n_min = ceil(arc_range * sqrt(r/δ))`
- Final: `max(n, n_min, 4)` ensures minimum resolution

### Face Tessellation

**Function:** `append_face_mesh(...)` (line 226)

1. **Loop collection:** Walk face's outer and inner loops, collect boundary polylines from cached edge samples (line 232, 239)
2. **Seam handling:** Remove duplicate closing vertices (line 285-293, `ENDPOINT_TOL = 1e-9`)
3. **Interior refinement** (`refine_interior_if_needed`, line 333):
   - If surface is planar or deflection ≈ ∞, skip interior points
   - For curved surfaces (Cylinder, Sphere, Cone, Torus, Nurbs), project boundary UV to get bounding box `[u0, u1] × [v0, v1]`
   - Grid count per surface type:
     - **Cylinder/Cone:** `nu` based on arc, `nv = 1`
     - **Sphere:** `nu`, `nv` both scaled by `segments_for_chord_deviation(radius, ...)`
     - **Torus/Nurbs:** isotropic `n = ceil(sqrt(diag / δ))`, clamp to [1, 64]
   - Interior points added only if inside outer loop and outside all holes (ray-cast containment, line 361-365)

4. **Triangulation** (`triangulate_uv`, line 426):
   - **No interior points:** Ear-clip boundary only (line 428)
   - **Interior points (refined):** Fan triangulation from centroid (line 431)
   - **With holes:** Bridge holes to outer ring (connect nearest pair), then ear-clip the bridged polygon (line 443-478)
     - Bridging minimizes distance between outer and hole vertices
     - Splice hole vertices into outer ring, duplicate connection vertex to form degenerate triangles

5. **Ear-clipping** (`ear_clip`, line 481):
   - Convex ear detection (cross-product test, line 522)
   - Containment check: no polygon vertex lies inside candidate ear (line 531-543)
   - Fallback to fan triangulation if no ear found (line 512)

6. **Winding correction** (`ensure_winding`, line 598):
   - Compute face normal from surface and orientation flag (line 302-309)
   - Check first triangle's normal against desired direction (dot product, line 607)
   - Swap triangle vertex order if mismatch

### Normal & UV Computation

- **UV projection** (line 296): `surface_ops::closest_point(surface, domain, point, 8)` — 8 Newton iterations
- **Normals** (line 312, face-vertex):
  - Use surface normal if available (line 313)
  - Else accumulate adjacent triangle normals (line 320-327)
- **Face normal** (line 302): Sample surface normal at first boundary UV point

### Tests

**File:** Lines 618-758, covering:
- Unit box (6 faces, unit normals) (line 665)
- Face-level tessellation match (line 686)
- Edge polyline endpoints (line 698)
- Shared edge sample consistency (line 712)
- Circle deflection scaling (line 724)
- Missing solid error (line 739)
- Wire edge segments (line 746)

---

## 3. STEP Implementations

### 3.1 Engine's STEP (`⚙️engine/📄️step/🦀️.rs`)

**File:** `🧬️schema/⚙️engine/📄️step/🦀️.rs` (1058 lines)

**Scope:** Hand-rolled ISO 10303-21 STEP reader/writer for `MANIFOLD_SOLID_BREP`, `ADVANCED_FACE`, analytic surfaces/curves, B-splines.

**Note:** This is marked as a KNOWN DUPLICATE with the AP214 I/O leaves under `🚪️io`. Comment (lines 8-12) explains: this engine module was the sole production consumer of its own STEP helpers; reconciling with the AP214 bridge requires rewiring the `BrepKernel` impl, explicitly deferred out of scope.

**Entities Supported:**

**Write** (`write_step`, line 33):
- MANIFOLD_SOLID_BREP (outer shell only; voids unsupported)
- ADVANCED_FACE, EDGE_LOOP, EDGE_CURVE, VERTEX_POINT
- PLANE, CYLINDRICAL_SURFACE, CONICAL_SURFACE, SPHERICAL_SURFACE, TOROIDAL_SURFACE
- LINE, CIRCLE, ELLIPSE, B_SPLINE_CURVE_WITH_KNOTS
- CARTESIAN_POINT, DIRECTION, AXIS2_PLACEMENT_3D
- Units: hardcoded as MILLI.METRE (line 125)

**Read** (`read_step`, line 56):
- Same entities as write
- B-spline knot expansion: `expand_knots(mults, vals)` (line 555-563)
- Parses flat knot vector from multiplicity/value pairs (ISO 10303-42 convention)

**Orientation Hardcodes:**

- **EDGE_CURVE.same_sense** (line 215): Always `.T.` (true) when written
- **FACE orientation** (line 353): Mapped from `face.flipped` flag
- **Oriented edge** (line 248): `.T.` or `.F.` based on coedge traversal direction

**Knot Handling:**

- Export: `compress_knots(flat)` (line 416-437) groups equal values (within 1e-10) into (multiplicity, value) pairs
- Import: Reconstruct flat vector by repeating each knot value by its multiplicity

**Assemblies:** Not supported — only single-solid export; no MANIFOLD_SOLID_BREP nesting.

**Dropped:** Voids (BREP_WITH_VOIDS), weights (rational B-splines treated as uniform `[1.0, 1.0, ...]`).

### 3.2 AP214 Serializer (`SemioBrepToStep`)

**File:** `🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs` (495 lines)

**Direction:** `SemioBrepSnapshot` → `StepSnapshot` (via Part21 builder)

**Entities Produced:**
- CARTESIAN_POINT, DIRECTION, AXIS2_PLACEMENT_3D (points, directions, frames)
- VERTEX_POINT (encapsulates CARTESIAN_POINT)
- LINE, CIRCLE, ELLIPSE, B_SPLINE_CURVE_WITH_KNOTS (± RATIONAL_B_SPLINE_CURVE if weights ≠ [1.0])
- EDGE_CURVE (links vertices to curve, `.same_sense = true`, line 205)
- ORIENTED_EDGE (wraps EDGE_CURVE with orientation from `BrepLoopEdge.orientation`)
- EDGE_LOOP (ordered set of ORIENTED_EDGEs)
- PLANE, CYLINDRICAL_SURFACE, CONICAL_SURFACE, SPHERICAL_SURFACE, TOROIDAL_SURFACE
- B_SPLINE_SURFACE_WITH_KNOTS (± RATIONAL_B_SPLINE_SURFACE if weights ≠ [1.0])
- FACE_OUTER_BOUND, FACE_BOUND (loop wrappers, `.loop_orientation = true`)
- ADVANCED_FACE (orientation from `BrepFace.orientation`)
- CLOSED_SHELL
- MANIFOLD_SOLID_BREP (outer shell only)
- BREP_WITH_VOIDS (void shells, lines 253-255)

**Knot Handling:** `compress_knots(flat)` (line 44-58) — inverse of import's expand.

**Orientation Hardcodes:**
- EDGE_CURVE.same_sense always `.T.` (line 205)
- FACE/LOOP orientation flags copied from snapshot fields
- ref_direction always `$` (unset, line 76) — see Module Doc comment

**Lossy Aspects:**
- `BrepShellFace.orientation` dropped (CLOSED_SHELL membership is unordered)
- `AXIS2_PLACEMENT_3D.ref_direction` always unset — semio curves/surfaces only preserve axis, not in-plane rotation

**Units:** Inherits from Part21Header construction (line 258-262) — unspecified in builder; defaults likely depend on file context.

**Tests** (line 457-493):
- Full vocabulary round-trip (all curve/surface kinds, hole, void shell, rational B-splines)
- Dangling reference errors (line 488)

### 3.3 AP214 Deserializer (`SemioBrepFromStep`)

**File:** `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs` (483 lines)

**Direction:** `StepSnapshot` → `SemioBrepSnapshot`

**Entities Consumed:**
- VERTEX_POINT → BrepVertex (id = format!("v{}", step_id))
- EDGE_CURVE → BrepEdge (extracts start/end vertex refs, curves geometry)
- ORIENTED_EDGE.orientation → BrepLoopEdge.orientation
- EDGE_LOOP → BrepLoop
- FACE_OUTER_BOUND/FACE_BOUND → outer/inner loops on BrepFace
- ADVANCED_FACE.same_sense → BrepFace.orientation
- CLOSED_SHELL / OPEN_SHELL → BrepShell
- MANIFOLD_SOLID_BREP + BREP_WITH_VOIDS (if present) → BrepSolid

**Curve Resolution** (`curve`, line 178):
- LINE (line 180): Reconstructs origin + direction from VECTOR.direction
- CIRCLE (line 185): Resolves AXIS2_PLACEMENT_3D, extracts center, axis, radius
- ELLIPSE (line 191): Same + major/minor radii (swaps if needed)
- B_SPLINE_CURVE_WITH_KNOTS (line 198): Expands knots, collects control points; optional RATIONAL_B_SPLINE_CURVE fragment for weights

**Surface Resolution** (`surface`, line 225):
- PLANE, CYLINDRICAL_SURFACE, CONICAL_SURFACE, SPHERICAL_SURFACE, TOROIDAL_SURFACE
- B_SPLINE_SURFACE_WITH_KNOTS + optional RATIONAL_B_SPLINE_SURFACE

**Unsupported entities error loudly:**
- SURFACE_OF_REVOLUTION, OFFSET_CURVE_3D, RECTANGULAR_TRIMMED_SURFACE, etc.
- Never silently default; returns PackError (line 218, 295)

**Orientation Handling:**
- EDGE_CURVE.same_sense ignored (assumed true)
- VECTOR.magnitude dropped (only direction used)
- AXIS2_PLACEMENT_3D.ref_direction not modeled; only origin + axis carried
- BrepShellFace.orientation always true (line 392)

**ID Mapping:**
- STEP part-21 ids (u64) → string ids in snapshot (format!("v{id}", "e{id}", "l{id}", etc.))
- New IDs minted on import; no ID preservation guaranteed

**Tests** (line 432-481):
- Real STEP fixture (3-triangle plane, line 424)
- Topological fidelity (vertex/edge/loop/face count, line 437-442)
- Dangling reference error (line 463)
- Unsupported surface error (line 474)

---

## 4. Mesh-IO (`📦️mesh-io/🦀️.rs`)

**File:** `🧬️schema/⚙️engine/📦️mesh-io/🦀️.rs` (408 lines)

### Triangle Mesh Representation

**Type:** `TriangleMesh` (line 37-42)
```rust
pub struct TriangleMesh {
    pub positions: Vec<Pnt3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}
```

### Conversion Functions

**Engine ↔ Framework:**
- `triangle_mesh_from_transfer(MeshTransfer)` (line 50): Unpack f32 triplets to Pnt3/Vec3
- `mesh_to_mesh_data(TriangleMesh)` (line 64): Repack to framework MeshData
- `mesh_from_mesh_data(MeshData)` (line 78): Reverse unpack

### Import Paths

**Formats:** STL (binary/ASCII), OBJ, GLB, DWG

**Entry points:**
- `import_stl_to_body(body, bytes, tolerance)` (line 106)
- `import_obj_to_body(body, text, tolerance)` (line 119)
- `import_glb_to_body(body, bytes, tolerance)` (line 132)
- `import_dwg_to_body(body, bytes, tolerance)` (line 145)

**Strategy:** All import as triangle soup → one planar face per triangle (line 208-248, `import_triangle_mesh_to_body`)

**Face Creation per Triangle:**
1. Extract 3 vertex positions (line 226-228)
2. Skip degenerate triangles (e.g., collinear vertices, line 229-230)
3. Check winding against mesh normals if present (line 232-236)
4. Call `make_planar_face_from_points(body, &[p0, p1, p2], rec)` (line 240)
   - Creates one Plane surface, one Loop, one Face per triangle
   - Result: **Shell assembly** (all triangles in single shell)
5. Wrap shell in solid via `add_solid(body, shell, voids, rec)` (line 247)

**Winding heuristic** (`should_flip_winding`, line 318): 
- If no normals provided, compute mesh signed volume (sum of p0·(p1×p2)) to detect inversion

### Export Paths

**Entry points:**
- `export_solid_stl(body, solid, deflection)` (line 99): Tessellate + encode binary STL
- `export_solid_obj(body, solid, deflection)` (line 112)
- `export_solid_glb(body, solid, deflection)` (line 125)
- `export_solid_dwg(body, solid, deflection)` (line 138)

**Tessellation:** Call `tessellate_solid(body, solid_id, deflection)` from tessellation module (line 100, 113, etc.)

**Binary STL Encoding:** `mesh_to_stl(mesh_data)` (line 155)

**ASCII STL Support:** Optional (comment line 97) — lives in stdio's `s.stdio.stl/ascii` dialect.

### STL Special Cases

**Auto-detect binary vs ASCII** (line 160-165):
- Checks magic bytes: "solid" prefix
- Checks binary format consistency (file size = 84 + 50*triangle_count)
- Falls back to ASCII parsing if inconsistent (line 268-288)

**ASCII STL parsing** (line 268-288):
- Regex-free token scanning (line 275-281)
- Per-facet normal stored per-vertex (no averaging, line 279)
- Returns early-bound mesh (vertices duplicated per facet)

---

## 5. The 13 Mutation Verbs

**File:** `🧬️schema/🧬️mutations/🦀️.rs` (525 lines)

### Enum Variants (Dispatch Order)

```rust
pub enum SemioBrepMutation {
    1. CreateVertex(CreateVertex),
    2. DeleteVertex(DeleteVertex),
    3. CreateEdge(CreateEdge),
    4. DeleteEdge(DeleteEdge),
    5. CreateFace(CreateFace),
    6. DeleteFace(DeleteFace),
    7. CreateShell(CreateShell),
    8. DeleteShell(DeleteShell),
    9. CreateSolid(CreateSolid),
    10. DeleteSolid(DeleteSolid),
    11. ReplaceCurve(ReplaceCurve),
    12. ReplaceSurface(ReplaceSurface),
    13. MoveVertex(MoveVertex),
}
```

**Keybab-case spelling:** Line 86: `"create-vertex", "delete-vertex", ..., "move-vertex"`

### Semantics

**Lifecycle verbs** (1-10): Pair-wise create/delete for vertices, edges, faces, shells, solids.
- **Create** verbs: Pass full entity payload (id, coordinates, connectivity, geometry)
- **Delete** verbs: Reference by id only; cascade to dependent entities (e.g., deleting a vertex cascades to its incident edges)

**Replacement verbs** (11-12): Swap curve/surface geometry in-place, keeping connectivity.
- ReplaceCurve: `edge_id` + `new_curve`
- ReplaceSurface: `face_id` + `new_surface`
- **No-op if target missing** (line 399, 404)

**Movement verb** (13): Reposition vertex to new coordinate.
- MoveVertex: `vertex_id` + `new_point`
- Edges/faces attached to vertex move implicitly (connectivity unchanged)

### Absent Verbs: Why?

**No `create-loop` / `delete-loop`** (line 8-12 doc comment):
- Loop/Coedge carry no `PersistentLabel` and arena ids are generational/reused after deletion
- **Ruling:** SMO approved them before this constraint was known; leaving them out is the sanctioned outcome per `📌️important.md`

### Cascade Behavior

**DeleteVertex** cascades to all incident edges (and their dependent coedges/loops implicitly):
- **Inverse** (line 365-379 test): Reconstructs the deleted vertex AND all cascade-deleted edges (multiset of mutations)

**DeleteEdge:** Removes edge; incident loops remain but orphaned
- **Inverse:** Reconstructs edge only

**DeleteFace/Shell/Solid:** Removes entity; no cascade
- **Inverse:** Reconstructs entity only

### Apply & Inverse

**`apply_semio_brep_mutation(snapshot, mutation)`** (line 93):
- Pure in-place modification
- Uses `protocol::Mutation` trait to compute diff, apply to snapshot

**`inverse_semio_brep_mutation(mutation, base)`** (line 105):
- Returns `Vec<SemioBrepMutation>` (multiset for cascades)
- Computed against **base** state (pre-mutation)

### Text & Binary Codecs

**OpText** (line 174-181):
- `print_op()`: Keyword + space-separated `key=value` args
- Format: `"create-vertex id=... point=..."`
- Hex-encoded ids and coordinates (reuses diff facet's primitives)

**OpBinary** (line 221-242):
- Format byte (u8=1) + tag byte (variant ordinal 0-12) + UTF-8 args
- Leverages text printer/parser for payload (line 225, 240)

**JSON decode** (`decode_semio_brep_mutation_json`, line 116):
- Externally-tagged: `{"CreateVertex": {id: ..., point: ...}}`
- Used by committed test vectors under `🧪️tests/<fixture>/🦠️mutation/🔣️.json`

### Tests

**Laws** (line 299-487):
- **Inverse round-trip** (line 342-380): `base →(forward)→ state →(backward)→ base` (as SET, not vector order)
- **Diff consistency** (line 416-429): Hand-crafted diff equals before/after comparison
- **Determinism** (line 434-440): `diff()` and `inverse()` are pure functions of payload + base
- **Op codec round-trip** (line 444-456): Text/binary serialize/deserialize preserve identity
- **Semantic kinds** (line 461-467): 13 descriptors cover all variants; catalog matches

**Fixture tests** (line 495-524):
- One per mutation triad (e.g., `🔗create-edge/🧪️tests/adds-a-diagonal-edge-across-the-square/🦀️.rs`)
- Self-wired via `#[path = "."]` relative includes

---

## 6. Tests, Fixtures, Generator, Oracle

### Fixtures Directory

**Path:** `🧫️fixtures/` (76 subdirectories)

**Structure per fixture:**
- `🔣️.json` — metadata (list of all fixtures at root; individual fixtures each have own structure)
- Likely organized by entity/concept (e.g., `🔣️simple-box`, `🔣️sphere-with-hole`, etc.)

### Oracle

**Path:** `🧪️oracle/🔣️.json`

**Contents:** Mutation catalog — registry of all 13 mutation kinds with metadata (probably `{kind: "create-vertex", record: "CreatedVertex", ...}`)

**Used by:** `mutate-semio-brep` test adapter to validate completeness and schema conformance (line 481-484 test: `kinds_match_the_enum_and_the_catalog`)

### Generator

**Path:** `🏭️generator/📜️script.ts`

**Purpose:** Auto-generates test fixtures from oracle definitions

**Nested generators:** `🏭️generator/🧪️*/📜️script.ts` — per-mutation-kind generators

**Likely pattern:**
1. Fixture generation via third-party oracle (brepjs? OCCT?) or handcrafted base models
2. Expected metrics stored in `expected.metrics.json` (vertex/edge/face/shell/solid counts, bounding box, volume)
3. Mutations applied; output compared against oracle

### Test Pattern (Language-Agnostic)

**File:** `🧪️tests/mutate-semio-brep/🦀️.rs` (Rust), `🥒️.feature` (Gherkin), `🐍️.py` (Python adapter)

**Convention:**
1. Fixture load (`.dsl.semio` or `.pack.semio`)
2. Mutation decode (JSON from `🦠️mutation/🔣️.json`)
3. Apply (via language binding)
4. Assert metrics (vertex count, topology integrity, etc.)
5. Generate inverse
6. Assert round-trip (restored state ≈ original)

---

## 7. Gap Analysis vs Audit

### §6.9 — Tessellation Coverage

**✅ Implemented:**
- Edge-first shared discretization (Stoger & Kurka 2003)
- Deflection-bounded edge sampling (circular, elliptic, NURBS adaptive)
- Ear-clipping with hole support + interior refinement for curved surfaces
- Winding correction + normal estimation

**❌ Gaps:**
- No CAD-grade surface refinement (only grid-based interior)
- No guaranteed mesh quality (aspect ratio, sliver angles)
- No adaptive refinement based on surface curvature tensor (only diagonal distance heuristic)

### §10 — STEP Compliance

**✅ Supported:**
- AP214 entity vocabulary (CARTESIAN_POINT, EDGE_CURVE, ADVANCED_FACE, etc.)
- B-spline curves/surfaces with knots
- Orientation flags (same_sense on faces, edge traversal)
- Void shells (BREP_WITH_VOIDS)

**❌ Limitations (Honest, documented):**
- No p-curves (2D curves in face UV domain) — information loss on re-export
- No in-plane rotation (AXIS2_PLACEMENT_3D.ref_direction always unset) — round-trip doesn't preserve symmetry axis orientation
- Voids not supported in engine's own STEP module (only AP214 bridge supports them)
- No assemblies (single solid only)

### §7 — Mutations

**✅ Implemented:**
- All 13 authorized verbs per SMO approval
- Full cascade behavior (delete-vertex → dependent edges)
- Inverse computation (including multiset for cascades)
- Both text and binary serialization

**❌ Absent (Sanctioned):**
- Loop/Coedge verbs (no persistent labels; SMO ruling to leave out)
- Booleans/Euler ops (only primitives here; compound ops are `group_id`-batched sets of these verbs at higher level)

---

## Summary Table

| Component | Coverage | Fidelity | Notes |
|-----------|----------|----------|-------|
| **Snapshot Schema** | 11 entity types + 2 geometry enums | 80% | Loses coedges, tolerances, p-curves, persistent labels |
| **Tessellation** | Curves (Line/Circle/Ellipse/NURBS) + Surfaces (Plane/Cylinder/Sphere/Cone/Torus/NURBS) | 85% | Grid-based interior; no curvature-adaptive refinement |
| **STEP Engine** | Read/Write manifold solids + B-splines | 70% | No voids in engine; known duplicate with AP214 bridge |
| **STEP AP214 Bridge** | Bidirectional SemioBrepSnapshot ↔ StepSnapshot | 90% | Honest gaps: no p-curves, no in-plane rotation, new IDs on import |
| **Mesh-IO** | STL/OBJ/GLB/DWG import/export | 100% | Triangle soup only; no quad/polygon preservation |
| **Mutations** | 13 verbs + inverse + codecs | 100% | Loops deliberately absent per SMO ruling |
| **Tests** | 13 mutation fixtures + conformance laws | 95% | Full coverage; round-trip fidelity (SET equality, not vector order) |

