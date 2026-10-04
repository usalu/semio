# Report: hint revision round 2 (design §8.4a), Python reference, shared vectors and bindings

Agent: hints-2 vectors. Abbreviations:
- `Q` = `🧰️framework/🛍️products/❓️quiz`
- `T` = this ticket folder
- `G` = the vector generator `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/UPDATING-ARCHITECTURE-QUIZ-WORDING/generate_quiz_vectors.py`

## Outcome

- The independent Python reference implements §8.4a items 1, 4 (sheets), 5, 6 and 8:
  - the verdict `under | over | reversed`;
  - familiar preference inside the tie window;
  - the profile anchor `other` / `above`, with the value form as fallback;
  - the cap of 3;
  - `short` passed through sheets.
- A numpy oracle recomputes all of it by a separate route.
- Every fixture is regenerated against the final quiz files, which carry the content agent's `short` and `familiar`.
- Both cores agree with the reference:
  - **parity 138/138**;
  - oracle 46/46;
  - TS suite 449/449.
- No core code needed a fix.

## Changed files

### Updated

**Python references**
- `Q/🧪️tests/⛰️challenge-rules/🐍️.py`:
  - Reference: `HINTS_PER_TASK`, `verdict`, `relation`, `compare_hints` (familiar, returns the weight), `profile_axis` (returns the ratio), new `profile_anchor`, `misplaced_hints` (weighted), new `capped`, `hints_of`.
  - numpy oracle:
    - `numpy_compare`: verdict by `numpy.sign` around the pivot, familiar mask.
    - `numpy_classification`: the anchor by `argmax` over masked distances.
    - New `numpy_capped`: stable `lexsort`, then `sort` of the kept positions.
    - `corroborate_hints`.
  - Module docstring updated.
- `Q/🧪️tests/🧾️learner-lifecycle/🐍️.py`:
  - The same hint functions, kept identical (with their own docstring emojis).
  - `short_of`; sheet items and hidden axes carry `short`.
- `Q/🧪️tests/🃏️sheet-assembly/🐍️.py`:
  - `short_of`; items and hidden axes carry `short`.
  - `familiar` counts as a member a sheet item must never carry.
  - A hidden axis may hold `{id, label, short}`.

**Bindings**
- `Q/🧪️tests/🧬️schema-conformance/🦀️.rs`: `twin_decodes` also maps `ShortText`, `Verdict`, `SortingItem`, `MatchingItem`, `ClassificationItem`, `Category`, `Axis` and `SheetItem`.
- The `⛰️challenge-rules` TS and Rust bindings are generic and needed no change.

**Features (prose)**
- `⛰️challenge-rules`:
  - verdict rule, familiar, profile anchor, the cap, the numpy route;
  - new `Then`/`And` lines in all three hint scenarios.
- `🧬️schema-conformance`: the new definitions and rejected quizzes.
- `🃏️sheet-assembly`: `short` passes through and `familiar` never does.
- No `:` follows a `shared://` URI anywhere.

**Oracles**
- `Q/🔮️oracles/🔣️.json`: the numpy role and the rationale now cover the verdict, the familiar masks, the anchor and the lexsort cap.

**Generator `G`**
- `quantity(…, short=None)`.
- `sheet_sorting` and `presented` carry `short`.
- `energy-basics`:
  - `short` on one classification item, one axis, one category, one sorting item and one quantity;
  - `living-room` is `familiar`.
- New synthetic tasks:
  - `FAMILIAR_POWER`: hair dryer and wallbox familiar.
  - `PARCELS`: matching, log and linear, with equal keys and equal truths.
  - `STANDARDS`: four profiled standards, two items each for two of them.
- New vectors, listed under "Vectors" below.
- Asserts rewritten for verdicts, the cap, anchors and familiar.
- `sorting-linear-over-below` renamed `sorting-linear-reversed-below`, because it is reversed now. The new `sorting-linear-over-below` and `sorting-over-below-with-a-factor-below-one` are found by search.
- Schema instances and rejected quizzes, listed below.

