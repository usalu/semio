# 🌐️ Wave 2 i18n Fixes (nothing compiled; verified by reading and by `🗑️generated/i18n-audit/fieldcov.py`)

Paths: `A` = `✏️s/🔌️plugins/📕️norm/🗿️artifacts`, `S` = `🏅️standards/🔖️1/🪆️subsets/✳️any`, `R` = `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`.

## 1. Field-meta
- The audit counts were partly stale: owners had added en1993 `storeys[]`, en1998 `site.sApR` and removed `site.seismicZone` / en1995 `members[].mCritNm` already. Re-scan now reports 0 gaps (the scanner misses rows wrapped in `ranged(..)`, and shared `CaseRow`).
- en1991 (`A/🏋️en1991/S/✏️editor/🏷️field-meta/🦀️.rs`): the 37 collection leaves have full-path rows (`floors[].area`, `roofs[].cE`, `windFaces[].cPe10`, `accidentalCases[].impact[].vehicleMass`, ...). Lookup: exact, then `[]`-templated path, then (element path only) the collection row, then top-level leaf segment. No more `path.split('[')` parent fallback. Five new id rows with distinct labels. Typo "Wärmekkoeffizient" fixed.
- en1993: 20 rows (`<collection>[].actions[].{id,loadCaseId,force}` for coldFormedMembers, platedPanels, tensionComponents, towerLegs, piles, craneRunways; `joints[].actions[].id`; `fatigueDetails[].spectrum[].id`).
- en1995: `ulsEquation` choice (6.10, 6.10ab), `cases[]` rows incl. `numbers[]`/`texts[]`.
- en1997 (`A/🌍️en1997/S/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`): 54 rows for anchors, anchor tests, embeddedWalls, pileGroups, labTests, lab points, layerWaters, with new choice tables (tendon, wall kind, lab test kind) and ranges via a local `ranged(..)` helper (lengths >= 0, angles, counts >= 1). Source-boundary: the checks crate was already near budget; ~10 KB added, re-run the scan.
- en1996: `walls[].lintels[].distanceA1M`; also removed a duplicate `position_m` field in `WallLintel` (E0124).
- Stale rows: none left (already removed by owners).

## 2. Text defects
- en1996 fire explanation: EN was not blank (full English sentence with empty tail); the real defect was the DE text being the English sentence plus "(tabellarisch)". Now `text_en`/`text_de` are two full sentences.
- 8 example titles (TS) translated: en1990, en1992, en1993, en1994, en1996, en1997 (EN also "Foundation Design Demo"), en1998, en1999. din/vdi `Demo` titles are out of scope.

## 3. Default-language assumptions
- `R/🪟️visual-windows/🧾️properties/🦀️.rs`: `si_hint(field, value, locale)` formats with the active locale ("SI:" is language-neutral).
- `R/🪟️visual-windows/✅️checks/🦀️.rs`: `ChecksFilter::admits(check, locale)` matches title, subject label and `clause_text` in the active locale plus entity, path, part, check id and raw clause family/part/section.

## 4. `LocalizedLabel` without `Default`
- Removed the derive (`🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🦀️.rs`). Construction sites fixed: `HistoryEntry` (derive removed, new `HistoryEntry::new(seq, action_id, label, kind)`; 6 test rows rewritten as `..HistoryEntry::new(..)`), the mounted command-entry close cursor (`LocalizedLabel::from_fn` filled from the taken fields), `mutation-inputs` (3 recovery fallbacks use the explicit `recorded_finding_label()`). Repo-wide scans for derive(Default) structs, `LocalizedLabel::default`, `serde(default)` on label fields and `mem::take` found nothing else. Unverified until the framework builds.

## 5. UI-path candidates (420)
Re-scanned with a stricter detector (isolated literals with no other language literal within 3 lines): of 234 first-pass hits only ~15 survive review; all others are ids, JSON paths, handle ids, symbols, or one-word partners.
- Fixed: bilingual strings crammed into one cell (`"EN / DE"` shown in both locales) in en1990/en1994/en1996/en1997 catalogue panels via new `CatalogueCell::localized(en, de)`; broken "EN empfohleniert" removed; `"s_min / s_min"` -> `"s_min"`.
- Not fixed, reported in `requests.md`: English catalogue `Err` reasons embedded in DE text (shared, ~150 sites, needs typed reason); `MutationOutcome::warning` English messages (framework fault-notice table); `Label::data("Unknown body: ..")` unreachable viewer fallback (framework-wide convention); din18599/din16798 catalogue "EN / DE" cells (out of scope).

## Build requests
`- ready for rebuild: i18n ...` appended to `🗑️generated/build-en1990..1999/latest.md` and `🗑️generated/integrate/latest.md`.

## Wave 2b (reasons, outcome messages, German-leak test) - unbuilt
1. Reasons: new `⚖️compliance/🗣️reasons` (`RULES`: ~150 rules with stable code, EN pattern with `{}` holes, DE template `{0}..`; `explain_reason`, `reason_copy`; unit tests for unique codes, hole placement, no English in any DE template). Evaluators still raise `String`; the catalogue `evaluate_case` of en1990/91/92/95/96/97 and three en1990 indeterminate sites now render `reason_copy(e.id, &reason)`. Unmapped reasons keep their text under `reason.unmapped` and fail the new per-catalogue test.
2. Outcome messages: all codes raised by `MutationOutcome` and the `norm.*` faults are covered by `norm_fault_notices()` (app-surface `🏷️ids-chrome-and-labels`), declared as `fault_notices()` on all 10 editors and 10 viewers. The framework shows the notice by code, never the English developer detail.
3. Test: `looks_english`/`german_check_leaks` (ASCII-only words, English marker present, no German marker; symbols, units, ids, numbers are not words) in compliance; `🧪️tests/🗣️german-reasons` in the en1990/91/95/96/97 catalogue crates evaluates every registry entry with empty input in EN and DE, fails on unmapped reasons and on English sentences in titles, explanations, remedies and subject labels; app-surface test checks the fault-notice table. Not yet covered: en1992, en1993/94/98/99 reports (no catalogue registry); scene descriptions; example reports.
4. Rebuilds requested in all `build-en19xx/latest.md` and `integrate/latest.md`; the framework `HistoryEntry`/`LocalizedLabel` change goes first. No rustc logs exist yet for these edits.
