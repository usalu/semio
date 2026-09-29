# 🦀 Core Rust Report — crate `semio-framework-quiz` (lib `quiz`, nx `@semio-tech/quiz-rs`)

Rust twin of the quiz core per `📓️design.md` §3–§8 and §11. It covers the contract types, randomness, sheet, validation,
scoring, badges, lifecycle and views. Everything is pure, and the only runtime dependency is `serde`. All shared
vectors under `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/` hold bit-exactly, with scores within 1e-12. I aligned the
crate with the TypeScript twin after reading `🔨️modules/*/🟦️.ts`.

## Files

Created (all in `🧰️framework/🛍️products/❓️quiz/`):

| File | Content |
|---|---|
| `🦀️.rs` | product façade: `pub use` of every module (flat `quiz::*` API) |
| `🧬️schema/🦀️.rs` | serde types for every `$defs` entry plus accessors; shared test kit (`fixture`, `typed`, `json`, `assert_close`) |
| `🔨️modules/🎲️randomness/🦀️.rs` | FNV-1a, MT19937, `uniform_index`, `shuffle` |
| `🔨️modules/🃏️sheet/🦀️.rs` | `sheet_of` |
| `🔨️modules/✅️validation/🦀️.rs` | `quiz_issues`, `catalog_issues`, `answer_rejection`, `answer_complete` |
| `🔨️modules/📏️scoring/🦀️.rs` | `score_task`, `score_run` |
| `🔨️modules/🏅️badges/🦀️.rs` | `earned_badges` |
| `🔨️modules/🧾️lifecycle/🦀️.rs` | handles, roster and learner `decide`/`evolve` |
| `🔨️modules/👁️views/🦀️.rs` | `catalog_view`, `learner_view`, `run_view`, `leaderboard` |
| `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📜️script.ts,📋️project.json}` | glue: `#[path]` wiring; `role = "product"`, `id = "quiz"`; targets `build`, `test`, `test-quick`, `test-long`, `test-exhaustive` |

Edited: root `Cargo.toml`. Two anchored lines: the workspace member after the server member, and
`semio-framework-quiz = { path = … }` after `semio-framework-server`.

Unit tests live inline (`#[cfg(test)] mod tests`, with heavier ones in `mod quick`). This mirrors the TS in-source tests.

## Public API (all re-exported at the crate root, `use quiz::*`)

### Types (`🧬️schema`), wire-identical to `🔣️.json`

Aliases: `Slug = String`, `Id = String`, `Timestamp = u64`, `Score = f64`, `Profile = BTreeMap<Slug, f64>`.

Structs, with camelCase fields and `deny_unknown_fields`:
- Quiz side: `Text`, `Quantity`, `Dimension`, `Axis`, `Category`, `ClassificationItem`, `SortingItem`, `MatchingItem`,
  `ClassificationTask`, `SortingTask`, `MatchingTask`, `Quiz`.
- Catalog: `Badge`, `Introduction`, `Catalog`.
- Sheet: `SheetItem`, `SheetClassificationTask`, `SheetSortingTask`, `SheetDimension`, `SheetMatchingTask`, `Sheet`.
- Answers and results: `ClassificationAnswer`, `SortingAnswer`, `MatchingAnswer`, `ClassificationItemResult`,
  `SortingItemResult`, `MatchingItemResult`, `DimensionResult`, `RunResult`.
- Views: `CatalogTaskView`, `CatalogQuizView`, `CatalogBadgeView`, `CatalogView`, `RunView`, `RunSummary`,
  `BadgeAward`, `LearnerView`, `LeaderboardRow`, `Leaderboard`.

The inline view objects have the same names as in the TS twin. Optional fields are `Option` and omitted when `None`.
`Quiz.json_schema` / `Catalog.json_schema` hold `$schema`. `draw`, `position`, `rank`, `runs` and card indices are
`usize`; the seed is `u32`.

Tagged unions, with kebab-case variants:

