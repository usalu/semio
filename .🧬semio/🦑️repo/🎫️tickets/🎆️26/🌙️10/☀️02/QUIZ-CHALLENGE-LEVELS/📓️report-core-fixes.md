# Report: core fixes after the audit (reach and misses as ratios, acted without the decider's clock)

Agent: core fixes. `Q` = `🧰️framework/🛍️products/❓️quiz`, `T` = this ticket folder. This report covers design §1 (revised 2026-10-03), §3.1 and §3.5, and `📓️audit-core.md` B1, B2 and P1.6.

## Outcome

- **Fix 1 (B1).** On a logarithmic scale, `reach` is now a factor: `hi > lo ? min(1000, sqrt(hi / lo)) : 1000`. On a linear scale it is a distance: `hi > lo ? (hi − lo) / 2 : +∞`.
  - `misses` tests `max / min > reach` on a logarithmic scale and `|v − t| > reach` on a linear one.
  - `REACH_DECADES` is renamed `REACH_FACTOR = 1000` in both twins, the three Python references, the tests and the README.
  - The bounds are explicit comparisons in both twins, so a `NaN` takes no part, as before.
- **Fix 2 (B2).** `acted(at, floor) = max(at, floor)` in both twins and in the Python lifecycle reference.
  - `open-task` uses the floor `run.startedAt`.
  - `record-answer` uses `opened[task]` on a timed run, and `run.startedAt` otherwise.
  - `now` no longer enters any time verdict.
- **P1.6, TS validation of `at`.** `commandRejection` now also refuses a command that Rust cannot decode, with `id-invalid`, the code it already uses for malformed commands. This covers two cases:
  - `start-run` whose `challenge` is none of the four, or missing;
  - `open-task` or `record-answer` whose `at` is not a safe non-negative integer (fractional, negative, above 2^53−1, non-number, or missing).

  No new rejection code was added. See decision 2.

## Changed files (all updated, none created in the product, none removed)

- **Core:**
  - `Q/🔨️modules/⛰️challenge/{🟦️.ts,🦀️.rs}` (the TS module no longer imports `scaled`, which removes the scoring↔challenge import cycle);
  - `Q/🔨️modules/🧾️lifecycle/{🟦️.ts,🦀️.rs}`;
  - `Q/🔨️modules/✅️validation/🟦️.ts`.
- **Unit tests:**
  - `Q/🧪️tests/🧗️challenge-ladder/🟦️.ts`: mathjs oracle (`sqrt`, `divide`, `max`/`min`); exact ×1000 for truths 1…2000 in both directions; one double beyond; the root of the ratio met exactly; `0.018` vs `18`; `acted`.
  - `Q/🧪️tests/⚖️partial-credit-scoring/🟦️.ts`: the oracle.
  - `Q/🧪️tests/🔁️run-lifecycle/🟦️.ts`:
    - instants that are independent of `now`;
    - a new test for a device clock that is ahead or behind, delivered at once or late;
    - a new test for `commandRejection` of a bad `at` or `challenge`.
  - `Q/🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`: one new test.
  - `Q/🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs`: every instant is checked at several `now` values.
  - `Q/🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs`: one message.
- **Shared case `⛰️challenge-rules`:**
  - `🐍️.py`: an independent rule implemented with numpy `sqrt`/`maximum`/`minimum`. A `Fraction` check proves every logarithmic reach below the cap is the correctly rounded `sqrt(hi/lo)`. `acted` is checked by `numpy.maximum`.
  - The `🟦️.ts` and `🦀️.rs` bindings drop `now` from `instants`.
  - `🥒️.feature` text.
