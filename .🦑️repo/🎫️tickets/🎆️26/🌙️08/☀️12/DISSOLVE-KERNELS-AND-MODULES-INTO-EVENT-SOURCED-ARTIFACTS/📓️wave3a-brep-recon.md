# Wave 3a: B-Rep Kernel Parallelisation Recon

**Date:** 2026-08-12  
**Ticket:** `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`  
**Scope:** File inventory, dependency graph, lane clustering strategy, imperative surface audit, Vec3 blast radius, benchmarks/tests.

---

## 1. File Inventory

**Total files:** 42 component files (1 scene module at top level, 41 brep submodules)  
**Total LOC (brep + mesh + scene):** ~23,840 lines  
**Sorted by size descending:**

| File Path | LOC | Category |
|-----------|-----|----------|
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs` | 2769 | mesh (not brep) |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🎬️scene/🦀️component.rs` | 1671 | scene (not brep) |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` | 1452 | **KERNEL HUB** |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs` | 1034 | io/format |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📏️measure/🦀️component.rs` | 973 | query |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🧱️primitives/🦀️component.rs` | 871 | construction |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🧩️tessellate/🦀️component.rs` | 777 | export/mesh |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🔀️boolean/🦀️component.rs` | 687 | boolean ops |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/➡️sweep/🦀️component.rs` | 679 | construction |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🏷️classify/🦀️component.rs` | 611 | query |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️int-cc/🦀️component.rs` | 565 | intersection |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/↔️offset/🦀️component.rs` | 513 | feature/offset |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️int-cs/🦀️component.rs` | 494 | intersection |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/〰️polynomial/🦀️component.rs` | 469 | math/nurbs |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️int-ss/🦀️component.rs` | 469 | intersection |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️curve-ops/🦀️component.rs` | 464 | nurbs/curve |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` | 434 | io/tessellation |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🪢️bspline/🦀️component.rs` | 414 | nurbs |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🎨️blend/🦀️component.rs` | 406 | feature (fillet/chamfer) |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/➰️curve/🦀️component.rs` | 400 | nurbs/curve |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/➡️vector/🦀️component.rs` | 398 | math |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` | 362 | **HOST/ENGINE** |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🕸️topology/🦀️component.rs` | 349 | topology |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🔢️matrix/🦀️component.rs` | 348 | math |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚖️predicates/🦀️component.rs` | 344 | math |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🔮️oracle/🦀️component.rs` | 327 | query |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🎢️bezier/🦀️component.rs` | 325 | nurbs |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🏄️surface/🦀️component.rs` | 314 | nurbs |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🩹️heal/🦀️component.rs` | 309 | repair |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🔺️euler/🦀️component.rs` | 277 | topology |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🌳️bvh/🦀️component.rs` | 271 | spatial |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🧵️sew/🦀️component.rs` | 260 | construction |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🏟️arena/🦀️component.rs` | 260 | **STORAGE HUB** |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗺️spatial/🦀️component.rs` | 247 | spatial (non-brep) |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/✅️validate/🦀️component.rs` | 243 | validation |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📏️tolerance/🦀️component.rs` | 235 | numeric |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🪡️surface-ops/🦀️component.rs` | 214 | nurbs |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🚨️error/🦀️component.rs` | 183 | types |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📜️history/🦀️component.rs` | 157 | event sourcing |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` | 149 | **HOST/INTERFACE** |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧮️compute/🦀️component.rs` | 13 | async helper |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🏷️classify/🦀️component.rs` | n/a | duplicate? |

---

## 2. Dependency Edges (Intra-Brep Module References)

Each module shown with its brep:: dependencies detected via `use crate::brep::` patterns.

