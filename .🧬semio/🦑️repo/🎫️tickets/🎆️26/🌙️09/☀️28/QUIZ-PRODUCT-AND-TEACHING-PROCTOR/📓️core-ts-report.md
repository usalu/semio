# 📓️ Core TS report — `@semio-tech/quiz`

Work package: the TypeScript core of `framework.product.quiz` (design §1–§8, §11). Status: **done, all green**.

## Files

Created (all under `🧰️framework/🛍️products/❓️quiz/`):

| File | Content |
|---|---|
| `🧬️schema/🟦️.ts` | One `readonly` TS type per `$defs` entry (55 exports) plus named variants (`ClassificationTask…`, `IdentifyLearnerCommand…`, `RunStartedEvent…`, `CatalogTaskView`, `CatalogBadgeView`) and `LANGUAGES`, `SCALES`, `TASK_KINDS`, `REJECTIONS`, `RUN_STATUSES`. |
| `🔨️modules/🎲️randomness/🟦️.ts` | `fnv1a32`, `runSeed`, `RandomSource`, `Mt19937` (`next()`), `uniformIndex`, `shuffle`. |
| `🔨️modules/🃏️sheet/🟦️.ts` | `sheetOf(quiz, seed)`, `ascendingItems`. |
| `🔨️modules/✅️validation/🟦️.ts` | `quizIssues`, `catalogIssues`, `answerRejection`, `answerComplete`, `ValidationIssue`, `compareCodePoints`. |
| `🔨️modules/📏️scoring/🟦️.ts` | `scoreTask`, `scoreRun`, `scaled`. |
| `🔨️modules/🏅️badges/🟦️.ts` | `earnedBadges(badges, quizzes, results, held)`. |
| `🔨️modules/🧾️lifecycle/🟦️.ts` | `normalizeHandle`, `RosterState`, `emptyRosterState`, `decideRoster`, `evolveRoster`, `LearnerState`, `RunState`, `LoadedQuiz`, `LearnerContext`, `Decision`, `emptyLearnerState`, `decideLearner`, `evolveLearner`. |
| `🔨️modules/👁️views/🟦️.ts` | `catalogView`, `learnerView`, `runView`, `leaderboard`. |
| `📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🟦️.ts}` | `@semio-tech/quiz` glue (library, `semio.role: framework`, `semio.id: quiz`), nx `test`/`test-quick`/`test-long`/`test-exhaustive` → `bun ./📜️script.ts test [level]`. devDependencies: `ajv 8.20.0`, `jstat 1.9.6`, `mathjs 14.0.0` (test oracles, versions already in the lockfile), `typescript`, `vitest`. |
| `🟦️.ts` | Product barrel. |
| `README.md` | Domain model (catalog, quiz, task kinds, sheet, answer, scoring with formulas and the rationale for magnitude weights, badges, run lifecycle, identity, leaderboard, state classes), layout, commands, validation code table. |
| `🧪️tests/🎚️config/🟦️.ts` | Vitest (node) with an explicit include list — the Protocol v2 adapters beside the `🥒️.feature` files are not vitest suites. |
| `🧪️tests/🌀️mt19937-generator/🟦️.ts` | FNV/MT19937/uniform/shuffle units and vectors. |
| `🧪️tests/🎴️sheet-randomization/🟦️.ts` | Sheet determinism, normative RNG consumption order, invariants over 500 seeds, rotation rule, ties. |
| `🧪️tests/🩺️document-validation/🟦️.ts` | 40+ structural and semantic cases, each differentially checked against ajv; answer validity/completeness. |
| `🧪️tests/⚖️partial-credit-scoring/🟦️.ts` | Sorting/matching/classification scoring, properties and oracles. |
| `🧪️tests/🎖️badge-awards/🟦️.ts` | All three rules, selectors, held badges, catalog order. |
| `🧪️tests/🔁️run-lifecycle/🟦️.ts` | Handles, roster, every learner decision row, revisions, badges, evolve. |
| `🧪️tests/👁️read-views/🟦️.ts` | Catalog/learner/run views, leaderboard ordering at every tie level, `reachedAt`. |
| `🧪️tests/🗃️shared-vectors/🟦️.ts` | Consumes every `🧫️fixtures/<case>/🔣️.json` (all 10 cases) with the 1e-12 parity tolerance. |

Modified (small anchored insertions): `🧰️framework/🛍️products/🔣️.json` (member `framework.product.quiz` after presentation), root `package.json` (`workspaces` entry `🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript` before the presentation entry).

Ticket inputs kept: `mt19937-numpy-vectors.py` (third-party MT19937 vectors), `quiz-schema-scope-probe.ts` (schema scope probe), `🗑️generated/tsconfig.core-ts.json`, `🗑️generated/tsconfig.adapters.json`.

## Verification (all run, not assumed)

