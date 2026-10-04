# Report: hint revision round 2 (design §8.4a), Rust twin and proctor

Agent: Rust twin + proctor of round 2. Normative: `📓️design.md` §8.4a. The shared readings are the "Round 2" section of
`📓️hints-schema-landed.md`, which the TypeScript agent wrote. The Rust twin matches it field for field and reading for
reading.

Abbreviations: Q = `🧰️framework/🛍️products/❓️quiz`, P = `🎓️teaching/🛂️proctor`.

## Changed files

Updated:

- `Q/🧬️schema/🦀️.rs`:
  - `WIRE_VERSION` 4.
  - New `ShortText = Text`, `SHORT_LENGTH = 40`.
  - New `Verdict { Under, Over, Reversed }` (kebab-case on the wire), and `VERDICTS`.
  - `short: Option<ShortText>` right after `label` on `Quantity`, `Axis`, `Category`, `ClassificationItem`,
    `SortingItem`, `MatchingItem`, `SheetItem` and `SheetAxis`.
  - `familiar: Option<bool>` on `SortingItem` (after `value`) and `MatchingItem` (after `values`), before
    `explanation`. It is on no sheet type.
  - `CompareHint.under: bool` became `verdict: Verdict`.
  - `ProfileHint` gained `other: Option<Slug>` and `above: Option<bool>`.
- `Q/🔨️modules/✅️validation/🦀️.rs`: every `short` is checked at 1…`SHORT_LENGTH` code points per language
  (`…/short/en|de`, `length-invalid`). A private `Head` struct replaces the item-head tuple.
- `Q/🔨️modules/🃏️sheet/🦀️.rs`: `short` is copied onto sheet items and hidden-key axes and categories. Shown-key
  categories, axes and quantities are the task's own, so they already carry it.
- `Q/🔨️modules/⛰️challenge/🦀️.rs` (§8.4a 1, 5, 6, 8):
  - `pub const HINTS_PER_TASK: usize = 3`.
  - `pub fn verdict_of(claim: f64, truth: f64, pivot: f64) -> Verdict`.
  - `relation` returns `(error, Verdict)`.
  - `compare_hint` prefers familiar items inside the tie window and returns its error with the hint.
  - `profile_axis` returns `(axis, gap/reach)`.
  - New `profile_anchor` (the anchor that lies strictly between claim and truth; the largest `|Q − r|` wins, then sheet
    order) and `capped` (a stable rank by weight, unweighted hints last; the first 3 are kept in emission order).
- Unit tests:
  - `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`:
    - All verdicts updated.
    - New: familiar inside and outside the tie window; the verdict table and pivot cases (equal claim, equal truth)
      on both scales; profile anchors above and below; largest gap, then sheet order; no anchor; the cap on sorting
      and classification (profile before group/category) and across matching dimensions.
    - Wire shapes and refusals for `verdict`, `other`/`above`, `VERDICTS`.
  - `Q/🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs`: new `short_labels_hold_one_to_forty_code_points_in_each_language`.
  - `Q/🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs`: builders carry `short`/`familiar`. New
    `short_labels_reach_every_sheet_and_familiarity_stays_behind`.
  - `Q/🔨️modules/👁️views/🧪️tests/🔬️unit/🦀️.rs`: the reversed easy sorting is all `reversed` and capped; profile
    hint `a` is against anchor `b`, above.
  - `Q/🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs`: `SheetItem` literal gained `short`.
  - `Q/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`: the run-view literal uses verdicts, a reversed hint and an anchored profile
    hint; a sheet with `short` everywhere round-trips; a sheet item with `familiar` is refused.
