# Retained Convex Mesh Intersection Clipping

The clipping owner exposes `MeshConvexIntersection` for two outward closed convex polygon soups. Each advance performs one certification, classification, clipping, provenance or verification event under a four-million-event ceiling. Progress reports the current stage and event count. Cancellation stops without publishing partial rings. Terminal geometry refusals cannot resume.

The job admits connected closed operands with reciprocal edge incidence, supporting planes, planar convex polygon faces and solid interiors. It clips the left operand against the right operand's anchored planes. Coincident constraints preserve the existing face once. Disjoint operands and contact confined to faces, edges or points return an empty solid. The result verifies reciprocal incidence, positive signed volume and every live point against both operands' halfspaces.

The result carries each polygon's source as `Left(face)` or `RightPlane(face)` and records every original or interpolated vertex. Earlier points remain addressable for interpolation even when a later clipping plane removes them from the live geometry. A right plane is the exact supporting plane reference and can span several coplanar authored faces. Authoring material, UV and other channels across such faces needs a second-stage source-membership and remapping policy. This scoped stage has no Flow or Generation3d widget registration.

## Verification

- The neutral schema and ten case fixture cover partial overlap, both containment directions, coincident geometry, a 45-degree octagonal prism, translation by 1,048,576 units, and face, edge, point and separated empty results. Manifold independently reproduced the area and volume or empty outcome for all ten cases.
- Both compact Rust regression tests passed on the current Contract and Clipping code, including the existing single-plane clipping provenance test. The new regression checks exact left/right face provenance, interpolated point lineage, bounds, paired oriented edges, Euler characteristic, area, volume, refusal of open, reversed and concave sources, terminal retry behavior and cancellation.
- A workspace cache reset removed the warm full Mesh Engine artifact. The compact test used a generated facade forwarding the same two production Protocol index and membership types exported by Mesh Engine. No substitute implementations were used. The complete Mesh Engine and application packages were outside this scoped native check.
- The Mesh portable and packaging suites passed 21 tests and 3,944 assertions. The Clipping owner measured 37,073 bytes, 611 active lines and three files, within its existing source gates.
