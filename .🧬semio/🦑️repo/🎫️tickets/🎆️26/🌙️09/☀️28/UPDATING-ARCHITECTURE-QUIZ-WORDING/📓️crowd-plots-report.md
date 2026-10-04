# The others show after submission, on request before, and everything is plotted

Revision 2026-10-02 of the quiz product (design §20). Requirement: by default the results of the others show only
after a quiz is submitted; the learner can explicitly ask to see the distribution before; everything is plotted,
nicely and visually.

## What a learner sees now

- **In a run** nothing of the others shows. Below the task's interaction stands one button, "Show it now", with the
  line "You see what the others answered once you have submitted this quiz." Pressing it opens the task's figure
  below (nothing above it moves) and turns the same button into "Hide it again". The ask holds for that quiz until the
  run is submitted, voided or closed elsewhere, or the learner changes; the next run starts closed again.
- **On the results** (the run is submitted) the others show without an ask: beside the score a histogram of how all
  runs scored with the learner's own bin in the active colour, and below every task's table the task's answer figure
  with the learner's answer (●) and the correct one (✓) marked, beside it the histogram of the scores all runs reached
  on that task (per dimension for a matching). The "Everyone" column of the result tables is gone.
- **On a quiz's page** the card "What everyone answered" is open once the learner has submitted a run of the quiz
  (score histogram with the learner's best, a figure per task of the learner's sheet); before that it is locked with
  the same button.
- **Settings**: "Others' answers: Never · After I submit · Always" replaces the checkbox "Show what others think";
  the default is "After I submit".
- The introduction of the architecture quizzes says so: "once you have submitted a quiz you see how everyone answered
  and scored – earlier only if you ask for it – and while you play the others can see what you answer."

## The figures

One form per job, one hue (dataviz method: form first, colour last):

- **Answers of a task**: the data is a grid (items × categories, places or values), so each item is a row of small
  columns — one per category in sheet order, per place 1…m of a sorting, or per value anyone gave, ascending. A column
  is as tall as the share of the item's submitted answers under that key; its percentage stands on its cap. The others
  are drawn in a quiet tone (ink mixed half with the surface), the learner's own answer in the active colour
  (emphasis instead of a categorical palette), and never by colour alone: ● marks the own answer, ✓ the correct one
  (results only), and a dot in the presence colour of every learner who thinks it right now sits under the baseline.
  A sorting row ends with the average place; a sorting lists its rows in the learner's own order, so the own marks
  run down the diagonal.
- **Scores**: a histogram over the ten score bins with a percent axis (0, 50, 100), counts on the caps, the learner's
  own bin in the active colour and "● You: 98.5 %" in the caption.