| Module | Direct Dependencies | Dependency Count |
|--------|-------------------|------------------|
| 🧰️**kernel** | arena, blend, boolean, bspline, classify, curve, engine, error, euler, heal, history, int_cc, int_cs, int_ss, mat, measure, mesh_io, offset, primitives, sew, step, surface, sweep, tolerance, topo, validate, vec | **26 deps** ⚠️ MASSIVE HUB |
| 🏷️classify | arena, bvh, curve, engine, error, int_cs, mat, measure, oracle, predicates, primitives, surface, tolerance, topo, vec | 15 deps |
| ➡️sweep | arena, curve, error, euler, history, mat, measure, primitives, surface, tolerance, topo, validate, vec | 13 deps |
| 🧩️tessellate | arena, curve, engine, error, euler, history, mat, primitives, surface, surface_ops, tolerance, topo, vec | 13 deps |
| 📏️measure | arena, curve, curve_ops, error, mat, surface, surface_ops, tolerance, topo, vec | 10 deps |
| ↔️offset | arena, boolean, curve, engine, error, euler, history, measure, primitives, surface, sweep, tessellate, topo, vec | 14 deps |
| 🔀️boolean | arena, classify, engine, error, euler, history, measure, primitives, tessellate, topo, vec | 11 deps |
| 📄️step | arena, bspline, curve, error, euler, history, mat, primitives, surface, tolerance, topo, vec | 12 deps |
| 🎨️blend | arena, error, measure, primitives, surface, topo, vec | 7 deps |
| 🩹️heal | arena, bspline, curve, error, primitives, sew, surface, tolerance, topo, validate | 10 deps |
| ✂️int-cc | bezier, bspline, curve, error, mat, vec | 6 deps |
| ✂️int-cs | curve, error, mat, surface, surface_ops, vec | 6 deps |
| ✂️int-ss | curve, error, mat, surface, vec | 5 deps |
| 🧵️sew | arena, curve, error, euler, history, mat, tolerance, topo, vec | 9 deps |
| ✅️validate | arena, curve, error, euler, history, mat, surface, tolerance, topo, vec | 10 deps |
| 📦️mesh-io | arena, engine, error, euler, history, primitives, tessellate, tolerance, topo, vec | 10 deps |
| 🧱️primitives | arena, curve, error, euler, history, mat, surface, tolerance, topo, validate, vec | 11 deps |
| 🔺️euler | arena, curve, history, mat, surface, tolerance, topo, vec | 8 deps |
| 🕸️topology | arena, curve, history, mat, surface, tolerance, vec | 7 deps |
| 🖋️imprint | arena, curve, error, euler, history, mat, primitives, surface, topo, validate, vec | 11 deps |
| 🏟️arena | (isolated: defines ArenaId, EdgeId, FaceId, etc.) | 0 internal deps |
| ⚙️engine | (isolated: defines EVec3, error traits, handles) | 0 internal deps |
| 🖥️host | engine, kernel | 2 deps |
| 🌳️bvh | arena, curve, engine, error, euler, history, mat, surface, tolerance, topo, vec | 11 deps |
| 🔮️oracle | mat, primitives, topo, vec | 4 deps |
| ✂️curve-ops | bspline, curve, mat, vec | 4 deps |
| ➰️curve | bspline, mat, vec | 3 deps |
| 🪢️bspline | (self-contained, may call bezier internally) | ~2 deps |
| 🏄️surface | bspline, mat, vec | 3 deps |
| 🪡️surface-ops | mat, surface, vec | 3 deps |
| ➡️vector | (pure math, no brep deps) | 0 deps |
| 🔢️matrix | (pure math, no brep deps) | 0 deps |
| ⚖️predicates | (pure math, no brep deps) | 0 deps |
| 🎢️bezier | (pure math, no brep deps) | 0 deps |
| 〰️polynomial | (pure math, no brep deps) | 0 deps |
| 📏️tolerance | (pure util, no brep deps) | 0 deps |
| 🚨️error | (pure types, no brep deps) | 0 deps |
| 📜️history | (self-contained event log) | 0 brep deps |
| 🧮️compute | (async helper, no deps) | 0 deps |

**Hub Modules Identified (depended on by many clusters):**
- **🧰️kernel** (26 internal deps) — collision risk: **MUST be frozen or serialized per lane**
- **🏟️arena** (storage backbone) — depended on by ~20 modules — collision risk: **MUST be frozen**
- **🔢️matrix/⚖️predicates/➡️vector/〰️polynomial** (math layer) — depended on by 15+ modules each — collision risk: **MUST be frozen**
- **⚙️engine** (type definitions, traits) — depended on by ~8 modules — collision risk: **MUST be frozen**

