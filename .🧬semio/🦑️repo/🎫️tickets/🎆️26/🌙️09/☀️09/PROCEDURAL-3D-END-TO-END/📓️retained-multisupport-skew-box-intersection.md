# Retained multi-support Intersect for skew Boxes

\`brep.bool.intersect\` now certifies convex planar overlap when differently oriented Boxes share material across several active support planes. The Intersect geometry owner considers the twelve authored planes, enumerates their bounded triple intersections and certifies the finite vertices and outward face rings. Construction uses the existing native convex boundary storage, carries the original authored plane anchors and normals, and verifies one reciprocal use per edge before starting publication.

The certificate supports up to 20 vertices, 30 edges and 12 active support planes. The one-support ClippedBox route still retains exact Section and Classify. Multi-support results use the native closed planar Preview and Measure paths. Separations and supporting contacts return the typed empty result. Positive overlap below the representable finite topology tolerance returns UnsupportedOwner.

## Evidence

- The neutral skew fixture now includes two 20-vertex, 30-edge, 12-face multi-support shapes, including asymmetric overlap with a reflected stock. Manifold returns the same topology counts, area, volume and world bounds. It also verifies the positive but sub-tolerance overlap used in the refusal test. Combined skew, shared-axis and point-hull Manifold checks passed 18 tests and 168 assertions.
- The compact retained skew Rust witness passed 26 tests. It exercises both operand orders, verifies exact authored plane ownership, admissibility against all twelve supports, outward winding, reciprocal incidence, trim-to-edge agreement, area, volume and bounds. The adjacent shared-axis Rust regression passed 22 tests, including its former multi-support refusal fixture under the newly certified topology.
- Registered \`original-brep-booleans\` verifies the real multi-support operation after placement, both operand orders, cancellation after private native faces exist, exact world support ownership, native vertex, edge and face counts, preview, Measure and typed separation and precision refusals. Managed execution remains pending shared capacity.
- Canonical source closure passed Intersect at 3,006 lines and 145,003 bytes, Construction at 3,966 and 199,229, Solid Transaction at 4,000 and 207,109, and original-booleans at 2,065 and 249,394. No new package was needed. The public Point Hull input limit remains 16 points.

See [Retained single-support Intersect for skew Boxes](📓️retained-skew-box-intersection.md) for the initial clipped-box result.
