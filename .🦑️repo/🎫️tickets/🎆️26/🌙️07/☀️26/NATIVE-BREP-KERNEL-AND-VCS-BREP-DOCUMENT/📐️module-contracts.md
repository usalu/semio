# Module Contracts

Status legend: `DRAFT` (stub only) → `FROZEN` (lane done; dependents may start) → `FLIPPED` (wired through native kernel).

| Module | Path | Status | Notes |
|---|---|---|---|
| error/vec/mat/tolerance/predicates/poly/bezier/bspline/curve/curve_ops/surface/surface_ops/arena/history/topo/euler/validate | existing | FROZEN | Phases 0–3 |
| `oracle` | `🔮️oracle/🦀️component.rs` | FROZEN | Wave 1: `Sdf` (+Torus/Cone), `ClosedFormMass`, watertightness stub API |
| `bvh` | `🌳bvh/🦀️component.rs` | FROZEN | `FaceBvh`/`EdgeBvh`, `build_face_bvh`/`build_edge_bvh`, `face_aabb`/`edge_aabb`, `query_ray`/`query_aabb`/`query_nearest` |
| `primitives` | `🧱primitives/🦀️component.rs` | FROZEN | Solids+wires+planar faces+convex hull; Result API; see 🧾lane-1-primitives-scope-note.txt |
| `measure` | `📏measure/🦀️component.rs` | FROZEN | `solid_volume/solid_surface_area/solid_center_of_mass/solid_bounding_box`, `face_area/edge_length`, `distance_solid_solid/closest_point_on_solid/classify_point_on_solid`, `AxisAlignedBox`, `PointSolidClassification` |
| `tessellate` | `🧩tessellate/🦀️component.rs` | FROZEN | tessellate_solid/face + sample_edge_polyline -> MeshTransfer; edge-first ear-clip |
| `int_cc` | `✂️int-cc/🦀️component.rs` | FROZEN | CurveCurveHit + intersect_curve_curve; analytic LL/LC/CC + Bezier clip general |
| `int_cs` | `✂️int-cs/🦀️component.rs` | FROZEN | CurveSurfaceHit + intersect_curve_surface; analytic LP/LS/LC + Newton general |
| `int_ss` | `✂️int-ss/🦀️component.rs` | FROZEN | `IntCurve{curve3}` + `intersect_surface_surface`; plane/plane + plane/cylinder analytic |
| `classify` | `🏷️classify/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `imprint` | `🖋️imprint/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `boolean` | `🔀boolean/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `sew` | `🧵sew/🦀️component.rs` | FROZEN | `sew_faces(body, faces, tol) -> SolidId`; spatial-hash vertex merge + canonical edge pairing |
| `heal` | `🩹heal/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `sweep` | `➡️sweep/🦀️component.rs` | FROZEN | Wave 2: `extrude_face` (plane/cylinder); revolve/loft/sweep/pipe/helical stub Operation; 🧾lane-2-sweep-scope-note.txt |
| `offset` | `↔️offset/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `blend` | `🎨️blend/🦀️component.rs` | DRAFT | Wave owns public API freeze before dependents start |
| `step` | `📄step/🦀️component.rs` | FROZEN | `write_step`/`read_step`; MANIFOLD_SOLID_BREP subset; see 🧾lane-2-step-scope-note.txt |
| `mesh_io` | `📦mesh-io/🦀️component.rs` | FROZEN | `TriangleMesh`, `StlFormat`, STL/OBJ/GLB/DWG codecs + `import_triangle_mesh_to_body` / `export_solid_*` via tessellate |
| engine (`BrepKernel`) | `⚙️engine/🦀️component.rs` | FROZEN | Trait frozen until Wave 6 |
| kernel (`BrepkitKernel`) | `🧰️kernel/🦀️component.rs` | LOCKED | Wave 6 only |
