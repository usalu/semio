# Verify log

## Change

- Removed `OperationInputUnion` (fictitious `union OperationInput` not present in `compose/graphql/target.schema.graphql`).
- Removed `OperationInputOneOf` and structs `CreatedFixedPieceInputDto`, `FixedPieceInputDto`, `DraggedPieceInputDto`, `RenamedKitInputDto`, `ChangedDescriptionInputDto` (duplicate of the `#[Object]` input projection types with no in-repo references).
- Trimmed `operation` module `async_graphql` import: dropped `InputObject`, `OneofObject`.

## Validation

- `read_lints` on `compose/rs/lib.rs`: no issues.
- `cargo test -p compose --lib` / `cargo check -p compose`: attempted; blocked on **artifact directory file lock** in this environment (another process holding `target`). Re-run when the lock clears.
