# I18N-FIX v2 (wave 2)

No cargo, no git writes. Python under `nice -n 19`.

## 1. note -> description (no alias)
Touched: `⚖️compliance/🧭️spec-trace/{🧬️schema/🔣️.json, 🧰️codegen/🐍️.py, 🦀️.rs, 🧪️tests/🔬️unit/🦀️.rs}`, `⚖️compliance/🧪️tests/🌐️i18n-sinks/🐍️.py`, `📇️registry/…/📚️catalogue-and-inspection/🦀️.rs` (`item.description`), the ten `🧭️spec-trace/🔣️.json` and their regenerated `🦀️.rs` twins (only Rust twins exist, no TS). Rust field `SpecTrace.description`; `SpecTrace::new(.., description)`. Verifier (`--write`, then check): en1990 29, en1991 71, en1992 159, en1993 142, en1994 54, en1995 68, en1996 37, en1997 42, en1998 32, en1999 42: all ok, twins in sync; i18n-sinks test ok.

## 2. EN 1995 descriptions
All 68 rows rewritten in EN and DE from `📐️specs/en1995-en1996-target.md` (formulas, factors, limits, NA-DE variants), keeping the realisation suffix ("realised by ..." / "covered as a design case"). Corrections to the heading-only drafts: T-33 (apex reinforcement per NA-DE), T-66 (footbridge excitation, see T-120), T-98 (delegates seismic behaviour factor q to EN 1998; the old text said CLT/panel rules), T-79 (delegated to EN 1993). T-122 keeps its partial-not-modelled statement. Unverified numerical details of the spec marked [V] are quoted as the spec has them. Source script: scratchpad only (not kept).

## 3. Clause and annex labels in UI paths
Already localised: checks window (`clause_text` -> `ClauseId::text`), clause links, spec-trace table. Fixed (raw English reached the UI):
- check-parameters `check_parameter_label`: `designator(clause)`.
- properties NDP row: tree item label clause via `designator`, source row locator via `locator_text` (was raw `source.locator`, i.e. `Annex B`).
- catalogue table header: `table.clause.text(&locale)` (was `Display`, English).
- design-case surface: clause cell and the "add case" item use `designator`.
- ids-chrome `report_table_rows`: `check.clause.text(&locale)` (was `to_string()`).
Not changed: screen-reader `CheckRowFacts.clause` is caller-supplied text (callers pass `clause_text`; only tests build it). Raw `"Annex ..."`/`"Table ..."` literals remaining are `ClauseId` section parts (rendered via `text`), spec-trace `clause` strings (localised cell) and en1996 bilingual column labels.

## Risks at rebuild
`ClauseId::text` needs `ClauseId` method only (no new import); `designator`/`locator_text` are used by full path `semio_s_plugin_norm_compliance::`; `entry.clause` types are `&'static str` (NdpEntry, CaseEntry). Appended "- ready for rebuild: i18n fix v2" to the build-en1990..en1999 latest.md.
