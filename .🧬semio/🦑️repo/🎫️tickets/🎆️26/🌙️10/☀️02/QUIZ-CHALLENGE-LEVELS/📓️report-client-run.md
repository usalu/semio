# Report — React run screen, task views, results and styles of the challenge levels

Agent: client run/task (design §5, run screen, task views, hints, clock, results, styles). `R` =
`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`, `Q` = `🧰️framework/🛍️products/❓️quiz`.

## Changed files

Created
- `Q/🧪️tests/🪜️challenge-views/🟦️.tsx` — new client suite (19 cases): every task kind at every challenge from the shared
  `icons-1*` sheets, guesses complete answers, hints (sorting log + linear, matching, classification count, announce once),
  normalised radar, locked views, the clock (start, ticks, 30 s / 10 s / up, lock without focus move, reduced motion
  stubbed on), submit of a timed run naming open tasks, untimed run unchanged, no crowd door while keys are hidden,
  results (challenge + points, guess with miss mark, "Not answered", no guess column where keys showed).
- Ticket: `inspect_icon_sheets.py` (prints the fixture sheets per challenge).

Updated
- `M/🧩️task/🟦️.tsx` — `TaskViewProps` + `hints?`, `locked?`; `DragGrip` + `locked?`; `GuessField` moved here (now
  `name`, `readOnly?`, `describedBy?`) with `exampleGuess`; `HINT_SYMBOLS`, `magnitudeText`, `HintNote`,
  `useHintAnnouncement`.
- `M/↕️sorting/🟦️.tsx` — keys shown: key per place (`.quiz-sort-key` in the `guess` grid area), move buttons/drag, keep
  flow, no guess fields, hint beside the label describing the move buttons; keys hidden: guess field per item, items order
  by guesses, no buttons/grip/drop, "Guessed: n of m"; locked.
- `M/🃏️matching/🟦️.tsx` — dimension without `cards`: no pool, one `GuessField` per item and dimension, answer
  `{kind, guesses}` (`{kind, assignments}` where cards show); hints; locked; `data-keys="hidden"` on the section.
- `M/🗂️classification/🟦️.tsx` — normalised radar when axes lack unit/min/max; misplaced-count hint under the categories
  heading describing every select; locked.
- `M/🕸️radar/🟦️.tsx` — `normalised?`: description and table say shares as percent, table without min/max.
- `M/▶️run/🟦️.tsx` — `TaskView` passes `hints`/`locked`; `useRemaining`, `clockStage`, `ClockStage`, `TaskClock`,
  internal `TaskBody`; run card "Challenge: …"; `crowdGate(…, hidden: keysHidden(view))`; timed run always submittable,
  confirmation lists open tasks.
- `M/🏁️results/🟦️.tsx` — `challengeScored` line in the summary; `Guessed` cell (guess + "≉ Far off" / "≈ Within reach");
  "Not answered" for absent `assigned`/`guess`; sorting guess column from `item.miss`; `hidden: false` to `crowdGate`.
- `M/📏️quantity/🟦️.ts` — `formatFactor`, `formatCountdown`.
- `M/🌐️i18n/🟦️.ts` (EN+DE, same positions) — task: `guessPlaceholder/Preview/Invalid/InvalidPositive` (moved from
  sorting), `farHigh`, `farLow`, `offBy`, `hintFor`, `locked`; run: `challenge`, `confirmOpen`, `clockIntro`,
  `clockAllowed`, `clockStart`, `clockStarting`, `clockLeft`, `clockUp`, `clockStarted`, `clockThirty`, `clockTen`;
  classification: `misplaced`; sorting: `moved` (+ value), `keepHint` (reworded), `keysHint`, `guessHint` (reworded),
  `guessCount`, removed `moveClears`; matching: `guessHint`, `guessCount`, `guessFor`, `guessed`, `guessCleared`; radar:
  `share`; results: `unanswered`, `miss`, `near`, removed `noGuess`.
- `R/🎨️.css` — own block at the end ("🎚️ Challenges"): `.quiz-sort-key`, `.quiz-slot > .quiz-guess`,
  `@container quiz-task (min-width: 64rem) .quiz-match[data-keys="hidden"] > .quiz-slots`, `.quiz-hint`, `.quiz-clock`
  states, `.quiz-clock-bar`, `.quiz-miss`, and its own `@media (forced-colors: active)` block.
- `Q/🧫️fixtures/🌗️contrast-states/🔣️.json` — 6 entries (hint, thirty, ten, up, clock bar, miss).
- `R/🟦️.tsx` — reexports (`GuessField`, `HINT_SYMBOLS`, `HintNote`, `exampleGuess`, `magnitudeText`,
  `useHintAnnouncement`, `TaskClock`, `clockStage`, `useRemaining`, `ClockStage`, `formatCountdown`, `formatFactor`).
