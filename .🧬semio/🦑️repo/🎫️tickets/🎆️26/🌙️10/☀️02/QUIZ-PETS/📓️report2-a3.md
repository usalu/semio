# 📓️ Report — round 2, work package A3 (swing: hanging, release, parachute; trigonometry additions)

Ticket `2026/10/02/QUIZ-PETS`, round 2, phase A (TypeScript; the Rust twin of `📐️trigonometry` additions included).
`P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.
Everything below was run on 2026-10-02/03 on the Windows host (bun 1.4.2, `.venv` Python 3.14.4, numpy 2.4.3, scipy 1.17.1)
while eight siblings worked in the same tree. Tool output: `TK/🗑️generated/a3/`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/📐️trigonometry/🟦️.ts`, `🦀️.rs` | + `atanTurns(y, x)` / `atan_turns`, `fastNegExp(x)` / `fast_neg_exp` (twins, bit-identical) |
| `P/🔨️modules/📐️trigonometry/🧪️tests/🔬️unit/🟦️.ts`, `🦀️.rs` | + 11 vitest tests, + 6 fundamental and 2 `quick::` cargo tests |
| `P/🧪️tests/📐️turn-trigonometry/{🥒️.feature, 🐍️.py, 🟦️.ts, 🦀️.rs}` | + scenarios `arctangents`, `arctangent-grids` (quick), `decays` |
| `P/🧫️fixtures/📐️turn-trigonometry/🔣️.json` | regenerated: + 40 points, 3 lattices (1355 directions), 19 decays |
| `P/🔨️modules/🪢️swing/🟦️.ts` (new, 365 lines) | the module (§2) |
| `P/🔨️modules/🪢️swing/🧪️tests/🔬️unit/🟦️.ts` (new) | 24 vitest tests (§4) |
| `P/🧪️tests/🪢️swing-dynamics/{🥒️.feature, 🐍️.py, 🟦️.ts}` (new) | 13 scenarios, oracle `pets-scipy` |
| `P/🧪️tests/🪂️parachute-descent/{🥒️.feature, 🐍️.py, 🟦️.ts}` (new) | 9 scenarios, oracle `pets-scipy` |
| `P/🧫️fixtures/🪢️swing-dynamics/🔣️.json`, `P/🧫️fixtures/🪂️parachute-descent/🔣️.json` (new) | generated vectors (109 kB, 28 kB) |
| `P/🔮️oracles/🔣️.json` | anchored edits: capabilities `pets-swing-dynamics`, `pets-parachute-descent` on `pets-scipy`; engine strings and rationales of `pets-numpy` and `pets-scipy`; scipy host-package rationale |
| `P/📦️packages/🟦️typescript/🟦️.ts` | `export * from "../../🔨️modules/🪢️swing/🟦️.ts"` (no collision, §6) |
| `TK/generate_swing_vectors.py` (new) | generator of both new fixtures; prints every measured deviation (§3) |
| `TK/generate_kinematics_vectors.py` | + the trigonometry groups; `write()` now writes beside and renames (a held fixture cannot be truncated on Windows: `Errno 22`) |
| `TK/probe_swing.py` (new) | the throw-away numbers behind the constants (sections `atan exp rod drift drag chute`) |
| `TK/a3_export_collisions.ts` (new) | which exported names of all pets modules collide |

No Rust file of `🪢️swing` and no `🦀️.rs` adapter of the two new cases exist yet (phase D). No git command was run.

## 2. Exports

### 2.1 `📐️trigonometry` additions

| Export | Meaning |
|---|---|
| `atanTurns(y: number, x: number): Turns` | direction of `(x, y)` in turns from +x towards +y, in (−½, ½]; Abramowitz–Stegun 4.4.49 on the octant ratio, `¼ − …` past the diagonal, `½ − …` for `x < 0`, negated for `y < 0`, a result rounding to −½ given as ½; 0 at the origin; exact 0, ¼, ½, −¼ on the axes; exactly odd in `y` (except ½), exactly scale-invariant for powers of two; no negative zero |
| `fastNegExp(x: number): number` | `1 ÷ (1 + x·(1 + x·(0.48 + 0.235·x)))` for `x > 0`, else 1 (NaN → 1); falls monotonically, never 0 |

Private literals: `ARC_1…ARC_9` = 0.999866, −0.3302995, 0.180141, −0.085133, 0.0208351; `DECAY_2 = 0.48`, `DECAY_3 = 0.235`.

