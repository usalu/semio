# Nested rows (wave 2)

Nothing compiled (no cargo/rustc). `repo ./📜️script.ts verify build-owners sync` passes (0 changes); `check-cargo-authority.py`: 28 crates, 0 errors. No git writes.

## 1. Shared edit rules (`📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs`)
- Finding: `IdRemoval` already carried `selectors` (parent selectors) and `address()` resolves `parent[id=P].child[id=C]`; what was missing was one place that reads templates for all three edits, and en1999 still had a parallel local `IdRemoval` struct plus a hand-rolled `resolve_edit` (top-level only).
- New pure `NormEditRules::payload(id_removals, tree, edit) -> EditPayload` (kind + payload entries) used by `resolve` and `resolve_with_id_removals` (resolve = the latter with no id removals). Set, insert and remove all use `selectors` per list level; a removal on an `IdRemoval` path is addressed by the row id (path forms `m[id=M].a[id=A]`, `m[1].a[1]`, `m[id=M].a` + index).
- Unit test `nested_rows_are_set_inserted_and_removed_through_parent_selectors` (app-surface `🧪️tests/🔬️unit`), mutation-free via `payload`.
- en1999 edit rules migrated to the shared `IdRemoval` (`selectors: &[]` on the 14 top-level removals) and `EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, ..)`.

## 2. en1999 kinds (script `🧰️scripts/leaf-kinds/en1999_nested.py`, merge mode, private stage `🗑️generated/en1999-nested/stage`)
- 10 kinds `insert-/remove-` `member-action`, `connection-action`, `section-element`, `sheet-action`, `shell-action`; tags 185..194, crates `mut-rows-<entity>` (5, registered in Cargo.toml, workspace deps, script.ts, build-owners, mutations/io/text Cargo.toml, en1999 Cargo.lock for these and the 3 earlier row crates).
- Payload: insert `{<parent>Id, index, <row>}`, remove `{<parent>Id, id}` (id-only). Diff via the existing keyed nested lanes `En1999<Parent>Patch.{actions,elements}` + `En1999<Parent><Child>Rows::insertion/removal` (these already had removed/inserted/moved/modified with id keys, so no lane change). Duplicate id, unknown parent, bad index rejected; inverses are the opposite kind (remove inverse restores at the original index).
- Twins/layers: Rust leaves, 4 schema twins (json, ts, graphql, proto), text codec, binary protocol (185 existing tags checked contiguous), taxonomy rows, oracle manifest + vectors, harness (rs/py/feature, dupe rows), 20 fixtures (apply + dupe vectors, diffs with all-null parent patch templates taken from existing fixtures).

## 3. Coordination
EN1994-NESTED already uses the shared `IdRemoval` with `SelectorKey::Id("beamId")`; same mechanism, no variant. I removed a duplicated `use ...app_surface::{InsertItemRule, ...}` line in en1994 edit-rules (E0252). Requests appended to `📌️important/requests.md`.

## 4. Generator
`wire.create_crates`, `keyed.create_crates`, `en1993_wire.create_crates` append missing mounts to an existing crate root (`common.mount_into`) and never overwrite crate roots/Cargo.toml (`Files.exists`). `en1999_rows.py` Kind got hooks (`fields/gql_fields/proto_fields/dsl`), text-codec model imports are deduplicated.

## Open
Not run: cargo build/tests. Text codec `use model::{..}` order and the en1999 harness `current N-kind` counters should be checked by the first build. Sorted order of the added Cargo.lock blocks may be re-sorted by cargo.
- ready for rebuild: nested rows