- `bun ./📜️script.ts test` (package dir): **8 files, 154 tests passed**; `test exhaustive` (with v8 coverage): 154 passed.
- `bun nx run @semio-tech/quiz:test --skip-nx-cache`: passed.
- `bunx tsc --noEmit -p 🗑️generated/tsconfig.core-ts.json` (root config + `noUncheckedIndexedAccess`, `noUnusedLocals`, `noUnusedParameters`) over all 19 files: **exit 0**.
- The conformance agent's 8 TS adapters type-check against this API (only unrelated `Bun` global typing in the test library appears under `types: ["node"]`).
- Schema parity: the repo inventory (`inventorySchemaScopes`, probe above) sees scope `framework.product.quiz` at level `product-root` with formats `🔣️jsonschema`, `🟦️typescript`, `🦀️rust`, 55 exports and **0 findings**; `bun ./📜️script.ts schema check` reports no quiz finding (its 9328 findings are pre-existing elsewhere; `schema-catalog-stale` needs a `schema generate` by whoever owns the catalog).
- `bun install` (repo root): the first two attempts failed with `Workspace dependency "@semio-tech/quiz-react" not found` (the site workspace depended on the react package before its workspace entry existed); after the react agent registered `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript`, **`bun install` exits 0** ("no changes") and links `node_modules/@semio-tech/quiz` → `📦️packages/🟦️typescript`.

## Third-party validation

| Subject | Oracle |
|---|---|
| MT19937 | numpy 2.4.3 `MT19937._legacy_seeding` (= `init_genrand`) + `random_raw`: first 5 and 1000th outputs for seeds 0, 1, 42, 5489, 2166136261, 2³²−1 (recorded by `mt19937-numpy-vectors.py`); C++ `[rand.predef]` (seed 5489 → 10000th = 4123659995). |
| FNV-1a | Reference vectors `""` → 2166136261, `"a"` → 3826002220, `"foobar"` → 0xbf9cf968. |
| Sorting score | **jStat 1.9.6 `spearmancoeff`**: with equally spaced ranks on a linear scale the score equals `(1 + ρ)/2` within 1e-12 over 300 random permutations (n = 2…15). |
| Classification credit | **mathjs 14.0.0 `distance`** for `d` and `d_max`. |
| Structural validation | **ajv 8.20.0** (draft-07, `strict: false`) on `🧬️schema/🔣️.json`: for every tested quiz/catalog document ajv-valid ⇔ no structural issue. |
| Everything | The committed Python-reference vectors in `🧫️fixtures/` (sheets, scores, badges, lifecycle decisions, views, leaderboard) all reproduce. |

## Decisions

1. **Design §6 Kendall claim is wrong as written.** "With `s = rank` the score equals `(1 + τ_a)/2`" does not hold for n ≥ 3: with `s = rank` the weights are `|rᵢ − rⱼ|`, and since `T − 2D = Σ_k r'_k(2k − n + 1) = T − Σdᵢ²` one gets `D = Σd²/2`, `T = n(n²−1)/6`, so **score = `(1 + ρ_Spearman)/2`**. Kendall's `(1 + τ_a)/2` holds for **unit weights** (which is what the conformance agent's scipy oracle correctly uses). Counterexample: ranks `[1,0,2]` score 0.75 while `(1+τ_a)/2 = 2/3`; a unit test pins this. The formula itself is implemented exactly as specified. Recommend correcting the §6 sentence.
2. Validation issues are deduplicated and sorted by `path`, then `code`, in Unicode code point order (= UTF-8 byte order, reproducible in Rust); the full code vocabulary is in the README table (`type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`, `integer-invalid`, `below-minimum`, `items-too-few`, `properties-too-few`, `duplicate-path`, `duplicate-id`, `category-unknown`, `axes-missing`, `profile-incomplete`, `axis-unknown`, `axis-range-invalid`, `profile-out-of-range`, `value-missing`, `dimension-unknown`, `value-not-positive`, `draw-exceeds-items`, `quiz-count-mismatch`, `quiz-unknown`, `badge-unreachable`). The Rust twin already uses the same vocabulary. Lengths count code points; a missing member is reported at its would-be pointer; the first occurrence of a duplicate id wins.
3. `catalogIssues(catalog, quizzes)` takes the loaded quizzes in catalog order (`readonly Quiz[]`); it does not repeat `quizIssues` of each quiz. A `perfect-tasks` selector matching no task is reported as `badge-unreachable` (it never awards, as §7 specifies); the shared-vector test derives the expected findings from the fixture itself, so it holds across fixture regenerations (the conformance agent regenerated the fixtures during this work — the phantom badge is gone, all 154 tests re-run green against the new vectors).
4. `answerRejection` / `answerComplete` live in `✅️validation`; `answerRejection` also rejects malformed shapes (non-object assignments, non-integer cards).
5. Handles: whitespace is the Unicode `White_Space` property (`/\p{White_Space}+/u`, identical to Rust `char::is_whitespace`; U+FEFF and U+200B are not whitespace); lowercase via `toLowerCase` (full default case mapping incl. final sigma, `ẞ → ß`); no NFC in the core — the client normalises before sending (§8).
6. `start-run` with a run id the learner already has is rejected `run-open` / `run-closed` (guard beyond the §8 table, protects the stream); `record-answer`/`submit-run` follow the table literally (no learner check).
7. `learnerView(state, catalog)` and `leaderboard(states, catalog)` take the **`CatalogView`** (the catalog document has only paths, the view has quiz ids). The total sums `best × 100` over the catalog quizzes in catalog order (schema text); the Python reference sums all bests in first-submission order — identical on every committed vector, different only for results of quizzes removed from the catalog. `reachedAt` follows submission order (`submittedAt`, ties by start order); a raise is strictly greater.
8. Views emit id-keyed maps (`best`, `answers`) with code-point-sorted keys, like the Rust `BTreeMap`s. `runView` rebuilds the sheet from the loaded quiz and the run seed and is `undefined` for an unknown run or unloaded quiz; `learnerView` is `undefined` before registration.
9. `scoreTask` / `scoreRun` require complete answers of the fitting kind and throw otherwise (deciders check completeness first).
10. Test case directories use emojis unused by the Protocol v2 cases and the react tests; `📐️shared-vectors` was renamed to `🗃️shared-vectors` after the react agent created `📐️quantity-formatting` six seconds earlier.

