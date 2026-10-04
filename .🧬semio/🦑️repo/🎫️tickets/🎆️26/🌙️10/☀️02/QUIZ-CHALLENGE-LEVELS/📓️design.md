# 🎚️ Quiz challenge levels — Design (normative)

Ticket `2026/10/02/QUIZ-CHALLENGE-LEVELS`. Request (owner): "Introduce different challenge for quizzes: easy, medium, hard,
expert. E.g. the sorting task behaves different. Easy: the keys to sort are visible and the user just needs to assign them;
receives hints when something is off by a factor more than 1000. Medium: the keys are visible and the user just needs to
assign them. Hard: the keys are not shown and the user needs to guess them. Expert: the questions need to be solved with a
timer. You receive more points for completing a quiz on a harder challenge level. This will affect every kind of question."

Inputs: `📓️explore-core.md`, `📓️explore-react.md`, `📓️explore-proctor.md`, `📓️explore-site.md`, `📓️explore-designs.md`,
`📓️explore-tests.md` (this folder).

Contract: `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json` is normative; the TypeScript and Rust twins match it field for
field; every rule below lives in the shared core (TypeScript + Rust twins, Python reference) so the device's deputy and the
proctor decide alike.

Supersedes, in `../../../🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md`: §4 ("sheet items carry no value": a
sorting sheet now carries the ascending keys while the challenge shows them; the sheet is a function of
`(quiz, seed, challenge)`), §6 (a run also earns points; partial answers are scored on timed runs), §7 (badge rules may ask
for a least challenge), §8 (commands, events, rejections), the leaderboard parts of §18 and of
`../QUIZ-PERIOD-LEADERBOARDS/📓️design.md` (`best` is the best run, `total` sums points), §20 (no crowd during a run that
hides the keys); and in `../QUIZ-SORTING-NUMERIC-GUESSES/📓️design.md`: "guesses never change a score" (they do where the
keys are hidden) and "a guess is optional" (it is the answer where the keys are hidden, and absent where they show).

Names: the concept is **challenge** (`Challenge`, `challenge`), never "level", "difficulty" or "mode" (those words are taken).

## 1. Decisions

