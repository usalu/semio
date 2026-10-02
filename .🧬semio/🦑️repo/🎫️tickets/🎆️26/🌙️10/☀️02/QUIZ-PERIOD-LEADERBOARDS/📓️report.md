# 🏆️ Period Leaderboards — Report

Request: turn the leaderboard into four leaderboards (daily, weekly, monthly, all-time); click the tables to sort by any
column; filter by category (heating, cooling, …). Design and decisions: [`📓️design.md`](📓️design.md).

## What a learner gets

- **Four leaderboards**: Today · This week · This month · All time — a segmented choice on the leaderboard page and on
  the centre card of the overview (German: Heute · Diese Woche · Dieser Monat · Allzeit).
- **Category filter**: All, or one quiz of the catalog (Physical Understanding, Heating, Cooling, Energy Demand) — a
  second segmented choice on the page. A leaderboard of one quiz shows *Points* instead of *Total* and the per-quiz
  bests; the card names the chosen category.
- **Click to sort**: every column heading of the page's table and of the card's excerpt is a button (`aria-sort`); a
  second click turns the order around. The order holds while switching boards and falls back to rank where the chosen
  column is not shown.
- **What a period counts** is written above the table in the viewer's own time zone ("Counts the quizzes submitted
  from … until …").

## What changed underneath

