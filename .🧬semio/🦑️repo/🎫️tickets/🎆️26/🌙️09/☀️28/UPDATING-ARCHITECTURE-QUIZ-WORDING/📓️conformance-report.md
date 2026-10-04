# 🧪️ Quiz conformance: shared vectors, oracles and Protocol v2 cases

Work package: shared vectors, third-party oracles and Protocol v2 cases for `🧰️framework/🛍️products/❓️quiz`.
The TypeScript twin (`@semio-tech/quiz`) and the Rust twin (`semio-framework-quiz`) are the subjects.

## Result

All ten cases pass. Every subject agrees with the Python oracle, and the two subjects agree with each other.

```
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"
[test] level=exhaustive cases=10 executed=75 passed=75 failed=0 errored=0 parity=75/75
```

The final run has 25 scenarios. Each scenario gives three results (Python oracle, TypeScript subject, Rust subject) and
three comparisons: oracle against TypeScript, oracle against Rust, and TypeScript against Rust. At the 20-scenario stage
I compared the projections directly:

- The TypeScript and Rust projection hashes were identical for every scenario.
- The projection lengths also matched the oracle's, down to the float digits. So no score differed by even one ulp,
  although the tolerance allows 1e-12.

The following commands were run, not assumed:

- The oracle phase passes on its own: `oracle quick` gave 20/20 at the time.
- The oracle-vs-TypeScript parity passed before the Rust adapters existed: `parity quick` gave 20/20.
- After the Rust adapters landed, `parity quick` and `parity exhaustive` were each rerun, both at 60/60.
- After the coordinator's follow-ups, the full run passed at 66/66 and then at 75/75. The follow-ups were the
  `LeaderboardRow.tag` contract change, catalog views, Spearman rank weights and the §13 degradation vectors.
- One earlier full run had one transient failure. The schema-conformance Python oracle host "exited 5 without emitting
  results". It did not come back in any later run. On Windows, exit code 5 is ERROR_ACCESS_DENIED. The likely cause is
  another agent's harness using the shared cache Python environment at the same moment.

There are no remaining disagreements.

## Coordinator follow-ups (done)

1. **`LeaderboardRow.learner` became `tag` (design §13).**
   - The reference emits `tag = "%08x" % fnv1a32(learner id)` and still orders by the internal learner id.
   - The leaderboard vectors were regenerated. Dan's tag `0d07d623` exercises the zero padding.
   - Cyd (`2f1d066e`) ranks before Dan only because of the learner id. Ordering by tag would put Dan first, so a
     tag-ordering bug would show up.
   - Both twins already emitted `tag`, and parity is green.
2. **`⚖️partial-credit-scoring` and `👁️read-views` are now language-agnostic Protocol v2 cases.** These were TS-only
   vitest suites. They stay as the TS agent's unit tests, and their features now also have Protocol v2 cases with an
   oracle, TypeScript and Rust, folded into existing cases:
   - **Scoring**, from `⚖️partial-credit-scoring`:
     - It was already covered by 📏️sorting-concordance, 🔀️matching-concordance and 🕸️profile-similarity (scipy and
       numpy).
     - A new scenario, `📏️sorting-concordance/@id-rank-weights`, has 14 orders of equally spaced values. For each one,
       `scipy.stats.spearmanr` must imply the score as `(1+ρ)/2`. This is the language-agnostic form of the TS
       suite's jstat check.
   - **Views**, from `👁️read-views`:
     - `learnerView` and `leaderboard` were already covered: `🏆️leaderboard` (learner-views, rankings) and
       `🧾️learner-lifecycle` (learner-views, which includes `runView`).
     - A new scenario, `🏆️leaderboard/@id-catalog-views`, adds `catalogView` for the leaderboard catalog and for the
       lifecycle catalog. The lifecycle catalog's profiles, values and explanations must all be stripped.
3. **The §13 degradation rule** ("no NaN, no throw") now has vectors: a `@id-degraded` scenario (`@mode-error`) in each
   of the three scoring cases, with 18 vectors in total.
   - An invalid or incomplete answer, a wrong kind, or a foreign sheet task scores none.
   - An empty mean is 0: a classification sheet without items, or a matching task without dimensions.
   - An empty sorting sheet scores 1.
   - A profile missing an axis is excluded from the profiled set and from `d_max`. Its extreme partial values would
     change `d_max` if it were included, so an implementation that includes it fails the vector.
   - The lifecycle reference now refuses `run-incomplete` when `scoreRun` scores none.

## Files

### Created

