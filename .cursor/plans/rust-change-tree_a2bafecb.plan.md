---
name: rust-change-tree
overview: Refactor `semio/rs` change tracking from a flat `KitChange { forward, inverse }` command list into an explicit typed change tree covering kit, design, type, and piece scopes while preserving command application semantics.
todos:
 - id: ticket
   content: Open or reopen the appropriate repo ticket before edits, linked to the closest goal.
   status: completed
 - id: types
   content: Add the typed Change tree structs/enums and public exports in semio/rs/lib.rs.
   status: completed
 - id: builders
   content: Implement command-to-change-tree builders with explicit non-1:1 store/change mapping rules.
   status: completed
 - id: apply
   content: Refactor forward/backward application to walk the change tree while delegating to existing command apply logic.
   status: completed
 - id: tests
   content: Extend inline Rust tests for tree shape, cluster-as-kit behavior, and undo restoration.
   status: completed
 - id: verify
   content: Run fmt/check/test for semio/rs and fix any introduced diagnostics.
   status: completed
isProject: false
---

# Rust Change Tree Plan

## Current Shape

- [`semio/rs/lib.rs`](semio/rs/lib.rs) defines `ChangeKitCommand` with kit-level lifecycle/metadata commands and nested command variants like `ChangeTypeCommands`, `ChangeDesignCommands`, and `ChangePieceKind`.
- `ChangeDesignCommand` already contains `ChangePieceCommands`, so command scoping exists, but `KitChange` is still only a flat forward/inverse list plus `KitChangeKind`.
- Some semantic operations such as `ClusterPieces`, `DragPieces`, `MovePieces`, and `FlattenDesign` intentionally yield kit-level snapshot changes, so the new tree must not assume store-to-change 1:1 mapping.

## Implementation Approach

- Work in [`semio/rs/lib.rs`](semio/rs/lib.rs), primarily the existing `change_command` and `kit_change` modules, and extend the existing inline Rust tests only.
- Introduce a typed `Change` enum with variants `Kit(KitChange)`, `Design(DesignChange)`, `Type(TypeChange)`, and `Piece(PieceChange)`.
- Refactor `KitChange` so it owns kit-scoped commands plus nested child changes, rather than being the only change shape. Add `DesignChange`, `TypeChange`, and `PieceChange` structs with ids, forward/inverse command lists, metadata (`kind`, `author`, `time` where appropriate), and children where useful.
- Keep command application behavior centralized: `ChangeKitCommand::apply_mutation`, `ChangeTypeCommand::apply`, `ChangeDesignCommand::apply`, and `ChangePieceCommand::apply` stay the source of mutation/inverse truth. The new change structs should delegate to these rather than duplicating mutation logic.
- Add conversion/build helpers that lift existing command batches into the tree:
  - kit metadata, family, file/folder, and semantic whole-kit operations remain `Change::Kit`.
  - `ChangeTypeCommands` becomes a `Change::Type` child under a kit change.
  - `ChangeDesignCommands` becomes a `Change::Design` child under a kit change.
  - `ChangeDesignCommand::ChangePieceCommands` becomes a `Change::Piece` child under a design change.
  - Commands like `ClusterPieces` stay kit-level even though they affect design stores, matching the user’s example.
- Update `KitChange::apply_forward` and `apply_backward` to walk the tree depth-first in forward order and reverse-depth order for undo, reusing the existing command `apply_many` and nested `apply` logic.
- Update serialization derives and public exports so downstream Rust/wasm callers can use the new typed tree API without legacy aliases.

## Verification

- Extend existing tests in [`semio/rs/lib.rs`](semio/rs/lib.rs) to cover:
  - kit metadata change as a `KitChange`.
  - type command becoming a `TypeChange` child.
  - design command becoming a `DesignChange` child.
  - piece command becoming a `PieceChange` child.
  - `ClusterPieces` or another semantic operation remaining a `KitChange`, not a `DesignChange`.
  - forward then backward application restoring the original kit snapshot.
- Run Rust formatting and tests for `semio/rs` after implementation, then inspect lints for the touched file.
