# Refactor Lib Rs To Golden Schema — Progress

## Direction (greenfield)

Single code-first `lib.rs` surface: GraphQL identifiers come only from golden SDL; internal runtime uses **non-schema** Rust names (`KitGraphParentWeak`, `KitGraphNavNode`, …). No parallel legacy naming.

## Owner constraint (2026-05-13) — **mandatory**

- **Forbidden:** `build.rs` / generated `.rs` from SDL / runtime SDL parsing / `include_str!` → macro string codegen / proc-macros that read SDL at compile time (unless already in tree).
- **Required:** Only **type-safe Rust** in `semio/client/lib/rs/lib.rs` (existing `pub mod` splits in-crate only; **no new crates/files** unless repo rules explicitly allow).
- **Allowed:** `macro_rules!` that expand at **compile time** to explicit `struct` / `enum` / `impl` with `#[derive(SimpleObject, Interface, Union, InputObject, Enum)]` and `#[Object]` — deterministic, reviewable expansions; invoke `entity_full!` / `entity_lite!` / `entity_bare!` **once per golden entity**, not loops over strings.
- **Gate:** `SEMIO_GOLDEN_STRICT=1` + `cargo test -p semio --lib` must pass before `ticket_close`. Windows: `CARGO_TARGET_DIR=c:\git\semio\target-agent-semio-rs` for `cargo test` / `cargo check`.

## Plan phases (`.cursor/plans/refactor_lib.rs_to_golden_schema_a5d816d2.plan.md`)

| Phase | Title | Status |
|-------|--------|--------|
| 0 | Ticket + scaffold | done (umbrella ticket) |
| 1 | Foundation: scalars + macro DSL + general interfaces + ladders | **in progress** (interfaces expanded; `Input`/`Diff`/`Modification` relay shells added; geom `Entity` bridge for weak nodes) |
| 2 | Geom full 12-ladders | **in progress** (Vector diff/mod/mods ladder + registrations; Point…Location ladders still missing) |
| 3 | Meta 12-ladders | pending |
| 4 | Type domain 12-ladders | pending |
| 5 | Design + Clump + Blueprint | pending |
| 6 | Kit 12-ladder + KitDiff/KitModification | pending |
| 7 | ~80 operations + aggregators | pending |
| 8 | VCS lite + Workspace | pending |
| 9 | Store / backbone / provider commands | partial (relay edges; runtime wiring evolving) |
| 10 | Query / Mutation / Subscription | partial |
| 11 | Runtime rewire | pending |
| 12 | Strict gate + nx + ticket_close | **blocked** until golden gap **0** |

## Done (latest)

- **Interface relay shells + registrations:** `InputEdge`/`InputConnection`, `DiffEdge`/`DiffConnection`, `ModificationEdge`/`ModificationConnection`, `Modifications` (+ edge/connection), `BackboneCommand*` relay pair, `Provider*` / `ProviderCommand*` relay pairs; all registered in `build_schema_sync_for` plus `gql_relay::{Vector,Point,…,Location}Connection`.
- **Vector golden ladder:** `VectorDiff` (+ `GqlDiffInterface::VectorDiff`), `VectorModification` (+ `GqlModificationInterface::VectorModification`), `VectorModifications` + `entity_relay!` edges/connections; `EntityInterface` extended with weak geom variants so modification `before`/`after` resolve to golden `Entity!`.
- **Golden interface SDL + registration:** `build_schema_sync_for` now registers `StrongEntityInterface`, `RichStrongEntityInterface`, `ArtifactInterface`, `DocumentInterface`, `EventInterface`, `GqlDiffInterface`, `GqlModificationInterface`, `PubInputInterface`, stub `Empty*` / `GqlEmptyDiff`, `BackboneCommandInterface`, `FileBackboneCommand`, `WebsocketBackboneCommand`, `ProviderInterface`, `ProviderCommandInterface`.
- **`BackboneCommand` collision:** Removed duplicate `BackboneCommand` `Object`; golden `interface BackboneCommand` is `BackboneCommandInterface` only. `StoreCommand.backbone` returns `BackboneCommandInterface::File(…)`; `FileBackboneCommand`/`WebsocketBackboneCommand` `detach`/`sync` call `ParentStore::dispatch_wip` (empty `connection_uri` on detach until golden exposes args).
- **`ProviderCommand`:** Interface derive uses `method = "create_backbone"` / `attach_backbone`; resolvers return `Id` (not `Result`) to match declared field types.
- **EntityConnection empty shell:** Dropped non-golden `EmptyEntityConnection`; `EntityConnectionInterface` now uses golden `PageInfoConnection` (via `PageInfoConnection::empty_entity_shell()`) for empty `owns` projections.
- **Renames:** `EntityOwnerWeak` → `KitGraphParentWeak`; `GqlNode` → `KitGraphNavNode`; `gql_node_to_node_interface` → `kit_graph_nav_node_to_node_interface`.
- **Merkle / id labels:** GraphQL-aligned typenames in hash/id segments (`Vector`, `Quality`, `weak:…`, …).
- **Dead DSL:** Removed no-op registration macros / stub macros per prior cleanup notes in git history.
- **Tests:** `golden_macro_dsl_item_splices_compile` proves macro bodies are non-empty splices.

## Verified (strict gate)

- `SEMIO_GOLDEN_STRICT=1 cargo test schema_matches_target_graphql_file -p semio --lib` — **fails** (**766** missing top-level declarations after interface relay + Vector ladder + geom `EntityInterface` bridge batch, 2026-05-13 late).
- `cargo test -p semio --lib` (strict env **unset**) — **passes** (37 passed, 1 ignored).

## Next

1. Add missing golden **interfaces** / **scalars** / **enums** as explicit Rust + `register_output_type` (e.g. `WeakEntity`, `Color`, `VersionKind`, then `StrongEntity`, `RichStrongEntity`, `Artifact`, …).
2. Register **InputObject** shapes with `SchemaBuilder::register_input_type` (async-graphql 7.2) so unused inputs (e.g. `LocationInput`) still appear in `gql::sdl()`.
3. Emit operation **output** types + lite relay shells (`*Edge` / `*Connection`) via compile-time macros + one invocation per operation family until coverage is zero.