| Path | Content |
|---|---|
| `🧰️framework/🛍️products/❓️quiz/🔮️oracles/🔣️.json` | The owner registry (schema v2). It validates against `🧪️test/🧬️schema/🔣️.json#/$defs/OracleRegistry` with python-jsonschema. |
| `…/❓️quiz/🧫️fixtures/{🎲️seeded-randomness,🃏️sheet-assembly,📏️sorting-concordance,🔀️matching-concordance,🕸️profile-similarity,✅️answer-validation,🏅️badge-rules,🏆️leaderboard,🧾️learner-lifecycle,🧬️schema-conformance}/🔣️.json` | The shared vectors: inputs and expected outputs. |
| `…/❓️quiz/🧪️tests/<same 10 names>/{🥒️.feature,🐍️.py,🟦️.ts,🦀️.rs}` | The Protocol v2 cases. `🐍️.py` is the oracle, `🟦️.ts` and `🦀️.rs` are the subjects. |
| Ticket: `generate_quiz_vectors.py` | Generates every vector from the case adapters' Python reference. It is reproducible: two runs give byte-identical output. |
| Ticket: `validate_quiz_fixtures.py` | Validates every schema-typed document of the fixtures with jsonschema. It reads the table from the conformance feature. |

The TypeScript agent's vitest suite `🧪️tests/🗃️shared-vectors` also reads these fixtures, so the vectors reach both cores.

### Modified (small anchored insertions in the Rust twin's files)

- `…/❓️quiz/📦️packages/🦀️rust/Cargo.toml` gains `[features] sut = ["dep:serde_json"]` and the optional dependency
  `serde_json = { workspace = true, optional = true }`.
- `…/❓️quiz/📦️packages/🦀️rust/🦀️.rs` (package glue) gains one feature-gated re-export: `#[cfg(feature = "sut")] pub use serde_json;`.

Why this was needed: the generated Rust host can only name the subject crate, and the crate had `serde_json` only as a
dev-dependency. The re-export follows the repo precedent (`🦑️repo/🔨️modules/*/📦️packages/🦀️rust/🦀️.rs`). The registry
enables it with `subjectFeatures: [{ implementation: "rust", features: ["sut"] }]`. Production builds of the crate stay
free of `serde_json`.

## Cases

| Case | Capability | Oracle | Comparison | Scenarios |
|---|---|---|---|---|
| 🎲️seeded-randomness | `quiz-randomness` | `quiz-numpy-mt19937` | `ordered-json-v1` | hashes, run-seeds, raw-outputs, uniform-draws, shuffles |
| 🃏️sheet-assembly | `quiz-sheet-assembly` | `quiz-python-reference` | `ordered-json-v1` | sheets |
| 📏️sorting-concordance | `quiz-sorting-score` | `quiz-scipy-stats` | `quiz-score-v1` | scores, rank-weights, degraded (`@mode-error`) |
| 🔀️matching-concordance | `quiz-matching-score` | `quiz-scipy-stats` | `quiz-score-v1` | scores, degraded (`@mode-error`) |
| 🕸️profile-similarity | `quiz-classification-credit` | `quiz-scipy-stats` | `quiz-score-v1` | credits, degraded (`@mode-error`) |
| ✅️answer-validation | `quiz-answer-validation` | `quiz-python-reference` | `ordered-json-v1` | verdicts |
| 🏅️badge-rules | `quiz-badge-rules` | `quiz-python-reference` | `ordered-json-v1` | awards |
| 🏆️leaderboard | `quiz-leaderboard` | `quiz-python-reference` | `quiz-score-v1` | catalog-views, learner-views, rankings |
| 🧾️learner-lifecycle | `quiz-learner-lifecycle` | `quiz-python-reference` | `quiz-score-v1` | handles, roster, learner-decisions, learner-views (quick) |
| 🧬️schema-conformance | `quiz-schema-conformance` | `quiz-jsonschema` | `ordered-json-v1` | fixture-documents, repository-quizzes, rejected-quizzes (`@mode-error`) |

All scenarios are `@level-fundamental` except the lifecycle's `learner-views`, which is `@level-quick`. All are
`@mode-differential` except `rejected-quizzes` and the three `degraded` scenarios, which are `@mode-error`.

- **Generation command.** Every feature states it in its prose.
- **Projection.** Only decisions and schema views are projected, never a core's private state.
- **`quiz-score-v1`.** This is an owner-contributed profile with tolerance 1e-12, as in design §6.

### What the vectors cover

