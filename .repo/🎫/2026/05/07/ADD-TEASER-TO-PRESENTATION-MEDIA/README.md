# Static Gql Schema Semio Rs

## Done

- Replaced `pub mod gql` dynamic imports with `Schema::build(Query, Mutation, Subscription)`; `AppSchema` is the static triple.
- `target_schema_sdl` / `sdl` use `include_str!("../graphql/target.schema.graphql")`.
- `build_schema_for` is synchronous; WASM `KitStoreHandle` updated accordingly.
- `Subscription::commandSucceeded` streams `CommandReceipt` from `EventBus`.
- `simple_conn!` hash no longer borrows temporaries from `id_fn`.
- Added `ConnectionConnection`, `LayerConnection`, `GroupConnection` in `gql_relay`.
- `Kit` / `Design` list fields aligned to Relay connection types; `KitOwner`, `DesignOwner` unions.

## Files

- `semio/rs/lib.rs`

## Validation

- `cargo test` (native): 11 passed, 1 ignored.
- `cargo check --target wasm32-unknown-unknown`: ok.
