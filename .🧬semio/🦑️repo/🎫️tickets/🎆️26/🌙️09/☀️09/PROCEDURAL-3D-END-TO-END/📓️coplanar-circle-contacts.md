# Original coplanar circle contacts

The original `brep.intersect.curveCurve` operation now resolves coplanar circle pairs over both native edge ranges. It computes the radical axis and evaluates the surviving candidates on both original circles. World-space residual checks account for the requested tolerance. Contacts are sorted by the first circle’s admitted parameter.

Coincident circles use periodic interval intersections. Disjoint intervals return no contact. Intervals that meet only at endpoints return isolated tangent contacts. Overlapping intervals refuse as infinitely many intersections. The interval comparison respects reversed normals and trims that cross the angular seam. Parallel planes that are separated beyond tolerance return no contact. Nonparallel circle supports remain outside the fixed original family.

The existing publication lane emits a vertex for one contact and a wire segment between two contact points. The untrimmed analytic path uses the same solver, while other pairs use the general curve intersection path.

## Verification

- The actual native intersection crate compiled against cached source dependencies and passed all 17 portable circle cases and the 15 existing line/conic cases. The check verifies the world point on both parameterized curves, both trim ranges, tangency, fault classification and untrimmed entry-point agreement.
- Three.js independently intersected the radical axis with a sphere in the circle plane. For coincident supports it checked the angular segments between trim endpoints and evaluated boundary points. Both third-party oracles passed.
- The registered original query law reuses the native publication and cancellation checks for the circle fixture. The full Flow BRep test crate was not built within the disk limit.
- Rust AST parsing and scoped whitespace checks passed. The source closure measured 2,644 lines across 11 files for intersection math and 2,090 lines across 32 files and 255,313 bytes for the original query tests. The test target is at its 32-file ceiling.

The portable cases live in `🔣️circle-pair-intersections.json`. The ticket scripts are `🐍️line-conic-source-check.py` and `🧮️circle-pair-oracle.mjs`.