- **Randomness:**
  - FNV-1a over empty, ASCII, umlaut and emoji texts, and over run ids.
  - Raw MT19937 words for seeds 0, 1, 42, 5489, 2³²−1 and the FNV offset basis.
  - Words 625 onward, which exercise the twist.
  - The C++ `[rand.predef]` word 10000, which is 4123659995.
  - Bounded draws, including `n = 1` (no draw), `2³¹+1` (about 50% rejection) and `2³²−1`.
  - Shuffles of length 0, 1, 2, 5, 10 and 52.
  - Every draw or shuffle projects the NEXT raw word, which pins how many words were consumed.
- **Sheets:** 12 sheets from three quizzes:
  - `energy-basics` has all three task kinds with draws: 3 axes, 4 profiled categories and 1 unprofiled, a logarithmic
    sort with a tie, a linear sort without a draw, and two-dimensional matching with duplicate card values.
  - `cooling-basics` has a profile-free classification task with a draw.
  - `rotation-demo` has seeds chosen for every rotation outcome, including a value-ascending tie that is not rotated.
- **Sorting (14 vectors):**
  - Logarithmic: perfect, reversed, tie swapped (still exactly 1), a neighbour swap, a small neighbour swap, the extreme
    swap, the untouched sheet order, and all eight items.
  - Linear: perfect, reversed, every single swap, and a rotation.
  - All values equal, so the total weight is 0 and the score is 1.
  - 14 rank vectors (Spearman): ascending, descending, and shuffled subsets of 2, 3, 5 and 8 equally spaced floors.
  - 6 degraded vectors.
- **Matching (9 vectors, plus 7 degraded):** perfect; equal cards exchanged (still 1); reversed indices; a neighbour
  swap; the extreme swap; a learner-made tie (half weight); a one-dimension task with equal cards; an underrated item.
- **Classification (9 vectors):**
  - Hits.
  - Near, mid and farthest profile misses; the farthest miss earns exactly 0.
  - An unprofiled category assigned, and an unprofiled category missed.
  - Everything wrong.
  - A profile-free task.
  - `d_max = 0` (identical profiles).
  - 5 degraded vectors, one of them with an incomplete profile that is excluded.
- **Answer validation (26 vectors):**
  - Valid, partial, empty and absent answers for each kind.
  - Unknown items, categories and dimensions.
  - Wrong kind.
  - Missing, duplicate or extra sorting items, and an empty order.
  - Card index out of range, a card reused within a dimension, and the same index across dimensions (valid).
- **Badges (10 vectors):**
  - Every rule, with held badges.
  - Perfection spread over several runs.
  - A selector that matches no task (never awarded).
  - The results are real, scored by the reference from real sheets.
- **Leaderboard (3 vectors, plus 2 catalog views):**
  - The full tie-break chain: equal total, badges and `reachedAt` split by id; a later `reachedAt`; fewer badges.
  - An improving learner whose later, worse run does not move `reachedAt`.
  - A score of 0, and a learner with no submission, who is excluded.
  - Every score is dyadic, so totals and ties are exact.
  - Rows carry `tag`, never the learner id.
  - The catalog views strip quiz paths, badge rules, categories, values and explanations.
- **Lifecycle:**
  - 13 handle vectors: NBSP, U+3000, tabs, umlauts, ẞ, emoji, empty and blank, 64 characters and 65 characters.
  - 4 roster sequences: anonymous learners always new; the shared pseudonym/name key space; handle-invalid; recall
    across case and umlauts.
  - 5 learner sequences with 62 steps. They reach every rejection in the §8 table, stale-revision voiding on start and
    on submit, `quiz-revised`, latest-answer-wins, and all 6 badges of the lifecycle catalog.
  - Final learner views and run views.
- **Schema conformance:**
  - 40 table rows that validate every typed document of the vectors: 493 documents over 16 `$defs`. The deliberately
    invalid `degraded` groups are left out.
  - The 5 teaching documents under `🎓️teaching/🏛️architecture` (1 catalog and 4 quizzes).
  - 17 rejected documents, each breaking one named rule, plus the 2 unbroken bases.

## Oracle registry (`🔮️oracles/🔣️.json`)

| Id | Kind | Package | Judges |
|---|---|---|---|
| `quiz-numpy-mt19937` | third-party-library | numpy 2.5.0 (python) | The MT19937 stream. The oracle also cross-checks it against CPython's own `random.Random` loaded with the same state. |
| `quiz-scipy-stats` | third-party-library | scipy 1.18.0 (+ numpy) | Sorting and matching concordance, and classification distances (see below). |
| `quiz-jsonschema` | third-party-library | jsonschema 4.26.0 (python) | Schema conformance, compared with `quizIssues`/`catalogIssues` and the strict Rust twins. |
| `quiz-python-reference` | cross-semio-implementation | numpy (python) | Sheet, answers, badges, leaderboard, lifecycle. |
| `quiz-jstat-spearman` | third-party-library | jstat 1.9.6 (js) | The TS unit suite `⚖️partial-credit-scoring`: with equally spaced values the score is `(1+ρ)/2`. Added at the coordinator's request. |
| `quiz-mathjs-distance` | third-party-library | mathjs 14.0.0 (js) | The TS unit suite: profile distances. Added at the coordinator's request. |
| `quiz-ajv-structure` | third-party-library | ajv 8.20.0 (js) | The TS unit suite `🩺️document-validation`: ajv-valid ⇔ no structural issue. Added at the coordinator's request. |
| `quiz-react-d3-scale` | third-party-library | d3-scale 4.0.2 (js) | The react unit suite `🕷️radar-geometry`: `scaleLinear().clamp(true)`. Added at the coordinator's request. |

