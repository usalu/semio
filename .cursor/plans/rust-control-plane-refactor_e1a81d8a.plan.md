---
name: rust-control-plane-refactor
overview: Refactor `semio/rs` so GraphQL mutations, typed kit changes, and subscription events share one Rust control-plane path instead of rebuilding command application and event emission in several places.
todos:
 - id: ticket
   content: Reopen the existing Rust change-tree ticket or open a focused continuation ticket before editing.
   status: completed
 - id: central-helper
   content: Add one internal helper for lift, apply, inverse flattening, count, and classified event emission.
   status: completed
 - id: route-callers
   content: Route transaction commands, GraphQL actor work, and direct graph helpers through the shared helper.
   status: completed
 - id: graphql-cleanup
   content: Consolidate GraphQL batch result construction and remove stale shell/lifecycle wording.
   status: completed
 - id: tests
   content: Extend existing Rust tests for helper parity, classified event emission, inverse payloads, and SDL stability.
   status: completed
 - id: verify-close
   content: Run formatting/tests, update SDL if needed, and close the ticket with touched paths.
   status: completed
isProject: false
---

# Rust Control Plane Refactor

## Scope

- Work mainly in [`semio/rs/lib.rs`](semio/rs/lib.rs), preserving the single-file Rust layout and existing test modules.
- Reopen/continue the existing Rust change-tree ticket after approval, since the current staged work already introduces `KitChange::lift_flat`, classified `KitEvent` rows, and GraphQL `changeKitWithInverse` plumbing.
- Update [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) only if the Rust GraphQL SDL changes.

## Current Smells to Remove

- `TransactionCommand::ChangeKitCommands`, `kit_graphql::spawn_actor`, and direct `KitGraph` helpers each lift/apply/emit changes independently.
- `GraphWork::ChangeKitCommands` and `GraphWork::ChangeKitWithInverse` duplicate nearly the same actor logic.
- `KitStoreBatchResult` construction is repeated across every GraphQL command branch, making changes to result semantics noisy.
- `KitEvent` mixes field invalidation events, classified mutation events, and command lifecycle rows without one shared emission helper.

## Implementation Plan

1. Introduce a small internal execution helper in [`semio/rs/lib.rs`](semio/rs/lib.rs), near `kit_change` or `kit_store_command`, that returns one structured result for a `Vec<ChangeKitCommand>`:
   - pre-lifted `KitChange`
   - semantic `KitChangeKind`
   - flattened inverse atoms
   - applied count / no-op handling
   - classified `KitEvent` emission through one function
2. Route all mutation paths through that helper:
   - `TransactionCommand::ChangeKitCommands`
   - `GraphWork::ChangeKitCommands`
   - `GraphWork::ChangeKitWithInverse`
   - direct field/add/remove helpers that currently call `KitChange::lift_flat` and emit manually
3. Collapse GraphQL batch result creation into concise constructors so every row has consistent `ok`, `count`, `changeKind`, `changeKindOther`, and `inverse` behavior.
4. Normalize event/change naming and serialization boundaries:
   - keep `KitEvent` as the GraphQL scalar wire shape
   - make classified mutation events derive from `KitChange` consistently
   - remove stale lifecycle/shell wording that no longer matches the batched control plane
5. Extend existing tests in [`semio/rs/lib.rs`](semio/rs/lib.rs):
   - change-tree lifting and flattening for mixed kit/type/design/piece commands
   - transaction command path emits the same classified event as the GraphQL actor path
   - `changeKitCommands` and `changeKitWithInverse` share apply semantics, with only the result payload differing
   - GraphQL SDL still matches [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql)

## Verification

- Run `cargo fmt` in [`semio/rs`](semio/rs).
- Run `cargo test --lib` in [`semio/rs`](semio/rs).
- If SDL changes, regenerate/check [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) using the existing schema export path and rerun the GraphQL smoke test.
