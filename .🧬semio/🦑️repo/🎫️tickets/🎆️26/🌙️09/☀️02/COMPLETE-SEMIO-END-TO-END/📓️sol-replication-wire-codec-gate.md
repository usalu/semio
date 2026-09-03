# Replication Wire Codec Compile Gate

## Scope

This packet repaired the selection codec compile boundary in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` without restoring serde on the affected wire types. No descriptor, WGPU, GIS, or `AGENTS.md` file was edited.

## Reproduction and attribution

The required isolated command was:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-replication-wire-codec/target" bun nx run '@semio-tech/framework-replication-rs:build' --skip-nx-cache
```

The first reproduction on the shared tree exited 0 in 25.60 seconds only because qualified serde derives had concurrently been restored. The preceding blocked lane's 18 diagnostics grouped into two roots:

1. Six derive-resolution diagnostics on the three leaf enums `SelectionMode`, `SelectionMethod`, and `MergeMode`.
2. Twelve `E0277` container fan-out diagnostics from `SelectionSpec` requiring serde for `Vec<SelectionMode>`, `Vec<SelectionMethod>`, and `Vec<MergeMode>` during both serialization and deserialization.

Removing serde from the four directly affected types exposed eight additional `E0277` diagnostics at the next container, `InteractionState`: its serde derive still required the already first-party-coded `DomainHover` and `SelectionMode`. Removing that obsolete container derive completed the boundary. This is a compile-contract repair; the existing hand-written `ToValue`/`FromValue` implementations and their field/variant labels were not changed.

During downstream verification, another concurrent writer restored the five serde derives. Per coordinator direction, the minimal removals were reapplied once against the latest file. The final immediate isolated build exited 0 in 6.80 seconds with zero errors.

## Changes

- Removed `serde::Serialize`/`serde::Deserialize` from `SelectionSpec`, `SelectionMode`, `SelectionMethod`, `MergeMode`, and `InteractionState`.
- Kept the canonical first-party representations unchanged: `SelectionSpec` remains its five-field `DslValue` object; enum labels remain `single|multiple`, `pick|rectangle|lasso`, and `replace|additive|subtractive|invertive|range`; `InteractionState` remains the `selection`, `hover`, `activeMode`, and `activeGranularity` object.
- Removed the obsolete `default_true` serde helper after its last attribute consumer disappeared.

No new fixture was warranted because this packet changes trait availability only, not wire behavior. Existing language-neutral `local-interaction` JSON fixtures already cover both `single` and `multiple`, and the existing `local_interaction_language_neutral_restore_parity` test validates the first-party `DslValue` projection against the fixture through the existing `serde_json::Value` test oracle.

## Verification

### Replication build

```sh
CARGO_TARGET_DIR=".../repair-replication-wire-codec/target" bun nx run '@semio-tech/framework-replication-rs:build' --skip-nx-cache
```

Final result: exit 0, 6.80 seconds, zero compile errors. Remaining warnings are unrelated pre-existing qualification/dead-code warnings.

### Replication quick tests

```sh
CARGO_TARGET_DIR=".../repair-replication-wire-codec/target" SEMIO_TEST_ARTIFACT_DIR=".../repair-replication-wire-codec/artifacts" bun nx run '@semio-tech/framework-replication-rs:test-quick' --skip-nx-cache
```

Result: compilation succeeded and 238 tests executed; 237 passed, 1 failed. All interaction/presence/local-interaction codec tests passed, including `local_interaction_language_neutral_restore_parity`. The sole failure is unrelated fixture drift in `causal::tests::causal_add_fixture_has_exact_required_descriptor`: the produced `payloadSchema` is `🛂️schema.json`, while the fixture expects `../🛂️schema/🔣️.json`.

### Narrow store codec test

```sh
CARGO_TARGET_DIR=".../repair-replication-wire-codec/target" SEMIO_TEST_ARTIFACT_DIR=".../repair-replication-wire-codec/artifacts" bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel --message-format short interaction_state_pack_matches_first_party_value_and_json_oracle -- --exact
```

The build compiled through replication, os-kernel, framework, plugin, and infinite without a replication/store codec diagnostic. It then reached the first unrelated blocker at `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️.rs:204`: `E0277`, `ProgramContributionEntry: serde::Serialize` is not satisfied. Remaining parallel cargo jobs were interrupted after that stable diagnostic, so this command ended with exit 130 and did not execute the focused test.

### DB lib test discovery

```sh
CARGO_TARGET_DIR=".../repair-replication-wire-codec/target" SEMIO_TEST_ARTIFACT_DIR=".../repair-replication-wire-codec/artifacts" bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db --all-features --message-format short -- --list
```

Result: exit 0 after 1 minute 44 seconds; `semio-framework-os-kernel-db` compiled and discovered 626 tests and 0 benchmarks.

### Hub quick

```sh
CARGO_TARGET_DIR=".../repair-replication-wire-codec/target" SEMIO_TEST_ARTIFACT_DIR=".../repair-replication-wire-codec/artifacts" bun nx run os-hub:test-quick --skip-nx-cache
```

Result: exit 1 after reaching `semio-hub`; the replication diagnostics are gone. The next six hub-owned/non-replication diagnostics are:

- one `E0277`: `HybridLogicalTimestamp` is not a future (the current hub test helper calls `HybridLogicalTimestamp::new(0, 0).await` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1999`);
- two `E0277`: `DbIoPages: axum::response::IntoResponse` is not satisfied;
- two `E0308` mismatched-type diagnostics;
- one `E0277`: `MutationMessage: serde::Serialize` is not satisfied, from the hub's remaining serde JSON message encoder at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:606`.

These are outside the replication wire selection boundary and were not changed in this packet.

## Residual status

The requested replication compilation gate is restored with the affected selection/runtime types on first-party `ToValue`/`FromValue` only. Replication behavior tests compile and all relevant codec tests pass. DB lib test discovery is green and nonzero. The next end-to-end blockers are the unrelated causal fixture mismatch, imperative registry serde call, and six hub-owned diagnostics listed above.
