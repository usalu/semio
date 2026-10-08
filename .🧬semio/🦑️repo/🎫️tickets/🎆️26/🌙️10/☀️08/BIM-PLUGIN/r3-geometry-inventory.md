# 📐️ Geometry Inventory for the BIM Inferences (r3, label `g-geometry`)

Scope: what the framework geometry kernels offered on 2026-10-08 before Wave F, what was missing for the BIM inference
catalogue (r2-design §5), and where each gap was closed. Paths are relative to the repo root; `M` = `🧰️framework/🔨️modules`.

## 1. Existing before this wave

### 1.1 `M/📐️geometry` — crate `semio-framework-geometry` (pure std, `kurbo 0.13.1` dev/oracle only)

Single engine file `⚙️engine/🦀️.rs` (+ `🎲️random`). f64 planar vocabulary, f32 render matrices.

| Area | Items (exact) |
|---|---|
| Vectors | `Point{x,y}` (`new`, `distance`, `ZERO`, `Point+Vec2`, `Point-Point->Vec2`), `Vec2{x,y}` (`new`, `hypot`, `dot`, `+ - * /`, `neg`), `Affine` (`IDENTITY`, `new([f64;6])`, `translate`, `scale`, `rotate`, `as_coeffs`, `Affine*Affine`, `Affine*Point`) |
| Shapes | `Rect`, `RoundedRect`, `Circle`, `Line`, `Arc` (elliptical, centre/radii/start/sweep/x_rotation, `eval`), `CubicBez`, `BezPath`, `PathEl`, `PathSeg{Line,Quad,Cubic}` (`eval`, `subdivide_at`, `subsegment`, `arclen`, `tight_bounds`) |
| Free fns | `segment_intersection(a0,a1,b0,b1)->Option<Point>` (bounded line/line), `circle_line_intersections(center,r,p0,p1)->Vec<Point>` (bounded), `convex_hull`, `polygon_area` (signed shoelace), `polygon_centroid`, `bounding_box`, `distance_point_to_polyline`, `normalize_or_zero`, `clamp_f64`, `geom_sel::{point_in_polygon, segment_intersects_polygon, WorldBox…}` |
| Render | `Vec3`/`Mat4` (f32 only), seeded `Rng` |
| Missing | bulge segments/arcs from start-end-bulge, arc offset, arc/arc and infinite-extension intersections, closest-point on arcs, bulged loops (area/centroid/containment/offset), triangulation, any f64 3D mesh |

### 1.2 `M/◻️2d` — crate `semio-framework-2d` (`serde` optional)

Types: `Vec2 = [f64;2]`, `PathSegment{Move,Line,Quad,Cubic,Arc,Close}`.

- `🔀️booleans`: `BooleanJob::new(BooleanInput{operation,operands,epsilon,max_edges,max_parameters,max_atomic_edges,max_segments,max_work})`, `advance(grant)->Result<BooleanProgress,_>`, `cancel()`, `into_result()->Vec<PathSegment>`; `BooleanOperation{Union,Difference,Intersection,Xor}`, `BooleanFillRule{Nonzero,Evenodd}`, `BooleanOperand{contours,fill_rule}`; sync wrappers `boolean_paths(a,b,op:&str)`, `boolean_paths_many`. Budgeted + cancellable (the progress/cancel contract of the repo).
- `🔀️booleans/🛤️paths`: `PathBooleanJob` (curved operands flattened first).
- `🛤️path/📏️flatten` (`PathFlattenJob`, device tolerance, arcs/Béziers), `🛤️path/🖊️stroke` (`StrokeOutlineJob`: caps, miter/round/bevel joins, dashes; output = overlapping polygons, not unioned), `🔍️trace`.
- Missing: any polygon offset/inset, a `Region` (outer + holes) abstraction, a sync-with-control boolean entry.

### 1.3 `M/🏗️mesh-engine` — crate `semio-framework-mesh-engine`

`MeshData` (f32 `positions`, `normals`, `indices`, attributes), `mesh_box/plane/uv_sphere/ico_sphere/cylinder/cone/torus`, `mesh_from_indexed(&[f32],&[f32],&[u32])`, `MeshData::{compute_normals, aabb, merge}`, OBJ/GLB/STL codecs. Display/IO type (f32, value/pack deps). No extrude, volume, area, section, triangulation.

