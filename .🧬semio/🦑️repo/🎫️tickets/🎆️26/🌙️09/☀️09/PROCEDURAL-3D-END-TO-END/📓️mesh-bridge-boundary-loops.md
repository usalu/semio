# Mesh Bridge Boundary Loops

`brep.mesh.bridge` is available in Flow and the procedural 3D catalogue. It takes two directed boundary halfedge indices as an ordered pair. The edges determine the loop correspondence, winding and wall ordering. The Flow example pair is `[0,7]`. The retained Mesh job is also available directly through `MeshModelingOps`.

The job walks the two boundary loops, verifies disjoint simple loops with equal edge counts, and rejects folded or degenerate wall quads. It rebuilds the full mesh with opposite rim incidence and keeps existing vertices, faces and authored source surface samples. Every new wall gets a flat-shaded, separate square UV chart including when a named UV channel is active. Generic boundary and corner attributes reuse their matching source samples. Wall faces take their source face attributes from the first loop.

## Verification

- The Mesh job and packaging Bun suites passed 17 tests with 3,814 assertions. AJV validated the neutral ordered anchor schema. Three.js independently confirmed the first wall’s anchor order, two reciprocal incidences per edge, surface area 6 and signed volume 1.
- A scoped Rust build compiled the current Mesh Contract, Document, Clipping, Reconstruction and Modeling sources. The native fixture passed closed-shell incidence, tessellated area and signed volume, original vertex normals and flat wall shading, indexed named UV / raw corner UV behavior, generic face provenance and preservation of both halfedges of an authored rim seam. Same-loop, shifted alignment and branched-boundary requests left their mesh byte-for-byte unchanged. An unequal-loop request was rejected.
- The four committed JSON manifest carriers independently parsed with the same operator ID and anchor channel. The packed descriptor parsed as a 19-field string table with just its embedded manifest changed and the remainder byte-identical. English and German labels were verified in both catalogue fixtures.
- The Flow workbench regression uses the same neutral fixture and checks generated face order and original UV samples through the registered operator. It passed Rust source parsing. Executable Flow integration was outside the scoped native proof.
- The framework 3D source owner measured 243,783 bytes, 2,812 active lines and 15 files. Mesh Document remained at 258,960 bytes, 3,423 active lines and 32 files. Existing gates passed without threshold changes.