### 2.2 `🪢️swing` (new)

Types: `Grip = {x, y, vx, vy}`, `Hang = {grip, bob, previous}`, `Chute = {terminal, reach, flare, length}`,
`Canopy = {x, y, vx, vy, bob, previous}`, `Reel = {bob, length}`.

| Function | Signature |
|---|---|
| `swingStep` | `(anchorBefore: Point, anchorNow: Point, bob: Point, previous: Point, length, gravity, damping, rope: boolean): Point` — MECH §1.3 literally (ternaries instead of `max`) |
| `coneClamp` | `(anchor: Point, bob: Point, length): Point` |
| `followStep` | `(grip: Grip, target: Point): Grip` — `springStep` per axis |
| `hangOf` | `(feet: Point, length): Hang` — grip `length` above the feet, at rest |
| `hangStep` | `(hang: Hang, target: Point, length): Hang` — follow, swing (`HANG_GRAVITY`, `HANG_DAMPING`), clamp |
| `leanOf` | `(anchor: Point, bob: Point, length): Turns` — `atanTurns((anchor.x − bob.x)/length, (bob.y − anchor.y)/length)` |
| `ringVelocity` | `(samples: readonly Point[]): Point` — LSQ2 slope at the newest of the last 7, px/s |
| `throwOf` | `(velocity: Point): Point` — dead zone, cap, rise cut |
| `releaseVelocity` | `(samples): Point` — `throwOf(ringVelocity)`, zero when stale |
| `throwVelocity` | `(hang: Hang, samples): Point` — MECH's blend `v_bob + ½(v_ring − v_grip)`, then `throwOf` |
| `impactSpeed` | `(vy, height): number` — `sqrt(vy² + 2·GRAVITY·max(height,0))`, at most `FALL_SPEED` |
| `chuteOpens` | `(vy, height): boolean` — `vy ≥ 240 && height ≥ 56 && impactSpeed > 600` |
| `chuteOf` | `(height): Chute` — 2h, 1.2h, 0.25h, 0.9h |
| `canopyOf` | `(feet: Point, vx, vy, chute): Canopy` — canopy `length` above the feet, no sway |
| `flareOf` | `(remaining, flare): number` — 1 above, ½ at and below the touch, linear between |
| `chuteWind` | `(ticks: Ticks, phase: Turns): number` — `10·sinTurns(0.35·ticks/64 + phase)` |
| `chuteStep` | `(canopy, chute, target: number, remaining, wind): Canopy` |
| `reelStep` | `(anchor: Point, bob: Point, previous: Point, length, least, age: Ticks): Reel` |

Constants (each with a docstring): `HANG_ROD 0.8`, `HANG_GRAVITY 7200`, `HANG_DAMPING 0.95`, `HANG_CONE 0.5`,
`FOLLOW_STIFFNESS 534`, `FOLLOW_DAMPING 46`, `RELEASE_WEIGHTS [7,−2,−7,−8,−5,2,13]`, `RELEASE_DIVISOR 28`,
`RELEASE_STALE 3`, `THROW_SHARE 0.5`, `THROW_LEAST 70`, `THROW_MOST 640`, `THROW_RISE 520`, `HARD_LANDING 600`,
`CHUTE_OPENING 240`, `CHUTE_HEADROOM 56`, `CHUTE_REFLEX 6`, `CHUTE_FACTOR 0.9168553557320289`, `CHUTE_DESCENT 2`,
`CHUTE_STEER_GAIN 1.2`, `CHUTE_STEER_SPEED 1.2`, `CHUTE_STEER_EASE 0.06`, `CHUTE_ROD 0.9`, `CHUTE_GRAVITY 900`,
`CHUTE_DAMPING 0.97`, `CHUTE_FLARE 0.25`, `CHUTE_WIND 10`, `CHUTE_WIND_RATE 0.35`, `REEL_SPEED 60`, `REEL_RAMP 10`,
`REEL_LEAST 0.9`, `REEL_CAP 520`, `REEL_DAMPING 0.9965`. Private: `TAUT = 1.000001`.

Imports: `springStep` (`🎞️animation`), `GRAVITY`, `FALL_SPEED` (`🏞️terrain`), `atanTurns`, `sinTurns` (`📐️trigonometry`),
types `Point`, `Ticks`, `Turns` (`🧬️schema`). No module imports `🪢️swing`, so the graph stays acyclic.

