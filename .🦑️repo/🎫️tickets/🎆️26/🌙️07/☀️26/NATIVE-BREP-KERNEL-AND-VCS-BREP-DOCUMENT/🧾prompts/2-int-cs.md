# Lane 2-int-cs
Owned: ✏️s/🔨️modules/🧊️3d/📐️brep/✂️int-cs/🦀️component.rs

Implement curve/surface intersection.
API: CurveSurfaceHit{point,t,u,v}; intersect_curve_surface(curve, surface, tol) -> Result<Vec<CurveSurfaceHit>, IntersectError>
Analytic line/plane, line/sphere, line/cylinder first; NURBS via Newton from samples.
Depends on FROZEN int_cc types style. Tests: line piercing plane z=0; line through sphere.
Read wave2-common.md + wave1-common.md rules.
