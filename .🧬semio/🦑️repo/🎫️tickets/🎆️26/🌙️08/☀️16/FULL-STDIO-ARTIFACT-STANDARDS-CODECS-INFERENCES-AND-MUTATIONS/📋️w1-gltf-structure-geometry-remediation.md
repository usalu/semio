# GLTF Structure Geometry Mutation Remediation

## Accepted Count

0 of 71 assigned matrix leaves are accepted in this handoff. Four relation leaves are source-complete but remain unaccepted until their Rust shared-vector tests execute after the concurrent Cargo builds finish:

- `s.stdio.gltf.mutation.bind-node-child.v1`
- `s.stdio.gltf.mutation.bind-scene-root-node.v1`
- `s.stdio.gltf.mutation.unbind-node-child.v1`
- `s.stdio.gltf.mutation.unbind-scene-root-node.v1`

Each owns 19 command-local files: all 15 typed phase facets (Rust, TypeScript, JSON Schema, GraphQL, Proto), a JSON shared vector with Rust and TypeScript consumers, and a canonical `🦀️component.rs` command-root descriptor adapter. The adapters delegate only to the command mutation/diff/inverse functions through the schema-owned `GltfMutationLeafDescriptor`; they recompute application touched paths from the typed relation coordinates and never trust serialized paths.

## Completed Source and TypeScript Gates

- All four TypeScript canonical vectors executed: forward application, deterministic replay, exact undo, stale forward rejection, stale inverse rejection, and forged diff/inverse touched-path rejection (`4 passed`).
- Every Rust diff/inverse apply recomputes its concrete path and rejects forged `touched_paths` with `gltf.mutation.invalid-touched-path`; the corresponding Rust shared-vector assertions cover forged paths, stale forward, stale inverse, replay, and undo.
- `bun x tsc --noEmit --strict --skipLibCheck --target esnext --module nodenext --moduleResolution nodenext --allowImportingTsExtensions --resolveJsonModule` completed for the four TypeScript vector consumers.
- `rustfmt --edition 2021 --check` completed for all 20 relation Rust facets, contracts, and descriptor roots.
- All 16 relation JSON phase/contract facets parsed successfully.
- The four accepted-scope folders contain zero noncanonical `🦀component.rs` names and 20 canonical `🦀️component.rs` files.
- Static scans found no `Gltf*Diff::between`, `GltfDiff`, `topLevelDiff`, or `payload_json` in the four leaves.

## Remaining Gate and Blocker

Rust vector execution was intentionally not run: the workspace had active Cargo/rustc builds owned by other work when checked. Running another Cargo command would violate the compile-concurrency instruction. Once the workspace is clear, run the four `🧪️contract/🦀️component.rs` tests, then let the integration owner mount the descriptor roots. Root/glue, dispatch, transports, artifact-definition JSON, and inference files remain untouched here.

## Explicitly Unaccepted

The other 67 matrix leaves are not counted. The partially remediated `change-node-name.v1` and `change-node-extra-data.v1` folders are also explicitly unaccepted because the parent closed this shard after the four-relation vertical batch; no integration claim is made for them.
