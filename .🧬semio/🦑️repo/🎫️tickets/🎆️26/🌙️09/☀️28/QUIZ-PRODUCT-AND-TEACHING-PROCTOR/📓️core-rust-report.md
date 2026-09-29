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

## 2026-09-29

### Contract change (design §14): required quiz emoji

`Quiz.emoji` and `CatalogQuizView.emoji` are now required strings of 1–16 code points.

- **`🧬️schema/🦀️.rs`:** `Quiz` gains `pub emoji: String` after `id`, and `CatalogQuizView` gains
  `pub emoji: String` after `id`. Both are required on the wire; a missing field is a serde error.
- **`quiz_issues`:** reports `length-invalid` at `/emoji` for 0 or more than 16 code points. This is the same rule
  and code the TS twin now uses (`string(json.emoji, "/emoji", report, 1, 16)`); the TS twin's `required` for a missing
  field is a deserialization error in Rust.
  - Neither twin checks "one emoji grapheme". The JSON Schema only constrains the length, and a grapheme check would
    need Unicode segmentation, which is a library.
- **`catalog_view`:** copies `quiz.emoji` into `CatalogQuizView`.
- **Tests:**
  - The shared test quiz carries `⚡`, and the structure test now also expects `/emoji` `length-invalid`.
  - The new test `quiz_emoji_is_required_and_holds_one_to_sixteen_code_points` accepts `⚡`, `❄️`, a ZWJ family and 16
    code points. It rejects `""` and 17 code points, and asserts that a document without `emoji` fails to deserialize.
  - `catalog_view_is_solution_free` asserts the copied emoji.
  - The regenerated shared fixtures (16:30) already carry quiz emojis, and every fixture test holds against them.
- **`teaching-proctor`:** I made only the edits the field requires. `⚡` went into `🧫️fixtures/⚡️power/🔣️.json` and
  `🏠` into `🧫️fixtures/🏠️homes/🔣️.json`. The inline test document in
  `🔨️modules/📚️catalog/🧪️tests/🔬️unit/🦀️.rs` (`quiz_document`) got `"emoji":"🧪"`. The site content quizzes
  already carried emojis.

### Commands and results

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz **72 passed**; proctor unit **38**, conformance **14**, end-to-end **2** passed; 0 failed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz -p teaching-proctor --all-targets` | 0 findings in quiz or proctor sources |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `cargo run -p teaching-proctor --bin proctor -- check 🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` | exit 0: 4 quizzes, 7 badges |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/❓️quiz"` | 0 breaches under `❓️quiz` and under `🎓️teaching` |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `parity=75/75` |

### Shared presence and cursors (design §15) — module `🔨️modules/👥️presence/`

The module emoji `👥️` is the taxonomy's registered emoji for `presence`. The TS twin belongs beside it as
`🔨️modules/👥️presence/🟦️.ts`.

**Schema twins in `🧬️schema/🦀️.rs`, region Presence:**

```rust
pub enum Screen { Introduction, Identity, Home, Run, Results, Leaderboard }            // kebab-case strings
pub struct Place { pub screen: Screen, pub quiz: Option<Slug>, pub task: Option<Slug> }  // optional fields omitted when None
pub type Anchor = String;                                                                // ^[a-z0-9]+(?:[:-][a-z0-9]+)*$, 1…64
pub struct Cursor { pub anchor: Anchor, pub x: f64, pub y: f64 }
pub struct PresenceState { pub tag: String, pub identity: Identity, pub place: Place, pub active: bool }
pub struct CursorState { pub tag: String, pub cursor: Option<Cursor>, pub focus: Option<Anchor> }
```

All structs use `deny_unknown_fields`. Unknown members (for example a `learner` id or an `item`), unknown screens and
missing required fields are serde errors.

**API, re-exported as `quiz::*`:**

```rust
pub fn roster_scope(catalog: &str) -> String                                   // "<catalog>"
pub fn room_scope(catalog: &str, place: &Place) -> Option<String>              // "<catalog>/introduction|home|leaderboard", run|results → "<catalog>/quiz/<quiz>"; None for identity (no tag yet, shared with nobody) and for run|results without quiz; the task never splits a room
pub fn presence_issues(state: &PresenceState) -> Vec<ValidationIssue>         // all, sorted by path then code (TS presenceIssues)
pub fn cursor_issues(state: &CursorState) -> Vec<ValidationIssue>             // all, sorted by path then code (TS cursorIssues)
pub fn presence_problem(state: &PresenceState) -> Option<ValidationIssue>      // first of presence_issues; None = admit
pub fn cursor_problem(state: &CursorState) -> Option<ValidationIssue>          // first of cursor_issues; None = admit
pub fn is_tag(value: &str) -> bool                                             // ^[0-9a-f]{8}$
pub fn is_anchor(value: &str) -> bool                                          // ^[a-z0-9]+(?:[:-][a-z0-9]+)*$ and ≤ 64
```