- **scipy checks:** the pair loop is recomputed with numpy pair matrices; the unit-weight engine must equal
  `(1+τ_a)/2` from `kendalltau`'s `τ_b` and the tie counts; distances are recomputed with `distance.euclidean` and `pdist`.
- **`quiz-python-reference`:** badges, the leaderboard and the lifecycle are semio-native, so no third-party library can
  judge them. The rationale field records this survey.
- **Host packages and dependencies:** `oracleHostPackages` pins numpy, scipy and jsonschema for the Python host.
  `noOracleDecisions` is empty.
  - `bun ./📜️script.ts dependency` classifies every quiz oracle package as `test-oracle`; all eight ids are listed.

Why the semio-native capabilities use a Python cross-semio reference instead of a `@no-oracle` decision: under a no-oracle
decision the harness runs no oracle role, so Python would never execute. With a cross-semio reference oracle, Python runs
as the reference, and the harness still holds TypeScript and Rust to each other. That gives three independent readings of
the design. The kind is `cross-semio-implementation`, a supplement and not a qualifying third-party kind, so it claims
no third-party evidence. There are no mutation manifests.

## Fixture validation (python-jsonschema 4.26.0, Draft 7)

`.venv/Scripts/python.exe <ticket>/validate_quiz_fixtures.py` exits 0 with 0 violations:

| Fixture | Rows (definition: documents) |
|---|---|
| 🃏️sheet-assembly | Quiz 3, Sheet 12 |
| 📏️sorting-concordance | Task 3, SheetTask 14 + 14, Answer 14 + 14, TaskResult 14 + 14 (scores + rank weights) |
| 🔀️matching-concordance | Task 2, SheetTask 9, Answer 9, TaskResult 9 |
| 🕸️profile-similarity | Task 3, SheetTask 9, Answer 9, TaskResult 9 |
| ✅️answer-validation | SheetTask 3, Answer 23 |
| 🏅️badge-rules | Quiz 2, Badge 7, RunResult 17 |
| 🧾️learner-lifecycle | Catalog 1, Quiz 2, Command 11 + 62, Event 9 + 4 + 58, Rejection 2 + 15, LearnerView 4, RunView 8 |
| 🏆️leaderboard | Catalog 1, Quiz 2 + 4, CatalogView 2, Event 82, LearnerView 9, Leaderboard 3 (rows with `tag`) |
| 🧬️schema-conformance | 17 rejected documents, each failing at its named rule, and 2 accepted bases |
| 🎲️seeded-randomness | No schema-typed documents |

The Python reference also asserts that every committed expected output is reproduced, with tolerance 1e-12 on numbers.
The harness oracle phase also re-validates everything.

The `degraded` groups (18 vectors) deliberately bypass validation (§13), so they are not in the validation table.
Two runs of the generator produce byte-identical fixtures (checked with `sha256sum`).

## Ambiguities in the design, and how they were resolved (the design text was followed)

1. **§6, "with s = rank the score equals (1+τ_a)/2" is false for n ≥ 3.** Counterexample: the order of ranks
   `[1,0,2]` scores 0.75, while `(1+τ_a)/2 = 2/3`. With rank-difference weights the score is `(1+ρ_Spearman)/2`. The
   core-ts agent derived and tested this with jstat. `(1+τ_a)/2` holds for **unit** weights, and that is the reading the
   scipy oracle checks. **The §6 sentence should be corrected.**
2. **Sorting `rank`: the true ascending order of what?** This was read as the drawn sheet items, ties by definition
   index. Both twins agree.
3. **`lastActivity` is not defined anywhere.** It was read as the latest `at` of the learner's stream. Both twins agree.
4. **`reachedAt` for a quiz's first submission.** A first submission counts as raising, even when it scores 0 (Fay in the
   tie-break vector). Otherwise a single zero-score learner would have no `reachedAt`, which the schema requires. Both
   twins agree.