## For the other work packages

- Conformance: the TS unit suites use `ajv`, `jstat`, `mathjs` (devDependencies of `@semio-tech/quiz`); register them under `oracleHostPackages` / the registry if the dependency gate needs it.
- Coordinator: fix the design §6 Kendall sentence (decision 1); run `bun ./📜️script.ts schema generate` for the stale schema catalog.

## Follow-up fixes

Requested by the coordinator after the audit; mirrored with the Rust twin (`🔨️modules/👁️views/🦀️.rs` `learner_tag`, `🔨️modules/📏️scoring/🦀️.rs` `Option` returns).

1. **Leaderboard tag (contract change).** `🧬️schema/🟦️.ts` `LeaderboardRow.learner` → `tag: string`. `👁️views` exports `learnerTag(learner)` = `fnv1a32(learner)` as 8 lowercase hex digits, zero-padded (same name/location as Rust `learner_tag`; `""` → `811c9dc5`, `"a"` → `e40c292c`). `leaderboard` still orders total ↓, badges ↓, reachedAt ↑, learner id ↑, but sorts on an internal `{ learner, row }` pair and emits only the tag. Tests: tag format/padding/value over 200 ids, a tie resolved by id where the tag order is the opposite, and "the serialized leaderboard contains no learner id". The regenerated `🧫️fixtures/🏆️leaderboard` (rows with `tag`) reproduces.
2. **Robustness parity (no throw, no NaN).**
   - `mean([])` → 0 (empty run, classification without items, matching without dimensions all score 0).
   - Classification: a category's normalised profile is `undefined` unless it has a value on every task axis (Rust `Option`-collect); such categories are left out of `d_max` and earn/give 0 credit. The previous `axes.length > 0` special case is gone (empty axes → all distances 0 → `d_max = 0` → 0, as in Rust).
   - Sheet: a matching item missing a dimension value becomes a `NaN` card (Rust `unwrap_or(f64::NAN)`), never `undefined`; scoring such a task yields `undefined`.
   - `scoreTask` / `scoreRun` return `undefined` instead of throwing, exactly where Rust returns `None`: task id ≠ sheet task id, `answerRejection` set, `answerComplete` false, kind mismatch, sheet item/order id missing from the task, unknown dimension, missing true value or card; `scoreRun` also for a sheet task not in the quiz or without an answer. `scoreRun` now takes `quiz` from `sheet.quiz` (as Rust).
   - Matching discordance written as Rust's `(tᵢ > tⱼ ∧ aᵢ < aⱼ) ∨ (tᵢ < tⱼ ∧ aᵢ > aⱼ)`; sorting ranks computed over the learner's order items (same set as the sheet items for a valid answer).
   - `submitRun` maps an unscorable complete run to `run-incomplete` (Rust `ok_or(RunIncomplete)`).
3. **Tests added:** `⚖️partial-credit-scoring` (undefined contract for `scoreRun`/`scoreTask`, empty means, incomplete profiles vs mathjs distances, NaN cards), `🔁️run-lifecycle` (unscorable complete run → `run-incomplete`), `👁️read-views` (tag). README updated (leaderboard privacy, robustness paragraph).
4. **Verification (run):** `bun ./📜️script.ts test` → **8 files, 161 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 19 core files → exit 0; the conformance TS adapters show no quiz type errors.
5. **Open for the react package:** `🎯️targets/⚛️react/🔨️modules/🏆️leaderboard/🟦️.tsx` still reads `row.learner` (lines 90, 125, 129); it must switch to `row.tag` and compare with `learnerTag(me)` from `@semio-tech/quiz`.
