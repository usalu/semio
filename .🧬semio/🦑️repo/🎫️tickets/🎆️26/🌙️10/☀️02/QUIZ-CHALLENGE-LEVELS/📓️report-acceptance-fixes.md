# Report: acceptance fixes of the challenge levels (2026-10-03)

Agent: acceptance fixes. Input: `📓️audit-acceptance.md` (gaps G1, G2, G4, G5, G6, G7, G8; G3 site e2e breadth is left to the
next agent), `📓️design.md` §1 rows "Where does the learner see the tolerance?", "Which challenge does a start use?", the
third revision of "Who keeps the time", and §3.1 `CLOCK_LEAD` / `acted(at, floor, now)`.

Abbreviations: `Q` = `🧰️framework/🛍️products/❓️quiz`, `R` = `Q/🎯️targets/⚛️react`, `M` = `R/🔨️modules`, `QT` = `Q/🧪️tests`,
`P` = `🎓️teaching/🛂️proctor`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `G` = the vector generator
`.🧬semio/…/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`.

## 1. What changed, by gap

### G2: clock lead cap

- Rule: `CLOCK_LEAD = 300000` and `acted(at, floor, now) = max(min(at, now + CLOCK_LEAD), floor)`.
  - TypeScript: `Q/🔨️modules/⛰️challenge/🟦️.ts`.
  - Rust: `🦀️.rs`, using `saturating_add`.
  - Both twins export `CLOCK_LEAD`.
- Lifecycle callers, both twins:
  - `start-run`: `run-started.at = acted(command.at, 0, now)`.
  - `open-task`: `acted(at, run start, now)`.
  - `record-answer`: `acted(at, opening or run start, now)`.
  - The module docs are rewritten.
- React session (`M/🧭️session/🟦️.ts` `overdue`): calls `acted(at, opened, at)`. The device decides at its own clock, so the lead never applies there.
- Python references:
  - `QT/⛰️challenge-rules/🐍️.py`: `acted` with `now`. The numpy oracle is `numpy.maximum(numpy.minimum(at, now + lead), floor)` over `uint64`.
  - `QT/🧾️learner-lifecycle/🐍️.py`: the three callers.
- Vectors, via `G`:
  - **Instants** now carry `now`. There are 16 vectors (was 8): within, at and one past the lead; four minutes ahead kept; an hour ahead lowered; a floor beyond the lead wins; behind the clock kept; the largest safe integer, lowered and at the largest clock.
  - **New learner sequences** (27, was 23):
    - `honest-clock-four-minutes-ahead-{decided-on-the-device, delivered-at-once, delivered-late}`. `G` asserts that the three verdicts are identical.
    - `a-clock-an-hour-ahead-buys-no-time`: the start and the opening are lowered to now + lead. An answer dated at the claimed opening + 1 s, delivered half an hour later, gets `time-up`.
  - **Updated sequences:** `the-clock` (an answer and an opening dated far ahead are lowered) and `the-last-instant` (a start at 2^53 − 1 starts at the lead; its answer now gets `time-up`).
  - Regenerated with `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py`. Written: `⛰️challenge-rules` and `🧾️learner-lifecycle`; the other 10 are unchanged.
- Adapters and features:
  - `QT/⛰️challenge-rules/{🟦️.ts,🦀️.rs,🥒️.feature}`.
  - `QT/🧾️learner-lifecycle/🥒️.feature`.
- Unit tests:
  - `QT/🧗️challenge-ladder` (mathjs oracle with the lead).
  - `QT/🔁️run-lifecycle`: a new "hour buys no time" case, and two cases adjusted.
  - Rust `⛰️challenge` unit test.
  - Rust `🧾️lifecycle` unit tests: opening and untimed tables, plus start, opening and answer an hour ahead.
