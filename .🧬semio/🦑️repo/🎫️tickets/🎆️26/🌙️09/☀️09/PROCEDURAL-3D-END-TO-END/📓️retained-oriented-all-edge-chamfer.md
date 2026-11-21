# Retained all-edge Box Chamfer in an authored frame

The retained all-edge \`brep.solid.chamfer\` operator now accepts a rectangular Box after rotation, translation or reflection. It places the exact chamfer vertices and normal vectors in the authored frame. The original topology retains 24 vertices, 48 straight edges, six stock planes, twelve bevel planes and eight triangular corner caps. A mirrored frame reverses each boundary traversal while keeping its outward normal.

The operation stages one closed native solid through the existing funded topology construction and publishes it atomically. A zero or negative distance, a distance that removes an inset stock face, or geometry that cannot represent the displaced supports at tolerance is refused. Existing axis-aligned results use the same bounded topology.

## Evidence

- The neutral all-edge fixture now covers the world-aligned corpus and three transformed cases, including a reflected compound rotation and a near-limit bevel. Manifold independently intersects the stock with all edge and corner half spaces and checks exact area, volume, genus and world support bounds. The combined all-edge and selected Chamfer run passed eight tests and 82 assertions.
- The compact all-edge Rust proof passed 13 tests. It verifies all three transformed cases, the six, twelve and eight support-plane families, containment by every support, exact trim-to-edge projection, closed face loops, one reciprocal coedge per direction, oriented area and volume, bounds and distance refusals. The neighboring selected Chamfer and Fillet native checks passed all 26 tests.
- Registered \`original-brep-booleans\` exercises actual retained Rotate, Mirror, Translate and all-edge Chamfer, with invalid-distance rollback, cancellation after private face insertion, final native plane and edge counts, reciprocal incidence and preview measures. Managed execution remains pending shared capacity.
- Canonical source closure passed Native Registry at 3,969 lines and 216,964 bytes, Solid Transaction at 3,980 and 205,400, Flow BRep at 2,796 and 151,488, original-booleans at 1,757 and 205,564, and original-faces at 2,261 and 258,051.

The retained certificate covers uniformly beveling all twelve edges of a rectangular solid, with the original corner closure semantics.
