# 📓️ Work package D2 (second round) — Rust twins: clearance and gesture

Ticket `2026/10/02/QUIZ-PETS`, second round, phase D. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Everything below was run on 2026-10-03 between 14:00 and 15:50 on the Windows host (`rustc 1.99.0-nightly (c4af71034 2026-07-06)`, bun 1.4.2). Raw outputs: `TK/🗑️generated/d2/` (`logs/`, `*-report.txt`, `mutants.txt`, `rehearsal-harness.txt`; build directories and the 76 MB bit dump were removed, `bun TK/d2_dump_bits.ts` regenerates it byte for byte).

**State: final.** Both twins are mounted in the real crate, their suites are green, both cases are at full parity, and the bit-exactness tools find 0 mismatches. The TypeScript sources ported from (`🚧️clearance/🟦️.ts` 12:23, `👆️gesture/🟦️.ts` 01:35, the clearance unit suite with the reference world 00:57, both TypeScript adapters) are copied to `TK/🗑️generated/d2/ported-from/`; at 15:49 `cmp` finds all five identical to the repository files (B4 had not changed clearance).

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🚧️clearance/🦀️.rs` (735 lines) | twin of `🟦️.ts`: all 13 constants (`MARGIN` … `SCOOT_HASTE`), `UPRIGHT`, `TALLY`; `Body`, `Posture`, `Pair`, `Seat`, `Seating`, `Slide`, `Tally`; all 39 functions under their snake_case names (`body_of`, `slot_in`, `claim_of`, `claim_clear`, `guarded_stride`, `seat_of`, `head_under`, `slide_of`, `pushed_out`, `must_poof`, `evicted`, `tallied`, …), plus the private `level`, `carved`, `spans_of`, `collide`, `ranked`. Unit suite attached `pub(crate)` (the scratch world check reaches its world) |
| `P/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🦀️.rs` | 64 tests: every case of the TypeScript suite, the brute-force comparisons on the same generated cases (same 32-bit generator), and **the reference world** (rules as switches, walkers, hops, glides, falls with heads, steering and parachutes, the hand, surveys, seating, scoots, poofs, tallies). Fundamental: 2 worlds × 1 500 ticks + slices of four + the shared-area judge + purity + every core rule's bite + the poof and quality ablations, all asserting 0 overlaps; `quick`: 6 × 5 000 …; `exhaustive`: the research prototype's 24 × 30 000 ticks, 12-seed ablation sweeps |
| `P/🔨️modules/👆️gesture/🦀️.rs` (730 lines) | the complete twin, replacing R0's partial one (its `IDLE`, `COLD`, `no_*` kept as they were): `Guards`, `UNGUARDED`, `PressInput`, `PressSignal`, `PressStep`, `Caress`, `CircleStep`, `StrokeStep`, `ShakeStep`, `HoverStep`, all 40 thresholds, `slop_of`, `press_step`, `press_due`, `heat_at`, `heat_after`, `tier_of`, `warmth_after`, `circle_step`, `stroke_step`, `shake_step`, `hover_step`, `hover_busy` |
| `P/🔨️modules/👆️gesture/🧪️tests/🔬️unit/🦀️.rs` | 40 tests: every case of the TypeScript suite (shapes drawn with `sin_turns`/`cos_turns`), the committed presses, histories, circles, pettings, held paths, floors, travel (sample; 4 variants at `quick`, all 16 at `exhaustive`), the variant involutions, a source scan |
| `P/🧪️tests/🚧️clearance-proof/🦀️.rs` | Rust subject adapter, 11 scenarios, the TypeScript adapter's ids and projections |
| `P/🧪️tests/👆️gesture-recognition/🦀️.rs` | Rust subject adapter, 9 scenarios, the TypeScript adapter's ids and projections, the same refusals (`agreed`, mirror ticks, wrong gestures, silent travel) |
| `P/📦️packages/🦀️rust/🦀️.rs` | one mount `pub mod clearance;` after `gesture` (anchored edit, re-read before) |
| `P/🦀️.rs` | `pub use crate::clearance::*;` and one docstring line naming the bodies/claims and the hand's gestures (anchored edits) |
| `TK/d2_scratch.ts` | `prepare` (private scratch crate `TK/🗑️generated/d2/crate`), `mutants [name…]` |
| `TK/d2_dump_bits.ts`, `TK/d2_check_bits.rs`, `TK/d2_check_world.rs` | the bit-exactness tool pair and the world check (§3) |
| `TK/d2_rehearse_adapters.ts` | the harness's Rust projections against the TypeScript adapters without tolerance |

No TypeScript file, no fixture, no feature, no oracle and no sibling module was touched.

## 2. The API as landed (for the stage port)

```rust
// crate::clearance (flat through the façade)
pub fn body_of(owner: &str, feet: Point, size: Size, hover: f64, posture: Posture, margin: f64) -> Body;
pub fn obstacles_of(owner: &str, bodies: &[Body], claims: &[Claim], tick: Ticks) -> Vec<Extent>;
pub fn slot_in(x: f64, extent: Extent, low: f64, high: f64, obstacles: &[Extent]) -> Option<f64>;   // spot_on, column_over take &Perch
pub fn claim_of(owner: &str, from: Ticks, extents: &[Extent], span: Ticks, rest: Option<Extent>) -> Claim;
pub fn claim_clear(claim: &Claim, bodies: &[Body], claims: &[Claim]) -> bool;
pub fn order_kept(before: &[Slug], after: &[Slug]) -> bool;
pub fn seat_of(bodies: &[Body], perch: &Perch, margin: f64) -> Seating;
pub fn head_under<'a>(before: Extent, after: Extent, owner: &str, bodies: &'a [Body]) -> Option<&'a Body>;
pub fn slide_side(rider: Extent, host: Extent) -> Facing;
pub fn slide_of(rider: Extent, side: Facing, ticks: Ticks, low: f64, high: f64, obstacles: &[Extent]) -> Slide;   // Slide { stride, side: Facing }
pub fn pushed_out(extent: Extent, obstacles: &[Extent], iterations: usize) -> Option<Point>;
pub fn tallied(tally: Tally, bodies: &[Body], margin: f64, poofs: u64, waits: u64) -> Tally;   // Tally of u64; per_million(u64, u64)
pub const FOREVER: Ticks; PATIENCE: Ticks; DELAY: Ticks; PUSHES: usize; STEERINGS: [f64; 13];
// crate::gesture
pub fn press_step(press: Press, input: PressInput, tick: Ticks, guards: Guards) -> PressStep;   // PressInput::{Pressed(Pressed), Dragged(Dragged), Released(Released), Cancelled, Ticked}
pub fn press_due(press: Press) -> Option<Ticks>;
pub fn warmth_after(warmth: Warmth, tick: Ticks, caress: Caress) -> Warmth;
pub fn circle_step(circling: Circling, pointer: Point, body: Rect, tick: Ticks, guards: Guards) -> CircleStep;   // cue: Option<Cue>
pub fn hover_step(hover: Hover, pointer: Point, body: Rect, tick: Ticks, guards: Guards) -> HoverStep;
pub fn shake_step(shaking: Shaking, grip: Point, height: f64, tick: Ticks, guards: Guards) -> ShakeStep;
pub fn hover_busy(hover: &Hover) -> bool;
```

- `Extent`, `Slice`, `Claim` are the schema's (as in TypeScript, where clearance imports them); boxes and other `Copy` values travel by value, lists by slice. `null` ≙ `None`; a host is a reference into the list it was found in (`std::ptr::eq` gives its index).
- JavaScript's `Math.min`/`Math.max` are terrain's `larger`/`smaller` everywhere a float is compared (NaN and ±0 as JavaScript answers); whole ticks use `std::cmp::max`. A unit test of each twin scans its source for `.max(`, `.min(`, `f64::max`, `.mul_add(`, transcendentals, maps, statics and prints.
- `!(a < b && …)` guards of the TypeScript are bound to positive names (`ordered`, `level`, `ahead`, `crossing`, `across`, `inside`, `moving`) and negated, which keeps the NaN branches of the twin without clippy's `neg_cmp_op_on_partial_ord`.
- `CIRCLE_ROUND * CIRCLE_ROUND * near`, `span * 64 * 64`, `(length * 64) / max(Δ, 1)` and every other product or quotient are written in the twin's order; `speed >= SLOW && speed <= FAST` is `(SLOW..=FAST).contains(&speed)` (the same two comparisons, clippy's `manual_range_contains`).

## 3. Bit-exactness

### 3.1 Every exported function (`TK/d2_dump_bits.ts` → `TK/d2_check_bits.rs`)

The TypeScript half evaluates every export of both modules and writes arguments and results as sixteen hexadecimal digits of their IEEE-754 patterns; the Rust half decodes the same flat layouts, recomputes with the twins and compares every pattern. Inputs: quarter-pixel lattices (so touching boxes, exact fits and ties happen), arbitrary doubles, ±0, NaN, ±∞, subnormals for the plain-number functions, generated crowds, claims (cut by `claim_of` and raw), seatings, trajectories of every recogniser whose inputs are the previous outputs (drawn circles, pettings, shakes, random walks and straight passes sampled like 30–144 Hz devices, guards switched on now and then), fuzzed recogniser states, and targeted edges: signed zeros through the extremes, symmetric pushes (left = right, up = down), band edges `inner`/`outer` ± 1 ulp, antipodal quarter turns with a cross product near 0, lap completions at `far = 6.25 · near` ± 1 ulp, stroke speeds exactly 60 and 900 px/s ± 1 ulp, hysteresis edges, drags exactly on the slop circle.

| Function | Cases | Mismatches | Function | Cases | Mismatches |
|---|---|---|---|---|---|
| `circle_step` | 17 953 | 0 | `stroke_step` | 18 353 | 0 |
| `shake_step` | 11 353 | 0 | `hover_step`, `hover_busy` | 5 156 each | 0 |
| `press_step` | 5 400 | 0 | `warmth_after` | 4 800 | 0 |
| `heat_at`, `heat_after`, `tier_of` | 1 500 each | 0 | `press_due`, `slop_of`, `no_*`, constants | 500, 3, 4 × 50, 1 | 0 |
| `slot_in`, `spot_on`, `column_over` | 1 518, 1 500, 1 500 | 0 | `claim_clear`, `claim_of`, `slice_at` | 1 500, 700, 800 | 0 |
| `guarded_stride`, `pushed_out`, `seat_of` | 1 500, 1 800, 1 500 | 0 | `head_under`, `lift_onto`, `rests_on`, `slide_side`, `slide_of` | 1 500 each | 0 |
| `overlaps`, `near_misses`, `order_of`, `order_kept`, `released`, `pruned` | 500 each | 0 | `body_of`, `united`, `meets`, `must_poof`, `evicted`, `scoot_fraction`, `scooted` | 800 each | 0 |
| `leaning`, `canopied`, `obstacles_of`, `free_at`, `free_among`, `vaults` | 600 each | 0 | `shifted`, `grown`, `tallied`, `slide_stride`, `lane_lift`, `per_million` | 400, 400, 400, 400, 300, 300 | 0 |
| **total** | **107 594 cases, 1 153 294 result numbers** | **0** | | | |

### 3.2 The committed gesture corpus (same tool, `corpus.json`)

Every committed trace replayed exactly as the subject adapters replay it, the full cue (or signal) sequence compared, and an FNV-1a digest (32-bit words) of the recogniser's complete state after every tick:

| Corpus part | Replays | Cues / signals | State digests | Mismatches |
|---|---|---|---|---|
| presses | 62 | 104 | 62 | 0 |
| histories of attention | 13 | — | 13 | 0 |
| circles × 16 variants | 832 | 712 | 832 | 0 |
| pettings × 16 variants | 448 | 784 | 448 | 0 |
| held paths × 16 variants | 640 | 336 | 640 | 0 |
| travel: 18 stretches × 16 variants × 40 bodies | 11 520 | 0 | 1 440 (variants 0 and 9) | 0 |
| **total** | **13 515** | **1 936** | **3 435** | **0** |

### 3.3 The reference world (`TK/d2_check_world.rs`)

`d2_dump_bits.ts` runs the reference world of the TypeScript unit suite (imported through a Vitest stand-in) and records a digest of the mode, the feet and the box of every pet after every tick plus the final tally and counts; the check (a unit test module of the scratch library, so it reaches `crate::clearance::tests`) replays every run with the Rust world: 12 seeds × 5 000 ticks with every rule, 4 × 5 000 with slices of four, each of the ten rules off (3 × 3 000), heads and steering off, and heads, steering and poof off. **52 runs, 188 000 ticks, 0 mismatches** (every per-tick digest, tally and count equal). With every rule in force the TypeScript world has 0 overlap ticks in those runs, and so does the Rust world.

### 3.4 The tools can fail (`bun TK/d2_scratch.ts mutants`)

Twenty-one deliberately broken copies of the twins, each run against the bit check and the world check: **21 mutants, 0 survived** (`TK/🗑️generated/d2/mutants.txt`).

| Mutant | Cases that differ | World runs that differ |
|---|---|---|
| native `f64::min/max` in `united` / in `slot_in` | 59 / 3 | 0 / 0 |
| `meets` not strict on one edge | 57 | 1 |
| reassociated carve bounds of `slot_in` | 279 | 14 |
| seat mean by reciprocal / reassociated seat shift | 18 / 94 | 0 / 0 |
| reassociated `scooted` | 167 | 30 |
| `pushed_out` ties broken right-first | 272 | 33 |
| `rests_on` tolerance strict | 101 | 0 |
| `-reach` for `0 - reach` in `guarded_stride` | 235 | — |
| reassociated `lift_onto` | 1 | 0 |
| fused radius / fused cross product of `circle_step` | 1 456 / 346 | — |
| reassociated roundness / outer band | 50 / 35 | — |
| reassociated stroke speed / stroke hysteresis | 18 / 60 | — |
| heat leak one ulp off | 963 | — |
| fused shake span / fused slop distance | 3 / 41 | — |
| native extremes in `stroke_step` | 95 | — |

Two earlier mutants were equivalent and were replaced, recorded here because they say something about the arithmetic: `(r + 16) / 2` for `r / 2 + 8` (halving is exact, so both round the same real number on the same grid) and `-smaller(…)` for `0 - smaller(…)` in `slide_of` (`guarded_stride` turns a −0 stride into +0 before it leaves). The edge cases of §3.1 were added after a first run in which signed-zero, tie and one-ulp-boundary mutants survived the random inputs.

### 3.5 The adapters, exactly (`bun TK/d2_rehearse_adapters.ts`)

The Rust projections the harness recorded in the last parity runs against what the TypeScript adapters answer now, numbers by `Object.is` after the JSON round trip: **2 cases, 20 scenarios, 10 402 numbers, 0 differences** (and all strings, keys and lengths equal).

## 4. Commands and real results

| # | Command (repository root unless `cd`) | Result |
|---|---|---|
| 1 | `bun TK/d2_scratch.ts prepare` then `bash TK/rust_scratch.sh d2 clippy --offline --all-targets --features sut -- -D warnings` | exit 0 (twins, both suites, both adapters as host examples, `d2_bits`, `world_check`) |
| 2 | `bash TK/rust_scratch.sh d2 test --release --offline --lib -- clearance:: gesture::` (all levels) | `test result: ok. 104 passed; 0 failed` (fundamental, `quick` and `exhaustive`, the 24 × 30 000-tick world included, 36 s in release; the same in a debug run earlier: 0 failures) |
| 3 | `bun TK/d2_dump_bits.ts` then `bash TK/rust_scratch.sh d2 test --release --offline --test d2_bits -- --nocapture` | `total: 107594 cases, 1153294 result numbers, 0 mismatches`; `corpus: 13515 replays, 0 mismatches`; `2 passed` |
| 4 | `bash TK/rust_scratch.sh d2 test --release --offline --lib world_check -- --nocapture` | `world: 52 runs, 188000 ticks, 0 mismatches` |
| 5 | `bun TK/d2_scratch.ts mutants` | `21 mutants, 0 survived` |
| 6 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test --skip-nx-cache` (15:46) | exit 1: `425 passed; 2 failed; 15 filtered out`. All 96 fundamental tests of clearance and gesture pass. The two failures are not D2's: `stage::tests::the_calm_home_replays_into_its_committed_trace` (the stage replay, waiting for the stage port) and `behavior::tests::the_graph_is_the_committed_graph` (a sibling's in-progress activity graph — `walk` now leads to `aim` and `climb` — against the committed fixture). An earlier run (15:08) had `climbing::tests::committed_wall_climbing_…` failing instead of the behaviour test (D1 in progress) |
| 7 | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0 (15:47, after the last edit) |
| 8 | the same with `--target wasm32-unknown-unknown` and `--target wasm32-wasip2` (library) | exit 0 both |
| 9 | `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets` (and both wasm targets) | **not runnable**: exit 1 before anything is built, `[verify rust-warnings] native misses semio-hub-dag.` — the command's own scope fixture requires a package the root `Cargo.toml` does not list (repository-wide state, not pets). Rows 7–8 stand in for it |
| 10 | `cd TEST && bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🚧️clearance-proof"` | `[test] level=exhaustive cases=1 executed=33 passed=33 failed=0 errored=0 parity=33/33` (rerun after the last adapter edit: the same) |
| 11 | same, `--case "👆️gesture-recognition"` | `[test] level=exhaustive cases=1 executed=27 passed=27 failed=0 errored=0 parity=27/27` (rerun after the last adapter edit: the same; `travel-hours` included) |
| 12 | `bun TK/d2_rehearse_adapters.ts` | `harness: 2 cases, 20 scenarios, 10402 numbers, 0 differences` |
| 13 | `rustfmt --check --edition 2021 --config-path rustfmt.toml` over the two twins (with their suites), both adapters, both ticket Rust tools | exit 0 (the formatter's layout applied by hand with Edit) |
| 14 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py` over the six product files, glue, façade and both ticket Rust tools | `0 finding(s) in 10 file(s)`; the three TypeScript tools have no repeated docstring emoji either |
| 15 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |
| 16 | `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with the repository-wide backlog (`6257 high-priority breach(es) across 5 rule(s)`); the only pets line is `🔮️oracles/🔣️.json oracles[9] pets-react-user-event` (an engine family pattern, not D2's); the `👆️gesture` lines belong to `🧰️framework/🔨️modules/🕹️interaction` |
| 17 | `cmp TK/🗑️generated/d2/ported-from/* <repository files>` (15:49) | all five identical |

## 5. Decisions and deviations

1. **Types say what the TypeScript twin floors or bounds.** `claim_of` takes the span in whole `Ticks` (the TypeScript twin floors it; the adapter floors the committed `2.75` with `.floor()` before it asks, exactly `Math.max(Math.floor(span), 1)` because `claim_of` applies `max(span, 1)`); `pushed_out` takes its rounds as `usize`; a slide's side is the schema's `Facing` (R0: "sides are `Facing`") and the adapter projects `side.sign()`; `Tally` counts in `u64`. `FOREVER`, `PATIENCE`, `DELAY` are `Ticks`, `PUSHES` is `usize`; the integer thresholds of gesture (`HOLD_TICKS`, `CIRCLE_QUARTERS`, …) are `Ticks`/`i64`, the others `f64` — the constants scenario compares them as numbers, so `4` and `4.0` agree.
2. **`PressInput` reuses the schema's event structs** (`Pressed`, `Dragged`, `Released`) plus `Cancelled` and `Ticked`; `PressSignal` and `Caress` are serde enums (kebab-case), so the adapters project `"click"`, `"hold"` without a string table.
3. **`order_kept` keeps the TypeScript `-1` as `Option<usize>`**; `head_under` answers a reference into its list; `released`/`pruned` clone the claims they keep.
4. **The reference world lives in the Rust unit suite**, as in TypeScript, and is the TypeScript world statement for statement (pets addressed by position instead of by object identity; the per-perch orders as a list of pairs, because a struct named `World` may not hold a map). It is `pub(crate)` with the suite attached `pub(crate)`, so the scratch world check can replay it; nothing in the real crate uses it outside tests.
5. **Shapes in the gesture suite come from `sin_turns`/`cos_turns`**, not `f64::sin`, so the suite is the same on every platform; the TypeScript suite's `1.5 ** (tick / 64)` spirals are multiplied up tick by tick (the laws do not depend on the last bits).
6. **The replay helpers exist twice in Rust** (gesture unit suite and adapter): the TypeScript suite imports them from its adapter, but a Rust unit test cannot see the harness host, and a shared file beside the adapter would be a new, unregistered taxonomy entry.
7. **The gesture twin was compiled and tested from a draft** (`TK/🗑️generated/d2/draft/`, deleted after it went in) because the repository file was mounted and used by the stage; it was written to the repository in one save, test file first, and `cmp`-identical to the green draft.
8. **Clippy's layout suggestions** (`(SLOW..=FAST).contains`, `std::slice::from_ref`, a type alias for the replay tuple of the ticket tool) changed no expression a twin evaluates.
9. **Ticket tools print their summaries with `[DEBUG] `** like the earlier tools of this ticket; no product file prints.

## 6. Open

- **`verify rust-warnings`** cannot run at all until its scope fixture and the root `Cargo.toml` agree on `semio-hub-dag` (repository-wide; §4 row 9).
- **The stage port** (B1's integration in TypeScript) is the next consumer: the stage replay test and `🎪️stage-trace` parity stay red until it lands. Nothing in the Rust stage calls clearance yet.
- **`behavior::tests::the_graph_is_the_committed_graph`** is red in the real crate from a sibling's in-progress graph change (not D2's files).
- `larger`/`smaller` still live in terrain (`pub(crate)`); clearance and gesture import them from there like the stage modules do.
- `TK/🗑️generated/d2/` keeps logs, reports, `ported-from/`, the scratch crate sources and mutants, `corpus.json` and `world.json` (3 MB); cargo directories and `bits.json` were removed. The repo MCP server was unreachable in this session, so no ticket bookkeeping was done from D2. Nothing was committed.
