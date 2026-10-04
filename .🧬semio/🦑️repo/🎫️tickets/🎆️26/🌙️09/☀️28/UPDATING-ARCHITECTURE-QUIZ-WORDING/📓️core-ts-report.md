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
6. **Resolved since:** the react leaderboard now keys and highlights rows by `row.tag` against `learnerTag(state.learner.id)` (checked 2026-09-29).

## 2026-09-29

Contract change from design §14 (already applied to `🧬️schema/🔣️.json`): `Quiz.emoji` and `CatalogQuizView.emoji` are required strings of 1–16 characters (one emoji grapheme).

1. **Types:** `🧬️schema/🟦️.ts` — `Quiz.emoji: string` (after `id`, as in the schema) and `CatalogQuizView.emoji: string`.
2. **Validation:** `quizIssues` requires `emoji` (missing → `required` at `/emoji`, non-string → `type-invalid`) and checks its length in code points, 1…16 (→ `length-invalid` at `/emoji`) — the same bounds and code the badge emoji already used, and identical to the Rust twin (`issues.length("/emoji", &quiz.emoji, 1, 16)` in `✅️validation/🦀️.rs`; a missing field fails Rust deserialization, which the TS `required` issue mirrors). Decision: no "exactly one grapheme" check — the schema states it only as a description, the Rust twin does not check it, and a grapheme rule would need `Intl.Segmenter` on one side and a Unicode segmentation table on the other; joined (`🧑‍🏫`), variation-selector (`❄️`) and flag (`🇨🇭`) emojis pass.
3. **Views:** `catalogView` copies `emoji` into every `CatalogQuizView` (Rust `catalog_view` does the same).
4. **Tests I own:** every hand-built `Quiz` now carries an emoji; `🩺️document-validation` adds missing / empty / 17-code-point / numeric emoji cases (each differentially checked against ajv) and a positive case for joined, flag and 16-code-point emojis; `👁️read-views` asserts the emoji in the catalog view. The regenerated shared vectors (with quiz emojis and the `missing-emoji`/`empty-emoji`/`overlong-emoji` schema-conformance rejections) reproduce.
5. **README:** the Quiz section describes the emoji; the `length-invalid` row lists the bounds.
6. **Verification (run):** `bun ./📜️script.ts test` → **8 files, 166 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` (root config + `noUncheckedIndexedAccess`, `noUnused*`) over the 19 core files via `🗑️generated/tsconfig.core-ts.json` → exit 0; the ticket's `check_quiz_core.ts` over the four architecture quizzes → `failures: 0` (the content already carries emojis).

## 2026-09-29 — shared presence and cursors (design §15)

1. **Types (`🧬️schema/🟦️.ts`, region Presence):** `SCREENS`/`Screen`, `Place`, `Anchor`, `Cursor`, `PresenceState`, `CursorState` — one type per new `$defs` entry, same names, `readonly`.
2. **Module `🔨️modules/👥️presence/🟦️.ts`** (same directory as the Rust twin `👥️presence/🦀️.rs`), exported from the package barrel:
   - `rosterScope(catalog)` → `<catalog>`; `roomScope(catalog, place)` → `<catalog>/introduction|identity|home|leaderboard`, `<catalog>/quiz/<quiz>` for run and results, `undefined` for run/results without a quiz (identical to Rust `room_scope`).
   - `presenceProblem(state)` / `cursorProblem(state)` → the first `{ path, code }` (smallest path, then code, code point order — Rust's `min_by`) or `undefined`. They accept `unknown` (wire input) and are backed by `presenceIssues`/`cursorIssues` in `✅️validation` (all issues, sorted), next to the other document checks.
   - `presenceRoster(entries, display)`: entries `{ session, state }` grouped by tag; representative session = active first, then smallest session id; `sessions` sorted; `online`/`active` = distinct learners; `quizzes` = distinct learners at a run or results of each quiz (keys sorted); learners sorted by lowercase display, then display, then tag (code point order). The label function comes from the client, so the core stays language-neutral; the result is independent of entry order. Client-only (no Rust twin requested).
   - `isTag`, `isAnchor` (Rust `is_tag`, `is_anchor`) live in `✅️validation` so validation needs no import from presence; the package vocabulary is the same.
3. **Codes (agreed with the Rust twin's vocabulary):** `tag-invalid` (`/tag`), `anchor-invalid` (`/cursor/anchor`, `/focus`), `out-of-range` (`/cursor/x|y`: not finite or outside 0…1; a non-number is `type-invalid`), `length-invalid` (`/identity/handle`, 1…64 code points), `slug-invalid` (`/place/quiz|task`), `required` at `/place/quiz` for run/results without a quiz; plus `quiz-outside-run` (`/place/quiz` on another screen) and `task-without-run` (`/place/task` on a screen other than run); structural codes as elsewhere (`type-invalid`, `required`, `property-unknown`, `value-invalid`). Place rules apply only on a known screen.
4. **Refactor:** `normalizeHandle`/`NormalizedHandle` moved from `🧾️lifecycle` to `✅️validation` (validation now needs the handle rules and lifecycle already imports validation — no import cycle). The public name and behaviour are unchanged; every consumer imports it from `@semio-tech/quiz`.
5. **Tests:** new `🧪️tests/🫂️presence-roster/🟦️.ts` — scope table; 22 presence and 17 cursor cases, every one checked differentially against **ajv 8.20.0** on the normative `PresenceState`/`CursorState` (schema-valid ⇔ only the three place rules beyond the schema fire); first-issue ordering; NaN/±∞ coordinates; `isTag`/`isAnchor`; roster grouping, representative choice, per-quiz counts, sorting, order independence, empty input. `🗃️shared-vectors` now also runs every presence/cursor vector of `🧫️fixtures/👥️shared-presence` (accept/refuse all agree).
6. **README:** new "Presence and cursors" section, layout row, state-class row, and the five new codes.
7. **Verification (run):** `bun ./📜️script.ts test` → **9 files, 216 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 21 core files → exit 0.

**Open — needs the coordinator:**
- **Identity room disagreement.** The coordinator's brief and the Rust twin give the identity screen the room `<catalog>/identity` (implemented); design §15 lists no identity room and the conformance fixture `👥️shared-presence` expects `room: null` for it ("the identity screen is shared with nobody"). One of them must change; the scope vectors are therefore not yet in `🗃️shared-vectors`.
- **Rust twin lacks two place rules** the conformance oracle and this core enforce: `task-without-run` (the coordinator's own rule) and `quiz-outside-run`. Until `presence_problem` adds them, Rust admits `task-on-results`, `task-on-home`, `quiz-on-home`, `quiz-on-leaderboard`, which the fixture refuses.
- **Taxonomy (site-infra):** register `👥️presence` in `members-of-modules` and `🫂️presence-roster` in `members-of-tests`.

### Presence follow-up — coordinator decision (design §15 "Decision 2026-09-29")

- `roomScope` now returns `undefined` for the identity screen (a learner there has no tag yet), so rooms exist only for introduction, home, leaderboard and `quiz/<quiz>`; README "Presence and cursors" updated. The Rust twin's `room_scope` still maps `Screen::Identity` to a room as of 18:20 — the coordinator relayed the decision.
- `🗃️shared-vectors` now runs the fixture's scope vectors as well (all 10, including the regenerated `identity → null` and the new `run-without-quiz → null`), next to every presence and cursor acceptance vector; `🫂️presence-roster` expects `undefined` for identity.
- The two place rules (`task-without-run`, `quiz-outside-run`) are confirmed for both cores by the same decision; this core already enforces them.
- Taxonomy for `👥️presence` / `🫂️presence-roster` follows in the final taxonomy pass (coordinator).
- **Verification (run):** `bun ./📜️script.ts test` → **9 files, 217 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 21 core files → exit 0.

## 2026-09-29 — screens of the layered home (design §16)

1. **Types:** `SCREENS` / `Screen` in `🧬️schema/🟦️.ts` now follow the schema enum exactly: `introduction`, `identity`, `home`, `quiz`, `run`, `results`, `leaderboard`, `learner`, `badges`, `preferences`; `Place` docstring updated.
2. **Rooms (`roomScope`):** `quiz`, `run`, `results` → `<catalog>/quiz/<quiz>` (`undefined` without a quiz); `introduction`, `home`, `leaderboard`, `badges` → `<catalog>/<screen>`; `identity`, `learner`, `preferences` → `undefined` (no tag yet / personal pages). Identical to the Rust twin's `room_scope` (checked 19:51: `Screen::Identity | Screen::Learner | Screen::Preferences => None`, `Screen::Badges => "badges"`, `Screen::Quiz | Screen::Run | Screen::Results => quiz/<quiz>`).
3. **Place rules (same codes as the Rust twin's `presence_issues`):** on the quiz page, a run or its results a missing quiz is `required` at `/place/quiz`; a quiz on any other screen is `quiz-outside-run` (name kept, meaning widened to "outside the quiz page, a run or its results"); a task anywhere but a run is `task-without-run`.
4. **Roster:** `presenceRoster` counts a learner in a quiz when any of its sessions is on that quiz's page, a run or its results (same notion as the quiz room).
5. **Tests:** `🫂️presence-roster` adds the five new scope rows, valid states on all ten screens, and the cases quiz page without quiz, task on the quiz page, quiz on learner/badges/preferences (each checked against ajv on the updated enum); the roster case now has a learner on a quiz page. `🗃️shared-vectors` runs the regenerated `👥️shared-presence` vectors (19:51:36 — 15 scopes, 37 presence and 26 cursor states), all matching.
6. **README:** Place, Rooms, Admission, Roster bullets and the `quiz-outside-run` row updated.
7. **Verification (run):** `bun ./📜️script.ts test` → **9 files, 222 tests passed** (before and after the fixture regeneration); `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 21 core files → exit 0.