**Problem codes.** They extend the shared `IssueCode` vocabulary with `tag-invalid`, `anchor-invalid`,
`out-of-range`, `quiz-outside-run` and `task-without-run`. They are identical to the TS twin's
`presenceIssues`/`cursorIssues` in `✅️validation/🟦️.ts`, whose code I read after it landed and aligned to. The problem
returned is the first in path-then-code order.

| Path | Code | When |
|---|---|---|
| `/tag` | `tag-invalid` | not 8 lowercase hex digits |
| `/identity/handle` | `length-invalid` | pseudonym or name handle outside 1…64 code points |
| `/place/quiz`, `/place/task` | `slug-invalid` | not a slug |
| `/place/quiz` | `required` | a `run` or `results` place without a quiz (so `room_scope` is always defined for an admitted state) |
| `/place/quiz` | `quiz-outside-run` | a quiz on any screen other than `run` or `results` |
| `/place/task` | `task-without-run` | a task on any screen other than `run` |
| `/cursor/anchor`, `/focus` | `anchor-invalid` | not an anchor |
| `/cursor/x`, `/cursor/y` | `out-of-range` | not a finite number in 0…1 |

**Proctor usage (`presence_admission(scope, state)`):**
1. If `scope == roster_scope(catalog)`, parse the state as `PresenceState` and refuse on a serde error or on
   `presence_problem`.
2. Otherwise, if the scope is one of the room scopes (`<catalog>/introduction`, `<catalog>/home`,
   `<catalog>/leaderboard`, plus `"<catalog>/quiz/<id>"` for each catalog quiz), parse it as `CursorState` and
   refuse on `cursor_problem`. The identity screen has no room.
3. Refuse any other scope.

A suitable refusal reason is `format!("{} at {}", issue.code.as_str(), issue.path)`. Optionally, the proctor can also
require `state.tag == quiz::learner_tag(<principal learner id>)`.

**Tests:** `🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs`, wired by `#[cfg(test)] #[path] mod tests;`, has 6 tests:
- every screen's room, including identity → `None` and no task splitting;
- the presence admission table;
- the place rules, with full sorted issue lists;
- the cursor admission table, including NaN and ±∞;
- the shared vectors of `🧫️fixtures/👥️shared-presence` (10 scope vectors, 27 presence, 26 cursor). Admitted means the
  state parses with serde and `*_problem` is `None`; every vector agrees with the Python oracle.
- the wire shapes, including refusal of unknown members and unknown screens.

**Coordinator decisions applied:**
- The identity screen has no room: `room_scope` returns `None` for it, matching the TS twin and the regenerated
  fixture.
- `quiz-outside-run` and `task-without-run` are enforced, matching the TS twin and the fixture.

**Results:**

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz **78 passed**; proctor 33 + 14 + 2 passed; 0 failed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz -p teaching-proctor --all-targets` | 0 findings in quiz or proctor sources |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/❓️quiz"` | 0 breaches under `❓️quiz` |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `cases=11 executed=84 passed=84 parity=84/84` |

### Design §16: new screens `quiz`, `learner`, `badges`, `preferences`

**Schema (`🧬️schema/🦀️.rs`):**
- `Screen` gains `Quiz`, `Learner`, `Badges` and `Preferences`, in schema order: introduction, identity, home, quiz,
  run, results, leaderboard, learner, badges, preferences.
- New constant `pub const SCREENS: [Screen; 10]` in schema order, the twin of TS `SCREENS`, so consumers can derive
  rooms from `room_scope` alone.

**`room_scope` (`👥️presence/🦀️.rs`):**

| Screen | Room |
|---|---|
| `quiz`, `run`, `results` | `<catalog>/quiz/<quiz>`; `None` without a quiz |
| `badges` | `<catalog>/badges` |
| `introduction`, `home`, `leaderboard` | unchanged |
| `identity`, `learner`, `preferences` | `None` |

**`presence_issues`:** the codes are unchanged, but the screen sets change.
- `required` at `/place/quiz` now covers the quiz, run and results screens without a quiz.
- `quiz-outside-run` covers a quiz on any other screen, including badges, learner and preferences.
- `task-without-run` still applies to a task on any screen except run, including the quiz screen.

This is identical to the TS twin's `roomScope` switch, which I checked after it landed.

**Tests (`👥️presence/🧪️tests/🔬️unit/🦀️.rs`):**
- The room table covers all new screens.
- The place-rule test covers: a quiz page with a quiz (admitted), without one (`required`), and with a task
  (`task-without-run`); and badges, learner and preferences with a quiz (`quiz-outside-run`) or without one (admitted).

