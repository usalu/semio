# SCHEMA-TOOLING v5 (wave 2)

Nothing compiled, no cargo/rustc/`repo run`, no git writes, no real domain change applied. Tool: `🧰️scripts/schema-evolve/`; tests `python3 -I test_evolve.py` (new class `WaveFive`, 8 tests); scratch round trip `roundtrip.py`.

## 1. Mutation vectors on the target's own base snapshot (`l_basedoc.py`)
- Base = the committed SQLite sample snapshot of the artifact (the document the contract measures; complete in all 10 artifacts), with the new lane filled by the spec's sample rows. Insert/remove vectors: before/after are the sample snapshot with the transformed donor rows (same count as the donor vector, so indexes stay valid) in the new lane; the diff touches only the new lane (every other lane gets its untouched form: empty delta, `[]` or `null`); mutation/outcome are the transformed donor documents. The donor's own document is no longer copied.
- Set-field vectors (`clone_fixtures`): before/after on the same base; for add-leaf the first existing row of the sample is used; the new value always differs from the stored one (the old vector set 1.0 over 1.0).
- Tests: `test_vectors_are_built_on_the_sample_snapshot_of_the_artifact` (en1998, en1993, en1995: every lane equals the sample, diff touches `probeRows` only, insert +1 row, remove -1 row), `test_set_field_vectors_change_the_value`.

## 2. Insert/remove kinds get their crate
`isolate_crate` now takes the insert/remove kinds (and nested ones) together with the set-field kinds: one crate `mut-<plural>` per new collection (manifest, workspace member + dependency, `script.ts`, dependents, `use` paths). Mounts leave the group crates of the donors; a bare `use` of a moved module in a dependent keeps working through a `use` of the new crate; `pub use` re-exports of the old group crates are dropped (consumers are re-pointed). Test `test_insert_and_remove_kinds_live_in_the_crate_of_the_collection` (en1998, en1993): exactly one mount per kind, in the new crate, no remaining `..._mut_<other>::insert_probe_row` in any planned Rust/TOML.

## 3. en1998 per-family text codec (`l_text.py`)
For artifacts with a `mutations-text-kit` crate: a new family crate `mutations-text-<plural>` (dir below `🚪️io/📝️text/🧬️mutations/`) with the cloned `print_<kind>` / `parse_<kind>` of every new kind (insert, remove, set-field, nested), `use` lines from the new `mut-<plural>` crate, manifest (kit + mut crate), workspace member/dependency, `script.ts`, dependency in the dispatcher crate, `use ... as text_<plural>;` and both dispatcher arms per kind routed to the new alias (the registry clone had copied them pointing at the donor family). add-leaf kinds join the family file of their donor kind. Monolithic codecs (all other artifacts) need nothing (the registry clone copies arms and functions). Tests: `test_en1998_text_codec_gets_a_family_crate_and_two_dispatcher_arms_per_kind` (5 kinds, 10 arms, no arm to another alias), `test_leaf_kinds_join_the_family_crate_of_their_donor_kind`.

## 4. `nested_patch!` (coordinator message, en1992 run 19)
`l_patch.py`: every `nested_patch!` block keeps one `set`, `within`, `optional`, `rows` section in that order; duplicated sections are merged in place (later item of the same key wins) and the block rewritten, never appended; runs at the end of the diff layer of add-collection and add-leaf. The collection clone no longer rewrites a `nested_patch!` body with the flat form (it kept the donor's `within`/`optional`/`rows` items before; now they are emptied). Tests: `test_nested_patch_blocks_keep_one_section_each_in_macro_order`, `test_leaf_on_a_nested_patch_entity_edits_in_place` (en1992 `members` leaf: order set, within, optional, rows; field added in `set`).

## 5. Nested collections outside en1997
Not implemented in the learned codec; reported by measurement (`nested_report`, replaces the blanket message) and without crashes: all artifacts have nested donors in the schema, but the statement-level learned codec clones one table's statements, while a nested collection needs parent and child statements and the per-parent counting loops together.
| artifact | nested donors in the DDL | child rows in the Rust codec | verdict |
|---|---|---|---|
| en1991 | accidental_case -> impact, explosion (4) | separate statements | next candidate |
| en1992 | member -> bar_layer, stirrups, punching, ... (14) | separate statements | next candidate |
| en1995 | member/connection -> action, case -> ... (6) | separate statements | next candidate |
| en1993 | joint -> joint_action, fatigue_detail -> band, ... (8) | inline in the parent statement | needs parent+child clone |
| en1994 | beam -> steel/sheeting/studs, ... (7; flattened parents) | inline | needs parent+child clone |
| en1998 | building -> system/storey/..., ... (6) | inline (Rust) | needs parent+child clone |
| en1999 | section -> plate_element, member -> action, ... (7) | inline | needs parent+child clone |
| en1990, en1996 | none besides `cases` | - | no reference collection with nested rows and a diff row group (now a clean `UNSUPPORTED`, was an uncaught exception in `Ctx`) |
Test `test_nested_collections_stop_with_a_measured_reason`.

## Other fixes
- `l_sqlite.run_admission`: artifacts without `🛂️admission` (en1992, en1998) crashed on the second run of an applied plan (FileNotFoundError, not caught); now falls to the learned path like the first run.
- `Ctx`: nested reference failure is an `UNSUPPORTED` entry, `nested_ref_names()` raises `Unsupported` only for nested specs.

## Verification
- `python3 -I test_evolve.py`: see the result line at the end of this file.
- `roundtrip.py` (scratch copy of each artifact, files only, no twins/builds): probe collection dry run 0 UNSUPPORTED and, after applying the plan to the scratch copy, the same plan again yields 0 files, for all ten artifacts: en1990 420, en1991 636, en1992 624, en1993 2105, en1994 401, en1995 529, en1996 396, en1997 729, en1998 788, en1999 759 planned files first time, 0 second time.

## Unverified by compiler
New crate manifests/mounts, re-pointed `use` paths, the family crate and dispatcher alias, Cargo.lock blocks of the new crates (left to cargo), `build-owners` sync (post step of `--apply`).

Result: `python3 -I test_evolve.py`: Ran 45 tests, OK (37 earlier + 8 new in `WaveFive`).
