# Report — React rendering of the specific hints (design §8.4)

Agent: client hints. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`,
`Q` = `🧰️framework/🛍️products/❓️quiz`. Coded against §8.2; re-checked after `📓️hints-schema-landed.md` (types match).

## Changed files

Updated (small `Edit` hunks only)
- `M/🧩️task/🟦️.tsx` — removed `HINT_SYMBOLS`, `magnitudeText`; new `HINT_SYMBOL = "?"`, `compareText(hint, quantity, label, text, locale)`;
  `HintNote({id, kind: Hint["kind"], children})` renders `.quiz-hint-symbol` (aria-hidden "?") + `.quiz-hint-question`;
  `hintKey` covers the four kinds and every member that changes the question (so a changed reference or factor is announced
  again); single new hint is announced by its question, several by "New hints: n" (unchanged).
- `M/↕️sorting/🟦️.tsx`, `M/🃏️matching/🟦️.tsx` — compare hints beside the item (per dimension in matching), describing
  its move buttons / select; no more `reach` in the client.
- `M/🗂️classification/🟦️.tsx` — new exported `classificationHintText(task, hint, text, locale)`; the task-level count note is
  gone, each profile/group/category hint stands beside its item chip and describes that chip's select.
- `M/🌐️i18n/🟦️.ts` (EN + DE, §8.4 texts verbatim) — added `quiz.task.sumUnder|sumOver|ratioUnder|ratioOver|aboveUnder|aboveOver`,
  `quiz.classification.fits|together|apart|belongs|belongsBare`; removed `quiz.task.offBy`, `quiz.task.hintFor`,
  `quiz.sorting.farHigh|farLow`, `quiz.matching.farHigh|farLow`, `quiz.classification.misplaced`.
- `R/🎨️.css` — `.quiz-hint` is a flex row; new `.quiz-hint-symbol` (circle: hairline `currentColor` border, radius 50 %).
- `R/🟦️.tsx` — reexports `HINT_SYMBOL`, `compareText`, `classificationHintText` (dropped `HINT_SYMBOLS`, `magnitudeText`).
- Tests: `Q/🧪️tests/🪜️challenge-views/🟦️.tsx` (24 exact-string cases = 12 branches × EN/DE, announcement cases, German
  classification announcement; fixtures `CLIMATES`, `RATIOS`); `Q/🧪️tests/🚶️learner-journey/🟦️.tsx` (state tests expected
  `misplaced`/`magnitude`: now `hintsOf(...)` of the core, `compare` kinds, literal hints of the new shapes).

Created: this report. Removed: nothing.

## Gates (in `R/📦️packages/🟦️typescript`)

- `bun ./📜️script.ts typecheck`: exit 0, **0 errors** (pets too).
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test challenge-views translation-completeness task-keyboard contrast-states live-regions`: **124 passed** (5 files).
- `… test challenge-views --reporter=verbose`: every hint case listed and green.
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` (full): **859 passed, 0 failed** (22 files).
- Docstring emoji probe on the touched files: clean. No new directories (taxonomy untouched).
- Not run: browser/dev-site check (behaviour observed in jsdom renders only); site e2e.

## Deviations and decisions

1. German grouping is the locale's own (`Intl`): 1000 reads "1.000" in German, "1,000" in English (the owner's example
   also wrote "×1.000"); the brief's "1 000" is not what `de` produces.
2. Factor rounding to two significant digits is directional: down (`floorSignificant`) when `under`, up
   (`ceilSignificant`) when not — the shown number never tips the claim over to the truth (2.96 under → 2.9, 1/0.35 over → 2.9).
3. Orientation: `factor < 1` or `difference < 0` swaps item/other and inverts/negates; the difference is
   `formatQuantity(|difference|)` without further rounding.
4. Category description: one trailing "." is dropped before the closing "?" (catalog descriptions end with a full stop).
5. Profile value: `withUnit(formatNumber(value), axis.unit ?? "")` from the sheet category's profile — the radar's formatting.
6. The circle uses `currentColor`, so forced colours need no new rule and no contrast fixture entry (no new painted state);
   the existing `.quiz-hint` forced-colours rule stays.
7. A hint is "new" (announced) when its question would change, not only when its item changes.

## Notes for the next agent

- Site (`🎓️teaching/🏛️architecture/❓️quiz`) still asserts the old hints: `🧪️tests/⛰️challenge-levels/🟦️.ts` lines ~1-10,
  119 (`[data-hint="low"]` "Key far too small"), 240-241 ("Card far too …"), 277-290 (`misplaced`, "Items in the wrong
  category"); `🎭️e2e/🚶️learner/🟦️.ts` `shownHints` docstring (kinds are now `compare|profile|group|category`, always on an
  item); `README.md` line 33 describes the old hints. Selectors: `.quiz-hint[data-hint="<kind>"]`, question text in
  `.quiz-hint-question`, symbol `.quiz-hint-symbol`.
- Signatures: `compareText(hint: CompareHint, quantity: Pick<Quantity, "unit"|"prefixed"|"additive">, label: (id) => string, text, locale)`,
  `classificationHintText(task: SheetClassificationTask, hint: Hint, text, locale)`, `HintNote({id, kind, children})`.
