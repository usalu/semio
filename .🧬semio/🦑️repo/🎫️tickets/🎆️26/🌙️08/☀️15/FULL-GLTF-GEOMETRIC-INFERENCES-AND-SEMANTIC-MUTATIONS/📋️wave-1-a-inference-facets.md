# Wave 1-A — Inference Facets

## Frozen public shape

`GltfInference` has one derived field, `geometry: GltfGeometricInference`. This is the clean greenfield API across TypeScript, GraphQL, JSON Schema, and Proto. There is no legacy `bounds` root, `GltfBounds` alias, or duplicated top-level `min`, `max`, `vertexCount`, `meshCount`, or `primitiveCount`.

The authoritative locations are:

- axis-aligned bounds: `geometry.overall.size.axisAlignedBounds`
- oriented bounds: `geometry.overall.size.orientedBounds`
- bounding-box dimensions: `geometry.overall.size.boundingBoxDimensions`
- aggregate counts: `geometry.counts`

## Contract coverage

The existing bounds TypeScript facet is repurposed as the inference value-contract kernel. It defines units, coordinate spaces, distinct availability and validity states, deterministic policy and tolerance fingerprinting, diagnostics, quality, provenance, entity addressing, typed measures, 67 concrete indicator fields in 14 taxonomy groups, aggregate counts, complete overall indicators, per-part records, and pairwise clearance/contact/interference/adjacency/orientation records.

Every requested universal indicator is a concrete field rather than an open property bag. Unavailable, invalid, degenerate, unresolved, open-surface, non-manifold, and unsupported results remain explicit records with diagnostics, quality, and provenance instead of disappearing from the result.

## Changed production facets

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️component.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️component.proto`

No grammar or transport-envelope facet was changed: the existing text and binary envelope files do not model inference fields. No Rust, mutation, diff, glue, snapshot, or assembly file was changed in this wave.

## Validation log

- `bun` TypeScript transpilation of both TypeScript facets: passed.
- JSON parsing and Draft 2020-12 compilation through the workspace's `ajv/dist/2020`: passed.
- normalized cross-facet parity check for all 67 concrete indicator names in TypeScript, GraphQL, JSON Schema, and Proto: passed.
- root-shape assertions for GraphQL and Proto: passed.
- `bun nx show project '@semio-tech/stdio-plugin'`: passed.
- A GraphQL parser and Proto compiler are not installed in the workspace, so those two facets received structural/root/parity validation rather than tool-native compilation.
- `bun nx run '@semio-tech/stdio-plugin:test-quick'`: failed after 25.4 seconds in concurrent Rust GLTF mutation work. The observed errors include a partial move of `error.path` and non-exhaustive matches for `TransformNode`, `ReparentNode`, `BindNodeMesh`, and `BindPrimitiveMaterial`. None of the reported compiler locations are in the five changed non-Rust inference facets.

## Ownership boundary

Wave 1-A owns the non-Rust public contract only. The Rust inference implementation and assembly lane must expose the same `geometry: GltfGeometricInference` root and populate bounds solely under `overall.size`, while the mutation/diff lanes own the semantic operation compiler failures recorded above.
