# Plugin Children Transparent Mount Plan

## Scope and Current State

This is a read-only design audit while Plugin is frozen for runtime R6. The current owner is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️children-fixture`.

The uninhabited roster is semantically sound: `ChildrenTestMutation {}` declares `Mutation::DESCRIPTORS = &[]`; all mutation methods are exhaustive `match *self {}`; text and binary decoders reject every input. The fixture owns four native tests:

- `empty_roster_has_no_fabricated_leaf`
- `every_neutral_json_value_is_uninhabited`
- `empty_and_nonempty_codec_inputs_are_rejected`
- `existing_children_diff_stays_identity`

The transparency defect is structural. `derived_artifact_children_tests` in the Plugin main source currently mounts `mutations/🦀️.rs` using `include!`, and that leaf in turn mounts its native tests using a second `include!`. This plan removes only those two inclusions. It does not change the uninhabited type, descriptors, codecs, child-slot behavior, or the pending unrelated TestDocument aggregate work.

## Exact Future Production Write Scope

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, only the crate-root fixture-mount block beside the existing test fixture modules and `derived_artifact_children_tests` lines currently containing `ChildrenTestSnapshot`, `ChildrenTestDiff`, and the `mod mutations` `include!` wrapper.
2. New direct fixture root `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️children-fixture/🦀️.rs`.
3. Existing `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️children-fixture/🧬️mutations/🦀️.rs`, only to remove its nested test `include!` and obtain snapshot/diff from the fixture-root parent.
4. Existing `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️children-fixture/🧪️tests/🦀️.rs` remains the single native test body and should not be copied.

No schemas, vectors, compatibility aliases, fake directories, lifecycle code, Interaction, commands, or TestDocument sources belong in the move.

## Crate-Root `#[path]` Layout

At the Plugin crate root beside `publication_fixture`, `mutation_fixture`, and `test_app_mutation_fixture`, introduce the direct fixture owner:

```rust
#[cfg(test)]
#[path = "🧪️tests/🧬️children-fixture/🦀️.rs"]
pub(crate) mod children_fixture;
```

`derived_artifact_children_tests` then imports only the three owned test types:

```rust
use crate::children_fixture::{ChildrenTestDiff, ChildrenTestMutation, ChildrenTestSnapshot};
```

It removes its inline snapshot/diff declarations and its `mod mutations { include!(...) }` wrapper. The new crate-root fixture owner imports only `serde::{Deserialize, Serialize}` and names `protocol::MutationDiff` directly; it does not import the Plugin root. It owns the two shared structural types, their identity `MutationDiff` implementation, and direct path mounts:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ChildrenTestSnapshot;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ChildrenTestDiff;

impl protocol::MutationDiff<ChildrenTestSnapshot> for ChildrenTestDiff { /* existing identity body */ }

#[path = "🧬️mutations/🦀️.rs"]
mod mutations;
pub(crate) use mutations::ChildrenTestMutation;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
```

The leaf becomes a normal direct child of that root with `use super::{ChildrenTestDiff, ChildrenTestSnapshot};`; its nested test wrapper is removed. The native test body replaces `use super::*` with an exact import of `ChildrenTestDiff`, `ChildrenTestMutation`, and `ChildrenTestSnapshot`. `pub(crate)` makes the three test-only types accessible to `derived_artifact_children_tests` via `crate::children_fixture`; no fixture imports the whole Plugin root. This is a real crate-root module tree, not `include!`, and each relative `#[path]` is resolved from its authored fixture directory.

## Regression Plan

The ticket controller now has a `--transparent-mount` source-regression mode. Its retained scoped Bun/Nx RED is [transparent-mount-ZL8BbZ](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-children-fixture-44/🧫️transparent-mount-ZL8BbZ/📓️result.md): `6/14`; the eight failures are exactly the missing crate-root owner/import/direct mounts plus the two current `include!` and visibility/glob-import defects. The nofollow captured source inputs were stable. After the move the mode must validate the one crate-root mount, the two fixture-owned direct mounts, the existing empty descriptor schema and neutral cases, and all four unchanged native test names. The runtime gate must then compile and execute the same four tests; no native run was performed in this read-only packet.

## Baseline Hashes

| Input | SHA-256 |
| --- | --- |
| Plugin main | `12bc97e01166b3c50fccdd5221264174c14aaaa8a7aae36d11587f3cf4a9345d` |
| Current mutation leaf | `cd030ae4b90d53b981fde6d9ee44e6e10eab8b8985d90021926267f1b3eab4b0` |
| Native test body | `a85e95ace55c4a30c68e9c2b4ceac74c3cbd14224972796f2c86e5535ec77a9a` |
| Empty mutation schema | `ad6736b9851348e1284f32db31db13f753de9ac83d1e9eb01a0a8f59fb58dcb4` |
| Neutral cases | `411f4b318d986b7dbbb88c2a1294061809b5452dc68920e2caa9300343a6f25d` |
