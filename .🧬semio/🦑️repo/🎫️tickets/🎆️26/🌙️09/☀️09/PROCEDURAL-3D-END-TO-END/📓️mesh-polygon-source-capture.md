# Retained Polygon Mesh Source Capture

`MeshPolygonSourceCapture` borrows the same immutable mesh across single funded events. It publishes a `PolygonMeshSource` only after collecting position and face rows, canonical corner and edge identities, raw surface samples, indexed attributes, materials and textures. Callers can quote the next grant with `demand(mesh, copy)`, observe progress, cancel, or retire staged data under the existing retained ownership protocol.

Reversed faces retain their canonical reversed vertex order, corner samples follow the corresponding old corner, and edge samples follow the old predecessor. Implicit attribute indices remain absent when rows keep their canonical order. Sparse surface records keep their original category and index ordering. The same capture now backs the synchronous `HalfedgeMesh::polygon_source()` API.

The existing PolygonCombine implementation moved into the Mesh Reconstruction owner and kept its public facade and progress method. Both operations use the same funded vector reservation path. Mesh Document measured 262,086 bytes, 3,465 active lines and 32 files. Reconstruction measured 158,314 bytes, 1,918 lines and 7 files. The existing source gates passed.

## Verification

- A neutral fixture exercises a flipped quad, an unflipped triangle, a loose point, a sparse normal, shading and UV samples, indexed corner data, unindexed edge data, a seam, nested Unicode metadata, a referenced material, and a texture copied in chunks. Three.js independently checked normal direction, and the portable harness derived the corner and edge remap from undirected vertex pairs. Both Mesh Bun suites passed 19 tests and 3,868 assertions.
- A compact Rust proof compiled the actual Mesh Document, Reconstruction, and Modeling owners. It verified exact output, allocation-refusal progress, every quote receipt, source identity refusal, cancellation after the texture backing was reserved, and serialized source equality afterward. It then ran the relocated Combine against two captured authored sources and confirmed offset faces and surfaces, attribute rows, and asset deduplication.
- The previous authored unwrap native regression still passed. The schema widget integration is tracked separately.
