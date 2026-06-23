# Strict Read Write Hooks

**Status:** In progress  
**Repo MCP:** Not available in this Cursor workspace — tracking here per AGENTS.md.

## Log

- 2026-05-09: Phase 1 started — Rust SDL uniform `scope`/`input` mutations; JS KitStore primitives.
- 2026-05-09: Rust compile + tests green — `Checkpoint` uses `root` RwLock (SDL + seed + fork); `Graph`/`Draft` `Default` fixed; `KitStoreBundleFile` RwLock kit access; replay `clear_piece` via read; `#[serde(transparent)]` on `Id` so `KitDiff` `__ops` JSON uses string ids (fixes materialized replay + GraphQL worker apply); `CreatedFixedPiecePayload` Serialize + snapshot string before move; `apply_create_fixed_piece` aligned with record_op + materialized path; gql resolver `the_kit` for `theKit`.
- 2026-05-09 (session): **Phase 1 JS/React partial** — `StoreField` read-only optional `source`; `WRITE_STATUS_*`; `RequestCorrelator` + `commandSucceeded` / `operationFailed` subscriptions; `invalidations` Subject + `query()` for `kitName`; `operation()` + uniform `renameKit(scope,input)`; removed `OperationRouter` / `kitRenamed` / `seedFieldsFromDto`; `operationSucceeded` fanout + `invalidations.next()`; `KitStore` ctor takes `transport`; `compose/graphql/schema.graphql` `renameKit` + input types aligned. **`useRenameKit`** still exposes `(name: string) => …` via adapter. **Rust:** fixed three invalid `unwrap_or_else(|| self.workspace_kit_id().await)` closures in `lib.rs` (tags/concepts/qualities collection apply). **Blocker:** `compose/rs` currently does not compile on this machine (`cargo test -p compose --lib` / wasm pretest / `nx run compose/graphql:build` fail with many unrelated errors, e.g. `ComplexObject` in `derive`, relay `.await`, `Checkpoint.root` vs `frozen_root`, etc.) — full `nx` schema regen and `compose/js` vitest blocked until Rust is green again.

## Commands attempted

- `npx nx run compose/graphql:build` — **failed** (Rust compile errors in `compose/rs/lib.rs`).
- `cargo test -p compose --lib` (via nx script cwd) — **failed** (same).
- `npx tsc --noEmit` in `compose/js` — **ok**.
- `npx tsc --noEmit` in `compose/react` — **ok** (after `RenameKitCommandArgs` type fix).

## Files touched

- `compose/js/index.ts`
- `compose/react/index.tsx`
- `compose/graphql/schema.graphql` (hand-aligned `renameKit` + `RenameKitScopeInput` / `RenameKitInput`)
- `compose/rs/lib.rs` (async `owner_id` defaulting in three collection-diff apply loops)