## 3. Measured accuracy (all from `TK/generate_swing_vectors.py`, log `TK/🗑️generated/a3/generate-swing-2.log`, and `TK/probe_swing.py`)

**Trigonometry.** `atanTurns` vs `atan2/2π`: max 1.829e-6 turns on 400 000 Gaussian points, 1.82e-6 on the diagonals,
step across a diagonal 3.6e-6 turns (polynomial max error 1.149e-5 rad, rising on [0, 1]). `fastNegExp` vs `e⁻ˣ`: max
1.876e-2 at x = 3.336, 5.06e-4 on [0, 1], 2.4e-4 on [0, 0.5]; monotone.

**Pendulum error** (max over every tick, SHAKE step vs `solve_ivp` DOP853 1e-12, start on the integrated pendulum):

| swing | L px | g | damping | start | horizon | max error | stated |
|---|---|---|---|---|---|---|---|
| gentle (MECH's case) | 100 | 1800 | 1 | 45° | 3 s | 0.166° | 0.4° |
| small / long rope / pushed off | 100 / 154 / 100 | 1800 | 1 | 20° / 5° / 0°+200°/s | 3 s | 0.048° / 0.005° / 0.231° | 0.4° |
| canopy | 43.2 | 900 | 1 | 30° | 3 s | 0.097° | 0.4° |
| wide | 100 | 1800 | 1 | 120° | 1 s (3 s: 0.84°) | 0.225° | 0.4° |
| held, undamped | 38.4 | 7200 | 1 | 60° | 0.5 s (3 s: 9.9°) | 1.377° | 1.5° |
| held | 38.4 | 7200 | 0.95 | 60° | 2 s | 0.600° | 0.8° |
| canopy damped / rope damped | 43.2 / 100 | 900 / 1800 | 0.97 / 0.9965 | 30° / 45° | 3 s | 0.016° / 0.109° | 0.4° |
| below a gliding anchor: rope ×2, held slow, held 120 px/0.4 s, lift, canopy | | | 1 | hanging | 1–2 s / 0.375 s | 0.109°, 0.113°, 0.282°, 1.313°, 0.000°, 0.152° | 0.4° / 1.5° |
| reeled 100→55 % at 60 px/s, 160→44 % at 90 px/s | | 1800 | 1 | 45°, 60° | 0.75 s, 1 s | 0.062°, 0.088° | 0.4° |
| below a canopy in uniform motion (4 sways) | 43.2 | 900 | 0.97 | −25°…20° | 3–8 s | ≤ 0.011° | 0.4° |

The held pendulum is stiff (ω·dt = 0.21): Störmer–Verlet's frequency error of (ω·dt)²/24 = 0.2 % costs 1.4° in half
a second at 60°. With damping the step is the centred difference of the pendulum with drag `128(1−d)/(1+d)` under a
gravity `2/(1+d)` times as strong (derived here; with the naive `γ = −64 ln d` the held swing is 2.94° off).

**Amplitude drift over 60 s** (peaks by a quartic `numpy.polyfit` through five samples): 45° / 100 px: 0.000000°;
60° held: 0.000051°; 120°: 0.000000°; 30° canopy: 0.000000° (bound 0.05°). Negative control, the shortcut from the
same start: loses 44.99°, 60.00°, 119.99°, 30.00° (control: > 10°).

**Lean for typical drags (MECH §1.3 scenarios, held gravity ×4, rod 38.4 px):** 120 px in 0.4 s: peak **46.9°**
(continuous system 48.5°), below 3° from 2.14 s; 300 px in 0.5 s: **60°** (cone), settled 2.22 s; 600 px in 0.35 s:
**60°** (cone), 1.88 s; 200 px in 1.0 s: **17.9°** (continuous 18.2°), 1.30 s. Probe for ×3 / ×5: 52.9° / 40.3°,
60° / 59.0°, 60° / 60°, 24.4° / 13.8° — MECH's ranges for ×3 … ×5 (40–51°, 60°, 60°, 14–24°) to within 2°.

**Follow spring (534, 46):** half of a jump after 4 ticks, real eigenvalues 0.799 and 0.352 (no overshoot), steady lag
behind a moving pointer `c/k − 1/64` = 70.5 ms, within 12.2 % of a jump of the continuous critically damped spring of
half-life 0.06 s (`scipy.linalg.expm`).

**Reel guards:** a pet reeled from 154 px at 85°: 722 px/s without the cap, 520.2 px/s with it (the step after the
re-projection may exceed the cap by at most what the reel takes in); the gentle reel never touches the cap (268.8 px/s).

**Impact speeds (h = 48, from rest, `fallStep` ticks):**

| drop | predicted, no chute | landed without a chute | with the chute: opens, lands | touch | descent speed at the touch (unflared) | MECH table (chute) |
|---|---|---|---|---|---|---|
| 100 px | 600.0 | 590.6 (tick 21) | **does not open** (600 is not > 600) | 590.6 | — | 233 |
| 100 px, chute forced at 240 px/s (probe) | | | tick 10, tick 26 | 126.9 | 221.4 | 233 |
| 102 px | 606.0 | | tick 10, tick 27 | 113.5 | 211.0 | |
| 110 px | 629.3 | | tick 10, tick 30 | 93.6 | 184.6 | |
| **150 px** | 734.8 | **731.2** (tick 26) | tick 10, tick 48 | **58.7** | 114.6 | 118 |
| **200 px** | 848.5 | **843.8** (tick 30) | tick 10, tick 79 | **50.7** | 97.3 | 98 |
| 300 / 500 px | 900 / 900 | | tick 10, tick 146 / 279 | 48.5 / 49.5 | 96.0 | 48 / 50 |
| 150 px, h = 40 / 56 | | | tick 10, tick 51 / 45 | 49.4 / 72.8 | 95.0 / 134.9 | |
| thrown down 300 px/s, 150 px | 793.7 | | tick 1, tick 43 | 58.5 | 111.0 | |
| thrown up 400 px/s, 60 px | 613.2 | 612.5 (tick 36) | does not open | 612.5 | — | |

Every drop lands within 0.52 ticks and 4.1 px/s of the continuous system (`solve_ivp` with events). MECH's chute row
is the unflared descent speed (my 114.6 / 97.3 vs 118 / 98; the probe's forced 100 px gives 221 vs 233). A canopy that
drifts at its limit trails its pet by 8.78° (balance of drag and gravity, matched to 0.003°).

## 4. Commands and real results

| Command | Result |
|---|---|
| `.venv/Scripts/python.exe TK/generate_kinematics_vectors.py` (repo root) | wrote `📐️turn-trigonometry`; second run: `unchanged` ×4 |
| `.venv/Scripts/python.exe TK/generate_swing_vectors.py` | wrote both fixtures; second run: `unchanged` ×2 |
| `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test` (final) | exit 0, `Test Files 15 passed`, `Tests 968 passed`, 8 s wall, Vitest 5.75 s; my suites: trigonometry 26 tests 393 ms, swing 24 tests 152 ms |
| same, `test quick` (whole package) | exit 1: 4 failures, all in `🚧️clearance` and `🧗️climbing` suites (siblings, mid-work); `test quick "🪢️swing/" "📐️trigonometry/"`: 50 passed; `test long …`: 50 passed; `test exhaustive …`: 50 passed |
| same, `typecheck` (final) | exit 0, no diagnostic |
| `TEST: bun ./📜️script.ts oracle exhaustive --owner P --case "🪢️swing-dynamics"` | `executed=13 passed=13 failed=0` |
| `… subject exhaustive --implementation typescript … "🪢️swing-dynamics"` | `executed=13 passed=13` |
| `… parity exhaustive --implementation typescript … "🪢️swing-dynamics"` | `executed=26 passed=26 parity=13/13` |
| `… oracle / subject --implementation typescript / parity --implementation typescript … "🪂️parachute-descent"` | `9/9`, `9/9`, `executed=18 passed=18 parity=9/9` |
| `… parity exhaustive --owner P --case "📐️turn-trigonometry"` (oracle, TypeScript and Rust) | `executed=27 passed=27 failed=0 parity=27/27` |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test` | `test result: ok. 190 passed; 0 failed; … 7 filtered out` (the `quick::` tests) |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test-quick` | `test result: ok. 199 passed; 0 failed` (incl. the platform `atan2` and `exp` comparisons) |
| `TEST: bun ./📜️script.ts contract --owner P` | 6252 repo-wide breaches; 0 name `🪢️swing-dynamics`, `🪂️parachute-descent` or `📐️turn-trigonometry` (the 7 pets breaches are all in `🚧️clearance-proof/🥒️.feature`, package A5) |
| `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py <17 files>` | `0 finding(s) in 17 file(s)`; no `console`, no `[DEBUG]` |
| `bun TK/a3_export_collisions.ts` | no collision with a swing name (see §6) |

The first oracle run caught a real disagreement (a bob exactly on the grip: numpy leaves it, the first clamp moved it
to the cone's edge); the clamp now leaves it.

## 5. Notes for the Rust twin (phase D)

- `🪢️swing/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`; wire `#[path = "../../🔨️modules/🪢️swing/🦀️.rs"] pub mod swing;` in `P/📦️packages/🦀️rust/🦀️.rs` and `pub use crate::swing::*;` in `P/🦀️.rs`; snake case names (`swing_step`, `cone_clamp`, `follow_step`, `hang_of`, `hang_step`, `lean_of`, `ring_velocity`, `throw_of`, `release_velocity`, `throw_velocity`, `impact_speed`, `chute_opens`, `chute_of`, `canopy_of`, `flare_of`, `chute_wind`, `chute_step`, `reel_step`).
- Evaluate exactly as written: `gravity / 4096.0`; `if root > 0.0 { root } else { 0.0 }`, `if radius > 1e-9 { radius } else { 1e-9 }` (no `f64::max`: it does not order signed zeros); `0.0 - THROW_RISE`; `(x * 64.0) / 28.0`; `(REEL_SPEED * (age + 1) as f64) / 10.0` with the integer test `age + 1 < REEL_RAMP`; `(CHUTE_WIND_RATE * ticks as f64) / 64.0 + phase`; `(vx + wind) / 64.0`; `(vy * flare) / 64.0`; `0.5 + (0.5 * remaining) / flare`; no `mul_add`.
- `ring_velocity`: loop over the first six weights, index `max(count − 7 + i, 0)`, every sample minus the newest; `stale` compares with `!=` exactly.
- `RELEASE_WEIGHTS` may be `[f64; 7]`; the `constants` projections compare numerically (7.0 = 7).
- Acceptance: the TypeScript projections in the two fixtures are the restated doubles and reproduce bit for bit (positions compared with `toEqual` in the unit suite); the `bit-patterns` scenario carries 12 single steps as hexadecimal. The Rust adapters mirror `P/🧪️tests/🪢️swing-dynamics/🟦️.ts` and `🪂️parachute-descent/🟦️.ts` function by function (`run`, `at`, `drift`, `anchored`, `guarded`, `followed`, `dragged`, `descended`, `swayed`, `dropped`).

## 6. Notes for the stage integrator (B1, B3, B4)

- **Held:** `hang = hangOf(feet, HANG_ROD × height)` at pick-up (or the species' `grip` as the rod); per tick `hang = hangStep(hang, pointerHeldInsideTheStage, length)`; the feet are `hang.bob`. Projecting a held pet out of a body (MECH §1.4) must shift `grip`, `bob` and `previous` together.
- **Tilt sign:** `leanOf` is in the rig's sense (positive turns +x towards +y, clockwise on screen, like SVG `rotate`): **negative when the feet trail to the right**. A target draws `translate(feet) rotate(tilt × 360° about pivot)`. MECH's literal `atanTurns(u.x, u.y)` is the opposite sign. Canopy sway for `ToolFrame{kind: "chute"}`: `leanOf(canopy, canopy.bob, chute.length)`.
- **Release:** keep the last 7 pointer targets, one per tick (sample and hold). Design-v2 §17 asks for the pointer's velocity: `releaseVelocity(ring)`; MECH's blend with the pendulum is `throwVelocity(hang, ring)`. Fewer than 7 samples: the oldest stands in.
- **Parachute:** per airborne tick with `vy > 0` and not in a planned hop: `chuteOpens(vy, H)` (H to the first thing below the feet); remember the tick; `CHUTE_REFLEX` ticks of plain `fallStep` later `canopy = canopyOf(feet, vx, vy, chuteOf(height))`; then per tick `canopy = chuteStep(canopy, chute, targetX, H, chuteWind(stageTick, actorPhase))`, feet = `canopy.bob`, touch speed `(next.y − canopy.y) × 64`. Brace (MECH §2.1): `impactSpeed(vy, H) > HARD_LANDING && H < CHUTE_HEADROOM`. Not in the module: the inflation and collapse springs and the ribbon (MECH §2.4–2.5), floaters sinking at 0.6h, the landing-spot search (§2.3).
- **Hard landings near the threshold:** the prediction `sqrt(vy² + 2gH)` is the continuous one; along the ticks of a fall it decays by `g²/4096` (0.66 px/s per tick at 600), and the decision is effectively taken at the first tick with `vy ≥ 240`. So a pet thrown up from 60 px lands at 612.5 px/s without opening. If "a pet with a parachute never lands hard" must hold exactly, classify hardness with the same prediction taken at that tick, or replace the prediction by the tick-invariant `sqrt((vy + g/128)² + 2gH) − g/128` (threshold from rest then 104.7 px) and regenerate.
- **Reel:** `reelStep(hook, bob, previous, length, REEL_LEAST × height, age)` with the bob at the centre of mass (MECH §3.3); the release-window search of §3.3 is not implemented here.
- **Export collisions among siblings** (found by `TK/a3_export_collisions.ts`, not mine): `Body` in `📝️draft` and `🚧️clearance`; `SLIDE_SPEED`, `SLIDE_GAIN` in `🚧️clearance` and `🧗️climbing` — `export *` of both would drop them.

## 7. Deviations and decisions

1. **`coneClamp`** tests the cone by direction (`dy < 0 || dy² < cone²·span`), also draws in a rod stretched by more than 1e-6 (MECH's step leaves the closest-approach point when the anchor jumps sideways by more than the rod, e.g. a 600 px flick: 50 px per tick on a 38 px rod), and leaves a bob on the grip untouched (numpy's reading).
2. **`leanOf` sign** as in §6 (rig sense), and it keeps the brief's `length` argument (the unit axis of MECH).
3. **Flare** scales the lagged speed (`vy × flareOf`), which equals MECH's `vt·(0.5 + 0.5·rem/flare)` once the descent is steady and avoids a jump from 233 to 96 px/s when the canopy is still braking; `Canopy.vy` stays unflared.
4. **From exactly 100 px at rest no parachute opens** (600 is not > 600 and the estimate decays); from rest every drop of 102 px or more opens. MECH's chute row for 100 px is a forced opening.
5. **Release** is split into `ringVelocity`, `throwOf`, `releaseVelocity` (brief and design-v2) and `throwVelocity` (MECH's blend).
6. **`fastNegExp` tolerance:** MECH §13 says 1e-3 against `numpy.exp`; that holds only up to x ≈ 1.06. Stated and tested: 1.9e-2 everywhere, 6e-4 up to 1.
7. **Tolerances of the swing:** MECH's 0.4° over 3 s holds for ropes and canopies; the stiff held pendulum states 1.5° over 0.5 s (undamped) and 0.8° over its damped life. The damped reference is the modified equation of §3 (stated in the feature).
8. **Follow spring** kept at 534 / 46 as specified (not the exactly critical 529 / 46); the lag the tick-wise spring shows is 70.5 ms (MECH's 87 ms is `2/ω` before the step).
9. **Oracles project the restated doubles** after the libraries confirmed them within the stated physical tolerance (the comparison grid of 1e-9 is far finer), as `🦘️hop-ballistics` does; the `turn-trigonometry` additions do the same with their bit patterns.
10. `chuteOpens` takes `(vy, height)` only; the `…` of the brief needed nothing more.

## 8. Open

- Phase D: Rust twin of `🪢️swing`, its unit suite and the two `🦀️.rs` adapters (until then the two new cases are TypeScript-only; full `parity` of them needs the Rust adapter).
- Registry: `pets-gl-matrix` judges `leanOf` in the swing unit suite (`mat2d.fromRotation`, `vec2.transformMat2d`), but its `capabilities` were not extended (I kept registry edits to the two capabilities on `pets-scipy`); add `pets-swing-dynamics` there if the dependency phase asks for it.
- Tuning by eye (stories gallery): `HANG_GRAVITY` (×3 … ×6), `HANG_CONE`, `FOLLOW_*`, `CHUTE_*`; rerun `TK/generate_swing_vectors.py` after any change.
- `TK/🗑️generated/a3/` holds the logs of this package (generator, tests, typecheck, contract, harness runs); delete it with the ticket's generated folder at closing.