**`teaching-proctor`:** this was a minimal change. `Rooms::of` in `🔨️modules/👥️presence/🦀️.rs` used a fixed page
list (`Introduction, Home, Leaderboard`), so `<catalog>/badges` was not admitted.
- It now maps every screen in `quiz::SCREENS` through `room_scope`, which admits `<catalog>/badges` and leaves the
  personal pages without rooms. Future screens need no proctor change.
- I also updated its module doc and three tests:
  - `🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs`: the scope list includes `proctor-fixture/badges`; `learner` and
    `preferences` are not rooms; the cursor admission loop includes badges.
  - `🔨️modules/🧩️instance/🧪️tests/🔬️unit/🦀️.rs`: the policy admits join and publish on `proctor-fixture/badges` and
    denies `learner` and `preferences`.

**Results:**

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz **78 passed**; proctor unit **39**, conformance **14**, end-to-end **4** passed; 0 failed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz -p teaching-proctor --all-targets` | 0 findings in quiz or proctor sources |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/❓️quiz"` | 0 breaches under `❓️quiz` and under `🎓️teaching` |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `cases=11 executed=84 passed=84 parity=84/84` |

### Design §17: crowd view, thinking room, item and category anchors, drag (published first; normative for the TS twin)

**Schema twins (`🧬️schema/🦀️.rs`):**

```rust
pub struct CrowdCount { pub key: String, pub count: usize }
pub struct CrowdItem { pub item: Slug, pub answers: usize, pub counts: Option<Vec<CrowdCount>>, pub mean_position: Option<f64> }   // wire: meanPosition; None fields omitted
pub struct CrowdTask { pub task: Slug, pub kind: TaskKind, pub dimension: Option<Slug>, pub items: Vec<CrowdItem> }
pub struct CrowdView { pub quiz: Slug, pub runs: usize, pub tasks: Vec<CrowdTask> }
pub struct CursorDrag { pub item: Slug }                                        // the inline `drag` object
pub struct CursorState { pub tag: String, pub cursor: Option<Cursor>, pub focus: Option<Anchor>, pub drag: Option<CursorDrag> }
pub struct ThinkingState { pub tag: String, pub answers: BTreeMap<Slug, ThinkingAnswer> }
pub enum ThinkingAnswer { Classification(ClassificationAnswer), Sorting(SortingAnswer), Matching(ThinkingMatchingAnswer) }   // tag "kind"
pub struct ThinkingMatchingAnswer { pub values: BTreeMap<Slug, BTreeMap<Slug, f64>> }   // dimension → item → value (card indices are publisher-local and never shared)
pub enum Query { …, Crowd { quiz: Slug } }                                      // {"type":"crowd","quiz":…}; type_name "crowd"
```

**API, re-exported as `quiz::*`:**

```rust
pub fn crowd_view<R: Borrow<RunResult>>(quiz: &Quiz, results: &[R]) -> CrowdView   // 👁️views
pub fn json_number_text(value: f64) -> String                                       // 👁️views (TS jsonNumberText)
pub fn thinking_scope(catalog: &str, quiz: &str) -> String                         // 👥️presence: "<catalog>/quiz/<quiz>/thinking"
pub const THINKING_LIMIT: usize = 64;                                               // 👥️presence
pub fn thinking_issues(state: &ThinkingState) -> Vec<ValidationIssue>               // 👥️presence, sorted by path then code
pub fn thinking_problem(state: &ThinkingState) -> Option<ValidationIssue>           // first of thinking_issues; None = admit
```

`cursor_issues` and `cursor_problem` keep their signatures and gain the drag rule below.

**`crowd_view(quiz, results)`, exact definition:**
- **Results:** only results with `result.quiz == quiz.id` count; `runs` is their number. A task result counts only when
  its `task` equals the quiz task's id and its kind equals the quiz task's kind.
- **Tasks:** every quiz task in definition order, always present even with no items. A matching task yields one
  `CrowdTask` per dimension in definition order, with `dimension: Some(id)`. The other kinds have `dimension: None`.
- **Items:** the quiz task's items in definition order. An item is left out when no counted result answered it.
  `answers` is the number of counted results that contain the item (per dimension for matching).
- **Classification:** `counts` per assigned category id, and no `meanPosition`.
- **Matching:** `counts` per assigned value, keyed by `json_number_text(assigned)`, and no `meanPosition`.
- **Count order:** counts are in ascending key order by code point (UTF-8 byte order; Rust `BTreeMap` order). This
  applies to value keys too, so `"120" < "18" < "5"`.
- **Sorting:** `meanPosition` is set and `counts` is omitted. Each result contributes the normalized position
  `position / (n − 1)`, where `position` is the item's zero-based position in the learner's order and `n` is the
  number of items in that result's sorting (`0` when `n < 2`). The mean sums these in result order starting at `0`,
  then divides by `answers`.