- **Other Python references:** `📏️sorting-concordance/🐍️.py`, `🔀️matching-concordance/🐍️.py` (numpy `sqrt`, ratio `maximum/minimum`) and `🧾️learner-lifecycle/🐍️.py`. Their features changed too: `📏️`, `🔀️`, `🧾️` `🥒️.feature`.
- `Q/README.md` (small hunks): reach and miss, the clock, lifecycle rows, the malformed-command paragraph, the deputy paragraph and the Rust names.
- `Q/🔮️oracles/🔣️.json`: the numpy role.
- **Generator** `…/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`:
  - `import math`;
  - the boundary vectors listed below;
  - a reworked `the-clock` sequence;
  - new sequences.
- **Fixtures regenerated:** `⛰️challenge-rules`, `📏️sorting-concordance`, `🔀️matching-concordance`, `🧾️learner-lifecycle`.
- **Ticket:** `T/ratio_boundary_probe.ts`, a probe that compares the quotient, product and `log10` forms on typed decimals. Kept as an input.

## New vectors

- `⛰️challenge-rules` (now 16 reaches, 34 misses, 8 instants):
  - **Reaches:** the root of 4; an irrational root; just under and just over the cap.
  - **×1000 and ÷1000 boundaries:**
    - `18000` vs `18` (in reach), plus one double beyond;
    - `18` vs `18000` (in reach), plus one double beyond;
    - `0.018` vs `18` (beyond, see the open problem);
    - `22200` vs `22.2`;
    - `29000` vs `29` with no spread, plus one double beyond.
  - **Square-root reach:** `36` exactly at `sqrt(72/18)` from both sides, each with one double beyond. Irrational roots in reach and beyond.
  - **Linear:** a non-dyadic distance at the reach and one double beyond.
  - **Instants:** now `{at, floor}` only; the floor is crossed from both sides; the largest safe integer.