| Question | Decision | Why |
|---|---|---|
| Where does the challenge live? | On the run: chosen at `start-run`, a fact of `run-started`, fixed for the run. Nothing in quiz or catalog documents except an optional least challenge on badge rules. | A learner picks it per attempt; quiz revisions (file hashes) stay untouched; one rule table in the core serves every quiz. |
| What are the four challenges? | One table, `CHALLENGE_RULES`: `easy {keys, hints, par 100}`, `medium {keys, par 200}`, `hard {par 300}`, `expert {timed, par 400}`. Each is the one before plus one step. | The owner's four sentences, one mechanic each. |
| What are "the keys"? | The numbers a task turns on. Sorting: the true values of the presented items as an ascending ladder (place *i* shows the *i*-th smallest). Matching: the value cards (as before). Classification: category descriptions and the numbers of the spider diagram axes. | "Visible and the user just needs to assign them" is exactly what matching already does; a ladder gives sorting the same without a new answer shape or new random draws. |
| What does "keys hidden" ask of the learner? | Sorting and matching: a typed numeric guess per item (per dimension); the answer is the guesses. Classification has no numbers to guess: descriptions go, diagrams show shape only. | "The user needs to guess them." |
| What is "off by a factor of more than 1000"? | A value **misses** when it lies farther from the truth than the **reach** of its set. On a logarithmic scale the reach is a factor: 1000, or the square root of the presented true values' max/min ratio (half their spread in decades) where that is less; a value misses when `max(value, truth) / min(value, truth)` exceeds it (by more than the relative `REACH_SLACK`, §3.1). On a linear scale the reach is half the spread and a value misses when it lies farther than that (likewise widened). (Revised 2026-10-03 after the core audit: the first formulation compared `log10` differences, which put an exact factor of 1000 on either side of the boundary depending on the digits; ratios and square roots are exactly rounded in every language, so exactly ×1000 never misses and the twins agree bit for bit.) | 1000 fits the physics sortings (16 to 25 decades); seven of nine numeric sets span under 4 decades, where a fixed 1000 would hardly ever fire (never under 3, since a key can be off by at most the set's own ratio). One concept serves hints (easy) and scoring (hard, expert). |
| What is a hint? | On easy, while the run is open: per item whose assigned key misses, "far too high" or "far too low"; per classification task, the count of misplaced items (never which). A pure function of (task, sheet task, answer), carried by `RunView.hints`. | Per-item hints on a two-category classification would hand over the solution; a count does not. A view needs no new event and both the proctor and the deputy compute it. |
| How are guesses scored? | The same magnitude-weighted pair concordance; a pair also counts as discordant when either of its items misses or has no guess. Perfect stays reachable (right order, every guess within reach). | Typing `1, 2, 3` in the right order must not earn a perfect score; exactness must not be required either. |
| How do points work? | `points = score × par(challenge)`. `score` stays in [0, 1] and means accuracy at every challenge. Best per quiz = the submitted run with the most points; total = sum of those points. | "More points on a harder challenge"; badges, bins and percentages keep their meaning. |
| Who keeps the time? | The device. `start-run`, `open-task` and `record-answer` carry `at`, the instant the learner acted; a decider raises it to its floor (the run's start, or the task's opening) and applies the limit to those instants. The decider's own clock never enters a time verdict. The limit is `seconds` on each sheet task. (Revised 2026-10-03 after the core audit: bounding by the decider's `now` as well made the proctor's verdict depend on delivery time when the device clock runs ahead; with the floor alone the proctor is never stricter than the device. Revised again the same day after the client audit: `start-run` carries `at` too and `run-started.at` is that claim, so a run started offline does not take the proctor's later delivery time as its start and floor, and every time fact of a run comes from the device's own claims. Revised a third time after the acceptance audit: a claim more than `CLOCK_LEAD` = 300 000 ms ahead of the decider's clock is lowered to `now + CLOCK_LEAD`, so `acted(at, floor, now) = max(min(at, now + CLOCK_LEAD), floor)`; a device decides at its own clock, so an honest device whose clock is less than five minutes ahead of the proctor's never meets the cap and both still decide alike, while an opening dated far ahead no longer buys time.) | The deputy decides offline and the proctor decides again at delivery; with the proctor's arrival clock a connection shortage would turn a timely answer into a late one. The device already holds every solution, so the trust model is unchanged. |
| What happens when time is up? | The task takes no more answers (`time-up`). A timed run may be submitted with tasks unanswered or partly answered; what is missing scores as a miss. | The proctor has no scheduler; lazy rules are deterministic. |
| Where does the learner see the tolerance? | Easy: in every hint ("more than ×N"). Hard and expert: in the results, per task ("A guess counts within ×N of the true value" on a logarithmic scale, "within ±d" on a linear one) and per item (the factor by which the guess was off). Not during play. | Showing the reach while guessing would reveal the spread of the hidden values; afterwards it explains every miss. (Added 2026-10-03 after the acceptance audit.) |
| Which challenge does a start use? | The one last chosen for that quiz (`preferences.challenges[quiz]`, default `medium`); the card's action names it. | A remembered expert on one quiz must not start another quiz on expert by one click. (Revised 2026-10-03 after the acceptance audit; was one remembered challenge for all quizzes.) |
| One open run per quiz? | Yes. `start-run` at the open run's challenge answers `run-open` (the client resumes); at another challenge it voids the open run and starts the new one. | Lets a learner step down from a challenge that is too hard without first submitting it. |
| Leaderboard per challenge? | No. One total of points; each row's `best` names the challenge of the best run. | Not asked for; the periods × quizzes matrix stays as it is. |
| Badges? | `perfect-quiz` and `perfect-tasks` take an optional `challenge`: the least challenge that counts. | Easy hints must not hand out "expert" badges. |
| Crowd? | One crowd per quiz, all challenges mixed; a guessed matching value is tallied under the nearest authored value. During a run that hides the keys the others' answers are not offered. | Figures stay comparable; the crowd would show the hidden keys. |
| Stored formats? | Proctor `FORMAT_VERSION` 3, `STATE_FORMAT` 2, `PROJECTOR_REVISION` 6. All new members are required; nothing is migrated. | Greenfield rules. |

## 2. Contract (`🧬️schema/🔣️.json`, twins `🟦️.ts`, `🦀️.rs`)

New and changed `$defs` (everything else unchanged). Optional members are absent, never `null`.

```
Challenge        enum  easy | medium | hard | expert                      (TS CHALLENGES, Rust enum Challenge, kebab-case)
MagnitudeHint    { kind: "magnitude", item: Slug, dimension?: Slug, direction: "high" | "low" }
MisplacedHint    { kind: "misplaced", count: integer >= 1 }
Hint             oneOf MagnitudeHint | MisplacedHint
Best             { challenge: Challenge, score: Score, points: number >= 0 }

SheetAxis        { id: Slug, label: Text, unit?: string, min?: number, max?: number }   unit, min, max travel together
SheetClassificationTask   axes?: SheetAxis[]   (was Axis[]);  + seconds?: integer >= 1
SheetSortingTask          + keys?: number[]  (ascending true values of the presented items);  + seconds?
SheetDimension            cards?: number[]   (was required)
SheetMatchingTask         + seconds?
Sheet                     + challenge: Challenge  (required)

MatchingAnswer            { kind: "matching", assignments?: {dimension: {item: cardIndex}}, guesses?: {dimension: {item: number}} }
ClassificationItemResult  assigned?: Slug    (was required)
SortingItemResult         + guess?: number;  + miss?: boolean
MatchingItemResult        assigned?: number  (was required);  + miss?: boolean
RunResult                 + challenge: Challenge;  + points: number >= 0   (both required)

Command start-run         + challenge: Challenge;  + at: Timestamp         (both required; `at` added 2026-10-03)
Command open-task         { type: "open-task", id: Id, learner: Id, run: Id, task: Slug, at: Timestamp }   (new)
Command record-answer     + at: Timestamp                                  (required)
Rejection                 + run-untimed | task-unopened | time-up | already-opened
Event run-started         + challenge: Challenge                           (required)
Event task-opened         { type: "task-opened", learner: Id, run: Id, task: Slug, at: Timestamp }          (new)

RunView                   + opened?: {task: Timestamp};  + hints?: {task: Hint[]}
RunSummary                + challenge: Challenge (required);  + points?: number >= 0  (beside score)
LearnerView.best          {quiz: Best}       (was {quiz: Score})
LeaderboardRow.best       {quiz: Best}       (was {quiz: Score})
BadgeRule perfect-quiz    + challenge?: Challenge
BadgeRule perfect-tasks   + challenge?: Challenge
```