---

## 3. Proposed Lane Clustering (6 Lanes)

**Goal:** Minimize cross-lane edges, balance LOC, group tightly coupled modules.

| Lane | Name | Files | Total LOC | Character |
|------|------|-------|----------|-----------|
| **Lane 1** | **Math Foundations** | ➡️vector, 🔢️matrix, ⚖️predicates, 〰️polynomial, 🎢️bezier, 📏️tolerance | 2148 | Pure math, **READ-ONLY boundary layer** — no brep state, 0 &mut methods |
| **Lane 2** | **Nurbs & Curves** | ➰️curve, 🪢️bspline, ✂️curve-ops, 🏄️surface, 🪡️surface-ops | 1725 | Curve/surface geometry; 0 &mut self (functions are pure) — can be parallelised with immutable reads to arena |
| **Lane 3** | **Topology & Storage** | 🏟️arena, 🕸️topology, 🔺️euler, 📜️history | 1146 | **MUST SERIALIZE:** arena owns all Vertex/Edge/Face/Solid handles; euler operations mutate arena; history logs all ops |
| **Lane 4** | **Boolean & Feature Ops** | 🔀️boolean, 🎨️blend, ↔️offset, ✅️validate | 2306 | High-level constructive ops; heavy arena mutation through boolean, blend, offset; validate is read-only |
| **Lane 5** | **Queries & Intersection** | 🌳️bvh, 🏷️classify, ✂️int-cc, ✂️int-cs, ✂️int-ss, 🔮️oracle, 📏️measure | 3421 | Spatial queries & intersection; mostly pure functions reading arena + curves/surfaces; ~0 &mut self |
| **Lane 6** | **Construction & Export** | 🧱️primitives, ➡️sweep, 🧵️sew, 🖋️imprint, 🩹️heal, 📦️mesh-io, 🧩️tessellate, 📄️step | 6061 | High-level constructors (primitive, sweep, loft) + import/export + tessellation; heavy arena mutation; ~0 &mut self in individual modules but each *invokes* arena ops |

**Out-of-lane singletons (must be frozen/serialized):**
- **⚙️engine** (362 LOC) — defines `Vec3=[f64; 3]`, error traits, GeometryHandle semantics — frozen
- **⚙️engine/🖥️host** (149 LOC) — owns `BrepEngineHost { cache, kernel }` — **CRITICAL: mutex-protected per-session state**
- **🧮️compute** (13 LOC) — async bridge (pollster::block_on) — frozen
- **🚨️error** (183 LOC) — error types — frozen
- **🧰️kernel** (1452 LOC) — **COLLISION RISK:** orchestrates *all* brep operations via &mut self; 191 mutable methods — **Option A: Freeze** (read-only after setup) **or Option B: Single-threaded lane + event sourcing**

**Cross-Lane Edge Count (sampled):**
- Lane 1 ↔ Lane 2: minimal (curves call predicates for numerical stability)
- Lane 2 ↔ Lane 3: medium (curve creation writes to arena via lane 6 constructors)
- Lane 3 ↔ Lane 4: high (boolean/blend/offset mutate arena topology)
- Lane 4 ↔ Lane 5: medium (offset requests BVH queries after topology changes)
- Lane 5 ↔ Lane 6: medium (tessellation reads BVH; export calls measure)
- Lane 6 ↔ Lane 3: very high (every constructor writes to arena; euler is the op record)

---

## 4. Imperative Surface (Mutable State & Mutation Points)

### 4a. Per-Module &mut self Method Count

| Module | &mut self Count | Type | Notes |
|--------|----------------|------|-------|
| 🧰️kernel | **191** | Brep impl | **MAXIMUM COLLISION RISK** — every public op mutates kernel.body, kernel.live, kernel.counter |
| ⚙️engine | **70** | Engine trait impl | async/sync wrapper methods; most delegate to kernel |
| 🥽️mesh | 39 | HalfedgeMesh impl | non-brep; low priority for lane strategy |
| 📄️step | 26 | Step import/export | mostly parsing, some arena write |
| 📜️history | 5 | OpRecorder impl | append-only log, lock-free candidate |
| 🏟️arena | 5 | Arena<T> impl | arena::insert, arena::get_mut — **single mutex guard per lane** |
| 🎬️scene | 3 | Scene3d impl | non-brep; low priority |
| All other brep modules | 0 | Functions | Pure functions over immutable arena refs + Handle<T> lookups |