- `📏️ guessed` +3: ×1000 of the largest in reach, one double beyond, and a typed thousandth of the smallest.
- `🔀️ guessed` +2: ×1000/÷1000 with no spread, and one double beyond.
- `🧾️ learners` +7 sequences:
  - `the-limit-of-a-sorting-and-a-matching`: at the limit, 1 ms past, and 1 ms past while the decider's clock is still before the limit.
  - `clock-{ahead,behind,far-behind}-{at-once,late}`, with skew ±20 s or −120 s and delay 0 or 150 s per answer. The generator asserts that all six give identical verdicts. `far-behind` exercises the `acted` floor (the opening is raised to the run's start).
- `the-clock`: a device instant far ahead of the decider's clock is now kept (`opened = ahead`, an untimed answer at `start + 10⁹`), and every rejection is unchanged.

## Gates (exact commands, observed)

| Gate | Command | Result |
|---|---|---|
| Fixtures | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` (repo root; second full run) | 12 of 12 `unchanged` (every generator assertion passes) |
| TS core | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=300000 bun ./📜️script.ts test` | 11 files, **410 passed**, 0 failed |
| Typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.core.json` | **0 errors** |
| Rust core | `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | **176 passed**, 0 failed (doc-tests 0) |
| Parity | in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=14 executed=129 passed=129 failed=0 errored=0 **parity=129/129** |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | clean=true errors=0 warnings=0 |

- The site catalog already carried 6 `"challenge"` entries when I regenerated. The final full regeneration was `unchanged`, so the committed lifecycle vectors match the current catalog.
- Not run: the proctor, React, site and e2e. The proctor does not call `acted` or `REACH_DECADES`; `git grep` over the repo finds no use outside `Q`.

## Open problem for the coordinator: typed decimals at ÷1000 (design §1 claim)

The quotient test is exact for doubles, but a learner's decimal input is not one: the double `0.018` lies just below `18/1000`, and `18 / 0.018 = 1000.0000000000001`. So `0.018` **misses** `18`.

`T/ratio_boundary_probe.ts` counts misses over typed-exact ×1000 and ÷1000 pairs (truths 1…2000 with 0 to 3 decimals, 16 000 pairs):

| Form | Misses at ×1000 | Misses at ÷1000 |
|---|---|---|
| quotient (now normative) | 788 | 1431 |
| product `max > reach × min` | 40 | 710 |
| old `log10` | 1066 | 45 |

Integer truths ×1000 never miss (0 of 2000), but integer truths ÷1000 miss in 233 of 2000 cases. This exceeds the old `log10` rate (9) in that direction.

I implemented the normative formula exactly. The vector `log-typed-thousandth-of-eighteen-lies-beyond` pins `miss: true` and states why.

If the owner's "a factor of more than 1000" should hold for typed decimals, the clean fix is a relative tolerance, e.g. `ratio > reach × (1 + 2⁻³⁰)`. It is still IEEE basic operations, so it stays bit-equal in all twins. Changing it touches `misses` in 2 twins and 4 Python references, plus regeneration; the vector names already describe the boundary.

## Deviations and decisions

1. `reach` keeps its signature and returns a factor or a distance. The React client already formats `within` as `×${formatFactor(within)}`, so a factor is what it expects. Its old "×3" would have been wrong.
2. **TS `at`/`challenge` validation.** I reused `id-invalid` through `commandRejection`, the TS core's existing path for malformed commands, instead of throwing or adding a code. This is consistent with design §3.3 ("no rejection code is added"). Coordinator: §3.3 and the README now say that the TS core refuses such a command as `id-invalid`, while the proctor refuses it with `command-malformed` before any decision. Both refuse, so they never disagree on acceptance.
   - **Residual:** Rust `u64` accepts `at` in (2⁵³, 2⁶⁴), which TS refuses. A device never stamps such a value. A schema `maximum` of 2⁵³−1 would close the gap but changes the contract, so I left it.
3. The `instants` vectors no longer carry `now`, and the TS/Rust bindings changed accordingly. This is greenfield with no compatibility, so nothing else had to follow.
4. A Python reach is compared bit for bit with numpy (it was within 1e-12 before), because only exactly rounded operations remain.
5. Python `json` writes one-ulp values with 17 significant digits, and Rust `serde_json` (without `float_roundtrip`) parsed every one of them correctly. The Rust fixture replay passed.
   - Note for the proctor: the workspace `serde_json` lacks `float_roundtrip`. A guess of 17 digits could in principle parse 1 ulp off in Rust, which matters now that the boundary is exact.
   - Typed guesses with ≤15 digits take the exact fast path.
   - I did not change the workspace `Cargo.toml`.

## Other audit findings (no core change made; for the coordinator)

- **Client (not mine):**
  - The deputy rebuilds `recorded` as `Object.keys(answers).length`, while the proctor counts every answer. This affects `answers-exhausted`.
  - A run known only from a `RunSummary` has `opened = {}` in the deputy, which then gives `task-unopened` where the proctor knows the opening.
  - An accepted `open-task` with no events must count as settled.
- **P1.5:** Rust reads `"guesses": null` as absent, while TS rejects it (`answer-invalid`). The deputy is stricter, which is harmless; no client sends null. Not changed.
- **P1.4:** `Math.max` vs `f64::max` on NaN in the scoring of profiles is unreachable through validated quizzes. `reach` now uses explicit comparisons in both twins.
- **P1.2/P1.3:** `300` vs `300.0` and integer-like slug key order are text-level only; comparison is structural. Not changed.
- **Python nit:** the lifecycle `learner_view` sorts by `startedAt` with `reverse=True` (stable), while the twins reverse the fold order. They differ only for equal `startedAt`, which the vectors never produce. Not changed.
- **Crowd nuance:** an expert sorting with an order but no guess adds only its score bin. This should be documented in design §3.6 (the coordinator's file).

## Notes for the next agent

- Changed API:
  - TS: `REACH_FACTOR` (was `REACH_DECADES`) and `acted(at, floor)`.
  - Rust: `REACH_FACTOR: f64` and `acted(at: Timestamp, floor: Timestamp) -> Timestamp`.
  - `reach` and `misses` keep their signatures. The meaning of `reach` on a logarithmic scale is now a factor ≥ 1.
- The proctor compiles against everything else unchanged.
