---
name: Full Brep CAD Kernel
overview: Expand the solid-only brep kernel into a full CAD kernel by refactoring the handle/registry to multi-entity, exposing brepkit's curves, surfaces, sweeps, features, intersection, evaluation, analysis, IO and utilities through the BrepKernel interface, registering them all as procedural/flow nodes, and rendering every entity kind in the preview.
todos:
 - id: ticket
   content: Read repo://goals and open/reopen a ticket for the full CAD kernel build-out
   status: completed
 - id: engine-types
   content: "Expand engine interface: GeometryKind Curve/Surface, MeshTransfer.points, new param/result types, and the full BrepKernel trait surface in geometry/brep/engine/lib.rs"
   status: in_progress
 - id: registry
   content: Refactor geometry/brep/rs registry to multi-entity Entity enum with KernelCurve/KernelSurface and typed accessors
   status: pending
 - id: ops-curves-surfaces
   content: Implement curve and surface construction ops (line/circle/arc/ellipse/polyline/rectangle/polygon/interpolate/approximate/helix; plane/planarFace/nurbsSurface/coons/offsetFace/thicken)
   status: pending
 - id: ops-sweeps
   content: "Implement sweeps: extrude/revolve/loft(/smooth)/sweep(/options)/pipe/helicalSweep"
   status: pending
 - id: ops-bool-xform-features
   content: Implement boolean extras (compoundCut), transform extras (copy/linear/circular/grid pattern), and features (shell/draft/offsetSolid/defeature/chamferAsymmetric/filletVariable)
   status: pending
 - id: ops-intersect
   content: "Implement intersect/section/split: section, split, curveCurve, curveSurface, surfaceSurface"
   status: pending
 - id: ops-eval-analysis
   content: Implement evaluate ops (curve/surface point/tangent/normal/domain/curvature) and analysis/measure (area/length/com/bbox/distance/closestPoint/classify/validate)
   status: pending
 - id: ops-construct-io
   content: Implement construct/utility ops (vertex/faceFromWire/sew/heal/convertToNurbs) and IO export/import (step/stl/obj/gltf)
   status: pending
 - id: tessellation
   content: Rewrite tessellate_sync to branch per Entity kind (solid/compound/face/wire/edge/curve/surface/vertex) with points buffer
   status: pending
 - id: flow-catalog
   content: Register all operators in flow/modules/brep/lib.rs with correct groups, kind-aware geometry_dictionary, list helpers, and output schemas
   status: pending
 - id: js-preview
   content: Update geometry/brep/js MeshTransfer points mapping and extend procedural/react handle regex + surface/point preview
   status: pending
 - id: tests
   content: Extend existing Rust test modules in rs/lib.rs and flow/modules/brep/lib.rs to cover every category
   status: pending
 - id: build-verify
   content: Build flow/core + module-brep wasm, run cargo tests, add demo fixture, verify in procedural play
   status: pending
 - id: close
   content: Close the ticket with summary and touched files
   status: pending
isProject: false
---

# Full Brep CAD Kernel

Turn the solid-only kernel into a full CAD kernel across all three layers: the `BrepKernel` interface ([geometry/brep/engine/lib.rs](geometry/brep/engine/lib.rs)), the brepkit-backed impl ([geometry/brep/rs/lib.rs](geometry/brep/rs/lib.rs)), and the procedural node catalog ([flow/modules/brep/lib.rs](flow/modules/brep/lib.rs)), plus preview rendering for non-solid entities.

All brepkit access stays behind the `BrepKernel` trait (external lib behind an interface). Work happens inside a repo ticket; code is added into existing files using `#region`/`pub mod` structure. No new files except an optional demo fixture in `procedural/fixture/`.

## 1. Multi-entity handle + registry refactor

- In [geometry/brep/engine/lib.rs](geometry/brep/engine/lib.rs): extend `GeometryKind` with `Curve` and `Surface`; add a `points: Vec<f32>` field to `MeshTransfer` (JS already accepts optional `points`). Add small result/param structs as needed (e.g. `Domain { min, max }`).
- In [geometry/brep/rs/lib.rs](geometry/brep/rs/lib.rs): replace `Entry { kind, solid }` with an `Entity` enum carrying the real backing id/object:

```rust
enum Entity {
  Vertex(VertexId), Edge(EdgeId), Wire(WireId), Face(FaceId),
  Shell(ShellId), Solid(SolidId), Compound(CompoundId),
  Curve(KernelCurve), Surface(KernelSurface),
}
```

- `KernelCurve` enum wraps `Line3D | Circle3D | Ellipse3D | Parabola3D | Hyperbola3D | Nurbs(NurbsCurve)` with `evaluate/tangent/domain/curvature/length/to_nurbs` dispatch (brepkit gives `ParametricCurve` impls + `brepkit_geometry::convert::*_to_nurbs`).
- `KernelSurface` enum wraps `Plane | Cylinder | Cone | Sphere | Torus | Nurbs(NurbsSurface)` with `evaluate/normal/domain/to_nurbs`.
- Add typed accessors: `solid_id` (keep), plus `edge_id/wire_id/face_id/curve/surface/entity_kind`; register helpers `register_entity(kind, ...)`.

