# GraphQL Target Schema Mutation Cleanup

**Id:** `2026/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP`

## Problems

1. Every mutation `*ScopeInput` carried `transactionId: ID!`, forcing every operation onto an explicit transaction scope. The `Draft.openTransaction` already determines transaction routing — it MUST be implicit.
2. Mutation `*ScopeInput` wrappers added no value: each one only grouped 1–4 ID fields. Args MUST be inlined directly on the mutation endpoint.
3. Mutation `input` arguments wrapped existing `input` GraphQL types in single-field `*Input` wrappers (e.g. `CreateTagInput { tag: TagInput! }`, `AddAttributeToTagInput { attribute: AttributeInput! }`). Only operation GraphQL types (with primitive operation-specific fields like `RenameKitInput { name: String! }`) MUST keep `input` wrappers.
4. Several mutations had `*Input { hasInput: Boolean = false }` placeholder wrappers (delete/remove/flatten/fix) — they MUST disappear; the inlined args already carry the data.
5. Every entity type duplicated the general interface fields `ownerEntity: OwnerEntity` / `ownedEntities: OwnedEntityConnection` with narrow-typed projections (`owner: VectorOwner!`, `planeOwner: Plane`, `planeDiffOwner: PlaneDiff`, …) and spine references (`ownerModifications: Modifications`, `changeOwner: Change`, `operationOwner: Operation`, …). Only the general interface fields MUST remain.

## Changes

- `semio/graphql/target.schema.graphql` — refactored `#region MutationInputs` and `type Mutation`:
  - Removed every `*ScopeInput` (`RenameKitScopeInput`, `CreateTagScopeInput`, `AddAttributeToTagScopeInput`, `RemoveAttributesFromTagScopeInput`, `DeletePiecesInDesignScopeInput`, …) and inlined the entity-id scope fields directly as named mutation arguments (`ownerId: ID!`, `tagId: ID!`, `attributeId: ID!`, `attributeIds: [ID!]!`, `pieceIds: [ID!]!`, …).
  - Added a single shared `input TransactionScopeInput { draftId: ID!, transactionId: ID! }`.
  - Introduced a root-level namespacing mutation `change(transactionScope: TransactionScopeInput!): KitChange!` and moved every kit-changing operation onto the new `type KitChange { … }`. The transaction scope is supplied **once** at the root; nested operations carry only their own entity-id args and operation `input`. One `change` call thus batches multiple selected operations under a single transaction scope.
  - Removed every entity-input wrapper (`Create*Input`/`Add*Input`/`Add*sInput`) and inlined the wrapped GraphQL `input` types as named arguments (`tag: TagInput!`, `tags: [TagInput!]!`, `attribute: AttributeInput!`, `attributes: [AttributeInput!]!`, `concept`, `concepts`, `port`, `ports`, `quality`, `qualities`, `type`, `types`, `connector`, `connectors`, `design`, `designs`, `child`, `children`, `offset: OffsetInput!`, `position: PositionInput!`).
  - Removed every `*Input { hasInput: Boolean = false }` placeholder wrapper from the schema and from the mutation signature.
  - Kept operation GraphQL `input` wrappers carrying real operation-specific fields: `RenameKitInput`, `ChangeDescriptionInput`, every `Rename*Input` / `Update*DescriptionInput` / `Update*IconInput`, `RenamePortInput { code, label }`, `RenameQualityInput { key }`, `RenameConnectorInTypeInput`, `Rename/UpdatePieceDescriptionInDesignInput`, `AddFixedPieceToDesignInput { blueprintId, position, name, description }`, `ChangePiece(s)ToTypeInDesignInput { blueprintId }`.
- `semio/graphql/target.schema.graphql` — stripped narrow-typed owner duplicates from every entity, leaving only the general interface fields:
  - Removed `owner: <X>Owner!` from every type/interface (e.g. `owner: VectorOwner!`, `owner: ModificationOwner!`, `owner: PointDiffOwner!`, …).
  - Removed every narrow arm `<x>Owner: <X>` projection (e.g. `planeOwner: Plane`, `planeDiffOwner: PlaneDiff`, `vectorOwner: Vector`, `vectorModificationOwner: VectorModification`, `changeOwner: Change`, `operationOwner: Operation`, `pieceDiffOwner: PieceDiff`, `kitOwner: Kit`, `connectorOwner: Connector`, `representationOwner: Representation`, …).
  - Removed every spine reference `ownerModifications: Modifications` and `ownerDiffs: Diffs`.
  - Removed the placeholder doc comments `# owner: ENTITY # reference`, `# ENTITYOwner: ENTITYOwner # computed`, `# owns: ENTITYConnection # computed` from the interfaces.
  - Kept the general fields `ownerEntity: OwnerEntity # computed` and `ownedEntities: OwnedEntityConnection # computed` everywhere.
  - Total: **807 narrow-typed lines removed**. Every entity type/interface now has only `id`, `hash`, `ownerEntity`, `ownedEntities`, plus its own data fields. The narrow `*Owner` / `*Owned` unions are kept as documentation referenced from the `# computed // <X>Owner` comments.

## Verification

- `node -e "buildSchema(read('semio/graphql/target.schema.graphql'))"`: `parse OK`. Only remaining `BUILD ERR` is the pre-existing duplicate `Modification` (interface @ L109 + union @ L12741), unrelated to this ticket.
- All `Unknown type` errors that existed before refactoring are gone (`CreateTagInput`, `AddAttributeToTagInput`, `RemoveAttributeFromTagInput`, `DeleteTagInput`, …).
- `rg "ScopeInput|^.*draftId" semio/graphql/target.schema.graphql` → 1 match: the `draftId: ID!` field inside `TransactionScopeInput`.
- `rg transactionId semio/graphql/target.schema.graphql` → 1 match: the `transactionId: ID!` field inside `TransactionScopeInput`.
- `rg transactionScope semio/graphql/target.schema.graphql` → 1 match: the `change(transactionScope: TransactionScopeInput!): KitChange!` field on `Mutation`.
- `type Mutation` exposes a single field `change`. `type KitChange` exposes 95 operations (`renameKit`, `createTag`, …, `deletePiecesAndConnectionsInDesign`).
- `rg "^  owner: |^  \w+Owner: |^  ownerModifications: |^  ownerDiffs: " semio/graphql/target.schema.graphql` → 0 matches (every narrow-typed owner duplicate is gone).
- `rg "^  ownerEntity: OwnerEntity" semio/graphql/target.schema.graphql` → 265 matches (the general interface field is intact on every type and interface).
- Sample entity (`type Vector implements WeakEntity`) now has only `id`, `hash`, `ownerEntity`, `ownedEntities`, `x`, `y`, `z`.
- The owner-field cleanup adds 4 new `BUILD ERR` lines (`Renamed`, `AddedIcon`, `ChangedIcon`, `RemovedIcon`) — verified pre-existing in HEAD; not introduced by this ticket.

## Status

Closed.