| Union | Tag | Variants |
|---|---|---|
| `Task`, `SheetTask`, `Answer` | `kind` | newtype variants around the structs above |
| `TaskResult` | `kind` | `Classification { task, score, items }`, `Sorting { … }`, `Matching { task, score, dimensions }` |
| `Identity` | `kind` | `Anonymous`, `Pseudonym { handle }`, `Name { handle }` |
| `BadgeRule` | `kind` | `PerfectQuiz { quiz }`, `PerfectTasks { task_kind: Option<TaskKind>, quiz: Option<Slug> }`, `CompletedQuizzes` |
| `Command` | `type` | `IdentifyLearner { id, learner, identity }`, `StartRun { id, learner, run, quiz }`, `RecordAnswer { id, learner, run, task, answer }`, `SubmitRun { id, learner, run }` |
| `Event` | `type` | `LearnerRegistered { learner, identity, at }`, `LearnerRecalled { learner, at }`, `RunStarted { learner, run, quiz, revision, seed: u32, at }`, `RunVoided { learner, run, at }`, `AnswerRecorded { learner, run, task, answer, at }`, `RunSubmitted { learner, run, result, at }`, `BadgeAwarded { learner, badge, run, at }` |
| `Query` | `type` | `Catalog`, `Learner { learner }`, `Run { run }`, `Leaderboard` |

Plain enums: `Scale { Linear, Logarithmic }`, `TaskKind`, `RunStatus { Open, Submitted, Voided }`, and `Rejection` (10
variants).

Accessors, for the proctor's `quiz.<type>` wire mapping in §9a:

```rust
impl Task      { pub fn id(&self) -> &Slug; pub fn kind(&self) -> TaskKind; pub fn title(&self) -> &Text }
impl SheetTask { pub fn id(&self) -> &Slug; pub fn kind(&self) -> TaskKind }
impl TaskResult{ pub fn task(&self) -> &Slug; pub fn score(&self) -> Score }
impl Command   { pub fn id(&self) -> &Id; pub fn learner(&self) -> &Id; pub fn type_name(&self) -> &'static str }
impl Event     { pub fn learner(&self) -> &Id; pub fn at(&self) -> Timestamp; pub fn type_name(&self) -> &'static str }
impl Query     { pub fn type_name(&self) -> &'static str }
impl Rejection { pub fn as_str(&self) -> &'static str }
```

### Functions

