---
name: rs graphql consolidation
overview: Consolidate the Rust-owned GraphQL schema so it exposes navigable domain entities, DTO tiers, and clean SDL naming from `semio/rs`, then regenerate `semio/graphql/schema.graphql` and update schema guards. The first implementation slice continues the existing open Rust GraphQL cleanup ticket and replaces the older `*Store` schema naming direction with the requested `Kit`/`Design`/`Type` domain names.
todos:
 - id: ticket
   content: Use the existing Rust GraphQL cleanup ticket and update its direction from `*Store` schema names to requested domain entity names.
   status: in_progress
 - id: inventory
   content: Inventory current SDL names and resolver fields that violate docstring, suffix, scalar-reference, DTO-tier, or container rules.
   status: pending
 - id: dto-exposure
   content: Expose DTO tiers as `*Dto` GraphQL objects/lists and attach `metadata`/`shallow`/`full` fields to the main entities.
   status: pending
 - id: navigation
   content: Replace resolvable ID fields with object references and add `container` resolvers from existing weak parent refs.
   status: pending
 - id: schema-tests
   content: Regenerate SDL and extend existing Rust schema smoke tests with cleanup guard assertions.
   status: pending
 - id: consumers
   content: Update direct JS GraphQL operation contracts only where the cleaned SDL changes existing queries.
   status: pending
isProject: false
---

# Rust GraphQL Consolidation

## Scope

Continue the existing open ticket `[.repo/🎫/26/04/27/RUST-GRAPH-QL-STORE-DTO-CLEANUP/ticket.json](.repo/🎫/26/04/27/RUST-GRAPH-QL-STORE-DTO-CLEANUP/ticket.json)` under goal `r26-03`. Primary files are `[semio/rs/lib.rs](semio/rs/lib.rs)`, `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)`, and direct consumers in `[semio/js/index.ts](semio/js/index.ts)` if the regenerated schema changes operation contracts.

## Implementation

- Clean the `kit_graphql` module in `[semio/rs/lib.rs](semio/rs/lib.rs)` without adding parallel in-memory or GraphQL-only domain structs.
- Remove GraphQL schema descriptions by stripping or converting `///` comments on GraphQL-exposed types, inputs, enums, scalars, and resolver methods to non-doc comments where internal notes are still useful.
- Keep main GraphQL entity names domain-shaped: `Kit`, `Design`, `Type`, `Piece`, `Connection`, `Connector`, `Representation`. Rename only exposed SDL names, not the underlying store structs.
- Replace exposed `*Object`, `Gql*`, `*Row`, and opaque list scalar names with existing or new serde DTO names ending in `Dto`, for example `TypeMetadataObject` to `TypeMetadataDto` and `DesignFlattenMapEntryObject` to `DesignFlattenMapEntryDto`.
- Move DTO tiers onto each main entity: add `metadata`, `shallow`, and where useful `full` resolver fields such as `Design.metadata: DesignMetadataDto!` and `Design.shallow: DesignShallowDto!`, using existing `to_*_dto()` methods.
- Replace ID-only references with object references wherever the Rust graph already has a store ref or can resolve one from the current graph context: `ReplaceableCatalog.types: [Type!]!`, `ReplaceableCatalog.designs: [Design!]!`, `Piece.refType: Type`, `Connector.port: Port` where supported, and equivalent fields for location, file, quality, author, concept, tag, prop, attribute as the current store graph permits.
- Add `container` fields from existing weak back-references: `Design.container: Kit!`, `Type.container: Kit!`, `Piece.container: Design!`, `Connection.container: Design!`, `Connector.container: Type!`, `Representation.container: Type!`. Use nullable only where the current in-memory invariant can genuinely be absent during construction.
- Keep scalar transport only where it is still intentionally opaque command/event wire (`ChangeKitCommand`, `KitEvent`, possibly `KitFullSnapshot` for the first slice); remove `TypeShallowList`, `DesignShallowList`, `PieceFullList`, and `ConnectionFullList` by exposing DTO list fields.

## Verification

- Extend the existing `kit_graphql_smoke` tests in `[semio/rs/lib.rs](semio/rs/lib.rs)`; do not add new test files.
- Add schema guard assertions for no SDL docstrings, no `*Object`, no `Gql*`, no `*List` DTO scalars, no `designIds`/`typeIds` where object references exist, and required `container` plus `metadata`/`shallow` fields on core entities.
- Regenerate `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)` through `[semio/graphql/project.json](semio/graphql/project.json)` with `npx nx build semio/graphql`.
- Run focused Rust tests for `kit_graphql_smoke`, then update and run relevant `semio/js` checks if GraphQL operation strings or generated contract assertions break.
