# 📓️ Work package A4 — climbing: walls, ladders, grappling rope; terrain additions

Ticket `2026/10/02/QUIZ-PETS`, second round, phase A (design-v2 §15.2, §16, §21, §23; MECH §0.1, §3, §4, §5, §13).
`P = 🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST = 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.
TypeScript is complete and green; the Rust twins of both modules and the Rust adapters of the three cases are phase D.

## 1. Files

| File | Content |
|---|---|
| `P/🔨️modules/🏞️terrain/🟦️.ts` | additions only: `WALL_LIP`, `Pitch`, `wallsOf`, `wallAt`, `nearestWall`, `segmentHits`, `segmentClear` (existing functions untouched) |
| `P/🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🟦️.ts` | +20 tests (54 now): pitches against polygon-clipping, lookups, sight against the separating axes and a sampled segment |
| `P/🔨️modules/🧗️climbing/🟦️.ts` (new, 689 lines) | walls, ladders, rope, routes; 72 exported constants, 40 functions, 5 types |
| `P/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🟦️.ts` (new) | 64 tests |
| `P/🧪️tests/🧗️wall-climbing/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 12 scenarios, oracle `pets-scipy` |
| `P/🧪️tests/🪜️ladder-geometry/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 5 scenarios, oracle `pets-numpy` |
| `P/🧪️tests/🎣️grapple-reach/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 8 scenarios, oracle `pets-scipy` |
| `P/🧫️fixtures/{🧗️wall-climbing, 🪜️ladder-geometry, 🎣️grapple-reach}/🔣️.json` | generated vectors (150 KB, 118 KB, 112 KB) |
| `P/🔮️oracles/🔣️.json` | anchored edits: capabilities `pets-ladder-geometry` (numpy), `pets-wall-climbing` and `pets-grapple-reach` (scipy), `pets-wall-climbing` (polygon-clipping); engine lists and rationales extended |
| `P/📦️packages/🟦️typescript/🟦️.ts` | `export * from "../../🔨️modules/🧗️climbing/🟦️.ts"` after `🪢️swing` (no export name collides, checked by `TK/a4_export_collisions.ts`) |
| `P/README.md` | terrain row extended, climbing row added |
| `TK/generate_climbing_vectors.py` | the generator; a second run is byte-identical |
| `TK/a4_probe_climbing.ts`, `TK/a4_export_collisions.ts`, `TK/a4_docstring_emojis.mjs` | probes and checks (kept) |

## 2. Terrain additions

- `WALL_LIP = 6` — the first 6 px beside a wall belong to the wall. **Why:** the survey reports every surface's own box grown 4 px sideways as a solid keep-out (5 stage px on a phone at scale 0.8), and a control flush inside a card has a 4 px margin too; with a band that starts at the wall line every wall would be blocked by its own box. 6 px forgives 4–5 px; the 8 px of a focused control still blocks.
- `type Pitch = { wall, surface, side: 1 | -1, x, y0, y1 }` — a free stretch of a wall.
- `wallsOf(walls, keepouts, width, height, clearance, minimum): Pitch[]` — `perchesOf` turned by 90°: walls whose band (`[x − clearance, x]` for side −1, `[x, x + clearance]` for 1) lies inside `[0, width]`; extent clipped to `[0, height]`; minus the open y-interval of every keep-out that overlaps the band from `WALL_LIP` to `clearance` with positive width (and has positive height); stretches `≥ minimum`; wall order, then down. Same edge rules as `perchesOf` (touching takes nothing, no zero-length pitch, order of keep-outs irrelevant). `clearance ≤ WALL_LIP` ⇒ no band.
- `wallAt(pitches, wall, y): Pitch | null`, `nearestWall(pitches, x, y): Pitch | null` — mirrors of `perchAt` / `nearestPerch` (asked with a perch end it answers the wall beside it).
- `segmentHits(from, to, rect, margin): boolean` — Liang–Barsky against the box grown by `margin`; only the open inside counts (an edge touched or a corner grazed is no hit); a box without area is never hit; a negative margin shrinks; a segment of no length is a point.
- `segmentClear(from, to, rects, margin): boolean` — **deviation:** a 4th parameter `margin` (MECH grows keep-outs by 2 px for ropes and 6 px for ladders; growing inside the test avoids allocating grown boxes).
- The non-exported structural `Wall = { id, surface, side, x, y0, y1 }` (design-v2 §21) stands in until the stage integrators add `Wall` to the schema (`Surveyed.walls`); then import it from the schema and delete the local alias.

