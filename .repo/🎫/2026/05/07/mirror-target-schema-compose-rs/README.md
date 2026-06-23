# Mirror Target Schema In compose/rs

Repo MCP unavailable in this environment; ticket folder tracks work.

## Notes

- Canonical SDL: `include_str!("../graphql/target.schema.graphql")` in `compose/rs/gql_target.rs` (`TARGET_GRAPHQL_SDL`).
- Runtime: `async_graphql::dynamic::Schema` built from parsed target SDL + overlays for Query/Graph/Kit/Design/Piece and connections.
- Pointer rule: GraphQL `#[Object]` resolvers avoid `*_by_id` helpers; relationships use `Weak`/`Arc` and write-side `*_weak_by_id` maps where external `Id` is accepted at the boundary.
- See `diff-report.md` for schema hash parity and Phase 2 / 3 notes.

## Files

- `compose/rs/lib.rs` — dynamic schema module, pointer sweep, tests
- `compose/rs/Cargo.toml` — `dynamic-schema` feature
- `compose/graphql/schema.graphql` — regenerated to match target (export)
