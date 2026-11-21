# Retained single-support Intersect for skew Boxes

\`brep.bool.intersect\` now accepts a bounded class of certified rectangular Boxes with no shared axis. If one inward tool plane cuts the stock and the other tool supports contain the stock, the existing retained clipped-box recipe publishes the exact supporting cap and surviving stock faces. The planner tests both operand orders, so either stock can provide the clipped material. This works with translated, compound-rotated and reflected placements.

The predicate tests both sets of face normals and all nondegenerate edge-cross axes. Separation and supporting contact within the contact tolerance return the typed empty result. Positive overlap requiring several active tool supports returns UnsupportedOwner. The funded topology path publishes one complete clipped solid and supports the existing planar preview, Measure, Classify and Section routes.

## Evidence

- \`🔣️original-skew-box-intersection.json\` covers a low tool support making a hexagonal cap, a high tool support on reflected stock, an edge-cross-only separation, a supporting corner contact and positive multi-support overlap. Manifold independently checks volume, area, topology, cap polygon size, bounds and representative classifications, and confirms the difference between separation and unsupported overlap. The combined skew and shared-axis oracles passed 12 tests and 123 assertions.
- The compact production-source witness passed 24 Rust tests. It exercises the actual Boolean dispatch in both operand orders and verifies exact cap location and outward support, surviving stock plane ownership, trim coincidence, reciprocal incidence, volume, area, bounds, contact and classifications. It independently checks that the edge-cross fixture overlaps along all six face axes. The shared-axis compact regression passed 22 tests.
- Registered \`original-brep-booleans\` covers placed operands, delayed publication, cancellation after private native faces exist, reciprocal incidence, the live cap label in preview, exact retained Measure and Classify, and Section through the clipped result. Managed execution remains pending shared capacity.
- Canonical source closure passed Solid Transaction at 4,000 lines and 206,956 bytes, Construction at 3,957 and 198,678, and original-booleans at 2,038 and 245,564.

This wave established one active support plane. The subsequent [multi-support skew Intersect](📓️retained-multisupport-skew-box-intersection.md) adds convex overlap requiring several active planes.
