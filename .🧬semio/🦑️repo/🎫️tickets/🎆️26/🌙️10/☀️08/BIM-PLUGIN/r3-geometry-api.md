# 📐️ Geometry API for the BIM Inferences (r3, label `g-geometry`)

Crates: `semio-framework-geometry` (`use semio_framework_geometry::{…}`) and `semio-framework-2d` (`regions`).
Units: metres and radians, f64. Right-handed: +X east, +Y north, +Z up; counter-clockwise (CCW) positive. No `unsafe`, no
panics on valid input, no global state, deterministic. Both crates check for `wasm32-wasip2`.

**Tolerance policy.** `vector::LENGTH_EPS = 1e-9` (1 nm) decides coincidence of lengths/distances/points;
`vector::ANGLE_EPS = 1e-12` angles; parameters `t` snap within `1e-9 / length`. Curve approximation (flattening) is always
caller-controlled via a chord `tolerance` (sagitta, metres); recommended BIM value `1e-3` (1 mm) for display, `1e-5` for
quantities. Mesh welding/watertight checks quantise positions at 1 nm.

**Cost / cancellation.** Everything in `semio-framework-geometry` is a pure synchronous function, O(n) to O(n²) in vertex
count of one element (ear clipping is O(n²)); call it per element inside the inference loop so cancellation is at
element granularity. The only genuinely expensive operations (region booleans/offsets over many elements) are in
`semio-framework-2d::regions` and take `control: &mut dyn FnMut(&BooleanProgress) -> bool` (called every 4096 work steps;
return `false` to cancel → `Err(BooleanError::Cancelled)`, no partial result; pass `&mut |_| true` to run to the end).

## 1. `vector` (module `semio_framework_geometry::vector`)

`type Xyz = [f64;3]`; `cross(Vec2,Vec2)->f64`, `perp(Vec2)->Vec2` (left normal), `unit(Vec2,eps)->Option<Vec2>`,
`lerp(Point,Point,t)->Point`, `angle_between(Vec2,Vec2)->f64` (signed, (-π, π]), `add3/sub3/scale3/dot3/cross3/length3/normalize3`.
Existing `Point`, `Vec2`, `Affine`, `Rect` are re-used from the crate root.

## 2. `bulge` — lines and circular arcs

`struct BulgeSeg { start: Point, end: Point, bulge: f64 }`, `bulge = tan(sweep/4)`, `>0` = CCW arc, `0` = line. This is the
snapshot `Axis::Line/Arc` and the loop-edge representation of r2-design §2. `t ∈ [0,1]` is proportional to arc length.

| Function | Semantics |
|---|---|
| `sweep_from_bulge(b)`, `bulge_from_sweep(θ)` | `4·atan(b)` and `tan(θ/4)` |
| `BulgeSeg::{line(a,b), new(a,b,bulge), from_arc(centre,r,start_angle,sweep), from_three_points(a,through,b)->Option}` | constructors; three collinear points → `None` |
| `.is_line() .chord() .sweep() .radius() .length()` | radius `INFINITY` for lines |
| `.center()->Option<Point>`, `.start_angle()->Option<f64>` | `None` for lines |
| `.point_at(t)`, `.point_at_length(s)`, `.tangent_at(t)`, `.tangent_at_length(s)` | unclamped; tangent is the unit direction of travel |
| `.param_of(p)->f64` | arc: angular fraction in the direction of travel (≤ 1 on the arc, may be negative within -1e-9); line: projection fraction |
| `.closest(p)->Closest{t,point,distance}` | exact (radial for arcs, endpoints when outside the sweep) |
| `.bounds()->Rect` | tight, includes arc extremes |
| `.offset(d)->Option<BulgeSeg>` | **left of travel positive**. Line: parallel shift. Arc: concentric, radius `r - d·sign(sweep)`; `None` when the radius would drop below 1 nm or the line has no length. Bulge unchanged. |
| `.reversed() .split_at(t) .subsegment(t0,t1)` | exact on the same carrier |
| `.retarget(start,end)` | same line / same circle and direction with new end points (re-measures the sweep in the direction of travel, `(0, 2π]`) — use after a miter trim |
| `.transformed(Affine)` | similarity only; a mirror flips the bulge sign |
| `.flatten(tol)->Vec<Point>`, `.flatten_into(tol,&mut Vec)` | start included / start excluded; sagitta ≤ `tol` |
| `.segment_area()`, `.segment_first_moment()` | signed chord-to-arc region `r²/2 (θ - sin θ)` and its moment |
| `intersect(a,b,Extent)->Vec<SegHit{point,ta,tb}>` | sorted by `ta`; `Extent::Bounded` or `Unbounded` (infinite line / full circle); parallel, coincident, concentric → none; tangent contact → one point |
| `nearest_intersection(a,b,extent,near)->Option<Point>` | the hit closest to `near` |
| `corner_join(prev,(left,right),next,(left,right))->Corner{left,right: Option<Point>}` | miter points of the two left faces and the two right faces of bands meeting at `prev.end`; per-side half widths of each band; `None` side = parallel/collinear (keep the square end) |
| `band_loop(axis,left,right,start_trim,end_trim)->Option<[(Point,f64);4]>` | CCW footprint `[right.start, right.end, left.end, left.start]` (point, bulge of the leaving edge); trims are `Option<(left_point,right_point)>` |

