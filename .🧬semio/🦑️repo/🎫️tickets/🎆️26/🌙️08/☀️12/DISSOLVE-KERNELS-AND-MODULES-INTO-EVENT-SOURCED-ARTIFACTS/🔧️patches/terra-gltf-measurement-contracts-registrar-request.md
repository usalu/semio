# glTF Measurement Contracts Registrar Request

Regenerate only the glTF 2.0 any-subset branch of `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` after the source move.

1. Delete the old `schema::inferences::measure` branch that currently mounts `🧬️schema/💡️inferences/🧾️measure/🦀️component.rs`.
2. Add this exact sibling branch inside `artifacts::gltf::standards::v2_0::subsets::any::modules`:

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧾️measurement-contracts/🦀️component.rs"]
pub mod measurement_contracts;
```

Do not create an inference forwarding module, old-path alias, or duplicate mount. Existing `vector_operations`, `mesh_topology`, and `inference_measures` module mounts remain unchanged.