## 2026-09-29 (night) — what the others think (design §17)

1. **Types (`🧬️schema/🟦️.ts`):** `ThinkingState`, `CursorState.drag { item }`, region Crowd with `CrowdCount`, `CrowdItem`, `CrowdTask`, `CrowdView`, and `Query` `{ type: "crowd", quiz }`.
2. **`crowdView(quiz, results)`** (`👁️views`, Rust `crowd_view`): results of other quizzes ignored, `runs` = the rest; per result the first task result with the task's id *and* kind; classification counts per assigned category; sorting `meanPosition` = mean of `position / (n − 1)` (0 for `n < 2`) summed in result order; matching one crowd task per dimension in definition order with counts per `valueKey(assigned)`; tasks/dimensions/items in definition order, unanswered items omitted (a task with no answers keeps `items: []`).
3. **`valueKey(value)`** (Rust `value_key`): ECMAScript `Number::toString` = `JSON.stringify` of a finite number — shortest round-trip digits, plain notation for decimal exponents −7 < e < 21, else `d.ddde±x`, `-0` → `0`. Tested on fixed cases (`1e+21`, `1e-7`, `5e-324`, `1.7976931348623157e+308`, `0.30000000000000004`, …) and 2000 random doubles (round-trip and equality with `JSON.stringify`); the Rust twin formats from `{:e}` digits with the same layout.
4. **Key order — decision:** counts are ordered by key in **code point order for category ids and value keys alike** (`120` before `15`). The coordinator's brief asked for numeric order of values; the Rust twin (`BTreeMap<String, _>`) and the conformance oracle (`📊️crowd-view/🐍️.py`, pinned "because the text leaves it open") had both already fixed code point order, so I aligned for three-way parity instead of splitting the contract. If numeric order is wanted, all three change together (TS `counted`, Rust `counted`, the Python oracle and its vectors). The client can always re-sort values numerically for display.
5. **Thinking (`👥️presence` + `✅️validation`):** `thinkingScope(catalog, quiz)` → `<catalog>/quiz/<quiz>/thinking`; `thinkingProblem`/`thinkingIssues` with the Rust twin's rules and codes: structural per schema (possibly partial drafts), `tag-invalid`, `slug-invalid` for task ids, items, categories, dimensions, order entries; `THINKING_LIMIT` = 64 — `too-many` beyond 64 tasks or 64 entries per classification map, order, matching map or dimension; `duplicate-id` at every repeat in an order; `out-of-range` for a card index ≥ 64. (My first cut had a 2048-byte `too-large` bound instead; dropped for parity — the framework socket already caps states at 2 KiB.)
6. **Cursor:** `drag { item }` with `slug-invalid` at `/drag/item`; `item:<id>` / `category:<id>` anchors already satisfy the anchor grammar.
7. **`thinkingCrowd(states, sheetTask)`** (client-only, `👥️presence`): one state per tag (last given wins); items in the viewer's sheet order, unanswered omitted; classification votes per category with tags (code point order); sorting per-tag positions `index / (len − 1)` in each learner's own order; matching lists who matched an item per dimension — **no values**, because a draft's card indices point into that learner's own shuffled cards (sheets differ per seed and draw), so another learner cannot read a value from them. To show others' matched values live, the contract needs a semantic draft (e.g. a thinking-only matching answer carrying values per dimension and item, which the publisher can fill from its own cards) — coordinator decision.
8. **Tests:** new `🧪️tests/🗳️crowd-answers/🟦️.ts` (valueKey vectors and random round-trip; crowdView hand-computed vectors, ajv validity of `CrowdView`, empty input, key order and `-0`/`0` merge, brute-force sorting means; thinking scope, 15 structural cases and the limits, each checked against ajv — schema-valid ⇔ only `too-many`/`duplicate-id`/`out-of-range` fire; drag and item/category anchors; thinkingCrowd per kind, last-wins, order independence). `🗃️shared-vectors` runs the conformance `📊️crowd-view` vectors (5, all match within 1e-12).
9. **README:** new section "What the others think", cursor bullet updated (§17 supersedes "never share answers"), state classes, codes `duplicate-id`, `out-of-range`, `too-many`.
10. **Verification (run):** `bun ./📜️script.ts test` → **10 files, 257 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 22 core files → exit 0.
11. **Taxonomy (final pass):** register the test directory `🗳️crowd-answers`.
12. **Housekeeping incident:** when cleaning up my temporary tsconfig I removed the whole ticket `🗑️generated/` folder (recreated by me minutes earlier after the ticket close had deleted it) instead of only my file; any output another agent wrote there between ~22:20 and ~22:40 (e.g. screenshots or logs of `quiz_ui_screenshots.ts`, `quiz_presence_two_devices.ts`, `layered_overview_rig.ts`) is gone and must be regenerated by re-running that script. The folder exists again (empty).

