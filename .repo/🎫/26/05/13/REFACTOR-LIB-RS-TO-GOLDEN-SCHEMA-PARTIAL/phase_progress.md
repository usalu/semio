# Refactor Lib Rs To Golden Schema — Progress (session 2026-05-13)

## Done this session

- **Compile:** Fixed `EntityConnectionInterface` (`#[derive(Interface)]`): mapped GraphQL `pageInfo` via `method = "page_info"`, unified `EmptyEntityConnection.page_info` as `Arc<PageInfo>`, set interface field `ty = "&Arc<PageInfo>"` so async-graphql derive accepts both variants with `StoreConnection`.
- **SDL hygiene:** Removed literal `#region` from two `///` docstrings that leaked into generated SDL (tripped `schema_matches_target_graphql_file` guard).
- **Tests vs golden commands:** Updated GraphQL test mutations to the golden chain `Mutation.session → store(id) → theKit → unsavedChange → kit → …` (was incorrectly calling `theKit` on `SessionCommand`). Fixed `create_alternative` to use `store.startAlternative`. Response JSON paths updated.
- **Fixtures:** Corrected `metabolism.new.kit.semio.json` path to `../../../assets/fixtures/…`. Added `kit_store_golden_fixture_paths()` resolving `../../../assets/semio/kit-store.golden.*.json` with skip + `[DEBUG]` when absent (fixtures present in full tree → tests run).
- **Archive:** Full `lib.rs` copied to `tmp_legacy_lib_full.rs.txt` in this ticket folder for porting reference.

## Verified

- `cargo test -p semio --lib` (36 passed, 1 ignored) with **no** `SEMIO_GOLDEN_STRICT` in environment.

## Not done (plan phases 1–12)

- **Strict gate:** With `SEMIO_GOLDEN_STRICT=1`, **817** top-level golden declarations still missing from `gql::sdl()` (interfaces, scalars, full relay ladders, operations, etc.). Needs macro DSL + mass `register_output_type` / reachability per plan.
- **Renames / cleanup:** `ParentStore` → non-schema runtime name, remove `EntityOwnerWeak` / `GqlNode` per plan, align `OperationInput` GraphQL object name with golden `KitOperation`, rename `*OperationInput` types to golden `*Operation` where applicable.
- **Feature `SEMIO_GOLDEN_STRICT`:** Leave env-gated until SDL covers golden set; optional Cargo feature wiring per plan.

## Suggested next steps

1. Implement `_ladder_relay_full!` / `_ladder_relay_lite!` + public `entity_full!` / `entity_lite!` / `entity_bare!` and operation macros; migrate one vertical slice (e.g. geom) end-to-end including `SchemaBuilder::register_output_type`.
2. Script or derive `register_output_type` list from `.repo/all_type_names.txt` until `collect_schema_decl_keys` diff is empty under strict.
3. Flip `SEMIO_GOLDEN_STRICT=1` in CI only after export matches `schema.golden.graphql`.

---

## Continuation (subagent session, same ticket)

### Done

- **Macro ladder (scaffold):** Added `_ladder_relay_lite!`, `_ladder_relay_full!`, `entity_full!`, `entity_lite!`, `entity_bare!`, `operation_with_input!`, `operation_no_input!` in `lib.rs` `//#region 🧬 entity_dsl` (per plan naming); `entity_full!` composes `entity_relay!` + optional `_ladder_relay_full!` when `ladder_full = (...)` is passed.
- **SDL reachability:** Implemented golden `PageInfoEdge` / `PageInfoConnection` in `gql_relay` and registered both in `Schema::build` (`build_schema_sync_for`).
- **Repo MCP `ticket_close`:** Tool schema requires `summary` (required); `files` is an optional **JSON array of path strings** (`{ "type": "array", "items": { "type": "string" } }`).

### Verified / blocked

- **Windows linker:** `cargo test -p semio` failed locally with `LNK1104` (cannot open `paste-*.dll` in `target/debug/deps`) — likely AV/IDE file lock; **not** re-run successfully in this environment. Prior agent reported 36 tests green without `SEMIO_GOLDEN_STRICT`.
- **Strict count:** Not re-measured here; expect **815** missing declarations after +2 registered types (817 − 2), until `Diff` / `Modification` / entity diff families and ~70 operations are added.

### Next

- Emit `interface Diff` / `Modification` + 30× concrete diff/modification/modifications types (or macro-generated), then wire `register_output_type` from `.repo/all_type_names.txt`.
