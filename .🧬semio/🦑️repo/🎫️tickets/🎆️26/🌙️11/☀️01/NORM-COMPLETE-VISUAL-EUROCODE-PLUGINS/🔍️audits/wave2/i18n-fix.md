# 🌐️ Wave 2 i18n fix (I18N-FIX)

No build, no cargo, no git writes. Python ran under `nice -n 19 ionice -c3`. Verified: `python3 -I ⚖️compliance/🧪️tests/🌐️i18n-sinks/🐍️.py` prints `ok`; spec-trace codegen `--write` regenerated the ten twins and `--check` passes for all but en1995 (four fragment names missing in the sources, pre-existing). Nothing compiled: every Rust edit awaits `- ready for rebuild: i18n fix` (appended to integrate and build-en1990..en1999).

## 1. Spec-trace
- Schema `⚖️compliance/🧭️spec-trace/🧬️schema/🔣️.json`: new definition `Text {en, de}` (both required, non-empty); `note` and the not-modelled `reason` use it. The field keeps the name `note` (the audit calls it description); the UI column is now `description` ("Description / Beschreibung").
- Rust `spec_trace/🦀️.rs`: `SpecText {en, de}` (const `new`, `resolve(&Locale)`, `shorter_than`), `SpecTrace.note: SpecText`, `Realisation::NotModelled(SpecText)`, `Realisation::describe(&Locale)`. `problems()` reports a missing language. Unit test added; the old test fixtures updated.
- Codegen `🧰️codegen/🐍️.py`: `text_rust`, verifies en != de and both reasons >= 20 chars.
- Content: 683 rows over ten artifacts, 263 existing EN texts translated to DE (EN verbatim), 420 blank rows written in EN and DE from spec text and realisation (Sonnet sub-agents, files `🗑️generated/i18n-fix/notes-enXXXX.json`, merged by script; all ids covered). en1995 (68 rows) was written from spec headings, clauses and realisation only: owner review requested.
- UI: `📇️registry/…/📚️catalogue-and-inspection/🦀️.rs::spec_trace_table` builds `Localized` cells for description, realisation detail (not-modelled reason) and clause (designator localised), so the active language renders.

## 2. Clause labels and NDP locators
Instead of rewriting 141 call sites, the canonical designator is stored once and rendered per locale: `designator()`, `DESIGNATOR_WORDS`, `annex()`, `ClauseId::text(&Locale)` and `locator_text()` in `⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs` (tests `Annex B` -> `Anhang B`, `EN 772-1 Table A.1` -> `EN 772-1 Tabelle A.1`, whole words only). `clause_text` of the checks window uses `ClauseId::text`. The 24 `.at("Annex B tabulated data")` rows are now `.at("Annex B")` (JSON source and twin), plus `Table 7.11 note` -> `Table 7.11`. Clause parts that were English prose were replaced by section numbers (en1991 hazard checks x2, en1990 x2, en1995 x1). `ClauseId::Display` stays English for logs.

## 3. Reasons, steps, texts, errors
- en1997 gate reasons: the shared table `🗣️reasons` already coded 33 of the 41; 8 rules (`series.*`) were added, so every gate reason has a DE template (`reason_copy`).
- Steps: en1991 `Outcome.steps_de` plus `.steps_de()` (`.steps()` sets both for formulas), 49 prose calls given a DE text; same for en1990 (12 outcome steps), en1992 (19 incl. fastenings lever-arm tuple) and en1996 (5 incl. Annex E source); the checks consume `steps_de` for the German explanation. en1993 storey-checks no longer prints the English `source` in the German explanation.
- Check texts that the audit listed in en1993 (hollow-joint, built-up, `may be neglected`) and en1998/en1999 were already paired (false positives of the heuristic scan).
- Snapshot titles (`snap.title`, `Record B`) are document data (user-owned names of fixtures), not UI sinks; the example titles shown in the UI are bilingual in `📚️examples`. Left as is, and the test does not flag them.
- Mechanics: `MechanicsError::Invalid(MechanicsText {en, de})` built by `MechanicsError::invalid(en, de)`; all 29 sites in section, statics, modal, ltb and elements converted with DE.
- Registry `ui_error` message is bilingual ("... / Zulassung der Norm-Oberfläche fehlgeschlagen"); `PluginAssemblyError.message` is a framework string type.

## 4. No default language
`Lang` moved out of the `config_choices!` macro (which derives `Default`): no `#[default]`, new `Lang::of(Locale)` with an exhaustive match; the three `if locale == Locale::De { Lang::De } else { Lang::En }` sites (checks window x2, toolbar, diagrams window) use it. Remaining `Lang::En` are explicit test inputs.

## 5. Repo-level test
`⚖️compliance/🧪️tests/🌐️i18n-sinks/{🐍️.py,🥒️.feature}` (text-based like handle-paths, sink-specific instead of the heuristic scanner so it has no false positives): spec-trace JSON and twin, gate reasons vs the reason table, English steps prose without `steps_de`, mechanics `invalid`, default-language enums and locale fallbacks, clause labels and annex/table locators. Run: `python3 -I ⚖️compliance/🧪️tests/🌐️i18n-sinks/🐍️.py`.

## Risks to check at the rebuild
Unbuilt edits: `SpecText` import of `Locale` in compliance, `designator` re-export through `core` glob, `semio_s_plugin_norm_ndp::NdpSource` path in core, `Lang` derive macros (`value_derive`) outside the macro, `.steps_de(..)` chained after `.into())` in 40+ sites, `MechanicsError::Invalid` consumers outside mechanics (none found), en1991/en1990/en1996 `Outcome` struct literals other than `make` (none found). Unfixed: NDP row symbol/description prose in the generated tables is not covered by the test (not rendered).