### 4b. BrepEngineHost State

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` (149 LOC)

**Owned State:**
```rust
pub struct BrepEngineHost {
    cache: Mutex<EngineCache>,      // LRU output cache (bytes budget)
    kernel: Mutex<Brep>,             // single session kernel
}
```

**Methods:**
- `new(cache_budget_bytes: usize) -> Self` — init with empty Brep
- `kernel(&self) -> &Mutex<Brep>` — raw mutex accessor
- `with_kernel<R>(&self, f: impl FnOnce(&mut Brep) -> R) -> Result<R, ...>` — closure runner
- impl `EngineHost` — `derive(engine_id, input) -> EngineHandle` (delegates to cache, which runs BrepDocumentOpEngine)

**Collision Risk:**
- Each lane cannot have its own `Brep` instance without a merge/rebase step (state lives in arena, not versioned)
- **Solution:** Host must remain a singleton; lanes are *not* independent processes but **async tasks granted mutable refs in sequence** (actor model or serialised work queue)

### 4c. Register Functions & Setters

**Search result:** No `pub fn set_*` or `pub fn register_*` found in brep modules.  
Constructor/builder pattern: Implicit via `&mut self` method chaining (e.g., `kernel.box_prim() -> handle`, then `kernel.fillet(handle, …) -> handle`).

---

## 5. Benchmarks & Tests

### 5a. Benchmarks

**File:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs` (313 LOC)

**Benchmark Groups (criterion):**

1. **primitives** (2 cases)
   - `box_prim_sync` (1.0 × 1.0 × 1.0)
   - `sphere_prim_sync` (radius=1.0)

2. **curves_surfaces** (parameterized, 5 cases)
   - `interpolate_curve` (n ∈ {10, 100, 500})
   - `nurbs_surface_from_grid` ((rows, cols) ∈ {(4,4), (20,20), (50,50)})

3. **sweeps** (parameterized, 5 cases)
   - `sweep_straight` (line path)
   - `sweep_polyline` (segments ∈ {5, 20, 50})
   - `helical_sweep` (turns ∈ {1, 10, 50})

4. **booleans** (5 cases)
   - `fuse_box_box`, `cut_box_sphere`, `fuse_box_torus_mesh_fallback`, `repeated_cut_same_torus_x10`

5. **transforms** (parameterized, 3 cases)
   - `translate` (boxes ∈ {1, 20, 60}) — multi-box fusion for face-count scaling

6. **features** (parameterized, 6 cases)
   - `fillet_all_edges` (boxes ∈ {1, 5, 15})
   - `chamfer_all_edges` (boxes ∈ {1, 5, 15})

7. **intersect_measure** (parameterized, 5 cases)
   - `closest_point` (boxes ∈ {1, 20, 60})
   - `classify_point` (boxes ∈ {1, 20, 60})
   - `curve_curve_intersect` (control_points ∈ {10, 200})

8. **tessellation** (parameterized, 6 cases)
   - `box_tolerance` (tolerance ∈ {0.5, 0.1, 0.01}) × 2 (box, sphere)
   - `multi_box_faces` (boxes ∈ {1, 20, 60})

9. **patterns** (parameterized, 6 cases)
   - `linear_pattern` (count ∈ {5, 50, 200})
   - `circular_pattern` (count ∈ {5, 50, 200})

**Brep Operations Covered:** box, sphere, interpolate, nurbs, sweep, loft, fuse, cut, fillet, chamfer, offset, translate, tessellate, closest_point, classify_point, curve_curve_intersect.

### 5b. Unit Tests

**Test Counts by Module:**

