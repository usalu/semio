# 🏆️ Period Leaderboards — Design

Request: turn the leaderboard into four leaderboards (daily, weekly, monthly, all-time), every table sortable by clicking
its columns, filterable by category (the quizzes of the catalog: heating, cooling, …).

Supersedes the leaderboard parts of `../../../🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md` (§9a wire mapping
of `quiz.leaderboard`, §18 `{ rows ≤ 100, learners, own? }`).

## 1. Decisions

| Question | Decision | Why |
| --- | --- | --- |
| What is a period? | A calendar window in UTC that contains the proctor's clock: the day, the ISO week (Monday 00:00) and the month; all-time has no window. Half-open `[from, until)`. | Same board for every viewer wherever they are, a pure function of `(events, instant)`, no time-zone database (none in Rust's std, an external runtime dependency otherwise). The answer carries the window, so the client shows its bounds in the viewer's own zone. A rolling 24 h / 7 d / 30 d window would mix yesterday afternoon's class into this morning's board. |
| What is a category? | A quiz of the catalog (`heating`, `cooling`, `demand`, `physics`). | It is what the catalog groups by and what a run belongs to. |
| Where is a board ranked? | In the proctor, per period and per quiz. | A client holds the top 100 of one order only; the top of another period or of one quiz is not inside it. |
| What does a board count? | The submitted runs inside its scope (window ∩ quiz): best score per quiz, total, `reachedAt`, `runs`, `lastActivity` and the badges those runs earned. | One rule for all 4 × (1 + quizzes) boards; the all-time board of every quiz is exactly today's leaderboard. |
| One query for all four? | No: one query per board, `{ type: "leaderboard", period, quiz?, learner? }`. | The client asks for what it shows: the same traffic as before (one board per poll), not four times as much. |
| How does the UI show four boards? | One leaderboard, two segmented choices above its table — period and category — on the page and (period only) on the home card; the choice is shared by both. | The four tables have the same columns; side by side they do not fit a phone or the centre cell of the overview, and four polls every 10 s per learner are not worth it. |
| Column sorting | Every table of the page and of the card sorts by clicking a column heading (`aria-sort`, a button per heading); the order holds over all boards. | Asked for. |
| What triggers the crowd refresh on home? | `Leaderboard.submissions`: how many runs were submitted in the whole catalog, whatever the board. | A board of one period or quiz no longer changes with every submission; the count does — also beyond the top 100, which the old trigger missed. |

## 2. Contract (`🧬️schema/🔣️.json`)

- `LeaderboardPeriod`: `"daily" | "weekly" | "monthly" | "all-time"`.
- `LeaderboardWindow`: `{ from, until }` — `from` inclusive, `until` exclusive.
- `Leaderboard`: `{ period, quiz?, window?, rows ≤ 100, learners, submissions, own? }`.
- `Query` leaderboard: `{ type, period, quiz?, learner? }`; a `quiz` that is no slug is `id-invalid`, a slug the catalog does
  not list answers `404 unknown-quiz`.
- `LeaderboardRow` is unchanged; in a board of one quiz `total` is that quiz's best × 100 and `best` holds that quiz only.

## 3. Core (`🔨️modules/👁️views`, TypeScript + Rust twins, Python reference)

- `Transcript` — what every standing of a learner is made of: learner id, tag, identity, the submitted runs in
  submission order (`quiz`, `score`, `at`) and the badges (`badge`, `quiz` of the run that earned it, `at`).
  `transcript(state)` is `undefined` before registration or the first submission.
- `periodWindow(period, at)` — the window of `period` containing `at`.
- `BoardScope { window?, quiz? }`; `standing(transcript, catalog, scope)` — the unranked row over the runs in scope.
- `leaderboard(transcripts, catalog, { period, quiz? }, at, caller?)` — the whole answer.
- Rust only: `Merit` (what orders a standing) so the proctor's rank index holds 32 bytes per learner and board.

The Python reference computes the windows with `datetime` — the third party the calendar arithmetic of the twins is
held to.

## 4. Proctor

- `quiz.leaderboard` stores one `Transcript` per ranked learner (projector revision 4 → the read models rebuild).
- `Board`: the transcripts, the count of submissions and one rank index per asked `(period, quiz)` with the window it
  was built for. A changed transcript moves one entry per index (`O(log n)` to find); a query whose instant lies in
  another window than the index was built for rebuilds that index from the transcripts (once per day, week or month).
  It equals the core's `leaderboard` over every transcript at every instant.
- `quiz.leaderboard` reads the wall clock for the instant.

## 5. Client

- `ProctorClient.leaderboard(choice, learner)`; `QuizState.board` (the choice, ephemeral local-only) and
  `QuizState.leaderboards` (the last answer per choice, ephemeral shared) — switching shows the last known board at once
  and asks again.
- `LeaderboardPage`: period and category segments, the window in the viewer's zone, the sortable table.
- `LeaderboardCard`: period segments, the sortable excerpt.
