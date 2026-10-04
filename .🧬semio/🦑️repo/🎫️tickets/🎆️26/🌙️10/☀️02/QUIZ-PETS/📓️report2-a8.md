# 📓️ Work package A8 (effects and mischief, pure parts) — report

Ticket `2026/10/02/QUIZ-PETS`, second round, phase A. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Everything below was run on 2026-10-03 between 00:00 and 01:00 on the Windows host (bun 1.4.2, vitest 4.1.10, `.venv` Python with numpy 2.4.3; the harness environment has numpy 2.5.0), while eight siblings edited the same tree. Raw outputs: `TK/🗑️generated/a8/`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/✨️effects/🟦️.ts` | the hash, births, the five motions, the caps, the end of an emitter (322 lines) |
| `P/🔨️modules/🪄️mischief/🟦️.ts` | topic match, pick, gates, station, lift path, wake, throw (300 lines) |
| `P/🔨️modules/{✨️effects, 🪄️mischief}/🧪️tests/🔬️unit/🟦️.ts` | vitest suites: 32 + 31 tests |
| `P/🧪️tests/✨️particle-motion/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 6 scenarios (`hashes`, `uniformity`, `births`, `motions`, `caps`, `ends`) |
| `P/🧪️tests/🪄️mischief-choice/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 8 scenarios (`matches`, `candidates`, `choices`, `leaks`, `gates`, `stations`, `lifts`, `throws`) |
| `P/🧫️fixtures/{✨️particle-motion, 🪄️mischief-choice}/🔣️.json` | generated vectors (446 161 and 267 228 bytes) |
| `TK/generate_effects_vectors.py` | the generator named in both features (idempotent: a second run prints `unchanged` twice) |
| `TK/compare_effects_vectors.ts` | runs both TypeScript subject adapters over the committed vectors and lists every difference (numbers within 1e-9) |
| `TK/mutate_effects.ts` | mutation check of both unit suites (75 mutants, sandbox under `🗑️generated/a8/mutation`) |
| `TK/probe_effects.ts` | golden words, particle counts per motion, lift extremes |

Anchored edits in shared files: `P/🔮️oracles/🔣️.json` (capabilities `pets-particle-motion`, `pets-mischief-choice` on `pets-numpy`, one sentence in its rationale); `P/📦️packages/🟦️typescript/🟦️.ts` (two `export *` lines after `🧠️behavior`; no export name collides — checked against the schema and every module present at 00:45, after renaming my `Occasion` to `Circumstances` because `💗️feeling` exports an `Occasion`, and after dropping my `Toss` for the identical one of `🧗️climbing`). `🎲️randomness` is untouched: no new stream is needed (particles hash a key, the pick and the lift take units the caller draws).

## 2. `✨️effects`

### Exports

```ts
lowbias32(word: number): number                       // Wellons' lowbias32, uint32
mix(a: number, b: number): number                     // lowbias32((a ^ 0x9e3779b9) + (b + 1)·0x85ebca6b), all mod 2³²
unit                                                  // = unitOf of 🎲️randomness, re-exported under this name
scattered(key, index, lane): number                   // unit(mix(mix(key, index), lane))
emitterKey(seed, species, emitter, since): number     // mix(mix(mix(seed, species), emitter), since)
type Emission = Pick<Emitter, "motion" | "count" | "life" | "speed" | "spread">
type Particle = { x, y, scale, rotation: Turns, opacity, age: Ticks }
lifeTicks(e): Ticks        // max(1, ⌊life·64 + ½⌋)
swarmOf(e): number         // min(max(⌊count⌋, 0), EMITTER_CAP)
periodOf(e): Ticks         // max(1, ⌈life ÷ max(1, swarm)⌉)
bornAt(e, since, key, index): Ticks                    // since + index·period + ⌊scattered(key, index, 0)·period⌋
particlesOf(e, origin: Point, facing: 1 | -1, since, until: Ticks | null, tick, key): Particle[]   // eldest first
capped(particles, cap): readonly Particle[]
emitterEnds(e, since, until: Ticks | null): Ticks | null
```

Constants (each exported with a docstring): `EMITTER_CAP = 32` (the schema's bound of `count`), `STAGE_CAP = 160`, `TURN_RADIANS`, `AHEAD = 0`, `DOWN = 0.25`, `UP = 0.75`, lanes `LANE_BIRTH 0, LANE_HEADING 1, LANE_PACE 2, LANE_PHASE 3, LANE_LOOK 4`, `FALL_SWAY = 3` px, `FALL_SWAY_SPEED = 60` px/s, `FALL_SWAY_RATE = 0.5` /s, `FALL_FADE_IN = 4` ticks, `RISE_WANDER = 24` px per unit spread, `RISE_WANDER_RATE = 0.35` /s, `RISE_ROCK = 0.1` turns per unit spread, `BURST_DRAG = 0.4` s, `BURST_GRAVITY = 120` px/s², `ORBIT_SQUASH = 0.35`, `ORBIT_DEPTH = 0.15`, `ORBIT_FADE = 8` ticks, `DRIFT_MEANDER = 3` px, `DRIFT_RATE = 0.5` /s. All are starting values to tune by eye in the stories gallery (then rerun the generator).

### Emitter numbers (as A2's schema docstring fixes them) and the motions

`count` 1…32 alive at once, `life` seconds, `speed` px/s, `spread` 0…1 of a full turn over which directions scatter around the motion's own direction. Turns on screen, y down: 0 ahead (the facing), ¼ down, ¾ up. `s = age ÷ life`, `τ = age ÷ 64`, `u_l = scattered(key, index, l)`.

| motion | MECH §9.3 kinds it covers | position | scale, rotation, opacity |
|---|---|---|---|
| `fall` | rain (fast), snow (slow), sweat, drips | `a = ¼ + (u₁ − ½)·spread`, `v = speed·(0.9 + 0.2u₂)`; `x = ox + facing·(v·cos a·τ + sway)`, `y = oy + v·sin a·τ`; `sway = 3·fastNegExp(v ÷ 60)·sin(0.5τ + u₃)` — the speed decides rain or snow | `0.8 + 0.4u₄`; `facing·(a − ¼)`; `min(1, age ÷ 4)·(0.55 + 0.35u₄)·(1 − smoothstep((s − 0.75) ÷ 0.25))` |
| `rise` | hearts, notes, steam, heat waves | `a = ¾ + (u₁ − ½)·spread`, `v = speed·(0.7 + 0.3u₂)`, `w = 0.35τ + u₃`; `x = ox + facing·(v·cos a·τ + 24·spread·sin w·(0.3 + s))`, `y = oy + v·sin a·τ` | `0.5 + 0.5·smoothstep(age ÷ 8)`; `facing·0.1·spread·cos w`; `smoothstep(age ÷ 6)·(1 − smoothstep((s − 0.6) ÷ 0.4))` |
| `burst` | sparks, confetti, pop, dust, splash | all `count` born at `since`, once; `a = ¾ + ((index + u₁) ÷ count − ½)·spread` (stratified fan); `v = speed·(0.45 + 0.55u₂)`; `r = 0.4·(1 − fastNegExp(τ ÷ 0.4))`; `x = ox + facing·v·cos a·r`, `y = oy + v·sin a·r + ½·120·τ²` | `1 − 0.6s`; `a` (`½ − a` facing left); `1 − s²` |
| `orbit` | dizzy stars, orbiting hearts | `count` particles from `since`; `t = index ÷ count·spread + (tick − since) ÷ life`, `θ = t` (`½ − t` facing left); ring radius `R = speed·(life ÷ 64) ÷ 2π` (one lap per `life` at `speed`); `x = ox + R·cos θ`, `y = oy + 0.35R·sin θ` | `1 + 0.15·sin θ` (front larger); `0`; `smoothstep(elapsed ÷ 8)·(1 − smoothstep((tick − until) ÷ 8))` |
| `drift` | dust motes, wind and draught streaks, air flow | `a = (u₁ − ½)·spread` around ahead, `v = speed·(0.5 + 0.5u₂)`, `m = 3·sin(0.5τ + u₃)` across the path; `x = ox + facing·(v·cos a·τ − m·sin a)`, `y = oy + v·sin a·τ + m·cos a` | `0.6 + 0.6u₄`; `a` (`½ − a` facing left); `smoothstep(s ÷ 0.3)·(1 − smoothstep((s − 0.7) ÷ 0.3))·(0.6 + 0.4u₄)` |

`beam` of MECH is not a particle (a state's overlay clip draws it). Every motion mirrors exactly about the origin for a pet facing left (x mirrored, y, scale, opacity, age unchanged).

### Births, caps, end

- Continuous (`fall`, `rise`, `drift`): particle `index` is born at `bornAt(…)`, only while that tick lies before `until`, lives `lifeTicks` ticks. At `tick` only the indices `⌊(tick − since − life) ÷ period⌋ … ⌊(tick − since) ÷ period⌋` are looked at (at most `⌈life ÷ period⌉ + 2`). Births keep the order of their indices; nothing is stored.
- **Per emitter**: when one more than `count` is alive (the jitter allows it), the eldest is left out. **Stage-wide**, `capped(particles, cap)`: the `cap` youngest stay (whoever is nearest to its birth is dropped last); among equals the earlier in the list stays, so emitters listed first keep theirs; survivors keep their order; under the cap the very list comes back. An orbit's particles are as old as the orbit, so long-running states give way to fresh bursts.
- `emitterEnds`: `burst` → `since + life`; `orbit` → `max(since, until + 8)`; continuous → `until + life − 1` (`since` when `until ≤ since`); `null` while `until` is `null` (except a burst); `since` for an emitter without particles. It is the first tick from which nothing is alive (proved against the simulation for five keys per vector, and by sweeps in the unit suite); for continuous emitters it is at most two periods after the last particle actually dies.

Measured (`bun TK/probe_effects.ts`, speed 60, spread ¼, until 2000): `count 6, life 1 s` → period 11, at most 6 alive, mean 5.74; `count 32, life 1.6 s` → period 4, at most 27, mean 25.5; `count 32, life 0.25 s` → 16 (the life holds only 16 ticks). A burst with `life 1.6 s` sinks about 150 px by gravity — reduce `BURST_GRAVITY` if sparks should float.

## 3. `🪄️mischief`

### Exports

```ts
fits(ground: string, key: string): boolean                   // equal, or ground + "/" begins key; "" covers nothing
fixtureFor<Item extends { key: string }>(grounds, fixtures: readonly Item[]): Item[]   // survey order, each once; reads only `key`
chosenFixture<Item>(candidates: readonly Item[], unit: number): Item | null           // candidate ⌊unit·count⌋ held inside the list; NaN → first
type Circumstances = { permitted, fine, width, mode: PetMode, quiet, lifting, tick, stirred, rested }
patienceOf(quiet): Ticks                  // 768 (12 s) or 1920 (30 s)
cooldownOf(mode): Ticks | null            // calm 11520 (3 min), lively 2880 (45 s), still null
allowedFrom(c): Ticks | null              // max(stirred + patience, rested + cooldown), null when consent, fine pointer, width ≥ 1024, a lively mode or no lift in progress is missing
allowed(c): boolean                       // allowedFrom(c) !== null && tick ≥ it
type Station = { footing: "perch" | "wall", wall: string | null, surface, x, y, side: 1 | -1, room }
stationFor(fixture, pitches: readonly Pitch[], perches: readonly Perch[], width): Station | null
type Lift = { dx, dy, tilt: Turns, opacity };  NO_LIFT = { 0, 0, 0, 0 }
liftAt(since, tick, side: 1 | -1, room, span, unit): Lift
liftEnds(since): Ticks                    // since + LIFT_TICKS
liftWake(since, tick): Ticks | null       // next tick to look again: since before the start, tick + 1 while moving, since + LIFT_RETURNS while resting, null when over
thrownOff(pusher: Point, fixture, unit): Toss   // Toss of 🧗️climbing
```

Constants (exported, documented): `MISCHIEF_WIDTH 1024`, `MISCHIEF_PATIENCE 768`, `MISCHIEF_PATIENCE_QUIET 1920`, `MISCHIEF_COOLDOWN_CALM 11520`, `MISCHIEF_COOLDOWN_LIVELY 2880`, `STATION_GAP 24`, `STATION_SLACK 2`, `STATION_STEP 12`, `LIFT_ROOM 12`, `LIFT_BRACE 24`, `LIFT_SHOVE 40`, `LIFT_WOBBLE 48`, `LIFT_HOLD 512`, `LIFT_RETURN 56`, `LIFT_FADE 8`, `LIFT_RETURNS 624`, `LIFT_TICKS 688` (10.75 s), `LIFT_LIMIT 1280` (20 s, asserted ≥ `LIFT_TICKS`), `LIFT_LEAST 0.6`, `LIFT_GIVE 1.5` px, `LIFT_RISE 2` px, `LIFT_TILT 0.004` turns, `LIFT_WOBBLES 2`, `HALF_TURN_RADIANS`, `THROW_SPEED 120`, `THROW_SPREAD 80`, `THROW_LIFT 220` px/s, `NO_LIFT`.

### Rule 7 (no answer leaks), as built and as tested

The pick has exactly two inputs: the candidates (selected by `fits` on keys alone) and a unit the caller drew; `chosenFixture` reads no property of any candidate and `fixtureFor` reads only `key` (unit suite: Proxies record every property read — `["key"]` and nothing; `chosenFixture.length === 2`). Case `leaks` and a unit test hand over whole host descriptions with `value`, `correct` and `answered` per item and permute those three among the items (12 committed permutations × 9 descriptions; 4 × 200 drawn ones): the pick never changes, while a leaky pick (first correct candidate) is shown to change in the same sweep. Three leak mutants (prefer a correct item, prefer an answered item, order by value) are killed.

### Station

For a fixture `[left, right] × [top, bottom]`: a perch with `top < y ≤ bottom + 12` that reaches within 24 px of one end (x = its point nearest that end), or a free wall stretch (`Pitch` of `🏞️terrain`) on the matching side (`side −1` = left edge, the fixture's left end 0…24 px inside it, 2 px slack) that overlaps the fixture's height (feet `y = clamp(bottom, y0, y1)`). A pusher on the left shoves right (`side 1`, `room = width − right`), on the right shoves left (`room = left`). Rooms below 12 px never count. Most room wins; ties: perches before walls, then list order.

### Lift path (`liftAt`)

`travel = max(0, room)·(0.6 + 0.4·unit)`, `lean = min(0.004, 2 ÷ (π·span))` turns (a copy's end rises 2 px at most). Ages: 0–8 appear lying on the element (opacity smoothstep); 8–24 give 1.5 px towards the pusher and back (half sine); 24–64 shove (`smoothstep`, dy −2·sin, tilt +lean·sin); 64–112 wobble twice (`dx = travel + 1.5·rock`, `dy = 1·sin(2·2·p)·calm`, tilt −lean·rock, `rock = sin(2p)·(1 − smoothstep p)`); 112–624 rest exactly at `travel`, level (nothing changes: `liftWake` lets the stage sleep); 624–680 slide home; 680–688 vanish lying on the element. Bounds asserted for 400 drawn lifts at exhaustive: |dy| ≤ 2, |tilt| ≤ lean, no jump larger than 0.6 + 1.5·room ÷ 40 px per tick, exact mirror for `side −1`.

## 4. Verification (commands and real results)

All from the repository root unless a `cd` is shown.

| # | Command | Result |
|---|---|---|
| 1 | `.venv/Scripts/python.exe TK/generate_effects_vectors.py` | `births: 18 vectors, 61 ticks at which an emitter's own cap bites`, `motions: 22 vectors, 655 particles placed`, `gates: 70 occasions, 13 allow`, `stations: 28 fixtures, 18 with a station`; final run: `unchanged` for both fixtures |
| 2 | `bun TK/compare_effects_vectors.ts` | 14 scenarios, `0 differences` each, exit 0 |
| 3 | `cd TEST && bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "✨️particle-motion"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=0/0` |
| 4 | same, `subject exhaustive --implementation typescript … --case "✨️particle-motion"` | `executed=6 passed=6 failed=0 errored=0` |
| 5 | same, `parity exhaustive --implementation typescript … --case "✨️particle-motion"` | `executed=12 passed=12 failed=0 errored=0 parity=6/6` (the first run said `parity=5/6`: `periodOf` of a `count 0` emitter was 63 instead of 64 — fixed) |
| 6 | `oracle exhaustive … --case "🪄️mischief-choice"` | `executed=8 passed=8 failed=0 errored=0 parity=0/0` |
| 7 | `subject exhaustive --implementation typescript … --case "🪄️mischief-choice"` | `executed=8 passed=8 failed=0 errored=0` |
| 8 | `parity exhaustive --implementation typescript … --case "🪄️mischief-choice"` | `executed=16 passed=16 failed=0 errored=0 parity=8/8` |
| 9 | `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test --reporter=json` | last run (01:01): exit 0, `967 passed, 0 failed`, 15 files, 9.8 s wall on a loaded host (budget 15 s); test time ✨️effects 32 tests 351 ms, 🪄️mischief 31 tests 230 ms (both alone: 63 tests, 0.5–0.6 s) |
| 10 | `bun ./📜️script.ts test quick --reporter=json` | last run (01:03): exit 0, `968 passed, 0 failed`, 15 files, 18.3 s wall; test time ✨️effects 1.7 s, 🪄️mischief 0.8 s. (An earlier run at 00:47 had 3 failures, all in the siblings' `🚧️clearance` and `🧗️climbing` suites, then in progress.) |
| 11 | `bun ./📜️script.ts test long "✨️effects/" "🪄️mischief/"` | `Tests 63 passed (63)` |
| 12 | `bun ./📜️script.ts test exhaustive "✨️effects/" "🪄️mischief/"` | `Tests 63 passed (63)`, 45 s of test time under v8 coverage |
| 13 | `bun ./📜️script.ts typecheck` | exit 0, 0 errors (the package `tsc --listFilesOnly` includes all six new TypeScript files) |
| 14 | `bun TK/mutate_effects.ts` | `untouched copy: unit suites pass`, `75 of 75 mutants killed` (twice; the second after the last edit of `chosenFixture`) |
| 15 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py <12 files of this package>` | `0 finding(s) in 12 file(s)`; every TypeScript docstring emoji carries U+FE0F; no `console`, no `[DEBUG]` |
| 16 | `bun TK/probe_effects.ts` | the five published words reproduced; `unit(mix(12345, i))` over 200 000 indices: mean 0.4999, variance 0.0831; lift: 688 ticks, farthest 33.4 px for room 40, highest 2.000 px, most tilt 7.2e-4 turns for an 883 px row |
| 17 | `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 as for every owner: `6252 high-priority breach(es) across 5 rule(s)`, the repository-wide backlog; 7 lines name the pets tree, all of them `🧪️tests/🚧️clearance-proof/🥒️.feature` (A5's, steps outside a scenario at that minute); the full breach set `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` has **0** entries for `✨️particle-motion`, `🪄️mischief-choice`, `✨️effects` or `🪄️mischief` |

Third-party evidence per scenario: numpy `uint32` arithmetic (hash, five golden words checked first), a tick-by-tick pool simulation in Python (births and lives; the subject is stateless), `numpy.sin/cos/clip` and the rational decay held to `numpy.exp` within 0.02 (motions; burst paths within 2 % of the reach under a true exponential drag — also checked in the unit suite with `Math.exp`), stable `numpy.argsort` (caps), Python's `str.startswith` (matches), `numpy.floor/clip` (choices), `numpy.random.Generator.permutation` (leaks), vectorised booleans (gates), `numpy.argmax` (stations), branch-free `numpy.interp` ramps (lifts). Uniformity: sums and 16-bin histograms of 10 runs (5 × 200 000 indices) compare exactly; the oracle also holds mean, variance and chi-square (≤ 45 for 15 degrees of freedom) of every long run.

## 5. Notes for the Rust twin

- `lowbias32`: `x ^= x >> 16; x = x.wrapping_mul(0x7feb352d); x ^= x >> 15; x = x.wrapping_mul(0x846ca68b); x ^= x >> 16` on `u32`. `mix(a, b) = lowbias32((a ^ 0x9e3779b9).wrapping_add(b.wrapping_add(1).wrapping_mul(0x85ebca6b)))`. `unit` = `unit_of` of randomness (`word as f64 / 4294967296.0`).
- Integers: `life_ticks = max(1, floor(life·64 + 0.5))`; `period = max(1, (life + s − 1) / s)` with `s = max(1, swarm)` in integer division; the window's lower index is 0 while `elapsed < life`, else `(elapsed − life) / period`; births `since + index·period + floor(unit·period)`. Ticks and indices stay far below 2⁵³; `index` passes through `mix` as `u32`.
- Floats: evaluate exactly in the written order (`facing * (a + b)` with `facing` as `±1.0`), no `mul_add`, `0.0 - x` where TypeScript writes `0 - x`; `toward(side, v)` is `if side > 0 { v } else { 0.0 - v }`. `rotation` of a `rise` with `spread 0` and of some `fall`s can be a negative zero in both twins; a digest of bit patterns sees the same −0.0 if the order is kept.
- `capped`: sort the ages (integers), take the `room − 1`-th as the eldest kept, keep everything younger plus the first `room − (younger count)` of that age in list order; return the input unchanged when it fits.
- `fits` on bytes: `!ground.is_empty() && key.starts_with(ground) && (key.len() == ground.len() || key.as_bytes()[ground.len()] == b'/')` (equivalent to the UTF-16 test for valid strings).
- `chosen_fixture`: `place = (unit * count as f64).floor()`; `place >= count → count − 1`, `place > 0 → place`, else 0 (NaN → 0).
- The committed vectors compare positions within 1e-9; there is no `bit-patterns` scenario yet (a mutant that only reassociates a product would survive). Add one when the twin lands (restate in Python, project `struct.pack(">d")` hex as in `📐️turn-trigonometry`).
- Adapters `🦀️.rs` for both cases are missing (phase D); the TypeScript adapters show the projection shapes (`null` for an open end, `-1` for no candidate).

## 6. Notes for the stage integrator (B3 projection, B5 mischief)

- **Particles in `frameOf`**: per visible actor and per running emitter (the state's, the running trick's, the purr's), `origin` = the bone's world matrix applied to `(emitter.x, emitter.y)` plus the feet, mirrored with the actor; `since` = the tick the state, trick or purr began; `until` = its end or `null` while it lasts; `key = emitterKey(stage.seed, speciesIndex, emitterIndex, since)` (no stage draw, no new stream). Concatenate in the order actors and emitters are drawn, then `capped(all, STAGE_CAP)`, then map to `ParticleFrame` (drop `age`). Nothing in `still` and under reduced motion.
- **Horizons (X4)**: while any `particlesOf` is non-empty or an emitter has not reached `emitterEnds`, the rate must stay above 0; `emitterEnds` is the wake horizon of a stopped emitter (`null` = it runs as long as its state). A burst fires at `since` whatever `until` says: a trick that starts a burst and is cut short still shows it.
- **Mischief**: candidates = `fixtureFor(species.grounds, surveyed.fixtures)` filtered by `stationFor(f, pitchesTheSpeciesCanClimb, perches, stage.width) !== null`; pick with `chosenFixture(candidates, unit)` from a stage draw. `allowed({ permitted: stage.mischief, fine, width: stage.width, mode, quiet, lifting, tick, stirred, rested })` gates the start; `allowedFrom` is its time horizon. `fine` must come from the shell (fold "(pointer: fine)" into `permitted.mischief`, or add it to an event). `rested` should start at the tick a scene opens so the first lift waits a cooldown too.
- **Lift**: `room = min(station.room, species.size.width)` ("up to a body width"), `span = fixture.width`, `unit` from a stage draw; the pusher's feet sit at `station.x − station.side·(half its width + margin)` on a perch or on the wall at `station.y`, facing `station.side`. `liftAt` is the frame's `LiftFrame` (add `fixture: id`); `liftWake` is the wake while it rests. To put back early (pusher picked up, mode change), move `since` to `tick − LIFT_RETURNS` from the rest; on `reclaimed` drop the lift at once and give the pusher `thrownOff(pusher, fixture, unit)` (parachute rule applies).

## 7. Notes for the shell (C1 `🪞️lifting`) — what a lift expects

- While a `LiftFrame` exists, draw the copy at the element's surveyed box translated by `(dx, dy)` and rotated by `tilt` turns about its centre, with `opacity`. **Keep the original opaque while `opacity < 1` and make it transparent exactly while `opacity === 1`**: the copy fades in and out lying exactly on the element (`dx = dy = tilt = 0` in those 8 ticks), so the swap never shows a hole or a double offset.
- `dy` never exceeds ±2 px and the tilt never lifts a corner by more than 2 px, so the copy stays in its row; `dx` reaches `room` at most, plus 1.5 px.
- Reclaim on the original (pointer over it, pointer down, focus, key, input, change, drag start, and in a run any input): restore in the same task, send `reclaimed`. The core keeps the copy out of the stack for 8 s at most at rest and 10.75 s in all; nothing is undone by a transition.
- The pick never needs a value, a correctness flag or an answer: do not put them on fixtures, and do not order or select `data-pet-prop` elements by them (for example results rows by `data-earned`) — the survey's order and its eligibility rules are inputs of the pick.

## 8. Deviations and decisions

- **`spread` is a fraction of a turn**, as A2's schema fixes it, not the pixel width MECH's rain and snow use. All particles start at the emitter's point; snow gets its width from its slow sway, rain from a narrow fan. Steam's growth (`0.6 + 0.8s`) and the rain splash of MECH are not built: a splash is a second `burst` emitter the stage starts on a landing.
- **`orbit` reads `life` as the time of one lap** and derives the radius from `speed` (`R = speed·lap ÷ 2π`); MECH ties the radius to the body (`0.45 w`). Authors set the radius through `speed`.
- **`drift` is continuous births around "ahead"**, not MECH's persistent wrapping lanes (no pop at the wrap, same cap rules as the other continuous motions).
- **`unit` is the randomness module's `unitOf`** re-exported, so a word still becomes a unit in one place.
- **`stationFor` takes the stage width** as a fourth argument (the brief named three): the room beyond the far end decides the side and the shove.
- **`fixtureFor` returns every fitting fixture** (the brief's name kept; it is a list) and is generic over anything with a `key`; `chosenFixture` is generic and reads nothing.
- **The stand-in `Fixture` type is module-private** (like `Wall` in `🏞️terrain`): replace it by the schema's `Fixture` once the survey event carries fixtures. `Toss` is `🧗️climbing`'s (identical shape).
- Rounded choices recorded in docstrings: an emitter keeps the youngest `count`; the stage keeps the youngest 160, ties by list order; `chosenFixture` of NaN is the first candidate; `allowedFrom` is `null` for a still stage.

## 9. Open

- Rust twins `🦀️.rs` of both modules, their unit suites and the two case adapters (phase D); a `bit-patterns` scenario then.
- Tuning by eye (stories gallery, A9's preview tool): `BURST_GRAVITY`, fall sway, the lift's timings.
- `P/README.md` does not yet name the two modules (A2 owns it).
- No ticket bookkeeping from this package: the repo MCP server was unreachable in this session (`CONNECTION_CLOSED`).