```rust
// 🎲️ randomness (§3)
pub fn fnv1a32(text: &str) -> u32
pub fn run_seed(run: &str) -> u32
pub struct Mt19937;  impl Mt19937 { pub fn new(seed: u32) -> Self; pub fn next_u32(&mut self) -> u32 }
pub fn uniform_index(random: &mut Mt19937, n: usize) -> usize            // n ≤ 1 → 0 without a draw
pub fn shuffle<T: Clone>(random: &mut Mt19937, items: &[T]) -> Vec<T>
// 🃏️ sheet (§4)
pub fn sheet_of(quiz: &Quiz, seed: u32) -> Sheet
// ✅️ validation (§5 + documents)
pub const QUIZ_SCHEMA: &str;  pub const CATALOG_SCHEMA: &str;
pub struct ValidationIssue { pub path: String, pub code: IssueCode }    // serde: {"path","code"}
pub enum IssueCode { … }  impl IssueCode { pub fn as_str(&self) -> &'static str }
pub fn quiz_issues(quiz: &Quiz) -> Vec<ValidationIssue>
pub fn catalog_issues(catalog: &Catalog, quizzes: &[Quiz]) -> Vec<ValidationIssue>   // quizzes in catalog order; does NOT include quiz_issues
pub fn answer_rejection(sheet_task: &SheetTask, answer: &Answer) -> Option<Rejection>
pub fn answer_complete(sheet_task: &SheetTask, answer: Option<&Answer>) -> bool
pub fn is_slug(value: &str) -> bool
pub fn pointer(base: &str, key: &str) -> String                          // RFC 6901 token escaping
// 📏️ scoring (§6)
pub fn scaled(scale: Scale, value: f64) -> f64
pub fn score_task(task: &Task, sheet_task: &SheetTask, answer: &Answer) -> Option<TaskResult>              // None unless valid + complete
pub fn score_run(quiz: &Quiz, sheet: &Sheet, answers: &BTreeMap<Slug, Answer>) -> Option<RunResult>       // None unless every sheet task is answered completely
// 🏅️ badges (§7)
pub fn earned_badges<Q: Borrow<Quiz>, R: Borrow<RunResult>>(badges: &[Badge], quizzes: &[Q], results: &[R], held: &BTreeSet<Slug>) -> Vec<Slug>
// 🧾️ lifecycle (§8)
pub struct NormalizedHandle { pub display: String, pub key: String }
pub fn normalize_handle(handle: &str) -> Option<NormalizedHandle>
pub enum Decision { Events(Vec<Event>), Rejection(Rejection) }          // serde: {"events":[…]} | {"rejection":"…"}
pub struct RosterState { pub handles: BTreeMap<String, Id> }           // Default; serde camelCase
pub fn empty_roster_state() -> RosterState
pub fn decide_roster(state: &RosterState, command: &Command, now: Timestamp) -> Decision   // non-identify commands → Events([])
pub fn evolve_roster(state: &mut RosterState, event: &Event)
pub struct RunState { pub run, pub quiz, pub revision: String, pub seed: u32, pub status: RunStatus, pub answers: BTreeMap<Slug, Answer>, pub result: Option<RunResult>, pub started_at: Timestamp, pub submitted_at: Option<Timestamp> }
pub struct LearnerState { pub learner: Id, pub identity: Option<Identity>, pub runs: Vec<RunState>, pub badges: Vec<BadgeAward>, pub last_activity: Option<Timestamp> }   // serde camelCase
impl LearnerState { pub fn run(&self, run: &str) -> Option<&RunState> }
pub fn empty_learner_state(learner: &str) -> LearnerState
pub struct LoadedQuiz { pub quiz: Quiz, pub revision: String }
pub struct LearnerContext<'a> { pub now: Timestamp, pub catalog: &'a Catalog, pub quizzes: &'a BTreeMap<Slug, LoadedQuiz> }   // Copy
pub fn decide_learner(state: &LearnerState, command: &Command, context: &LearnerContext<'_>) -> Decision   // identify-learner → Events([])
pub fn evolve_learner(state: &mut LearnerState, event: &Event)         // ignores events of other learners
// 👁️ views
pub fn catalog_view(catalog: &Catalog, quizzes: &[Quiz]) -> CatalogView
pub fn learner_view(state: &LearnerState, catalog: &CatalogView) -> Option<LearnerView>                     // None before registration
pub fn run_view(state: &LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>) -> Option<RunView>
pub fn learner_tag(learner: &str) -> String                             // format!("{:08x}", fnv1a32(learner))
pub fn leaderboard<'a>(states: impl IntoIterator<Item = &'a LearnerState>, catalog: &CatalogView) -> Leaderboard   // rows carry `tag`, never the learner id
```

### What the proctor needs to know

- The roster state and the learner state serialize with serde, so they can go straight into `ActorState.bytes`.
- **The proctor must fold the roster's `learner-registered` event into that learner's `LearnerState` with
  `evolve_learner`.** Without it, `start-run` rejects with `unknown-learner`.
- Events are always addressed to `state.learner`.

## Validation vocabulary (identical to `✅️validation/🟦️.ts`)

Issues are JSON Pointers, deduplicated and sorted by path and then code, in code point order.

Codes the Rust twin emits:

| Code | Path |
|---|---|
| `value-invalid` | `/schema` |
| `slug-invalid` | the slug, including profile and values keys |
| `length-invalid` | texts `/…/en`, `/…/de`; units 1…32; emoji 1…16; catalog paths |
| `below-minimum` | `/…/draw` when below 2 |
| `draw-exceeds-items` | `/…/draw` |
| `items-too-few` | the array |
| `properties-too-few` | the profile or values object |
| `duplicate-id` | `<array>/<i>/id`; for loaded catalog quizzes, `/quizzes/<i>` |
| `duplicate-path` | `/quizzes/<i>` |
| `axis-range-invalid` | `/axes/<i>/max` |
| `axes-missing` | the profile, when the task has no `axes` |
| `profile-incomplete` | `profile/<axis>` |
| `axis-unknown` | `profile/<key>` |
| `profile-out-of-range` | `profile/<key>` |
| `category-unknown` | `items/<i>/category`, only when it is a valid slug |
| `value-not-positive` | sorting `items/<i>/value` or matching `values/<dim>` on a log scale |
| `value-missing` | `values/<dim>` |
| `dimension-unknown` | `values/<key>` |
| `quiz-count-mismatch` | `/quizzes` |
| `quiz-unknown` | `/badges/<i>/rule/quiz` |
| `badge-unreachable` | `/badges/<i>/rule` |