**Wall layout recipe (`🧱️wall-layout`).**
```rust
let axis = BulgeSeg::new(a, b, bulge);                          // authored Axis
let (l, r) = (location_offset_left, location_offset_right);      // from LocationLine and Σ layer thickness
// L/X corner of wall A end with wall B start:
let c = corner_join(&a_seg, (a_l, a_r), &b_seg, (b_l, b_r));
let a_loop = band_loop(&a_seg, a_l, a_r, a_start_trim, c.left.zip(c.right))?;
// T-join: branch ends on the through wall's face
let end_l = nearest_intersection(&branch.offset(bl)?, &through.offset(face_side)?, Extent::Unbounded, branch.end);
```
Verified: mitered L tiles the union exactly; T trim; tangent arc→line join keeps the annulus area (unit tests
`bulge::tests::{mitered_wall_network…, t_join…, curved_walls…}`).

## 3. `loops` — closed bulged loops

`struct Vertex { point: Point, bulge: f64 }` (bulge of the edge to the next vertex; last closes to first), `Containment{Inside,Boundary,Outside}`.

`from_polygon(&[Point])`, `segments(&[Vertex])->Vec<BulgeSeg>`, `signed_area`, `area`, `perimeter`, `centroid`,
`bounds->Option<Rect>`, `is_ccw`, `reversed`, `ccw` (normalised copy), `transformed(&[Vertex],Affine)`,
`winding(&[Vertex],Point)->i32`, `locate(&[Vertex],Point,eps)->Containment`, `contains(&[Vertex],Point)->bool` (strict, 1 nm),
`flatten(&[Vertex],tol)->Vec<Point>` (no repeated closing point), `offset(&[Vertex],distance,miter_limit)->Option<Vec<Vertex>>`
(**positive grows regardless of orientation**; exact arcs; bevel beyond `miter_limit·|d|`; `None` on collapse/inversion;
not cleaned of self-intersections → for hard cases use `regions::offset_regions` on `flatten`), `self_intersections(&[Vertex])->Vec<Point>`.
Area/centroid/containment are exact for arcs (circular-segment terms); a point exactly on a chord is resolved as if nudged +Y.

## 4. `triangulation`

`triangulate(outer:&[Point], holes:&[Vec<Point>]) -> Triangulation{vertices, triangles:[u32;3]}` — vertices = outer then every
hole in input order; any ring orientation; triangles CCW; no Steiner points; **collinear boundary vertices are kept**
(boundary equals the input rings, crack-free for developed curved faces); `Triangulation::area()`.

## 5. `placement`

`Affine3{m:[[f64;4];3]}`: `IDENTITY`, `translation(Xyz)`, `scaling(Xyz)`, `rotation_z(a)`, `rotation_axis(axis,a)`,
`from_frame(origin,x,y,z)`, `a.then(&b)` (a first), `apply_point/apply_vector/apply_normal`, `determinant()`.
`ZPlane{a,b,c}` (`z = a·x + b·y + c`): `flat(z)`, `sloped(anchor,z,direction,slope)`, `at(Point)`, `raised(dz)`.

## 6. `mesh` — `TriMesh{positions,normals:Vec<Xyz>, indices:Vec<[u32;3]>}` (f64, CCW outward)