Member order (Rust struct order, schema `properties` order, TypeScript declaration order): `start-run` `id, learner, run, quiz,
challenge, at`; `open-task` `id, learner, run, task, at`; `record-answer` `id, learner, run, task, answer, at`; `run-started`
`learner, run, quiz, challenge, revision, seed, at`; `Sheet` `quiz, seed, challenge, title, description, tasks`; `RunResult`
`quiz, challenge, score, points, tasks`; `RunSummary` `run, quiz, challenge, status, startedAt, score, points, submittedAt`;
`RunView` gains `opened, hints` at the end; sheet tasks gain `keys` (sorting, after `quantity`) and `seconds` at the end;
item results gain `guess, miss` before `explanation`; `Best` `challenge, score, points`; `TranscriptRun`
`quiz, challenge, score, points, at`; `RunState` gains `challenge` after `quiz` and `opened` after `recorded`.

Views module types (not in the schema): `TranscriptRun { quiz, challenge, score, points, at }`.
Lifecycle state: `RunState` gains `challenge: Challenge` and `opened: {task: Timestamp}` (empty map on untimed runs).

When a member is present:

| Member | Present exactly when |
|---|---|
| `SheetSortingTask.keys`, `SheetDimension.cards`, `SheetAxis.unit/min/max`, `Category.description` in a sheet | the challenge shows the keys (easy, medium) |
| `seconds` on every sheet task, `RunView.opened` | the challenge is timed (expert) |
| `RunView.hints` | the run is open, the challenge hints (easy) and at least one hint exists; only tasks with hints appear |
| `SortingAnswer.guesses`, `MatchingAnswer.guesses` | only when the sheet task hides the keys (complete when every item has one; a recorded answer may lack them); `MatchingAnswer.assignments` exactly when it shows them |
| `SortingItemResult.guess`, `miss`, `MatchingItemResult.miss` | the sheet task hides the keys (`guess` only where the learner guessed; `miss` on every item) |
| `…ItemResult.assigned` absent | the learner left the item unanswered (timed runs only) |
| `RunSummary.points` | the run is submitted (as `score`) |

## 3. Core (`🔨️modules/…`, TypeScript + Rust twins, Python reference)

### 3.1 New module `⛰️challenge` (`challenge.ts` twin names in backticks, Rust in snake_case)

- `CHALLENGE_RULES: {[C in Challenge]: ChallengeRules}`, `ChallengeRules { keys: boolean, hints: boolean, timed: boolean, par: number }`:

  | challenge | keys | hints | timed | par |
  |---|---|---|---|---|
  | easy | yes | yes | no | 100 |
  | medium | yes | no | no | 200 |
  | hard | no | no | no | 300 |
  | expert | no | no | yes | 400 |

- `challengeRules(challenge)`; `challengeRank(challenge)` = 0…3 in the order above; `challengeMeets(challenge, least)` = `rank(challenge) >= rank(least)`.
- `points(score, challenge)` = `score × par`.
- `REACH_FACTOR = 1000`. `reach(values, scale)`: logarithmic (all values > 0): `lo = min(values)`, `hi = max(values)`; `reach = hi > lo ? min(1000, sqrt(hi / lo)) : 1000` — a factor. Linear: `reach = hi > lo ? (hi − lo) / 2 : +∞` — a distance.
- `REACH_SLACK = 1e-9`. `misses(value, truth, scale, reach)`: logarithmic `max(value, truth) / min(value, truth) > reach × (1 + REACH_SLACK)`; linear `|value − truth| > reach × (1 + REACH_SLACK)` (+∞ never misses). The slack lets a value typed in decimal at exactly the reach count as within it (`0.018` against `18` is a hair over 1000 in binary); revised 2026-10-03. Only `/`, `*`, `+`, `sqrt`, `−`, `abs`, `min`, `max` and comparisons: exactly rounded, so an exact ×1000 never misses and every twin agrees. (Revised 2026-10-03, see §1.)
- `TASK_SECONDS = { base: 30, classification: 8, sorting: 12, matching: 12 }`; `taskSeconds(kind, items, dimensions)` = `base + per(kind) × items × (kind = matching ? dimensions : 1)`.
- `CLOCK_LEAD = 300000`. `acted(at, floor, now)` = `max(min(at, now + CLOCK_LEAD), floor)`. (Revised 2026-10-03 twice: first without any bound by `now`, then with the five-minute lead cap of §1.) The `run-started.at` claim takes the same cap with floor 0.
- `hintsOf(task, sheetTask, answer): Hint[]` (answer may be absent → `[]`):
  - sorting (sheet has `keys`): for every item at position `p` of `answer.order`, in that order: if `misses(keys[p], value(item), scale, reach(keys, scale))` → `{kind: "magnitude", item, direction: keys[p] > value ? "high" : "low"}`.
  - matching (dimension has `cards`): per dimension in sheet order, per sheet item in sheet order with an assigned card: `truth` = the item's value, `assigned = cards[index]`, `reach` over the presented items' true values of that dimension; a miss → `{kind: "magnitude", item, dimension, direction}`.
  - classification: `count` = assigned sheet items whose category is not their own; `count > 0` → `[{kind: "misplaced", count}]`.

