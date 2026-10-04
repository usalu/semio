# 📊 Crowd distributions — schema, both cores, proctor (design §20 "Crowd view")

Scope: the core/proctor half of §20. The react target, the react tests and `🧫️fixtures/💭️crowd-client` were not touched.

## Final contract

### TypeScript (`🧰️framework/🛍️products/❓️quiz/🧬️schema/🟦️.ts`, `🔨️modules/👁️views/🟦️.ts`)

```ts
export type CrowdCount = { readonly key: string; readonly count: number };
export type CrowdItem = { readonly item: Slug; readonly answers: number; readonly counts?: readonly CrowdCount[]; readonly meanPosition?: number; readonly places?: readonly number[] };
export const CROWD_SCORE_BINS = 10;
export type CrowdScores = readonly number[];
export type CrowdTask = { readonly task: Slug; readonly kind: TaskKind; readonly dimension?: Slug; readonly scores: CrowdScores; readonly items: readonly CrowdItem[] };
export type CrowdView = { readonly quiz: Slug; readonly runs: number; readonly scores: CrowdScores; readonly tasks: readonly CrowdTask[] };

export function scoreBin(score: Score): number;
export function presented(task: Task): number;
export function placeBin(position: number, length: number, places: number): number;
export function crowdView(quiz: Quiz, results: readonly RunResult[]): CrowdView;
```

All of them are exported from `@semio-tech/quiz` (the package glue re-exports the schema twin and the views module with `export *`, nothing to add there).

### Rust (`🧬️schema/🦀️.rs`, `🔨️modules/👁️views/🦀️.rs`, re-exported by the `quiz` crate façade)

```rust
pub const CROWD_SCORE_BINS: usize = 10;
pub type CrowdScores = [usize; CROWD_SCORE_BINS];
pub struct CrowdItem { pub item: Slug, pub answers: usize, pub counts: Option<Vec<CrowdCount>>, pub mean_position: Option<f64>, pub places: Option<Vec<usize>> }
pub struct CrowdTask { pub task: Slug, pub kind: TaskKind, pub dimension: Option<Slug>, pub scores: CrowdScores, pub items: Vec<CrowdItem> }
pub struct CrowdView { pub quiz: Slug, pub runs: usize, pub scores: CrowdScores, pub tasks: Vec<CrowdTask> }

pub fn score_bin(score: Score) -> usize;
pub fn presented(task: &Task) -> usize;
pub fn place_bin(position: usize, length: usize, places: usize) -> usize;
pub fn crowd_view<R: Borrow<RunResult>>(quiz: &Quiz, results: &[R]) -> CrowdView;
```

### Semantics as implemented

- `scoreBin(score) = min(9, ⌊⌊score · 100 + ½⌋ / 10⌋)`, never below 0 (Rust by the saturating cast, TypeScript by `Math.max(0, …)`).
- `presented(task) = min(draw, items.length)`, every item without `draw`; it takes any task kind.
- `placeBin(i, n, m) = n < 2 ? 0 : ⌊(2·i·(m−1) + (n−1)) / (2·(n−1))⌋` in integer arithmetic. Two additions make it a total function and change nothing for a valid position (`i ≤ n − 1`): the result never exceeds `m − 1`, and `m < 1` gives 0. Rust multiplies and adds saturating, so a broken result cannot overflow or index out of a `places` array (a panic there would stop a projector).
- `CrowdView.scores`: run scores of the results of the viewed quiz, sums to `runs`.
- `CrowdTask.scores`: the task score of each result's first task result of the task's id and kind (classification, sorting); for a matching crowd task the score of the first dimension of that id inside that task result, so a result without the dimension does not count.
- `CrowdItem.places`: sorting items only, `presented(task)` counts, sums to `answers`; an item nobody ordered is still left out.
- Empty crowd: ten zeros in the view's and in every task's `scores`, no items.
- JSON member order follows the schema: `quiz, runs, scores, tasks` / `task, kind, dimension?, scores, items` / `item, answers, meanPosition, places`.

## What changed, per file

Quiz product `🧰️framework/🛍️products/❓️quiz/`:

