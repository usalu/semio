# Report: Python reference, shared vectors and case bindings for the specific hints (design §8)

Agent: hints vectors. Abbreviations:
- `Q` = `🧰️framework/🛍️products/❓️quiz`
- `T` = this ticket folder
- `G` = the vector generator, which now lives at `.🧬semio/…/🌙️09/☀️28/UPDATING-ARCHITECTURE-QUIZ-WORDING/generate_quiz_vectors.py`. Another session renamed the 09-28 ticket folder; see "Open".

## Outcome

- The independent Python reference implements §8.3, including the tie rule added on 2026-10-04.
- A numpy oracle recomputes every hint by a separate route: array ratios and differences, boolean pool and tie masks, `argmin` and `argmax` with first-on-tie semantics, `ptp` per axis, and `flatnonzero`.
- Every fixture is regenerated.
- The TypeScript and Rust cores agree with the reference: **parity 138/138**, up from 129. The two new scenarios across three implementations add 6, and the wire-version case adds 3.
- Apart from the coordinator's tie rule, no core reading disagreed with the reference. I implemented that rule minimally in both twins and updated their unit tests.

## What changed

### Created
- `T/add_additive_to_generator.py`: gives every `quantity(...)` call in `G` its `additive` argument.
- `T/probe_hint_vectors.py`: runs any writer of `G` without writing a fixture and prints the hints.
- This report.

### Updated

**Python references**
- `Q/🧪️tests/⛰️challenge-rules/🐍️.py`:
  - Reference functions: `relation`, `oriented`, `compare_hints`, `profile_axis`, `misplaced_hints` and `hints_of`.
  - numpy oracle: `numpy_compare`, `numpy_classification` and `corroborate_hints`.
  - New oracle scenarios `classification-hints` and `quiz-hints`.
- `Q/🧪️tests/🧾️learner-lifecycle/🐍️.py`: the same `hints_of`, kept identical to the one above, for the run views.

**Bindings**
- `Q/🧪️tests/⛰️challenge-rules/{🟦️.ts,🦀️.rs}`: one `hinted(group)` helper and three subjects: `hints`, `classification-hints` and `quiz-hints`.
- `Q/🧪️tests/🧬️schema-conformance/🦀️.rs`: the four hint `$defs` map to `Hint`, and `Quantity` decodes into its twin. The Rust agent made the same hint mapping at the same time.

**Features**
- `⛰️challenge-rules`: §8 rules, the tie rule, three hint scenarios and the numpy route.
- `🧬️schema-conformance`: the new definitions and 6 new table rows.
- `🧾️learner-lifecycle`: prose.

**Generator `G`**
- `quantity(…, additive)`: powers, energies, masses, lengths and volumes are `true`; everything else is `false`.
- New synthetic tasks:
  - `ELEVATIONS`, `LENGTHS` (linear, two pairs) and `DECADES` (log, two pairs);
  - `fill_levels` (linear near ties);
  - `BOILERS` (an axis without spread);
  - `MATERIALS` (four items of one category).
- Helpers `presented`, `dealt` and `searched`.
- Three hint groups with asserted branch outcomes.
- Lifecycle "hints-on-easy" assertion on the new shapes.
- Schema instances (below) and two rejected quizzes.

**Fixtures** (all 12 regenerated):
- `⛰️challenge-rules`, `🧾️learner-lifecycle`, `🧬️schema-conformance`, `🃏️sheet-assembly`, `📏️`, `🔀️`, `✅️`, `🏅️`, `🏆️`, `📊️` (`additive` in every quantity).
- `💭️crowd-client`: 4 quantities edited by hand.
- `🤝️wire-version`: `wireVersion: 3`, fingerprint `58fef8b489f6eb6f`. Printed by its `🐍️.py` and checked byte-equal after converting CRLF to LF.

**Other**
- `Q/🔮️oracles/🔣️.json`: numpy role and rationale.
- `T/regenerate_challenge_vectors.py`: finds `G` under any `☀️28/*`.

**Cores (tie rule only, minimal)**
- `Q/🔨️modules/⛰️challenge/🟦️.ts` `compareHint`.
- `Q/🔨️modules/⛰️challenge/🦀️.rs`: new `oriented`, plus `compare_hint`.
- Unit tests:
  - `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`: one test renamed and its expectation changed; new test `errors_within_the_slack_tie_to_the_smallest_claim_then_the_first_in_sheet_order`.
  - `Q/🧪️tests/🧗️challenge-ladder/🟦️.ts`: the mathjs oracle follows the tie rule; the tie test is rewritten (smallest claim, equal claims, near ties); two expectations updated.

### Removed
- Nothing. My tool output under `T/🗑️generated/hints` is deleted.

## Vectors (`⛰️challenge-rules`)

### `hints` (30 vectors): compare hints and every no-hint case

Anchors:
- An anchor is preferred over a missing extreme that has a larger error.
- Exact anchors tie to the smallest claim (nuclear plant → tea light at 0.225; LED bulb → wallbox).
- Equal claims (kettle and hair dryer, ×5) take the first in sheet order.
- The single most wrong non-exact anchor wins.

No anchor:
- Linear pair, and log matching with a factor of 0.15.
- A lone assignment has no reference, so no hint.

Under and over:
- Log under above 1 (nuclear plant / kettle ×5).
- Log under below 1 (`DECADES`, found by search: coin / truck at 0.1).
- Log over in the same direction.
- Linear under above (server room / bedroom, +25).
- Linear under below (`LENGTHS`, −10).
- Linear over below.
- Wrong direction.
- Equal keys give factor 1 with `under: true` and difference 0 with `under: false`.

