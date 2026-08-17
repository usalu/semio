# glTF Release Audit

## Verdict

**No-go.** The aggregate rename, I/O migration, module LCA, and source mounts are coherent, but the canonical manifest/tree and graph-verification release gates still fail.

## Confirmed

- All parsed manifests have unique semantic IDs and their declared component members exist.
- `vector-operations` has five direct terminal metric consumers and `inference-measures` has fourteen.
- Module ownership is correctly at the glTF 2.0 any-subset LCA.
- The field migration to `geometricAnalysis` is consistent across Rust, TypeScript, JSON, GraphQL, Proto, text/binary formats, and mounts. No public geometry alias remains.
- I/O codecs are under the any-subset I/O owner, and direct glTF mounts resolve. No generated business-code edit was observed.

## Required Corrections

1. Remove stale empty `💡inferences/📦bounds` and `💡inferences/🔨modules` directories; exact manifest/tree bijection forbids them.
2. `mesh-topology` has six direct terminal consumers. Add `s.stdio.gltf.inference.geometric-analysis` to its manifest, or remove that real dependency by making it aggregate-private. The existing five metric declarations alone are incomplete.
3. Reconcile the inference collection: its manifest has sixteen declared members while the tree has eighteen child directories.
4. Correct aggregate manifest targets if they are intended to resolve actual serialized `GltfGeometricInference` fields. `measures`, `context`, and `size` are conceptual nodes, whereas the serialized aggregate has `overall`, `parts`, and `pairs`.
5. Rerun the scoped taxonomy report after the central graph adapter update and then rerun the post-manifest Nx/Cargo validation; prior generic Nx failure is not a release result.
