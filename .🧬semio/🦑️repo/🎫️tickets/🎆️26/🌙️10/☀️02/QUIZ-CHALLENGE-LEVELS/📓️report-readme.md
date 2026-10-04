# Report — product README of the quiz (challenge)

Area: `🧰️framework/🛍️products/❓️quiz/README.md` only. Prose; no code, no `🎓️teaching/**`.

## What changed

Updated (one file, `Edit` on small hunks, 566 → 765 lines): `🧰️framework/🛍️products/❓️quiz/README.md`.
Created: this report. Removed: nothing.

| Chapter | Change |
|---|---|
| Intro | names "the four challenges of a run" among what the model holds |
| Layout | row `🔨️modules/⛰️challenge/`; sheet row reads `sheetOf(quiz, seed, challenge)` |
| Quiz | a run keeps the first task first and is played at one challenge |
| Task kinds | per kind what shows and what is asked where the keys show or are hidden; the sentence "Guesses never change the score" is gone; pointer to the chapter |
| Sheet | function of quiz, seed and challenge (Rust `sheet_of`); step 4 corrected (the first task stays first); step 5: identical draws at every challenge; table of what a sheet carries with keys shown or hidden; `seconds` on timed sheets; "never says which item a number belongs to" |
| Answer | `guesses` exactly where the keys are hidden (sorting beside `order`, matching in place of `assignments`), what is invalid, validity and completeness with `answerRejection` / `answerComplete` and their Rust names |
| Scoring | points and `RunResult { quiz, challenge, score, points, tasks }`; guessed variants of sorting and matching with the formulas; **Partial answers** on timed sheets; **Why a miss costs every pair it touches**; `undefined` only for incomplete answers on sheets that are not timed; results carry `guess`, `miss`, absent `assigned` |
| Badges | `challenge?` on `perfect-quiz` and `perfect-tasks`, the least challenge that counts |
| **Challenge levels** (new, between Badges and Run lifecycle) | rule table, rank and meets, keys, reach and miss (with the Why), hints, the clock (who keeps the time and why, `acted`, `taskSeconds`, time up), points, one open run and switching; Rust names |
| Run lifecycle | `start-run` rows (same challenge → `run-open`, other challenge → void and start, `run-started { challenge, … }`); six `open-task` rows; `record-answer` split into `unknown-task`, `task-unopened`, `time-up`, `answer-invalid`, `answer-recorded { at = t }`; `submit-run` incomplete only "on a run without a clock"; which `at` is the learner's instant; `RunState.opened`; malformed (not rejected) `challenge` / `at`; caps: one open run whatever its challenge, a task opened once |
| Leaderboards | `TranscriptRun { quiz, challenge, score, points, at }`; `Best { challenge, score, points }`; total = sum of the points of the bests; no leaderboard per challenge; learner view and run summaries |
| What the others think | draft of a guess; one crowd per quiz, guesses under the nearest authored value (tie: the smaller), unanswered tasks; `scoreBin` formula variable renamed `percent` (it collided with points) and "bins take the score, never the points"; no crowd during a run that hides the keys |
| The leaderboards on screen | a best shows as points with its challenge |
| **The challenge on screen** (new, after The leaderboards on screen) | design §5: choosing, keys shown, keys hidden, hints, the clock, showing |
| State classes | challenge of a run and task openings persisted shared; the challenge last chosen persisted local-only; the outbox keeps an opening before the answers of its task |
| While the proctor is away | table row (task openings, hints, any challenge); what the deputy knows (challenge from the sheet, `RunView.opened`, sheet for seed and challenge); new bullet **The challenge and the time**; "a run the proctor will not start" is an open run at the same challenge |
| Validation issues | `value-invalid` also names the challenge of a badge rule; no code added |

## What I ran

Nothing is executable here. Read-only checks: `git -c core.quotepath=false diff --stat` on the README, a grep of the
README for `difficult|mode|level|previously|new|as before|guesses never|score × 100|best score` (only hits left: the
heading and its link, and unrelated older text), the heading list.

## What I could not align yet

- **No core report existed** when I finished (`📓️report-core-typescript.md`, `📓️report-core-rust.md`,
  `📓️report-vectors.md` all absent). Text follows `📓️design.md`, the landed schema (`🧬️schema/🔣️.json`),
  `📓️module-name.md` (`⛰️challenge`, confirmed) and the exports of `🔨️modules/⛰️challenge/🟦️.ts`, which match the
  README names one to one (`CHALLENGE_RULES`, `challengeRules`, `challengeRank`, `challengeMeets`, `points`,
  `REACH_DECADES`, `reach`, `misses`, `TASK_SECONDS`, `taskSeconds`, `acted`, `hintsOf`). The Rust twin of the module was
  not there yet and Rust `sheet_of` still took `(quiz, seed)`: the Rust names in the README are the design's snake_case.
  A later audit must re-check against the three reports' "Deviations and decisions".
- **The client chapter is written from design §5**, nothing of it was built: `preferences.challenge`,
  `QuizSession.startRun(quiz, challenge, signal)`, `openTask(run, task, signal)`, the quoted texts ("Challenge", "Start
  the clock", "Your guess", "Not answered", "Hard · 87 % · 261 of 300 points"), the announcements at 30 s and 10 s.
- **Crowd during a run that hides the keys**: the README says the others are not offered (no figure, no button)
  and names no value of `crowdGate`; align once the client decides how the gate expresses it.

## Deviations and decisions

- Heading `### Challenge levels` as the brief names it; the body and every other chapter say "challenge" only.
- Why of the reach is worded without the site's numbers (the README is the product's): a fixed factor of 1000
  never fires among values that span three decades or fewer.
- `Best`: "the earliest of equals" is taken from the schema description (the design says "replaces only when strictly more").
- Lifecycle row order of `start-run` kept as it stood: `run-open` (same challenge), caps, then void and start (stale
  revision or another challenge). Audit against `decideLearner` if the cores check the caps elsewhere.
- Issue code of an invalid badge rule `challenge` assumed `value-invalid` (as `taskKind`, an enum checked by `literal`).
- Not carried over from the design because the README states what is, not what changed: stored format numbers,
  "stored runs without a challenge are not restored", the proctor chapter (belongs to `🎓️teaching/🛂️proctor/README.md`).

## Notes for the next agent

- The site README and catalog texts (`🎓️teaching/**`) are untouched; the anchor to link is `#challenge-levels`.
- The README was staged-modified by other sessions before; my edits are unstaged on top. Other sessions may edit it
  again: use `Edit` on small hunks.