## 2. Kernel operations (exhaustive, behind the trait)

Add methods to the `BrepKernel` trait + sync impls, grouped by `#region`:

- Primitives: keep box/sphere/cylinder/cone/torus; add `convex_hull(points)`.
- Curves: `line`, `circle`, `arc`, `ellipse`, `polyline`(wire), `rectangle`, `regular_polygon`, `interpolate_curve`, `approximate_curve`, `helix_curve` (`builder::*`, `fitting::interpolate/approximate`, `helix::make_helix_curve`).
- Surfaces: `plane`, `planar_face`(wire/points), `nurbs_surface`(grid via `interpolate_surface`), `coons_patch`(`fill_coons_patch`), `offset_face`, `thicken`.
- Sweeps: `extrude`, `revolve`, `loft`/`loft_smooth`, `sweep`/`sweep_with_options`, `pipe`, `helical_sweep`.
- Booleans: keep fuse/cut/intersect; add `compound_cut(target, tools)`.
- Transforms: keep translate/rotate/scale/mirror; add `copy`, `linear_pattern`, `circular_pattern`, `grid_pattern`.
- Features: keep fillet/chamfer; add `chamfer_asymmetric`, `fillet_variable`, `shell`, `draft`, `offset_solid`, `defeature`.
- Intersect/Section: `section`, `split`, `curve_curve`, `curve_surface`, `surface_surface` (`nurbs::*intersect*`, `analytic_intersection::*`).
- Evaluate: `curve_point`, `curve_tangent`, `curve_domain`, `surface_point`, `surface_normal`.
- Analysis/Measure: keep volume; add `area`, `length`, `center_of_mass`, `bounding_box`(returns a box solid), `distance`, `closest_point`, `classify_point`, `curve_curvature`, `validate`.
- Construct/Utilities: `vertex`, `face_from_wire`, `sew`, `heal`, `convert_to_nurbs`.
- IO: `export_step/stl/obj/gltf` (return text/base64), `import_step/stl/obj` (return solids) via `brepkit_io`.

## 3. Multi-kind tessellation + preview

- Rewrite `tessellate_sync` to branch on `Entity`:
  - Solid/Compound -> `tessellate_solid` + `sample_solid_edges` (existing path; compound via `explode`).
  - Face -> `tessellate(face)` + boundary edge sampling.
  - Wire/Edge -> sample to polyline into `edges`.
  - Curve -> `sample_deflection`/`sample_uniform` into `edges`.
  - Surface -> `surface_grid` triangulated into positions/normals/index.
  - Vertex -> `points` buffer.
- JS: confirm `MeshTransfer.points` mapping in [geometry/brep/js/index.ts](geometry/brep/js/index.ts).
- Preview: extend the geometry-handle regex in [procedural/react/index.tsx](procedural/react/index.tsx) to include `curve-|surface-` and ensure surface/point items render.

## 4. Flow node catalog

In [flow/modules/brep/lib.rs](flow/modules/brep/lib.rs) register one operator per kernel op, grouped (`Primitives 3D`, `Curves`, `Surfaces`, `Sweeps`, `Booleans`, `Transforms`, `Features`, `Intersect`, `Evaluate`, `Measure`, `Utilities`, `IO`):

- Fix `geometry_dictionary` to emit the real `kind` (from `kernel.kind`).
- Add input helpers: `read_point_list`, `read_geometry_list` (using `ChannelSpec::list`), reuse `read_xyz`/`read_channel_number`/`read_geometry`.
- Output schemas: geometry handle (most), `number` (measure/curvature/classify), `point` (eval point/com/closest), `vector` (tangent/normal), `text` (validate/IO). One `out` port each; multi-value results split into separate operators.
- Call `registry.finalize()` after registration.

## 5. Build, test, verify

- Extend Rust tests in-place: kernel tests in [geometry/brep/rs/lib.rs](geometry/brep/rs/lib.rs) (each category: construct->evaluate->tessellate, measure values, intersect counts, IO round-trip) and operator/dispatch tests in [flow/modules/brep/lib.rs](flow/modules/brep/lib.rs).
- Build wasm: `nx run @semio-tech/flow-module-brep:wasm` and `nx run @semio-tech/flow-core:wasm` (browser uses `flow_core`).
- Run `cargo test -p geometry_brep_brepkit -p flow_module_brep`.
- Add a demo `procedural/fixture/*.procedural.json` exercising a curve -> surface -> sweep -> measure chain and verify in the procedural play harness.
- Register any new `launch.json` entries only if a new runnable target is introduced (existing brep targets already cover build/test).

## Decisions / risks

- Curves/surfaces stored as analytic-or-NURBS enums; converted to NURBS when an op (sweep path, edge/face build) needs it. Conics/quadrics convert exactly (rational NURBS).
- Single output port per operator (matches existing convention; avoids flow-editor multi-port changes).
- Bounding box returns a box solid (renderable) rather than raw min/max.
- Some brepkit ops can fail/への edge cases; all are wrapped and surfaced as `BrepError`/`EvalError` (no silent guards), consistent with the prior cut/fuse ticket.
- wasm size grows; acceptable for greenfield.