5. **`total` "over the catalog quizzes".**
   - The TypeScript and Rust twins sum over the `CatalogView`'s quizzes in catalog order.
   - The Python reference sums every best.
   - Every vector only submits catalog quizzes, so the two readings agree on every vector and differ by at most one ulp.
6. **Handle "whitespace" and "chars" are unspecified.**
   - The twins use Unicode `White_Space` and count code points.
   - The reference's `str.split()` additionally splits on U+001C–U+001F.
   - The vectors avoid U+001C–U+001F, U+0085, U+FEFF and U+200B, and do not test code-point versus UTF-16 counts at the
     64 limit.
7. **Unreachable badges in catalogs.**
   - §7 says a selector matching no task never awards, but `catalogIssues` reports `badge-unreachable`.
   - The phantom badge is kept only in the badge-rules vectors (as `Badge` documents), where `earnedBadges` must handle
     it. It is not in any `Catalog` document.
8. **Behaviour the design does not state (not covered by vectors):**
   - Idempotency by command id: the pure deciders do not dedupe, so every committed command id is distinct.
   - A start-run under an already used run id: the twins add a `run-open`/`run-closed` guard.
   - record-answer or submit-run by an unregistered learner: the reference answers `unknown-run`.
   - A quiz removed from the context: the reference treats it as a stale revision.
   - `runView` of a run on a stale revision: only current-revision runs are viewed.
9. **Sorting rotation compares to the stable ascending order.** A draw that is ascending by value but has tied items
   against definition order is not rotated. The vector `rotation-tie-reversed-kept` pins this literal reading.
10. **§13 "invalid answers score to none".** Read here to mean the following, and the degraded vectors show both twins
    agree:
    - An answer that §5 holds *invalid or incomplete* scores none.
    - A sheet task that does not present the task (a different id, or an unknown item, category or dimension) also
      scores none.
    - The empty-sorting-sheet score of 1 comes from the "no weight → 1" rule, not from a mean.
    - "Incomplete profile" means a profile that misses an axis of the task.

    One further reading has no vector: extra profile axes are ignored.

## Findings for other work packages

- **Contract, owned by core-rust.** `bun ./📜️script.ts contract --owner …/❓️quiz` reports 79 high-priority
  `inline-test-body` breaches for the `#[cfg(test)]` modules in `🔨️modules/*/🦀️.rs` and `🧬️schema/🦀️.rs`. The layout
  rule requires moving them into `🧪️tests/<case>/`. The new cases, fixtures and registry produce 0 breaches.
- **`bun ./📜️script.ts run`** (contract, then parity) exits 1 in its contract phase. There are 5123 high-priority
  breaches repo-wide, none of them in quiz files, so parity never starts inside `run`. `parity` is therefore the command
  to use for the quiz cases.
- **`dependency`** fails repo-wide for reasons that predate this work: production-debt entries and new Go modules.
  There is no quiz finding. numpy is still classified `test-runner` in the generated `🔒️dependencies.json`.
- **Python drift between the lockfile and `.venv`.**
  - `uv.lock` pins numpy 2.5.0 and scipy 1.18.0; the repo `.venv` still has 2.4.3 and 1.17.1.
  - The registry pins the lockfile versions. On its first run the harness provisioned a cache environment with pip, which
    needs the network.
  - The vectors were generated under 2.4.3/1.17.1 and reproduce identically under 2.5.0/1.18.0.
- **Build output in the source tree.** `❓️quiz/📦️packages/🦀️rust/dist/build/` holds Rust build output. It does not come
  from this work package.
- **Taxonomy.** All 10 case and 10 fixture directory names were already registered by site-infra, in
  `members-of-tests` and `members-of-fixtures`.
- **Type check.** The TypeScript adapters type-check cleanly (`tsc --strict`, no quiz diagnostics). The Rust hosts build
  without warnings.
- **The two TS-only vitest suites remain.** `⚖️partial-credit-scoring` and `👁️read-views` (owned by core-ts) still exist
  beside the Protocol v2 cases that now cover their features. The harness README says to "never leave two test
  hierarchies alive". Deleting or trimming those suites is a decision for core-ts and the coordinator. I did not touch
  them.

## 2026-09-29 — required quiz emoji (design §14)

The schema change is in place in the vectors, and every check passes again.

- **The change.** `Quiz.emoji` and `CatalogQuizView.emoji` are now required (1–16 characters).
- **Generator** (`generate_quiz_vectors.py`):
  - Every vector quiz carries an emoji: `energy-basics` ⚡, `cooling-basics` ❄️, `rotation-demo` 🔄, `warm-up` 🔥,
    `cool-down` 🧊, and the rejected-vector base quiz 🐜.
  - Three new rejected vectors: `missing-emoji` (required), `empty-emoji` (minLength) and `overlong-emoji` (17
    characters, maxLength). The conformance fixture now holds 20 rejected documents and 2 accepted bases.
