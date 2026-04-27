---
name: rust graphql store consolidation
overview: Consolidate the first Rust GraphQL/store cleanup slice by removing redundant GraphQL node wrappers and aligning the exposed API around the existing `*Store` Rust entities. The implementation should continue the open `Rust GraphQL Store Dto Cleanup` ticket and keep changes focused on `semio/rs`, `semio/graphql`, and direct JS/React callers.
todos:
  - id: reopen-ticket
    content: Reopen or attach to the existing `Rust GraphQL Store Dto Cleanup` ticket before implementation.
    status: pending
  - id: collapse-node-layer
    content: Refactor `kit_graphql` wrapper structs and async-graphql object names so exposed objects are consistently `*Store`.
    status: pending
  - id: regenerate-schema
    content: Regenerate `semio/graphql/schema.graphql` and update direct JS/React query/type callers.
    status: pending
  - id: extend-tests
    content: Extend existing Rust tests to assert unified schema names and absence of exposed node/mixed names.
    status: pending
  - id: verify-close
    content: Run focused Rust/JS/React checks, then close the repo ticket with touched files.
    status: pending
isProject: false
---

# Rust GraphQL Store Consolidation

## Scope

- Reopen the existing `Rust GraphQL Store Dto Cleanup` ticket before edits; it is the closest open work item for this request.
- Work primarily in [semio/rs/lib.rs](semio/rs/lib.rs), especially the `kit_graphql` module around `KitStoreNode`, `DesignNode`, `PieceNode`, `TypeNode`, `ConnectionNode`, `ConnectorNode`, and `RepresentationNode`.
- Regenerate and verify [semio/graphql/schema.graphql](semio/graphql/schema.graphql), then update direct callers in [semio/js/index.ts](semio/js/index.ts) and [semio/react/index.tsx](semio/react/index.tsx) only where schema names or query fields change.

## Refactor Approach

- Replace GraphQL wrapper names and public GraphQL object names so the API exposes store objects directly and consistently: `KitStore`, `DesignStore`, `PieceStore`, `TypeStore`, `ConnectionStore`, `ConnectorStore`, `RepresentationStore`.
- Internally collapse the distinct `*Node` wrapper structs into the store layer where practical. If async-graphql requires a lightweight wrapper due orphan rules or trait constraints, make it an internal implementation detail named consistently with `*StoreGraphql` or equivalent, not part of the exposed schema vocabulary.
- Align inconsistent GraphQL object naming: `Connection`, `Connector`, and `Representation` should become `ConnectionStore`, `ConnectorStore`, and `RepresentationStore` like the rest.
- Keep DTO names explicit and separate: `*MetadataDto`, `*ShallowDto`, `*FullDto`, `KitFullSnapshot`, and command input types remain DTO/control-plane shapes for this first pass unless they block schema consistency.

## Validation

- Extend existing Rust GraphQL/schema tests inside [semio/rs/lib.rs](semio/rs/lib.rs), not a new test file, to assert the generated SDL includes the unified store names and no exposed `*Node`/mixed object names.
- Run focused Rust checks for `semio/rs`, regenerate the GraphQL SDL, then run the JS/React type/test checks that cover GraphQL queries.
- Close the ticket through the repo MCP with the final touched file list after verification.