- Proctor:
  - `P/🧫️fixtures/⚡️power/🔣️.json`: kettle `familiar: true`; `short` on the nuclear plant and the full-load-hours
    quantity.
  - `P/🧫️fixtures/🏠️homes/🔣️.json`: `short` on the cost axis, district heating and the air-to-water unit.
  - `P/🧪️tests/🌐️end-to-end/🦀️.rs`, challenges test: a reversed easy sorting gives exactly 3 `reversed` hints out of
    4 misses (the kettle ties with the laptop and comes first), in the learner's order.
  - `P/🧪️tests/🌐️end-to-end/🦀️.rs`, hints test:
    - Short labels reach the sheet, and `familiar` does not.
    - A misordered sorting picks the familiar kettle over the charger, which claims less.
    - Swapped matching cards give 4 misses → 3 hints: hours both `reversed`, plus the first capacity hint in sheet
      order (`under` or `over`).
    - Pellets as a heat pump: profile hint against condensing, `above: true`.
    - Air-to-water as a wood stove: against condensing, `above: false`.
    - Short label on an axis and an item of the classification sheet.
  - `P/README.md`: wire version 4, run-view hints (verdict, familiar, anchor, cap, short), fixtures, end-to-end
    description.

Created: this report and `hints2_sheet_item_short.py` (a temporary rewrite of `SheetItem` literals in three test files).
Removed: nothing.

## Gates run

All final runs are from the repo root.

- `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 cargo test -p semio-framework-quiz --no-fail-fast` → lib
  **190 passed, 0 failed**.
  - Before the vectors agent regenerated the fixtures, 3 vector-bound tests failed on `under` and on the old count of 4
    hints: `challenge::tests::shared_challenge_rules_of_the_python_numpy_reference_hold`,
    `views::tests::shared_learner_and_run_views_of_the_python_reference_hold` and
    `schema::tests::every_fixture_document_round_trips_through_the_twin`.
  - They pass against the regenerated fixtures. `⛰️challenge-rules` carries 78 `verdict` and 12 `above`, and no
    `under` remains. The Rust twin therefore agrees with the Python reference on every round-2 vector.
- `cargo clippy -p semio-framework-quiz --all-targets -- -D warnings` → exit 0.
- `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` →
  **90 + 15 + 21 passed**, 0 failed.
  - The first run failed one end-to-end assertion. The expert sheet axis now carries `short`, and the assertion still
    expected exactly `{id, label}`. I adapted it: only `id`, `label` and `short`, and `short` is checked.
- `cargo clippy --manifest-path "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/Cargo.toml" -p teaching-proctor --all-targets
  -- -D warnings` → exit 0, after `vec!` became an array in the end-to-end test. The only warnings come from
  dependency crates (`semio-framework-value`).

Not run: TypeScript, React and site gates (other agents own them).

## Deviations and decisions

- Before Round 2 landed I had read the verdict as `claim ≠ p && (claim ≥ p) ≠ (truth ≥ p)`. Round 2 fixed the
  symmetric sign rule: `claim > p ? truth ≤ p : claim < p && truth ≥ p`. I switched to it. A truth at the pivot with
  the claim off it is now `reversed` on both scales, and a claim at the pivot is never reversed. `verdict_of` is
  public, as in TypeScript.
- `capped` ranks with `partial_cmp(...).unwrap_or(Equal)` (descending), followed by a stable sort. A NaN weight
  therefore ties, as TypeScript's `>`/`<` comparator does.
- The Rust `ProfileHint` does not enforce JSON's `dependencies` (`other` ⇔ `above`). Serde has no such constraint.
  `hints_of` always sets both or neither.
- Profile anchors are only items whose own category carries the axis. Items in categories without a profile never
  qualify, the same as in TypeScript.

## Notes for the next agent

- Rust exports: `HINTS_PER_TASK`, `verdict_of`, `Verdict`, `VERDICTS`, `ShortText`, `SHORT_LENGTH`.
- Proctor fixture: kettle is familiar. Shorts: `nuclear-plant` (Nuclear/AKW), `hours` quantity (hours/Stunden), `cost`
  axis (cost/Kosten), `district-heating` (district heat/Fernwärme), `air-to-water` (Air unit/Luftgerät). Client or site
  tests that read these fixtures see them.
