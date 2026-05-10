# GraphQL Target Schema Mutation Cleanup

**Id:** `2026/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP`

## Problems

1. Every mutation `*ScopeInput` carried `transactionId: ID!`, forcing every operation onto an explicit transaction scope. The `Draft.openTransaction` already determines transaction routing — it MUST be implicit.
2. Mutation `*ScopeInput` wrappers added no value: each one only grouped 1–4 ID fields. Args MUST be inlined directly on the mutation endpoint.
3. Mutation `input` arguments wrapped existing `input` GraphQL types in single-field `*Input` wrappers (e.g. `CreateTagInput { tag: TagInput! }`, `AddAttributeToTagInput { attribute: AttributeInput! }`). Only operation GraphQL types (with primitive operation-specific fields like `RenameKitInput { name: String! }`) MUST keep `input` wrappers.
4. Several mutations had `*Input { hasInput: Boolean = false }` placeholder wrappers (delete/remove/flatten/fix) — they MUST disappear; the inlined args already carry the data.
5. Every entity type duplicated the general interface fields `ownerEntity: OwnerEntity` / `ownedEntities: OwnedEntityConnection` with narrow-typed projections (`owner: VectorOwner!`, `planeOwner: Plane`, `planeDiffOwner: PlaneDiff`, …) and spine references (`ownerModifications: Modifications`, `changeOwner: Change`, `operationOwner: Operation`, …). Only the general interface fields MUST remain.
6. The schema carried 393 narrow `union` declarations (`OwnerEntity`, `OwnedEntityConnection`, `VectorOwner`, `Scope`, `Input`, `Blueprint`, etc.) that are no longer necessary now that fields are typed against the general interfaces (`Entity`, `EntityConnection`). All unions MUST be removed; union-typed fields MUST be retyped to the general interface and document the previous union members in a trailing `// Member1 | Member2 | …` comment.
7. The mutation API still exposed a flat `KitChange` namespace with redundant `*Id` arguments (`tagId`, `conceptId`, `designId`, `pieceId`, …) and per-operation `*Input` wrappers (`RenameTagInput`, `UpdateTagDescriptionInput`, `UpdateTagIconInput`, …). The transaction-scope was passed once as an `input TransactionScopeInput` argument. The mutation API MUST be refactored into a hierarchical scoped command tree where each scope is entered via a navigation field (`session → draft(id) → transaction(id) → kit → tag(id)/concept(id)/…`), every leaf field executes and returns `ID!`, and the per-operation single-field `*Input` wrappers MUST disappear (`rename(newName: String!): ID!`, `changeDescription(newDescription: String!): ID!`, …).

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
- `semio/graphql/target.schema.graphql` — removed every `union` declaration and retyped every union-typed field:
  - **Removed all 393 unions** (5561 lines): `OwnerEntity`, `OwnedEntityConnection`, `EntityConnection` (the union, not the interface), `VectorOwner`, `VectorOwned`, `VectorDiffOwner`, every `*Owner` / `*Owned` / `*DiffOwner` / `*ModificationOwner` / `*ModificationsOwner` narrow union, plus `Scope`, `Input`, `Blueprint`.
  - **Renamed `interface EntityConnectionInterface` → `interface EntityConnection`**, since the union previously holding that name has been removed.
  - **Rewrote `ownerEntity: OwnerEntity # computed [// X]` → `owner: Entity # reference [// Member1 | Member2 | …]`** on 265 fields. Where the original line carried a narrow union annotation (e.g. `// VectorOwner`), the trailing comment now lists the union's members (`// Plane | PlaneDiff`). Where the original line referred only to the global `OwnerEntity` (no narrow annotation), the trailing comment is omitted to avoid noise — the field type `Entity` already conveys the meaning.
  - **Rewrote `ownedEntities: OwnedEntityConnection # computed [// X]` → `owned: EntityConnection # reference [// Member1 | Member2 | …]`** on 265 fields, same rule.
  - **Narrowed `scope: Scope!` and `input: Input!`** on every concrete operation type to its operation-specific concrete `*Scope!` / `*Input!` (95 + 95 fields). On `interface Operation` the fields are dropped entirely (each implementation now declares its own narrow scope/input).
  - **Rewrote `blueprint: Blueprint(!)?` → `blueprint: Entity(!)? # … // Type | Design`** (2 fields) and `node: Blueprint! # reference` → `node: Entity! # reference // Type | Design` on `BlueprintEdge`.
