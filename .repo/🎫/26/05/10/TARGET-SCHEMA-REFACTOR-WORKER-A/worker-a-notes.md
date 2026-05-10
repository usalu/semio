# Worker A summary

- **geom**: Added `LocationInput`, `LocationNode` (lon/lat/alt + merkle hash), `Design.location` now `Arc<LocationNode>`; removed obsolete `meta::Location` stub.
- **gql_relay**: Introduced `entity_full_family!` (paste → `simple_conn_entity!`) for geometry relay names matching target SDL (`VectorEdge`/`VectorConnection`, … `LocationEdge`/`LocationConnection`). Legacy `entity_relay!` retained as alias.
- **gql::interfaces**: Registered `NodeIface`, `EntityEdgeIface` (+ `register_output_type`). `EntityConnection` interface omitted until async-graphql field/`Arc`/`page_info` resolver wiring is aligned (derive hit `From<&Arc<PageInfo>>`).
- **iface**: `OwnerEntity::Location`, `#[Object]` for `LocationNode`.
- **Misc compile fixes touched outside strict regions**: `Type::compute_hash`, `entity_description` for `Type`, `#[derive(Debug)]` on geometry nodes / `PlaceNode` / `Port`, `Kit` OneOf inputs requiring `Debug` on connections.

**Tests**: `cargo check` OK. `cargo test --lib` has 10 failures from mutation/schema drift (`transactionOpen`, `kitStoreInitializeDefaults`, …) — Worker D / gql root scope.

**MCP**: `repo://goals` and `ticket_open` were not available in this environment; ticket folder created manually at `2026/05/10/TARGET-SCHEMA-REFACTOR-WORKER-A`.

**Follow-up**: Expand `entity_full_family!` with Diff/Modification/Modifications 9 remaining types per entity; add `Entity`/`WeakEntity`/`EntityConnection` interfaces with proper `owner`/`owns` resolvers on `#[Object]` impls; register `Operation` next to existing `OperationIface` if SDL merge requires.
