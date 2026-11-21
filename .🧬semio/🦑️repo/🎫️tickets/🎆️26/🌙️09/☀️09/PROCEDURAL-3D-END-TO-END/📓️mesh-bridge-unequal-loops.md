# Bridge Unequal Mesh Boundary Loops

The Bridge modeling job accepts two disjoint simple boundary loops with at least three edges each. The input remains an ordered pair of directed boundary halfedges. Starting at the selected alignment, the job compares both loops' normalized edge progress using integer arithmetic. A single boundary advance emits a triangle. A shared step emits a quad. Equal-loop wall order and UVs remain identical to the original Bridge fixture.

Wall faces preserve the first loop's face provenance and keep exact edge and corner samples on the old rims. Both directions of a generated cross edge inherit the same adjacent first-rim sample. Triangles use the first three corners of the existing square wall UV chart. Bridge retains individual work phases and checks the exact planned face and corner totals against the existing Mesh limits.

The neutral triangle-to-square fixture fixes the ordered walls as `[[0,3,6],[2,0,6],[2,6,5],[1,2,5],[1,5,4],[0,1,4,3]]`. Its closed shell has signed volume `5/6` and surface area `(9 + √3)/2`. The fixture records every generated edge’s source index.

## Verification

- The current Mesh job and packaging Bun suites passed 17 tests and 3,826 assertions. The independent Three.js oracle verified both fixture shells’ geometry and oriented incidence. AJV verified both anchor pairs.
- The compact native proof compiled Mesh Contract, Document, Clipping, Reconstruction and Modeling, then passed the current regression on both loops. Coverage includes exact polygon order, closing twins, tessellated area and volume, generated active UVs, original and generated generic edge data, and paired seam retention.
- Flow registration, all four committed JSON carriers and the packed descriptor use the expanded Bridge summary. Only the packed descriptor’s embedded manifest changed. The suffix bytes remained identical.
- The Flow workbench regression now requests the triangle-to-square Bridge through the registry and asserts the exact wall list. Its Rust source parsed successfully. Executable Flow integration remains unverified.
