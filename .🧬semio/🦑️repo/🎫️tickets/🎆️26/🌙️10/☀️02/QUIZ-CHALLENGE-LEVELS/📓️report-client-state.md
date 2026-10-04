# Report — React client: state, wire and page layer of the challenge levels

Agent: client state (design §1, §2, §5). `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`,
`T` = `🧰️framework/🛍️products/❓️quiz/🧪️tests`. Interface for the run/task agent: `📓️client-interface.md` (written first,
updated at the end).

## Changed files

Created
- `M/⛰️challenge/🟦️.tsx` — `CHALLENGE_LABELS`, `CHALLENGE_HINTS`, `challengeScored(challenge, points, text, locale)`,
  `ChallengeChooser` (fieldset of four radios in the identity step's pattern, `data-options="4"`).
- Ticket: `crowd_gate_hidden_fixture.py` (rewrites the crowd-client fixture by text edits: 72 gates with `hidden`, sheet
  `challenge: "medium"`, sorting `keys`, schema `v3`; idempotent; writes through a staged file + `os.replace` because the
  fixture is memory-mapped by running vitest processes on this host).

Updated
- `M/🧭️session/🟦️.ts` — `startRun(quiz, challenge, signal)`, `openTask(run, task, signal)`, `answer` stamps `at`, public
  `now()`; client events `task-opened`, `run-hinted`; `openChallengeOf(state, quiz)`; restored runs need
  `sheet.challenge`; easy-run hints (deputy right after the answer; without deputy re-read once no answer of the run waits);
  `time-up`/`task-unopened` → notice + re-read; `decidable()` keeps the deputy from opening/submitting a run known from
  its listing alone (audit note b); docstrings.
- `M/🫡️deputy/🟦️.ts` — `RunState.challenge`/`opened` from the views, summary fallback with `challenge`/`points`,
  `sheetOf(quiz, seed, challenge)`, new `hints(state, run)`; docstring on `recorded` (audit note a).
- `M/📮️outbox/🟦️.ts` — restores `start-run` only with a valid challenge, `open-task`/`record-answer` only with a
  Timestamp `at`; docstring.
- `M/💾️persistence/🟦️.ts` — `isChallenge`, `isTimestamp`.
- `M/🎛️preferences/🟦️.tsx` — `challenge` (default `medium`, not in the panel).
- `M/🗳️crowd/🟦️.tsx` — `crowdGate(…, {asked, submitted, hidden})`, `keysHidden(view)`; matching own mark of a guess =
  nearest column on the quantity's scale; `assignments?` fix.
- `M/📖️quiz-page/🟦️.tsx` — actions name the challenge, chooser on the page, discard `Dialog`, open-run line, best as
  points, hidden gate on the page, `ScoreFigure own = best.score`.
- `M/🏠️home/🟦️.tsx` (two props), `M/📇️profile/🟦️.tsx` (Challenge + Points columns, `Records fold` 46 → 60, resume
  names challenge), `M/🏆️leaderboard/🟦️.tsx` (best cell "261 (Hard)", sort by points, fold 68 → 80),
  `M/🚏️navigation/🟦️.tsx` (run place "Quiz (Hard)"), `M/🌐️i18n/🟦️.ts` (EN+DE: group `challenge`; keys in `home`,
  `learner`, `nav`, `leaderboard`, `rejection`; `REJECTION_LABELS`), `R/🟦️.tsx` (reexports).
- Tests: `T/📬️outbox-delivery`, `T/🫡️deputy-decisions`, `T/🚶️learner-journey`, `T/🏠️home-grid`, `T/💭️crowd-client`,
  `T/📇️learner-pages`, `T/🚏️navigation`, `T/🚦️rate-limits`, `T/📡️presence-client`, `T/🐾️pet-companions`;
  fixture `🧫️fixtures/💭️crowd-client/🔣️.json`.

## Gates (exact commands, in `R/📦️packages/🟦️typescript`)

- `bun ./📜️script.ts test` (vitest, all React suites): **22 files, 749 passed, 0 failed** (last run).
- `bun ./📜️script.ts typecheck`: **0 errors in quiz-react files**. The last run shows 4 errors, all in
  `🐾️pets/📦️packages/🟦️typescript/🟦️.ts` (duplicate exports `Pointer`, `PressPhase`, `TIERS`, `Tier` — pets session
  mid-change); the run before showed 2 in core `🧾️lifecycle/🟦️.ts` (`acted` arity, the fix agent mid-change), the one
  before that 0. Not mine; not touched.
- `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` (repo root): clean, 0 errors.
- New cases (selection): challenge per sheet + resume at same challenge; switch voids without notice and drops the queue;
  expert clock via the session clock seam (`task-unopened`, opening once, in-time answer, `time-up` with re-read,
  submission with unanswered tasks, best by points); expert offline (deputy openings, queue order open-task before
  answers, proctor's `opened` equals the device's); hints on easy without deputy (after delivery) and with deputy (at once,
  proctor away); medium never hints; hard guesses → 300 points, best by points across challenges; deputy refuses to decide
  from a listing alone; restore only with challenge; outbox restore/ordering/wire; deputy vectors now include
  `hints-on-easy`, `points-by-challenge`; chooser radios/labels/keyboard (Space) in EN and DE; discard dialog (keep,
  Escape, discard); crowd gate truth table with `hidden` (72); card compactness in EN/DE; leaderboard best by points.

## Deviations and decisions

1. **No plural forms** forbid "{{par}} points": points read "Points: 261 of 300" (`quiz.challenge.scored`, for results),
   the card fact "Best: 261 of 300 (Hard)", the runs table "261 of 300", the leaderboard cell "261 (Hard)".
2. **Card fit** cannot be measured in jsdom; the home-grid test now asserts in EN and DE: no chooser on the card, ≤ 2
   actions of ≤ 20 characters, facts ≤ 34 characters (the longest new strings are 20 and 33). Action labels are shorter
   or equal to before ("Start (Medium)" vs "Start quiz"). Rerun `QUIZ-ADAPTIVE-LAYOUT/layout_survey.ts` to confirm.
3. **Page with an open run at another challenge**: primary = "Start (chosen)" which opens the alert dialog; "Resume
   (open)" stays in the footer. Choosing a radio changes only the remembered challenge (no dialog on arrow keys).
4. **Crowd gate** `hidden` turns the gate `off` at `run` and `quiz` (an open run that hides the keys), never at `results`.
5. **Hints without deputy**: re-read only when no command of the run waits, so hints match the delivered answers.
6. **Audit (a) `recorded`**: the deputy never decides `record-answer` (session `decision()`), so the cap is only ever
   judged by the proctor at delivery (`answers-exhausted` → notice). No faithful count is available from views; left as
   is and documented. **Audit (b)**: `decidable()` routes `open-task`/`submit-run` of an open run without a held view to
   the proctor (waits, cancellable); `openTask` loads the view first.
7. Matching crowd "own" for a guess = nearest column (log/linear distance, ties to the smaller), oracle lodash `minBy`.
8. Profile and leaderboard `Records fold` widened for the new columns (60, 80 em) — the adaptive session may tune them.

## Notes for the next agent

- See `📓️client-interface.md` (signatures, state, `crowdGate` facts, texts, CSS request `.quiz-kinds[data-options="4"]`).
- The run screen must pass `hidden: keysHidden(view)` (it does now — typecheck is clean there).
- Results texts in `🚶️learner-journey` still assert "Your score: 100%" (owned by the run/results agent's output).
- Design revision of 2026-10-03 (`acted(at, floor)`, factor reach): my code calls neither; tests use the core's
  `hintsOf` as the expected value, so they follow the core.