`IssueCode` also has `type-invalid`, `required`, `property-unknown` and `integer-invalid`, so TS output deserializes.
In Rust those cases are serde errors instead.

## How I aligned with the TS twin and the Python oracle

I adopted TS semantics wherever mine differed:
- **Whitespace:** Unicode `White_Space` (`char::is_whitespace`, which is `\p{White_Space}`).
- **Roster:** `evolve_roster` overwrites the key.
- **Learner evolve:** `evolve_learner` overwrites the identity, appends badges and keeps `lastActivity` optional.
- **Starting a run:** `start-run` voids the first open run of the quiz. It refuses a reused run id with `run-open` or
  `run-closed`.
- **Answers and submissions:** record and submit do not check `command.learner`.
- **Names:** `LoadedQuiz` and `ValidationIssue`.
- **Catalog validation:** `catalog_issues` no longer nests quiz issues.

## Commands run and results

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | **69 passed, 0 failed**. This includes 11 fixture tests: randomness, sheets, answers, schema-conformance and clean fixture docs, three scoring files, badges, handles and roster, learner decisions, learner and run views, leaderboards, and the serde round-trip of every fixture document |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz --all-targets` | clean (0 warnings) |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts test` (in `📦️packages/🦀️rust`) | 67 passed, 2 `quick::` tests filtered out (fundamental level) |
| `bun ./📜️script.ts test quick` | 69 passed |
| `bun nx run @semio-tech/quiz-rs:test-long --skip-nx-cache` | Successfully ran |
| `bun nx run @semio-tech/quiz-rs:build --skip-nx-cache` | Successfully ran; output goes to the git-ignored `dist/build` |
| numpy `RandomState(seed).randint(0, 2**32)` cross-check | confirms seed 5489 → 3499211612, seed 1 → 1791095845 …, seed 0 → 2357136044 |

## Open issues and notes

