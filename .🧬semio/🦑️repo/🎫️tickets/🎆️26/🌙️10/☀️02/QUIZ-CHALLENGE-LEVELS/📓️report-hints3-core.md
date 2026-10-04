# Report: hint revision round 3, core (design §8.4b item 3, reference diversity)

Agent: hints-3 core, 2026-10-04. The final reading is in `📓️hints-schema-landed.md`, "Round 3".

Abbreviations:
- `Q` = `🧰️framework/🛍️products/❓️quiz`
- `T` = this ticket folder
- `G` = the vector generator `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/UPDATING-ARCHITECTURE-QUIZ-WORDING/generate_quiz_vectors.py`

## Outcome

Inside the tie window, a reference not yet named by an earlier kept compare hint of the same task (per dimension for a matching) wins first. After that come familiar, then the smallest oriented claim, then sheet order.

All three implementations and the numpy oracle are aligned:
- TS twin, Rust twin and Python reference;
- the numpy oracle;
- the lifecycle Python copy, kept identical.

Cap interaction:
- Hints are weighed and capped first; the references of the kept hints are then chosen in display order.
- The cap weight of a compare hint is now the largest error of its pool. It used to be the error of the chosen reference; the two differ by at most the 1e-9 slack, and the new weight does not depend on the choice.

Results:
- Parity is **138/138**.
- One committed vector changed reference (`sorting-under-above-and-over-in-the-same-direction`: the wallbox now takes `hair-dryer`).
- 6 new vectors were added.
- One proctor end-to-end assertion changed.

## Changed files

### Updated

**Twins**
- `Q/🔨️modules/⛰️challenge/🟦️.ts`: new `Related` and `Pending` types, `compareHint` (now pending, weighted by `max e`) and new `referenced`. `capped` returns entries, and `hintsOf` resolves them after the cap.
- `Q/🔨️modules/⛰️challenge/🦀️.rs`: new `Related`, `relation`, `Pending` and `enum Drafted`; `compare_hint` and new `referenced`; `capped<T>` is now generic; `hints_of` is restructured.

**Unit tests**
- `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`: 7 expectations follow the rule. New test `a_reference_no_earlier_hint_of_the_task_names_wins_the_tie_window_first`, which covers a smaller claim, an equal claim, a familiar reference, forced reuse, the cap, and per-dimension diversity.
- `Q/🧪️tests/🧗️challenge-ladder/🟦️.ts`: the mathjs oracle (`oracleCompare`, `oracleCapped`) now chooses references after the cap with diversity. 6 expectations are updated, and there is one new `it` for diversity.

**Python reference and oracle**
- `Q/🧪️tests/⛰️challenge-rules/🐍️.py`: `compare_hints`, new `referenced`, `capped`. numpy: `numpy_compare` and new `numpy_referenced` (an `isin` mask); `numpy_capped`. Module docstring updated.
- `Q/🧪️tests/🧾️learner-lifecycle/🐍️.py`: the same reference changes, kept identical.

**Prose**
- `Q/🧪️tests/⛰️challenge-rules/🥒️.feature`: the rule and the numpy route, plus a new `And` line.
- `Q/🔮️oracles/🔣️.json`: the numpy role and the rationale now mention the `isin` mask.
- `Q/README.md`: the hints section and the cap weight.

**Vectors**
- `G`: two sheets (`laptop_power`, `every_power`) and 6 vectors:
  - `sorting-a-later-hint-takes-a-reference-not-yet-named-over-a-smaller-claim`
  - `…-over-a-familiar-one`
  - `sorting-a-reference-is-named-again-when-the-tie-window-holds-no-other`
  - `sorting-a-hint-the-cap-drops-uses-up-no-reference`
  - `matching-a-later-hint-of-the-dimension-takes-a-reference-not-yet-named`
  - `matching-a-reference-named-in-another-dimension-is-not-used-up`
- `G` also gained 7 asserts.
- `Q/🧫️fixtures/⛰️challenge-rules/🔣️.json` was regenerated.

**Proctor**
- `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs`: the laptop's hint now names `phone-charger` (factor 600, `over`), because the heat pump already names the kettle.

**Reading**
- `T/📓️hints-schema-landed.md`: "Round 3".

### Created
- `T/hints3_vector_diff.py`: diffs the hint lists against the committed fixtures without writing.
- `T/hints3_diversity_probe.py`: brute-force search for the branch cases.
- This report.

### Removed
- My tool output, `T/🗑️generated/hints3`.

## Gates (exact commands, observed)

All commands run from the repo root unless a folder is named.

| Gate | Command | Result |
|---|---|---|
| Fixtures | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` | final run after `📓️report-hints3-content.md` was final: 12/12 `unchanged`, every generator assert passes; the fixtures carry the content agent's new shorts |
| Quiz TS | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` | 11 files, **450 passed** |
| Quiz Rust | `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 cargo test -p semio-framework-quiz` | **191 passed** |
| Clippy | `cargo clippy -p semio-framework-quiz --all-targets -- -D warnings` | exit 0 (after `then` → `then_some`) |
| Typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.core.json` | exit 0 |
| Oracle | in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | 46/46 |
| Parity | same folder: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | **138/138** |
| Proctor | `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` | 90 + 15 + 21 passed. The first run had 1 failure, the changed reference above. |

Not run:
- Proctor clippy: the only change there is a JSON literal.
- React and site suites: those are other agents' areas. The site spec asserts the reference by structure (`referenceIn` among the anchors), so it does not pin a reference.

## Deviations and decisions

1. **The cap comes before the choice of references.**
   - The design says the hints "are decided in their order". The question was which order: the order of all hints, or the order of the kept ones. I chose the order of the kept hints, in display order.
   - Assigning references before the cap would let a hint the learner never sees use up a reference. The vector `sorting-a-hint-the-cap-drops-uses-up-no-reference` pins this down: the dropped hair-dryer hint would otherwise take the tea light from the wallbox.
2. **The cap weight is the largest error of the pool**, not the error of the chosen reference.
   - This avoids a circle: the weight would otherwise depend on the choice, and the choice on the cap.
   - It changes no committed vector. A probe over all fixtures before the new vectors showed only the one intended reference change.
3. **What counts as "used".** Only `other` counts. The `item` of an earlier hint is not "used". This follows the brief.
4. **Diversity never widens the tie window**, and it never overrides the anchor pool.

## Notes for the next agent

- No schema or wire change.
- Client texts are unaffected: only which `other` is named changes.
