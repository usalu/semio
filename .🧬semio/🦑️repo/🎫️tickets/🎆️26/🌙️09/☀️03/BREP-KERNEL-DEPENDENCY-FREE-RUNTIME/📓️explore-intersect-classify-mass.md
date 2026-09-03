# 🔍️ Explore — intersection, classification, BVH, mass properties, validation

Read-only audit (Haiku explorer, 2026-09-03). Base path: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema`.

## 1. Intersection (`🔺️diff/✂️intersect/🦀️.rs`, 1239 lines)

Public API: `intersect_curve_curve(a, b, tol) -> Vec<CurveCurveHit>` (39), `intersect_curve_surface(curve, surface, tol) -> Vec<CurveSurfaceHit>` (529), `intersect_surface_surface(a, b, tol) -> Vec<IntCurve>` (910).

Pair cases:
- Line/Line 57–85 analytic; parallel/coincident checks 70–74.
- Line/Circle 88–113 analytic (3D/planar hybrid).
- Circle/Circle 156–197 coplanar analytic; non-coplanar falls to generic (161).
- Plane/Plane 1087–1104 → line.
- Plane/Cylinder 1108–1149 → circle/ellipse/two lines; parallel sub-case 1133–1149.
- Plane/Sphere 926–947 → circle or empty.
- Sphere/Sphere 951–971 → circle.
- Generic curve fallback 204–220: Bézier clipping + Newton; 24-sample domain seeding (365). Newton 390–421: 12 iterations (391), acceptance at 10× tol (416).
- Generic surface/surface 975–1022: dense 24×24 UV grid (976–977), project to `b`, keep samples within 4× tol (985), emit degree-1 NURBS through hits (1019).

Representation: `IntCurve` carries `curve3` only (905) — **no p-curves on either support**.

## 2. Classification (`💡️inferences/🏷classification/🦀️.rs`, 570 lines)

`point_in_solid(body, solid, point, tol) -> PointClassification` (74). Multi-ray consensus over 6 irrational directions `RAY_RETRY_DIRS` (95–102); one crossing per face (306, first hit with `t > RAY_T_MIN`); parity voting 281–286. **BVH parameter `_bvh` never dereferenced (270).** Non-planar faces: surface eval + curve/surface intersection 312–318; trim test via `point_in_face_uv` loop winding 346–357 (reprojection, not stored p-curves). No vertex/grazing rule beyond `RAY_T_MIN = 1e-12` (93, 334). Trim dedupe tolerance ×10 (798).

## 3. BVH (`💡️inferences/🌳bounding-volume/🦀️.rs`, 564 lines)

Build 113–136: recursive median split on longest axis (129–131); binary nodes (92, 136); `Node::Leaf { aabb, item }` (91). Queries: `query_ray` 177–181, `query_aabb_overlap` 200–204, `query_point_nearest` 143–147 (AABB distance only). Callers: classification imports it (13, 85) but does not traverse; tests 521+.

## 4. Mass properties (`💡️inferences/📏mass-properties/🦀️.rs`, 1356 lines)

`solid_volume` (45, divergence `V=(1/3)∫P·n dA`, analytic sphere fast path 46–47), `solid_surface_area` (62), `solid_center_of_mass` (76, tetrahedral moments 95). Quadrature: Gauss–Legendre 5-node (264–265); `gauss_samples(tol)=ceil(sqrt(1/tol))` clamped [4,32] (268–269); `integrate_parametric_face` 299–314. Planar faces: Newell 491–505, tetra sum 433–452. Non-planar: parametric moments 273–295, flipped orientation 286. `distance_solid_solid` 163–190: bbox gap then face-sample brute force; **overlapping solids return 0** (187–189). **Second, independent classifier `classify_point_on_solid` 218–237** (ray parity on 3 cardinal axes). Thresholds: 1e-9/1e-12 (154, 251, 706); 24 samples for closest point (721). Oracle in tests: `Sdf` enum + `ClosedFormMass` + watertightness probe (1018–1354).

## 5. Validation report (`💡️inferences/✅validation-report/🦀️.rs`, 446 lines)

Checks: loop ring closure + first coedge 122–146; edge valence > 2 → non-manifold 153–159; tolerance containment 169–188; same-parameter with **only 5 samples** (198) and **skips coedges without p-curves** (203). Not checked: self-intersection, manifold orientation, shell closure, slivers. Header 9–23 explains tessellation/mass-properties are deliberately not inferences here.

## 6. Inferences root (`💡️inferences/🦀️.rs`, 101 lines)

Only `validation_report` exposed (22–28). Lines 9–11 state tessellation/mass-properties are omitted because real curve/surface evaluation "belongs in framework-3d" — the dependency-direction contradiction the audit names. Reads spec (44): `["vertices","edges","loops","faces","shells","solids"]`.

## 7. Tests

Intersection: line-circle, circle-circle, line-plane, line-sphere, plane-plane, plane-cylinder, plane-sphere (437–1228). Classification: box in/out/boundary, sphere SDF oracle, loop UV center (453–566). BVH: tetra ray/aabb, empty solid (519–562). Mass: unit box, sphere, box COM, box distance, edge length, SDF oracle box/sphere/cylinder/torus (938–1338). Validation: clean tetra, broken ring, tolerance violation, non-manifold, same-parameter (365–443).

## 8. Gaps vs exact kernel

1. Two classifiers; BVH built but bypassed.
2. SSI emits 3D curve only — no paired p-curves.
3. Exact SSI only for plane/sphere/cylinder pairs listed; cylinder/cylinder, cone, torus, NURBS pairs use 24×24 sampling.
4. Mass props: no error certificate; overlapping-distance miss.
5. Validation misses self-intersection/orientation/closure/slivers; missing p-curves silently skipped.
6. Tessellation/mass inferences blocked by dependency direction.
