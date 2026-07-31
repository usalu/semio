# Agent verify (cursor) — `compose/rs/lib.rs` macro + build

- `cargo check` (compose/rs): ok
- `cargo test` (compose/rs): 36 passed, 1 ignored; `schema_matches_target_graphql_file` ok after skipping empty SDL fragments in `gql::sdl()`.

## Macro / roster

- `register_entities!` now takes `$ty:ty` paths, implements `HasSdlFragment` (empty `SDL_FRAGMENT` for now), and `push_all_fragments` pushes non-empty-ready fragments.
- Removed `simple_conn_sync!` / `simple_conn_entity!`; added `entity_relay_sync!`; `entity_relay!` holds async relay expansion; `entity_full_family!` delegates to `entity_relay!`.
- `gql_relay`: standard Arc-node connections use `entity_relay!` + thin `from_*` wrappers; sync DTO rows use `entity_relay_sync!`; Blueprint / design `Connection*` / OperationConnection kept hand-written where custom.

## Other fixes required for green build

- `Operation` GraphQL: split duplicate `owner` into `operationOwner` + `owner` (`owner_entity` resolver) on concrete operation types and `OperationInterface` attribute list.
- `gql`: renamed nav struct to `KitOperationInputRoot` + import `KitOperation as KitOpApply` to disambiguate from `operation::KitOperation` enum.
- `gql` command nav: `impl SessionCommandNav` / `BackboneCommandNav` / `VersionCommandNav`; return types aligned (`UnsavedChange`, not `UnsavedChangeNav`); `Mutation::session` returns `SessionCommandNav`.
