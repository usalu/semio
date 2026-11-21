# Original spatial elliptic contacts

The original `brep.intersect.curveCurve` operation now resolves ellipse pairs and mixed circle–ellipse pairs on intersecting planes. Their shared conic model carries the local frame, both radii and the optional edge trim. The solver weights the conic/plane equation by both radii, verifies the other curve in world space, and applies both authored ranges. Tangency uses the normalized derivatives of the two curves at the admitted parameters.

The registered output still publishes a vertex for one contact and a wire segment between two contact points. Parallel separated planes return a miss. Coplanar ellipse pairs and coplanar circle–ellipse pairs remain outside the fixed original analytic solver.

## Verification

- The actual native intersection crate passed 15 new ellipse and mixed cases, including an eccentric ellipse, an oblique chord, true tangency, near-tangent secants and both periodic trims. It also passed 46 prior curve-contact cases.
- The independent Three.js witness transforms the plane intersection line to the first ellipse’s unit frame and intersects it with a unit sphere. It verifies the candidates and tangent directions using the second conic and an implicit ellipse gradient.
- The existing registered `original-curve-contacts` target now includes the spatial conic fixture. It checks bounded turns, cancellation of partial publication and funded preview coordinates. The full Flow BRep test package was not compiled below the storage gate.
- Rust AST parsing and scoped whitespace checks passed. The source closure reader measured 2,685 lines in the intersection crate and 300 lines across 8 files and 52,048 bytes for the registered contact test target.

Portable cases are in `🔣️spatial-conic-intersections.json`. The ticket scripts are `🐍️line-conic-source-check.py` and `🧮️circle-pair-oracle.mjs`.
