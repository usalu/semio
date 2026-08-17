# Lane 1-primitives

Owned file: ✏️s/🔨️modules/🧊️3d/📐️brep/🧱primitives/🦀️component.rs

Implement analytic solid constructors that mutate a topo::Body via euler ops + curve/surface stores.

Public API:
- make_box(body, w,d,h) -> SolidId
- make_sphere(body, radius, segments) -> SolidId  
- make_cylinder(body, radius, height, segments) -> SolidId
- make_cone(body, radius, height, segments) -> SolidId
- make_torus(body, major, minor, segments) -> SolidId
- make_convex_hull(body, points: &[Pnt3]) -> Result<SolidId, KernelError>
- make_polyline_wire / make_rectangle_wire / make_regular_polygon_wire / make_planar_face_from_points / make_planar_face_from_wire

Use Frame3, Surface::{Plane,Cylinder,...}, Curve3, OpRecorder. Closed solids must validate_body clean (or document deferred checks). Tests: box V-E+F, sphere volume later via measure stub ok if volume not ready — at least topology counts + validate ring integrity.

Reference brepkit operations/primitives if needed for topology layout. Read wave1-common.md.