| File | Change |
| --- | --- |
| `🧬️schema/🔣️.json` | New `$defs/CrowdScores` (array of exactly 10 integers ≥ 0); `CrowdTask.scores` and `CrowdView.scores` required and referencing it; `CrowdItem.places` (array of integers ≥ 0); descriptions. |
| `🧬️schema/🟦️.ts` | `CROWD_SCORE_BINS`, `CrowdScores`, `scores` on `CrowdTask` and `CrowdView`, `places` on `CrowdItem`. |
| `🧬️schema/🦀️.rs` | The same; `CrowdScores = [usize; 10]`, so a view with nine or eleven bins, a fraction or a negative count does not deserialize. |
| `🔨️modules/👁️views/🟦️.ts` | `scoreBin`, `presented`, `placeBin`, private `binned`; `crowdView` fills `scores` and `places`. |
| `🔨️modules/👁️views/🦀️.rs` | `score_bin`, `presented`, `place_bin`, private `binned`; `crowd_view` fills `scores` and `places`. |
| `🔨️modules/👁️views/🧪️tests/🔬️unit/🦀️.rs` | Existing crowd tests extended with places and scores; new tests for `score_bin` (edges 0.0949/0.095/0.1/0.8949/0.895/0.9/0.995/1, every thousandth, out-of-range), `place_bin` (n < 2, n = m, n < m, n > m, half-up, an exact rational bracket check for m ≤ 12 and n ≤ 16, broken positions), `presented`, scores per task and dimension of the first task result, orders of every length against two draws, and the ten-bin shape on the wire. |
| `🧪️tests/📊️crowd-view/🐍️.py` | The reference now takes the first task result, dimension and item (as the design says) and adds `percent`, `score_bin`, `binned`, `presented`, `place_bin` (exact `fractions.Fraction`, rounded half up), `scores` and `places`. Corroboration: `numpy.histogram` over the whole percents with edges 0, 10, … 90, 101 for every `scores`, `collections.Counter` over the integer formula for every `places`, sums against `runs` and `answers`. |
| `🧪️tests/📊️crowd-view/🥒️.feature` | Text and the `Then` step describe scores, places, the first-entry rule and the new vectors. |
| `🧫️fixtures/📊️crowd-view/🔣️.json` | Regenerated (see below): 8 vectors, 30 results. |
| `🧪️tests/🗳️crowd-answers/🟦️.ts` | Expected views carry `scores` and `places`; new tests: ajv refuses nine, eleven, fractional and negative bins and a view or task without `scores`; scores per run, task and dimension; places for orders of every length against `draw` 3 and an over-drawn task; `scoreBin` edges and every thousandth against a `jStat.histogram`; `placeBin` for n < 2, n = m, n < m, n > m against an exact `mathjs` fraction; `presented`. |
| `README.md` | New bullet "Crowd distributions" under "What the others think". |

Not changed because nothing in them depends on the shape: `🧪️tests/🗃️shared-vectors/🟦️.ts` (compares `crowdView` with the committed vectors generically, and passes), `🧪️tests/👁️read-views/🟦️.ts` (no crowd view in it), `🧪️tests/🧬️schema-conformance/*` (its table row `crowd-views` already validates every committed expected view against `CrowdView` with `jsonschema`; its own fixture holds quizzes and catalogs only), the package glue and façade files.

Ticket folder:

| File | Change |
| --- | --- |
| `generate_quiz_vectors.py` | Region `🔖️Crowd`: `SCORE_EDGES`, `edge_result`, `ordered`, three new vectors. |

Proctor `🎓️teaching/🛂️proctor/`:

| File | Change |
| --- | --- |
| `🔨️modules/👪️crowd/🦀️.rs` | `CrowdTally { runs, scores, classification, sorting, matching }` with `Tallied<T> { scores, items }` per task (per dimension for matching); `Positions { answers, sum, orders }` where `orders` is order length → position → count. `view` bins the orders with `place_bin` against `presented(task)` of the current definition. |
| `🔨️modules/👪️crowd/🧪️tests/🔬️unit/🦀️.rs` | Random results carry scores on and beside the bin edges or any thousandth, and orders of 1 to 6 items; after each of 300 folds (through a serde round trip of the tally) the view equals `quiz::crowd_view` by value and by bytes, for the quiz as defined and for the same quiz drawing 2 and 3 items. New deterministic test: the stored orders and the places for `draw` 2, 3 and none. |
| `🔨️modules/🔭️projections/🦀️.rs` | `PROJECTOR_REVISION` is 5 (see "Deviations"). |
| `🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs` | The crowd projection test also checks the score bins and the places. |
| `🧪️tests/🌐️end-to-end/🦀️.rs` | The crowd over HTTP: ten bins summing to 2 on the view and on every task, the perfect run in the last bin, mirrored places for a perfect and a reversed order. |
| `README.md` | Boot step 3 names revision 5; `quiz.crowd` describes scores, places and the tally. |

