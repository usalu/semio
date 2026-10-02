# 📓️ Work package D — terrain

Ticket `2026/10/02/QUIZ-PETS`, design §4.5, §5.1–§5.2, §10. Phase 1 (TypeScript) is complete and green; the Rust twin and its adapters are phase 2. Paths: `P = 🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST = 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🏞️terrain/🟦️.ts` | the module (187 lines): constants, `perchesOf`, `perchAt`, `nearestPerch`, `strideTo`, `fallStep`, `landingOf`, `hopOf`, `hopStep`, `hopLanding`, types `Fall`, `Hop`, `Flight` |
| `P/🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🟦️.ts` | vitest suite, 34 tests; `polygon-clipping` 0.15.7 is the JavaScript oracle of the perch cutting on 300 generated layouts |
| `P/🧪️tests/🏞️terrain-walking/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 4 scenarios (`perches`, `standing`, `nearest`, `strides`), oracle `pets-numpy` |
| `P/🧪️tests/🦘️hop-ballistics/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 7 scenarios (`constants`, `falls`, `landings`, `drops`, `hops`, `flights`, `routes`), oracle `pets-scipy` |
| `P/🧫️fixtures/🏞️terrain-walking/🔣️.json` | 35 layouts (65 perches), 26 stand queries, 21 nearest queries, 12 walks |
| `P/🧫️fixtures/🦘️hop-ballistics/🔣️.json` | constants, 8 falls, 21 sweeps, 13 drops, 25 hops (all five verdicts), 15 flights, 13 routes |
| `TK/generate_terrain_vectors.py` | the generator; a second run writes byte-identical files |
| `TK/probe_terrain.ts`, `TK/typecheck_terrain.tsconfig.json` | ticket probes (flight against the committed arc; `tsc` over the four TypeScript files) |

No Rust file was created. No file of a sibling package or of the quiz product was edited.

## 2. The API as landed (package E already imports it)

All positions are viewport pixels (y down), velocities pixels per second, time whole ticks of 1/64 s. Only `+ − × ÷`, `sqrt`, `abs`, `floor`, `min`, `max` and comparisons; a unit test reads the module source and fails on any other `Math.*`, `Date`, `performance`, `console`.

- `perchesOf(surfaces, keepouts, width, height, clearance, minimum): Perch[]` — as design §4.5. Decisions on the edges:
  - the band is half-open, `[y − clearance, y)`, and a keep-out is the box `[y, y + height)`: a keep-out that ends exactly where the band begins, or begins exactly on the surface, blocks nothing; with `clearance = 0` there is no band and nothing blocks;
  - a keep-out removes the open interval `(x, x + width)`: one that only touches an end takes nothing away, abutting keep-outs leave no zero-width perch between them;
  - a keep-out with `width ≤ 0` or `height ≤ 0` blocks nothing (it would otherwise split a perch in two at a point);
  - a surface whose clipped extent is empty or a point yields nothing; `minimum` is compared with `x1 − x0 ≥ minimum`;
  - keep-outs are subtracted in list order without sorting; the result does not depend on that order (tested in three orders).
- `perchAt(perches, surface, x)` — first perch of the surface with `x0 ≤ x ≤ x1`.
- `nearestPerch(perches, x, y)` — least squared distance to the nearest point of the perch, the first among equals; no `sqrt`.
- `strideTo(x, goal, speed)` — step `max(speed, 0) ÷ 64`; the goal itself as soon as it is within one step.
- `fallStep(y, vy): { y, vy }` — `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`, then `y ← y + vy ÷ 64` (semi-implicit Euler, the same scheme as `springStep`).
- `landingOf(perches, x, fromY, toY)` — highest perch with `x0 ≤ x ≤ x1` and `fromY ≤ y ≤ toY`, first among equals. Both heights are included: an actor whose surface was re-surveyed at the same height lands at once instead of falling through, and a hop that ends exactly on its target height counts. Rising (`toY < fromY`) never lands.
- `hopOf(from, to): { vx, vy, ticks } | null` — see §3.
- `hopStep(x, y, vx, vy, to, ticks): { x, y, vx, vy }` — one tick of a hop with `ticks` left (counting the one being stepped); `fallStep` for the height, `x + vx ÷ 64`; on the last tick (`ticks ≤ 1`) the position is `to` itself.
- `hopLanding(perches, from, to, hop): Perch | null` — runs `hopStep` + `landingOf` for the whole flight and answers the perch it really ends on (the launch perch or a shelf when one lies in the way down, `null` when the target carries nothing). The stage uses it to accept only hops that end where they aim.

