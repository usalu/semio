---
name: semantic-kit-commands
overview: Replace kit-modifying diff paths with semantic command execution across Rust, JS, React, and Sketchpad, using the existing Semio JS thin-client refactor ticket as the work container.
todos:
 - id: rust-command-only
   content: Refactor `semio/rs/lib.rs` so semantic commands mutate the kit directly and no mutation path applies `KitDiff` or `DesignDiff`.
   status: completed
 - id: rust-wire-tests
   content: Remove raw diff wire mutations and update Rust tests around semantic command application, inverse commands, and undo/redo.
   status: completed
 - id: js-client-contract
   content: Remove diff mutation methods from `KitStoreClient` and route `semio.kit.*` command execution through semantic store commands.
   status: in_progress
 - id: react-downstream
   content: Replace React piece/connection diff update hooks with semantic command or field patch calls.
   status: completed
 - id: sketchpad-downstream
   content: Replace sketchpad kit-diff edit/event/store flows with semantic command steps and snapshot refreshes.
   status: pending
 - id: verify-close-ticket
   content: Run relevant Rust/JS/React/Sketchpad checks, fix failures, and close the existing ticket with touched files and summary.
   status: pending
isProject: false
---

# Semantic Kit Commands Only

## Objective

All kit modifications must flow through semantic commands. `KitDiff` / `DesignDiff` can remain only for non-mutating comparison, diagnostics, or legacy test fixtures that are deleted during this refactor; they must not be accepted by public mutation APIs or used internally to apply kit changes.

Relevant existing paths:

- [semio/rs/lib.rs](semio/rs/lib.rs): `ChangeKitCommand`, `ChangeDesignCommand`, `KitStoreCommand`, `KitGraph::apply_kit_diff`, `KitGraph::apply_design_diff`, GraphQL `apply_kit_diff` / `apply_design_diff`.
- [semio/js/index.ts](semio/js/index.ts): `KitStoreClient.applyKitDiff`, `KitStoreClient.applyDesignDiff`, `executeSemioKitCommand`, `semioKitCommandHandlers`, JS stores and embedded tests.
- [semio/react/index.tsx](semio/react/index.tsx): `useUpdatePiece*` / `useUpdateConnection*` currently construct design diffs.
- [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx): `KitDiffAppStore`, `KitDiffAppEdit`, `KitMachineEvent.CHANGE`, local `applyKitDiff` projections, and `executeSemioKitCommand` usage.

## Implementation Plan

1. Reuse the existing open ticket `.repo/🎫/26/04/25/SEMIO-JS-THIN-CLIENT-REFACTOR/ticket.json` and close it when implementation and verification are complete.

2. Refactor Rust mutation internals in [semio/rs/lib.rs](semio/rs/lib.rs):

- Change `ChangeKitCommand::apply` / `apply_many` to mutate and return inverse semantic commands only, not `KitDiff`.
- Delete or demote `apply_many_kit_diff` and tests that assert command output equals `KitDiff::between`.
- Replace `ChangeDesignCommand` branches that construct `DesignDiff { ... }` with direct semantic graph mutation helpers for add/remove/update piece and connection.
- Replace higher-level operations that build `DesignDiff` only to call `apply_design_diff` with semantic `ChangeKitCommand` / `ChangeDesignCommand` batches or direct command handlers.
- Remove public mutation entry points `KitGraph::apply_kit_diff`, `KitGraph::apply_design_diff`, async wrappers, and GraphQL/WASM methods that accept raw diff JSON.

3. Make the Rust wire layer command-only:

- Keep `change_kit_with_inverse`, `TransactionCommand::ChangeKitCommands`, field patch command compilation, and named operation methods.
- Ensure undo/redo/checkpoint code stores and replays semantic commands only.
- Update GraphQL schema exposure/generated resolver surface so there is no `applyKitDiff` or `applyDesignDiff` mutation path.

4. Refactor [semio/js/index.ts](semio/js/index.ts) into the command-only downstream contract:

- Remove `KitStoreClient.applyKitDiff` and `KitStoreClient.applyDesignDiff` from the interface and both worker/fallback implementations.
- Replace JS `semio.kit.*` handlers returning `{ diff }` with handlers that call `client.execute(...)`, `setField`, `addChild`, `removeChild`, or named client methods.
- Delete `expandSemanticCommandToDiff`, `applyKitDiff` mutation usage, JS-side stores that mutate via `KitDiff`, and embedded tests whose unit is diff application rather than command behavior.
- Add/extend embedded tests around command execution, inverse command behavior, field patch compilation, and representative `semio.kit.*` commands.

5. Refactor [semio/react/index.tsx](semio/react/index.tsx):

- Replace `useUpdatePiece`, `useUpdatePieces`, `useUpdateConnection`, and `useUpdateConnections` so they emit semantic field/child commands instead of `applyDesignDiff` envelopes.
- Remove re-exports of `applyKitDiff`, `inverseKitDiff`, and raw `KitDiff` mutation helpers unless a remaining read-only kind alias is genuinely needed by UI state.
- Keep existing entity update hooks that already use `setField`, and align piece/connection updates with that pattern.

6. Refactor [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx):

- Rename and rework `KitDiffAppStore` / related edit/result/event kinds to store semantic kit command steps rather than `KitDiff`.
- Replace local `applyKitDiff` kit projections with either live snapshot refreshes from `KitStoreClient` or command execution followed by `getSnapshot`.
- Convert ad hoc quality/type/design app kit diffs into semantic commands executed through the kit store.
- Keep sketchpad-only app diffs separate from kit graph mutations.

7. Verification:

- Run Rust tests for `semio/rs`.
- Run `npm test` / build checks in `semio/js`.
- Run `npm test` in `semio/react`.
- Run the sketchpad embedded test command from `semio/sketchpad/package.json` if the local environment can run Playwright; otherwise report the exact blocker.
- Search the repo for remaining kit-modifying `applyKitDiff`, `applyDesignDiff`, `apply_kit_diff`, `apply_design_diff`, and `DesignDiff {` usages and verify each survivor is non-mutating or removed.
