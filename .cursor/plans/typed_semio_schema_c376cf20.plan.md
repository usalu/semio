---
name: Typed Semio Schema
overview: Replace generic patch/update and field-level mutation paths with typed semantic command requests, starting from semio/rs as the authority and flowing through GraphQL, semio/js stores, and existing tests. The work will complete the command schema around concrete request/response/event types such as RenamePieceCommandRequest, RenamePieceCommandResponse, and RenamedPieceEvent while keeping mutations inside existing stores.
todos:
 - id: ticket
   content: Open or reopen the closest repo ticket for typed semio schema commands before editing.
   status: completed
 - id: rust-contracts
   content: Replace Rust generic command/patch GraphQL inputs with typed command request, response, and event definitions.
   status: in_progress
 - id: rust-store
   content: Implement typed request handlers in existing Rust stores and remove generic field patch mutation helpers.
   status: pending
 - id: schema
   content: Regenerate and validate semio GraphQL schema with typed write commands only.
   status: pending
 - id: js-react
   content: Remove JS/React patch/update APIs and route consumers through typed semantic store methods.
   status: pending
 - id: tests
   content: Extend existing Rust/JS/React tests and run focused validation commands.
   status: pending
isProject: false
---

# Typed Semio Schema Commands

## Scope

- Attach work to the existing open control-plane/schema ticket if it still covers this (`Rust Control Plane Refactor` or the closest current open schema ticket), otherwise open a new repo ticket under `r26-02` / Running Sketchpad before editing.
- Primary files: [semio/rs/lib.rs](semio/rs/lib.rs), [semio/graphql/schema.graphql](semio/graphql/schema.graphql), [semio/js/index.ts](semio/js/index.ts), [semio/react/index.tsx](semio/react/index.tsx), and existing relevant tests embedded in those files.

## Current Problem

- Rust still exposes `GqlChangeKitCommand` as a scalar around `ChangeKitCommand`, which keeps a JSON-like command escape hatch in GraphQL.
- Rust still has generic field mutation helpers such as `piece_diff_to_change_commands`, `connection_diff_to_change_commands`, `change_kit_commands_for_field_patch`, and `set_field_rpc`.
- JS still exposes patch/update helpers (`PieceFieldPatchInput`, `piecePatchToChangeCommands`, `kitStoreClientUpdatePiece`, `patchField`, etc.) that let UI code express field surgery instead of semantic command intent.

## Implementation Plan

- Define typed command contracts in Rust for semantic operations:
  - `RenamePieceCommand`, `RenamePieceCommandRequest`, `RenamePieceCommandResponse`, `RenamedPieceEvent` as the first concrete pattern.
  - Follow the same naming pattern for the existing semantic design operations already present in GraphQL (`dragPieces`, `movePieces`, `fixPieces`, `createFixedPiece`, etc.).
  - Add typed event payload variants to `KitEvent` instead of opaque/scalar event rows.
- Replace GraphQL scalar command entry points:
  - Remove `GqlChangeKitCommand` from the external mutation schema.
  - Replace `ChangeKitCommandsInput` with `OneofObject` typed command request inputs.
  - Keep the existing actor queue and store mutation path, but feed it from typed request handlers that produce concrete `ChangeKitCommand`/diff fragments internally.
- Remove field-level mutation APIs:
  - Delete Rust patch helpers and `set_field_rpc`/`get_field_rpc` style mutation endpoints where they mutate fields generically.
  - Remove JS patch input builders and `patchField` methods, replacing each public store method with semantic methods (`renamePiece`, `setPieceColor` only if this is considered a real semantic command, `movePieces`, `fixPieces`, etc.).
  - Update React hooks to call typed JS store methods only, with no generic field key/value write path.
- Complete schema generation:
  - Regenerate [semio/graphql/schema.graphql](semio/graphql/schema.graphql) from Rust after typed inputs/outputs/events are in place.
  - Verify no `ChangeKitCommand` scalar, generic patch input, or JSON-like field mutation remains in the public GraphQL write surface.
- Update tests in existing files only:
  - Add Rust tests for rename piece request id lifecycle, result event, graph mutation, and inverse/undo behavior.
  - Update JS tests around command submission to assert typed GraphQL payloads and event correlation.
  - Update React/sketchpad-facing tests only where existing helpers were calling patch/update.

## Verification

- Run focused Rust tests for `semio/rs` after the Rust schema/store changes.
- Run JS/React type checks or test slices that cover `semio/js` and `semio/react` command calls.
- Regenerate and inspect GraphQL schema for absence of generic write scalars/patch inputs.
- Close the ticket with touched files and verification summary.
