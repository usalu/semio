# Verify log

## Change

- Removed `OperationInputUnion` (fictitious `union OperationInput` not present in `semio/graphql/target.schema.graphql`).
- Removed `OperationInputOneOf` and structs `CreatedFixedPieceInputDto`, `FixedPieceInputDto`, `DraggedPieceInputDto`, `RenamedKitInputDto`, `ChangedDescriptionInputDto` (duplicate of the `#[Object]` input projection types with no in-repo references).
- Trimmed `operation` module `async_graphql` import: dropped `InputObject`, `OneofObject`.

## Validation

- `read_lints` on `semio/rs/lib.rs`: no issues.
- `cargo test -p semio --lib` / `cargo check -p semio`: attempted; blocked on **artifact directory file lock** in this environment (another process holding `target`). Re-run when the lock clears.
