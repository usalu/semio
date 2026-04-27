---
name: rs graphql cleanup
overview: Consolidate the Rust GraphQL surface so it exposes store objects and DTO tiers consistently, removes ad hoc Row/List/Gql naming, and regenerates the checked-in SDL from `semio/rs` without adding duplicate in-memory GraphQL structs.
todos:
 - id: ticket
   content: Attach the implementation to the existing relevant repo ticket before editing.
   status: in_progress
 - id: inventory
   content: Inventory every exposed GraphQL type/scalar in `kit_graphql` and classify it as Store, DTO, enum, or removable wrapper.
   status: pending
 - id: stores
   content: Rename exposed GraphQL entity object names to `*Store` and remove `Gql`/`Node`/bare entity schema names.
   status: pending
 - id: dtos
   content: Replace `*List` scalars and `*Row` objects with explicit DTO list/object outputs.
   status: pending
 - id: enums
   content: Replace finite string fields with concrete GraphQL enums.
   status: pending
 - id: schema
   content: Regenerate `semio/graphql/schema.graphql` from Rust and add schema-name guard assertions.
   status: pending
 - id: verify
   content: Run formatting, Rust GraphQL tests, and downstream checks required by changed schema consumers.
   status: pending
isProject: false
---

# Rust GraphQL Store/DTO Cleanup

## Scope

Focus on `[semio/rs/lib.rs](semio/rs/lib.rs)` and the generated SDL at `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)`. The work should stay inside the existing inline Rust module layout and extend the existing `kit_graphql_smoke` tests rather than adding new test files.

## Implementation Approach

- Reuse the closest open ticket for this surface, likely `SEMIO-JS-EXACT-GRAPH-QL-AND-WIRE-TYPING`, before editing. It already covers replacing GraphQL scalars with concrete SDL and wire typing cleanup.
- Refactor `kit_graphql` around the existing domain stores instead of adding parallel GraphQL structs:
  - Keep wrappers only where Rust orphan rules require them, but name them as implementation wrappers, not exposed schema concepts.
  - Expose entity GraphQL object names as `KitStore`, `DesignStore`, `PieceStore`, `TypeStore`, `ConnectionStore`, `ConnectorStore`, and `RepresentationStore`.
  - Remove exposed `Piece`, `Design`, `Type`, `Connection`, `Connector`, `Representation` entity names.
- Replace ad hoc GraphQL-only value names:
  - Rename `GqlPlaneObject`, `GqlPoint3`, `GqlVector3`, `GqlCoordinate3`, `GqlPoseObject`, and `GqlIdOnly` to schema names without `Gql` such as `PlaneStore`, `PointStore`, `VectorStore`, `CoordinateStore`, `PoseStore`, and `IdStore` if they represent GraphQL objects, or make them proper DTO outputs if they are wire DTOs.
  - Rename `Vcs*Gql` to `*Store` or domain-aligned DTO names, depending on whether they are live graph objects or serialized snapshots.
- Remove GraphQL scalar list wrappers:
  - Replace `TypeShallowList`, `DesignShallowList`, `PieceFullList`, and `ConnectionFullList` scalar fields with explicit list fields like `[TypeShallowDto!]!`, `[DesignShallowDto!]!`, `[PieceFullDto!]!`, and `[ConnectionFullDto!]!`.
  - Keep `ChangeKitCommand`, `KitEvent`, and `KitFullSnapshot` only temporarily if they still require serde-backed opaque transport, then audit whether they should become explicit input/output objects in the same pass or a follow-up slice.
- Normalize DTO exposure:
  - GraphQL should expose only `*FullDto`, `*ShallowDto`, `*MetadataDto`, and `*IdDto` for serialized DTO tiers.
  - Remove all exposed `*Row` types; rename current row-shaped outputs to one of the DTO tiers or fold them into store fields as concrete store/DTO lists.
  - Convert local GraphQL metadata objects (`TypeMetadataObject`, `DesignMetadataObject`, `KitMetadataObject`) to `*MetadataDto` schema types where they are serialized DTOs.
- Introduce proper enums instead of strings where semantics are finite:
  - Convert `BackboneBatchStatus.kind`, `IncludedDesignObject.connectionKind`, batch result kinds, conflict resolution, kit change semantic kinds, and any other finite string-valued fields to `async_graphql::Enum` types.
  - Keep free-form `changeKindOther` only for true extension semantics.
- Regenerate and validate SDL:
  - Use the existing ignored `export_semio_graphql_schema_file` path or the existing `semio/graphql` build command so `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)` is generated from Rust.
  - Strengthen the existing schema parity test to reject exposed `Gql`, `Row`, and `*List` scalar names, and to assert entity object names end with `Store`.
- Run verification:
  - `cargo fmt` for `semio/rs`.
  - Focused Rust tests covering `kit_graphql_smoke`.
  - The schema build command for `semio/graphql`.
  - Relevant JS/React checks only if the SDL change requires consumer updates.

## Important Constraints

- Do not introduce separate in-memory vs GraphQL domain structs. Only DTO structs may be additional serialized shapes.
- Do not preserve legacy GraphQL API names. Update tests and checked-in SDL to the cleaned surface.
- Do not create new test files; extend the embedded Rust tests already in `[semio/rs/lib.rs](semio/rs/lib.rs)`.