1. **Design §6 wording:** "with s = rank the score equals (1+τ_a)/2" only holds for *unit* pair weights. The Python
   oracle checks it correctly that way (`concordance(…, 1 if distinct)` against scipy's τ_b). The implementation is
   unaffected.
2. **Reused run id:** Rust and TS refuse a `start-run` with an already-used run id. The Python oracle does not check
   this, and no vector covers it, so conformance could add one.
3. **Order of `learner_view.runs`:** Python orders by `startedAt` descending; TS and Rust reverse the start order.
   They only differ on equal or non-monotone `startedAt`.
4. **Learner total:** Python's total sums every best, while TS and Rust sum `best × 100` over catalog quizzes in catalog
   order. This is equal for all fixtures.
5. **Edge conventions (only invalid input):** the mean of an empty set is `0` in Rust (TS gives `NaN`).
   `score_task`/`score_run` return `None` where TS throws. `sheet_of` emits a `NaN` card for a matching item missing a
   dimension.
6. **Fixture catalog issue (resolved):** the conformance agent dropped the phantom badge `matching-in-cooling` from the
   `🧾️learner-lifecycle` catalog when it regenerated the fixtures at 17:39. Every fixture catalog is now asserted to
   have no issues.
7. **Taxonomy:** the new directory `📦️packages/🦀️rust` (and the `🔨️modules/*` directories shared with TS) needs
   registering. That is site-infra's job per design §12.

## Follow-up fixes

### Contract change: `LeaderboardRow.learner` becomes `tag`

The learner id is a bearer credential, so it must never appear in the public leaderboard.

- **`🧬️schema/🦀️.rs`:** `LeaderboardRow` now has `pub tag: String` in place of `learner: Id`, with a docstring
  explaining why.
- **`👁️views/🦀️.rs`:** adds `pub fn learner_tag(learner: &str) -> String`, which returns `format!("{:08x}",
  fnv1a32(learner))`. It is re-exported as `quiz::learner_tag`.
  - `leaderboard` sorts `(learner id, row)` pairs, so the ordering is unchanged: total ↓, badges ↓, `reachedAt` ↑, then
    the hidden learner id ↑. It assigns ranks and then drops the id.
- **New tests:**
  - `learner_tags_are_zero_padded_fnv1a_hex` checks `""` → `811c9dc5`, `"a"` → `e40c292c`, and a hash below
    `0x10000000` that starts with `0`.
  - `leaderboard_rows_never_carry_the_learner_id_and_ties_break_by_the_hidden_id` picks two ids whose tag order is the
    reverse of their id order. It asserts the id order, and asserts that the serialized board contains neither id nor
    a `learner` key.
  - The existing leaderboard tests now compare tags.
- **`teaching-proctor`:** I made two minimal test edits. `🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs` compares
  `row.tag` with `quiz::learner_tag(ADA)`. `🧪️tests/🌐️end-to-end/🦀️.rs` compares `row["tag"]` with
  `quiz::learner_tag(&ada)` and additionally asserts that the served board JSON never contains Ada's id. No production
  code in the proctor needed changes, because `quiz::leaderboard` produces the rows.

### Fixture changes by the conformance agent

- The phantom badge `matching-in-cooling` is gone from the `🧾️learner-lifecycle` catalog. The validation test now
  asserts that every fixture catalog has no issues.
- The regenerated `🏆️leaderboard` vectors carry `tag`, and the shared-leaderboard test holds against them.

### Commands and results

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz **71 passed**; proctor unit **36 passed**, conformance **14 passed**, end-to-end **2 passed**; 0 failed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz -p teaching-proctor --all-targets` | 0 findings in quiz or proctor sources (the remaining warnings are pre-existing, in upstream framework crates) |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts test quick` (in `📦️packages/🦀️rust`) | 71 passed |

### Test layout: inline tests moved to canonical test implementations

The Protocol v2 contract check reported 79 high-priority `testing/layout` breaches (`inline-test-body`) against the
quiz owner. They came from the trailing `#[cfg(test)] mod tests { … }` blocks in `🧬️schema/🦀️.rs` and
`🔨️modules/*/🦀️.rs`.

**The move:** every test body now lives in `<owner>/🧪️tests/🔬️unit/🦀️.rs`, the same layout the server product uses,
which has 0 breaches.
- The eight owners are `🧬️schema` and `🔨️modules/{🎲️randomness,🃏️sheet,✅️validation,📏️scoring,🏅️badges,🧾️lifecycle,👁️views}`.
- Each implementation file ends in `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`.
- It is `pub(crate) mod tests;` for `schema`, `sheet` and `lifecycle`, whose shared test kits the other test modules
  import.
- The module paths are unchanged (`<module>::tests::…`, `<module>::tests::quick::…`). The `mod quick` level
  convention, the `--skip quick::` filtering and all fixture-reading tests keep working.
- Each test file gets a `//!` header with a unique emoji and a `@see` link to the implementation under test.

**The script:** the move was mechanical, done by `core_rust_extract_tests.py` in this ticket folder, which I kept as an
input script. It extracts the trailing block, removes one indentation level and writes the wiring.

| Command | Result |
|---|---|
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/❓️quiz"` (from `🦑️repo/🔨️modules/🧪️test`) | **0 breaches under `❓️quiz`**, down from 79. The repo-wide total fell from 5125 to 5046; the rest belong to other owners, and the exit code 1 comes from them. `🎓️teaching` has 0 as well |
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz 71 passed; proctor 36 + 14 + 2 passed; 0 failed |
| `bun ./📜️script.ts test` / `test quick` (in `📦️packages/🦀️rust`) | 69 passed with 2 `quick::` filtered / 71 passed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz -p teaching-proctor --all-targets` | 0 findings in quiz or proctor sources |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `level=exhaustive cases=10 executed=75 passed=75 failed=0 errored=0 parity=75/75` |

**Taxonomy:** the new directories `🧪️tests/🔬️unit` under the eight owners follow the server product's existing
precedent. If the taxonomy registry needs entries for them, that is site-infra's job.
