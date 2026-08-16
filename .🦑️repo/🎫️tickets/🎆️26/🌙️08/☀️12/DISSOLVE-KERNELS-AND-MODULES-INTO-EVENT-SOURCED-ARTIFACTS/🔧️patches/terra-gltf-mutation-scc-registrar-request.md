# glTF Mutation SCC Registrar Request

## Source Fingerprint

Rehash before applying this central-only change. The source-side SCC fingerprint is `068d5321f7a15c782540a83db5b1c207f5a62d8fa8a8a9f50fef4c2b0a35aca4`, calculated from sorted SHA-256 entries for every file under:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations`

Key leaf hashes are:

| Leaf | SHA-256 |
| --- | --- |
| mutation collection manifest | `2b3405064bcc8eb1ec8f4616cb8a22990ab8201c6e50dfbd84df25fc17d62c61` |
| dispatch Rust aggregate | `b91e39305b7375e838470c65f3aa681a93a6b14a3f8619360c1d852bc9663a3f` |
| transport collection manifest | `6005417aa329484cb18ddf905b59dd557e274bfac4404eb3b0a77d1510142684` |

## Central-Only Targets

Do not edit source owners. Regenerate only the glTF 2.0 any-subset branch in:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, current mutation block lines 2247–2510, module block lines 2512–2521, and I/O block starting at line 2523.
- root `📜️script.ts`, current generator-input entry at line 7750.

## Required Registrar Replacement

1. In `schema::mutations`, delete every retired mount: root `component` plus `pub use component::*`, `planning`, `binary`, `text`, and all 84 nested `diff`/`inverse`/`mutation` mounts. Retain this collection only as the following 28 direct command mounts; do not add any re-export or forwarding alias.

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️no-mutation/🦀️component.rs"]
pub mod no_mutation;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️set-snapshot/🦀️component.rs"]
pub mod set_snapshot;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-asset/🦀️component.rs"]
pub mod set_asset;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-scene/🦀️component.rs"]
pub mod insert_scene;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-scene/🦀️component.rs"]
pub mod remove_scene;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-scene/🦀️component.rs"]
pub mod set_scene;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-node/🦀️component.rs"]
pub mod insert_node;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-node/🦀️component.rs"]
pub mod remove_node;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-node/🦀️component.rs"]
pub mod set_node;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️transform-node/🦀️component.rs"]
pub mod transform_node;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️reparent-node/🦀️component.rs"]
pub mod reparent_node;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️bind-node-mesh/🦀️component.rs"]
pub mod bind_node_mesh;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-mesh/🦀️component.rs"]
pub mod insert_mesh;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-mesh/🦀️component.rs"]
pub mod remove_mesh;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-mesh/🦀️component.rs"]
pub mod set_mesh;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-accessor/🦀️component.rs"]
pub mod insert_accessor;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-accessor/🦀️component.rs"]
pub mod remove_accessor;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-accessor/🦀️component.rs"]
pub mod set_accessor;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-material/🦀️component.rs"]
pub mod insert_material;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-material/🦀️component.rs"]
pub mod remove_material;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-material/🦀️component.rs"]
pub mod set_material;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️bind-primitive-material/🦀️component.rs"]
pub mod bind_primitive_material;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-buffer/🦀️component.rs"]
pub mod insert_buffer;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-buffer/🦀️component.rs"]
pub mod remove_buffer;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-buffer/🦀️component.rs"]
pub mod set_buffer;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-animation/🦀️component.rs"]
pub mod insert_animation;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-animation/🦀️component.rs"]
pub mod remove_animation;
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-animation/🦀️component.rs"]
pub mod set_animation;
```

2. In sibling `schema::modules`, add exactly this direct leaf mount. Keep the existing measurement/vector/topology/inference module mounts unchanged.

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🦀️component.rs"]
pub mod mutation_dispatch;
```

3. In sibling `io`, preserve `io::component` and `io::inferences`, then add the two final transport leaves under one mechanical `mutations` collection:

```rust
#[path = "."]
pub mod mutations {
    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️component.rs"]
    pub mod text;
    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️component.rs"]
    pub mod binary;
}
```

4. In root `📜️script.ts`, replace only the glTF generator-input entry:

```text
.../🧬️schema/🧬️mutations/🦀️component.rs
```

with:

```text
.../🔨️modules/🧭️mutation-dispatch/🦀️component.rs
```

Do not add aliases, do not retain retired source path mounts, and do not modify non-glTF registrar branches.