| Layer | Change |
| --- | --- |
| Contract `🧬️schema/🔣️.json` (+ TS/Rust twins) | `LeaderboardPeriod`, `LeaderboardWindow`; `Leaderboard { period, quiz?, window?, rows, learners, submissions, own? }`; the leaderboard query names `period` and may name `quiz`. |
| Core `🔨️modules/👁️views` (TS + Rust) | `Transcript` (submitted runs + badges of a learner), `periodWindow` (UTC day / ISO week / month, own days-from-civil arithmetic), `BoardScope`, `standing(transcript, catalog, scope)`, `leaderboard(transcripts, catalog, board, at, caller)`; Rust `Merit`. |
| Validation | a leaderboard query's `quiz` must be a slug (`id-invalid`). |
| Proctor `🔭️projections` | `quiz.leaderboard` stores transcripts (projector revision 4 → read models rebuild at boot); `Board` keeps the transcripts plus one compact rank index per asked `(period, quiz)`, rebuilt when the clock enters another window. |
| Proctor `❓️queries` | answers the asked board at the wall clock; a quiz the catalog does not list is `404`. |
| Client session | `QuizState.board`, `leaderboards` (last answer per board), `submissions`; `chooseBoard`; only the board looked at is polled, the overall one is refreshed when submissions moved (the profile's rank). |
| Client UI | `BoardChoices`, `SortHeading`, page and card rewritten on them; home refreshes the crowds when `submissions` moves. |
| Tests | leaderboard case rewritten (new scenario `windows`, boards per vector, calendar-edge vector); unit tests in both cores, the proctor (property test: every board equals the core at instants on both sides of a day, week and month), React and Playwright. |

## Why these choices (short)

- **UTC calendar windows**, not rolling 24 h / 7 d / 30 d and not a time zone of the catalog: same board for every
  viewer, a pure function of events and instant, no time-zone database at runtime. The answer carries the window and
  the client prints it in local time.
- **Ranked in the proctor per period and quiz**: the top 100 of one order do not contain the top of another.
- **One board per query, one board shown**: the traffic of a poll is what it was before; four boards per poll would
  quadruple it.
- **`submissions`** replaces "the board's JSON changed" as the trigger for refreshing the crowds: a filtered board does
  not change with every submission, the count does.

## Verification (2026-10-02)

| Gate | Result |
| --- | --- |
| `cargo test -p semio-framework-quiz --lib` | 109 passed |
| `@semio-tech/quiz` vitest (`test long`) | 10 files, 297 tests passed |
| parity `exhaustive --owner 🧰️framework/🛍️products/❓️quiz` | 108/108 over 13 cases (the leaderboard case alone 15/15: Python reference, TypeScript, Rust, pairwise) |
| `cargo test -p teaching-proctor --lib` | 87 passed |
| `cargo test -p teaching-proctor --test end_to_end` | 17 passed (period boards, quiz boards, `404` / `400` over real HTTP) |
| `@semio-tech/quiz-react` typecheck | clean |
| `@semio-tech/quiz-react` vitest (`test long`) | 19 files, 613 tests passed |
| `@teaching/architecture-quiz` typecheck / node tests | clean / 132 passed |
| `@teaching/architecture-quiz` `test-e2e dev live-leaderboard` | passed: the four boards, the category of the played quiz and of another one, a heading clicked to sort |
| `@teaching/architecture-quiz` `test-e2e dev layered-home` | 6 passed (after the rest fix below) |
| `@teaching/architecture-quiz` `test-e2e dev` (all 34 specs, once) | 10 passed, 8 failed, 16 did not run: 2 were the rest fix below; 6 (`🎯️quiz-runs` ×3, `🗣️both-languages` ×2, `📱️phone`) fail in the results screen — doubled result rows, results wording — from another session's change in flight at the time, after the leaderboard page had passed its both-languages check |
| taxonomy `verify taxonomy report` | clean for `🧰️framework/🛍️products/❓️quiz` and `🎓️teaching` |
| Browser, own stack on 6063 / 8793, 14 seeded learners | page and card in English and German: period and category choices, window line in local time (02:00 = 00:00 UTC), *Points* column, sorting both ways, no console error, every query `200` |

Not run: `@teaching/proctor:capacity` (needs a quiet machine; other sessions were building), the `rehearsal` topology of
the e2e gate, and a second full e2e run after the other session's results change settled.

### The overview "at rest" in the e2e gate

The leaderboard card is one row taller (the period choice). In a 1440 × 900 viewport with rows on the board, its box
now reaches the spot where the identity step's *Continue* button was. The pointer stays where the last click was, the
browser reports it as over the card once home mounts, and the card shows its page clear — the designed hover reveal, but
the two specs that measure the overview at rest (`🥞️layered-home`: the grid of nine cards, the camera) had relied on
that spot being between cards. A real pointer move is no way out: the overview's camera follows every one. `enter` and
`identify` of the e2e learner (`🎭️e2e/🚶️learner`) therefore take `atRest`: the pointer is moved to the corner of the
navbar before the identity step and the step is submitted from the keyboard, so home mounts with no card under the
pointer and no pointer move. For a real learner nothing changed in kind: a card under the pointer on arrival shows its
page, as before.

## Notes for whoever continues

- The vectors `🧫️fixtures/🏆️leaderboard` and `🪪️identity-shapes` were regenerated with
  [`regenerate_leaderboard_vectors.py`](regenerate_leaderboard_vectors.py), which calls only those two functions of
  the shared generator (`../../../🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`) so that the
  fixtures other sessions were changing stayed untouched.
- [`seed_boards.ts`](seed_boards.ts) fills a dev proctor with learners and runs through the client's own envelopes.
- `.claude/launch.json` has a new row `architektur-und-technologie-quizze-beside` (site 6063, proctor 8793, own data
  folder) for a second stack beside a running one. It starts the site's script directly, because the Nx `dev` target
  pins the port to 6061. The preview launcher's `bun` on this workstation is 1.3.13, under which Vite's WebSocket proxy
  ends the dev server when a presence socket is torn down mid-handshake (`socket.destroySoon is not a function`); the
  repo's own `bun` 1.4.2 does not.
- A dev proctor that was started before this change refuses the new query (`period` is required): restart it — its
  read models rebuild on boot (projector revision 4).
- A daily test that submits a run and asks for the daily board can straddle 00:00 UTC; the proctor's end-to-end test
  compares against the window the answer carries instead of assuming it.