### 1.4 `M/🧊️3d` — crate `semio-framework-3d` (`parry3d 0.17` dev/oracle only)

- `📐️brep`: arena `Body` B-Rep with recorder-based ops: `extrude_face`, `extrude_wire`, `pipe`, `sweep_along_path`, `boolean_solid`, `section_solid_by_plane`, `offset_face`, `tessellate_solid`, `solid_volume`, `mass_properties`; engine `Brep::{extrude_wire_sync, offset_face_sync, volume_sync, tessellate_sync, mass_properties_sync, …}`. Heavy (handles, arenas, NURBS-capable); right for exact CSG, wrong for per-element inference loops.
- `🥽️mesh`: f32 half-edge editing mesh (`extrude_faces`, `inset_faces`, `bevel_edges`, `triangulate`).
- `🧿️collision` (BVH), `🛞️inertia`, `🌀️rigid`.
- Missing for BIM: lightweight f64 solid builders and measures.

## 2. Gap analysis → decisions

| Needed primitive | Existing? | Decision |
|---|---|---|
| Bulge segment: centre, radius, sweep, length, point/tangent at t or length, closest point, tight bounds, offset, split, flatten | no | `📐️geometry/🌙️bulge` |
| Intersections line/line, line/arc, arc/arc, bounded and infinite extension; miter points | only bounded line/line, line/circle | `📐️geometry/🌙️bulge` (`intersect`, `nearest_intersection`, `corner_join`) |
| Wall band footprint with trims | no | `📐️geometry/🌙️bulge` (`band_loop`) |
| Bulged loop: area, signed area, centroid, perimeter, bounds, orientation, containment, flatten, offset, transform | polygon-only versions | `📐️geometry/➰️loops` |
| Triangulation with holes (no Steiner points, crack-free boundary) | none (brep tessellation is B-Rep only) | `📐️geometry/🔺️triangulation` |
| f64 triangle mesh + builders (extrude, prism, sweep, wall-with-openings), transform/merge/weld, volume/area/bounds/centroid/watertight, f32 export | none (MeshData is f32 display) | `📐️geometry/🕸️mesh` + `🧭️placement` (`Affine3`, `ZPlane`) |
| Plane section → oriented linework + chaining | `section_solid_by_plane` only on B-Rep | `📐️geometry/🔪️section` |
| Polygon offset/inset with joins, region booleans with budget/cancel | boolean jobs only | `◻️2d/🧱️regions` |

Why `📐️geometry` hosts the mesh builder: it is the dependency-free leaf (wasm32-wasip2 verified) that `🧊️3d`, plugins and
artifact crates already name; `◻️2d` depends on value/hash and could not be a dependency of the mesh builder.
`🏗️mesh-engine` stays the f32 display/IO type; the bridge is `TriMesh::{positions_f32, normals_f32, indices_flat}` →
`mesh_from_indexed` (no crate edge geometry → mesh-engine).

## 3. Result

New files (all additive, no existing public API touched):

```
M/📐️geometry/{➗️vector,🌙️bulge,➰️loops,🔺️triangulation,🧭️placement,🕸️mesh,🔪️section}/🦀️.rs   (+ 🧪️tests/🔬️unit/🦀️.rs each)
M/📐️geometry/🧫️fixtures/<domain>/🔣️.json   M/📐️geometry/<domain>/🧬️schema/🔣️.json
M/📐️geometry/🧪️tests/🏙️aec-oracles/{🦀️.rs,🟦️.ts,🐍️.py}   M/📐️geometry/🧪️tests/🎚️config/🟦️.ts
M/◻️2d/🧱️regions/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs,🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}
```

Edited: `M/📐️geometry/📦️packages/🦀️rust/{🦀️.rs (mounts), Cargo.toml (dev-deps serde_json, parry3d; second [[test]])}`,
`M/◻️2d/📦️packages/🦀️rust/🦀️.rs` (mount `regions`). API: `r3-geometry-api.md`. Execution: `r3-exec-g-geometry.md`.