## 3. The hop and why it is exact

The design asks for a ballistic arc with an apex above both ends; the brief asks that an actor launched with the returned velocity lands on the target after the returned ticks under the per-tick integration. Under semi-implicit Euler the height after `k` ticks is `y + vy·k ÷ 64 + GRAVITY·k·(k + 1) ÷ 8192` (the ticks are samples of the continuous arc launched with `vy + GRAVITY ÷ 128`). `hopOf`:

1. `rise = max(HOP_CLEARANCE − min(dy, 0), |dx| × HOP_STEEPNESS)`; refused when `rise > HOP_HEIGHT` (too high) or `|dx| > HOP_DISTANCE` (too far).
2. `ticks = floor((sqrt(2·rise ÷ GRAVITY) + sqrt(2·(rise + dy) ÷ GRAVITY)) × 64 + 0.5)`; refused when `ticks > HOP_TICKS` (too long).
3. `vy = (dy − GRAVITY·ticks·(ticks + 1) ÷ 8192) × 64 ÷ ticks`, `vx = dx × 64 ÷ ticks` — the discrete sum, not the continuous formula of the research brief, so the sum ends on `to` (within 1e-12 by rounding; `hopStep` writes `to` itself on the last tick, so the landing is exact to the bit).
4. refused when `vy + ticks × GRAVITY ÷ 64 > FALL_SPEED` (too fast: such a descent is a fall; this also guarantees that the terminal clamp of `fallStep` never bends a granted hop).

Measured over 1 500+ granted hops in the unit suite: 15…48 ticks, the plain sum ends within 1e-9 of the target, the apex is within 3 px of `rise` and more than 9 px above the higher end, the first tick rises, the last tick descends.

## 4. Constants (starting values, exported, all literals)

| Constant | Value | Why |
|---|---|---|
| `GRAVITY` | 1800 px/s² | research §4.5; a 100 px drop takes ⅓ s; `GRAVITY ÷ 64 = 28.125` is exact in binary |
| `FALL_SPEED` | 900 px/s | research §4.5; reached after 32 ticks (232 px); 14 px per tick, no tunnelling because landing is swept |
| `HOP_CLEARANCE` | 12 px | apex above the higher end (research: ~10); a quarter of a 48 px pet |
| `HOP_STEEPNESS` | 0.375 | apex ≥ ⅜ of the horizontal distance → launch angle ≈ 56° for a level hop; without it a wide hop is a flat dart (the research brief bounded `vx` instead) |
| `HOP_HEIGHT` | 84 px | 1.5 × the tallest pet (56 px); a perch up to 72 px higher is reachable |
| `HOP_DISTANCE` | 160 px | about three body widths: a card gap plus the keep-outs at both ends |
| `HOP_TICKS` | 48 (0.75 s) | the research value 0.9 s could never bind beside the terminal-speed rule (≤ 53 ticks) |

Resulting hops: on the spot 15 ticks; level 96 px 26 ticks (`vx` 236 px/s); level 160 px 33 ticks; up 72 px 27 ticks; down as far as about 210 px (straight) or 155 px (at full width). To tune: change the literal in `🟦️.ts` (and later `🦀️.rs`), the same literal in `P/🧪️tests/🦘️hop-ballistics/🐍️.py`, rerun the generator; scenario `constants` fails if one side is forgotten.

## 5. Verification (real results, 2026-10-02)