| Module | Test Count | Type |
|--------|-----------|------|
| 🎬️scene | 77 | camera, mesh3d, picking, drawing |
| 🥽️mesh | 62 | HalfedgeMesh topology operations |
| 〰️polynomial | 18 | polynomial evaluation, derivatives |
| 🪢️bspline | 13 | knot insertion, basis functions |
| 🔢️matrix | 12 | matrix operations, inverse |
| 🔮️oracle | 11 | geometric queries |
| ⚖️predicates | 11 | numerical predicates |
| 🧱️primitives | 10 | box, sphere, cone, etc. creation |
| ➰️curve | 10 | curve creation, interpolation |
| ➡️vector | 10 | vector algebra, normalize, cross |
| ✂️curve-ops | 10 | curve operations |
| 🧰️kernel | 9 | kernel level (limited) |
| 📏️tolerance | 9 | tolerance comparison |
| 🏄️surface | 9 | surface operations |
| 🏟️arena | 8 | arena allocation, deallocation |
| 🎢️bezier | 8 | Bezier curve evaluation |
| 🧩️tessellate | 7 | triangulation, tessellation |
| 🕸️topology | 7 | halfedge topology |
| 📏️measure | 7 | distance, area, volume |
| 🏷️classify | 7 | point in solid |
| 🪡️surface-ops | 6 | surface operations |
| 🔺️euler | 6 | Euler operators (split, merge) |
| 🔀️boolean | 6 | fuse, cut, intersect |
| 📦️mesh-io | 6 | import/export |
| 📜️history | 6 | operation recording |
| 🩹️heal | 5 | heal operations |
| 🗺️spatial | 5 | spatial indexing (non-brep) |
| 🎨️blend | 5 | fillet, chamfer |
| ✅️validate | 5 | validation checks |
| ✂️int-cs | 5 | curve-surface intersection |
| ↔️offset | 5 | face/solid offset |
| 🚨️error | 4 | error types |
| 🖋️imprint | 4 | imprint, project |
| 🌳️bvh | 4 | BVH construction, query |
| ➡️sweep | 4 | sweep, loft, pipe |
| ✂️int-ss | 4 | surface-surface intersection |
| ✂️int-cc | 4 | curve-curve intersection |
| 🧵️sew | 3 | sew faces |
| 📄️step | 3 | STEP import/export |

**Test Distribution by Lane:**
- **Lane 1 (Math):** 62 tests (poly, matrix, bezier, vector, predicates)
- **Lane 2 (Nurbs):** 28 tests (bspline, curve, surface, surface-ops)
- **Lane 3 (Topology):** 21 tests (arena, euler, topology, history)
- **Lane 4 (Boolean/Feature):** 16 tests (boolean, blend, offset, validate)
- **Lane 5 (Query):** 27 tests (classify, oracle, measure, bvh, int-*)
- **Lane 6 (Construction/Export):** 32 tests (primitives, sweep, sew, imprint, heal, tessellate, step)

**Lanes Requiring Before/After Performance Check:**
- **Lane 6 (Construction):** sweep, tessellation, primitives — benchmarks available for all three
- **Lane 4 (Boolean):** fuse, cut, fillet, chamfer — benchmarks available
- **Lane 5 (Query/Intersect):** measure, classify, intersect — benchmarks available
- **Lane 3 (Topology):** arena ops are atomic; no separate benchmark (coverage via lanes 4/6)

---

## 6. Vec3 & ID Type Blast Radius

### 6a. Type Definitions

**Three separate Vec3 types exist in the codebase:**

1. **math::Vec3** (`🧮️math/➕️algebra`)
   - Definition: `pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }`
   - Uses: Scene camera, transformation matrices (Mat4)
   - Not re-exported from glue.rs as a **public type** — only internally used

2. **brep::vec::Vec3** (`📐️brep/➡️vector`)
   - Definition: `pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }`
   - Uses: Geometric computations, nurbs evaluation, intersection
   - Not re-exported from glue.rs

3. **brep::engine::Vec3** (`📐️brep/⚙️engine`)
   - Definition: `pub type Vec3 = [f64; 3]` (array alias)
   - Uses: Public BrepKernel API, GeometryHandle point parameters
   - **Directly re-exported** via engine module (trait definitions in public interface)

### 6b. Reference Count Breakdown

**Total references across framework:** ~1211  
**Breakdown by type:**