- Proctor:
  - **e2e:** the opening that was dated an hour ahead is now dated two minutes ahead, within the lead, and is taken as claimed. A new assertion on the offline power run checks that an opening dated an hour ahead is lowered to `[sent + LEAD, received + LEAD]`.
  - **Actors unit test** (renamed `…_and_the_proctors_clock_only_bounds_their_lead`): a claim four minutes ahead is kept; an hour ahead is lowered.
- Docs that state the time rule:
  - `Q/README.md`: layout row, clock paragraph, lifecycle table, decider paragraph, Rust names, deputy paragraph.
  - `P/README.md`: the `at` paragraph.
  - `Q/🧬️schema/🔣️.json` (`Command`, `Rejection.time-up`) and the Rust `Command` doc.
  - `Q/🔮️oracles/🔣️.json` rationale.

### G1: tolerance shown in results

- `M/🏁️results/🟦️.tsx`:
  - **Per sorting, and per matching dimension, where the keys were hidden:** a line above the table, `data-tolerance`.
    - Text: "A guess counts within ×N of the true value." (logarithmic) or "…within ±d…" (linear, formatted with the quantity).
    - Computed with the core's `reach` over the presented true values in the result (`value`, `correct`), cut down to 2 significant digits.
    - Hidden when the reach is unbounded.
    - The table is `aria-describedby` that line.
  - **Per guessed item:** a line under the verdict, `data-off`. It reads "×5.3 from the true value" (factor) or "250 K from the true value" (difference).
    - Rounded **up** for a miss and **down** otherwise, so it never contradicts the tolerance shown above it.
  - New exported helpers: `deviation(guess, truth, miss, quantity, locale)` and `tolerance(values, quantity, locale)`.
- `M/📏️quantity/🟦️.ts`:
  - New `ceilSignificant(value, digits)`.
  - `formatFactor(factor, locale, round = floorSignificant)`.
- i18n (`M/🌐️i18n/🟦️.ts`), EN and DE:
  - `quiz.results.tolerance`: DE "Ein Schätzwert zählt, wenn er höchstens {{within}} vom wahren Wert abweicht."
  - `quiz.results.offBy`: DE "{{off}} vom wahren Wert entfernt".
- Tests (`QT/🪜️challenge-views`):
  - The existing results test now separates the verdict from the deviation.
  - New test: log sorting, linear sorting and log matching, in EN and DE. It checks the lines and table descriptions against an independent `factorOf` and plain arithmetic, the deviations, the miss marks, "Not answered", and that no raw key or `{{` is shown.

### G5: per-quiz remembered challenge

- `M/🎛️preferences/🟦️.tsx`:
  - `QuizPreferences.challenges: {[quiz: Slug]: Challenge}` replaces `challenge`.
  - `readPreferences` keeps only entries with a slug key and a valid challenge. An old single `challenge` value is ignored.
  - New `challengeOf(preferences, quiz)` (default `medium`) and `withChallenge(preferences, quiz, challenge)`.
  - These are re-exported from `R/🟦️.tsx`.
- `M/🏠️home/🟦️.tsx`: the page and the card use the quiz's own challenge.
- Site driver `S/🎭️e2e/🚶️learner/🟦️.ts`: `rememberedChallenge(device, quiz)` reads `challenges[quiz]`. Three small edits.
- Tests:
  - `QT/🏠️home-grid`: cards are per quiz with medium as the default; a new "remembers per quiz" test.
  - `QT/📡️presence-client`: stored maps, invalid entries, and old/odd shapes giving `{}`.
  - `QT/🐾️pet-companions`: two fixture literals.
- Docs: `Q/README.md` "Choosing".

### G4: German rendering

All of these tests assert the German strings and that no `{{` and no raw `quiz.x.y` key is shown:

- `QT/🪜️challenge-views`:
  - **Clock:** every stage (closed chip, allowed time, intro, "Uhr starten", started, running text and spoken words, 30 s, 10 s, up), and the locked note as the select's description.
  - **Hints:** a matching hint and a classification hint.
  - **Guess fields:** names, examples, the sorting and matching guides, counts, and the invalid messages (positive and any number) as alert and description.
  - **Results:** miss marks and tolerance lines, as in G1.
