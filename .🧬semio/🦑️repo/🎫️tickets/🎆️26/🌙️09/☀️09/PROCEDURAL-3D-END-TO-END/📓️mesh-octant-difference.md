# Authored Three-Plane Mesh Difference

The existing Flow `brep.mesh.difference` and Generation3d `mesh.edit.difference` operators support cutting a full corner from a certified outward closed convex source. Three independent cutter planes must each cross a solid section. The retained workflow constructs three disjoint exterior partitions and a clipped interior partition. It keeps the original exterior polygons and reverses the three cutter support patches on the removed part. Exact coordinate matching and subdivision at collinear seam vertices recover reciprocal edge incidence.

The three authored cap pairs must each share an edge and all three must meet at one vertex. Positive signed volume and reciprocal oriented edges are checked before publication. The three cap polygons derive corner UVs, material bindings and flipped authored normals from their own source faces. The retained job supports cancellation during production. Two-plane corners, opposing slabs and single-plane Difference continue to use the same public operation. Dependent three-plane cutters and unmatched authored charts yield a validation error. The later [through-tunnel report](./📓️mesh-tunnel-difference.md) describes the certified four-wall extension.

The neutral corpus includes centered, offset and opposite corners. The centered corner has 19 vertices, 32 edges and 15 polygons, with volume 7 and area 24. The offset corner has volume 7.4375 and area 24.

## Checks

- Both focused native Clipping tests passed. They check geometry, all three cutter-plane owners and outward reversed cap normals, the neutral shared junction on every cap, reciprocal edges and cancellation during active production. The dependent three-plane and four-plane tunnel fixtures return validation errors.
- The full compact authored Mesh witness passed, including distinct cap material bindings, UVs, reversed named normals, the shared position ID and PolygonMesh reimport. The same native checks preserve two-plane and slab cases.
- The Mesh jobs and packaging suite passed 24 tests and 4,193 assertions. Manifold independently reproduced area, signed volume and all cutter face normals on the centered, offset and opposite cuts.
- All four compact retained Flow producer/parser tests passed, covering Union and the three selected Difference cases. Registered workbench source passed Rust syntax parsing. The full linked workbench was outside the compact run. Source packaging and scoped whitespace checks passed.
