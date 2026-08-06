# Lane 2-step
Owned: ✏️s/🔨️modules/🧊️3d/📐️brep/📄step/🦀️component.rs

Hand-rolled ISO 10303-21 reader/writer subset (MANIFOLD_SOLID_BREP, ADVANCED_FACE, analytic + B_SPLINE).
write_step(body, solids) -> String; read_step(text) -> Body.
Tests: box round-trip topology counts.