`🔨️modules/❓️queries`, `🔨️modules/🧩️instance` and `🧪️tests/🏋️capacity/🟦️.ts` pass crowd views through as bytes and needed no change.

## Vectors

`🧫️fixtures/📊️crowd-view/🔣️.json` now has the five earlier vectors plus:

- `scores-on-bin-edges` — ten hand-built energy results whose run, task and dimension scores are 0, 0.0949, 0.095, 0.1, 0.5, 0.8949, 0.895, 0.9, 0.995 and 1, spread differently per task; every fourth leaves the CO₂ dimension out, none plays the room temperatures.
- `orders-shorter-and-longer` — orders the lifecycle reference scored, of 8, 8, 7, 6, 5, 3, 2 and 1 power ratings (a sheet presents 6 of 8, `draw`) and of 2 and 3 room temperatures (3, no `draw`); they include positions whose share is an exact half.
- `first-entries-count` — one result that repeats a task, a dimension and items and carries a task result of another kind, next to a real one.

The generator was not run as a whole. A dry run of every region showed that it would also rewrite `🧫️fixtures/🏅️badge-rules/🔣️.json`: the committed results there list the tasks in the sheet order from before the "first task stays first" change, which another session made in the sheet reference. That is unrelated to the crowd, so only the crowd region's output was written; every other fixture is byte-identical to before (md5 over all 24 fixtures). After the write, a second dry run reports the crowd fixture as unchanged.

The existing crowd vectors changed too, beyond the new members: their results come from real sheets, so their task order follows the same sheet change.

## Gates (as run on 2026-10-02, after the last edit)

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/quiz:test` | 10 files, 297 tests passed |
| `bun nx run @semio-tech/quiz-rs:test` | 107 passed, 0 failed (2 filtered out by level) |
| `bun nx run @teaching/proctor:test` | unit 87 passed, conformance 15 passed, end-to-end 17 passed |
| `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` (in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | `cases=13 executed=108 passed=108 failed=0 errored=0 parity=108/108` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=true errors=0 warnings=0` |
| `tsc --noEmit` over the core entry and `🧪️tests/*/🟦️.ts` (temporary tsconfig, since the core has no typecheck target) | no error in any file of this change; 2 errors in `🧪️tests/🔁️run-lifecycle/🟦️.ts` (see below) |
| `tsc --noEmit -p tsconfig.json` in the react target (read only) | no errors at the time of the run |

Runs that did not pass on the way, and why:

- The first `quiz:test` and `quiz-rs:test` runs were killed by the 15 s budget and two more `quiz-rs:test` runs failed inside nx ("Plugin worker … exited unexpectedly") while the machine was saturated by other sessions. A direct vitest run in that phase took 57 s and timed out two Unicode-table tests of `🔁️run-lifecycle` at 5 s. The reruns listed above are green.
- The first parity run was 106/108: the Python oracle of `🧬️schema-conformance::fixture-documents` raised on the row `leaderboard-rankings` (the leaderboard fixture of the period work did not match its pointer at that moment). The crowd rows were fine (`crowd-quizzes` 2, `crowd-results` 30, `crowd-views` 8 documents). The other session fixed it meanwhile; the rerun is 108/108.

## Deviations from §20 and open points

1. **`PROJECTOR_REVISION` is 5, not 4.** While this was implemented, the leaderboard-period session raised it from 3 to 4 ("keeps a transcript per ranked learner"). A store built by that revision 4 holds tallies in the old shape, which the new `CrowdTally` does not decode, and an undecodable stored state stops the boot. So the distributions got their own revision. §20 still says 4; the design file was left alone.
2. **`placeBin` clamps and `scoreBin` has a floor**, as described under "Semantics". For every valid input they equal the formulas of §20; the Python reference implements the unclamped text.
3. **`CrowdScores` is a named schema definition and type** rather than an inline array, so the ten-bin rule is stated once.
4. **`🧫️fixtures/🏅️badge-rules/🔣️.json` is stale against the generator** (task order inside its results). The badge oracle recomputes from the committed results, so nothing fails; whoever owns the sheet change should regenerate it.
5. **`🧪️tests/🔁️run-lifecycle/🟦️.ts` has two type errors** (lines 186 and 239: an object with `task` passed where a `Command` is expected). It is not in any tsconfig, so no gate sees it; it is not part of this change.
6. The docstring of `valueKey` in `🔨️modules/👁️views/🟦️.ts` names its Rust twin `value_key`; the Rust function is `json_number_text`. Not touched.

Nothing of the assigned scope is unfinished.