**Fixtures** (regenerated)
- `🃏️sheet-assembly`, `📏️`, `🔀️`, `🕸️`, `🏅️`, `🧾️learner-lifecycle`, `🏆️`, `🧬️schema-conformance`, `📊️` and `⛰️challenge-rules`.
- `🤝️wire-version`: the TS agent recommitted it (`wireVersion` 4, `e683a4482e9316af`). I checked it is byte-identical to the `🐍️.py` output with CR stripped, after all schema changes.

### Created / removed
- Created: this report.
- Removed: my tool output `T/🗑️generated/hints2`.

## Vectors

### `hints` (new)

- **Familiar:**
  - `sorting-familiar-anchors-win-the-tie-window-then-the-smallest-claim`: nuclear plant → hair dryer, over the smaller-claim tea light and the equal-claim kettle earlier in sheet order. LED → wallbox, the smallest claim among the familiar items.
  - `sorting-a-familiar-anchor-outside-the-tie-window-loses`: LED → kettle.
- **Verdict edges:**
  - `matching-equal-truths-claimed-apart`: ρ > 1 with τ = 1 → reversed; δ < 0 with Δ = 0 → reversed.
  - `matching-log-equal-keys-and-equal-truths`: ρ = 1 with τ < 1 → over; ρ < 1 with τ = 1 → reversed.
  - `matching-linear-equal-keys-and-equal-truths`: δ = 0 with Δ > 0 → under; δ > 0 with Δ = 0 → reversed.
  - Existing `matching-equal-keys…`: ρ = 1 with τ > 1 → under; δ = 0 with Δ < 0 → over.
- **Cap:**
  - `sorting-capped-at-three-by-error`: four misses, no anchor.
  - `matching-capped-at-three-across-dimensions-by-error`: errors 100, 100, 10, 10; the later 10 is dropped.
- An assert requires every combination of verdict × scale × claim side (12 in all) somewhere in the vectors.

### `classificationHints` (new)

- `profile-anchor-above-farthest-from-the-truth-then-first-in-sheet-order` → `retrofit-a`, `above: true`.
- `profile-anchor-below` → `efficient-house`, `above: false`.
- `profile-value-form-without-an-anchor-between` → no anchor.
- `profile-anchor-only-among-items-in-their-own-category`: misplaced items are skipped.
- `profile-capped-at-three-by-relative-gap`: ratios 1.83, 1.5, 2, 2; the 1.5 is dropped and sheet order is kept.
- Existing HEATING vectors now carry anchors.
- `classification-every-item-misplaced` is capped at 3.

### `quizHints` (authored quizzes, final content)

- The resting person on the top key → the familiar kettle, not larger unfamiliar anchors.
- The passive house as profile B → `heating` against `wschvo-1995`, `above: true`.
- The sheets carry the authored `short` labels.
- Every reversed-task vector has at most 3 hints.

### Schema instances

**Conforming**
- `ShortText`, including 40 code points of `ä`.
- `Verdict` (all three values).
- `Quantity` with `short`.
- `SortingItem`, `MatchingItem`, `ClassificationItem`, `Category` and `Axis` with `short` and `familiar` where allowed.
- `SheetItem` and `SheetAxis` with `short`.
- `CompareHint` with each verdict.
- `ProfileHint` above or below an anchor.
- `Hint` with an anchor.

**Breaking**
- `ShortText`:
  - 41 code points in EN or DE → `maxLength`;
  - empty → `minLength`;
  - no DE → `required`;
  - a third language → `additionalProperties`.
- `Verdict`: unknown or boolean → `enum`.
- Items: `familiar` as text → `type`; a `short` that is too long → `maxLength`.
- `familiar` on a classification item or a sheet item → `additionalProperties`.
- `CompareHint`:
  - without `verdict` → `required`;
  - with `under` → `additionalProperties`;
  - an unknown verdict → `enum`.
