# 📓️ Work package A5 — clearance (second round): the machinery that keeps pets apart

Ticket `2026/10/02/QUIZ-PETS`, design-v2 §14 decision 3, §15.2, §18, §23; research2-mechanics §6 and Appendix A.3. Paths: `P = 🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST = 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. TypeScript only; the Rust twin is phase D. No stage file was touched (integration is B1).

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🚧️clearance/🟦️.ts` | the pure module (617 lines, 13 numeric constants plus `UPRIGHT` and `TALLY`, 10 types, 39 exported functions), no dependency besides the schema twin |
| `P/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts` | vitest suite, 66 tests: cases from the design, brute force on the 1/64 px lattice, exhaustive pooling, fuzzed walkers, and the **reference world** (exported: `RULES`, `openWorld`, `tickWorld`, `runWorld`, `bodiesOf`, types `Rules`, `World`, `Counts`) |
| `P/🧪️tests/🚧️clearance-proof/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 11 scenarios, oracle `pets-scipy` (with numpy), comparison `pets-float-v1` |
| `P/🧫️fixtures/🚧️clearance-proof/🔣️.json` | 377 vectors (285 KB), generated, never hand-edited |
| `P/🔮️oracles/🔣️.json` | anchored edits: capability `pets-clearance-proof` on `pets-scipy`, its engine names `isotonic_regression` and `minimize (SLSQP)`, one rationale paragraph |
| `P/📦️packages/🟦️typescript/🟦️.ts` | one line `export * from "../../🔨️modules/🚧️clearance/🟦️.ts"` (no name collides; see §8.6) |
| `TK/generate_clearance_vectors.py` | the vector generator (a second run is byte-identical, sha256 `515ab8c5…d2a5`) |
| `TK/clearance_world.ts` | runs the reference world for metrics, ablations, teeth search and traces (imports the suite with an idle Vitest stand-in) |
| `TK/clearance_trace_check.py` | judges recorded world traces with numpy, independently of `overlaps` |
| `TK/clearance_vectors_check.ts` | holds the TypeScript subject to the committed vectors number by number (stands in for `parity`, which needs the Rust adapter) |
| `TK/a5_docstring_check.mjs` | docstring emojis (present, unique per file, no `@`), no console / `print` / `[DEBUG]`, no comments inside definitions |

Directory names were copied from `bun TK/taxonomy_name_probe_v2.ts` (`🚧️clearance`, `🚧️clearance-proof` were already registered by A2: "0 fresh, … already present").

## 2. Exports

Types: `Extent = { x0, y0, x1, y1 }` (edges, y down) · `Body = { owner: Slug; extent: Extent }` · `Posture = { left, right, above }` · `Pair = { first: Slug; second: Slug }` · `Slice = { from: Ticks; until: Ticks; extent }` (both ticks included) · `Claim = { owner; slices: Slice[]; rest: Extent | null }` · `Seat = { owner; shift }` · `Seating = { seats: Seat[]; leavers: Slug[] }` · `Slide = { stride; side }` · `Tally = { ticks, actors, overlaps, nears, poofs, waits }`.

Constants: `MARGIN = 4`, `SEAM = 1/64`, `FOREVER = 2³²−1`, `STEERINGS = [0, 30, −30, 60, −60, 90, −90, 130, −130, 180, −180, 240, −240]` (px/s, order tried), `PUSHES = 4`, `PATIENCE = 40`, `DELAY = 32`, `SLIDE_OFF_SPEED = 32`, `SLIDE_OFF_GAIN = 2` (px/s per tick), `SLIDE_OFF_LIMIT = 128`, `LEAN = 0.25`, `LANE_LIFT = 0.9`, `SCOOT_HASTE = 1.5`, `UPRIGHT`, `TALLY`.

| Group | Signature |
|---|---|
| bodies | `leaning(height, sine): Posture` · `canopied(size, canopy): Posture` · `bodyOf(owner, feet, size, hover, posture, margin): Body` · `shifted(extent, dx, dy)` · `grown(extent, margin)` · `united(one, other)` · `meets(one, other): boolean` · `overlaps(bodies): Pair[]` · `nearMisses(bodies, margin): Pair[]` |
| free places | `obstaclesOf(owner, bodies, claims, tick): Extent[]` · `freeAmong(extent, obstacles)` · `freeAt(extent, owner, bodies, claims, tick)` · `slotIn(x, extent, low, high, obstacles): number \| null` · `spotOn(perch, x, extent, foot, obstacles)` · `columnOver(perch, x, extent, drop, foot, obstacles)` |
| claims | `claimOf(owner, from, extents, span, rest): Claim` · `sliceAt(claim, tick): Extent \| null` · `claimClear(claim, bodies, claims): boolean` · `released(claims, owner)` · `pruned(claims, tick)` |
| order on a perch | `guardedStride(extent, stride, obstacles): number` · `vaults(hopper, rise, hurdle)` · `orderOf(bodies): Slug[]` · `orderKept(before, after)` |
| seating | `seatOf(bodies, perch, margin): Seating` · `scootFraction(shifts, stride)` · `scooted(from, to, fraction)` |
| heads | `headUnder(before, after, owner, bodies): Body \| null` · `liftOnto(extent, host): number` · `restsOn(rider, host)` · `slideSide(rider, host): 1 \| −1` · `slideStride(ticks)` · `slideOf(rider, side, ticks, low, high, obstacles): Slide` |
| held | `pushedOut(extent, obstacles, iterations): Point \| null` |
| last resort | `mustPoof(owner, stay, plans, bodies, claims, tick)` · `evicted(bodies, claims, tick): Slug[]` |
| lanes, metrics | `laneLift(height, margin)` · `tallied(tally, bodies, margin, poofs, waits): Tally` · `perMillion(count, actors)` |

Only `+ − × ÷`, `abs`, `floor`, `min`, `max` and comparisons (a unit test reads the source and finds exactly `abs, floor, max, min`); no `Map`/`Set`, every sort a stable insertion, lists walked in their order; nothing mutates an input (deep-frozen test).

## 3. The rules as implemented

1. **Body** (`bodyOf`): `x0 = x − w/2 − left − m`, `x1 = x + w/2 + right + m`, `y0 = y − h − above − m`, `y1 = y + hover + m` (feet = the actor's `x, y`; a floater's hover reaches down to its perch). `leaning(h, sine)` adds `0.25·h·|sine|` per side (held at full tilt: sine 1), `canopied` the canopy above the head and its overhang. **Invariant**: `overlaps(bodies) = []` with `m = MARGIN = 4` (design-v2 §18: touching bodies keep 8 px between their size boxes); overlap = open boxes intersect (strict), so touching is apart.
2. **Seam.** Predicates are exact; every placement (`slotIn`, `guardedStride`, `seatOf`, `pushedOut`, `liftOnto`, `slideOf`) leaves `SEAM = 1/64 px`, so the rounding of sums of coordinates (≈1e-13) can never turn placed bodies into overlapping ones. Resting neighbours stand 8 + 1/64 px apart.
3. **Free from a tick on** (`obstaclesOf`/`freeAt`): other bodies, every slice of another claim with `until ≥ tick` (the rest of a plan, not just the next tick: research bug "a mover that ignored the corridor of a faller whose plan had not started yet"), and every other claim's `rest` (reserved landing spots).
4. **Nearest free place** (`slotIn`): carve the open interval `(x + o.x0 − e.x1 − SEAM, x + o.x1 − e.x0 + SEAM)` of every obstacle that shares the height out of `[low, high]`; a touched end stays (a single exact-fit point included); nearest, left on ties. `spotOn` = footprint (`foot` = half the species width) on the perch, `columnOver` = the box extended down by `drop` (the whole fall column).
5. **Claims** (`claimOf`): slice `k` covers ticks `from + k·span … from + min((k+1)·span, n) − 1` and is the hull of their boxes; `span = 1` is the path itself, a wider span is coarser and more conservative. `claimClear`: my time line (slices, then `rest` for ever; nothing before `from`) against every other owner's time line — its body from tick 0 until its claim begins (for ever without one), its slices, then its rest (absent when `rest = null`). A conflict is one tick at which both hold boxes that meet. This contains, as special cases, "final spot vs later arrivals" (research bug 4), "a path through somebody's rest", and the planned body that is where its claim says rather than where it stands. One claim per owner is expected; a second claim of an owner is tested on its own (pinned by a vector).
6. **Guarded stride**: only obstacles that share the height and whose middle lies ahead count; the stride stops `SEAM` before the nearest (`max(gap − SEAM, 0)`). An obstacle the box already overlaps blocks the way in, not the way out. Consequence: nobody passes anybody whose height it shares → the order on a perch only changes by leaving it (a hop). `vaults(hopper, rise, hurdle) = hopper.y1 − rise ≤ hurdle.y0 − SEAM` is the necessary precondition of a swap by a hop (tested against real terrain hops: no hop over a neighbour was ever claim-clear while `vaults` said no); the swap itself is just a claim that must be clear.
7. **Seating** (`seatOf`): members sorted by middle (stable); while `Σ widths + (n−1)·SEAM` exceeds `perch.x1 − perch.x0 + 2·margin` the **last in the input list** leaves (priority = list order). With `o₁ = 0`, `oᵢ₊₁ = oᵢ + widthᵢ + SEAM`, `qᵢ = x0ᵢ − oᵢ`: pool adjacent violators on `q` (merge while left mean > right mean, strict), clamp each block mean into `[perch.x0 − margin, perch.x1 + margin − widthₙ − oₙ]`, `shiftᵢ = mean − qᵢ`. A valid seating returns shifts of exactly 0. Footprint rule = box minus margin on the perch (for upright bodies exactly the first round's "size box on the perch").
8. **Scoot**: `scootFraction(shifts, stride) = stride ÷ max|shift|` capped at 1 (1 without way, 0 for stride ≤ 0); `scooted(from, to, f)` lands exactly on `to` at `f ≥ 1`. Every mixture of two valid seatings is valid, so a group moving by one common share never overlaps within itself (tested on generated groups tick by tick).
9. **Heads**: `headUnder` = the body whose top the faller's lower edge comes down onto between two ticks (`before.y1 ≤ top`, `after.y1 > top − SEAM`, not rising, sharing the width at `after`); highest top wins, first among equals; never one it was beside already. `liftOnto` puts the lower edge a seam above the top; `restsOn` = shares the width and the lower edge lies within two seams above the top (checked every tick: a host that walks off, falls or vanishes drops the rider). Slide off to `slideSide` (right when the middle is at or beyond the host's middle), `slideStride(k) = min(32 + 2k, 128)/64 px`, guarded and kept between the stage edges; blocked completely → turn round (`side = −side`).
10. **Held**: `pushedOut` walks the obstacles in order, each overlapping one is left by its shortest axis way (left, right, up, down on ties) plus a seam, at most `iterations` rounds; `null` when it still overlaps → the caller keeps the previous feet and posture. A narrow gap (two obstacles closer than the body) is never escaped upwards by the local rule — it answers `null`, which is legal (keep the place).
11. **Poof** (design-v2 §18, decision 3: no ghost): `mustPoof(owner, stay, plans, …) = !(stay ≠ null ∧ freeAt(stay)) ∧ no plan clear`. `stay` is the box to keep while patience lasts (`DELAY` ticks), `null` after. `evicted(bodies in order of their right to stay, claims, tick)`: a body leaves when it overlaps an earlier body that stays, or — having no plan of its own — a corridor that is left or a rest of a claim whose owner stays.
12. **Lanes**: `laneLift(h, m) = 0.9·(h + m)`; a floater tries its glide at its own altitude, then in the high lane, then waits.

## 4. Proof: the reference world

The world (in the unit suite, driven only by this module plus `🏞️terrain` — `fallStep`, `landingOf`, `hopOf`, `hopStep`, `hopLanding`, `strideTo` — and `🎲️randomness`): a 960 px stage with a floor at 520 and six shelves on 64 px levels; 6–10 pets of widths 28–58 and heights 40–56 (a fifth floaters with hover 6–18, half with a parachute). Sequential update with a rotating first actor; per tick: prune claims, maybe a survey (1/300 per tick: a shelf vanishes, shrinks to 40–80 %, jumps by up to ±60 px in x and y, or comes back), the hand (picks **any** visible pet every ~120 ticks — standing, walking, flying or riding — drags it for 20–120 ticks at 4–14 px/tick half of the time straight into another pet, then throws it with the hand's speed or drops it), the seated groups scoot, then every pet acts: walkers (`guardedStride` vs `obstaclesOf`; goals cut by `guardedStride` too; patience 40), hops (`spotOn` + terrain arc + `claimOf` + `claimClear`), glides of floaters (two lanes), falls without footing (candidates: as it flies, towards `columnOver`, the `STEERINGS`, and for a thrown pet a bounce at half speed and let-go with air control; landings on perches by `landingOf` on footprint-narrowed perches, on heads by `headUnder` + `liftOnto`; a parachute opens at 480 px/s and the body becomes `canopied`; waiting up to `DELAY` ticks, then `mustPoof`), riders (`restsOn`, `slideOf`), held pets (`leaning` posture, `pushedOut`). Surveys: riders of a moved shelf ride it rigidly (head riders with their hosts), `evicted` (unmoved first) names who poofs, riders of a vanished shelf lose footing, every airborne plan is re-planned as a fall with its current velocity, the shelf is re-seated by `seatOf` (leavers off the end fall, others poof) and scoots by `scootFraction`. After every tick: `tallied` (invariant + near misses), the order of every perch (`orderKept` for pets grounded in both ticks).

Rules as switches: core `guard`, `corridors`, `vetting`, `rests`, `projection`, `eviction`, `poof`; quality `heads`, `steering`, `seating`.

### 4.1 Metrics (all rules; `bun TK/clearance_world.ts --seeds 24 --ticks 30000 --ablation-seeds 12 --ablation-ticks 15000 --name final`, 12.4 s for the full set)

| run | ticks | actor-ticks | **overlap ticks** | order breaks | poofs (stranded / evicted / crowded) | poofs per million actor-ticks | waits (walker stalls / airborne delays) | waits per actor-tick |
|---|---|---|---|---|---|---|---|---|
| 24 seeds × 30 000, slices of 1 tick | 720 000 | 5 908 023 | **0** | 0 | 20 (2 / 16 / 2) | 3.4 | 231 553 (230 994 / 130) | 0.039 |
| 24 seeds × 30 000, slices of 4 ticks | 720 000 | 5 907 127 | **0** | 0 | 24 (3 / 19 / 2) | 4.1 | 245 870 | 0.042 |
| 100 seeds × 30 000 | 3 000 000 | 24 320 043 | **0** | 0 | 88 (6 / 72 / 10) | 3.6 | 966 008 | 0.040 |

Activity in the 24-seed run: 2 400 surveys, 3 796 grabs, 9 145 planned falls (4 780 ending on a head, 784 steered, 1 168 under a canopy), 915 hops, 821 glides (143 in the high lane), 18 129 travel attempts refused because no clear plan existed, 102 pets re-seated with a move, 11 spilled off a shrinking end. Near-miss ticks: any pair within 2 px 655 267 (91 % of ticks — resting neighbours stand a seam apart by design, riders a seam above heads); a *flying* body within 2 px of another 38 037 ticks (5.3 %; landings next to a neighbour and slides down beside a host). Research comparison: the prototype's hard check was margin 0 and its "near miss" margin 4; here the hard check is the margin-4 body itself, i.e. the prototype's near-miss level never happens.

### 4.2 Ablations (12 seeds × 15 000 ticks = 180 000 world ticks each)

| rule switched off | overlap ticks | order breaks | poofs | note |
|---|---|---|---|---|
| none | 0 | 0 | 9 | |
| guard (walkers unguarded) | **145 251** | 2 274 | 225 | |
| corridors (walkers ignore claims) | **11 089** | 0 | 29 | |
| vetting (plans taken unchecked) | **22 503** | 0 | 32 | |
| rests (claims name no rest: final spot vs later arrivals) | **442** | 0 | 11 | research bug 4 |
| projection (held pet follows the hand blindly) | **23 690** | 0 | 378 | |
| eviction (carried riders stay) | **346** | 0 | 4 | |
| poof alone | 0 | 0 | 9 | stranded pets are this rare with heads and air control (2 in 720 000 ticks) |
| heads | 0 | 0 | 238 | 26× the poofs |
| steering | 0 | 0 | 121 | 13× the poofs |
| seating | 0 | 0 | 11 | 38 pets spill off a shrinking end instead of 3 |
| heads and steering | 0 | 0 | 557 | the poof keeps them apart … |
| heads, steering and poof | **73 001** | 0 | 75 | … and without it they overlap |

Teeth search (`--teeth 40 --ticks 2500`, first overlapping tick per seed) gives the cheap runs the fundamental level uses: guard seed 12 tick 24, corridors seed 9 tick 88, vetting seed 23 tick 23, rests seed 21 tick 81, projection seed 6 tick 7, eviction seed 17 tick 21.

### 4.3 Independent judgement of traces

`bun TK/clearance_world.ts --seeds 3 --ticks 10000 --trace 3 --name traced` and `… --seeds 2 --ticks 3000 --trace 2 --without guard --name unguarded`, then `.venv/Scripts/python.exe TK/clearance_trace_check.py` (numpy, shared-area test): `trace-traced-{1,2,3}`: 30 000 ticks, 239 809 bodies, 0 overlap ticks (module 0, agree); negative control `trace-unguarded-1/2`: numpy 2 270 and 1 885 overlap ticks with 4 066 and 3 191 pairs, module the same (agree). The unit suite repeats this check inside vitest (`is judged alike by an overlap test that measures the shared area`).

## 5. Protocol v2 case `🚧️clearance-proof`

Scenarios (all `@level-fundamental`): `constants` (conformance), `bodies` 11, `crowds` 22, `spots` 53, `strides` 45, `orders` 14, `claims` 55, `seatings` 49, `heads` 53, `pushes` 41, `resorts` 34 vectors. Coverage counted by the generator: 36 clear / 19 refused plans, 19 seatings with leavers, 45 seatings that move, 7 landings on a head, 4 pushes that fail, 17 evictions. Every coordinate is on the quarter-pixel lattice, so every answer lies on the 1/64 px lattice and the references search it: free places and strides by lattice searches (`numpy.argmin`/`argmax`), overlaps by the shared-area test (broadcast `minimum`/`maximum`, `argwhere`), claims by dense time lines (`numpy.minimum.reduceat`/`maximum.reduceat` hulls, one box per tick per owner), heads by boolean masks, pushes replayed and judged (free afterwards; for one obstacle the shortest axis way found on the lattice). Seatings are answered as the binary64 result of the pooling written out in the oracle and refused unless `scipy.optimize.isotonic_regression` (clipped) agrees within 1e-9 and `scipy.optimize.minimize` (SLSQP, accepted at status 0 or 8 when every constraint holds within 1e-5) within 1e-5. shapely and scikit-learn are not in the harness environment (declined in the oracle rationale).

## 6. Commands and real results (2026-10-03)

| Command | Result |
|---|---|
| in `P/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | exit 0, `Test Files 15 passed (15)`, `Tests 968 passed (968)`, Duration 5.59 s (wall 7.4 s); the clearance suite alone at this level: 66 tests, 0.67 s of test time |
| `bun ./📜️script.ts test quick` | exit 0, 15 files, 968 tests, 14.95 s |
| `bun ./📜️script.ts test exhaustive` | first run exit 1: a concurrent exhaustive run of another agent removed the shared coverage directory (`Something removed the coverage directory …`) and the gesture/mischief suites of siblings failed; rerun: exit 0, `Test Files 15 passed (15)`, `Tests 972 passed (972)`, 205 s. The clearance suite alone at exhaustive (`SEMIO_TEST_LEVEL=exhaustive bun x vitest run … clearance`): 66 passed, 68 s (world 24 × 30 000 ticks 12.8 s, ablation sweeps 2.3–3.4 s each) |
| `bun ./📜️script.ts typecheck` | exit 0, 0 errors (the whole package, with the glue line) |
| repo root: `.venv/Scripts/python.exe TK/generate_clearance_vectors.py` | exit 0, counts as in §5; a second run byte-identical (sha256 `515ab8c5b7ee727b42e1930dbc829565ce43b38d86b676d524366e49a375d2a5` before and after) |
| in `TEST`: `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🚧️clearance-proof"` | `[test] level=exhaustive cases=1 executed=11 passed=11 failed=0 errored=0 parity=0/0` (also at `fundamental`: 11/11) |
| in `TEST`: `bun ./📜️script.ts subject exhaustive --implementation typescript --owner … --case "🚧️clearance-proof"` | `[test] level=exhaustive cases=1 executed=11 passed=11 failed=0 errored=0 parity=0/0` (the harness records projections here and compares only under `parity`) |
| repo root: `bun TK/clearance_vectors_check.ts` | `scenarios=11 vectors=377 numbers=7563 bit-exact=7563 differences=0` |
| in `TEST`: `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with the repository-wide backlog (`6245 high-priority breach(es) across 5 rule(s)`); no breach names the pets tree (the full set `⚡️cache/breaches/testing.json` contains neither `clearance` nor `🐾️pets`). A first run flagged my feature's bullet lines (`* …` is a Gherkin step); rewritten as paragraphs |
| repo root: `node TK/a5_docstring_check.mjs` | exit 0: every docstring of the 9 files starts with an emoji, none repeated, no console/`print`/`[DEBUG]`, no comments inside definitions |
| world, traces, teeth | as in §4 |

Not run, as instructed: `parity` (no Rust adapter yet), cargo, e2e, deploy checks. No modifying git command.

## 7. For the stage integrator (B1): which call belongs where in a tick

Bodies: one helper in the stage builds `Body[]` of every visible actor in actor order with `bodyOf(species, feet, size, hover, posture, MARGIN)` — posture `UPRIGHT`, `leaning(h, sin tilt)` while held/tumbling, `canopied(size, canopy)` under an open chute, the aim/climb widenings of A3/A4 as a plain `Posture`. A fading-in newcomer is a body; a poofed (vanished) actor is not. The stage keeps `claims: Claim[]` (at most one per actor) next to the actors; they are runtime state (all three schema twins).

1. **Start of a tick**: `claims = pruned(claims, now)`.
2. **Events before the tick** (`👥️population`, `👀️attention`):
   - `surveyed`: (a) every actor with a claim: `released`, then "no footing" with its current velocity (re-planned in its turn — P3); (b) grounded riders ride their surface rigidly (and riders standing on their heads with them); (c) `evicted(bodies ordered unmoved first, then moved; claims; now)` → poof each named; (d) riders of a vanished perch: no footing; (e) per changed perch: `seatOf(bodies of its grounded actors in priority order, perch, MARGIN)` → leavers whose footprint is off the perch get no footing, the others poof (crowd out); if any shift ≠ 0 every member gets the goal `x + shift` and the activity `scoot`. This replaces the first round's `carry`/`seat` teleports.
   - `pressed` that picks an actor up: `released(claims, actor)`, footing `hand`. `released`: footing `air` with the release velocity (it plans in its turn). `cancelled`/Escape: the same with zero velocity.
3. **Group scoots** (before the actor loop, per perch): `f = scootFraction(goals − xs, SCOOT_HASTE × speed ÷ 64)`, candidate `xs' = scooted(x, goal, f)`; commit all only when every member's new box is `freeAmong(…, obstaclesOf(<non-member>, bodies of non-members, claims, now))`, else a wait; after `PATIENCE` waits the group gives up (members off the perch get no footing).
4. **Actor loop** (`🕰️clock`, rotating first actor `tick mod n` as research §6.3 recommends), per footing in `🚶️locomotion`:
   - **perch, walking**: choose goals cut by `x + guardedStride(box, goal − x, obstaclesOf(self, bodies, claims, now))`; per tick `x += guardedStride(box, strideTo(x, goal, speed) − x, obstaclesOf(…))`; shorter than wanted = a wait; `PATIENCE` → new goal (replaces `hindered`/`clearway`).
   - **starting a plan** (hop, glide, wall, ladder, rope — B1/B4): target `spotOn(perch, x, box at the target, foot, obstaclesOf(…))`; the path from the terrain/climbing/swing modules → `claimOf(self, now, boxes per tick, 1, last box)`; take it only when `claimClear(plan, bodies, claims)`, then push it to `claims`; else dwell and retry. A swap over a neighbour: `vaults` first, then the claim. Floaters: second plan with `laneLift(h, MARGIN)`.
   - **air with a plan**: position = the plan's step for `now − from`; on the last step `released(claims, self)` and stand (perch) or footing `head`.
   - **no footing** (perch gone, thrown, slid off, host gone): candidates in order — as it flies, towards `columnOver(perch below, …)`, `STEERINGS`, and for a thrown pet bounce (−vx/2) and let-go with air control; each path: `fallStep` (+ chute), `landingOf` on perches narrowed by the footprint (never clamp a landing x onto a perch end: research bug 1), `headUnder(before, after, self, bodies without claims)` + `liftOnto`; first `claimClear` wins. None: `mustPoof(self, waited < DELAY ? box : null, plans, bodies, claims, now)` → poof (vanish, not a body, `released`, arrive anew later) or wait in place.
   - **head**: `restsOn(box, host box)` else no footing; `slideOf(box, side, ticks, stage left, stage right, obstaclesOf(…))`.
   - **hand**: follow target → box with `leaning`; `pushedOut(box, obstaclesOf(self, bodies, claims, now), PUSHES)`; `null` or outside the stage → keep the previous feet and posture; else shift anchor and bob together by the push.
   - **arrival**: `spotOn(…)` with `obstaclesOf`; none → wait (as today).
