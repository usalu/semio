# GlTF Inference Root Registrar

## Lease

- Owner: Sol central registrar
- Registrar: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
- Semantic owner: `s.stdio.gltf`

## Pre-Change Evidence

- The former inference-root Rust and TypeScript leaves each contained only a 75-byte mechanical region.
- GraphQL and Protocol aggregate contracts had moved to `🧮️geometric-analysis`.
- No Rust source imported the former `schema::inferences::component` module or its aggregate public symbols.
- The direct `geometric_analysis` mount was present exactly once.

## Registrar Change

Removed the sole stale former-root mount and its forwarding re-export:

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
mod component;
pub use component::*;
```

The direct mount remains the only aggregate mount:

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧮️geometric-analysis/🦀️component.rs"]
pub mod geometric_analysis;
```

No alias, forwarding export, or compatibility mount was introduced.

## Validation

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

The report-mode command exited successfully with 23 components, 76 errors, and 0 warnings. The errors are remaining active structural work: artifact/standards manifests, I/O serializer/deserializer manifests, and the coupled mutation-root tree. Report mode is intentionally non-enforcing; this result is not a clean-gate pass.