- `QT/🏠️home-grid`: the open-run line, the discard dialog (name, description, focus, both buttons) and the discard. The chooser already had a German test.

### G8: contrast

- Fixture `Q/🧫️fixtures/🌗️contrast-states/🔣️.json`: a new `texts` block (description extended). It lists hint, miss mark, clock at 30 s, clock at 10 s and time-up, each with its ink, ground `--base`, and the warning tint at 12 % for the low-time states.
- `QT/🌗️contrast-states/🟦️.tsx`:
  - Resolves the design-system tokens from the palette and chrome stylesheets, following their `var()` fallbacks.
  - Composites the tint in sRGB.
  - Asserts at least 4.5 : 1 with colord in light and dark.
  - Checks that the quiz stylesheet paints none of these selectors in a colour of its own, and that the tint is the declared `color-mix`.
  - Adds a sanity test of the resolved tokens.

### G7: indistinct classification

- `S/🧪️tests/🧪️catalog/🟦️.ts`, per quiz:
  - Every classification has category descriptions or axes.
  - Its medium and hard sheet tasks differ (seeds 1 to 3, core `sheetOf`).
- `Q/README.md`, the challenge chapter "Keys": one sentence on why.

### G6: capacity figures

- `P/README.md`: the gate now plays all four challenges, has not yet run on that mix, and every figure is to be re-measured. The gate was not run.

## 2. Gates

All of these were run on the final tree.

