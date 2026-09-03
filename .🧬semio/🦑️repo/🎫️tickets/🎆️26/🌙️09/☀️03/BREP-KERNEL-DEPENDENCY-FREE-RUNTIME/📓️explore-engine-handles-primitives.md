# 🔍️ Explore — engine contract, handles, primitives, topology

Read-only audit (Haiku explorer, 2026-09-03; classification column corrected by the coordinator against `📓️explore-modeling-ops.md`). Base: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema`.

## 1. Handle model (`⚙️engine/🦀️.rs`)

- `mint()` 480–487: payload `"{GeometryKind:?}:{counter}:{entity_tag}"` hashed with `semio_framework_hash::hash_bytes` (483); counter `u64` wrapping; stored in `live: HashMap<String, Entity>`.
- `Entity` 372–381: `Vertex(VertexId) | Edge(EdgeId) | Wire(Wire) | Face(FaceId) | Solid(SolidId) | Curve(Curve3) | Surface(Surface)`. `GeometryKind` 71–81 also lists `Shell`, `Compound` — **no Entity variant for them**.
- `dispose_sync` 1374 removes from `live`; `retain_sync` 1759 keeps only given set. Neither reclaims arena entities.
- `import_step_sync` 1285, `import_stl_sync` 1292, `import_obj_sync` 1297, `import_glb_sync` 1280, `import_dwg_sync` 1307: **none clears `live`** — stale handles survive body replacement.
- Handles are session tokens, not persistent labels (`PersistentLabel` lives in topology, unrelated to `GeometryHandle`).

## 2. Primitives (`🔺️diff/🧱️primitives/🦀️.rs`)

- `solid_from_triangle_soup(body, triangles: &[[Pnt3;3]], rec) -> Result<SolidId>` 309–355; vertex quantisation `(v*1e6).round()` (314); one planar face per triangle, Newell normals, shared-edge dedupe.
- `make_box` 111 — 8 V / 12 line E / 6 planar F, analytic.
- `make_sphere` 148 — two hemisphere faces on an analytic sphere surface with a `segments`-gon equator (24 default, engine 586).
- `make_cylinder` 181 — circular caps + segmented side (32, engine 592).
- `make_cone` 209 — apex + segmented (32, engine 598).
- `make_torus` 237 — segmented ring (24, engine 604) → faceted.
- `make_convex_hull` 359 — quickhull → planar triangles.
- `make_polyline_wire`, `make_rectangle_wire`, `make_regular_polygon_wire`, `make_planar_face_from_points/wire` — analytic planar.

## 3. `_sync` method classification (engine line → path)

Exact analytic today: box 578, convex_hull 608, line/circle/arc/ellipse curves 615–640, polyline/rectangle/polygon wires 645–658, helix (as NURBS) 692, plane 706, planar faces 711/718, extrude 808 (prism; single-circle cylinder case), section 1056 (planar), evaluation 1091–1118, mass props 1124–1169 (quadrature), classify 1175, validate 1180, vertex 1195, face_from_wire 1201, sew 1205, deconstruct 1229, kind 1312, solid_face_loops 1325, tessellate 1365, export/import step 1253/1285.

Mesh-derived (tessellate → soup or hull): sphere/cylinder/cone/torus prims (faceted topology), **translate 894 / rotate 900 / scale 908 / mirror 914 / copy 925 all via `transform_solid_mesh` 441 (`tessellate_solid(…, 0.05)` at 445 → `solid_from_triangle_soup`)**, linear/circular/grid patterns 929–950 (transform + fuse loops), fuse/cut/intersect/compound_cut 859–883 (boolean mesh path), revolve 815 / loft 822 / sweep 832 / pipe 840 / helical_sweep 852 (lofted soup), fillet* 965–982 / chamfer* 996–1008 (soup strips), shell 1022 / offset_solid 1038 / draft 1030 / thicken 794 (hull), split 1063, export stl/obj/gltf/glb/dwg 1261–1302, import stl/obj/glb/dwg 1280–1307 (per-triangle faces).

Ignored args / stubs: `chamfer_asymmetric_sync` ignores `_d2` (1004); `draft_sync` ignores `_neutral_point` (1030); `loft` ignores `smooth`; `pipe` ignores `guide`; `interpolate_curve_sync` 664 and `approximate_curve_sync` 675 treat points as control points / downsample; `nurbs_surface_from_grid_sync` 726, `coons_patch_sync` 744 return surfaces built from grids (verify); `offset_face` errors on non-planar; `closest_point`/UV on curves/surfaces not exposed.

## 4. Topology (`📸️snapshot/🕸️topology/🦀️.rs`, `🏟️arena/🦀️.rs`)

- `Store<T, Id>` generational arena, `(index: u32, generation: u32)`, LIFO free list (arena 96). Ids: Vertex/Edge/Coedge/Loop/Face/Shell/Solid/Curve3/Curve2/Surface.
- `Vertex{position,tol,label}`, `Edge{curve,range,v0,v1,tol,label}`, `Coedge{edge,forward,pcurve: Option<Curve2Id>,prange,loop_id,next,prev}` (53), `Loop{first,face}`, `Face{surface,outer,inners,flipped,tol,label}`, `Shell{faces,label}`, `Solid{outer,inners,label}`.
- History 404–600: `PersistentLabel(u64)`, `LabelSource` (monotonic, `from_next`), `OpDelta{generated,modified,deleted}`, `OpRecorder`.
- `impl EngineRep<BrepArenaSeed> for Body` 288 (os-kernel trait, topology 22): seed/build round trip, `next_label` carried (300). Seed types `SeedVertex/Edge/Face/Shell/Solid`.

## 5. External imports from outside ✳️brep

- `semio_framework_3d::engine::{Vec3, Aabb, ParamDomain, MeshTransfer, FaceGroup, PointClassification}` — engine 63, bounding-volume ×3, tessellation ×2, classification ×2, mesh-io ×2, boolean ×1, offset ×1.
- `semio_framework_os_kernel::EngineRep` — topology 22.
- `semio_framework_hash::hash_bytes` — engine 483.
- `semio_framework_mesh_engine::{MeshData, mesh_from_obj/stl, mesh_to_obj/stl, GlbExporter/Importer}` — engine 468, mesh-io.
- `semio_framework_number::Rational` — predicates.

## 6. Rust consumers outside ✳️brep

stdio bench `benches/brep_kernel.rs` (all methods); stdio ✳️cad subset (`cad_brep_kernel()`, tessellate, witness geometry; editor interaction ×5 `Brep::new()`; 🚪️io geometry-import ×8); stdio ✳️process3d (`ProcessKernelReplay`, `Brep::new()`, volume/area, `SolidExporter/Importer`); `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` (thread-local `Brep::new()`, `with_kernel`/`with_kernel_read`); flow extension brep (60 methods); cad plugin rust `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/…/🧬️schema/💡️inferences/🦀️.rs` + `🚪️io/🗺️geometry-import/🦀️.rs` (uses `semio_framework_3d::engine` types). All use the concrete `Brep`, never `dyn BrepKernel`.
