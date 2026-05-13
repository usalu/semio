# Refactor Lib Rs To Golden Schema — Progress

## Direction (greenfield)

Single code-first `lib.rs` surface: GraphQL identifiers come only from golden SDL; internal runtime uses **non-schema** Rust names (`KitGraphParentWeak`, `KitGraphNavNode`, …). No parallel legacy naming, no `tmp_legacy_*` archives in this ticket (prior `tmp_legacy_lib_full.rs.txt` removed as obsolete).

Macro DSL: `entity_full!` / `entity_lite!` compose `entity_relay!` + `_ladder_relay_full!` where applicable; `entity_bare!` / `operation_with_input!` / `operation_no_input!` are **item splices** (expand to real `item` tokens) so call sites own struct/`#[Object]` bodies until codegen covers the full 985-declaration set.

## Done (latest)

- **EntityConnection empty shell:** Dropped non-golden `EmptyEntityConnection`; `EntityConnectionInterface` now uses golden `PageInfoConnection` (via `PageInfoConnection::empty_entity_shell()`) for empty `owns` projections. Removed `register_output_type` for the old hack.
- **Renames:** `EntityOwnerWeak` → `KitGraphParentWeak`; `GqlNode` → `KitGraphNavNode`; `gql_node_to_node_interface` → `kit_graph_nav_node_to_node_interface`.
- **Merkle / id labels:** Removed `semio:*` product prefixes from hash/id segments in favor of GraphQL-aligned typenames (`Vector`, `Quality`, `weak:…`, `diff:…`, `RelayCollection`, `test:…`, etc.). `resolve_local_semio_root` path helper unchanged (filesystem, not merkle).
- **Dead DSL:** Removed no-op `register_entities!` / `register_operations!` / `register_commands!` blocks and stub `command_*` / `operation_family!` / `relay_collection!` / `entity_interface_enums!` macros. Removed unused `entity_diffs!`.
- **Tests:** Added `golden_macro_dsl_item_splices_compile` proving macro bodies are non-empty splices.
- **Windows:** `cargo test` against default `target/debug` may hit `LNK1104` (locked `.dll`); `CARGO_TARGET_DIR=c:\git\semio\target-agent-semio-rs` avoids contention — documented for agents/CI.

## Verified (this session)

- `cargo check` (semio crate) with alternate `CARGO_TARGET_DIR` — OK.
- `cargo test golden_macro_dsl_item_splices_compile` — OK.
- `SEMIO_GOLDEN_STRICT=1 cargo test schema_matches_target_graphql_file` — **fails**: **815** golden top-level declarations still missing from `gql::sdl()` (unchanged vs prior baseline: removing `EmptyEntityConnection` did not affect golden-key coverage).

## Not done

- Mass registration: interfaces (`Diff`, `Modification`, `Modifications`, `WeakEntity`, …), scalars (`Color`, `Timestamp` if missing), full relay ladders per entity, ~70+ operation GraphQL types — needs generated or exhaustive `register_output_type` wiring per plan.
- `ticket_close` deferred until strict gate passes.

## Next

1. `build.rs` or bundled generator emitting golden stubs + `Schema::build` registration from `.repo/all_type_names.txt` / `schema.golden.graphql` (single source of truth).
2. Wire real resolvers onto generated shells; then `SEMIO_GOLDEN_STRICT=1` in CI.
