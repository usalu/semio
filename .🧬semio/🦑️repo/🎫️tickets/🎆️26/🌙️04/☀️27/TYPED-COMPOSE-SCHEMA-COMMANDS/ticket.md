# Ticket: Typed Compose Schema Commands

**Goal:** `🎯️r2602/running-sketchpad`

## Summary

Replace generic patch/update and field-level `ChangeKitCommand`/`ChangePieceCommand`/`ChangeConnectionCommand` mutations with typed semantic commands, GraphQL inputs/outputs/events, and JS store APIs. MCP `repo` server unavailable in agent environment; ticket opened manually per repo convention.

## Status

**closed** — partial delivery (2026-04-27)

### Done

- Removed generic field patch / RPC surface from `compose/rs`: `find_design_id_for_connection`, `piece_diff_to_change_commands`, `connection_diff_to_change_commands`, `change_kit_commands_for_field_patch`, `set_field_rpc`, `get_field_rpc` (~577 lines in `compose/rs/lib.rs`).
- Resolved accidental duplicate `ChangeDesignCommand` variants and orphan delegator match arms introduced during refactor attempts; restored canonical `ChangePieceCommands` / `ChangeConnectionCommands` shapes.
- Regenerated `compose/graphql/schema.graphql` with `npx nx build compose/graphql` so SDL matches Rust (`cargo test generated_schema_sdl_matches` passes).

### Not completed (follow-up)

- Replace GraphQL `scalar ChangeKitCommand` / `ChangeKitCommandsInput` with typed `@oneOf` mutation steps (`RenamePieceCommandRequest`, …), typed inverse rows, and `RenamedPieceEvent` (or equivalent) on the event bus.
- Refactor `compose/js` / `compose/react`: remove `patchField`, `piecePatchToChangeCommands`, generic `submitChangeKitCommands` JSON batches; route through semantic store methods only.
- Optional domain flattening: replace nested `ChangePieceCommand` / `ChangeConnectionCommand` enums with semantic `ChangeDesignCommand` variants end-to-end (GraphQL naming must use `kind` not `type` where applicable).

### Verification

- `cargo check -p compose` (pass)
- `cargo test generated_schema_sdl_matches` after `nx build compose/graphql` (pass)

### Files touched

- `compose/rs/lib.rs`
- `compose/graphql/schema.graphql` (regenerated)
