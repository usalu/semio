# ArtifactApp Snapshot Bound Verification

## Scope

This follow-up verifies the framework-side removal of the `ArtifactApp::Snapshot`
serde constraint identified in `📓️serde-fanout-imperative.md`. It excludes the already
converted `📜️imperative/🧩️extensions/{🎮️control,📝️text,📣️effect,🧠️logic,🧮️math}`
directories.

## Static evidence — 2026-09-01

The shared, in-flight edit to
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` now declares
the primary `ArtifactApp::Snapshot` bound as:

```rust
Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync
    + store::ArtifactDsl + ArtifactPack + 'static
```

The two surface-adapter declarations in the same file use the identical snapshot bound.
The generic `PluginBuilder::document_codec_bare` and `DocumentCodecSpec::bare` restatements
were moved from `Serialize + DeserializeOwned` to the same first-party codec pair, including
the nested `codec` helper. `serde::de::DeserializeOwned` is no longer imported by the app
module; its only remaining uses in the file are owned-component ABI JSON helpers, which are
not part of the `ArtifactApp` contract.

The `ArtifactApp` implementors directly called out by the incident are still correctly
identified as:

- `🌀️procedural`: `AssemblySnapshot`, `Procedural2dSnapshot`, and `Procedural3dSnapshot`.
- `📖️playbook/🧩️extensions/🌀️procedural`: `ModuleRenderPayload`.

`ModuleRenderPayload`, `ModulePayloadMutation`, and `ModulePayloadDiff` in the playbook
procedural extension now derive `ToValue` and `FromValue` alongside their serde derives. Their
`#[value(...)]` layouts reproduce the serde JSON field/tag naming, and the action bridge now
accepts the contract's `DslValue` argument instead of `serde_json::Value`. The extension adds the
first-party `semio-framework-value-derive` macro dependency while deliberately retaining
`serde`/`serde_json`: other runtime JSON work in that crate still needs the latter. A test compares
the first-party `DslValue` encoding with `serde_json::to_value` and round-trips the snapshot,
mutation, and diff through `FromValue`.

The three procedural snapshots still need their own recursive first-party codec migration.
Their `FlowFixture`/generation and stdio child graph has the documented large fan-out, so the
procedural manifest must retain serde until that source conversion is complete.

## Verification scheduling

The required foreground `cargo check` has not been started by this follow-up. At inspection,
the shared host had multiple active Cargo checks/tests, including workspace checks and a
wasip2 plugin check; starting another one would violate the ticket's one-build-at-a-time rule.
When the build slot is free, run the plugin-focused foreground checks without overriding
`CARGO_TARGET_DIR`, beginning with the framework/plugin crate and then the two named plugin
crates, to surface and complete the concrete `ToValue`/`FromValue` implementation fan-out.

`git diff --check` on this follow-up's extension files and `rustfmt --edition 2021 --check` on
its Rust source both completed successfully. They are structural checks only, not a substitute
for the blocked Cargo verification.