- **Python reference.** `🏆️leaderboard/🐍️.py` `catalog_view` carries each quiz's emoji. The leaderboard feature prose
  says so.
- **Vectors.** All shared vectors under `❓️quiz/🧫️fixtures/` were regenerated. Two generator runs produce
  byte-identical output.
- **jsonschema checks** (Draft 7, the current normative schema):
  - `validate_quiz_fixtures.py` exits 0 with 0 violations, including the emoji on every `Quiz` and `CatalogView`
    document.
  - The four real quizzes (`⚡️energy/{❄️cooling,📊️demand,🔥️heating,🧲️physics}/❓️quiz/🔣️.json`, emojis ❄️ 📊 🔥 🧲)
    and the architecture catalog all validate with no violations.
- **Harness, run from `🧪️test`:**
  - `oracle exhaustive` gives 25/25.
  - `RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` gives
    `executed=75 passed=75 … parity=75/75`.
  - Both twins already emit the emoji in `catalogView`, and they reject all three new emoji vectors, the same way the
    oracle does.
- **Unit suites of the twins,** still green against the regenerated vectors: `@semio-tech/quiz` passes 166 of 166
  tests, and `semio-framework-quiz` passes 70 of 70.

## 2026-09-29 — shared presence and cursors (design §15)

The new case `🧪️tests/👥️shared-presence` is green in all three languages.

```
RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"
[test] level=exhaustive cases=11 executed=84 passed=84 failed=0 errored=0 parity=84/84
```

This is 11 cases and 28 scenarios, each run three ways: oracle against TypeScript, oracle against Rust, and TypeScript
against Rust.

### The case

- **Tags:** `@capability-quiz-presence`, `@oracle-quiz-jsonschema` and `@comparison-ordered-json-v1`.
- **Scenarios:** `room-scopes`, `presence-states` and `cursor-states`.
- **Files:** `🥒️.feature`, `🐍️.py` (oracle), `🟦️.ts` and `🦀️.rs` (subjects).
- **The oracle** has two halves:
  - python-jsonschema judges the structure against the normative `PresenceState`/`CursorState`.
  - The place rules the schema cannot state are written from the design text:
    - `run-without-quiz`: a run or its results names its quiz.
    - `quiz-outside-run`: only a run or its results names a quiz.
    - `task-without-run`: only a run names the task on screen.
  - The room scopes follow the §15 list.
  - Every refused vector names the rule that must refuse it, and the oracle checks that this rule fires.
- **What is projected:**
  - The scopes: `{roster, room}`, with `null` where a place has no room.
  - Accepted or refused per state. Each core words its problems in its own vocabulary, so the wording is not compared.
- **Subjects:**
  - `rosterScope`, `roomScope`, `presenceProblem` and `cursorProblem` of `@semio-tech/quiz`.
  - Their Rust twins. For Rust, a state is accepted when it decodes into the strict typed twin and
    `presence_problem`/`cursor_problem` reports nothing.

### The vectors (`🧫️fixtures/👥️shared-presence/🔣️.json`)

They are generated by `generate_quiz_vectors.py` (`presence()`). Two runs produce byte-identical output.

- **10 scopes.** Every screen, including the run task. A second catalog. The identity screen and a run without its
  quiz both have no room.
- **27 presence states: 8 accepted, 19 refused.**
  - Bad tags: uppercase, 7 digits, a 32-hex learner id.
  - Undeclared members smuggling an id or an answer.
  - `active` missing or not a boolean.
  - An unknown or missing screen, and a quiz that is not a slug.
  - Bad handles: missing, empty, 65 characters.
  - The three place rules, two vectors each: `run-without-quiz` (run, results), `task-without-run` (results, home) and
    `quiz-outside-run` (home, leaderboard).
- **26 cursor states: 10 accepted, 16 refused.**
  - Accepted boundaries: 0/0, 1/1, 0.0/1.0, −0.0, a 64-character anchor, `task:`/`card:quiz:` anchors, focus only,
    tag only.
  - Refused: 1.0000001, −0.001, 1920, `"0.5"`, `null`, a missing y, and bad anchors (empty, uppercase, trailing or
    double separator, 65 characters).
  - Also refused: focus containing a space, a non-hex tag, a missing tag, and a smuggled drag target or item.
  - JSON cannot carry NaN or an infinity (and serde_json refuses `1e999`), so the "not a finite number" rule is covered
    by a text and a null. Every projection is NaN-free.