| Command | Result |
|---|---|
| in `P/📦️packages/🟦️typescript`: `bun x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" terrain` | `Test Files 1 passed (1)`, `Tests 34 passed (34)` |
| in `P/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | `Test Files 6 passed (6)`, `Tests 261 passed (261)` (all pets unit suites present at that time) |
| repo root: `.venv/Scripts/python.exe TK/generate_terrain_vectors.py` | exit 0; `{"🏞️terrain-walking": {"layouts": 35, "perches": 65, "standings": 26, "nearests": 21, "strides": 12}, "🦘️hop-ballistics": {"falls": 8, "landings": 21, "drops": 13, "hops": 25, "flights": 15, "routes": 13}}`; a second run left both fixtures byte-identical (`cmp`) |
| in `TEST`: `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with the repository-wide backlog `5868 high-priority breach(es) across 4 rule(s)`; **no line names `🏞️terrain-walking` or `🦘️hop-ballistics`**; the only three lines naming pets are unresolved fixtures of package E's cases (`🎪️stage-trace`, `🤝️bond-dynamics`, `🧠️behavior-choice`) |
| `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🏞️terrain-walking"` | `[test] level=exhaustive cases=1 executed=4 passed=4 failed=0 errored=0 parity=0/0` |
| `bun ./📜️script.ts parity exhaustive --owner … --case "🏞️terrain-walking"` | `[test] level=exhaustive cases=1 executed=8 passed=8 failed=0 errored=0 parity=4/4` |
| `bun ./📜️script.ts oracle exhaustive --owner … --case "🦘️hop-ballistics"` | `[test] level=exhaustive cases=1 executed=7 passed=7 failed=0 errored=0 parity=0/0` |
| `bun ./📜️script.ts parity exhaustive --owner … --case "🦘️hop-ballistics"` | `[test] level=exhaustive cases=1 executed=14 passed=14 failed=0 errored=0 parity=7/7` |
| repo root: `bun x tsc -p TK/typecheck_terrain.tsconfig.json` | 0 errors in pets files (134 errors in files of other owners that the harness package pulls in) |
| repo root: `bun TK/probe_terrain.ts` | every committed flight: `worst 0` (the subject equals the committed arc bit for bit) |

`--case` takes the directory name with its emoji; the bare slug selects nothing (`cases=0`).

## 6. Deviations from the design and the brief

1. **What the oracles project.** The first version projected the library numbers (scipy's integrated arc, numpy's closed forms). Parity of scenario `flights` then failed (`parity=6/7`, 3 differences) on 3 of about 900 numbers that agreed to 1e-12: the harness rounds both projections onto a decimal grid of 1e-9 before it compares, and exact binary fractions with ten fractional bits (e.g. `251.9384765625`), which a flight in 1/64 s ticks produces all the time, are ties on that grid. The oracles therefore answer the plain binary64 value of the stated tick (a few lines written from the design text) and refuse to answer unless the library reaches the same number within 1e-9: `solve_ivp` + the straight line at the terminal speed + `numpy.cumsum` for falls and flights, `numpy.roots` for the duration and `scipy.optimize.brentq` for the launch velocity of a hop, the closed form for walks. Perches (grid masks), standing, nearest, landings, drops and routes are decided by the libraries alone (membership on the quarter-pixel lattice; sweeps over the integrated flight with a 1e-6 robustness margin). The comparison is thereby bit-exact on every platform, which is stricter than the profile and will hold the Rust twin to the same bits.
2. **A fourth reason for no hop** beside too high, too far, too long: landing faster than `FALL_SPEED`.
3. **`HOP_STEEPNESS`** replaces the research brief's `vxMax`; **`hopStep` takes the target and the ticks left** so that the last tick is exact; **`hopLanding`** is an addition.
4. **JavaScript oracle registration.** The unit suite imports `polygon-clipping` (0.15.7, hoisted; a dependency of the ui react package and already a classified test oracle in `dependency`). Package A owns `P/🔮️oracles/🔣️.json` and the package manifest, so the entry `pets-polygon-clipping` (`hostPath` like `pets-ajv-structure`, capability `pets-terrain-walking`) and the devDependency `"polygon-clipping": "0.15.7"` are **not** added yet.

## 7. Open

- Package A: register `pets-polygon-clipping` and add the devDependency (see 6.4).
- Phase 2 (Rust twin, package L): translate expression by expression. Notes: `Math.max(a, 0, b)` ≙ `a.max(0.0).max(b)`; `0 - step` stays a subtraction; the three limits of `hopOf` are written `!(… <= …)` so that non-finite points are refused (Rust's `max`/`min` ignore NaN where JavaScript propagates it — both still answer `None`/`null`, tested in TypeScript only); `ticks` is a float until the hop is returned; `GRAVITY * ticks * (ticks + 1)` is evaluated left to right; the adapters project the shapes of `P/🧪️tests/*/🟦️.ts` (perch answers as indices into the committed list).
- Repo test platform (not ours): `compareProjections` in `TEST/🟦️.ts` rounds onto the tolerance grid and then compares with the tolerance, so two numbers 1e-13 apart can fail when they straddle a rounding boundary. Cases of other packages that project library floats may meet it; comparing the unrounded numbers would remove it.
- Constants are starting values; judge them in the stories gallery once the stage is running, then follow §4 to retune.
- `TK/🗑️generated/wp-d/` was emptied after these runs.