**`json_number_text(v)`** is ECMAScript `Number.prototype.toString` (= `String(v)` = `JSON.stringify(v)` for finite
`v`), verified against bun's `String(v)` on 25 values:
- It uses the shortest round-trip digits.
- It uses plain notation for decimal exponents −7 < e < 21, and otherwise `d.ddd e±x` without spaces, e.g. `1e+21` and
  `1.5e-7`.
- `-0` renders as `"0"`. NaN and ±∞ render as `"NaN"` and `"±Infinity"`, which cannot occur for quiz values.
- Examples: `42.6`, `50`, `0.027`, `0.30000000000000004`, `100000000000000000000`, `1e+21`, `0.000001`, `1e-7`, `-3.5`.

**Cursor rules (`cursor_issues`):**
- An anchor (`/cursor/anchor` or `/focus`) only needs to match the one Anchor pattern, which covers cards,
  `item:<id>` and `category:<id>`, else `anchor-invalid`. I first published an extra "slug after `item:`/`category:`"
  rule, then dropped it to match the TS twin and the conformance vectors.
- `drag.item` must be a slug, else `slug-invalid` at `/drag/item`.

**`thinking_issues(state)` (paths are JSON Pointers with RFC 6901 escaping; limit = `THINKING_LIMIT` = 64):**

| Path | Code | When |
|---|---|---|
| `/tag` | `tag-invalid` | not 8 lowercase hex digits |
| `/answers` | `too-many` | more than 64 tasks |
| `/answers/<task>` | `slug-invalid` | task key not a slug |
| `/answers/<task>/assignments` | `too-many` | classification assignments beyond 64 |
| `/answers/<task>/assignments/<item>` | `slug-invalid` | classification item key or category value not a slug |
| `/answers/<task>/order` | `too-many` | sorting order beyond 64 |
| `/answers/<task>/order/<index>` | `slug-invalid` | not a slug |
| `/answers/<task>/order/<index>` | `duplicate-id` | an item repeated at a later index |
| `/answers/<task>/values` | `too-many` | matching dimensions beyond 64 |
| `/answers/<task>/values/<dimension>` | `slug-invalid` | dimension key not a slug |
| `/answers/<task>/values/<dimension>` | `too-many` | more than 64 items in the dimension |
| `/answers/<task>/values/<dimension>/<item>` | `slug-invalid` | item key not a slug |
| `/answers/<task>/values/<dimension>/<item>` | `type-invalid` | value not a finite number |

Partial answers are fine: thinking is a draft and is never checked against a sheet. There is one new code, `too-many`.

The matching rows follow the coordinator's contract change to `ThinkingAnswer`/`ThinkingMatchingAnswer { values }`.
Semantic values replace the publisher-local card indices, so the earlier card-index `out-of-range` rule is gone. A
matching draft still carrying `assignments` is a serde error.

**Differences from the TS twin (for the TS agent):**
- **Count key order:** TS `counted(keys, numeric)` sorts matching value keys numerically (`"50" < "120"`). The Rust
  twin, my published definition and the Python oracle of `🧫️fixtures/📊️crowd-view` all use code point order for
  every key (`"120" < "50"`); the fixture vectors `energy-three-runs` (`['120', '50']`, `['0', '0.2']`) hold for Rust.
  TS must switch to code point order.
- **Thinking values:** the TS validation still checks thinking drafts against the old matching `assignments` shape
  (card indices). It needs the `values` rules in the table above.

**Tests:**
- 🔬️unit views: `json_number_text` against 25 JavaScript `String(v)` vectors; crowd counts, mean positions, dimensions,
  key order and JSON shape; empty quiz; all shared `📊️crowd-view` vectors.
- 🔬️unit presence: item and category anchors and drag; thinking drafts with values, the full sorted issue list, NaN/∞,
  `assignments` refused; every limit; all shared `👥️shared-presence` vectors, including `thinkingScopes` and
  `thinking`.

**Results:**

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test --no-fail-fast -p semio-framework-quiz -p teaching-proctor` | quiz **85 passed**; proctor unit **44**, conformance **14**, end-to-end **5** passed; 0 failed |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz --all-targets` | 0 findings in quiz sources |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p semio-framework-quiz --no-deps` | clean |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/❓️quiz"` | 0 breaches under `❓️quiz` and under `🎓️teaching` |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `cases=12 executed=93 passed=93 parity=93/93` |

**Proctor usage:**
- Admit `thinking_scope(catalog, quiz)` for every catalog quiz. Parse the state as `ThinkingState` and refuse on a
  serde error or on `thinking_problem`.
- Fold `run-submitted` results per quiz and answer `quiz.crowd` with `crowd_view(&quiz, &results)`.