- **Checks:**
  - `validate_quiz_fixtures.py` exits 0 with 0 violations.
  - The new table row `presence-places` (`/scopes/*/place` → `Place`) validates all 10 places.
  - The states are not in that table on purpose, because most of them are meant to be refused.

### Registry and taxonomy

- **Registry:** `quiz-jsonschema` now also declares `quiz-presence`, and its rationale explains the presence split.
- **Taxonomy:** `👥️shared-presence` joins `members-of-tests` and `members-of-fixtures` in `🔣️taxonomy.json`. This was
  two anchored insertions. Afterwards the site-infra registration dry run reports no remaining additions.

### Where the design and the twins moved, and what the vectors pin

- **The first Rust twin I ran disagreed on 5 vectors:**
  - It gave the identity screen the room `<catalog>/identity`.
  - It accepted a task on home or results, and a quiz on home or leaderboard.
- **Both twins now agree with the design text:**
  - The Rust twin added `quiz-outside-run` and `task-without-run`.
  - Both give the identity screen no room, with the reason that "a learner there has no tag yet, so it is shared with
    nobody".
  - While they were changing, I briefly gave identity a room, reading the schema's "learners at the same place share
    one room". I reverted that to follow the explicit §15 room list and the twins. The vectors now pin no room for
    identity.
- **Ambiguity to record in the design:**
  - The schema's `Place` description says "Learners at the same place share one presence room".
  - The §15 room list leaves out the identity screen.
  - The Place description should say that the identity screen has no room.

### Other checks, all run

- The oracle phase for the new case passes 3/3.
- The twins' unit suites, which also read these vectors, pass: `@semio-tech/quiz` 217/217 tests and
  `semio-framework-quiz` 76/76.
- `contract --owner …/❓️quiz` reports no quiz breaches. The Rust twin's inline tests have moved to `🧪️tests/🔬️unit`.
- The new adapters type-check cleanly. No `__pycache__` was left.

## 2026-09-29 — presence for the §16 screens

The presence reference and vectors now cover every screen, and the full parity run is back to green.

```
RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"
[test] level=exhaustive cases=11 executed=84 passed=84 failed=0 errored=0 parity=84/84
```

- **The contract change.** `Screen` gains `quiz`, `learner`, `badges` and `preferences`.
- **Rooms:**
  - `introduction`, `home`, `leaderboard` and `badges` each get `<catalog>/<screen>`.
  - The quiz page, the run and the results of one quiz share `<catalog>/quiz/<quiz>`.
  - `identity`, `learner` and `preferences` have no room, and neither has a quiz page, run or results place without
    its quiz.
- **Place rules**, renamed because the quiz page joined the quiz screens:
  - `quiz-required`: the quiz page, a run or its results names its quiz.
  - `quiz-not-allowed`: no other screen names a quiz.
  - `task-without-run`: only a run names a task.
- **Python reference.** `👥️shared-presence/🐍️.py` now has `ROOMS = (introduction, home, leaderboard, badges)` and
  `QUIZ_SCREENS = (quiz, run, results)`, with the renamed rules. The feature prose follows.
- **Vectors:**
  - 15 scopes, covering every screen, including the quiz page with and without its quiz.
  - 37 presence states: 12 accepted, 25 refused. The new ones are:
    - Accepted: the quiz page, badges, learner (inactive) and preferences (anonymous).
    - `quiz-required`: a quiz page without its quiz.
    - `task-without-run`: a task on the quiz page or on preferences.
    - `quiz-not-allowed`: a quiz on badges, learner or identity.
  - The 26 cursor states are unchanged.
  - Two generator runs produce byte-identical output.
  - `validate_quiz_fixtures.py` exits 0 with 0 violations. `presence-places` validates all 15 places against the new
    `Screen` enum.
- **Twins.** Both had already landed the §16 rules, so their outputs matched the reference on the first run:
  - The presence case passes 9/9 (oracle against TypeScript, oracle against Rust, TypeScript against Rust).
  - Their unit suites pass against the regenerated vectors: `@semio-tech/quiz` 222/222 and `semio-framework-quiz`
    76/76.

## 2026-09-29 (night) — crowd view, thinking, item/category anchors and drags (design §17)

All three parts now have language-agnostic cases, and the full run is green.

```
RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"
[test] level=exhaustive cases=12 executed=93 passed=93 failed=0 errored=0 parity=93/93
```

That is 12 cases and 31 scenarios, each compared three ways (oracle against TypeScript, oracle against Rust,
TypeScript against Rust).

### New case `🧪️tests/📊️crowd-view`

