# Lane 1-tessellate

Owned file: ✏️s/🔨️modules/🧊️3d/📐️brep/🧩tessellate/🦀️component.rs

Crack-free tessellation producing crate::brep::engine::MeshTransfer.

Public API:
- tessellate_solid(body, solid, deflection) -> Result<MeshTransfer, KernelError>
- tessellate_face(body, face, deflection) -> Result<MeshTransfer, KernelError>
- sample_edge_polyline(body, edge, deflection) -> Vec<f32> positions xyz packed

Edge-first shared discretization then per-face UV triangulation. Include face_groups. Tests: unit box → 6 face groups, nonempty positions/indices/normals; normals roughly unit.

Read wave1-common.md.
