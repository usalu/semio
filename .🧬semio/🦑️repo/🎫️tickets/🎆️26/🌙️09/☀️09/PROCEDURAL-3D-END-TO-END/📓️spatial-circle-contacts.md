# Original spatial circle contacts

The original `brep.intersect.curveCurve` operation now resolves circles on distinct intersecting planes. It uses the conic/plane equation from the existing curve–surface query to find the first circle’s contacts with the other plane. Scaling by the plane angle keeps shallow crossings numerically resolvable. It verifies each candidate on the second circle in world units and applies both native edge trims, including across the angular seam.

The solver keeps distinguishable near-tangent contacts. A contact is tangent only when the actual tangents of both circles agree. Parallel separated circles still return no contact. The existing retained output publishes a vertex for one point and a wire segment between two points.

## Verification

- The native intersection crate compiled against cached geometry crates. It passed 15 spatial cases, 16 coplanar cases and 15 line/conic cases. The spatial corpus includes perpendicular, oblique and shallow crossings, true tangency, one-sided tangency, a near-tangent pair, rotated frames and both trims.
- Three.js independently constructed the plane intersection line and intersected it with a sphere in the circle plane. It checked the surviving points and the tangency classification. The two existing third-party oracles passed.
- The registered `original-curve-contacts` target covers original curve construction, edge imports, single-item admission, publication cancellation and funded preview coordinates across all three corpora. The full Flow BRep package was not compiled under the storage gate.
- Rust AST parsing and whitespace checks passed. The original query test target now has 29 source files and 239,177 bytes. The separate curve contact test target has 7 source files and 47,674 bytes.

The portable spatial cases are in `🔣️spatial-circle-intersections.json`. Ticket scripts `🐍️line-conic-source-check.py` and `🧮️circle-pair-oracle.mjs` run the focused checks.