### 3.2 Sheet — `sheetOf(quiz, seed, challenge)`

The generator is consumed exactly as before for every challenge (same draws in the same order), so one seed deals the same
items in the same order at every challenge. After the draws:
- `sheet.challenge = challenge`.
- Keys shown: sorting gets `keys` = the presented items' values, ascending; matching keeps `cards`; classification keeps category `description`s and whole axes.
- Keys hidden: sorting has no `keys`; matching dimensions have no `cards` (the card shuffle is still drawn); classification categories lose `description`, axes are `{id, label}` only and every profile value becomes its share of the axis range, `(v − min) / (max − min)`.
- Timed: every sheet task gets `seconds = taskSeconds(kind, presented items, dimensions)`.

### 3.3 Validation — `answerRejection(sheetTask, answer)`, `answerComplete(sheetTask, answer)`

- Sorting with `keys`: `guesses` present → invalid. Complete as before (a recorded order).
- Sorting without `keys`: `guesses` as before (keys are sheet items, finite, positive on a logarithmic scale, guessed items stand in non-decreasing guess order). Complete when every sheet item has a guess.
- Matching dimension with `cards`: `assignments` as before, `guesses` present → invalid. Without `cards`: `assignments` present → invalid; `guesses` keys are sheet dimensions and items, values finite and positive on a logarithmic scale. Complete when every item has a guess in every dimension.
- Classification: unchanged.
- `commandRejection`: `open-task` has its ids and its task slug checked like `record-answer`. Rust cannot decode a `challenge` outside the four or an `at` that is no `u64` (the proctor answers `command-malformed`); an `at` beyond 2^53 − 1 decodes and both cores refuse it `id-invalid`; TypeScript refuses all of them `id-invalid` (its types, the proctor client and the outbox's `restoredCommand` keep such a command from being built or restored in the first place). No new rejection code. (Revised 2026-10-03 after the docs audit: the code refuses these as `id-invalid`, see the core fixes.)
- Badge rule: `challenge`, when present, is a `Challenge` (issue code as for the other rule members).
- Thinking drafts: `ThinkingMatchingAnswer.values` carries guessed numbers as it carries card values, and sorting drafts carry `guesses`. The core checks only that they are finite; the proctor, which knows the quiz, admits a matching value or a sorting guess when it is positive on a logarithmic scale, whether or not it is a card value (`value-invalid` otherwise), and a guess only for an item of its task. (Revised 2026-10-03 after the docs audit: the proctor had admitted card values only, so every guess was refused.)

### 3.4 Scoring — `scoreRun(quiz, sheet, answers)`

`RunResult { quiz, challenge: sheet.challenge, score, points: points(score, challenge), tasks }`, `score` = mean of task scores in sheet order.

- **Partial answers.** On a timed sheet a task may have no answer or an incomplete one; on any other sheet `scoreRun` is undefined unless every answer is complete (as before).
- **Sorting, keys shown.** Unchanged.
- **Sorting, keys hidden.** Items in the learner's `order`; without an answer, in sheet order with no guesses. `r = reach(presented true values, scale)`. `miss(i)` = no guess, or `misses(guess, value, scale, r)`. For every pair `i < j`: `w = |s(v_i) − s(v_j)|`, `total += w`; `discordant += w` when `v_i > v_j` or `miss(i)` or `miss(j)`. `score = total > 0 ? 1 − discordant / total : (any miss ? 0 : 1)`. Item results carry `guess?` and `miss`.
- **Matching, keys shown.** Unchanged; on a timed sheet an unassigned item takes part as a miss (never the case today: keys shown is never timed).
- **Matching, keys hidden.** Per dimension over the sheet items: `a_i` = the guess, `miss(i)` as above with `r` over the presented true values of the dimension. Pair: `miss(i)` or `miss(j)` → `discordant += w`; else opposite order → `w`; else equal guesses and different truths → `w / 2`. Dimension score as above (`total = 0`: `any miss ? 0 : 1`). `assigned` = the guess (absent when unguessed), `miss` on every item.
- **Classification.** Unchanged; an unassigned item (timed) has credit 0 and no `assigned`.

Why a miss costs every pair it touches: a guess that is three orders of magnitude off does not know the item, wherever it
happens to stand.

### 3.5 Lifecycle — `decideLearner`, `evolveLearner`

| Command | Rule (after the existing checks, in this order) | Events |
|---|---|---|
| `start-run` | An open run of the quiz at the current revision: same `challenge` → `run-open`; another → void it. Caps (`runs`, `runsPerQuiz`) count every run the learner started, whatever its status (revised 2026-10-03 after the proctor audit: switching must not grow a learner's state without bound). `at` is a safe integer in both cores. | `[run-voided? {at: now}, run-started {…, challenge, at: command.at}]` (the run's start is the device's claim; voids keep the decider's clock) |
| `open-task` | `unknown-run`, `run-closed`, stale → `quiz-revised`, untimed run → `run-untimed`, task not in the sheet → `unknown-task`, already opened → `already-opened` (revised 2026-10-03 after the proctor audit: an accepted turn without events would still store a receipt; the client treats it as success). | `task-opened { at: acted(command.at, run.startedAt) }` |
| `record-answer` | As before through `unknown-task`. Timed run: task not opened → `task-unopened`; `t = acted(command.at, opened[task])`; `t − opened[task] > seconds × 1000` → `time-up`. Untimed: `t = acted(command.at, run.startedAt)`. Then `answer-invalid`. | `answer-recorded { at: t }` |
| `submit-run` | Untimed: every task complete, else `run-incomplete` (as before). Timed: never `run-incomplete`. | `run-submitted { result }`, badges |

`evolveLearner`: `run-started` stores `challenge` and `opened: {}`; `task-opened` sets `opened[task] = at`. Every
`sheetOf` call passes the run's challenge.

### 3.6 Views, badges, crowd

- `runView`: sheet at the run's challenge; `opened` on timed runs; `hints` per §2.
- `learnerView`: `RunSummary.challenge`, `points`; `best[quiz]` = `Best` of the submitted run with the most points (a later run replaces only when strictly more); `total` = Σ over catalog quizzes of `best.points`.
- `transcript`, `standing`, `leaderboard`: `TranscriptRun` carries `challenge`, `points`; `bests` compares points; `reachedAt` = `at` of the run that last raised any best; order unchanged (total, badge count, reachedAt, learner id).
- Badges: `perfect-quiz {quiz, challenge?}` = some result of the quiz scored 1 at a challenge that meets `challenge`; `perfect-tasks` likewise per task; `completed-quizzes` unchanged.
- Crowd: score bins take `score` (accuracy) of every challenge. Matching: an assigned card value counts under its value as before; a guess counts under the nearest value among **all** authored items of the task for that dimension (distance in `s`; a tie takes the smaller value); an unanswered item counts nowhere. Sorting places as before from `position`; a sorting whose keys were hidden and in which no item was guessed (a timed run left it unanswered) adds its score bin only.

## 4. Proctor (`🎓️teaching/🛂️proctor`)

- Command `quiz.open-task` joins the manifest, the learner policy template and admission (target = the learner stream, offline policy `Optimistic` like `record-answer`).
- `FORMAT_VERSION` 3 (storage, bootstrap, storage unit test, README), `STATE_FORMAT` 2, `PROJECTOR_REVISION` 6.
- The projector and `Board` keep calling the core (`run_view`, `learner_view`, `standing`); the crowd tally folds a run with its quiz (nearest authored value) and stays equal to `quiz::crowd_view` bit for bit.
- End-to-end: the "no value in a sheet" assertion holds for hard and expert; easy and medium sheets carry sorting `keys`. New cases: a run per challenge, hints on easy, guesses on hard, the clock on expert (`open-task`, `time-up`, partial submission), switching the challenge voids the open run, points on the board.

## 5. Client (`🎯️targets/⚛️react`)

- **Choosing.** The quiz page shows a "Challenge" group of four radios (the identity step's fieldset pattern), each with one line saying what it does and its most points. The choice is remembered on the device (`preferences.challenge`, default `medium`; not part of the preferences panel). The overview card starts with the remembered challenge and names it in its action. With an open run the page and the card offer to continue it and name its challenge; choosing another challenge on the page asks before the open run is discarded.
- **Session.** `startRun(quiz, challenge, signal)`; `openTask(run, task, signal)`; `answer` stamps `at` from the session clock; the outbox restores `start-run` only with a `challenge`, `open-task` and `record-answer` only with `at`; `open-task` keeps its place in the queue before the answers of its task. The deputy rebuilds `RunState.challenge` from `view.sheet.challenge` (or the summary) and `opened` from `view.opened`, and compares sheets with `sheetOf(quiz, seed, challenge)`. Stored runs without a challenge are not restored.
- **Hints.** With a deputy the view comes from the deputy after every answer; without one the session re-reads the run once an answer of an easy run is delivered. A hint shows beside its item (words and a symbol, never colour alone) and is announced politely once when it appears.
- **Sorting.** Keys shown: each place shows its key; items are moved by the buttons or by dragging; no guess fields. Keys hidden: a guess field per item (the existing field), items order themselves by their guesses, no moving by hand.
- **Matching.** Keys shown: as before. Keys hidden: no cards; one guess field per item and dimension.
- **Classification.** Keys hidden: no category descriptions; diagrams without axis numbers.
- **Clock.** A timed task shows only its title, the time allowed and "Start the clock" until it is opened. Then the remaining time shows as text (`m:ss`, tabular, not a live region), with a separate polite status that speaks at the start, at 30 s, at 10 s and when time is up. When time is up the task turns read-only and says so; focus is not moved. Submitting a timed run names the tasks that are still open or unanswered in the confirmation. The countdown is driven by the session clock seam (`now`) and by the device's own opening instant; hidden tabs change nothing (the deadline is an instant).
- **Showing.** The run names its challenge. Results: the score ("87 %") and, below it, challenge and points ("Hard · Points: 261 of 300"), "Your guess" with a miss mark where the keys were hidden, "Not answered" where time ran out. Quiz card and page: best as points with its challenge. Learner runs table: challenge and points. Leaderboard: per-quiz best as points with the challenge. The others' answers are not offered during a run that hides the keys.
- **Texts** in English and German (*du*), no plural forms, every key used as a literal.

## 6. Site (`🎓️teaching/🏛️architecture/❓️quiz`)

- Catalog: the introduction explains the four challenges and their points; the four `*-expert` badges, `numerical-brain` and `pattern-seer` ask for `"challenge": "medium"`.
- End-to-end driver: `playQuiz(device, quiz, challenge = "medium")`; answering by challenge (ladder moves, typed guesses); `shownResults` reads points. New spec `challenge-levels` in the `desktop` project: one quiz on each challenge, hints on easy, guesses on hard, the clock on expert with the page clock.

## 7. Tests

Language-agnostic (feature + Python reference with a third-party oracle + TypeScript + Rust over shared vectors):
- new case `⛰️challenge-rules`: rule table, points, `reach`, `misses`, `taskSeconds`, `acted`, `hintsOf` (oracle: numpy);
- `🃏️sheet-assembly`: one seed at all four challenges (same items and order; keys, cards, axes, seconds per §3.2);
- `✅️answer-validation`: guesses and assignments by challenge, completeness;
- `📏️sorting-concordance`, `🔀️matching-concordance`: guessed variants with misses and missing guesses; `🕸️profile-similarity`: unassigned items;
- `🧾️learner-lifecycle`: challenge at start, switching voids, `open-task`, `task-unopened`, `time-up`, instants raised to their floors, partial submission, points in views;
- `🏆️leaderboard`: best by points across challenges; `🏅️badge-rules`: least challenge; `📊️crowd-view`: guessed values, unanswered items; `🧬️schema-conformance`: every new and changed `$defs` type.

Unit and client tests follow the modules they belong to; gates are those of the earlier tickets (quiz TypeScript and Rust
suites, parity, React suite and typecheck, proctor suites, site tests, taxonomy, end-to-end).

## 8. Revision 2026-10-04 — specific hints

Request (owner): "The hints on easy shouldn't be general and show: ≪ Wert viel zu klein (mehr als ×1.000). Instead always
use specific examples relative to the others: Bist du sicher, [dass] XX 1000 zusammen nur eine YY ergeben? Make a
sophisticated hint mechanism that is never generic."

This section supersedes `MagnitudeHint`, `MisplacedHint` and the hint rules of §1 ("What is a hint?"), §2, §3.1 and §5.
When a hint fires is unchanged for sorting and matching (an assigned key misses its value); what it says changes: every
hint questions one concrete relation **the learner's own answer claims** between two items of the task, never a bare
direction or a count. It never states the truth; its wording ("only" or "as much as") says on which side the truth lies.

### 8.1 Decisions

| Question | Decision | Why |
|---|---|---|
| What does a numeric hint say? | For the missed item `X` and a reference item `R` of the same task (and dimension): the relation the learner's keys claim between them, questioned. Additive quantities: "Are you sure 1,000 × “R” together only add up to 1 × “X”?" / "…that it takes 1,000 × “R” to add up to 1 × “X”?". Other quantities: "Are you sure “X” is only 1,000 times as large as “R”?" / "…as much as 1,000 times as large…". Linear scales: "…lies only 5 °C above…" / "…as much as 5 °C above…". | The owner's example; "together" is only true where amounts add up (powers, energies), not for U-values, loads per m² or air change rates. |
| Which reference? | Among the other presented items with an assigned key, preferably those whose own key does **not** miss (the learner's anchors), the one whose claimed relation to `X` is the most wrong; ties take the first in sheet order. With no anchor, every other assigned item is a candidate. | The relation that is most wrong is the most telling; an anchor the learner got right makes the doubt concrete. |
| "only" or "as much as"? | `under` = the truth lies further from 1 than the claim, in the same direction (on a linear scale: further from 0, same sign); otherwise the claim overstates or points the wrong way. | The owner's "nur" is the understating case; the other case needs its own wording. |
| Additive? | New required `Quantity.additive: boolean`, authored: `true` for the physics powers and energies, `false` for every other quantity of the four quizzes. | Only the author knows whether amounts add up. |
| Spider profiles? | When the assigned profile `P` differs from the item's own profile `Q` on some axis by more than that axis's reach (half the spread of the presented categories' values on it, widened by `REACH_SLACK`), a `ProfileHint` names the axis with the largest gap relative to its reach: "Are you sure “Passive house” fits Profile B, with heating demand at about 303 kWh/(m²·a)?". Near misses give no hint. | Same "far off" idea as the numbers, made concrete with the visible value of the assigned profile. |
| Plain classification? | Per misplaced item `X`: a pair hint with an item `R` the learner put in the same category although its own category differs ("Are you sure “X” and “R” belong to the same category?"), else with an item of `X`'s own category the learner put elsewhere ("…belong to different categories?"), else a category hint naming the assigned category and its description ("Are you sure “X” belongs to Power — the rate at which energy flows?"). | Specific without saying which of the two is wrong. |
| Several hints? | One hint per hinted item (per dimension for matching), at most one per item and dimension; a reference may appear in several. | Each doubt stays concrete and short. |

### 8.2 Contract

```
Quantity          + additive: boolean   (required; after `prefixed`)
CompareHint       { kind: "compare", item: Slug, other: Slug, dimension?: Slug, factor?: number, difference?: number, under: boolean }
                  factor = key(item) / key(other) on a logarithmic scale; difference = key(item) − key(other) on a linear one;
                  exactly one of them; keys are the learner's assigned keys (sorting: the ladder key at the item's place,
                  matching: the assigned card)
ProfileHint       { kind: "profile", item: Slug, category: Slug, axis: Slug }   category = the assigned one
GroupHint         { kind: "group", item: Slug, other: Slug, together: boolean }
CategoryHint      { kind: "category", item: Slug, category: Slug }               category = the assigned one
Hint              oneOf CompareHint | ProfileHint | GroupHint | CategoryHint    (MagnitudeHint, MisplacedHint removed)
WireVersion       3
```

`RunView.hints` keeps its shape (`{task: Hint[]}`), order: sorting in the learner's order, matching per dimension in sheet
order then items in sheet order, classification in sheet order.

### 8.3 Core (`⛰️challenge`, TypeScript + Rust twins, Python reference)

- Numeric: `X` hinted when its key misses (unchanged). Candidates `C` = other items with an assigned key (matching: in that
  dimension). Anchors = candidates whose key does not miss. Pool = anchors if any, else `C`; empty pool → no hint.
  Logarithmic: `ρ = k_X / k_R`, `τ = v_X / v_R`, error `e = max(ρ, τ) / min(ρ, τ)`; `under = (ρ ≥ 1 and τ > ρ) or (ρ < 1 and τ < ρ)`.
  Linear: `δ = k_X − k_R`, `Δ = v_X − v_R`, `e = |δ − Δ|`; `under = (δ ≥ 0 and Δ > δ) or (δ < 0 and Δ < δ)`.
  `R` = pool member with the largest `e`; members whose `e` lies within the relative `REACH_SLACK` of the largest are tied
  (exact anchors all give the same `e` up to rounding), and among them the smallest oriented claim wins — `max(ρ, 1/ρ)`
  on a logarithmic scale, `|δ|` on a linear one — so the hint questions the most absurd relation; remaining ties take the
  first in sheet order. (Tie rule added 2026-10-04 after the Rust report: without it rounding noise picked `R`.) Only `/`, `−`, `abs`, comparisons: bit for bit
  across twins; the factor is not rounded in the core.
- Profile: for each presented, assigned item whose assigned category `P` is not its own `Q` and both carry profiles:
  per axis `a` of the task with values in both, `gap_a = |P_a − Q_a|`, `reach_a = (max − min) / 2` over the presented
  categories' values on `a` (no hint on an axis whose reach is 0); hinted when some `gap_a > reach_a × (1 + REACH_SLACK)`;
  `axis` = largest `gap_a / reach_a` (first in axis order on ties).
- Group / category: for each presented item `X` assigned to a category that is not its own (and not covered by a profile
  rule because a profile is missing): first `R` in sheet order assigned to `X`'s assigned category whose own category
  differs from `X`'s → `together: true`; else first `R` whose own category is `X`'s and that is assigned elsewhere →
  `together: false`; else `CategoryHint` with the assigned category.

### 8.4 Client

The client renders each hint as one question beside its item (symbol + text, announced once, as before), with item labels
in quotes, the factor shown with at most two significant digits (oriented so it reads ≥ 1: when `factor < 1` the items
swap roles and the factor inverts), the difference with the quantity's unit, the profile value from the sheet's category
profile on that axis with the axis unit, the category label and description. English and German (*du*):

- additive under: EN "Are you sure {{count}} × “{{small}}” together only add up to 1 × “{{large}}”?" DE "Bist du sicher, dass {{count}} × „{{small}}“ zusammen nur 1 × „{{large}}“ ergeben?"
- additive over: EN "Are you sure it takes {{count}} × “{{small}}” to add up to 1 × “{{large}}”?" DE "Bist du sicher, dass es {{count}} × „{{small}}“ braucht, um 1 × „{{large}}“ zu ergeben?"
- ratio under/over: EN "Are you sure “{{large}}” is only / as much as {{count}} times as large as “{{small}}”?" DE "Bist du sicher, dass „{{large}}“ nur / ganze {{count}}-mal so groß ist wie „{{small}}“?"
- difference under/over: EN "Are you sure “{{large}}” lies only / as much as {{difference}} above “{{small}}”?" DE "Bist du sicher, dass „{{large}}“ nur / ganze {{difference}} über „{{small}}“ liegt?"
- profile: EN "Are you sure “{{item}}” fits {{category}}, with {{axis}} at about {{value}}?" DE "Bist du sicher, dass „{{item}}“ zu {{category}} passt, mit {{axis}} bei rund {{value}}?"
- group: EN "Are you sure “{{item}}” and “{{other}}” belong to the same category / to different categories?" DE "Bist du sicher, dass „{{item}}“ und „{{other}}“ in dieselbe Kategorie / in verschiedene Kategorien gehören?"
- category: EN "Are you sure “{{item}}” belongs to {{category}} — {{description}}?" DE "Bist du sicher, dass „{{item}}“ zu {{category}} gehört — {{description}}?" (without a description: the question ends after the category)

Hints are questions: the symbol is "?" in a circle, never an arrow that gives the direction away.

### 8.4a Revision after the hint audit (2026-10-04, `📓️audit-hints.md`; supersedes the parts of §8.1–§8.4 it contradicts)

1. **Verdict.** `CompareHint.under: boolean` becomes `verdict: "under" | "over" | "reversed"`. `reversed`: the claim and
   the truth point in opposite directions (log: `(ρ ≥ 1) ≠ (τ ≥ 1)` with `ρ ≠ 1`; linear: `sign δ ≠ sign Δ`); `under`
   and `over` as before when they agree. A reversed hint asks for the order without a number: EN "Are you sure
   “{{larger}}” is larger than “{{smaller}}”?" DE "Bist du sicher, dass „{{larger}}“ größer ist als „{{smaller}}“?"
   (larger = the item the learner's keys make larger). Additive quantities use the same reversed question.
2. **Over wording.** "really" / "tatsächlich" replace "as much as" / "ganze": EN "Are you sure “{{large}}” is really
   {{count}} times as large as “{{small}}”?" DE "Bist du sicher, dass „{{large}}“ tatsächlich {{count}}-mal so groß ist
   wie „{{small}}“?"; difference likewise ("really {{difference}} above" / "tatsächlich {{difference}} über").
3. **Numbers.** Counts and factors: below 10⁶ plain with locale grouping and at most two significant digits; from 10⁶ to
   below 10¹⁵ two significant digits with a scale word (EN million, billion, trillion; DE Million(en), Milliarde(n),
   Billion(en)); from 10¹⁵ "about 10ⁿ" / "rund 10ⁿ" with a superscript exponent (n = rounded log10). Minus signs are U+2212.
4. **Short labels.** Optional `short: Text` (each language ≤ 40 characters) on sorting, matching and classification
   items, on categories, on axes and on `Quantity`; hints use `short` when present, else `label`. The site content test
   requires `short` wherever a label exceeds 40 characters in either language.
5. **Familiar references.** Optional `familiar: boolean` on sorting and matching items (an everyday thing a learner can
   picture: a tea light, a kettle, a smartphone charge). In the reference choice, after the error tie window of §8.3,
   familiar items win, then the smallest oriented claim, then sheet order.
6. **Profiles relative to items.** `ProfileHint` becomes `{ kind: "profile", item, category, axis, other?: Slug,
   above?: boolean }`: on the chosen axis, if some presented item `R` is placed in its own category (an anchor) and the
   learner's placement claims `X` lies above `R` while `X`'s own profile lies below `R`'s (or the reverse), the hint names
   the first such `R` (largest gap first, then sheet order) and `above` = the claimed side: EN "Are you sure
   “{{item}}” lies above “{{other}}” in {{axis}}?" / "below"; DE "Bist du sicher, dass „{{item}}“ beim {{axis}} über
   „{{other}}“ liegt?" / "unter". Without such an `R` the value form of §8.4 stays.
7. **Dimensions.** On a matching with more than one dimension the question names the quantity: EN "…in {{quantity}}…",
   DE "…beim {{quantity}}…" (placement chosen per template so the sentence stays grammatical; the client agent settles
   the exact positions and writes them into its report).
8. **Cap.** At most `HINTS_PER_TASK` = 3 hints per task: those with the largest error first (compare `e`; profile
   `gap/reach`; group and category hints after compare and profile hints, in sheet order); the rest are not given.
9. `WireVersion` 4.

### 8.4b Revision after the second hint audit (2026-10-04, `📓️audit-hints-2.md`)

1. Every compare question names the quantity (its `short` or label), not only on multi-dimension matchings, so "larger"
   never reads as physical size; English lower-cases the first letter of a quantity or axis name mid-sentence (German keeps
   its capitalised nouns).
2. Counts from 10¹⁵ show the learner's claim as a mantissa with two significant digits times a power of ten
   ("1.1 × 10¹⁶" / "1,1 × 10¹⁶"), never a rounded power alone.
3. Reference choice inside a task: the hints of one answer are decided in their order, and inside the tie window a
   candidate not yet used as the reference of an earlier hint of the same task wins first, then familiar, then the smallest
   oriented claim, then sheet order.
4. Short labels that read awkwardly in hints are re-authored (content).

### 8.5 Tests

`⛰️challenge-rules` vectors for every hint kind and branch (anchor preference, no anchor, ties, under/over both
directions, factor < 1, linear difference, profile reach and axis choice, near miss without hint, group together/apart,
category fallback); proctor end-to-end and React tests assert the new shapes and texts; the site spec asserts a concrete
easy hint question in English and German; the four quiz files gain `additive` (revision change accepted).
