# Fix Lowpoly Mesh Wrong Filled Faces

## Problem
Default Concrete Forest Left lowpoly mesh had large filled faces spanning open space between vertical supports (visible in wire+shade view).

## Root cause
CAD→lowpoly regeneration used `fill_holes()` after triangle tessellation. Per-face BREP tessellation leaves T-junction seams; `fill_holes` capped those loops and also capped the large inter-support openings, inventing faces that are not CAD geometry. Coplanar merge then grew those caps into huge n-gons (e.g. 31-gon wrapping both columns).

## Fix
1. `BrepkitKernel::solid_face_loops_sync` — extract n-gon/hole loops from shared BREP face wires (no Steiner edge samples).
2. `HalfedgeMesh::from_face_loops` — simple faces → n-gons; holed faces → keyhole triangulation (openings stay empty).
3. `HalfedgeMesh::orient_faces_consistently` — repair inconsistent CAD wire windings so the mesh is directed-watertight.
4. Regenerated `concrete-forest-left.mesh.json` + `default.lowpoly.json`: **71 verts / 57 faces / 0 open edges / 0 spanning faces**.
5. Removed `fill_holes` from the export pipeline.

## Verify
- `cargo test -p kernel_3d_mesh` — 24 passed
- `cargo test -p lowpoly_core --lib` — 15 passed
- Regenerate: `bun .repo/🎫/26/07/23/FIX-LOWPOLY-MESH-WRONG-FILLED-FACES/export-concrete-forest-mesh.ts`
