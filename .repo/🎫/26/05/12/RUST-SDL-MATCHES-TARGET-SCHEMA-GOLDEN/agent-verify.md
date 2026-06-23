# Verify (2026-05-12)

- `cargo test -p compose --lib` (CARGO_TARGET_DIR=c:\\git\\compose\\.tmp-target-rust-sdl): **36 passed**, 1 ignored.
- `cargo check -p compose --target wasm32-unknown-unknown`: **ok**.

## Changes

- `crate::id::Id` GraphQL scalar renamed to wire name **`ID`** via `#[Scalar(name = "ID")]` so executable schema + clients match `target.schema.graphql` / Relay (`ID!`).
- Docstrings on `Id`, `gql::sdl()`, and `schema_matches_target_graphql_file` clarify the macro-driven plan: non-empty `SDL_FRAGMENT` prefixes precede the embedded golden until W0 fragments subsume the file.

## Note (plans e6121b3c / 51ba1616)

- Byte-for-byte equality of `Schema::sdl()` (async_graphql) vs hand-authored `target.schema.graphql` is **not** asserted: the golden is schema-first (regions, interfaces, comments); the executable surface is code-first. Full convergence is tracked by plan todos (`rust-sdl-roundtrip`, `integrate`). `schema_matches_target_graphql_file` remains the contract for `gql::sdl()` vs disk.
