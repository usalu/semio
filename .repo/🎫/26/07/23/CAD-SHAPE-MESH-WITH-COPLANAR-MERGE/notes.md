# CAD Shape Mesh With Coplanar Merge

## Request
Discard the lowpoly mesh; take the CAD shape; merge all coplanar faces. Fix wrongly fused column webs and unmerged plate-side triangulation.

## Pipeline
1. `solid_face_loops_sync` — CAD B-Rep face wires (shared vertices, no Steiner samples, no `fill_holes`)
2. `from_face_loops` — one n-gon per simple CAD face
3. `orient_faces_consistently` — directed-watertight
4. `merge_coplanar_faces` — join adjacent coplanar CAD faces (plate sides, etc.)

## Result
- **71 verts / 50 faces / 238 halfedges / 0 open edges / 0 spanning faces**
- Coplanar merge: 57 → 50 (7 merges)
- Plate sides become 16-gon faces (span ~8.1); top plate is one quad (10.8×4.68)
- Regenerated `concrete-forest-left.mesh.json` + `default.lowpoly.json`

## Verify
- `cargo test -p lowpoly_core --lib` — 15 passed
- Reload lowpoly playground after plugin rebuild to see the new embedded default