| Type | Count | Primary Users | Cross-Module Exposure |
|------|-------|----------------|----------------------|
| `Vec3` (engine alias `[f64; 3]`) | 974 | kernel, step, measure, classify, tessellate, primitives, sweep, etc. | HIGH — public API parameter in BrepKernel trait |
| `FaceId` | 161 | arena, topology, euler, sweep, boolean, measure, classify, tessellate, step | MEDIUM — internal handle type |
| `VertexId` | 110 | arena, euler, topology, step, primitives, validate | MEDIUM — internal handle type |
| `HalfEdgeId` | 1 | single isolated use (possibly dead code) | LOW — almost unused |

### 6c. Re-export Analysis

**glue.rs examination:**  
`/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust/📦️glue.rs` does **NOT** directly re-export `Vec3`, `VertexId`, `FaceId`, `ArenaId`, etc. from brep module.

**Public 3D API Surface:**
- Only **mesh module** (HalfedgeMesh) is re-exported, which defines its own `Vec3(pub [f32; 3])`
- **brep module is not mounted** in glue.rs (no `pub mod brep`)
- Scene module is re-exported; uses math::Vec3 (f32-based)

**Collision Risk: LOW for Vec3/ID types**
- Brep types are **not yet public API** — internal to the module
- If you change mesh::Vec3 or brep::engine::Vec3, only internal brep modules affected
- **No ripple into plugins or external crates** (currently)
- **BUT:** Once brep operations become async/event-sourced lanes, lanes exchanging geometry must agree on Vec3 representation — currently baked into BrepKernel trait signature `fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError>`

---

## Summary: Lane Split Feasibility

### ✅ Ready for Parallelisation

**Lane 1 (Math)** — Pure functions, 0 &mut self, no state mutations.  
**Lane 2 (Nurbs)** — Pure functions over immutable surfaces/curves, read-only arena access via handles.  
**Lane 5 (Query)** — Pure queries over BVH + arena reads, no mutations.  
**Lane 3 (Topology/Storage)** — Serialisable via OpRecorder; mutation order is critical; can be a single actor lane.

### ⚠️ Requires Synchronisation

**Lane 4 (Boolean/Features)** — Heavy arena mutation; depends on Euler operators (Lane 3).  
**Lane 6 (Construction)** — Every primitive/sweep/loft writes to arena; tight coupling to Lane 3.  

**Cross-Lane Dependencies:**
```
Lane 1 (Math) ← read-only by all
Lane 2 (Nurbs) ← read-only by Lane 5, Lane 6
Lane 3 (Topology) ← written by Lane 4, Lane 6; read by Lane 5
Lane 4 (Boolean) → Lane 3 (via Euler), Lane 5 (via BVH requests)
Lane 5 (Query) ← read from Lane 3, Lane 2
Lane 6 (Construction) → Lane 3 (via primitives/sweep), Lane 5 (tessellation needs BVH)
```

### 🔴 Collision Risk: Kernel & Host

- **🧰️kernel** (1452 LOC, 191 &mut self methods) — **Not parallelisable as-is**
  - Option A: Freeze after init (read-only after all lanes complete setup)
  - Option B: Replace with event-sourced command journal (ticket mentions DISSOLVE goal)

- **🖥️host** (149 LOC) — Single-threaded session; lanes must coordinate through it (work queue / actor model)

---

## Recommendations for Wave 3b: Implementation Plan

1. **Freeze Lane 1 & 2** — Compile to immutable libraries; no serialisation overhead.
2. **Serialise Lane 3 operations** — Use `OpRecorder` as the event log; replay to rebuild arena state per-lane-cycle.
3. **Queue Lane 4 & 6** — Batch mutating operations; submit to Lane 3 via work queue; await commit before continuing.
4. **Async Lane 5** — Snapshot arena at cycle boundary; run queries in parallel over snapshot.
5. **Kernel Refactor** — Break `Brep::new()` + 191 &mut methods into **initializer** + **immutable query interface** + **command journal** (aligns with DISSOLVE-KERNELS goal).
6. **Benchmark Lane 6** — Use existing `benches/kernel.rs` to validate before/after on sweep, tessellate, fillet.

