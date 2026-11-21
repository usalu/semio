# Authored Partial-Overlap Mesh Union

Flow `brep.mesh.union` and Generation3d `mesh.edit.union` accept certified outward closed convex pairs with a positive overlapping solid, up to three crossing support planes on each operand, and at least two on one operand. Partitions retain only exposed source polygons. If both operands share an outward support plane, the primary source contributes the overlap patch once and both keep their exposed regions outside it.

The retained job shares the anchored convex clipper and exact ring subdivision used by Difference. It coalesces exact positions from both operands, records right-source point and interpolation lineage, and subdivides edges at live shared positions. Publication verifies each original supporting plane, reciprocal oriented edges, a connected live boundary, positive volume and Euler characteristic two. Retained source face provenance drives material, UV, normal and other authored surface projection.

Pairs with one crossing support each still use the existing complete-face certificate. The same exact-weld path covers matching full-face contact. Strict separation and containment keep their original paths. Partial contact without volume, more than three crossing supports on either operand, unresolved numeric seams and unsupported authored projections yield validation errors. The partition producer honors the fixed work ceiling and supports cancellation.

The neutral corpus includes shifted equal-height boxes with common outward top and bottom supports, the previous wider-section refusal, overlap across three axes without common support planes, and the typed unit-box fixture whose old stitching face matched only in part.

## Checks

- The focused native Clipping run passed all three tests. It checks the old Union and Difference cases, all four new overlap cases, coalesced cross-source IDs, interpolation lineage, reciprocal edges, Euler characteristic two, signed volume, area and late cancellation. The shifted and former unit-box fixtures have 24 vertices, 42 edges and 20 polygons. The unit result has volume 1.625, area 8.75 and bounds `[[0,0,0],[1.5,1.25,1]]`.
- The authored native Mesh witness passed after refreshing the actual component-reference and Mesh Engine owners against the current cache. It checks both material/UV domains on the shifted result, primary-only coincident top and bottom patches, native tessellation, and PolygonMesh reimport.
- The portable Mesh jobs and packaging suite passed 24 tests and 4,672 assertions. Manifold independently confirmed area, volume, one genus-zero component and exposed support-plane membership across the partial-overlap cases. The registered Flow workbench passed Rust syntax parsing and the scoped whitespace check passed.
- All eight compact retained Flow producer/parser tests passed with the refreshed Mesh Engine, including partial Union and its exact-weld and Difference regressions. The full linked registered workbench was outside the compact run.