Methods: `new`, `vertex_count`, `triangle_count`, `triangle(i)`, `push_triangle(a,b,c)` (skips zero area), `push_quad`, `append`,
`transformed(&Affine3)` (mirror-safe), `translated`, `signed_volume`, `volume` (abs), `surface_area`, `volume_centroid`,
`bounds->Option<(min,max)>`, `is_watertight` (directed edges matched at 1 nm), `welded()`, `unwelded()`,
`crease_normals(max_angle)`, export views `positions_f32 / normals_f32 / indices_flat` (feed `mesh_from_indexed` of `semio-framework-mesh-engine`).

Builders (free functions):

| Function | Use |
|---|---|
| `extrude(outer,holes,bottom:ZPlane,top:ZPlane)` | slabs (holes, slope via parallel planes), columns/beams with rectangle profile, any prism over a polygon with holes |
| `extrude_loops(outer:&[Vertex],holes:&[Vec<Vertex>],tol,bottom,top)` | same over bulged loops (arcs flattened within `tol`, smooth normals) |
| `prism_between(lower:&[Xyz], upper:&[Xyz])` | equal-length rings: roofs/hips/frusta, either orientation |
| `sweep_profile(outer,holes,path:&[BulgeSeg],base_z,tol)` | profile `(u left, v up)` swept along line/arc paths with mitered corners and end caps: beams, columns by path, railings rails, mullions |
| `extrude_between_faces(left:&ElevationFace,right:&ElevationFace,left_curve,right_curve,axis_length,base_z,tol)` | **walls with openings**: faces are outlines+holes in `(s, z)`; holes become reveals; door notches in the outline; different end `s` per side = slanted mitered ends; curved walls sliced crack-free; `left_curve = axis.offset(l)`, `right_curve = axis.offset(-r)` |

Wall solid recipe: build `ElevationFace` for the left face from the wall height outline with door notches and window
holes (positions are axis arc lengths `s`), the right face with identical ring structure (end vertices at the right
miter `s`), then call `extrude_between_faces`. Straight example (verified): 5 × 2.5 × 0.2 with door and window → volume
1.882, area 23.54. Quarter-circle wall r=5 with window: 4.51239 vs analytic 4.51239.

Affine placement: columns = `extrude`/`sweep_profile` then `.transformed(&Affine3::rotation_z(a).then(&Affine3::translation(p)))`.

## 7. `section`

`section_plane(&TriMesh, origin, normal, u_axis, eps)->Vec<[Point;2]>`: segments directed with the material on the left;
2D coordinates `(dot(p-origin,u), dot(p-origin,v))`, `v = normal × u` (**`normal` points to the viewer**: plan view from above
= `+Z`, front elevation from -Y = `-Y`). `section_z(&TriMesh, z, eps)` is the plan cut. Vertices exactly on the plane count as
above it (no degenerate output; coplanar faces vanish). `chain(&[[Point;2]], eps)->Vec<Polyline{points,closed}>` joins
segments end-to-start, removes collinear points; closed meshes give closed chains: outer CCW (poche), holes CW.
Plan-linework recipe: `section_z(solid, cut_height)` → `chain` → fill via `regions::region_boolean` union per storey.

## 8. `semio-framework-2d::regions`

`Region{outer (CCW), holes (CW)}` over `Vec2 = [f64;2]`: `Region::new(outer,holes)` (normalises), `.area()`;
`region_boolean(BooleanOperation, subject:&[Region], clip:&[Region], control)->Result<Vec<Region>,BooleanError>`;
`offset_regions(&[Region], distance, OffsetJoin::{Miter{limit}|Bevel|Round{tolerance}}, control)` (**positive grows, negative
shrinks**, holes behave correctly, collapse → empty result); `regions_from_path(&[PathSegment])->Vec<Region>`.
Use for: space boundaries (wall-face arrangement ∖ footprint union), slab/roof overhang offsets, plan poche union, clash areas.

## 9. Fixtures and oracles (reproduce these when adding behaviour)

`M/📐️geometry/🧫️fixtures/{🌙️bulge,➰️loops,🔺️triangulation,🕸️mesh,🔪️section,🧭️placement}/🔣️.json` and
`M/◻️2d/🧱️regions/🧫️fixtures/🔣️.json`: inputs plus closed-form expected values (generator
`T/r3-g-geometry-fixtures.py`, schemas `…/🧬️schema/🔣️.json`). Oracles: `kurbo` + `parry3d` (Rust,
`🧪️tests/🏙️aec-oracles/🦀️.rs`), `three` (`🟦️.ts`), `shapely` (`🐍️.py`).
