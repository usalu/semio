# Schema Mesh Bridge and Combine

Generation3d's typed geometry catalogue now exposes Bridge and Combine. Its coverage fixture matches all 55 committed Flow Mesh operator IDs. Both new kinds have English and German labels and preserve the mesh output contract.

Bridge reads one directed edge from each of two separately named selection ports on the same mesh. Both ports begin empty. It validates their order and staleness before admitting the retained Bridge job. Combine reads two typed inputs in authored order, captures each source with the same kernel-owned cursor used for winding and surface metadata, and then advances the funded PolygonCombine and mesh import. A cancellation retires the widget before a mesh can publish. The Bevel description now reflects the certified concave-edge case.

## Verification

- The shared Mesh fixture covers equal Bridge rims, triangle-to-square Bridge, a stale second anchor, and Combine appending a separate triangle to a quad with offset face indices. The Three.js and Manifold oracles match the stated geometry and topology. Ajv validates the catalogue, including the empty initial single-edge picks. All 58 portable Mesh and catalogue tests passed with 349 assertions.
- Two compact Rust tests ran the actual Bridge and Combine compute sources with the real StepContext and Mesh Document, Reconstruction and Modeling owners behind a small typed widget adapter. The four fixture rows matched at fuel 1, 7 and 1,024, including exact ordered topology and the stale port. Cancellation during capture and during the join produced no mesh.
- The full native semantic catalogue package and canonical app are still pending. Its linked Rust fixture is registered and shares the neutral cases. The scoped source measure found Mesh Support at 2 files, 25,266 bytes and 531 active lines and Mesh Edit at 3 files, 39,583 bytes and 182 active lines.

The focused command is `repo tools exec --light python -- <ticket>/🐍️bevel-native-proof.py schema-edit`.