- `ProfileHint`:
  - `other` without `above`, or the reverse → `dependencies`;
  - `above` as text → `type`;
  - `other` not a slug → `pattern`.

**Quizzes**
- Rejected everywhere: an item `short` of 41 code points; `familiar` as text; a quantity `short` without DE.
- Accepted everywhere: a quiz with 40-code-point shorts and familiar items.

## Gates (exact commands, observed)

| Gate | Command | Result |
|---|---|---|
| Fixtures | repo root: `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` (final run) | 12/12 `unchanged`; every generator assertion passes |
| Wire | `.venv/Scripts/python.exe Q/🧪️tests/🤝️wire-version/🐍️.py`, CR stripped, `cmp` against the fixture | identical (4, `e683a4482e9316af`) |
| Oracle | in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=15 executed=46 passed=46 |
| Parity | same folder: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | executed=138 passed=138, **parity=138/138** |
| Quiz TS | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` | 11 files, **449 passed** (the lifecycle shared-vector failure that the TS and content agents reported is gone) |
| Quiz Rust, proctor | run by the Rust agent against my regenerated fixtures (see `📓️report-hints2-rust-proctor.md`) | 190 passed; proctor 90+15+21 |

**Not run by me:**
- React and site suites: the client agent owns them.
- `cargo test` on its own: parity exercises the Rust subjects, including the schema-conformance twin mapping.

## Deviations and decisions

1. **Verdict edges.**
   - §8.4a gives `(ρ ≥ 1) ≠ (τ ≥ 1)` with ρ ≠ 1 on a log scale, and `sign δ ≠ sign Δ` on a linear one. The two disagree at the mirrored edges:
     - a log truth τ = 1 with ρ > 1 would be `over`;
     - a linear claim δ = 0 would be `reversed`.
   - My first literal implementation followed both formulas. I switched to the symmetric rule that the TS agent also landed: `reversed = claim > p ? truth ≤ p : claim < p && truth ≥ p`.
   - This rule keeps exactly the two edges the design names: ρ = 1 is never reversed, and Δ = 0 is reversed.
   - It never asks a reversed question ("is X larger than R?") when the keys name no larger item.
   - It treats both scales alike.
2. **Cap output order.** The kept hints stay in emission order: learner order, or sheet order. §8.4a only says which hints are kept, so the §8.2 order still holds. I first returned them in rank order and aligned.
3. **Profile anchor "largest gap"** = the largest |Q − r|, the item's true distance from the anchor. Since r lies strictly between P and Q, this equals the smallest claimed |P − r|, so it mirrors the "most absurd claim" rule of §8.3. TS and Rust read it the same way.
4. **`familiar` on sheets.** The schema says it is "never presented", so the sheet reference treats it like a solution member.
5. **Vector rename.** `sorting-linear-over-below` (Davos) is now `reversed`, so it is renamed `sorting-linear-reversed-below`. Real over-below cases are found by search.

## Core disagreements

- None at parity: 138/138 on the first run against the final fixtures.
- The only differences were the readings in decisions 1 and 2, and the reference took them over before regeneration. No core file was touched.

## Open

- The feature prose still names the old 09-28 ticket path (`QUIZ-PRODUCT-AND-TEACHING-PROCTOR`). This is unchanged since round 1 and is for the owner to decide.
- German "beim {{quantity}}" with feminine or plural shorts: the content agent raised this for the client; it is not mine.

## Notes for the next agent

- Regenerate: `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py [writer …]`.
- Probe without writing: `T/probe_hint_vectors.py challenge [hints|classificationHints|quizHints]`.
- New synthetic task ids in `⛰️challenge-rules` `tasks`: `familiar-power-ratings`, `parcels`, `building-standards`.
- `energy-basics` (used by many fixtures) carries `short` on `ground-source`, the `seasonal-efficiency` axis, `electric-resistance`, `nuclear-plant` and the `energy-density` quantity; `living-room` is `familiar`.
