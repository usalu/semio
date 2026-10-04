# Report — specific hints (design §8): Rust twin and proctor

Agent: Rust twin + proctor of the 2026-10-04 revision ("specific hints"). Normative: `📓️design.md` §8.

## Changed files

Updated (Q = `🧰️framework/🛍️products/❓️quiz`, P = `🎓️teaching/🛂️proctor`):

- `Q/🧬️schema/🦀️.rs` — `WIRE_VERSION` 3; `Quantity.additive: bool` (after `prefixed`); new `CompareHint { item, other, dimension?, factor?, difference?, under }`, `ProfileHint { item, category, axis }`, `GroupHint { item, other, together }`, `CategoryHint { item, category }`; `Hint` = `Compare | Profile | Group | Category` (tag `kind`, kebab-case). Removed `HintDirection`, `MagnitudeHint`, `MisplacedHint`. `Hint` is no longer `Eq` (it carries `f64`).
- `Q/🔨️modules/⛰️challenge/🦀️.rs` — `hints_of` per §8.3 with private helpers `Keyed`, `relation`, `compare_hint`, `profile_axis`.
- `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs` — hint tests rewritten: most-wrong anchor, anchor preference over a more wrong miss, ties first in sheet order, no anchor, lone key (empty pool), `under` both ways on both scales, factor < 1, linear difference, matching per dimension, profile axis choice/tie/near miss/axis without spread/reach over presented categories, a category without a profile on either side, group together/apart, category fallback, unknown category, wire shapes of all four kinds and refusals of the removed ones.
- `Q/🔨️modules/👁️views/🧪️tests/🔬️unit/🦀️.rs` — easy run view asserts compare and profile hints.
- `Q/🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs` — `quantity()` builder gains `additive: false`.
- `Q/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` — `additive` in quantity literals; run view literal carries the new hint kinds.
- `Q/🧪️tests/🧬️schema-conformance/🦀️.rs` — the union map names the four new hint `$defs` (the vectors agent later added `Quantity` to the decoded list).
- `P/🧫️fixtures/⚡️power/🔣️.json` — `additive` (power, capacity `true`; full-load hours `false`).
- `P/🧫️fixtures/🏠️homes/🔣️.json` — new category `district-heating` with a description and no profile (no item belongs to it), so the end-to-end test reaches the group and category hints.
- `P/🔨️modules/📚️catalog/🧪️tests/🔬️unit/🦀️.rs` — `additive` in the quiz literal.
- `P/🧪️tests/🌐️end-to-end/🦀️.rs` — easy sorting: reversed answer → `compare` heat pump vs phone charger, factor 5/3000, `under: false`; renamed `an_easy_run_questions_far_off_cards_and_misplaced_items_until_it_closes`: matching `compare` with `dimension` (capacity: factor 300 `under: true`, 140 000 `under: false`; hours: difference −1050 and 5800), classification `profile` (efficiency, incl. a tie), `category` (district heating), `group` together and apart; submitted/voided runs still carry no hints.
- `P/README.md` — wire version 3, run-view hints, end-to-end and fixture descriptions.

Created: this report. Removed: nothing.

## Gates run

- `SEMIO_TEST_BUDGET_MS=900000 cargo test -p semio-framework-quiz --no-fail-fast` → lib **183 passed, 0 failed** (doc-tests 0). Earlier runs, before the vectors agent regenerated `🧫️fixtures`, failed 12 vector-driven tests on `missing field additive` only. After the regeneration all pass, including `challenge::tests::shared_challenge_rules_of_the_python_numpy_reference_hold`. Its fixture holds 66 compare, 15 profile, 10 group and 18 category hint expectations, so the Rust twin matches the Python reference.
- `cargo clippy -p semio-framework-quiz --all-targets -- -D warnings` → clean (exit 0).
- `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test` → fresh compile, **90 + 15 + 21 passed**, 0 failed.
- `cargo clippy --manifest-path "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/Cargo.toml" -p teaching-proctor --all-targets -- -D warnings` → exit 0. The only warnings come from dependency crates (`semio-framework-value`). Plain `-p teaching-proctor` from the repo root does not resolve the package; it needs `--manifest-path`.

Red or not run: nothing red. I ran no TypeScript, React or site gates; other agents own those.

## Deviations and decisions

All readings match the TypeScript twin as recorded in `📓️hints-schema-landed.md`:
- Sorting: keyed items in sheet order, key = `keys[first index of the item in answer.order]`. Hints come in `answer.order`, and only for items that are keyed sheet items.
- Error: logarithmic `ρ > τ ? ρ/τ : τ/ρ`, linear `|δ − Δ|`. Strict `>` keeps the first on ties. `under = claim ≥ pivot ? truth > claim : truth < claim`. `factor`/`difference` = `k_X / k_R` / `k_X − k_R`, not rounded.
- Classification: an assignment to a category the task does not know gives no hint. When both categories carry profiles, only the profile rule applies, and a near miss gives nothing. Profile reach per axis = `(max − min) / 2` over the task profiles of the sheet's categories that carry the axis. An axis is skipped unless its reach is > 0. Axis order is the task's.
- Group `together: false`: the first other assigned item of the same own category whose assignment differs from the item's ("elsewhere" = elsewhere than the item).
- Known twin seam: the profile spread uses `f64::max/min` (NaN ignored), while TS uses `Math.max/min` (NaN propagates). Validated profiles are finite, so the two never differ in practice.
- Design observation: when every anchor's key is exactly right, the error `v_X / k_X` is mathematically the same for all anchors, so "first in sheet order" can be decided by rounding. The rounding is still bit-identical in the twins, because they use the same operations.
- Docstring emojis: `ProfileHint` 🔺️ and `CategoryHint` ❔️, kept unique within `🧬️schema/🦀️.rs`.

## Notes for the next agent

- New Rust types re-exported via `crate::schema`: `CompareHint`, `ProfileHint`, `GroupHint`, `CategoryHint`. `pub fn hints_of(task: &Task, sheet_task: &SheetTask, answer: Option<&Answer>) -> Vec<Hint>` keeps its signature.
- Proctor fixture `homes` now has 4 categories: `district-heating` has no profile and a description. Any client/site test that reads this fixture sees it.
- I deleted my tool output under `🗑️generated` (`hints-*`).