- `semio/graphql/target.schema.graphql` — replaced `#region MutationInputs` and `type Mutation { change(transactionScope) … }` + `type KitChange { … }` with a hierarchical scoped command API:
  - **Removed `input TransactionScopeInput`** plus every per-operation single-field `*Input` wrapper (`RenameKitInput`, `RenameTagInput`, `RenameConceptInput`, `RenamePortInput`, `RenameQualityInput`, `RenameTypeInput`, `RenameConnectorInput`, `RenamePieceInput`, every `Update*DescriptionInput`, every `Update*IconInput`, `ChangeDescriptionInput`, `ChangePieceToTypeInput`, `ChangePiecesToTypeInput`). Only the multi-field `AddFixedPieceInput { blueprintId, position, name, description }` remains.
  - **Replaced the flat `type Mutation { change(transactionScope): KitChange! }`** with `type Mutation { session: SessionScopedCommandInput! }`. The full hierarchy is:
    - `SessionScopedCommandInput { start: ID!, end: ID!, draft(id: ID!): DraftScopedCommandInput! }`
    - `DraftScopedCommandInput { transaction(id: ID!): TransactionScopedCommandInput! }`
    - `TransactionScopedCommandInput { kit: KitScopedOperationInput! }`
    - `KitScopedOperationInput` — Artifact (`rename`/`changeDescription`) + scoped owns: `tag`/`concept`/`quality`/`type`/`design` navigation + bulk create/delete.
    - `Tag`/`Concept`/`Quality`/`Type`/`Port`/`Connector`/`Design`/`Piece`/`Pieces` ScopedOperationInput — each carries the operations that belong to that entity kind (Artifact + Attributes + entity-specific operations like `flatten`, `drag`, `move`, `fix`, `changeToType`, …).
  - **Inlined per-operation primitive args** as named field arguments: `rename(newName: String!)`, `changeDescription(newDescription: String!)`, `changeIcon(newIcon: String!)`, `rename(newCode: String!, newLabel: String)` for `Port`, `rename(newKey: String!)` for `Quality`, `changeToType(blueprintId: ID!)`, `drag(offset: OffsetInput!)`, `move(position: PositionInput!)`, `move(offset: OffsetInput!)` for `Pieces`, `fix: ID!`, `flatten: ID!`.
  - **No more `*Id: ID!` arguments at the leaf** — entity ids are carried by the scope-navigation chain (`design(id: ID!)` → `piece(id: ID!).drag(offset)`).

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
- After the union removal: `parse OK`. The only `BUILD ERR`s are the 7 pre-existing unknown stub types (`Renamed`, `AddedIcon`, `ChangedIcon`, `RemovedIcon`, `AddedPreviewImage`, `ChangedPreviewImage`, `RemovedPreviewImage`) — verified present in HEAD; out of scope for this ticket.
- `rg "^union\s" semio/graphql/target.schema.graphql` → 0 matches.
- `rg "\bOwnerEntity\b|\bOwnedEntityConnection\b|\bEntityConnectionInterface\b" semio/graphql/target.schema.graphql` → 0 matches.
- File size went from 12 198 → 5 828 lines (-52%). The schema now has 0 unions, narrow-typed scope/input on every concrete operation, and clean general-interface fields on every entity.
- After the scoped command refactor: `parse OK` and **`build OK`** (no errors at all).
- `rg "^input TransactionScopeInput|^input Rename|^input Update|^input ChangeDescription|^input ChangePiece" semio/graphql/target.schema.graphql` → 0 matches.
- `rg "^type KitChange\b" semio/graphql/target.schema.graphql` → 0 matches; `rg "^type Mutation\b" …` → 1 match.
- `type Mutation` exposes a single field `session: SessionScopedCommandInput!`. The hierarchy `session → draft(id) → transaction(id) → kit → {tag,concept,quality,type,design,…}(id)` is the only path to every kit-changing operation; every leaf returns `ID!`.

## Status

Closed.
