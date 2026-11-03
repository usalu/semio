# Catalogue bridge v2: EN 1996 `cases`

Scripts: `🧰️scripts/catalogue-bridge-v2/en1996_{rust,twins,sqlite,fixtures,leaves,registry,mutwire,checks}.py` (uncompiled, nothing built).

Done
- Model `cases: Vec<CaseRow>` (default demo `case-m32`, M-32), root artifact struct, diff lane `cases: CaseRowDelta` (rs + json/ts/graphql/proto). The diff `apply` literal also lacked `placements` (pre-existing compile error); both lanes are now committed.
- Twins: snapshot typed, root inline objects, diff compact.
- SQLite: `en1996_case`, `_case_number`, `_case_text` (sql, rs write/bound/read, admission field 6 + row count, ts, tests; 16 tables, widest table unchanged at 101 columns), `schemaBytes` 12560 and table widths in `🎛️semantic.json`.
- 7 leaves in `semio-s-artifact-norm-en1996-mut-design-cases` (depends on `-masonry`, which owns `En1996Cases`, no cycle); root enum/KINDS/json/ts/graphql/proto, JSON text codec samples, protocol tags 143-149, edit rules, oracle, taxonomy, harness py/feature rows, 7 apply vectors (M-30, M-32), `cases: []` in 247 snapshots and empty case delta in 123 diffs.
- Checks: `masonry::evaluate_design_cases(annex, cases, scope)` (duplicate id on the first row of a repeated id, then the bridge), appended in `inferences::evaluate` and as one unit per case in `En1996Units`; field-meta via vocabulary; per-case scene diagram (`case_diagram`); `catalogue_demo_cases()` + test; full-catalogue document `🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json` (40 cases); the editor catalogue panel appends the case browser table.

Open / unverified
- Bless on first run: scene golden, example assets, sqlite extents (fixture holds `cases: []`).
- Add/properties actions for catalogue cases are not wired in the editor (browser table only).
- Seeded demo values rely on vocabulary examples; M-30/M-32 sample values are my own estimates.