- **Tags:** `@capability-quiz-crowd`, `@oracle-quiz-python-reference` (whose capabilities now include `quiz-crowd`) and
  `@comparison-quiz-score-v1`.
- **Scenario:** `crowds`, with subjects `crowdView` and `crowd_view`.
- **The reference (`🐍️.py`)**, written from §17 and the schema:
  - Only results of the viewed quiz count.
  - Tasks, matching dimensions and items follow definition order, and unanswered items are left out.
  - Classification and matching give `{key, count}`; sorting gives the mean of `position / (n − 1)`.
  - Every count is recounted with `collections.Counter`, and every mean recomputed with `numpy.mean`, before
    projecting.
- **The vectors (`🧫️fixtures/📊️crowd-view/🔣️.json`):** 5 vectors of real run results, scored by the lifecycle
  reference:
  - Three energy runs: perfect, worst and mixed.
  - One run, so drawn-out items stay unanswered.
  - A foreign cooling result, which is ignored.
  - No runs, so every task is present with `items: []`.
  - A hand-built cooling result with a one-item order, plus a real one.
  - Across them: all three kinds; heating oil and diesel equal in both dimensions, and wood pellets and firewood sharing
    a CO₂ factor.
- **Pinned readings:**
  - Count keys are ordered by code point, so `"120"` precedes `"15"`. The design now states this for every
    implementation: "Crowd count keys sort by code point… clients sort values numerically for display".
  - A value key is written as JavaScript prints a JSON number.
  - A one-item order sits at 0.
- **Parity history.** TypeScript first ordered value keys numerically: 8 differences, while Rust matched the oracle.
  After the TypeScript twin switched to code-point order, the case passes 3/3.

### `👥️shared-presence` extended

- **Cursor states (35).** New accepted: `item:<id>` and `category:<id>` anchors, a drag onto a category, drag only. New
  refused:
  - An uppercase item anchor.
  - A `drag.item` that is not a slug.
  - `drag: {}`.
  - An extra member in `drag`.
  - A drag given as text.
- **Thinking scopes (3).** `thinkingScope(catalog, quiz)` = `<catalog>/quiz/<quiz>/thinking`.
- **Thinking states (31: 11 accepted, 20 refused), after the `ThinkingAnswer` contract change:**
  - Classification and sorting drafts are ordinary answers.
  - A matching draft is `ThinkingMatchingAnswer { values: dimension → item → number }`, including a negative value and
    an empty dimension.
  - The limit is at its boundary on both sides: 64 or 65 tasks, order entries, assignments, dimension entries and
    dimensions (`too-many`).
  - Refused besides the limit:
    - A repeated sorting item (`duplicate-id`).
    - Card indices instead of values.
    - Non-numbers in place of non-finite values (`null`, `"120"`, `"Infinity"`).
    - Dimension or task ids that are not slugs.
    - A bad tag, missing or non-object `answers`, an unknown answer kind, a smuggled learner id, and a 120-task
      state.
- **Change in the reference.** It first refused thinking states above 2 KiB, which is the §15 frame limit. It now
  follows the bounds both cores share, `THINKING_LIMIT` = 64 plus the duplicate rule, as the coordinator's "bounds"
  asks. **The design text names neither the number nor the duplicate rule, and should record both.**

### Checks, all run

- **Fixture validation.** `validate_quiz_fixtures.py` exits 0 with 0 violations. New rows `crowd-quizzes`,
  `crowd-results` and `crowd-views` (5 `CrowdView` documents); `presence-places` still covers 15 places.
- **Generator.** It is reproducible. It now skips a fixture whose bytes are unchanged, and retries a write refused
  with `Errno 22`: another agent's process was intermittently holding `👥️shared-presence/🔣️.json` open.
- **Unit suites of the twins** (they read these vectors): `@semio-tech/quiz` 260/260 and `semio-framework-quiz` 83/83.
- **Other checks:**
  - `contract --owner …/❓️quiz` reports no quiz breaches.
  - The new adapters type-check with no quiz diagnostics, and docstring emojis are unique.
  - No `__pycache__` was left.
  - `📊️crowd-view` joins `members-of-tests` and `members-of-fixtures` in `🔣️taxonomy.json` (anchored insertions).

### For the coordinator

- **Duplicate path emoji.** `🧪️tests/👥️presence-client` (from another work package) repeats the path emoji of the
  earlier `🧪️tests/👥️shared-presence`. The path-emoji statute requires unique sibling emojis; one of them should be
  renamed.
- **Unregistered names.** The taxonomy registration dry run still lists names from other work packages:
  `🔨️modules/🗳️crowd` and `🧪️tests/🗳️crowd-answers`.