| Gate | Command | Result |
|---|---|---|
| Quiz TS core | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` | 11 files, **415 passed** |
| Core typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.core.json` | **0 errors** |
| Rust core | `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | **180 passed** |
| Rust clippy | `cargo clippy -p semio-framework-quiz --lib --tests -- -D warnings` | clean |
| Parity | in `…/🦑️repo/🔨️modules/🧪️test`: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=14 executed=129 **parity=129/129** |
| Proctor | `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` | **90 unit + 15 conformance + 21 e2e passed** (the first run failed only in the actors unit test that expected the old rule; fixed) |
| Proctor clippy | in `🎓️teaching`: `cargo clippy -p teaching-proctor --all-targets -- -D warnings` | clean |
| React | in `R/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` | 22 files, **831 passed** |
| React typecheck | same folder: `bun ./📜️script.ts typecheck` | **0 errors** (pets errors are gone too at this moment) |
| Site node tests | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 5 files, **228 passed** |
| Site typecheck | `bun nx run @teaching/architecture-quiz:typecheck --skip-nx-cache` | exit 0, 0 errors |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | clean=true errors=0 warnings=0 |
| Docstring emojis | `T/docstring_emoji_duplicates.py` | no TS or RS file listed; the Python files and `G` repeat emojis, as before |

## 3. Red or not run

- **Schema catalog:** `bun ./📜️script.ts schema generate --check` reports `schema-catalog.json` as stale. That catalog does not hold the descriptions I changed (grep finds none of them), so the staleness comes from other sessions. I did not regenerate it.
- **Not run:** the site e2e (the integration agent's run, plus G3 for the next agent), the capacity gate (G6, which needs a quiet machine) and the browser.

## 4. Deviations and decisions

1. **Rounding of the per-item deviation.** It is rounded up for a miss and down otherwise; the tolerance is rounded down.
   - So a miss always reads strictly above the tolerance, and a near guess never reads above it.
   - A plain 2-digit rounding would show contradictions such as "×7.62, not far off" against "within ×7.6".
2. **No tolerance line for an unbounded reach.** For a linear set without spread, where no guess can miss, the line is left out. The per-item difference still shows.
3. **Placement of the deviation.** It sits as a block line inside the guess cell rather than in a new column. This keeps the tables' columns and their fold widths as they are, which the adaptive-layout session owns.
4. **Reading old preferences.** `readPreferences` keeps only entries whose key is a slug, judged with the core's `isSlug`. An old `{challenge}` is ignored, with no migration.
5. **Proctor e2e.** The opening "an hour ahead" became "two minutes ahead".
   - The last-instant and one-ms-later answers relative to that opening must stay within the lead, or they would be lowered.
   - The cap itself is asserted on the offline run's sorting task, within bounds taken around the request.
6. **Scope of the contrast check.** It covers the states the brief names. The guess error message, which uses the `muted-foreground` utility, is not included: it has no quiz selector of its own to tie to the stylesheet.
7. **Catalog rule.** It checks the content (descriptions or axes) and also that the core's medium and hard sheets differ, so the rule proves its purpose.

## 5. Notes for the next agent

- **New APIs:**
  - TS: `CLOCK_LEAD`, `acted(at, floor, now)`.
  - Rust: `quiz::CLOCK_LEAD`, `acted(at, floor, now)`.
  - React: `ceilSignificant`, `formatFactor(…, round)`, `deviation`, `tolerance`, `challengeOf`, `withChallenge`.
  - `QuizPreferences.challenges`.
- **Site e2e (G3):**
  - The results cells of guesses now also contain "… from the true value". The table has a `[data-tolerance]` line before it; deviations are `[data-off]`.
  - `rememberedChallenge` now takes the quiz.
  - Any spec that seeds `localStorage` preferences must write `challenges: {quiz: challenge}`.
- **Capacity:** run `bun nx run @teaching/proctor:capacity` on a quiet machine, then replace the figures in `P/README.md`.

## 6. Files

- **Updated**
  - **Core:**
    - `Q/🔨️modules/⛰️challenge/{🟦️.ts,🦀️.rs}`, `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`.
    - `Q/🔨️modules/🧾️lifecycle/{🟦️.ts,🦀️.rs}`, `Q/🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs`.
    - `Q/🧬️schema/{🔣️.json,🦀️.rs}`, `Q/🔮️oracles/🔣️.json`, `Q/README.md`.
  - **Shared cases and vectors:**
    - `QT/⛰️challenge-rules/{🐍️.py,🟦️.ts,🦀️.rs,🥒️.feature}`.
    - `QT/🧾️learner-lifecycle/{🐍️.py,🥒️.feature}`.
    - `Q/🧫️fixtures/{⛰️challenge-rules,🧾️learner-lifecycle,🌗️contrast-states}/🔣️.json`.
  - **Core unit tests:** `QT/🧗️challenge-ladder/🟦️.ts`, `QT/🔁️run-lifecycle/🟦️.ts`.
  - **React:**
    - Modules: `M/🧭️session/🟦️.ts`, `M/🏁️results/🟦️.tsx`, `M/📏️quantity/🟦️.ts`, `M/🌐️i18n/🟦️.ts`, `M/🎛️preferences/🟦️.tsx`, `M/🏠️home/🟦️.tsx`, `R/🟦️.tsx`.
    - Tests: `QT/🪜️challenge-views/🟦️.tsx`, `QT/🏠️home-grid/🟦️.tsx`, `QT/📡️presence-client/🟦️.tsx`, `QT/🐾️pet-companions/🟦️.tsx`, `QT/🌗️contrast-states/🟦️.tsx`.
  - **Proctor:** `P/README.md`, `P/🧪️tests/🌐️end-to-end/🦀️.rs`, `P/🔨️modules/🎭️actors/🧪️tests/🔬️unit/🦀️.rs`.
  - **Site:** `S/🧪️tests/🧪️catalog/🟦️.ts`, `S/🎭️e2e/🚶️learner/🟦️.ts`.
  - **Generator:** `G`.
- **Created:** `T/check_lead_vectors.py` (ticket input) and this report.
- **Removed:** my tool output under `T/🗑️generated/acceptance`.