## 3. Climbing module (`P/🔨️modules/🧗️climbing`)

Structural, non-exported (stated here until the integrators put them into the schema): `Ladder = { wall, surface, side, foot: Point, top: Point }`, `Shot = { surface, facing, muzzle, hook, length, reel: "zip" | "swing" }`. Exported types: `WallHold = { x, y, over }`, `Effort = "climb" | "hang" | "rest"`, `Toss = { vx, vy }`, `Haul = { rope, hand, before, x, y }`, `Leg` (union below).

**Hoisting** — `hoistPath(from, to, height, phase): Point`: MECH §5.3 for any two points; `to` exactly at phase ≥ 1, `from` exactly at ≤ 0.

**Walls**
- `clingOf(pitch, size)` x of a clinging actor (`x + side·w/2`); `ledgeOf(pitch, size)` where a mantle ends (`x − side·(w/2 + 8)`); `rimOf(pitch, size)` highest feet (`y0 + 0.85h`); `footOf(pitch, size)` lowest feet (`y1 − 8 + 0.85h`); `clings(pitch, y, size)`.
- `crowns(perch, pitch, size)` (same surface, `perch.y === pitch.y0`, carries the ledge), `rimFor(pitch, perches, size)`.
- `gripFor(perch, pitch, size): WallHold | null` — over the rim from the crowning perch (feet at `rimOf`), else standing at `clingOf` on the perch with the hands on the pitch.
- `wallHolds(pitch, pitches, y, size): Pitch | null` (same wall and side, `|Δx| ≤ 6`, still clinging), `slipOf(pitch): Toss` (`side·140`, `−160`).
- `gripStep(grip, effort)`, `climbStep(y, goal, ticks)`, `climbTicks(y, goal)` (`GRIP_BUDGET + 1` = does not last), `climbPhase(pitch, y, size)` (anchored at `pitch.y0`), `slideStep(y, vy, floor): Fall`, `mantlePath(pitch, size, phase)`.

**Ladders**
- `ladderFor(low, high, pitch, keepouts, size): Ladder | null` — foot at lean 0.25 clamped into `[x0 + 10, x1 − 10]`; rise in `[0.8h, 3.6h]`; contact `(pitch.x, pitch.y0 + 7)` on the pitch; lean in `[0.14, 0.40]` on the air side; line clear of keep-outs grown by 6 px, except those that hold the foot or the contact.
- `ladderHolds(ladder, perches, pitches, keepouts, size): Ladder | null` — foot keeps its x on a perch of its surface moved ≤ 12 px vertically, top follows a pitch of its wall with a crowning perch within 8 px, still valid for the owner's `size`; `null` = topple.
- `ladderLength`, `ladderRungs` (`floor(length ÷ 10.5)`), `ladderLean`, `ladderExit(ladder, size)`, `ladderStep(travel, goal, ticks)`, `ladderAt(ladder, travel)`, `ladderPhase(travel)`, `ladderLanding(ladder, size)`, `spillOf(ladder, y, size): Toss | null`.

