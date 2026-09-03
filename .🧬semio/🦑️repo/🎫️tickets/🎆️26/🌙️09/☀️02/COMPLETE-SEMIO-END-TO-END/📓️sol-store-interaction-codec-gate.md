# Store Interaction Codec Gate

## Scope

Repaired the `ArtifactPack for protocol::InteractionState` boundary in the OS store. The change is limited to the owning store module and one language-neutral store fixture. No WGPU, descriptor, GIS, AGENTS, or hub files were edited.

## Root cause

`InteractionState` deliberately no longer implements serde. Its canonical codecs are the first-party `ToValue` and `FromValue` traits over `DslValue`. The pack bridge still routed through `serde_json::to_value` and `serde_json::from_value`, which imposed the obsolete `Serialize` and `DeserializeOwned` bounds and produced the two E0277 diagnostics formerly reported at store lines 19828 and 19833.

The current tree already contained a concurrent partial repair that converted through `DslValue` but still detoured through `serde_json::Value`. The completed boundary is direct and single-source:

```text
InteractionState ⇄ ToValue/FromValue ⇄ DslValue ⇄ ArtifactPack
```

This is wire-compatible with the previous JSON value pack path because `pack_rt::encode_json_value` itself converts JSON to the same `DslValue` and calls `encode_pack_value`.

## Changes

- `🏪️store/🦀️.rs`: `InteractionState::encode_pack_with` now converts through `ToValue` and delegates to `ArtifactPack for DslValue`; decode performs the inverse through `FromValue`. There is no serde runtime bound or JSON intermediate.
- `🏪️store/🧪️interaction-state-pack.json`: added a language-neutral state fixture covering selection, hover, active mode, active granularity, target ids, and anchor id.
- Added `interaction_state_pack_matches_first_party_value_and_json_oracle`, which proves:
  - the fixture hydrates through the first-party value codec;
  - `InteractionState` pack bytes exactly match `DslValue` pack bytes;
  - the same bytes match the existing independent `serde_json::Value` pack oracle;
  - decoding restores the exact state and JSON shape.

## Verification

### Narrow store build

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-store-interaction-codec/target" \
bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- \
  -p semio-framework-os-kernel --all-features --message-format short
```

Result: blocked before OS store compilation by 18 current-tree errors in `semio-framework-replication`:

- six missing serde derive macro errors at `📡️wire/🦀️.rs:1690`, `:1727`, and `:1755`;
- twelve E0277 serde bounds rooted at `📡️wire/🦀️.rs:1574–1580` for `SelectionMode`, `SelectionMethod`, and `MergeMode`.

Those files are concurrently changing and are outside this exact two-diagnostic packet, so they were not modified.

### Focused store test

Corrected command:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-store-interaction-codec/target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel --message-format short \
  interaction_state_pack_matches_first_party_value_and_json_oracle -- --exact
```

Result: test execution was not reached. The cold test profile began rebuilding the full OS dependency graph, including unrelated renderer dependencies, and was cancelled rather than broaden this packet while the known replication compile gate remained unresolved.

### DB discovery and hub quick

The requested DB `--lib -- --list` and `os-hub:test-quick` commands were invoked with the same isolated target. Neither reached DB discovery or hub test execution before the cold dependency rebuild was cancelled. The attributable compile blocker remains the 18 replication-wire diagnostics captured by the narrow check above; no new store, DB, or hub diagnostic was observed.

### Diff integrity

```sh
git diff --check -- \
  '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs' \
  '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️interaction-state-pack.json' \
  '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-store-interaction-codec-gate.md'
```

Result: no whitespace errors.

## Residual blocker

The replication serde migration must compile before the focused store test, DB test inventory, or hub quick lane can provide nonzero test execution evidence. The store codec no longer requires serde and is ready for those reruns once that upstream gate settles.
