# EN 1995 source boundary (2026-11-24)

## Refused crate
`semio-s-artifact-norm-en1995` (root artifact crate). "5 of 32 files, 294013 of 262144 bytes" is exactly
`🔮️oracles/🔣️.json` (231085, via `include_str!` in the mounted mutations unit test) + mutations unit test (49632)
+ root `🦀️.rs` (10426) + diff unit test (2611) + `📜️artifact-definition.json` (259) = 294013. The schema crate (201589 B) and
checks crate (~154 KB) pass admission. Boundary untouched.

## Split
- New crate `semio-s-artifact-norm-en1995-mutations-bridges` at
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridges/` (lib `🦀️.rs` + `📦️packages/🦀️rust/Cargo.toml`). It mounts the existing
  `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (file not moved); the root no longer mounts it. Dev-dependencies carry `features = ["test-support"]`
  (model, mutations). No cycles: nothing depends on it.
- The 231 KB oracle manifest is read at run time (`fixture_text!`) instead of `include_str!`.
- Registered: workspace members, `[workspace.dependencies]`, `📦️packages/🦀️rust/📜️script.ts` crate list, Cargo.lock (auto).
- Root crate: removed `mod mutations_unit`, and the dev-dependencies only that test used (21 `mut-*`, app-surface, test-fixtures).
- `🧬️mutations/🦀️.rs`: its imports of the `mut-*` modules and `En1995Diff` are now `pub use` (the unit tests need them; explicit re-export).

## Stale tests repaired (they had never compiled in the root)
- Mutations unit test: rewritten around the committed vectors (`before` + `mutation` per kind): label/kind check, codec round trip + apply + inverse,
  edit rules, value-tree edits. Dropped the hand-written 200-entry `every_mutation()` list, regen and replay (replay is covered by
  `mutations/🧪️tests/🔬️vectors`, `committed_vectors_hold`).
- Diff unit test: `ColdMutation as _` import. Root sqlite test: migrated to `original_owners::{decode_native,encode_native,export_sqlite,import_sqlite}` like en1998.
- The `validate_json_schema` / `assert_serde_snapshot_law` law in `schema/tests/compliance` was not touched.

## Results (bare `repo cargo --store=shared --configuration=prototype -- test -p <crate> --offline`)
- `semio-s-artifact-norm-en1995-mutations-bridges`: `test result: ok. 4 passed; 0 failed`, EXIT 0.
- `semio-s-artifact-norm-en1995`: source admission accepted, compiles; `test result: FAILED. 3 passed; 14 failed`. All 14 are
  `sqlite_snapshot_tests::*` failing at run time inside the framework SQLite snapshot code ("duplicate authored IEEE scalar column",
  "norm native text required", from `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot` and the norm registry sqlite admission), not
  in the split. The 2 `diff_unit` tests pass. Not fixed here (shared framework behaviour, other agents' area).
- Not re-run after the `pub use` change: `-p semio-s-artifact-norm-en1995-mutations` (its lib compiled as a dependency of the bridges run).
- Earlier runs were blocked twice by other agents' in-flight framework edits (`semio-plugin-instance-render`, `semio-plugin-tool-machine`); both resolved on retry.