**Rope**
- `shotFor(feet, perches, keepouts, size): Shot | null` — MECH §3.2 candidates (both ends inset `w/2 + 4`, the nearest point, or the middle of a narrow perch), facing towards the candidate, muzzle `(±0.3w, −0.65h)`, hook 1 px above the edge; rope `[0.8h, 3.2h]`, rise ≥ 0.42·length, line clear of keep-outs grown by 2 px except those holding the point of the edge under the hook; least `length + 0.3·|dx|`, first among equals; `zip` when `|dx| ≤ 0.35·rise`.
- `shotHolds(shot, perches, keepouts)`, `missOf(shot, perch)`, `hookTicks(from, to, speed)`, `hookStep(from, to, speed, ticks)` (out at `HOOK_SPEED`, back at `HOOK_RETURN`).
- `haulOf(shot, rope, size)` (at rest on the taut line), `zipStep(shot, haul, ticks, size)`, `swayStep(shot, haul, ticks, size)` (A3's `reelStep` with the hands as the bob), `haulStep` (dispatch on `shot.reel`), `haulTicks(shot, size)`, `landingFor(shot, perch, size)`.

**Routes** — `routeOf(x, from, to, gear, size, grip, pitches, ladders, keepouts): Leg[] | null`, with
`Leg = { means: "ladder", at, ladder, up } | { means: "wall", at, pitch, hold, goal } | { means: "raise", at, ladder } | { means: "grapple", at, shot }`.
It returns **every** feasible leg in the order of design-v2 §16 (standing ladder, wall, own ladder, rope), so the stage can take the first whose corridor is clear of bodies (§18). `at` is where the actor walks to on `from` first.

## 4. Constants (starting values; why)

| Constant | Value | Why |
|---|---|---|
| `HAND_HEIGHT` | 0.85 h | MECH's mantle start `y0 + 0.85h`; one hand height for entry, hold, rim and mantle |
| `CLIMB_RISE` / `CLIMB_DESCENT` / `CLIMB_RAMP` | 30 / 40 px/s, 6 ticks | MECH §5.2; the ramp of the ladder applied to walls too |
| `GRIP_SPACING` | 0.3 h | MECH gait phase from distance |
| `GRIP_BUDGET` / `GRIP_CLIMB` / `GRIP_HANG` / `GRIP_REST` | 384 / 1 / 0.25 / 2 | MECH §5.2; 384 ticks reach 178 px of wall |
| `GRIP_BITE` | 8 px | new: the hands never hold the very end of a stretch; a wall whose foot is above the perch can be taken while the hands reach 8 px above its end (≤ 0.85h − 8 above the feet) |
| `WALL_FOLLOW` | 6 px | MECH §5.4 |
| `SLIDE_START` / `SLIDE_GAIN` / `SLIDE_SPEED` | 30 px/s, 600 px/s², 160 px/s | MECH §5.2 |
| `SLIP_PUSH` / `SLIP_LIFT` | 140 / 160 px/s | MECH §5.4 |
| `WALL_GRAB_TICKS` / `WALL_HANG_TICKS` / `MANTLE_TICKS` | 8 / 10 / 28 | MECH §5.2–5.3 |
| `MANTLE_INSET`, `HOIST_HUMP`, `HOIST_RISE` | 8 px, 0.1 h, 0.65 | MECH §5.3 |
| `LADDER_LEAN` / `LADDER_STEEP` / `LADDER_FLAT` | 0.25 / 0.14 / 0.40 | MECH §4.1 (4 : 1 rule) |
| `LADDER_SHORT` / `LADDER_TALL` | 0.8 h / 3.6 h | MECH §4.1, of the owner |
| `LADDER_TUCK` / `LADDER_HORNS` / `RUNG_SPACING` | 7 / 14 / 10.5 px | **deviation:** MECH's 0.15h / 0.3h / 0.22h of a 48 px pet as pixels — a ladder is an object; whoever climbs it meets the same rungs and the same rim |
| `LADDER_FOOTING` / `LADDER_GIRTH` | 10 / 6 px | MECH §4.1 (footprint ±10, capsule half-width 6, here a box grown by 6) |
| `LADDER_RISE` / `LADDER_DESCENT` / `LADDER_RAMP` / `LADDER_EXIT` | 26 / 34 px/s, 6 ticks, 0.4 h | MECH §4.3 |
| `LADDER_FOLLOW` / `LADDER_SHIFT` | 8 / 12 px | MECH §4.4 |
| `LADDER_MOUNT_TICKS` / `LADDER_DISMOUNT_TICKS` / `LADDER_RAISE_TICKS` / `LADDER_IDLE` / `LADDER_LIFE` | 8 / 24 / 30 / 1280 / 7680 | MECH §4.2–4.3 (for the stage) |
| `TOPPLE_STIFFNESS` / `TOPPLE_DAMPING` / `TOPPLE_PUSH` / `TOPPLE_STEP` | 150 / 24 / 70 px/s / 0.5 h | MECH §4.4 |
| `HOOK_SPEED` / `HOOK_RETURN` | 640 / 1280 px/s | MECH §3.4 |
| `ROPE_SHORT` / `ROPE_LONG` / `ROPE_ELEVATION` / `ROPE_MARGIN` / `ROPE_DETOUR` | 0.8 h / 3.2 h / 0.42 / 2 px / 0.3 | MECH §3.2 |
| `ROPE_RISE` | 0.8 h | MECH §3.2; never binds alone (with the muzzle at 0.65h, `ROPE_SHORT` and the elevation exclude every perch less than ~1.45h above the feet) — kept as the cheap first test |
| `ROPE_FOLLOW` | 8 px | MECH §3.1 ("target moved > 8 px") |
| `MUZZLE_FORWARD` / `MUZZLE_HEIGHT` / `HOOK_INSET` / `HOOK_LIFT` | 0.3 w / 0.65 h / 4 px / 1 px | MECH §3.2 |
| `ZIP_SLANT` / `ZIP_SPEED` / `ZIP_RAMP` | 0.35 / 150 px/s / 10 ticks | MECH §3.1 |
| `ROPE_AIM_TICKS` / `ROPE_RECOIL_TICKS` / `ROPE_TUG_TICKS` / `ROPE_HOIST_TICKS` / `ROPE_SHRUG_TICKS` | 14 / 6 / 4 / 46 / 40 | MECH §3.1, §3.4 (hoist = 0.35 s climb-last + 24-tick mantle) |
| `ROPE_MISS_CHANCE` / `ROPE_MISS_OVERSHOOT` / `ROPE_REST` / `ROPE_SULK` | 0.1 / 14 px / 256 / 3840 ticks | MECH §3.2, §3.4 |

The reel itself (`REEL_SPEED`, `REEL_RAMP`, `REEL_LEAST = 0.9 h`, `REEL_CAP`, `REEL_DAMPING`) is A3's (`🪢️swing`); the zip ends at the same `REEL_LEAST` (MECH: both end at 0.9h), so there is no second constant.

## 5. Verification (real results, 2026-10-03)

| Command | Result |
|---|---|
| repo root: `.venv/Scripts/python.exe TK/generate_climbing_vectors.py` (twice) | exit 0 both; all three fixtures and the summaries byte-identical (`cmp`). Summary: walls 43 layouts / 107 pitches, 559 sight verdicts, 82 holds, 9 climbs (0 … 385 ticks), 11 surveys; ladders 33 placements (17 granted, all 9 verdicts), 6 climbs, 27 surveys (12 standing); rope 35 shots (23 granted, all 6 verdicts, both reels), 8 flights, 6 zips, 5 swings, 13 surveys, 26 routes (every means) |
| swings vs `solve_ivp` (while rope ≥ 0.4 of its start) | worst 0.04°, 0.36°, 0.15°, 0.13°, 0.07° — all within MECH's 0.4°; no step cut by the cap |
| in `TEST`: `bun ./📜️script.ts oracle exhaustive --owner P --case "🧗️wall-climbing"` | `executed=12 passed=12 failed=0` |
| same, `🪜️ladder-geometry` / `🎣️grapple-reach` | `executed=5 passed=5` / `executed=8 passed=8` |
| `subject exhaustive … --implementation typescript`, the three cases | `12/12`, `5/5`, `8/8` passed |
| `parity exhaustive … --implementation typescript`, the three cases | `executed=24 passed=24 parity=12/12`, `executed=10 passed=10 parity=5/5`, `executed=16 passed=16 parity=8/8` |
| `parity exhaustive --owner P --case "🏞️terrain-walking"` (with Rust) | `executed=12 passed=12 failed=0 parity=12/12` |
| `parity exhaustive --owner P --case "🦘️hop-ballistics"` (with Rust) | `executed=21 passed=21 failed=0 parity=21/21` |
| in `P/📦️packages/🟦️typescript`: `bun ./📜️script.ts typecheck` | exit 0 (also after the glue edit) |
| `bun ./📜️script.ts test` | exit 0, `Test Files 15 passed`, `Tests 967 passed`, 7.9 s wall (host busy); my two suites 0.13 s + 0.05 s of added test time |
| `bun ./📜️script.ts test quick` | exit 0, 15 files, 967 tests, 21.3 s wall |
| `bun ./📜️script.ts test exhaustive` | exit 1: 6 failures, all in `🔨️modules/👆️gesture` (sibling A6, in progress); terrain and climbing pass at this level |
| `contract --owner P` | exit 1 with the repository backlog (6252 high-priority breaches); no line names my cases; the only pets lines are in `🚧️clearance-proof/🥒️.feature` (sibling A5) |
| repo root: `bun ./📜️script.ts verify taxonomy report --scope P` | `clean=true errors=0 warnings=0` |
| `bun TK/a4_export_collisions.ts` | `112 runtime exports of 🧗️climbing; collisions: none` (types checked by grep: none) |
| `node TK/a4_docstring_emojis.mjs` | every docstring of my ten files starts with an emoji, unique per file; no console, no `[DEBUG]`, no comment inside a definition |

Found and fixed on the way: the exhaustive `wallsOf` sweep met a 3e-14 px pitch that polygon-clipping snaps away (decimal grain sums such as `100.3 + 60.2`); the sweep now uses dyadic grains (1, 4, 8), like the lattice of the vectors. A generated-ladder law in the quick sweep missed that keep-outs holding the foot or contact are passed over; the law now says so.

## 6. For the stage integrator (B4)

State per footing:
- **wall**: the pitch (or wall id + side, re-found by `wallHolds` after each survey, after riding the wall's vertical move), feet `y`, goal, the tick the climb began (ramp), the actor's `grip` (number, `gripStep` every tick: "climb", "hang" while still or sliding, "rest" on a perch), the mantle tick. Entry: `gripFor`; exit up: `mantlePath` over `MANTLE_TICKS` onto `rimFor`; exit down: climb to the foot hold of `gripFor(lowPerch, pitch)`; grip 0 → `slideStep` to `footOf` or the perch below, then fall; lost wall → `slipOf`.
- **ladder** (stage object): `Ladder` fields + owner, state (raising/standing/toppling), since, last use, occupant, queue; the climber: ladder id, `travel`, goal (`ladderExit` or 0), climb start tick; `ladderHolds` with the **owner's** size after each survey; `spillOf` for the rider; `hoistPath(ladderAt(exit), ladderLanding)` over `LADDER_DISMOUNT_TICKS`.
- **rope**: `Shot`, phase (aim / fly / tug / haul / hoist / retract), phase start tick, `Haul` (`haulStep` per tick until `rope ≤ REEL_LEAST·h`), then `hoistPath(feet, landingFor)` over `ROPE_HOIST_TICKS`; `shotHolds` while the hook flies or holds; misses: `missOf` + `hookStep(…, HOOK_RETURN, …)`; cooldown counters (`ROPE_REST`, `ROPE_SULK`).
- Recommended survey call: `wallsOf(walls, keepouts, width, height, widestBody + 8, 1.5 × smallestHeight)` (MECH §5.1 strip `w + 8`, stretches ≥ 1.5h); pitches stored beside the perches (schema `Stage` gains them).
- Bodies are not keep-outs here: pass the boxes of other solid actors in `keepouts` when a line must also miss them (MECH §3.2), and judge corridors/claims (§18) per leg of `routeOf`.
- Wall-to-wall transfers are **not** provided: on quiz pages the cards of a column are 30 px apart and only the first has a perch; with hands at 0.85h and an 8 px bite a pet bridges a gap of at most `0.85h − 16` px (24.8 px at 48 px) with its feet still on the lower wall. Whether pets rest on walls (design-v2 §16) or lunge across is a stage decision; `clings` gives the geometry.

## 7. For the Rust twin (phase D)

- Translate expression by expression; `Math.max/min` → the terrain's `larger`/`smaller` (NaN and ±0 as JavaScript); `0 - x` stays a subtraction; `ceiling(v) = 0 - floor(0 - v)`; `fraction(v) = v - floor(v)`; `1 - HOIST_RISE` stays an expression.
- `segmentHits`: `dx == 0.0` also for −0; the guard `!(width > 0 && height > 0 && left < right && top < bottom)` exactly.
- Loops are bounded: `climbTicks` by `GRIP_BUDGET + 1`, `haulTicks` by 1024.
- `routeOf` returns `Option<Vec<Leg>>` with an enum `Leg`; `gear.contains(&Gear::…)`.
- `Wall`, `Ladder`, `Shot` become schema types (all three twins) when the integrators add them; `Pitch` likewise if the stage stores pitches.
- Adapters (`🦀️.rs` of the three cases) project the shapes of the `🟦️.ts` adapters: pitches and ladders by index where the TypeScript adapter does, `null` ≙ `None`.

## 8. Oracles and cases

- `🧗️wall-climbing` (scipy): pitches by grid membership (no interval subtraction), lookups by raster and brute force, sight by separating axes (`numpy.cross`) + 4097 samples (non-robust vectors refused), climbs/grips/slides by closed forms, mantles by `scipy.interpolate.CubicHermiteSpline`; holds, surveys, routes by a numpy restatement (supplement).
- `🪜️ladder-geometry` (numpy): `numpy.hypot`, ratio, `numpy.floor` of fits, `numpy.cumsum`, `numpy.cross`; the line by the wall oracle's separating axes.
- `🎣️grapple-reach` (scipy): candidates by `numpy.hypot` and `numpy.argmin`, flights and zips by closed forms, swings by `solve_ivp` through the 🪢️swing-dynamics oracle (`pendulum`, `felt`, `reel_step`, `reel_lengths`, `REEL_*`), routes composed from the three oracles.
- The oracles load neighbours like `🪢️swing-dynamics` does (`neighbour(case)`): `🪜️` and `🎣️` load `🧗️wall-climbing`; `🎣️` also loads `🪜️` and `🪢️swing-dynamics`. **A3:** renaming those Python functions or retuning a reel constant breaks or invalidates `🎣️grapple-reach/swings` until the generator is rerun.
- The unit suites' third-party judge is polygon-clipping (pitches); registered under `pets-polygon-clipping`.

## 9. Deviations and decisions

1. `WALL_LIP` (§2), the own-box rule for lines ("supports": keep-outs that hold the foot, the contact or the point under the hook are passed over) instead of MECH's "the wall's own rectangle excluded" — keep-outs carry no identity.
2. `segmentClear` has a `margin` parameter; a ladder's capsule is a box grown by 6 px (conservative at the ends).
3. Hands at 0.85h; `GRIP_BITE`; a wall can be taken from a perch below its foot while the hands reach it.
4. Ladder measures in pixels (`LADDER_TUCK`, `LADDER_HORNS`, `RUNG_SPACING`); the rise limits stay relative to the owner.
5. `routeOf` returns all legs in preference order; standing ladders are open to every actor owning `climb`, `ladder` or `grapple` (a parachute alone climbs nothing); its own ladder only where none joins the pair.
6. The mantle and the hang over a rim cost grip like climbing (MECH is silent).
7. Not built: the swing's release window ("Tarzan", optional in MECH §3.3), wall-to-wall transfers (§6), the ladder's stage life (raise/topple animation, queue) — constants exported for the stage.
8. No Rust file was created or changed; no file of a sibling package was edited except the anchored registry/glue/README lines named in §1.
