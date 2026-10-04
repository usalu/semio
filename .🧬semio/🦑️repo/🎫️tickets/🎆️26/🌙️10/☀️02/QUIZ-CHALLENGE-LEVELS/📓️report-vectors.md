# Report — Python reference, shared vectors and language-agnostic cases of the challenge levels

Agent: vectors. Product root `Q` = `🧰️framework/🛍️products/❓️quiz`. Ticket folder `T`.

## What changed

Created
- `Q/🧪️tests/⛰️challenge-rules/{🥒️.feature, 🐍️.py, 🟦️.ts, 🦀️.rs}` — the new case (4 scenarios: `rules`, `reach`, `clock`, `hints`).
- `Q/🧫️fixtures/⛰️challenge-rules/🔣️.json` (generated).
- `T/regenerate_challenge_vectors.py` — thin driver: loads the 09-28 generator and calls every writer the challenge touches (all but `randomness` and `presence`), or the writers named on the command line.
- `T/inspect_challenge_vectors.py`, `T/count_challenge_vectors.py`, `T/docstring_emoji_duplicates.py` — probes (kept as inputs).

Updated
- Python references (`🐍️.py`) of `🃏️sheet-assembly`, `✅️answer-validation`, `📏️sorting-concordance`, `🔀️matching-concordance`, `🕸️profile-similarity`, `🏅️badge-rules`, `📊️crowd-view`, `🏆️leaderboard`, `🧾️learner-lifecycle`, `🧬️schema-conformance` (unchanged: `🪪️identity-shapes`, `🎲️seeded-randomness`, `👥️shared-presence`).
- Features of the same ten cases plus `🪪️identity-shapes`: prose; new scenarios `📏️ @id-guessed`, `🔀️ @id-guessed`, `🕸️ @id-timed`; schema-conformance table +16 rows (guessed/timed groups, the new case, the malformed learner's run views).
- Bindings: TS `⛰️`, `📏️`, `🔀️`, `🕸️`, `🧾️` (site play by challenge, hidden-key answers, `open-task`); Rust `⛰️`, `📏️`, `🔀️`, `🕸️`, `🧬️` (conforming typed instances must decode into their twins — checked, not projected). The Rust `🧾️`/`🃏️` bindings had already been brought up to the new shapes by the Rust core agent and needed nothing more.
- All 12 generated fixtures except `🎲️seeded-randomness` and `👥️shared-presence`.
- Generator `.🧬semio/…/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py` (small hunks): `CH` module, `challenge()` writer, two new catalog badges (`hard-cooling` perfect-quiz ≥ hard, `expert-sorter` perfect-tasks sorting ≥ expert), hidden-key `correct_answer`/`reversed_answer`, `Learner.open/answer(at)/start(challenge)`, new learner sequences, leaderboard learners, crowd and schema instance vectors.
- `Q/🔮️oracles/🔣️.json`: capability `quiz-challenge-rules` on `quiz-python-reference`; numpy role and rationale extended.
- `🔣️taxonomy.json`: `⛰️challenge-rules` in `members-of-tests` and `members-of-fixtures`.

Removed: nothing.

## The new case `⛰️challenge-rules` — fixture shape

```
{ "$comment",
  "rules":    { "challenges": ["easy","medium","hard","expert"],
                "expected": { "table": {c: {keys,hints,timed,par}}, "ranks": {c: 0..3}, "meets": {c: {least: bool}} } },
  "points":   [ {id, score, challenge, expected: number} ]                         24 (6 scores × 4)
  "reaches":  [ {id, values, scale, expected: number | null} ]                     12 (null = unbounded)
  "misses":   [ {id, values, scale, truth, value, expected: {reach, miss}} ]       18 (at/beyond every bound)
  "seconds":  [ {id, kind, items, dimensions, expected} ]                           8
  "instants": [ {id, at, floor, now, expected} ]                                    8 (incl. floor > now)
  "tasks":    [Task]                                                                6
  "hints":    [ {id, task, sheetTask, answer?, expected: Hint[]} ]                 24 (all three kinds) }
```
Projections: `rules` → `{table, ranks, meets, points: {id: n}}`; `reach` → `{reaches: {id: n|null}, misses: {id: {reach, miss}}}`; `clock` → `{seconds, instants}`; `hints` → `{id: Hint[]}`. Profile `quiz-score-v1`. Every boundary sits on exactly computable numbers (linear differences, log10 of powers of ten).

Other new fixture groups: `📏️ guessed` (22) and `🔀️ guessed` (19) `{id, task, sheetTask, answer?, expected}`; `🕸️ timed` (8); `🃏️ sheets[].challenge` (27 sheets: every base seed at medium, 4 bases at all four challenges, ids `…-easy|-hard|-expert`); `🧬️ instances` (122: `{id, definition, document, violates?}`) and `accepted` +1 catalog with least challenges, `rejected` +3; lifecycle +5 learner sequences (`hints-on-easy`, `points-by-challenge`, `switching-the-challenge`, `the-clock`, `the-cap-before-the-switch`), site plays carry `challenge` per run (+1 `perfect-on-easy`); leaderboard +1 vector `points-across-challenges`; crowd +4 vectors with the icon quiz; badge-rules +9; identity +8 shapes; validation 66 + 4 malformed.

## Gates (exact commands, observed counts)

All from the repo root unless noted; logs went to `T/🗑️generated/vectors/` and were deleted afterwards.

| Gate | Command | Result |
|---|---|---|
| Fixtures | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` (every generator assertion passes; second run) | 12 of 12 fixtures `unchanged` |
| Python references | in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`: `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` (`.venv` Python 3.14.4, numpy 2.4.3, scipy 1.17.1, jsonschema 4.26.0) | cases=14 executed=43 passed=43 failed=0 |
| TypeScript subject | same folder: `bun ./📜️script.ts parity exhaustive --owner "…/❓️quiz" --implementation typescript` | executed=86 passed=86, parity=43/43 — the TS core matched the reference on the first run |
| Parity (all three) | same folder: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=14 executed=129 passed=129 failed=0 errored=0 **parity=129/129** (was 108/108: +4 scenarios of `⛰️challenge-rules`, +`guessed` ×2, +`timed`) |
| Quiz TS suite | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=300000 bun ./📜️script.ts test` | 11 files, 406 passed |
| Quiz Rust suite | `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | 175 passed, 0 failed (lib); doc-tests 0 |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | clean=true errors=0 warnings=0 |

Not run: proctor, React, site, e2e (other agents' areas).

## Deviations and decisions

1. **Matching where the cards show requires `assignments`** (`{kind: "matching"}` there is `answer-invalid`). My first reference accepted it as "valid and incomplete"; both cores reject it, reading §3.3 "assignments as before" with the §2 presence table. The design is silent on an answer with neither member, both cores agree, and their reading keeps the old contract — so I changed the reference (and the vector `matching-cards-without-assignments` now expects `answer-invalid`). Where the keys are hidden an answer without `guesses` stays valid and incomplete.
2. "Present" means present, also when empty: vectors `sorting-keys-with-empty-guesses`, `matching-cards-with-empty-guesses`, `matching-hidden-with-empty-assignments` → `answer-invalid` (both twins carry `Option` members, so they can tell `{}` from absent).
3. A sheet task is timed when it carries `seconds`; the task-level scoring cases use that (the cores do too). `scoreRun` in the reference takes timed from `sheet.challenge` — equal for every real sheet.
4. Keys shown + timed + no answer (never dealt): items in sheet order, every item a miss, score 0, no `miss` member; cards shown + timed + unassigned item: a miss, no `assigned`, no `miss`. Both vectored only for matching (`timed-cards-*`), as the design states it.
5. Hidden-key matching is decided per sheet task (any dimension without cards); sheets are uniform, so it equals the cores' per-dimension reading on every vector.
6. Hidden profiles keep only members that name an axis (TS agent's decision 6; no vector has another member).
7. Crowd: a guess is recognised by `miss` on the matching item result; a sorting result where every item has `miss` and none a `guess` is "a task without any answer" (score bin only) — the TS agent chose the same.
8. Hints: classification counts misplaced items also on a sheet that hides the keys (the design puts no condition on it); vector `classification-keys-hidden`.
9. Reversed order fully within the reach is impossible by construction (the extremes can only meet at the midpoint), so no such vector exists; "1, 2, 3 in the right order" is `log-counted-one-to-six` (< 1).
10. The site plays read the live catalog. When `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` gains `"challenge": "medium"` on its badges (site agent), the committed expectation of `perfect-on-easy` changes: rerun `.venv/Scripts/python.exe T/regenerate_challenge_vectors.py lifecycle` (with `PYTHONIOENCODING=utf-8` on Windows) and the parity gate. At the time of writing the catalog names no challenge.
11. Pre-existing duplicate docstring emojis remain in `🏆️leaderboard/🐍️.py`, `🪪️identity-shapes/🐍️.py` and the generator; I fixed the ones I introduced.

## Core deviations found and fixed

None needed. The TypeScript core agreed with the reference on all 43 scenarios at its first parity run. The Rust core agreed on everything once its subjects for the three new scenarios were registered. The only disagreement was on the reference side (decision 1), and I fixed the reference. Both cores also reached the same readings as the reference on design-silent points (decisions 3–7) without coordination, and the vectors now pin those readings down.

## Notes for the next agent

- Regenerate: `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py [writer …]` from the repo root; a second run prints `unchanged`.
- `🗃️shared-vectors/🟦️.ts` (TS agent's suite) replays the old groups only; the new groups are covered through the Protocol cases.
