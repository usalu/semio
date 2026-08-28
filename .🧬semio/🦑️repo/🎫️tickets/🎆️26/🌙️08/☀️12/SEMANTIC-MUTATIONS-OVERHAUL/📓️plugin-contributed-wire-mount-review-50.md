# Plugin Contributed-Wire Mount Review

## Reviewed Snapshot

Read-only review of the Plugin fixture mount, its direct fixture root, its child tests, the contributed-wire source controller, and the queued TestMutation aggregate test import. No controller or production source was edited and no native command was run.

At the reviewed current source, the mount repair is present at [Plugin component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:222):

```rust
#[cfg(test)]
#[path = "🧪️tests/🧬️contributed-mutation-wire/🦀️.rs"]
pub(crate) mod contributed_mutation_wire;
```

It is directly adjacent to the existing crate-visible publication, mutation, and test-app fixture mounts, before `pub mod app`. The former nested mount under inline `plugin_runtime` is absent. This is the required Rust path-resolution correction: a `#[path]` inside that inline module is resolved through the virtual `plugin_runtime` child directory, whereas the fixture exists at the Plugin crate root's `🧪️tests/🧬️contributed-mutation-wire/🦀️.rs`.

The nested wire-client tests now import the module through [Plugin component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:29135) with `crate::contributed_mutation_wire::{AddValue, WireTestMutation, WireTestSnapshot}`. `pub(crate)` is sufficient: these tests are compiled in the same Plugin crate and do not require an external public API.

The fixture's own `crate::store::{ArtifactPack, PackEncodeOptions, PackDecodeOptions, PackError}` references at [contributed-wire fixture.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:12) independently confirm that a Plugin-crate-root mount is the correct owner context. Its internal `mod mutations` and `mod tests` remain children of the fixture root and need no visibility change.

## Controller Coverage

The current controller at [plugin-contributed-wire script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-contributed-wire-43/📜️script.ts:105) already has the exact mount regression assertions needed for this repair:

- the mount is crate-root and `pub(crate)`;
- it precedes `pub mod app`;
- nested tests use `crate::contributed_mutation_wire`;
- neither the old nested module declaration nor `super::contributed_mutation_wire` remains.

It also first-hashes and final-rereads the component input through its existing guarded reader. No controller correction is needed for this mount repair. The controller was not replayed here, so this review is not a source-gate or native-pass claim.

## Queued TestMutation Import Blocker

The aggregate at [test-app TestMutation.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs:28) has a real child-test scope issue. Its `#[mutations(snapshot=super::TestSnapshot, ...)]` attribute resolves from the aggregate module, but that does not import `TestSnapshot` into the nested `tests` module. `use super::*` imports the aggregate's own items, not the document-parent re-export.

The bare `TestSnapshot { ... }` in that test therefore needs the narrow explicit child import:

```rust
use super::super::TestSnapshot;
```

The document parent re-exports `TestSnapshot` as `pub(crate)` from `crate::app`, so the import is valid within the same crate. This is independent of the contributed-wire mount repair and should be applied only in the queued TestMutation test module.

## Native Status

No Cargo, rustc, or Plugin native test was run. The reviewed source placement removes the known R5 virtual-directory path-resolution failure; derive expansion and the wire-client tests remain for the root-owned native gate.
