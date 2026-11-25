# Boot failure: UnsupportedOwner, field owner has no retained source projection

## Root cause
`controlled_tagged_variant_field!` (process3d sqlite snapshot `native-fields`) hand-implements `DslField` for `MeasureRecipe`, `WorkingSolid` and `ProcessMeasure`. It had no `projection_view` or `projection_key`, so it fell back to the `DslField` default, which refuses with `UnsupportedOwner`.
The derive's `variant_field_codegen` emits both methods. The hand macro never did. The retained encode path (projection measure in `dsl/schema/encode`) now reads every field through `projection_view`, and the brep agent's retained-projection work made that mandatory. So the boot fails the first time a recipe or solid is encoded.

## Fix
Added `projection_view` (`Statements(1)`, then `projected_variant_view`) and `projection_key` to the macro. Both mirror the derive's emission.

## Law
`sqlite_snapshot_process3d_tagged_variant_fields_project_retained_statements` in the sqlite snapshot tests.
- It asserts `projection_view` returns `Statements(1)` at the root and `Record` at path `[0]`.
- It asserts `projection_key` returns `disc-cut`.
- It asserts an out-of-range path is refused.
- On the old code the first `unwrap` fails with UnsupportedOwner.
- Native result with the fix: 1 passed. The first run failed only on my wrong expected keyword, after the projection asserts had passed.

## Boot check
hub wasm rebuilt (exit 0), published, and `rebuild-and-boot.sh` reported "boot ok attempt 1"; the UnsupportedOwner error is gone. A mid-edit compile error from the brep agent in `semio-plugin-ownership` appeared once in a native test build and cleared on retry.

## Files changed
- `process3d/.../io/sqlite/snapshot/native-fields/🦀️.rs`
- `process3d/.../io/sqlite/snapshot/tests/🦀️.rs` (new test)
- `boot-cargo.sh` (private build-dir copy of replay-cargo.sh, in the ticket)
- `boot-projection.md`
The private build dir was deleted. No brep-agent files were touched.
