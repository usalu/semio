# GraphQL Target Schema Mutation Cleanup

**Id:** `2026/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP`

## Problems

1. Every mutation `*ScopeInput` carries `transactionId: ID!`, forcing every operation to be invoked inside an explicit transaction scope. The `Draft.openTransaction` already determines routing — the transaction id MUST be implicit.
2. Mutation `input` arguments wrap existing `input` GraphQL types in single-field `*Input` wrappers (e.g. `CreateTagInput { tag: TagInput! }`, `AddAttributeToTagInput { attribute: AttributeInput! }`). Only operation GraphQL types (which carry primitive operation-specific fields like `RenameKitInput { name: String! }`) MUST keep `input` wrappers.
3. Several mutations have `*Input { hasInput: Boolean = false }` placeholder wrappers (delete/remove/flatten/fix) — these MUST disappear from the mutation signature entirely; the scope already carries the data.

## Changes

- `semio/graphql/target.schema.graphql` — refactored `#region MutationInputs` and `type Mutation`:
  - Dropped `transactionId: ID!` from every `*ScopeInput`.
  - Removed `Create*Input`, `Add*Input`/`Add*sInput` wrappers around entity inputs and inlined the wrapped `input` GraphQL types as named arguments (`tag: TagInput!`, `tags: [TagInput!]!`, `attribute: AttributeInput!`, `attributes: [AttributeInput!]!`, `concept`, `concepts`, `port`, `ports`, `quality`, `qualities`, `type`, `types`, `connector`, `connectors`, `design`, `designs`, `child`, `children`, `offset`, `position`).
  - Removed all `*Input { hasInput: Boolean = false }` placeholder wrappers and their `input:` arguments from the mutation.
  - Kept operation GraphQL `input` wrappers that carry primitive/multi-field operation data (`RenameKitInput`, `ChangeDescriptionInput`, `Rename*Input`, `Update*DescriptionInput`, `Update*IconInput`, `RenamePort/ConnectorInput`, `RenameQualityInput`, `Rename/UpdatePieceDescriptionInDesignInput`, `AddFixedPieceToDesignInput`, `ChangePiece(s)ToTypeInDesignInput`).

## Status

Open.
