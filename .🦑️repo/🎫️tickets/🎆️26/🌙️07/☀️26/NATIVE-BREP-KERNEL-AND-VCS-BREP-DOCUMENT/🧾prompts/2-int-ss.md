# Lane 2-int-ss
Owned: ✏️s/🔨️modules/🧊️3d/📐️brep/✂️int-ss/🦀️component.rs

Surface/surface intersection emitting IntCurve{curve3, pcurve_a, pcurve_b}.
API: intersect_surface_surface(a,b,tol) -> Result<Vec<IntCurve>, IntersectError>
Quadric-pair analytics first; general marching with BVH seeds.
Tests: two orthogonal planes -> line; plane/cylinder.
