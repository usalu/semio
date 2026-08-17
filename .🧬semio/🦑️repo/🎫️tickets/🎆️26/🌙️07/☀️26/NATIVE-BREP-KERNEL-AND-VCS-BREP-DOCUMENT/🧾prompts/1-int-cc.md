# Lane 1-int-cc

Owned file: ✏️s/🔨️modules/🧊️3d/📐️brep/✂️int-cc/🦀️component.rs

Curve/curve intersection.

Public API:
- pub struct CurveCurveHit { point: Pnt3, t_a: f64, t_b: f64 }
- intersect_curve_curve(a: &Curve3, b: &Curve3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError>
- Analytic cases for Line/Line, Line/Circle, Circle/Circle first; general NURBS via Bézier clipping using crate::brep::bezier

Tests: crossing lines at origin; circle/line diameter. Read wave1-common.md.
