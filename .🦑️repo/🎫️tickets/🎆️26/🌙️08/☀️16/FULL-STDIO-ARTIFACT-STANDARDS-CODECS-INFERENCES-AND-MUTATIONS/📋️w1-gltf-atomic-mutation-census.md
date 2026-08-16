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

## Canonical semantic vocabulary

Every resulting command carries a stable `.v1` descriptor and owns its payload validator, sparse direct diff, inverse, touched paths, and reference repair. The command root is limited to descriptor and wire-enum assembly. It does not read payload fields or perform mutations.

| Canonical command | Replaces |
| --- | --- |
| `change-asset-metadata` | `SetAsset` |
| `create-scene`, `delete-scene`, `change-scene` | scene insert/remove/set |
| `create-node`, `delete-node`, `change-node`, `transform-node`, `reparent-node`, `bind-node-mesh`, `unbind-node-mesh` | node insert/remove/set/transform/reparent and optional bind |
| `create-mesh`, `delete-mesh`, `change-mesh` | mesh insert/remove/set |
| `create-accessor`, `delete-accessor`, `change-accessor` | accessor insert/remove/set |
| `create-material`, `delete-material`, `change-material`, `bind-primitive-material`, `unbind-primitive-material` | material insert/remove/set and optional bind |
| `create-buffer`, `delete-buffer`, `change-buffer-descriptor`, `update-buffer-bytes` | buffer insert/remove/set |
| `create-animation`, `delete-animation`, `change-animation` | animation insert/remove/set |

`NoMutation` and `SetSnapshot` have no replacement: valid commands must produce an observable sparse domain diff. Optional relationship fields are split into bind and unbind commands to make intent and inverse replay deterministic.

## Bounded batches

1. Contract manifests and all root transport schemas: canonical `.v1` IDs and enum-only root descriptors.
2. Asset, scene, and node leaf triplets with Rust/TS/JSON/GraphQL/Proto/text/binary facets.
3. Mesh, accessor, material, buffer, and animation leaf triplets plus direct fixtures.
4. After the inference freeze, hand off the canonical leaf inventory to the dispatcher/transport owner for enum assembly and executable round-trip gates. No aliases, legacy tags, or compatibility decoding are permitted.

## Verification boundary

Before handoff, verification is source-contract-only: JSON/GraphQL/Proto/text/binary facet inventories, stable ID checks, and fixture shape checks. Runtime Cargo/Nx checks are owned by the coordinator after the dispatcher and inference declaration paths have frozen.