- Every figure is a real table or list: each cell or bin says its share, count and marks in words ("75% (3 of 4), your
  answer, thinking this now: 2"), which is also its tooltip; nothing is a live region. Every plot has one fixed height
  and every table fixed columns, so a figure has the same size before the crowd is known and after. Columns rise once
  (240 ms) unless the device asks for reduced motion; with forced colours they are `CanvasText`, the own one
  `Highlight`. The paint reaches 3 : 1 against the surface in both appearances (test with `colord`).
- A matching whose quantity keeps one unit names it once in the caption ("U-value (W/(m²·K))") and heads the columns
  with bare numbers; found by looking at the real heating quiz, where "0.15 W/(m²·K)" wrapped in every column head.

## What changed

Client (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`):

| File | Change |
|---|---|
| `🔨️modules/📊️plot/🟦️.tsx` (new) | `Column` mark, `columnShare`, `peakOf`, `formatShare` |
| `🔨️modules/🗳️crowd/🟦️.tsx` (rewritten) | `OTHERS_CHOICES`, `crowdGate`, `crowdShown`, `CrowdDoor`; models `answerFigure`, `scoreFigure`, `crowdPlace`, `livePlace`; components `AnswerFigure`, `ScoreFigure`, `TaskFigures`. The crowd lines (`CrowdChoices`, `CrowdPosition`, `CrowdSource`, `CrowdProvider`, `chooseCrowd`, …) are gone |
| `🔨️modules/🧭️session/🟦️.ts` | `QuizState.asked`, events `crowd-asked` / `crowd-unasked`, `askCrowd`, `unaskCrowd`; a closed run and a new learner end the ask |
| `🔨️modules/🎛️preferences/🟦️.tsx` | `others: "never" \| "submitted" \| "always"` replaces `showAnswers`; segmented choice |
| `🔨️modules/▶️run/🟦️.tsx` | door and figures below the interaction |
| `🔨️modules/🏁️results/🟦️.tsx` | score histogram in the summary, `TaskFigures` below every table, no "Everyone" column |
| `🔨️modules/📖️quiz-page/🟦️.tsx` | crowd card: door, score histogram, a figure per task |
| `🔨️modules/🗂️classification`, `↕️sorting`, `🃏️matching` | the crowd lines under the items are gone |
| `🔨️modules/🌐️i18n/🟦️.ts` | wording EN/DE (`quiz.crowd.*`, `quiz.preferences.others*`) |
| `🎨️.css` | `.quiz-column*`, `.quiz-plot*`, `.quiz-histogram*`, `.quiz-figures`, forced colours; the `.quiz-crowd*` rules are gone |
| `🟦️.tsx` | exports; `others` handed to run, results and home |

Tests and fixtures (`🧰️framework/🛍️products/❓️quiz`):

- `🧫️fixtures/💭️crowd-client/🔣️.json` v2: the truth table of the gate (36 rows), ten answer-figure vectors (all three
  kinds; submitted only, thinkers only, both, own answer, result), score-figure vectors, shares, live places. Written by
  `crowd_plots_fixture.py` in this ticket folder (kept as input).
- `🧪️tests/💭️crowd-client/🟦️.tsx` rewritten: gate, ask lifetime (`evolveQuizState`), door (one button keeps focus),
  figures against the vectors with `lodash` / `d3-scale` / `colord` oracles, rendering (sentences, heights, marks,
  dots, legend, German), stillness, run / results / quiz page / preferences.
- `🧪️tests/🚶️learner-journey/🟦️.tsx`: the run asks explicitly; the results read the figure; the home backdrop is
  locked. `🧪️tests/🏠️home-grid`, `🐾️pet-companions`, `📡️presence-client`, `📇️learner-pages`: new preference and state
  field. `🧪️tests/🌗️contrast-states` + fixture: forced-colour rules of the columns; its selector reader now reads
  attribute-presence selectors and child combinators.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `📊️plot` is a member of modules.

Site (`🎓️teaching/🏛️architecture/❓️quiz`): introduction wording in `🔣️.json`; the end-to-end spec
`🧪️tests/👥️shared-presence/🟦️.ts` follows the new behaviour (nothing by default, ask, dot of the other learner,
quiz page asked and hidden again, results figure with own and correct); the learner driver
`🎭️e2e/🚶️learner/🟦️.ts` reads the rows of the result tables only (`table:not(.quiz-plot)`) — the figures are tables
too, and every spec that checks the per-item feedback would otherwise have counted their rows. Proctor README: who
sees the crowd is the client's gate.

Core and proctor (schema, both cores, vectors, tally): see `📓️crowd-distributions-core-report.md`.

## Decisions

- **The gate is client-side.** The learner may ask for the distribution at any time, so the proctor has nothing to
  withhold; `quiz.crowd` stays an open read. The crowd is still fetched when a run opens, so the ask works at once and
  through a connection shortage.
- **The ask is ephemeral local-only and per quiz**, not a preference: it is a wish about this run. The standing wish
  is the preference (`always`).
- **One figure for both crowds.** Before, a run showed the live drafts *or* the submitted runs. Now the columns are
  always the submitted runs and the learners thinking along are dots on the same figure, so asking for "the
  distribution" never swaps a distribution of hundreds for two live drafts.
- **Bars, not a heat map and not stacked colours.** Length is read more exactly than shade, needs no palette (so no
  categorical colours to validate against an unknown theme), and works in forced colours.
- **The figures live in the quiz target** next to the spider diagram (`🕸️radar`), not in the framework's UI elements:
  the marks are generic (`📊️plot`), but an element of the design system needs its own stories and contract; promoting
  `Column` there is a follow-up once a second product draws columns.
- The published thinking drafts are unchanged: a learner's answers are still shared live; who sees them is decided on
  the viewer's side.

## Verified

Gates, as run by the lead session on 2026-10-02 after the last edit of each area:

| Command | Result |
|---|---|
| `bun ./📜️script.ts test` in `🎯️targets/⚛️react/📦️packages/🟦️typescript` (`@semio-tech/quiz-react`) | 20 files, 639 tests passed in the last run (14:10, with the other sessions' new tests); 19 files, 613 passed at 13:05; in between it was red from other sessions' work in flight, see "Not mine" below |
| `bun ./📜️script.ts typecheck` in the same package | 0 errors |
| `bun nx run @semio-tech/quiz:test` | 10 files, 297 passed |
| `bun nx run @semio-tech/quiz-rs:test` | 107 passed |
| `bun nx run @teaching/proctor:test` | 87 unit, 15 conformance, 17 end-to-end passed |
| `parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `executed=108 passed=108 parity=108/108` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` |
| `bun nx run @teaching/architecture-quiz:typecheck` | passed at 13:10 |
| `bun ./📜️script.ts test-e2e dev --project presence --no-deps` in the site package (throw-away stack, newly built proctor, Chromium) | `1 passed (1.1m)` — `🧪️tests/👥️shared-presence`: nothing of the other learner's answer by default, the ask, the other's dot on the answer given, no item moved, the quiz page asked and hidden again, the results figure with own and correct |
| `bun ./📜️script.ts test-e2e dev --project phone --no-deps` | `1 passed (58.9s)` — `🧪️tests/📱️phone`: a quiz played and its results read on a phone, through the driver's `shownResults` |
| the rest of the end-to-end gate (projects `boot`, `desktop`, `shortage`, `pets`; topology `rehearsal`) | **not run against this change**: the attempts stopped for reasons outside it — the ports held by another session's gate (13:20), `useMediaQuery is not defined` at page load (13:47, the overview rework in flight), a syntax error in `🚀️deploy/🟦️.ts` (13:56), `PocScript is not defined` in the site's `📜️script.ts` (13:58), and since 14:00 `🧪️tests/🥞️layered-home/🟦️.ts` does not parse, which keeps the `desktop` project from loading ("Unexpected ]"). `🎯️quiz-runs`, `🏆️live-leaderboard` and `🔌️connection-shortage` read the results through the same driver function the phone spec passed with |

Seen running:

- In the Browser pane against the dev site (`localhost:6061`, old proctor): the locked door in a run, the figure after
  "Show it now" with the same button turned into "Hide it again", the results with own and correct marks, dark and
  light, and at 375 px (the figure scrolls inside its card, the page does not overflow).
- `crowd_plots_shots.ts` (this folder): a private throw-away stack — the newly built proctor on 8895 over its own data
  directory, the dev site on 6165 — nine simulated learners submit the physics quiz (perfect, flawed, scrambled), then
  one more learner is photographed. Its report: gate `locked` and 0 figures in the fresh run; 9 runs in the figure
  after asking; on the results 10 runs in the bins `[0,0,0,0,0,0,0,0,2,8]` with the own bin 9; cell sentences such as
  "89% (8 of 9), your answer, correct"; the quiz page open without a door after the submission and `locked` with 0
  figures for a new learner (German); 0 px overflow at 390 px; no console or page error.

Changes made after looking at the screenshots: the unit of an unprefixed matching moved into the caption; the others'
tone went from 45 % to 50 % ink (3.5 : 1 on the light surface); the figure is its own size container, the items take
`clamp(9em, 36cqi, 20em)` of it and a figure is at most `20em + columns × 9em` wide, so a two-category figure no
longer stretches across the card and a phone gives the items less; the "Avg. place" header may wrap. The end-to-end
spec first failed on its own measurement (pressing the button scrolls the page, and item boxes are read relative to
the viewport); it now scrolls the button into view before it takes the baseline.

Screenshots kept beside this report (from `crowd_plots_shots.ts`): `📸️crowd-plots-run-locked.png`,
`📸️crowd-plots-run-asked-classification.png`, `📸️crowd-plots-run-asked-sorting-light.png`,
`📸️crowd-plots-results.png`, `📸️crowd-plots-quiz-page.png`.

Not mine, seen on the way (other sessions worked on the same files the whole time):

- A later run of the react suite had 3 failures in `🧪️tests/🐾️pet-companions` (`petsChosen` expected false, received
  true): the pets work in progress in `🔨️modules/🎛️preferences`, which I did not touch there.
- At 13:45 the suite had 54 failures in `home-grid`, `navigation` and `learner-journey` and `tsc` 18 errors, all from
  `🥞️LayeredOverview` being edited at that moment (`restRect is not defined`); `💭️crowd-client` and
  `🌗️contrast-states` passed in that run too.
- While the leaderboard-period work was in flight, `home-grid`, `learner-pages`, `outbox-delivery`, `rate-limits` and
  `presence-client` were red (`state.leaderboards`, `board.period`); they were green again at 13:05.
- From the core report: `🧫️fixtures/🏅️badge-rules` is stale against the vector generator (task order after the
  "first task stays first" change), and `🧪️tests/🔁️run-lifecycle/🟦️.ts` has two type errors no tsconfig sees.