### Crowd follow-up — coordinator decisions (semantic drafts)

1. **Key order:** code point order for all count keys is confirmed (clients sort values numerically for display); no change.
2. **Contract change `ThinkingAnswer`:** `🧬️schema/🟦️.ts` gains `ThinkingMatchingAnswer { kind: "matching", values: dimension → item → number }` and `ThinkingAnswer = ClassificationAnswer | SortingAnswer | ThinkingMatchingAnswer`; `ThinkingState.answers` holds `ThinkingAnswer`s.
3. **`thinkingIssues`/`thinkingProblem`:** matching drafts are checked under `/answers/<task>/values` — slugs for dimensions and items (`slug-invalid`), `too-many` beyond 64 dimensions or 64 items per dimension, and a value that is not a finite number is `type-invalid` (identical to the Rust twin's `thinking_issues`, which reports `type-invalid` for `!value.is_finite()`; my first cut used `out-of-range` and was switched). A matching draft with card `assignments` is refused (`property-unknown` + `required` at `/values`). Card-index bounds are gone with the card indices.
4. **`thinkingAnswer(sheetTask, answer)`** (new export, `👥️presence`; suggested Rust name `thinking_answer`): `undefined` when `answerRejection` refuses the answer for the sheet task; classification and sorting answers are returned unchanged; matching card indices become the card values of the publisher's own sheet (entries whose card value is not finite are left out, so a broken quiz never publishes NaN).
5. **`thinkingCrowd`:** matching now yields per dimension and item the distribution of the peers' values — `votes` keyed by `valueKey(value)` with tags, keys and tags in code point order (the same vote folding as classification).
6. **Tests:** matching drafts switched to `values`; new cases (textual value, dimension not a map, card indices instead of values, NaN/±∞ → `type-invalid`, 64/65 values per dimension, 65 dimensions); `thinkingAnswer` per kind, empty, NaN cards, invalid answers, and its output admitted by `thinkingProblem`; matching crowd distribution. `🗃️shared-vectors` now also runs the fixture's `thinkingScopes` and all 31 `thinking` accept/refuse vectors (regenerated with semantic drafts at 22:34) — all agree.
7. **README:** drafts and live crowd bullets rewritten; code rows `out-of-range` (cursor only) and `too-many` (values maps) updated.
8. **Verification (run):** `bun ./📜️script.ts test` → **10 files, 260 tests passed**; `bun nx run @semio-tech/quiz:test --skip-nx-cache` → passed; strict `tsc --noEmit` over the 22 core files → exit 0.
9. **`🗑️generated/`:** acknowledged — from now on only my own files in it are created and removed (the temp tsconfig `tsconfig.core-ts-crowd.json` was deleted individually; `conformance-*.txt`, `layered-final/`, `proctor/` untouched).

### Taxonomy registration of `🗳️crowd-answers`

- One anchored insertion in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `"🗳️crowd-answers"` appended to `semanticDirectoryMemberKinds["members-of-tests"].memberNames` right after `"📊️crowd-view"` (re-read before editing; the identical tail of `members-of-fixtures` was left alone; file still parses as JSON; diff is exactly that one entry).
- `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"`: before `clean=false errors=1` (`directory-kind-unresolved 🧪️tests/🗳️crowd-answers`), after **`clean=true errors=0 warnings=0`**.
- `bun ./📜️script.ts test` (package) still **260 passed**; the scope log was a file of mine in `🗑️generated/` and was removed individually.
