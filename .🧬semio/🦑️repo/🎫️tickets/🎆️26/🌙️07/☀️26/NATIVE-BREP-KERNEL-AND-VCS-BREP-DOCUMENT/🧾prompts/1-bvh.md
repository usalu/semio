# Lane 1-bvh

Owned file: ✏️s/🔨️modules/🧊️3d/📐️brep/🌳bvh/🦀️component.rs

Implement a B-Rep-oriented BVH helper module that wraps/adapts crate::spatial::Bvh for Body faces/edges.

Public API (freeze these names):
- pub struct FaceBvh / EdgeBvh or generic BvhIndex
- build_face_bvh(body: &Body, solid: SolidId) -> Result<..., KernelError>
- query_ray / query_aabb / nearest helpers returning entity ids
- Unit tests: build over tetrahedron from euler tests; ray hit; empty solid

Reuse crate::spatial and crate::brep::engine::{Aabb,Vec3} / topo ids. Do not duplicate spatial algorithms — thin adapter + AABB from face bounds via surface/curve eval.

Read wave1-common.md.
