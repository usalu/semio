# GraphQL Target Schema Mutation Cleanup

**Id:** `2026/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP`

## Problems

1. Every mutation `*ScopeInput` carried `transactionId: ID!`, forcing every operation onto an explicit transaction scope. The `Draft.openTransaction` already determines transaction routing — it MUST be implicit.
2. Mutation `*ScopeInput` wrappers added no value: each one only grouped 1–4 ID fields. Args MUST be inlined directly on the mutation endpoint.
3. Mutation `input` arguments wrapped existing `input` GraphQL types in single-field `*Input` wrappers (e.g. `CreateTagInput { tag: TagInput! }`, `AddAttributeToTagInput { attribute: AttributeInput! }`). Only operation GraphQL types (with primitive operation-specific fields like `RenameKitInput { name: String! }`) MUST keep `input` wrappers.
4. Several mutations had `*Input { hasInput: Boolean = false }` placeholder wrappers (delete/remove/flatten/fix) — they MUST disappear; the inlined args already carry the data.

## Changes

- `semio/graphql/target.schema.graphql` — refactored `#region MutationInputs` and `type Mutation`:
  - Removed every `*ScopeInput` (`RenameKitScopeInput`, `CreateTagScopeInput`, `AddAttributeToTagScopeInput`, `RemoveAttributesFromTagScopeInput`, `DeletePiecesInDesignScopeInput`, …) and inlined the entity-id scope fields directly as named mutation arguments (`ownerId: ID!`, `tagId: ID!`, `attributeId: ID!`, `attributeIds: [ID!]!`, `pieceIds: [ID!]!`, …).
  - Added a single shared `input TransactionScopeInput { draftId: ID!, transactionId: ID! }` and made it the **first** argument of every mutation endpoint (`transactionScope: TransactionScopeInput!`). All draft + transaction routing flows through this one wrapper instead of duplicating the two ids on every per-operation `*ScopeInput`.
  - Removed every entity-input wrapper (`Create*Input`/`Add*Input`/`Add*sInput`) and inlined the wrapped GraphQL `input` types as named arguments (`tag: TagInput!`, `tags: [TagInput!]!`, `attribute: AttributeInput!`, `attributes: [AttributeInput!]!`, `concept`, `concepts`, `port`, `ports`, `quality`, `qualities`, `type`, `types`, `connector`, `connectors`, `design`, `designs`, `child`, `children`, `offset: OffsetInput!`, `position: PositionInput!`).
  - Removed every `*Input { hasInput: Boolean = false }` placeholder wrapper from the schema and from the mutation signature.
  - Kept operation GraphQL `input` wrappers carrying real operation-specific fields: `RenameKitInput`, `ChangeDescriptionInput`, every `Rename*Input` / `Update*DescriptionInput` / `Update*IconInput`, `RenamePortInput { code, label }`, `RenameQualityInput { key }`, `RenameConnectorInTypeInput`, `Rename/UpdatePieceDescriptionInDesignInput`, `AddFixedPieceToDesignInput { blueprintId, position, name, description }`, `ChangePiece(s)ToTypeInDesignInput { blueprintId }`.

## Verification

- `node -e "buildSchema(read('semio/graphql/target.schema.graphql'))"`: `parse OK`. Only remaining `BUILD ERR` is the pre-existing duplicate `Modification` (interface @ L109 + union @ L12741), unrelated to this ticket.
- All `Unknown type` errors that existed before refactoring are gone (`CreateTagInput`, `AddAttributeToTagInput`, `RemoveAttributeFromTagInput`, `DeleteTagInput`, …).
- `rg "ScopeInput|^.*draftId" semio/graphql/target.schema.graphql` → 1 match: the `draftId: ID!` field inside `TransactionScopeInput`.
- `rg transactionId semio/graphql/target.schema.graphql` → 1 match: the `transactionId: ID!` field inside `TransactionScopeInput`.
- Every mutation now begins with `transactionScope: TransactionScopeInput!` as its first argument (95 endpoints).

## Status

Closed.