Near ties:
- 4000 ± 1e-6 is within the slack, so the smallest claim wins (quarter tank).
- 4000 ± 1e-5 is beyond the slack, so the largest error wins (tank A).

Also covered:
- Matching per dimension, on both scales.
- Hidden keys, no answer, nothing assigned, no spread.

### `classificationHints` (22 vectors)

Profile hints:
- Axis choice: investment wins at 2.0 against 1.92 and 1.91.
- Two misplaced items.
- Near miss gives no hint.
- Gap exactly at the reach gives no hint.
- Axes tie at 2.0 after skipping an axis without spread: seasonal efficiency.
- Identical profiles give no hint and no group fallback.
- Hidden-key sheet.

Group and category hints:
- Missing profile leads to the group rule.
- Group together, including skipping a partner of the same category.
- Group apart, past unassigned and alike-placed partners.
- Category hint with and without a description.
- Hints follow sheet order.

### `quizHints` (14 vectors, authored quizzes, easy sheets)

- **Sun on the smallest key:**
  - Sun / tea light, factor 0.35, `under: false`.
  - The resting person on the top key ties among exact anchors; the smallest claim wins (sunlight on Earth).
- **Sun on the smallest key, every other item one up.**
- **Standard profiles:**
  - Passive house as profile B: axis `heating` (three axes tie at 2.0, so the first in axis order).
  - KfW 40 as profile C: no hint, because the cooling gap of 3 equals its reach of 3.
- **Tea light among the energies:** a group hint.
- **Every task of physics, heating, demand and cooling, reversed.**

### Schema instances

- Conforming: `Quantity` with `additive` true and false; every new hint `$def`; `Hint` of every kind.
- Breaking:
  - Missing `additive` → `required`; `additive` as text → `type`.
  - Compare hint without a factor or difference, or with both → `oneOf`.
  - `factor` 0 → `exclusiveMinimum`.
  - Other `required`, `additionalProperties`, `type` and `pattern` cases.
  - The removed `magnitude` and `misplaced` hints → `oneOf`.
- Rejected quizzes: a quantity without `additive`, and a dimension with `additive` as text.

## Gates (exact commands, observed)

| Gate | Command | Result |
|---|---|---|
| Fixtures | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` (repo root, final run) | 12/12 `unchanged`; every generator assertion passes |
| Wire | `.venv/Scripts/python.exe Q/🧪️tests/🤝️wire-version/🐍️.py` output with CR stripped, compared to the fixture | identical |
| Oracle | in `…/🦑️repo/🔨️modules/🧪️test`: `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=15 executed=46 passed=46 |
| Parity | same folder: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | executed=138 passed=138 **parity=138/138** |
| Quiz TS | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` | 11 files, **431 passed** (5 failed before the tests were adapted to the tie rule) |
| Core typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.core.json` | 0 errors |
| Quiz Rust | `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | **184 passed** (1 failed before: the old first-in-sheet-order expectation) |
| Clippy | `cargo clippy -p semio-framework-quiz --lib --tests -- -D warnings` | clean |
| Proctor | `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` | 90 unit + 15 conformance + 21 e2e passed |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | clean=true |

**Not run:** the React suite and the site. The tie rule may change which `other` the React and site tests expect for exact-anchor ties; the React agent should rerun them.

## Deviations and decisions

1. **Tie test.** A member is tied when `e × (1 + REACH_SLACK) ≥ max e`. This has the same form as `misses`, and all three implementations use exactly this expression. The oriented claim is `max(ρ, 1/ρ)` or `|δ|`; a strict `<` keeps the first in sheet order.
2. **Profile versus group.** When both categories carry profiles, only the profile rule applies, even when it finds no axis. Group and category hints apply only where a profile is missing. This matches the TypeScript note.
3. **Hidden-key classification sheets still get hints from the task's true profiles.** Hints are shown only on easy, and §8 sets no condition on keys for classification.
4. **Additive in synthetic quizzes.** Masses, lengths and volumes count as additive, beside powers and energies. Heights, temperatures, loads per m², densities, CO₂ factors and lifespans do not. The authored quizzes follow §8.1: only the physics powers and energies are additive.
5. **Searched vectors.** Two vectors (`sorting-under-below-with-a-factor-below-one` and `sorting-linear-under-below`) are found by searching the permutations of a small task in lexicographic order. The tie rule made the hand-picked POWER and ELEVATIONS orders stop producing these branches. The search is deterministic and asserted.
6. **Hint scenarios split by family.** The hints are split into three scenarios (compare, classification, authored quizzes) so that a failure names its family.

## Core disagreements

No core reading disagreed with §8. Both cores matched the reference on the first parity run.

The only core changes are the coordinator's tie rule, made in both twins:
- TS `compareHint`;
- Rust `oriented` and `compare_hint`;
- their unit tests.

## Open

- **Renamed 09-28 ticket folder.** Another session renamed it from `QUIZ-PRODUCT-AND-TEACHING-PROCTOR` to `UPDATING-ARCHITECTURE-QUIZ-WORDING`, ticket title "Updating Architecture Quiz Wording". The 14 quiz features, every fixture `$comment`, `🎓️teaching/🏛️architecture/README.md` and the proctor e2e still name the old path. I left them as they are, because the rename looks unintended; the owner should decide.
- **React and site suites** were not rerun after the tie rule.

## Notes for the next agent

- Regenerate: `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py [writer …]`.
- Look at hints without writing any fixture: `T/probe_hint_vectors.py challenge` or `T/probe_hint_vectors.py lifecycle learners`.
- The fixture groups of `⛰️challenge-rules` are `hints`, `classificationHints` and `quizHints`, all of shape `{id, task, sheetTask, answer?, expected}`. `tasks` includes every task of the four authored quizzes.