- Tests: `⌨️task-keyboard` (ladder task for moves, hidden task for guesses, hidden matching case, `spoken()` helper),
  `🖼️task-icons` (also `icons-1-hard`), `📐️quantity-formatting` (countdown vs `Intl.DurationFormat`, factor vs
  `toPrecision`), `🕷️radar-geometry` (normalised case vs d3-scale), `R/🧪️tests/🎚️config/🟦️.ts` + package
  `tsconfig.json` include, taxonomy `members-of-tests` (`🪜️challenge-views`).
- Adjacent consumers, small hunks: `📐️adaptive-layout` test (MASSES gets `keys`, `quiz-sort-key` → "key", hidden-key
  order) and its fixture (`quiz-sort` order with `key`, new `quiz-sort-guessed`); `🚶️learner-journey` `answerCurrentTask`
  reads `.quiz-sort-label`.

## Gates

- `bun ./📜️script.ts typecheck` in `R/📦️packages/🟦️typescript`: 0 errors before the core fix landed; last run **2
  errors, both in the core** `Q/🔨️modules/🧾️lifecycle/🟦️.ts:133,147` (`acted` called with 3 arguments after it lost
  `now`, the fix agent's change in progress). None in the client.
- `bun ./📜️script.ts test` there (full suite, 22 files, after the core's factor `reach` landed): **747 passed, 2
  failed** — `📡️presence-client` "keeps the cursor preference on the device" and `🐾️pet-companions` "knows whether the
  learner chose" (preferences, the other agent's area, mid-change). Every suite I own or touched passes:
  challenge-views (19), task-keyboard, quantity-formatting, radar-geometry, contrast-states, task-icons, live-regions,
  adaptive-layout, translation-completeness, learner-journey.
- `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"`: 1 error, a file changed
  during the run (`🚶️learner-journey`), none about my folders.
- Not run: browser end-to-end on the dev site (the other agent's layers were still changing; nothing was observed in a
  browser).

## Deviations and decisions

1. `hints?`/`locked?` are optional props (absent = no hints, not locked) so every view keeps working standalone.
2. Hints get their own polite live region per task (a second `LiveRegion`), so a hint arriving within the 400 ms select
   debounce never cancels the move announcement; hints present when a task mounts are not announced.
3. Hint factor: `magnitudeText` shows `reach` directly — `×formatFactor(reach)` (two significant digits) on logarithmic
   scales, `formatQuantity(reach)` on linear ones, words only for an infinite reach. Symbols: ≫ high, ≪ low, ≠ misplaced.
4. "n items in the wrong category" is "Items in the wrong category: n" (no plural forms).
5. Keys-shown sorting has no guesses; the moved announcement names the key of the new place.
6. Hidden-key radar says shares as percentages ("Share of the range") — the same information as the shape, so screen
   reader users are not left with nothing; no units, ranges or values.
7. A timed task hides its prompt too until opened (design: "only its title, the time allowed and Start the clock").
   After a successful start focus goes to the task heading (the button it was on disappears); at time up focus stays.
8. Clock stages: running > 30 s, thirty ≤ 30 s, ten ≤ 10 s, up at 0; the visible clock is `aria-live="off"`; the
   status speaks "The clock runs: m:ss left", "30 seconds left", "10 seconds left", and `quiz.task.locked` at time up,
   only on entering a stage while shown. The bar shortens per second without transition (no motion rules needed).
9. Locked selects use `aria-disabled` (not `disabled`), guess inputs `readOnly`, so focus is never lost; answers after
   the deadline are also not sent (`TaskBody` checks `session.now()`).
10. Results: a miss shows "≉ Far off", a guess within reach "≈ Within reach"; "Not answered" replaces the old "No guess".
11. Low time, time up, hint, clock bar and miss are painted → forced-colours rules and fixture entries.

## Notes for the next agent

- New exports: `GuessField({id, name, quantity, value, onCommit, readOnly?, describedBy?, text, locale})`,
  `exampleGuess(quantity)`, `magnitudeText(direction, reach, quantity, text, locale)`, `HintNote({id, kind, children})`,
  `useHintAnnouncement(hints, say): Announcement`, `useRemaining(deadline, now)`, `clockStage(left)`,
  `TaskClock({seconds, left, onStart, onStarted, text})`, `formatCountdown(ms)`, `formatFactor(factor, locale)`.
- `TaskView` takes `hints?`, `locked?`. `RunScreen` uses `session.now()` and `session.openTask(run, task, signal)`.
- Site e2e (labels): sorting keys at `.quiz-sort-key`; guess fields "Guess for <item>" (sorting) and
  "Guess for <quantity> of <item>" (matching); "Start the clock"; clock text "Time left: m:ss" / "Time is up";
  confirmation "Not answered or incomplete, scored as missed:"; results "Expert · Points: x of 400".
