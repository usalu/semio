# 📓️ Report — round 2, work package D1 (Rust twins: trigonometry additions check, swing, climbing, terrain additions)

Ticket `2026/10/02/QUIZ-PETS`, round 2, phase D. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Host: Windows, bun 1.4.2, `rustc 1.99.0-nightly (c4af71034 2026-07-06)`. Everything below was run on 2026-10-03 between 13:55 and 16:13 while D2, D3 and B4 worked in the same tree. Tool output: `TK/🗑️generated/d1/` (logs, `ported-from/`, the scratch crate sources; its build directories and the 44 MB bit dump were deleted at 16:13 — `d1_scratch.ts prepare` and `d1_dump_gear_bits.ts` restore them in seconds).

**State: final.** The Rust twins mirror the TypeScript as it stood at 16:13 (`cmp` of every ported file against `TK/🗑️generated/d1/ported-from/`: identical), including what B4 changed while this package ran (§3). Every check of §5 is green except two things that are not this package's: the stage replay `calm-home` and the repository-wide self-check of `verify rust-warnings`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🪢️swing/🦀️.rs` (new, 397 lines) | twin of `🟦️.ts`: the 33 constants, `Chute`, `Reel`, `swing_step`, `cone_clamp`, `follow_step`, `hang_of`, `hang_step`, `lean_of`, `ring_velocity`, `throw_of`, `release_velocity`, `throw_velocity`, `impact_speed`, `chute_opens`, `chute_of`, `canopy_of`, `flare_of`, `chute_wind`, `chute_step`, `reel_step` (private `stale`, `TAUT`) |
| `P/🔨️modules/🪢️swing/🧪️tests/🔬️unit/🦀️.rs` (new) | 23 tests: the cases of the TypeScript suite (gl-matrix's rotation restated with the platform's sine and cosine), every group of both committed fixtures (`🪢️swing-dynamics`, `🪂️parachute-descent`) number for number, the 12 committed bit patterns, a source scan |
| `P/🔨️modules/🧗️climbing/🦀️.rs` (new, 949 lines) | twin of `🟦️.ts`: the 74 constants, `WallHold`, `Effort`, `Toss`, `Haul`, `LadderStand`, `Handwork`, `Clamber`, `Entry`, `Leg<'a>`; `sighted`, `hoist_path`, the walls (`cling_of` … `mantle_path`), the wall lines (`crossable`, `lunge_path`, `chain_of`, `wall_path`, `wall_cost`), the ladders (`ladder_for` … `spill_of`), the rope (`shot_for` … `landing_for`), `route_of` |
| `P/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🦀️.rs` (new) | 48 tests: the cases of the TypeScript suite (both generated-stage law sweeps included), the wall lines, `sighted`, and every member of the committed `🧗️wall-climbing`, `🪜️ladder-geometry`, `🎣️grapple-reach` vectors within the oracles' 1e-9 |
| `P/🔨️modules/🏞️terrain/🦀️.rs` | additions: `WALL_LIP`, `walls_of`, `wall_at`, `nearest_wall`, `segment_hits`, `segment_clear`, private `flanks`; module docstring and imports (`Facing`, `Wall`). `Pitch` is the schema's (B4 moved it there at ~14:22 and replaced my struct by `pub use crate::schema::Pitch`) |
| `P/🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🦀️.rs` | +14 tests (45 now): the wall and sight cases of the TypeScript suite, segments against the separating axes and a 513-point sampled segment (400 generated), the committed pitches/stretches/nearests/sights; `index_of` made generic |
| `P/🧪️tests/{🪢️swing-dynamics, 🪂️parachute-descent, 🧗️wall-climbing, 🪜️ladder-geometry, 🎣️grapple-reach}/🦀️.rs` (new) | Rust subject adapters, scenario for scenario the `🟦️.ts` adapters (13 + 9 + 13 + 5 + 8 scenarios, `crossings` and the routes' `exit` included) |
| `P/📦️packages/🦀️rust/🦀️.rs` | two mounts after `terrain`: `pub mod swing;`, `pub mod climbing;` |
| `P/🦀️.rs` | `pub use crate::climbing::*;`, `pub use crate::swing::*;`; one phrase in the module docstring (walls; a pet in the hand, under its parachute and on a reeled rope; walls, ladders and grappling ropes) |
| `TK/d1_scratch.ts` | `prepare` (scratch crate `TK/🗑️generated/d1/crate`, own `[workspace]`, private build/target directories through `rust_scratch.sh`), `mutants` (25 broken twins against the bit check) |
| `TK/d1_dump_gear_bits.ts`, `TK/d1_check_gear_bits.rs` | the bit-exactness tool (§4) |
| `TK/d1_rehearse_adapters.ts` | the Rust adapters' projections against the TypeScript adapters, number for number (`scratch` or `harness`) |
| `TK/d1_fixture_shapes.ts`, `TK/d1_probe_case.ts` | the shapes of the five fixtures; one decoded case of the bit dump |

`atan_turns` and `fast_neg_exp` exist in `P/🔨️modules/📐️trigonometry/🦀️.rs` (A3), expression for expression the TypeScript; the bit tool covers them (§4) and `📐️turn-trigonometry` is 27/27 with the Rust subject. No TypeScript file, no fixture, no oracle, no registry and no sibling's module was touched.

## 2. The API as landed

```rust
// crate::swing
pub struct Chute { terminal, reach, flare, length: f64 }      pub struct Reel { bob: Point, length: f64 }   // Copy, serde
pub fn swing_step(anchor_before: Point, anchor_now: Point, bob: Point, previous: Point, length: f64, gravity: f64, damping: f64, rope: bool) -> Point;
pub fn cone_clamp(anchor: Point, bob: Point, length: f64) -> Point;            pub fn follow_step(grip: Grip, target: Point) -> Grip;
pub fn hang_of(feet: Point, length: f64) -> Hang;                              pub fn hang_step(hang: Hang, target: Point, length: f64) -> Hang;
pub fn lean_of(anchor: Point, bob: Point, length: f64) -> Turns;               pub fn ring_velocity(samples: &[Point]) -> Point;
pub fn throw_of(velocity: Point) -> Point;   pub fn release_velocity(samples: &[Point]) -> Point;   pub fn throw_velocity(hang: Hang, samples: &[Point]) -> Point;
pub fn impact_speed(vy: f64, height: f64) -> f64;   pub fn chute_opens(vy: f64, height: f64) -> bool;   pub fn chute_of(height: f64) -> Chute;
pub fn canopy_of(feet: Point, vx: f64, vy: f64, chute: Chute) -> Canopy;       pub fn flare_of(remaining: f64, flare: f64) -> f64;
pub fn chute_wind(ticks: Ticks, phase: Turns) -> f64;                          pub fn chute_step(canopy: Canopy, chute: Chute, target: f64, remaining: f64, wind: f64) -> Canopy;
pub fn reel_step(anchor: Point, bob: Point, previous: Point, length: f64, least: f64, age: Ticks) -> Reel;
// constants: f64 except CHUTE_REFLEX, REEL_RAMP: Ticks; RELEASE_STALE: usize; RELEASE_WEIGHTS: [f64; 7]

// crate::terrain (additions)
pub const WALL_LIP: f64 = 6.0;
pub fn walls_of(walls: &[Wall], keepouts: &[Rect], width: f64, height: f64, clearance: f64, minimum: f64) -> Vec<Pitch>;
pub fn wall_at<'a>(pitches: &'a [Pitch], wall: &str, y: f64) -> Option<&'a Pitch>;   pub fn nearest_wall(pitches: &[Pitch], x: f64, y: f64) -> Option<&Pitch>;
pub fn segment_hits(from: Point, to: Point, rect: &Rect, margin: f64) -> bool;      pub fn segment_clear(from: Point, to: Point, rects: &[Rect], margin: f64) -> bool;

// crate::climbing
pub struct WallHold { x, y: f64, over: bool }   pub enum Effort { Climb, Hang, Rest }   pub struct Toss { vx, vy }
pub struct Haul { rope: f64, hand: Point, before: Point, x: f64, y: f64 }
pub struct LadderStand { wall: String, surface: String, side: Facing, foot: Point, top: Point }
pub enum Handwork { Grab, Hang, Climb, Lunge, Mantle }   pub struct Clamber { x, y: f64, hold: usize, work: Handwork }   pub enum Entry { Grab, Hang, Cling }
pub enum Leg<'a> { Ladder { at, ladder: &'a LadderStand, up }, Wall { at, pitch: &'a Pitch, hold: WallHold, exit: &'a Pitch, goal }, Raise { at, ladder: LadderStand }, Grapple { at, shot: Shot } }
pub fn sighted(from: Point, to: Point, keepouts: &[Rect], margin: f64, first: Point, second: Point) -> bool;
pub fn grip_for(perch: &Perch, pitch: &Pitch, size: Size) -> Option<WallHold>;   pub fn wall_holds<'a>(pitch: &Pitch, pitches: &'a [Pitch], y: f64, size: Size) -> Option<&'a Pitch>;
pub fn rim_for<'a>(pitch: &Pitch, perches: &'a [Perch], size: Size) -> Option<&'a Perch>;   pub fn grip_step(grip: f64, effort: Effort) -> f64;
pub fn climb_ticks(y: f64, goal: f64) -> Ticks;   pub fn slide_step(y: f64, vy: f64, floor: f64) -> Fall;
pub fn chain_of<'a>(pitch: &'a Pitch, pitches: &'a [Pitch], size: Size) -> Vec<&'a Pitch>;
pub fn wall_path(chain: &[&Pitch], from: usize, entry: Entry, x: f64, y: f64, to: usize, goal: f64, mantle: bool, size: Size) -> Vec<Clamber>;
pub fn wall_cost(path: &[Clamber]) -> f64;
pub fn ladder_for(low: &Perch, high: &Perch, pitch: &Pitch, keepouts: &[Rect], size: Size) -> Option<LadderStand>;   // ladder_holds likewise
pub fn ladder_rungs(ladder: &LadderStand) -> u32;   pub fn hook_ticks(from: Point, to: Point, speed: f64) -> Ticks;   pub fn haul_ticks(shot: &Shot, size: Size) -> Ticks;
pub fn shot_for(feet: Point, perches: &[Perch], keepouts: &[Rect], size: Size) -> Option<Shot>;   // shot_holds likewise
pub fn zip_step / sway_step / haul_step(shot: &Shot, haul: Haul, ticks: Ticks, size: Size) -> Haul;
pub fn route_of<'a>(x: f64, from: &Perch, to: &Perch, gear: &[Gear], size: Size, grip: f64, pitches: &'a [Pitch], ladders: &'a [LadderStand], keepouts: &[Rect]) -> Option<Vec<Leg<'a>>>;
// every other function: the TypeScript signature in snake_case, pitches/perches/ladders/shots by reference, Point and Size by value
```

How the TypeScript semantics are kept (design §2.4): every expression in the twin's order, `0.0 - x` for `0 - x`, no `mul_add`, no libm; `Math.max`/`Math.min` are terrain's `larger`/`smaller` (NaN and ±0 as JavaScript), the ternaries of the swing stay comparisons; `!(a > b)` conditions are named booleans (`climbed`, `raised`, `solid`, `between`) so NaN takes the twin's branch; ticks enter float expressions as `ticks as f64`, divisions by a tick count as `(n as f64) / (ramp as f64)`; an answered pitch, perch, standing ladder or wall-line member is a reference into the list it was asked of (the TypeScript `indexOf` is `ptr::eq`). Unit tests scan both module sources for `.max(`, `.min(`, `f64::max`, `.mul_add(`, the transcendentals, statics and print macros.

## 3. How the TypeScript was followed

B4 changed `🧗️climbing`, `🏞️terrain`, the schema and two adapters while this package ran. Each change was ported from a `diff` against the last fully ported copy, the copy replaced in the same step (`TK/🗑️generated/d1/ported-from/`):

| When (B4) | Change | Ported |
|---|---|---|
| ~14:14–14:22 | `Pitch` and `Wall` into the schema (all three twins); terrain's private types gone | terrain's `Pitch` struct replaced by B4 itself with `pub use crate::schema::Pitch`; climbing imports it from the schema |
| 14:18 | wall lines: `CROSS_REACH`, `LUNGE_TICKS`, `Handwork`, `Clamber`, `crossable`, `lunge_path`, `chain_of`, `wall_path`, `wall_cost`; `Leg` wall gains `exit`; `scaled` climbs whole lines; `LadderStand = Pick<Ladder, …>` | 14:32 (twin, unit tests, adapters: `crossReach`/`lungeTicks`, routes' `exit`, scenario `crossings`) |
| 14:30 / 15:14 | fixtures regenerated (`crossings`, `exit`) | nothing to port; parity found the next row |
| 15:13 | `wallCost` = every tick (the grab costs grip too) | 15:27, after `🧗️wall-climbing` parity showed 4 differences (routes 1, crossings 3) |
| ~16:05 | `sighted` exported | 16:08 (`pub fn sighted`, a unit test, 1 500 bit cases) |

At 16:13 every ported file is identical to its copy.

## 4. Bit-exactness beyond the tolerance profile

`bun TK/d1_dump_gear_bits.ts` evaluates every exported function of swing and climbing, the terrain additions and `atanTurns`/`fastNegExp`, and writes arguments and answers as IEEE-754 bit patterns (`nan` for every NaN) to `🗑️generated/d1/gear-bits.json`; `d1_check_gear_bits.rs` (integration test `gear_bits` of the scratch crate) recomputes every case with the Rust twins and compares bit for bit. Inputs: lattice values, arbitrary doubles, ±0, NaN, ±∞, subnormals and huge magnitudes; whole trajectories fed back (100-tick swings, 24 drags of 120 ticks, 40 descents of 120 ticks, 40 reels of 120 ticks, climbs and slides to their goals, ladder climbs up and down, 120 shots hauled to the end); generated stages that grant and refuse (ladders, shots, columns of cards with lunges, routes with every gear); and boundary families aimed at the mutants of the first round: equally distant pitch pairs with swapped offsets, segments lying exactly on the grown edges of a box, ladders exactly at the steepest and flattest lean, feet below binade boundaries, twin pitches (first among equals), gaps exactly at the lunge reach.

| Module | Functions (cases) | Result numbers | Mismatches |
|---|---|---|---|
| trigonometry | `atan_turns` 2 500, `fast_neg_exp` 1 200 | 3 700 | 0 |
| terrain | `walls_of` 900, `wall_at` 700, `nearest_wall` 1 500, `segment_hits` 6 500, `segment_clear` 900 | 15 570 | 0 |
| swing | `swing_step` 7 000, `cone_clamp` 1 600, `follow_step` 1 000, `hang_of` 400, `hang_step` 3 480, `lean_of` 4 380, `ring_velocity` 1 200, `throw_of` 1 200, `release_velocity` 1 200, `throw_velocity` 1 000, `impact_speed` 1 200, `chute_opens` 1 200, `chute_of` 400, `canopy_of` 600, `flare_of` 900, `chute_wind` 900, `chute_step` 5 800, `reel_step` 5 800 | 140 220 | 0 |
| climbing: walls | `hoist_path` 1 500, `cling_of`/`ledge_of`/`rim_of`/`climb_phase`/`slip_of` 600 each, `foot_of` 900, `clings` 1 000, `crowns` 1 000, `rim_for` 700, `grip_for` 1 500, `wall_holds` 1 000, `grip_step` 1 200, `climb_step` 9 379, `climb_ticks` 600, `slide_step` 3 600, `mantle_path` 900 | 33 935 | 0 |
| climbing: wall lines | `crossable` 1 500, `lunge_path` 1 500, `chain_of` 900, `wall_path` 400, `wall_cost` 400 | 179 714 | 0 |
| climbing: ladders | `ladder_for` 1 600, `ladder_holds` 1 360, `ladder_length`/`ladder_lean`/`ladder_exit`/`ladder_landing` 700 each, `ladder_rungs` 693, `ladder_step` 5 909, `ladder_at` 5 709, `ladder_phase` 5 609, `spill_of` 900 | 38 645 | 0 |
| climbing: rope and routes | `sighted` 1 500, `shot_for` 1 800, `shot_holds` 1 200, `miss_of` 700, `landing_for` 700, `haul_of` 700, `hook_ticks` 1 000, `hook_step` 1 000, `zip_step` 3 694, `sway_step` 8 378, `haul_step` 5 685, `haul_ticks` 420, `route_of` 3 000 | 175 947 | 0 |
| the 108 constants (`WALL_LIP`, swing 33, climbing 74; the release weights are 7 numbers) | 1 | 114 | 0 |
| **total (72 functions)** | **138 797** | **587 845** | **0** |

The same with `--release` on the previous dump (137 408 cases, 587 336 numbers): 0 mismatches. `route_of` answered 1 460 single-pitch wall legs, 38 legs across a wall line, 547 raised ladders, 399 shots and 333 standing-ladder legs.

That the tool can fail — `bun TK/d1_scratch.ts mutants`: **25 mutants, 19 caught, 6 survived (6 expected to), 0 surprises.**

| Mutant | Cases that differ (of 138 797) |
|---|---|
| `f64::max`/`f64::min` instead of `larger`/`smaller` | 48 (segment_hits 9, nearest_wall 9, walls_of 8, slide_step 10, segment_clear 5, sighted 3, ladder_exit 3, grip_step 1) |
| fused squared distance in `nearest_wall` | 50 |
| `rect.x + (width + margin)` in `segment_hits` | 31 |
| `<=` instead of `<` in the wall band (`flanks`) | 15 |
| fused free fall in `swing_step` | 306 |
| reassociated root in `swing_step` | 1 380 |
| reassociated release slope (`x × (64 ÷ 28)`) | 850 |
| fused steering in `chute_step` | 199 |
| fused span of a capped reel step | 18 |
| `y1 + (0.85h − 8)` in `foot_of` | 126 |
| reassociated hoist height | 302 |
| fused `span` (ladders, hooks, shots) | 121 |
| `speed × (ticks ÷ 64 ÷ length)` in `hook_step` | 106 |
| native extremes in `slide_step` | 5 |
| reassociated ramp in `gathered` | 2 |
| strict steepest lean | 80 |
| strict lunge reach in `crossable` | 281 |
| last instead of first among equals in `chain_of` | 12 |
| blended grab (`x(1−s) + hold·s`) in `wall_path` | 21 |
| **equivalent, survive by construction:** `root.max(0.0)` (NaN ignored alike; differs only for −0 under −0), `RATE × (ticks ÷ 64)` and `(RATE·ticks + 64·phase) ÷ 64` in the wind (scaling by 64 is exact), `60 × (k ÷ 10)` in the reel ramp (rounds to 6k for every k it meets), `±1 × (w ÷ 2)` in `cling_of`, `(s·t) ÷ (64·L)` in `hook_step` | 0 each |

The first mutant run (8 survivors) showed where the inputs were blind: ties of equal distance, segments on box edges, the lean bounds, binade-crossing feet, free (non-lattice) grabs. The boundary families above were added for them; the equivalent mutants were kept and marked with their reason.

## 5. Commands and real results

From the repository root unless a `cd` is shown.

| # | Command | Result |
|---|---|---|
| 1 | `bun TK/d1_scratch.ts prepare` then `bash TK/rust_scratch.sh d1 test --offline --lib` | `test result: ok. 201 passed; 0 failed` (14:58: swing 23, climbing 47, terrain 45, the scratch's other modules); 16:08 `… --lib climbing::` → `48 passed` |
| 2 | `bash TK/rust_scratch.sh d1 clippy --offline --lib --tests --features sut --example {swing_dynamics, parachute_descent, wall_climbing, ladder_geometry, grapple_reach, terrain_walking, hop_ballistics} -- -D warnings` | exit 0 (16:08); the same `--lib` with `--target wasm32-unknown-unknown` and `--target wasm32-wasip2`: exit 0 |
| 3 | `bun TK/d1_dump_gear_bits.ts` then `bash TK/rust_scratch.sh d1 test --offline --test gear_bits -- --nocapture` | `total: 138797 cases, 587845 result numbers, 0 throwing, 0 mismatches` |
| 4 | `bun TK/d1_scratch.ts mutants` | `25 mutants, 6 survived (6 expected to), 0 surprises` (§4) |
| 5 | `bun TK/d1_rehearse_adapters.ts scratch` | `scratch: 8 cases, 68 scenarios, 35282 numbers, 0 differences` |
| 6 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test --skip-nx-cache` (16:10) | exit 1: `427 passed; 1 failed; … 15 filtered out` — swing 23, climbing 48, terrain 45 all pass; the one failure is **`stage::tests::the_calm_home_replays_into_its_committed_trace`** (the stage is ported in a later package). A run at 16:00 also failed `behavior::tests::the_graph_is_the_committed_graph` (B4 had changed the activity graph before regenerating its fixture); gone at 16:10 |
| 7 | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` (16:11) | exit 0, no finding (D2's clearance and D3's mischief mounted) |
| 8 | the same `--lib` with `--target wasm32-unknown-unknown` and `--target wasm32-wasip2` (16:11) | exit 0 twice, no finding |
| 9 | `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets` (also `wasm32-unknown-unknown`, `wasm32-wasip2`) | exit 1 before compiling anything: `[verify rust-warnings] native misses semio-hub-dag.` — the command's own scope self-check fails repository-wide (the package `semio-hub-dag` is missing from the native scope); rows 7–8 run the same compilers on the same crate instead |
| 10 | in `TEST`: `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🪢️swing-dynamics"` | `executed=39 passed=39 failed=0 errored=0 parity=39/39` |
| 11 | … `--case "🪂️parachute-descent"` | `executed=27 passed=27 failed=0 errored=0 parity=27/27` |
| 12 | … `--case "🧗️wall-climbing"` (16:12) | `executed=39 passed=39 failed=0 errored=0 parity=39/39` (at 15:24, before porting B4's `wallCost`: `parity=35/39`) |
| 13 | … `--case "🪜️ladder-geometry"` (16:12) | `executed=15 passed=15 failed=0 errored=0 parity=15/15` |
| 14 | … `--case "🎣️grapple-reach"` (16:13) | `executed=24 passed=24 failed=0 errored=0 parity=24/24` |
| 15 | … `--case "📐️turn-trigonometry"` | `executed=27 passed=27 failed=0 errored=0 parity=27/27` |
| 16 | … `--case "🏞️terrain-walking"` | `executed=12 passed=12 failed=0 errored=0 parity=12/12` |
| 17 | … `--case "🦘️hop-ballistics"` | `executed=21 passed=21 failed=0 errored=0 parity=21/21` |
| 18 | `bun TK/d1_rehearse_adapters.ts harness` (the real Rust subject's last projections against the TypeScript adapters, no tolerance) | `harness: 8 cases, 68 scenarios, 35282 numbers, 0 differences` |
| 19 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |
| 20 | `rustfmt --check --edition 2021 --config-path rustfmt.toml` over my eleven product files, glue, façade and the check tool | no diff in any of mine (the findings it prints are in `🎪️stage` and `🧬️schema` unit tests, reached through the glue — not mine); my diffs of the first pass were applied by hand with Edit |
| 21 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py` over 19 files (product, glue, façade, ticket tools) | `0 finding(s) in 19 file(s)`; no `[DEBUG]`, no print macro, no comment inside a definition, no banned stem |

Each parity run executes the Python oracle, the TypeScript subject and the Rust subject and compares all three pairwise.

## 6. Decisions and deviations

1. **`LadderStand`, not the schema's `Ladder`.** The schema's `Ladder` is the stage object (owner, `since`, `until`, `rider` besides the geometry); the climbing functions build and answer the five-field geometry and the fixtures carry exactly that. B4 then named the TypeScript type `LadderStand = Pick<Ladder, …>` — the same name. `Wall`, `Shot` and `Pitch` are the schema's.
2. **References for answered entries.** `Option<&Pitch>`, `Option<&Perch>`, `Leg<'a>` holding `&'a Pitch`/`&'a LadderStand`, `chain_of → Vec<&'a Pitch>`, `wall_path(chain: &[&Pitch], …)`: the TypeScript answers the very object and its adapters project `indexOf`.
3. **Types that say what they are.** `ladder_rungs → u32` (the frame's `LadderFrame.rungs`), `hook_ticks`, `climb_ticks`, `haul_ticks → Ticks`, `Clamber.hold: usize`, tick constants `Ticks`, `RELEASE_STALE: usize`. Outside every valid stage they part from the TypeScript number: a non-finite or > 4.5e10 px ladder, a hook with a non-finite distance or a non-positive speed. The bit tool keeps to valid ladders and hooks there and says so in the dump.
4. **Closed sets are enums:** `Effort`, `Handwork`, and `Entry` for `wallPath`'s inline `"grab" | "hang" | "cling"` (wire names kebab-case).
5. **`#[allow(clippy::too_many_arguments)]`** on `swing_step`, `wall_path` and `route_of` (8, 9 and 9 arguments, the twin's signatures).
6. **Fixture tests of the climbing cases compare within 1e-9 and only the members the committed vectors state.** Their oracles restate the arithmetic in numpy (scipy for the swings) and stay within the comparison tolerance, and B4 adds members to the projections before it regenerates; the parity compares the whole projections. Swing and parachute vectors are the restated doubles and compare exactly.
7. **`chain_of` never ends for a degenerate pitch — in both twins.** A pitch of no length at |y| ≈ 1e300 absorbs `0.85 × height`, so its rim equals its foot and it is crossable to itself; TypeScript's `for (;;)` then grows the line until the array length throws, Rust until memory runs out. Found by the bit tool (case `chain_of`, pitch `y0 = y1 = −1e300`); outside every survey (`walls_of` yields positive lengths, and a cycle needs `y1 ≤ y0`). The column stages of the dump use finite pitches.
8. **The turn-trigonometry Rust adapter** (K's, extended by A3) writes `&Context` and warns under the lint set the scratch crate restates; it is not mine and the harness host compiles it without those lints, so the scratch's clippy runs name the other examples explicitly.
9. **Equivalent mutants are kept and labelled** instead of being replaced silently; each states why it cannot differ.
10. No sibling module was edited; the glue and façade were re-read right before each anchored edit (D2 and D3 added their lines around mine within minutes).

## 7. Open

- **B4** is still running: whatever it changes in `🧗️climbing`, `🏞️terrain` or the two climbing adapters after 16:13 needs the same port (`diff TK/🗑️generated/d1/ported-from/<file> P/…`, port, `bun TK/d1_dump_gear_bits.ts`, the check, `d1_rehearse_adapters.ts scratch`, then §5). Consider bounding `chainOf`'s loops (decision 7) when touching it.
- **`calm-home`** stays red until the stage twin mirrors the second round (B1–B4's TypeScript); the stage will call `wall_path`, `route_of`, `ladder_holds`, `sighted` etc. through the types of §2 and has to map its schema `Ladder`s to `LadderStand`s (or the climbing twin gains a conversion then).
- **`verify rust-warnings`** cannot run until its scope self-check (`semio-hub-dag`) is fixed by whoever owns that package.
- Only 38 of the bit tool's route legs climb across a line; the mechanics behind them (`chain_of`, `wall_path`, `crossable`) have their own 2 800 cases and 176 314 numbers.
- `larger`/`smaller` still live in terrain (`pub(crate)`); climbing and swing import them from there like everyone else.
- `TK/🗑️generated/d1/` holds the logs of §5, the parity outputs, the rehearsal projections, the mutant sources and `ported-from/`; build directories and the bit dump were removed. Delete it with the ticket's generated folder.
- The repo MCP was unavailable (`CONNECTION_CLOSED`); no ticket bookkeeping was done from this package. Nothing was committed.
