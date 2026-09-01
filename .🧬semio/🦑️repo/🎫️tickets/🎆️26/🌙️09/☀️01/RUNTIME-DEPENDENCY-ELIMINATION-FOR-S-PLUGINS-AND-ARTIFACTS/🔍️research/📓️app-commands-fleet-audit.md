# `app_commands!` Fleet Audit

Date: 2026-09-01

## Inventory

The literal text search returns 157 files under `✏️s`. It is not an invocation
inventory: it includes macro re-exports, imports, command-payload references, test
fixtures, and documentation. The Rust source invocation pattern
`app_commands!\s*\{` has **50 real expansions** across **29 plugin crates**.

The 50 expansions are distributed as follows:

| Expansions | Plugin |
| ---: | --- |
| 15 | `📕️norm` |
| 3 | `🪐️space` |
| 3 | `🧱️block` |
| 2 each | `🏗️fem`, `🌍️gis`, `🌀️procedural` |
| 1 each | `✒️writer`, `➗️mathematical`, `🌊️flow`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`, `🎪️demonstrator`, `🎬️sequence`, `🏛️architect`, `🏭️process`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`, `📐️cad`, `📖️playbook`, `📜️imperative`, `📸️remodel`, `🕸️dag`, `🖍️draw`, `🖨️raster`, `🗒️note`, `🪵️sourcing` |

27 of the 29 invoking-plugin manifests have no direct `serde` dependency. They
account for 46 of the 50 actual expansions. Under the former macro body, each was
susceptible to the same direct dependency / payload-trait regression; this is much
larger than the 156-file textual inventory suggested.

## Current Macro Contract

All three `app_commands!` arms now derive precisely:

```rust
#[derive(Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, dsl::DslOps)]
```

The staged framework change removes the former hardcoded
`::serde::Serialize, ::serde::Deserialize` derives. The macro's actual wire
implementation remains `DslOps` plus the handwritten `protocol::OpText` and
`protocol::OpBinary` implementations; it does not use a `serde_json::Value` wire
path. A global serde-free generated command is therefore the correct contract; an
opt-out would retain an unnecessary second API and make every future caller decide
whether the same wire path needs serde.

## Dependent Plugin Cleanup

`🏗️fem` already has neither a direct serde manifest entry nor non-comment,
production serde references.

`➗️mathematical` still had the historical restoration. This audit removed serde
from `MathematicalCamera`, `MathematicalPoint`, `MathematicalGeometry`, the seven
generated-command payloads, and the `MathematicalGraphDsl` serde bridge that only
existed because `SetArtifact` was wrapped by the old macro. Its manifest now retains
only `serde_json` for the `JsonSnapshot` boundary in `🗄️stdio`; a production source
scan finds no direct serde API, derive, or attribute.

## Verification

`cargo check -p semio-framework-plugin` was requested first. The first attempt
reached the former shared blocker (duplicate `DslValue` serde implementations),
which a concurrent wave then resolved. Two further framework checks remained
blocked by other in-flight OS-kernel work: first by missing serde on
`MutationLeafDescriptor` in two SPR report structs, then by 18 `store` errors
including orphaned `#[serde(...)]` attributes and removed serde implementations
for `SpaceAlternative`, `SpaceCheckpoint`, `ArtifactCursorOwners`,
`ArtifactBackboneRef`, `MigrationProvenance`, `OwnerRef`, and `HistoryLane`.

The required framework-clean gate is therefore not satisfied. Per the requested
order, `cargo check -p semio-s-plugin-mathematical` and
`cargo check -p semio-s-plugin-fem` were not run: either would stop in the same
shared dependency before reaching its plugin. The Mathematical source/manifest
scan after cleanup is clean of direct serde API, derives, and attributes;
`serde_json` is its sole remaining direct serde-family manifest dependency.