5. **End of the tick** (suites, traces, debug builds): `overlaps(bodies) = []`; per perch `orderKept(previous orderOf, orderOf)` over the actors grounded in both ticks; `tallied`/`perMillion` for the published metrics.

Order of tick-level timing used by the claims: `path[0]` is the position at the planning tick (the planner stands still for that tick), `path[k]` the position at the end of tick `from + k`; walkers test against slices with `until ≥ now`, so a corridor whose plan has not started yet is respected too.

## 8. Deviations and decisions

1. **Ghost → poof** (design-v2 decision 3): the research's ghost slide is replaced by `mustPoof` (in the air) and `evicted` (after a survey); no translucent overlap exists.
2. **Hard invariant on the margin-4 bodies** (design-v2 §18) instead of the research's margin-0 check; added the `SEAM` (1/64 px) to every placement so the strict invariant survives rounding. Near misses are reported but are dominated by intended contacts (§4.1).
3. **Shapes of the API**: `Body` nests an `Extent` (`{ owner, extent }`), boxes are edges, not the schema's `Rect` (exact edges, no `x + width` rounding); `headUnder` takes the faller's boxes before and after plus its owner (the brief said feet points — boxes carry width, hover and posture); `pushedOut` answers the translation or `null`; `seatOf` takes `margin` and reads the priority from the list order; `mustPoof` takes `stay` and candidate `plans`; `columnOver` is the "free column for a landing"; `evicted`, `obstaclesOf`, `sliceAt`, `pruned`, `released`, `freeAmong`, `laneLift`, `tallied`, `perMillion` are additions.
4. **Names**: the slide-off constants are `SLIDE_OFF_*` because `🧗️climbing` exports `SLIDE_SPEED`/`SLIDE_GAIN` for walls (now both are in the glue without collision). The type `Body` has the name of `📝️draft`'s internal mutable actor type; `📝️draft` is not in the glue, so nothing collides, but B1 will need an alias where both are imported.
5. **Oracles**: scikit-learn and shapely are not installed; `scipy.optimize.isotonic_regression` (exists in scipy 1.17.1) and SLSQP judge the seating, numpy everything else. SLSQP sometimes stops with status 8 at the optimum (constraints satisfied within 2e-9); accepted when every constraint holds within 1e-5 and the seats agree within 1e-5.
6. **The world lives in the unit suite** (exported); `TK/clearance_world.ts` imports it through a Bun plugin that replaces Vitest with idle stand-ins, so there is one world, not two.
7. **`parity` is not available** before the Rust adapter; `TK/clearance_vectors_check.ts` holds the TypeScript subject to the committed oracle answers (7 563 numbers, all bit-exact).
8. **Ablation of `poof` alone** shows no overlap because, with heads and air control, a pet is stranded only twice in 720 000 ticks; its teeth are shown with heads and steering off (0 → 73 001 overlap ticks). Swap-by-hop is not exercised by the world (terrain hops rarely clear a neighbour from where a walker stalls); it is covered by the unit test against real hop arcs.
9. **Not in the module**: dynamic priorities ("waited ticks raise priority") and "parked pets yield" (research §6.3) — the yield target is `slotIn` against the new claim's slices, documented for B1; the Gipps speed cap is not needed for the guarantee.

## 9. Open

- Phase D, Rust twin: translate expression by expression; `Infinity` in `slotIn` → `f64::INFINITY`; `indexOf`/`splice` → `contains`/`insert`; `FOREVER` as `u32::MAX` (ticks are below 2³²); `Math.min(left, right, up, down)` then the equality chain in `pushedOut`; `0 - x` stays a subtraction; one Rust adapter `P/🧪️tests/🚧️clearance-proof/🦀️.rs` with the same projections, then `parity`. The world could become a scenario with TS/Rust digests then.
- B1: integration as in §7; the published stage metrics (design-v2 §18) can reuse `tallied`/`perMillion`.
- `🗑️generated/a5/` was emptied after these runs.
