# 🔍️ Explore — consumers: flow extension, viewer, editor, bridge

Read-only audit (Haiku explorer, 2026-09-03).

## 1. Flow extension (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`, 1866 lines)

60 distinct `BrepKernel` methods (doc 23–25: num/point/vec/text ops via read lock, geo ops via write lock through `with_kernel`/`with_kernel_read` from `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`):
- primitives: box, sphere, cylinder, cone, torus, convex_hull (93–100)
- curves: line, circle, arc, ellipse, helix, rectangle_wire, regular_polygon_wire
- surfaces/sweeps: plane_surface, sweep, revolve, extrude, helical_sweep, face_from_wire, planar_face_from_wire, thicken_face, convert_to_nurbs
- booleans: fuse, cut, intersect
- modifiers: fillet, fillet_variable, chamfer, chamfer_asymmetric, offset_solid, offset_face, heal_solid, mirror, rotate, scale, translate
- patterns: linear, circular, grid
- measures (read): volume, area, length, center_of_mass, distance
- analysis (read): curve_point, curve_tangent, curve_curvature, surface_point, surface_normal
- utility: vertex, bounding_box, validate, copy_shape
- io: export/import dwg, step, obj, stl
Memoisation: static mesh cache (`mesh_cache()`), `retain_geometry_handles` GC 1723–1744, `tessellate_geometry_is_memoized` 1747–1762, `reset_test_kernel` 1507–1511. Handles content-addressed via blake3-style hash from `BrepKernel::mint`.

## 2. Viewer (`✳️brep/👁️viewer/…`)

- `SemioBrepViewCommand { #[default] Noop }` viewer 24–28; `handle` returns `ViewEmit::default()` (64).
- main window 15: `SEMIO_BREP_VIEW_FALLBACK_MESH_KIND = "box"`; 40–56 `world_instances_json` maps snapshot array lengths to instances; 62 `mesh_from_kind("box")`. No snapshot→Body→tessellate path exists.
- Sibling mesh subset viewer and lowpoly `🌐️model` window use the same fallback (`mesh_from_kind`, lowpoly 25/77) — **no viewer in the repo renders real artifact geometry**; the reusable pattern is `MeshWindowKit::render(&MeshView{ meshes_json, instances_json, camera_json, selection_json })` with `BuiltNode` from `semio_framework_plugin`.

## 3. Editor (`✳️brep/✏️editor/🦀️.rs`, 118 lines)

Comment 21–27 explains `set-vertex` is a no-op because the schema "declares no by-index replace/set op" — but `SemioBrepMutation::MoveVertex` exists (mutations 66, text form 142 `move-vertex id=… point=…`). `handle` 76–78 returns `Emit::default()`. Fix: emit `MoveVertex`.

## 4. Bridge (`✳️brep/🏭️bridge/`)

Mutation-inventory binary answering `listMutations` from the dispatch descriptor; deps are all first-party (os-kernel, schema, plugin, 3d, number, pack). Zero third-party.

## 5. Consumer matrix

| crate | usage |
|---|---|
| flow extension brep | 60 methods via `with_kernel` |
| os flow `📐️brep-geometry` | thread-local `Brep::new()`, handle registry |
| stdio ✳️cad inferences/editor/io | `cad_brep_kernel()`, `tessellate`, 5+8 `Brep::new()` sites |
| stdio ✳️process3d | `ProcessKernelReplay`, `Brep::new()`, volume/area, `SolidExporter/Importer` |
| cad plugin rust | `semio_framework_3d::engine` types in inferences + geometry-import |
| lowpoly / procedural | no direct kernel use |

19 `Brep::new()` sites total; every consumer holds its own kernel.

## 6. Gaps (audit §8–§9)

Viewer placeholder; editor no-op; no tessellation inference in the artifact; flow nodes inherit approximations with no quality metadata.
