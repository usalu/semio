# Strict Read Write Hooks

**Status:** In progress  
**Repo MCP:** Not available in this Cursor workspace — tracking here per AGENTS.md.

## Log

- 2026-05-09: Phase 1 started — Rust SDL uniform `scope`/`input` mutations; JS KitStore primitives.
- 2026-05-09: Rust compile + tests green — `Checkpoint` uses `root` RwLock (SDL + seed + fork); `Graph`/`Draft` `Default` fixed; `KitStoreBundleFile` RwLock kit access; replay `clear_piece` via read; `#[serde(transparent)]` on `Id` so `KitDiff` `__ops` JSON uses string ids (fixes materialized replay + GraphQL worker apply); `CreatedFixedPiecePayload` Serialize + snapshot string before move; `apply_create_fixed_piece` aligned with record_op + materialized path; gql resolver `the_kit` for `theKit`.
