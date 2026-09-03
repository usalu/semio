# 🔍️ Explore — geometry math layer (vector, matrix, predicates, polynomial, curves, surfaces, tolerance)

Read-only audit (Haiku explorer, 2026-09-03). Base: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot`.

## 1. Core types

- `Pnt3`/`Vec3`/`Pnt2`/`Vec2` in `➡️vector/🦀️.rs` (Pnt3 309–350, Vec3 212–282, 2D 66–190): distance, lerp, dot, cross, normalized (None on zero, ≤ f64::EPSILON at 112), `any_orthogonal`, `angle_to`.
- `Mat3` (row-major, 14–66), `Quat` (72–155, slerp), **`Trsf { rotation: Quat, translation: Vec3, scale: f64 }`** (160–213) — rigid + uniform scale only, deliberately no shear/non-uniform scale so analytic surfaces stay analytic; `apply_point/vector/normal`, `compose`, `inverse`. `Frame3 { origin, x, y, z }` (218–267): `from_normal` (deterministic), `from_x_z` (Gram-Schmidt), `to_world/to_local`. All in `➡️vector/🔢️matrix/🦀️.rs`.
- `Tol` (`📏️tolerance/🦀️.rs` 40–98) with containment hierarchy vertex ≤ edge ≤ face; `Resolution::DEFAULT { linear: 1e-7, angular: 1e-9, param: 1e-9 }` (14–32); `Iv` certified interval (103–166).
- Errors (`🚨️error/🦀️.rs` 12–132): `KernelError::{InvalidInput, MissingEntity, Operation, Intersect, Boolean, Step}`, `IntersectError::{Tangent, Unresolved, Degenerate}`, `BooleanError::{ImprintFailed, ClassificationAmbiguous, InvalidResult, Intersect}`, `StepError::{Syntax, UnresolvedReference, Unsupported}`.

## 2. Curves (`➰️curve/🦀️.rs`)

`Curve3::{Line{origin,dir} (35), Circle{frame,radius} (37), Ellipse{frame,major,minor} (39), Nurbs{knots,controls,weights} (42)}`. Analytic eval/d1/d2 exact for line/circle/ellipse; **NURBS d1/d2 are central finite differences h=1e-4 (158)**. `Curve2` (216–272): Line/Circle/Ellipse/Nurbs; NURBS d1 FD h=1e-5 (259). `to_nurbs(domain)` 121–141 exact (circle/ellipse degree-2 rational ≤120° spans; not angle-linear inside spans, 123–127). No helix or polyline variant.

Curve ops (`✂️curve-ops/🦀️.rs`): `arc_length` adaptive GL5 + Richardson, depth 24 (31–50); `param_at_length` bisection 60 iters (55–78); `closest_point` uniform seed + Newton 30 iters, periodic wrap (89–147; step guard 1e-300 at 135, convergence 1e-13 at 140); `all_extrema` (154–174); `interpolate_centripetal` true Gaussian-elimination solve, degree ≤3 (184–228); `reverse_nurbs` (266–274); `split_nurbs` via knot insertion (280–313).

B-spline (`🪢️bspline/🦀️.rs`): `KnotVector` (12–93: validate, domain, find_span, clamped_uniform, multiplicity), `basis_functions` Cox–de Boor (103–122), `basis_function_derivatives` (127–187), `de_boor` per coordinate (196–201), `insert_knot` Boehm (211–226), `elevate_bezier_span` (232–244). Bezier (`🎢️bezier/🦀️.rs` 14–215): rational de Casteljau, subdivide, hull box, elevate (polynomial only), `subdivide_until_flat`, box overlap.

Polynomial (`〰️polynomial/🦀️.rs`): `Poly` Horner (12–57), `solve_quadratic` stable (66–83), `solve_cubic` (88–119, 1e-14 at 96), `Bernstein` de Casteljau/subdivide/sign_variations (129–209), `isolate_roots` (231–252), `refine_root` safeguarded Newton (261–290).

## 3. Surfaces (`🏄️surface/🦀️.rs`)

`Surface::{Plane{frame} (29), Cylinder{frame,radius} (30), Cone{frame,half_angle} (33), Sphere{frame,radius} (36), Torus{frame,major,minor} (39), Nurbs{u_knots,v_knots,controls,weights} (42)}`. Analytic first+second partials, normal, Gaussian/mean curvature for the five analytic kinds; **NURBS partials FD h=1e-4 (207–214)**; header 96–97 admits FD unsuitable for tight Newton. Periodic flags per direction; sphere v ∈ [-π/2, π/2]; cone v ≥ 0. No extrusion/revolution/offset surface variant.

Surface ops (`🪡️surface-ops/🦀️.rs`): `closest_point` (20–36): plane exact, sphere exact, others coarse grid + 2D Newton 30 iters, `wrap_or_clamp` per direction (convergence 1e-13 at 93); `coons_patch_eval` (119–128).

## 4. Predicates (`➡️vector/⚖️predicates/🦀️.rs`)

Filter-then-escalate to `semio_framework_number::Rational` (14, 42–57): `orient2d` 71–93, `orient3d` 99–135, `in_circle2d` 141–176, `sign_of_dot` 193–208, `collinear2d`/`coplanar3d` wrappers; `Orient::{Positive,Negative,Zero}` 19–35. Used by: (callers to be confirmed per module; classification/tessellation use own f64 tests).

## 5. Hardcoded constants

| value | where |
|---|---|
| 1e-7 / 1e-9 / 1e-9 | Resolution defaults, tolerance 25 |
| f64::EPSILON | vector 112, 250 |
| 1e-300 | bspline 115, 141; curve-ops 135 |
| 1e-13 | curve-ops 140; surface-ops 93 |
| 1e-4 | NURBS FD step curve 158, surface 207–213 |
| 1e-5 | Curve2 NURBS FD 259 |
| 1e-14 | cubic 96 |

`Tol` is a type but polynomial/bezier/bspline/curve-ops ignore it; callers pass raw f64.

## 6. Tests present

Vector/matrix (cross orthogonality, normalize, quat rotate/slerp, Trsf inverse/compose, frame handedness, det), tolerance/interval (500 random), predicates (QuickCheck 5000/3000/3000 near-degenerate), polynomial (quadratic/cubic/Bernstein/isolate/refine + 200 random with bisection oracle), bspline (partition of unity, FD derivative match, knot insertion no-op, 200 random vs Bernstein), curves (line/circle/ellipse derivatives, to_nurbs exactness incl. 100 random circles, arc-length, param_at_length, closest point, extrema, centripetal interpolation), surfaces (plane/cylinder/sphere/torus/cone derivatives + curvature, 50 random cylinders grid oracle).

## 7. Gaps vs exact kernel

1. NURBS exact derivatives (rational de Boor derivative recurrence) missing — FD only.
2. No certified inverse evaluation for surfaces (grid+Newton can miss/seam-trap); no Bézier-clipping projection for surfaces.
3. Periodic NURBS knot vectors not constructible; trims across seams unsupported.
4. Knot removal, degree reduction, periodic normalisation missing.
5. Interpolation with end tangents / closed curves missing; no error-bounded approximation (`approximate_curve` downsamples).
6. No offset curve/surface types; no swept/extruded/revolved surface types (sweeps therefore lose analytic form).
7. `Trsf` cannot express mirror (det −1) or non-uniform scale; tolerance scaling under `Trsf` not defined.
8. Helix/polyline curve variants absent (helix currently sampled to NURBS in engine).
