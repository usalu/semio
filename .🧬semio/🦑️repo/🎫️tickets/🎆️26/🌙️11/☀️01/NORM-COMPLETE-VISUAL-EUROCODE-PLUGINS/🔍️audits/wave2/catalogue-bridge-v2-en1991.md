# Catalogue bridge v2: EN 1991 `cases`

Scripts: `🧰️scripts/catalogue-bridge-v2/en1991_{rust,twins,sqlite,leaves,leaves2,text,registry,fixtures,checks,meta}.py`, `lib2.py`/`fixtures2.py` (post kind-convention names `change-case-number|text`). Uncompiled, nothing built.

Done
- Model `cases: Vec<CaseRow>` (default demo `case-e11-04`), root artifact struct, diff lane `cases: CaseRowDelta` (rs apply literal, absorb, is_empty, inverse; json compact defs, ts, graphql JSON, proto string field 72); snapshot/root twins typed (json `$defs`/`definitions` compact, ts).
- SQLite: `en1991_case`, `_case_number`, `_case_text` (sql, rs, admission field 92, ts, tests: 21 tables, widest table unchanged at 188), `schemaBytes` 18925 and table widths in `🎛️semantic.json`.
- 7 leaves in `semio-s-artifact-norm-en1991-mut-design-cases` (depends on `-checks`, no cycle), root enum/KINDS/json/ts, JSON text codec samples, protocol tags 122-128, edit rules, oracle, taxonomy, harness feature rows, 7 apply vectors (E11.04, E11.03), `cases: []` in 245 snapshots and empty case delta in 122 diffs.
- Checks: cases evaluated in `check_full_actions_scoped` after the wave-3 rows, duplicate-id check, one live unit per case, outline field + entry count, field-meta via vocabulary, per-case scene diagram, editor catalogue panel appends the case browser table, `catalogue_demo_cases()` + test, full-catalogue document fixture (98 of 99 registry entries parsed by the fixture script; the Rust test covers all).

Open / unverified
- Kind-convention rename ran while I installed the leaves: the two `change-case-number|text` leaf dirs were installed afterwards from the EN 1995 templates (names consistent now).
- Bless on first run: scene golden, example assets, sqlite extents (fixture `cases: []`).
- Add/properties actions in the editor are not wired (browser table only). Seed values: vocabulary has no `example` for most EN 1991 text/number inputs, so many demo cases yield input-issue results (still non-empty).
