# Catalogue bridge v3 (wave 2)

Status: UNCOMPILED, nothing built. Shared crates: `⚖️compliance/🧰️design-case`, app-surface `🗂️design-case-surface`, `🪟️visual-windows/🧾️properties`, projection-kit `🗂️design-case`. Artifacts: en1990/91/92/95/96/97 editors, en1997 scene/catalogue/protocol.

## 1. Editor actions
- `design-case-surface/✏️case-edits`: `insert_case_edit` (seeded from the entry), `duplicate_case_edit`, `remove_case_edit`, `set_number_edit`, `set_text_edit`, `clear_input_edit` (value `null`, which the artifact resolves to `change-case-number|text` with `None` = row removal), `input_row_edit` (typed text in any compatible unit -> stored unit, range / choice refusal with EN/DE messages, empty text = clear), `input_row_text`, `case_input_tree` (declared unset inputs as empty rows for the properties window), `render_inspector_with_cases`, `render_case_browser` / `with_case_browser` (add per entry, duplicate and remove per case as `insertItem` / `removeItem` actions). `render_inspector_projected` added to the properties window.
- Wired in all six editors: properties window, catalogue arm of `render_body`, plus `✏️editor/🧪️tests/🗂️case-edits` calling `assert_case_edit_paths`.
- Live: the law harness applies every edit through the artifact's own `EDIT_RULES` mutations on a `NormHost` and asserts live report == full evaluation after each step. Checks window and remedies are the generic ones (case results are normal `CheckResult`s).
- Tests: pure edit values incl. the null-clearing path, typed units/ranges/choices, vector rows, duplicate/remove, empty-row projection, remedy apply (`design-case-surface/✏️case-edits/🧪️tests/🔬️unit`), a `.feature` beside it, per-artifact law test (add, set number, typed row, set/clear choice, clear number, duplicate, remove).
- BUG FOUND AND FIXED: case remedies wrote `unit.to_si(value)` into number rows stored in the vocabulary unit (200 mm lever -> 0.25). Remedies of cases now carry stored-unit values with kind dimensionless; crate tests updated.

## 2. Cases in 3D
- projection-kit: `input_metres`, `input_per_metre`, `size_binding` (unit-aware handle write) + tests.
- EN 1997 (dedicated entity `case`, collection `cases`, layer `cases`): `🎬️scene/🗂️cases` (footing block, pile shaft, wall block from the declared inputs), default row `ROW_CASES = 88`, persisted `placements` win, grips for width/length/embedment, diameter/length, width/height writing the case numbers in the stored unit + move-x/move-y, kit registry/layer/entity list, palette skips `case`. Entries GEO-F01..03, PILE-04, PILE-09, RW-11 declare `geometry`. 4 scene tests.
- NOT done: en1990/1991/1992/1996 scenes (their entries declare no geometry yet; helpers ready, hooks follow the en1997 pattern). en1995 keeps its own case scene.

## 3. EN 1997 binary protocol
Already 165 records (the 47 figure was stale); the record `change-annex tag=0` was swallowed by a comment line. Fixed: 166 records == 166 KINDS, tags 0..165.

## 4. EN 1990 SQLite extent
`🧰️scripts/catalogue-bridge-v3/en1990_extent_replay.py` replays the independent extent (sqlite3 + the framework formula): 17 tables (every schema table is written even for an empty document), widths, schemaBytes 12082, full 31 rows / 4438 bytes, empty 1 / 173 all equal `🎛️semantic.json`; rs and ts tests assert 17. 15/16 guesses obsolete. A with-cases extent case is still missing.

## 5. Units review
Details in `units-review-en1991.md` (212 -> 228 rows: e.g. `s_b` N/m, `m_drive` N·m, `p_v` kPa, `av_over_v` 1/m, `alpha_c` W/(m²K); scoped splits, 17 choice rows; uncertain: `m_drive`, `a@E3.03`, `category`) and `units-review-en1992.md` (447 -> 491 rows: ~75 numeric units changed, e.g. `e_p` mm, `theta_pl`/`tendon_angle` rad, `theta_m` °C; 38 choice rows; 3 missing rows added; no generic labels left; ~15 uncertain). Missing `UnitId`s kept as `1`: 1/K, 1/min, mm²/mm, N·mm², permille. Demo fixtures reseeded by the agents' tools; the Rust demo-document tests are untested.

## 6. EN 1993 / 1994 / 1998 / 1999
These codes have no case registry (entity-driven). Gap analysis per code in `catalogue-bridge-v3-gaps.md` (spec rows that are data / not-modelled / without a snapshot entity, with candidate pure functions and inputs). Whether case catalogues (new files only) were created is stated in that file; document-side plumbing is not done.

## Open
- `add_case` -> `insert_case` rename; with-cases SQLite extent case; scene golden regeneration; en1998-mutations is over the source budget (not mine).
