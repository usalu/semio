# Mirror Target Schema In semio/rs

Repo MCP unavailable in this environment; ticket folder tracks work.

## Notes

- Canonical SDL: `include_str!("../graphql/target.schema.graphql")` from `semio/rs/lib.rs`.
- Runtime: `async_graphql::dynamic::Schema` built from parsed target SDL + overlays for Query/Graph/Kit/Design/Piece and connections.
- Pointer rule: no `*_by_id` in GraphQL resolvers; `Weak`/`Arc` for links; command boundary may use id indexes.

## Files

- `semio/rs/lib.rs` — dynamic schema module, pointer sweep, tests
- `semio/rs/Cargo.toml` — `dynamic-schema` feature
- `semio/graphql/schema.graphql` — regenerated to match target (export)
