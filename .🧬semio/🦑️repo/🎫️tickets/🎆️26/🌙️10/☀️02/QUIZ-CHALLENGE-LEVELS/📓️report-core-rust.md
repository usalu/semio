# Report — Rust core of the challenge levels

Agent: core Rust (design §1, §2, §3). `Q` = `🧰️framework/🛍️products/❓️quiz`. I checked the Rust twin against the landed `🧬️schema/🔣️.json` member by member, and then against the TypeScript twin (`📓️report-core-typescript.md`) wherever the design leaves an edge case open.

## Changed files

Created
- `Q/🔨️modules/⛰️challenge/🦀️.rs`: the new module.
- `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`: 16 tests, including a replay of the shared vectors `🧫️fixtures/⛰️challenge-rules`.

Updated
- `Q/🧬️schema/🦀️.rs`: `Challenge`, `CHALLENGES`, `SheetAxis`, `HintDirection`, `MagnitudeHint`, `MisplacedHint`, `Hint`, `Best`, `Command::OpenTask`, `Event::TaskOpened`, the three new rejections, and every changed member in design order. Also `SheetTask::seconds()` and `SheetTask::items()`.
- `Q/🔨️modules/{🃏️sheet,✅️validation,📏️scoring,🧾️lifecycle,👁️views,🏅️badges,👥️presence}/🦀️.rs` and the unit tests of each module, plus `Q/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`.
- `Q/📦️packages/🦀️rust/🦀️.rs`: adds the `#[path]` for the `challenge` module.
- `Q/🦀️.rs`: adds `pub use crate::challenge::*`.
- `Cargo.toml`: unchanged. Unit tests are inline modules.

Edits to case bindings owned by the vectors agent (only to keep them compiling; replace freely)
- `🃏️sheet-assembly/🦀️.rs`: `sheet_of(quiz, seed, vector.challenge)`.
- `📏️sorting-concordance`, `🔀️matching-concordance`, `🕸️profile-similarity`: `score_task(..., Some(&answer))`, and an optional answer for `degraded`. The vectors agent has since restructured all three.
- `🧾️learner-lifecycle/🦀️.rs`: the site play ported to the Python `play`:
  - each run starts at `run.challenge`;
  - timed tasks are opened first, and every command uses `at = now + 1`;
  - `perfect_answer` guesses where the keys are hidden;
  - `flawed_answer` swaps guesses too.

## Gates (run from the repo root)

- `cargo test -p semio-framework-quiz --lib`: **175 passed, 0 failed**.
- `cargo clippy -p semio-framework-quiz --lib --tests`: 0 warnings.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/quiz-rs:test`: succeeded, 172 passed, 3 filtered out (`quick` level).
- In `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:
  - `bun ./📜️script.ts subject exhaustive --owner "🧰️framework/🛍️products/❓️quiz" --implementation rust`: the host compiled.
  - `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"`: the first run gave 115/121. The only failures were three new scenarios with no Rust registration yet (`sorting/matching guessed`, `profile timed`). After the vectors agent registered them: **cases=14 executed=129 passed=129 parity=129/129**.
- I did not build or run the proctor.

## Deviations and decisions

1. **Optional answer members are `Option` maps**: `SortingAnswer.guesses`, `MatchingAnswer.assignments` and `MatchingAnswer.guesses`.
   - This lets "present" mean present, including `{}`, exactly as in TypeScript and Python. Before, Rust read `guesses: {}` as absent.
   - Where the keys show, a matching answer without `assignments` is `answer-invalid`, as in TypeScript. The vector `matching-cards-without-assignments` now expects this too.
2. **`BadgeRule::CompletedQuizzes {}` is a struct variant now.** A unit variant silently accepted extra members such as `challenge`, which the vector `badge-challenge-on-completed-quizzes` refuses.
3. These edge cases follow the TypeScript twin exactly:
   - `reach` ignores NaN positions (it compares in a loop rather than using JS `Math.max`).
   - A sheet task is timed when it carries `seconds`.
   - On a keys-shown timed sheet, a missing answer loses every pair and carries no `miss`.
   - Hints skip what they cannot resolve.
   - A hidden profile keeps only the values that belong to an axis, and category icons stay.
   - The crowd treats an item result that carries `miss` as a guess.
   - A sorting where every item misses without a guess adds only its score bin.
   - The nearest authored value takes the smaller of two on a tie.
4. A JSON `null` for an optional member decodes as absent. Every optional member of the twin already behaved this way.

## Notes for the next agent: new or changed public Rust API (crate `quiz`)

- `challenge`:
  - `ChallengeRules { keys, hints, timed, par: f64 }` (serde)
  - `CHALLENGE_RULES: [ChallengeRules; 4]`, in `CHALLENGES` order
  - `challenge_rules(Challenge) -> ChallengeRules`
  - `challenge_rank(Challenge) -> usize`
  - `challenge_meets(Challenge, least: Challenge) -> bool`
  - `points(Score, Challenge) -> f64`
  - `REACH_DECADES: f64`
  - `reach(&[f64], Scale) -> f64` (may be infinite)
  - `misses(value, truth, Scale, reach) -> bool`
  - `TaskSeconds`, `TASK_SECONDS`, `task_seconds(TaskKind, items: usize, dimensions: usize) -> u64`
  - `acted(at, floor, now) -> Timestamp`
  - `hints_of(&Task, &SheetTask, Option<&Answer>) -> Vec<Hint>`
- Schema:
  - `Challenge` (also `PartialOrd`/`Ord` in rank order) and `CHALLENGES`
  - `Command::StartRun{.., challenge}`, `Command::OpenTask{id, learner, run, task, at}`, `Command::RecordAnswer{.., at}`
  - `Event::RunStarted{.., challenge, ..}`, `Event::TaskOpened{learner, run, task, at}`
  - `Rejection::{RunUntimed, TaskUnopened, TimeUp}`
  - `RunResult{quiz, challenge, score, points, tasks}`
  - `RunSummary{.., challenge, .., points}`
  - `LearnerView.best`, `LeaderboardRow.best` and `Standing.best` are now `BTreeMap<Slug, Best>`
  - `RunView{.., opened: Option<BTreeMap<Slug, Timestamp>>, hints: Option<BTreeMap<Slug, Vec<Hint>>>}`
  - `SheetTask::seconds()` and `SheetTask::items()`
  - Item results: `assigned: Option<_>`, `guess`, `miss`
- Other modules:
  - `sheet_of(&Quiz, u32, Challenge)`
  - `score_task(&Task, &SheetTask, Option<&Answer>)`
  - `RunState{run, quiz, challenge, revision, seed, status, answers, recorded, opened, result?, started_at, submitted_at?}`; `opened` is always serialised and required
  - `TranscriptRun{quiz, challenge, score, points, at}`
- **Crowd helpers for the proctor's `CrowdTally`** (`crowd_view` uses them, so a tally that uses them stays bit-equal):
  - `nearest_value(&MatchingTask, dimension: &str, Scale, guess) -> Option<f64>`
  - `crowd_value(&MatchingTask, &Dimension, &MatchingItemResult) -> Option<f64>`: the value an item counts under, or `None` if unanswered.
  - `crowd_orders(&[SortingItemResult]) -> bool`: whether a sorting result places its items.
- The proctor will not compile until it is adapted to the API above. Where it builds or matches `CompletedQuizzes`, it now needs `CompletedQuizzes {}`.
