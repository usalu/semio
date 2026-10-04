# Report — fixes of the React run-layer audit (`📓️audit-client-run.md`)

Agent: client run/task fixes. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`,
`Q` = `🧰️framework/🛍️products/❓️quiz`. Finding 2 (stale hints in the session reducer) is left to the state-layer agent.

## Findings → fixes

| # | Fix |
|---|---|
| 1 | `M/📏️quantity` `typedExponent`: parsing reads every SI prefix (`h`, `da`, `d`, `c` added to `PREFIX_EXPONENTS` only; the display stays on multiples of three). Order: unit in its own case → prefix + unit (unit any case) → unit in another case → lone prefix; a lone prefix that is also the unit in another case (`5 M` on m, `5 G` on g) reads as nothing (invalid message). 11 new shared parse vectors, description rewritten. New oracle test: lengths `Intl.NumberFormat` writes in its unit style (mm, cm, m, km) read back. |
| 3 | `floorSignificant(value, digits)` (new, exported) cuts the shortest round-trip decimal; `formatFactor` and the linear amount in `magnitudeText` use it: a reach of 1.96 reads ×1.9, 66.65 K reads 66 K. Tested by bounds (shown ≤ factor < shown + one step, ≤ 2 digits, 438 cases) and by plain-arithmetic `factorOf` in `🪜️challenge-views` — no shared rounding code. |
| 4 | The clock `<p>` is `role="timer"` (implicit live off, no `aria-live`); visible `m:ss` is `aria-hidden`, an `sr-only` twin says "Time left: 1 minute, 5 seconds" (`formatDuration`, new: CLDR unit names via `Intl.NumberFormat` unit style + `Intl.ListFormat`, tested against `Intl.DurationFormat` long style). Spoken start status uses words too. |
| 5 | Step chips of timed tasks show `ClockState` (⏸ Clock not started / ⏱ Time left m:ss + words / ⌛ Time is up), ticking on its own. `ClockWatch` (run card, one timer to the next deadline, `visibilitychange`) says "Task 1: time is up" once for a task that is not shown; silent for deadlines already past when mounted. |
| 6 | Keys-shown sorting: the place cell is read as "Place 3: 2.5 kW" (`quiz.sorting.place`, `sr-only`), the key cell is `aria-hidden`; grid parts unchanged (adaptive-layout rows still match). |
| 7 | Hints name the key/card: "Key far too small (more than ×150)", "Card far too large (…)"; DE "Wert viel zu klein (mehr als …)", "Wertkarte viel zu groß (…)". `quiz.task.farHigh/farLow` removed, `offBy` reworded; `magnitudeText(side, …)` takes the worded side. |
| 8 (audit 8) | `useHintAnnouncement(hints, say, text)`: one new hint → its sentence; several → "New hints: n". |
| 9 | `GuessField` turning read-only over an uncommitted text: commits it once more if it parses (the run's `answer` keeps the `now() < deadline` guard), else `onDropped`. `TaskBody` records a refused late answer as dropped too, shows "Your last entry was not saved." under the clock and says "Time is up: … Your last entry was not saved." Views no longer refuse a settle commit themselves; they only stay silent while locked. |
| 10 | Hidden-key sorting: the answer is ordered at every commit, but the rows stay put while focus moves from field to field of the list (`held` order); they reorder on Enter (focus stays on that field) or when focus leaves the list. |
| 11 | Locked selects/buttons/fields are described by a hidden `LockedNote` ("Time is up: …"); classification and matching selects announce it when the arrow keys try to change them. |
| 13 | Results: a timed sorting nobody ordered shows "Not answered" in every "Your position" cell and no verdict. |
| 14 | `quiz.results.near` "Not far off" / "Nicht weit daneben". |
| 15 | Confirmation lists open timed tasks with their `ClockState`. |
| i18n | DE `quiz.results.credit` "Beurteilung" (not "Punkte"; "Anrechnung" is banned by the completeness test, "Anteil" is the radar's share); EN `clockIntro` "The task shows …"; DE `quiz.run.incomplete` "Unvollständig". |
| style | `timed` from `challengeRules(...).timed`; `describedBy` of `GuessField` is now used (locked note); stale docs (`formatPoints`, run header) fixed; duplicate docstring emojis removed in run, task, sorting, matching, classification, radar, results, quantity and the challenge-views test (probe clean). |
| perf | Ticking lives in `TaskClock`/`ClockState`; `TaskBody` uses `usePassed` (one render at the deadline) and a memoised `TaskView` (`SteadyTaskView`), so the status speaking does not re-render the task. Tested by counting renders. |
| client-interface §5a | `@container quiz-card` 44rem: `.quiz-kinds[data-options="4"]` 2 columns, 64rem: 4 columns (my CSS block). |

## Files

Updated: `M/▶️run/🟦️.tsx`, `M/🧩️task/🟦️.tsx`, `M/↕️sorting/🟦️.tsx`, `M/🃏️matching/🟦️.tsx`, `M/🗂️classification/🟦️.tsx`,
`M/🕸️radar/🟦️.tsx` (docstring emojis only), `M/🏁️results/🟦️.tsx`, `M/📏️quantity/🟦️.ts`, `M/🌐️i18n/🟦️.ts`, `R/🎨️.css`
(challenge block), `R/🟦️.tsx` (reexports `floorSignificant`, `formatDuration`, `LockedNote`, `describedBy`, `usePassed`),
`Q/🧫️fixtures/📐️quantity-formatting/🔣️.json`, tests `Q/🧪️tests/{🪜️challenge-views,⌨️task-keyboard,📐️quantity-formatting,📢️live-regions}/🟦️.tsx`,
site e2e `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/⛰️challenge-levels/🟦️.ts` (one assertion: "Key far too small").
Created: ticket `duration_words_probe.ts`. Removed: nothing.

## Gates (run in `R/📦️packages/🟦️typescript`)

- `bun ./📜️script.ts test challenge-views task-keyboard quantity-formatting live-regions translation-completeness adaptive-layout contrast-states`: **195 passed, 1 failed** (7 files).
- `bun ./📜️script.ts test` (full): **791 passed, 7 failed of 798** (22 files). Failures are not in this area: `🗣️translation-completeness` "explains every proctor rejection" (core added rejection `already-opened`, `REJECTION_LABELS` lacks it) and 6 × `🫡️deputy-decisions` (`id-invalid` / missing views — core `start-run` now needs `at`, state layer mid-change).
- `bun ./📜️script.ts typecheck`: **22 errors, none in my files**: 11 in `🐾️pets` (`🎥️projection` 3, `🎪️stage` 1, `👥️population` 2, `💞️sociability` 2, `🕰️clock` 2 — pets session), 11 in `🧪️tests/📬️outbox-delivery` (`start-run` without `at`), 1 in `M/🌐️i18n` (`REJECTION_LABELS` misses `already-opened`).
- Not run: site end-to-end (`⛰️challenge-levels`), browser check.

## Deviations and decisions

1. "Commit an uncommitted guess at time up if still within the deadline": the lock itself comes from `now() >= deadline`, so the settle commit is always late in practice and is reported as lost; the path exists (and the guard is tested) for a commit that races the lock.
2. Lone-prefix rule applied only where it collides with the unit (`5 M` on metres); `5 M` on watts stays valid (existing vector).
3. The key's accessible text sits on the place cell ("Place 3: …"), the key cell is `aria-hidden`, so the row reads place → item → buttons.
4. Audit 12 (summary on two lines) kept: "261 of 300 points" needs a plural form, which the bundle forbids; `challengeScored` belongs to `⛰️challenge`.
5. Step-chip time reads in words for screen readers and changes each second; chips are buttons, not live regions.
6. Contrast of `--color-warning` against its 12 % tint is not tested (needs resolved theme tokens; text on the tint is foreground ink).

## Notes for the next agent

- `REJECTION_LABELS` needs `already-opened` (state layer).
- New signatures: `TaskClock({seconds, deadline, now, onStart, onStarted, announce, text, locale})`, `usePassed(deadline, now)`,
  `magnitudeText(side, within, quantity, text, locale)`, `useHintAnnouncement(hints, say, text)`, `GuessField({…, onDropped?})`,
  `TaskViewProps.onDropped?`, `LockedNote({id, text})`, `describedBy(...ids)`, `formatDuration(ms, locale)`, `floorSignificant(value, digits)`.
- Site e2e: the step chip clock is `[data-clock-state]` (not `data-clock`), the clock is `role="timer"`, hints read "Key far too …"/"Card far too …", moves "is now in place n of m".
