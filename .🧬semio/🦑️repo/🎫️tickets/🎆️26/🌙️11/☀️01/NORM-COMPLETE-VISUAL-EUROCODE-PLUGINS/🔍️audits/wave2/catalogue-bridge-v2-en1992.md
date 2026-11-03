# Catalogue bridge v2: EN 1992 `cases`

Scripts: `🧰️scripts/catalogue-bridge-v2/en1992_{rust,twins,sqlite,leaves,text,registry,fixtures,checks,meta}.py` (uncompiled, nothing built).

Done
- Model `cases: Vec<CaseRow>` (demo `case-m04`, M-04), root artifact struct, diff lane `cases: CaseRowDelta` (rs apply/absorb/is_empty/inverse; json compact defs, ts, graphql, proto); snapshot and root twins typed.
- SQLite: `en1992_case`, `_case_number`, `_case_text` (sql, rs, semantic-cells admission: shape 12, table widths, nested census; ts incl. exact field checks; tests: 21 tables; fixture `snapshot.cases`, `tableNames`, `tableWidths`).
- 7 leaves in `semio-s-artifact-norm-en1992-mut-design-cases` (depends on `-checks`, `En1992Cases` via `checks::En1992Cases`, no cycle), root enum/KINDS/json/ts, text codec arms (spaces in JSON tokens escaped as ` `), protocol tags, edit rules, oracle, taxonomy, harness rs/py/feature, 7 apply vectors (M-04, M-02), `cases: []` in 286 snapshots and empty case delta in 141 diffs.
- Checks: cases evaluated in `evaluate_scoped` (rows `cases`), duplicate-id check, one live unit per case, outline, field-meta via vocabulary (the editor crate now depends on `-checks`; Cargo.toml and lock updated), per-case scene diagram, editor catalogue panel appends the case browser table, `catalogue_demo_cases()` + test, full-catalogue document fixture (174 entries parsed by the script).

Open / unverified
- Bless on first run: scene golden, example assets; sqlite fixture holds `cases: []` (no with-cases extent case).
- Add/properties actions for catalogue cases are not wired in the editor (browser table only). Many seeded demo values come from vocabulary examples that are absent (133 EN 1992 keys are dimensionless fallbacks); such cases yield input-issue results.
- The fixture script parsed 174 entries (registry reports 172): duplicates in test/helper code; the Rust test covers the registry itself.
