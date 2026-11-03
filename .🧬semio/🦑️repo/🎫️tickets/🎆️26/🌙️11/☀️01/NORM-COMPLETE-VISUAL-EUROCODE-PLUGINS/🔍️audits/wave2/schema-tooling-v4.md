# SCHEMA-TOOLING v4 (wave 2)

Nothing compiled, no cargo/`repo run`, no git writes. Tool: `🧰️scripts/schema-evolve/`; tests `python3 -I test_evolve.py` (new class `WaveFour`, 10 tests); taxonomy script `🧰️scripts/schema-tooling-v4/taxonomy_ground_investigation.py`.

## 1. Reference collection is never rewritten (en1992)
Causes in `l_codec.py` (TypeScript codec): (a) the type rename `En1992SqlitePlacement[]` -> new row type was a global regex over the whole file, so the reference's own declarations were retyped; now applied to the cloned statements only (`pre_pair`); (b) `ts_inline` used statement spans of the original text on the already edited text (stale offsets) and the "already present" check looked at the whole file; spans are recomputed and the check is per line; (c) unquoted names of a single-word reference (`placements`) were renamed to the snake spelling (`probe_rows:`, `s.probe_rows`) in the camelCase TypeScript file; `Names.rename(.., unquoted="camel")` for TS; (d) the `const probeRows:T[]=[]` declaration and the shorthand `return{..,probeRows,..}` item were never cloned (spans bug), now both are.
Test `test_add_collection_only_inserts_into_the_sqlite_codecs_of_the_reference`: char-level diff of the planned sqlite DDL/Rust/TS has no `replace`/`delete` op (en1992, en1995, en1996).
Current en1992 TS/Rust/SQL placements statements verified against git HEAD diff: only the intended additions (type, `PLACEMENT` constant, write loop, read statement, return item, table), the reference text is intact; nothing to repair.

## 2. en1997 ground-investigation taxonomy
The edit rules existed; the taxonomy had 7 rows without `edit` (one with the stale kind `change-ground-investigation-reading-value`, the leaf is `...-reading-measurement`) and no insert/remove rows. `taxonomy_ground_investigation.py` (idempotent; bun generator unavailable) rewrote them from the `edit-rules` leaf: 11 rows (5 new: insert/remove investigation and reading, reading-measurement; 1 stale removed). Fixture checker `verify_fixtures.py en1997`: 222 scenarios, 209 inverted, 0 failures.

## 3. Fixes folded into evolve.py (hand repairs of EN 1998 v17)
| bug | fix | test |
|---|---|---|
| diff JSON order and `required` differ from the Rust diff | `l_schema.sync_diff_lanes` after the Rust diff layer: lane moved behind its Rust predecessor, added to `required` like the reference lane | `test_diff_schema_lists_the_lane_where_the_rust_diff_declares_it` (en1990, en1997, en1998, en1999) |
| kebab/snake edit-rule paths and items | `Tr` hint `path` (camel), items/paths camelised in `edit-rules` | `test_edit_rules_and_registries_use_camel_case_paths_and_one_kind_name` |
| `tie-beam-tie-beam-*` kind names | `Tr.__call__` protects the new kind spellings with placeholders before the entity word renames | `test_donor_entity_words_do_not_double_a_new_kind_name` |
| set-field kinds mounted in the reference crate | `l_setfield.isolate_crate`: new crate `mut-<plural>` (manifest, mounts moved out of every donor group crate, `pub use` lists, `use` paths, workspace member and dependency, `script.ts`, only the dependents that use it) | `test_set_field_kinds_get_a_crate_of_their_own_and_describe_themselves` |
| unwired incremental units | already wired (row collection, owners, `group(..)` row); `*-na` parts belong to hand-written checks, not generated | note only |

## 4. EN 1993 v20 findings (coordinator message)
(g) not idempotent: `run_ddl` tested `CREATE TABLE x (` and missed the learned DDL (`CREATE TABLE x(`), the template path then appended a duplicate and `replay` raised `sqlite3.OperationalError`; now word-boundary check plus the OperationalError falls back to the learned codec path. A second dry run of the committed `en1993-frame-actions|member-bases`, `en1998-tie-beams|slopes` specs plans 0 files (test `test_committed_collection_specs_are_a_no_op_second_time`).
(a) row structs missing and no `as_str`: `l_rust.row_structs` clones the reference `pub struct` header for the entity (and nested rows) when the model keeps hand-written records; `clone_enum` takes the donor token from the single quoted arm value, so the `impl` with `as_str` is always cloned (test `test_models_with_hand_written_rows_get_row_structs_and_as_str`).
(b) enum token spelling: `ctx.conform_enum_tokens` adopts the PascalCase/camelCase spelling of the artifact's existing enums for new enums (test `test_new_enum_tokens_follow_the_artifact_spelling`).
(c) donor descriptors in set-field leaves: `own_semantics` rewrites `entity`/`record` of the semantic descriptor and the value widget unit/displayUnit/precision/step from the spec field (was `N`/`kN` of a donor); labels already came from the spec (checked in the dry run).
(e) taxonomy rows of set-field kinds are written (`taxonomy_row`), the `mutate-<artifact>-1` mirrors (rs/py/feature) are now found for every artifact (filter was the en1997 emoji) and their kind counts grow (`bump_mirror_counts`) (test `test_exhaustive_mutation_mirrors_list_the_new_kinds_and_count`).
(f) file budget: covered by the new set-field crate; insert/remove leaves still mount into the reference's group crate.
(d) mutation fixtures: the generator builds the vectors from the donor vector with the new row (ids `probe-row-1`, spec samples) and keeps the donor document as base; the hand-ruled en1993 regeneration (`en1993-v20-frame-fixtures.py`) is not folded in; open.

## Verification
`python3 -I test_evolve.py`: Ran 37 tests, OK (27 earlier + 10 new in `WaveFour`). Add-collection dry runs with the probe spec: en1990 420 files, en1991 636, en1992 624, en1993 2097, en1994 401, en1995 529, en1996 396, en1997 723, en1998 780, en1999 758, all 0 unsupported layers; `matrix.py --leaf en1998`: 0 of 17 collections unsupported; the four committed specs (en1993 x2, en1998 x2) re-planned: 0 files.

## Open
- Nested collections for non-en1997 artifacts, bool fields, en1991 `accidentalCases` (unchanged).
- Fixture documents of set-field kinds keep the donor base document (see (d)).
- The new `isolate_crate` and `row_structs` are verified by dry-run text only (nothing compiled); apply additionally needs `verify build-owners sync` (done by `post.py`).
