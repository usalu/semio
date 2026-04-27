---
name: Remove Rust DTOs
overview: Remove the Rust/GraphQL FullDto, MetadataDto, ShallowDto, and IdDto implementation tiers while preserving semio/js client DTO/query naming where needed.
todos:
 - id: ticket
   content: Use the existing open Rust GraphQL Store Dto Cleanup ticket for the work.
   status: completed
 - id: rename-rust
   content: Remove or rename Rust DTO tier structs, conversion helpers, command payloads, and GraphQL resolver return types.
   status: in_progress
 - id: regen-schema
   content: Regenerate GraphQL SDL and update schema hygiene assertions to forbid the removed suffixes.
   status: pending
 - id: js-boundary
   content: Preserve semio/js DTO/query layer, adjusting only query aliases/mappings if schema names changed.
   status: pending
 - id: verify
   content: Run focused Rust/GraphQL and affected JS checks, then grep Rust and GraphQL for removed suffixes.
   status: pending
isProject: false
---

# Remove Rust DTO Tiers

## Scope

- Work under the existing open ticket `Rust GraphQL Store Dto Cleanup` because it directly covers the GraphQL/store DTO cleanup surface.
- Change Rust as the source of truth in [semio/rs/lib.rs](semio/rs/lib.rs); regenerate [semio/graphql/schema.graphql](semio/graphql/schema.graphql) from Rust with the existing `npx nx build semio/graphql` path.
- Do not remove the semio/js client-side DTO/query names in [semio/js/index.ts](semio/js/index.ts). Only adjust JS query strings or mappings if the generated GraphQL schema name changes require it.

## Implementation Plan

- Replace `*FullDto`, `*MetadataDto`, `*ShallowDto`, and `*IdDto` Rust structs with non-DTO domain/API names:
  - Full entity payloads become the direct entity GraphQL/API names where they are still needed as serializable snapshots.
  - Metadata/shallow projections become non-DTO projection names, or are collapsed into existing store object fields when they are redundant.
  - ID-only wrappers are removed in favor of direct `Id`/`String` fields and command inputs.
- Update all conversion helpers in `semio/rs/lib.rs`:
  - `to_full_dto`, `to_metadata_dto`, `to_shallow_dto`, `to_id_dto` and `from_full_dto`/`from_shallow_dto` become non-DTO equivalents or direct entity snapshot methods.
  - Command payloads such as `ReadKitFullCommand`, `ReadKitMetadataCommand`, and ID command outputs use the renamed/collapsed shapes.
  - Diff, change, materialization, checkpoint, store, and GraphQL wrapper code is updated to the new names without compatibility aliases.
- Update GraphQL resolvers and schema hygiene tests in `semio/rs/lib.rs` so the generated SDL no longer contains `FullDto`, `MetadataDto`, `ShallowDto`, or `IdDto` type names.
- Regenerate `semio/graphql/schema.graphql` and `semio/graphql/local.schema.graphql` if it is also generated from or expected to mirror the same Rust SDL.
- Preserve `semio/js/index.ts` DTO terminology as the client query layer. If needed, use GraphQL aliases or local TypeScript types so JS can keep names like `KitFullDto` while querying the renamed schema fields.

## Validation

- Run focused Rust checks/tests around schema generation and DTO cleanup:
  - `cargo test --manifest-path semio/rs/Cargo.toml generated_schema_sdl_matches_semio_graphql_schema`
  - `npx nx build semio/graphql`
- Run the relevant JS/React boundary checks if JS query mappings are touched.
- Search `semio/rs` and `semio/graphql` for `FullDto|MetdataDto|MetadataDto|ShallowDto|IdDto` and require zero matches there, while allowing matches in `semio/js`.
