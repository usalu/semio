# glTF Atomic Mutation Census and Leaf Contract

Ticket: `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`  
Goal: `🎯aioptimizedrepo`

## Scope and barrier

This lane exclusively owns `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/**/🧬️schema/🧬️mutations/**` plus the glTF mutation text/binary transport facets. It does not change artifact/stdio roots, registry or definitions, inference paths, framework, dispatcher, or cross-artifact glue before the inference declaration freeze. The latter currently owns executable enum dispatch and therefore cannot consume renamed variants safely yet.

## Exact authoritative census

The glTF mutation schema currently has 28 leaves, each with a Rust `🦀️component.rs` and TypeScript `🟦️component.ts`, plus only one root JSON manifest. Rust leaves inline the `🦠️mutation`, `🔺️diff`, and `↩️inverse` regions; TypeScript exports payload/diff/inverse types only. No leaf has JSON Schema, GraphQL, Proto, text, or binary parity. The root has no Rust, TypeScript, GraphQL, Proto, text, or binary facet.

| Legacy family | Count | Legacy variants |
| --- | ---: | --- |
| prohibited no-op/snapshot | 2 | `NoMutation`, `SetSnapshot` |
| generic object replacement | 7 | `SetScene`, `SetNode`, `SetMesh`, `SetAccessor`, `SetMaterial`, `SetBuffer`, `SetAnimation` |
| generic collection insertion | 7 | `InsertScene`, `InsertNode`, `InsertMesh`, `InsertAccessor`, `InsertMaterial`, `InsertBuffer`, `InsertAnimation` |
| generic collection removal | 7 | `RemoveScene`, `RemoveNode`, `RemoveMesh`, `RemoveAccessor`, `RemoveMaterial`, `RemoveBuffer`, `RemoveAnimation` |
| semantic but overloaded | 2 | `BindNodeMesh { mesh: Option<usize> }`, `BindPrimitiveMaterial { material: Option<usize> }` |
| semantic and retained | 3 | `TransformNode`, `ReparentNode`, `SetAsset` (renamed below) |

The legacy wire manifest uses unversioned `s.stdio.gltf.mutation.<slug>` IDs and unversioned `gltf.<slug>` command/event names. `🚪️io/🧬️mutations/📝️text` and `💾️binary` manually match the old enum, inspect every legacy payload, and encode the two prohibited variants. The enum/dispatcher lives outside this lease and is intentionally not changed in this wave.

## Legacy-to-semantic replacement map (not the authoritative inventory)

This table exhausts the 28 legacy variants only. It is a removal map, not a claim that the resulting domain has 29 commands. The authoritative inventory is the complete `GltfSnapshot` coverage matrix in [`w1-gltf-authoritative-mutation-matrix.md`](./w1-gltf-authoritative-mutation-matrix.md). Every resulting command carries a stable `.v1` descriptor and owns its payload validator, sparse direct diff, inverse, touched paths, and reference repair. The command root is limited to descriptor and wire-enum assembly. It does not read payload fields or perform mutations.

| Canonical command | Replaces |
| --- | --- |
| `change-asset-version`, `change-asset-descriptive-metadata`, `change-asset-extension-data`, `change-asset-extra-data` | `SetAsset` |
| `create/delete/move/reorder-scene`; `change-scene-name`; `change-scene-extension-data`; `change-scene-extra-data`; scene-root relation commands | `InsertScene`, `RemoveScene`, `SetScene` |
| `create/delete/move/reorder-node`; `transform-node`; node mesh/camera/skin relation commands; `change-node-morph-weights`; node metadata commands; child-order and `reparent-node` commands | `InsertNode`, `RemoveNode`, `SetNode`, `TransformNode`, `ReparentNode`, `BindNodeMesh` |
| `create/delete/move/reorder-mesh`; mesh weight/metadata commands; primitive and morph-target commands | `InsertMesh`, `RemoveMesh`, `SetMesh` |
| `create/delete/move/reorder-accessor`; accessor layout/bounds/sparse/metadata commands | `InsertAccessor`, `RemoveAccessor`, `SetAccessor` |
| `create/delete/move/reorder-material`; PBR, texture, render-state, and metadata commands; primitive material relation commands | `InsertMaterial`, `RemoveMaterial`, `SetMaterial`, `BindPrimitiveMaterial` |
| `create/delete/move/reorder-buffer`; buffer descriptor/data commands | `InsertBuffer`, `RemoveBuffer`, `SetBuffer` |
| `create/delete/move/reorder-animation`; sampler/channel/target and metadata commands | `InsertAnimation`, `RemoveAnimation`, `SetAnimation` |

`NoMutation` and `SetSnapshot` have no replacement: valid commands must produce an observable sparse domain diff. Optional relationship fields are split into bind and unbind commands to make intent and inverse replay deterministic.

## Bounded batches

1. Contract manifests and all root transport schemas: canonical `.v1` IDs and enum-only root descriptors.
2. Asset, scene, and node leaf triplets with Rust/TS/JSON/GraphQL/Proto/text/binary facets.
3. Mesh, accessor, material, buffer, animation, skin, image/texture/sampler, camera, extension declaration, and document-data leaf triplets plus direct fixtures.
4. After the inference freeze, hand off the canonical leaf inventory to the dispatcher/transport owner for enum assembly and executable round-trip gates. No aliases, legacy tags, or compatibility decoding are permitted.

## Verification boundary

Before handoff, verification is source-contract-only: JSON/GraphQL/Proto/text/binary facet inventories, stable ID checks, and fixture shape checks. Runtime Cargo/Nx checks are owned by the coordinator after the dispatcher and inference declaration paths have frozen.
