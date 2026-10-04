# Report 2 — B4: gear in the stage (walls, ladders, grappling rope)

Ticket `2026/10/02/QUIZ-PETS`, round 2, work package B4. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this
ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Tool output: `TK/🗑️generated/b4/`.
Includes both coordinator notes (edges of the stage as walls for fliers; the unbounded `chainOf`).

## 1. Result

- The pure module `🧗️climbing` (A4) now drives three new footings in the split stage — `wall`, `ladder`, `rope` —
  with the activities `climb`, `mantle`, `slide`, `carry`, `aim`, `reel` (and `shrug` after a miss), on top of B1's
  claims and its no-collision invariant.
- Every gear motion is a **trip**: planned whole (every foothold from the walk to the gear to the end), claimed with
  one `claimOf` corridor, taken only when `claimClear`, travelled foothold by foothold, given up where it is when
  anything interrupts it. Fuzzed over 12.2 M actor-ticks: **0 overlap ticks, 0 actor-ticks outside the stage box,
  0 hard landings of a pet with a parachute**.
- Pets rest on walls (gutter life on pages whose cards leave no room on top), cross the 30 px gaps between the cards
  of a column with a lunge (A4's open point closed), raise and climb ladders that topple when their edge moves, fire
  grappling hooks that sometimes miss, and follow or slip off walls a survey moves.
- The viewport's left, right and top edges are walls for whatever flies (half the speed back, a `bonked` grumpy
  appraisal); at the bottom edge a pet is gone in a puff — nothing airborne ever ends outside the stage box.
- `bun ./📜️script.ts test` (P TS package): 1103 tests in 6.5 s; `test quick` 1103 in 20 s; typecheck clean; React
  typecheck/test clean (170 tests); Protocol v2 oracle + subject green for every case touched.

## 2. Gear as built

### 2.1 Schema (TS, JSON Schema, Rust — all three done)

- `Pitch` (moved from `🏞️terrain` into the schema): `{ wall, surface, side, x, y0, y1 }`.
- `Foothold` `{ x, y, footing, activity, hold }` (`hold` = index into the trip's pitches, −1 off a wall),
  `Trip` `{ owner, from, steps, pitches, facing, ladder, ending: "perch" | "wall" | "air", landing, grip }`.
- `Rope.caught` (false: the shot misses, the hook flies past and is pulled back).
- `Actor.pitch` (the stretch of wall it clings to while on a wall), `Actor.grip` (ticks of climbing left: exact while
  it climbs, the value at `since` while it hangs — hanging drains `GRIP_HANG` per tick lazily; whole off a wall).
- `Stage.pitches` (measured by every survey/summons), `Stage.trips`.

### 2.2 Footings and transitions (`P/🔨️modules/🚶️locomotion/🟦️.ts`, region Gear)

| From | Trip (activity sequence) | Ends |
|---|---|---|
| perch → wall spot | `walk` to the hold (grab beside the wall or hang over the rim) → `climb` (lunges across gaps) | rest on the wall (`climb`, clip paused) |
| perch → perch over a wall line | `walk` → `climb` … → `mantle` over the rim, or step off onto a lower perch | `idle`, proud (`climbed`) |
| perch → perch by raising its ladder | `carry` to the foot, raise (`LADDER_RAISE_TICKS`) → `climb` (`ladder`) → `mantle` | `idle`, proud; ladder stands `LADDER_IDLE` more (≤ `LADDER_LIFE`) |
| perch → perch over a standing ladder | `walk` → `climb` up or down → step off | `idle` |
| perch → perch by rope | `walk` → `aim` (hook flies) → tug → `reel` (`rope`) → `mantle` (hoist) | `idle`, proud |
| missed shot | `walk` → `aim` (hook flies past, comes back) → `shrug` | `idle` for `ROPE_REST`, grumpy (`missed`) |
| resting on a wall | explore another spot (lively, one in two), cheapest way off, wait `REST_RETRY`, `slide`, let go startled | perch, wall or air |
| resting on a ladder | up or down, the nearer first; wait while the ladder may stand on | perch, or let go startled |

- **Rest** (`rest`, exported for B5): `climb` held still, `until` bounded by `(grip − reserve) ÷ GRIP_HANG`, the
  reserve being the cheapest way off its line (`waysOff`). Rest spots need `REST_LEAST` (2 s) of grip beyond the
  reserve (`lasting`) and keep `COMFORT_GAP` from every other body (`spaced`) — walls are spaced like perches.
- **Interruption** (`halt`): a click, a trick, a mode that stops walkers or a hand that takes another's place changes
  the activity → the trip is given up where the actor is: on its perch it stays (rope in, ladder freed), on a wall or
  ladder it rests and finds a way on next tick, on a rope it falls. `unfoot`/`lift` let go of all gear (`ungear`).
- **Edges**: `charted` reflects free flight off the left/right/top edges (`BOUNCE = 0.5`), `fly` feels `bonked`; a
  course whose feet would pass the bottom edge ends there `away` and the pet is gone in a puff (not a counted poof);
  the hand never holds the upright box or the drawing beyond an edge; a survey carrying a body beyond a side edge
  holds it at the edge.

### 2.3 Surveys (`P/🔨️modules/👥️population/🟦️.ts`)

- `measure` cuts `pitches = wallsOf(walls, keepouts, w, h, widest + WALL_ROOM(8), WALL_LEAST(0.5) × smallest height)`.
- `ride(…, walls, uprooted, reshaped)`: when perches, pitches or keep-outs changed (`alike`), every trip is halted
  (perched owners come to rest); `holdOn` then carries ladders that still stand (`ladderHolds`, riders at the same
  share) or topples them (rider steps off low or is thrown, `spillOf`, startled), and carries wall climbers with the
  top of their wall (`wallHolds`, `clingOf`) or throws them off (`slipOf`, startled). Then the side-edge clamp.
- `freeze` halts trips (standing ladders stay, as the projection expects on a still stage); `tune` counts trips off a
  perch as movers and halts perched trip owners beyond the limit.

### 2.4 Routes and decisions (`P/🔨️modules/🎯️choice/🟦️.ts`)

- `venturesOf`: friends' perches (affinity ≥ `FRIENDS`, now exported), in lively mode or when crowded every other
  perch with room — only when `routeOf` offers a way —, and (with `climb` gear, lively or crowded) one wall venture
  per wall line it takes hold of.
- A hop wish with ventures draws five words on `GEAR_STREAM = 0xfffffffb` keyed `[seed, GEAR_STREAM, stream, counter
  of the decision]` (no other draw moves): with no launch, or the first word < `VENTURE_SHARES` (calm 0.25, lively
  0.6), `setOut`; if nothing is clear the hop/wander logic goes on (no launch and no other room → stays idle).
- In lively mode a walk becomes a climb to a wall spot `CLIMB_SHARE` (0.4) of the time.
- `outingsTo` orders outings by trip length (quickest first; `routeOf` order among equals); `setOut` takes the
  first clear one. Gear by species via `routeOf(kind.gear)`; floaters are never `geared` (they keep their lanes).
- Movers (`astir` in `📝️draft`): walking, hopping or on a trip — used by `optionsOf` and `pair`.
- Activity graph: walk → aim/climb; aim → reel/shrug; reel → mantle; climb → mantle/slide; carry → climb; mantle like
  slide; push stays an episode (TS, Rust, Python behavior-choice).

### 2.5 Clock, projection, appraisals

- `act`: trip → `travel`; air/chute/rope → `fly`; wall/ladder → `cling`. `lull` returns 0 while any trip runs,
  lets wall/ladder resters lull like perched ones and stops at a ladder's `until`. `step` removes expired free ladders.
- `frameOf`: a climber on a wall/ladder shows its clip whole (`weightOf` 1) at `clipTime` = phase by distance
  (`climbPhase` on its pitch, `ladderPhase` on its ladder) — still while it rests; tilt from `actor.pitch`
  (`wallward` removed); a missed hook flies out, then back at `HOOK_RETURN` with the gun aimed; `paceOf` 64 while a
  trip runs, resting climbers no faster than their look/eyes/feeling/plumes.
- Occasions `climbed` (content 0.3 + proud 0.4), `missed` (grumpy 0.3), `bonked` (grumpy 0.3); topple and slip use
  `startled`. A dizzy appraisal after a bonk was not added (grumpy only) — see open items.

## 3. Decisions (recorded)

1. **Trips over per-tick plans**: the claim is exactly the executed path; interruption = activity mismatch.
2. **Grip**: grab, climb, lunge, mantle and step-off cost `GRIP_CLIMB` per tick (A4's `wallCost` = path length),
   slide/hang `GRIP_HANG`; exact per foothold during a trip, lazy while resting.
3. **Crossing gaps**: `CROSS_REACH = 1.5` heights (a 30 px gap + `GRIP_BITE` fits pets ≥ 25.3 px tall),
   `LUNGE_TICKS = 16`; `chainOf` never holds a pitch twice (coordinator note: terminates on every survey).
4. **Rope misses** are aimed past the edge where the line is clear but for the card under the hook (`sighted`, now
   exported) — otherwise a pet standing under the edge could never miss.
5. **Bottom edge** = gone in a puff at the edge (not counted as a poof); side/top edges bounce.
6. **Pitch minimum** = half the smallest height (1.5 cut the shelf walls the architecture ladders need).
7. **Rest spacing** = `COMFORT_GAP` around the resting body; **rest minimum** = 2 s of grip beyond the reserve.

## 4. Fuzz (`TK/stage_fuzz.ts`, extended: `--walls` makes every card a 40–240 px box — keep-out with walls)

120 seeds × 6000 ticks per menagerie, random surveys/summons/modes/hand (`fuzz-walls-120.log`, `fuzz-plain-120.log`):

| menagerie | walls | actor-ticks | overlap ticks | poofs (per M) | hard / with parachute | outside the box | bounces | bottom exits | trips | ticks wall / ladder / rope | ladders raised / toppled | let go of a wall | order breaks |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| sample | yes | 2 149 755 | **0** | 8 (3.7) | 0 / **0** | **0** | 9 | 233 | 20 | 2861 / 0 / 12 | 0 / 0 | 1 | 0 |
| architecture | yes | 3 958 199 | **0** | 42 (10.6) | 292 / **0** | **0** | 33 | 380 | 32 | 3268 / 112 / 0 | 2 / 1 | 6 | 1 |
| sample | no | 2 156 656 | **0** | 7 (3.2) | 0 / **0** | **0** | 19 | 0 | 2 | 0 / 0 / 155 | 0 / 0 | 0 | 0 |
| architecture | no | 3 906 929 | **0** | 57 (14.6) | 259 / **0** | **0** | 36 | 0 | 4 | 0 / 0 / 195 | 0 / 0 | 0 | 2 |

B1's baseline was 4.6 / 11.5 poofs per million. The 1–2 order breaks are a pet that a survey pushed against the
stage edge into another one and that poofed and re-arrived in the same tick (`firstDisorder` in the logs). The fuzz
is hand-heavy; gear frequency is shown better by the page runs (`b4_explore.ts`): lively 5 min on a two-column page
gives the troupe 7–33 trips, the architecture menagerie on a quiz page 7–23 per 10 min.

## 5. Stage traces (`TK/generate_behavior_vectors.py`)

New scripts (all laws hold; replayed with `b4_explore.ts script:<id>` to confirm what they show):

| script | seed, length | shows | digest |
|---|---|---|---|
| gutter-life | 3, 60 s | lively page, no card-top room: rests on walls in the gutter, mantles back | 869940545 |
| ladder-up | 1, 72 s | carry → raise → climb → mantle onto the shelf | 1887671247 |
| toppled-ladder | 1, 75 s | the shelf jumps at 67.5 s while mossy is on the ladder: ladder gone, mossy falls and lands | 3131542223 |
| missed-hook | 1, 48 s | aim → hook flies past and back → shrug | 1388150856 |
| grappling-rope | 3, 20 s | aim → reel → hoist onto the shelf | 2408627772 |
| gap-crossing | 7, 20 s | thorny climbs `c1-right`, lunges to `c0-right`, mantles | 1292308066 |
| scrolled-climber | 1, 20 s | page scrolls under a resting climber (follows), jumps 256 px (follows), its card is removed (thrown off, lands dizzy) | 2431192800 |

Moved existing digests (against the last digests B3 recorded, 13:27): `moving-card` 2798023599 → 1772047031,
`crowded-strip` 3495807588 → 1612683227, `new-ground` 3028183423 → 1146971258 — pets whose ground was removed fell
through the bottom edge; they now end at the edge in a puff (earlier vanish, dust in the frames). All other existing
digests are unchanged (e.g. `calm-home` 1993928575, `chemistry-and-moods` 700410868 after restricting ventures to
reachable ones). Second run without `--rerecord`: `traces as committed: 29 of 29 scripts`, fixture `unchanged`.
The checkpoint `Sighting` gained `wall`; `perched` now also requires footing wall ⇔ a pitch of a surveyed wall and
footing ladder ⇒ a standing ladder ridden; the Python keeper checks the wall sightings.

## 6. Storyboard excerpt — architecture menagerie, lively, quiz page (`b4_explore.ts quiz --menagerie architecture --species radiatory,chilly,thermy,waly,shady,battery,sunny`)

Terrain: header bar, a question column (two cards, 30 px apart), an answer column `s0`/`s1` with room above both,
`s1` running below the stage; floor free under the questions and in the 120 px gutter. Seed 1 (10 min, 18 trips):

```
 68.16s battery    perch/idle trip→wall: at 912.3,260.0 perch s0           (sets out for a rest on s0's left wall)
 71.86s battery    wall/climb trip→wall: on s0-left at 745.8,259.0          (hangs over the rim)
 73.50s battery    wall/climb on s0-left at 699.0,362.2 grip 278            (rests in the gutter, clip still)
 80.69s battery    wall/climb trip→perch:s0 on s0-left grip 163             (climbs back)
 82.69s battery    wall/mantle trip→perch:s0 on s0-left at 699.0,303.8
146.34s waly       perch/idle trip→perch:s1 ladder waly at 522.9,720.0      (ladder-only species)
146.36s waly       perch/carry trip→perch:s1 ladder waly
154.91s waly       ladder/climb trip→perch:s1 ladder waly at 681.8,720.0
160.44s waly       ladder/mantle trip→perch:s1 ladder waly at 715.5,584.5
579.66s chilly     perch/idle trip→wall: at 884.3,560.0 perch s1
583.86s chilly     wall/climb on s1-left at 694.0,633.8 grip 313           (rest), then explores higher (607.5)
597.41s chilly     wall/climb trip→perch:s1 … grip 52 → 597.80s wall/mantle → back on s1
overlap ticks 0, outside 0, poofs 0
```
Seed 2: `119.84s battery … trip→perch:s1 rope caught` → `124.13s perch/aim` → `124.63s rope/reel` →
`125.27s rope/mantle`. Seed 3: `133.05s thermy … rope miss` → `140.16s perch/aim` → `140.69s perch/shrug`.

## 7. Commands and real results

From the repository root unless a directory is named; logs in `TK/🗑️generated/b4/`.

| Where | Command | Result |
|---|---|---|
| `P/📦️packages/🟦️typescript` | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts typecheck` | exit 0 (`final2-typecheck.log`) |
| same | `… test` | 16 files, **1103 passed**, 6.5 s (`final2-test.log`) |
| same | `… test quick` | 1103 passed, 20.1 s (`final-test-quick.log`) |
| same | `… test fundamental -t "gear of"` | 20 passed (both companies) |
| `P/🎯️targets/⚛️react/📦️packages/🟦️typescript` | `… typecheck` / `… test` | exit 0 / 8 files, 170 passed (`final2-react-*.log`; an earlier run hit another owner's in-flight `🖌️depiction` edit) |
| `TEST` | `… oracle exhaustive --owner P --case <case>` | `🧗️wall-climbing` 13/13, `🪜️ladder-geometry` 5/5, `🎣️grapple-reach` 8/8, `🚧️clearance-proof` 11/11, `🧠️behavior-choice` 8/8, `💗️feeling-dynamics` 10/10, `🎪️stage-trace` executed 0 (no-oracle case by design) |
| `TEST` | `… subject exhaustive --implementation typescript …` | same cases all passed; `🎪️stage-trace` 3/3 (traces, laws, determinism) |
| `P/📦️packages/🦀️rust` | `cargo check --tests` | exit 0 |
| same | `cargo test --lib -- behavior:: feeling:: schema:: randomness::` | 81 passed |
| repo root | `.venv/Scripts/python.exe TK/generate_behavior_vectors.py --rerecord`, then without | wrote, then `29 of 29 … unchanged` |
| repo root | `.venv/Scripts/python.exe TK/generate_feeling_vectors.py` | feeling-dynamics rewritten (missed 0.25 → 0.3) |
| repo root | `.venv/Scripts/python.exe TK/generate_climbing_vectors.py` | exit 0 (cases still pass) |
| repo root | `bun TK/stage_fuzz.ts --seeds 120 --ticks 6000 --menagerie both [--walls] --out b4` | §4, exit 0 |
| repo root | `node TK/b4_code_rules.mjs` | `problems: 0` |

## 8. TypeScript changes the Rust twins must mirror

Already mirrored in Rust by B4: schema (`Pitch`, `Foothold`, `Arrival`/`Trip`, `Rope.caught`, `Actor.pitch/grip`,
`Stage.pitches/trips` + unit test), `terrain` re-export of `Pitch`, `randomness::GEAR_STREAM`, `feeling` occasions
and appraisals (`missed` 0.3), `behavior::FRIENDS` (pub) and the activity graph, `open_stage`, `arrive` fields.

Still to port (stage Rust package):
- **`🧗️climbing` after D1's 16:16 snapshot**: `chainOf` skips pitches equal to one in the line (`listed`, value
  equality — Rust `chain.contains(next)`); `sighted` is public. (Before 16:16 and so in D1's port: `LadderStand`,
  `Handwork`/`Clamber`, `CROSS_REACH`, `LUNGE_TICKS`, `crossable`, `lungePath`, `chainOf`, `wallPath`, `wallCost`,
  `scaled` with `exit`.) No `🏞️terrain` change after 16:16.
- `📝️draft`: `Draft.menagerie`; `pitches`/`trips` copied; `TURN_TICKS`/`turn` moved here from attention; `remove`
  drops the trip and frees the ladder ridden; `astir`.
- `🚶️locomotion`: `BOUNCE` + `bounced`/`bonked`; bottom edge `away` at feet > height with a puff in `arrive`; `hold`
  checks the upright box; the whole Gear region (`geared`, `tripOf`, `outingsTo` sorted by length, `setOut`,
  `embark` (grip from `gripNow`), `travel` (grip per foothold), `alight`, `rest` (pub), `explored`, `onward`,
  `cling`, `halt`, `freed`, `ungear`, `spaced`, `lasting` with `REST_LEAST`, `restSpots`, `ladderOuting`,
  `ladderWays`, `ropeOuting` (miss via `sighted`), `slideOuting`, `scaling`, `fromPerch`, `waysOff`, `reserveOf`,
  `walked`, `rungs`, `inside`, `tripClaim`); `unfoot`/`lift` ungear; `yielded`/`leave` halt.
- `👥️population`: `measure` pitches (`WALL_ROOM` 8, `WALL_LEAST` 0.5), `alike`, `holdOn`, `ride` (halt on reshape,
  holdOn, side-edge clamp), `survey`/`summon` reshaped, `freeze` halts, `tune` trip movers.
- `🕰️clock`: `act` dispatch, `lull` (trips, ladder horizon, wall/ladder resters), `step` ladder removal.
- `🎯️choice`: `venturesOf`, ventures in `optionsOf`, `astir` movers, hop branch on `GEAR_STREAM`, walk → climb
  (`CLIMB_SHARE`), `VENTURE_SHARES`. `💞️sociability::pair` counts `astir`.
- `🎥️projection`: `clinging`, `clipTime`, `weightOf`, `tiltOf` from the pitch, miss retract in `toolsOf`, `paceOf`.
- `🧪️tests/🎪️stage-trace/🦀️.rs`: `Sighting.wall` and the extended `perched` law (that adapter also still lacks
  `footing`, `body`, `drawn` of earlier rounds).

## 9. What B5 needs — sending a pusher to a station on a wall

```ts
const pitch = draft.pitches.find((entry) => entry.wall === station.wall)!;           // the station's wall
const sent = setOut(draft, index, [{ kind: "wall", pitch, y: station.y }], 0, [0.5, 0.5, 1], now);
```
`y` is the height of the feet at the station (between `rimOf` and `footOf` of a pitch of that wall line). `setOut`
walks to the hold, climbs (lunges included) and ends resting there (`trip.ending === "wall"`), only when the whole
way is clear and the grip lasts for a 2 s rest plus the cheapest way off. When the trip ends (`footing === "wall"`,
no trip), call `rest(draft, index, span, now)` (bounded by its grip) and then `shift(draft, index, "push", now)`; when
`until` passes, `cling` → `onward` takes it off the wall by itself. B5 must add `climb → push` and `push → climb/
fall` to the activity graph, and decide whether a pusher on a wall should be `clinging` in the projection.

## 10. Open items

- A **dizzy** appraisal after a bonk was not added (grumpy only); a hard bonk could `daze` on landing.
- **Two-leg routes** (down one wall, up another) do not exist: `routeOf` gives single legs, so the top cards of a
  two-column page are not reachable from the gutter; rests on the gutter walls are.
- **Ropes are rare** with keep-out cards (their lines are often blocked); ladders need ladder-only or slow climbers.
- The architecture menagerie needs pages with more room (`b4_explore.ts roomy`/`quiz`); on the cramped page only
  one big pet finds a perch.
- **Rust**: everything in §8; the Rust stage-trace adapter cannot replay the new traces until then.
- `TK/🗑️generated/b4/` holds the logs referenced here; delete it when the ticket closes.
- Ticket tools: `TK/b4_explore.ts`, `TK/b4_debug.ts`, `TK/b4_trace_digests.ts`, `TK/b4_code_rules.mjs`,
  `TK/stage_fuzz.ts` (extended), `TK/generate_behavior_vectors.py` (7 scripts, `carded`, `page`, `shelf`, `.new` +
  `os.replace` writes), `TK/generate_climbing_vectors.py` (unique docstring emojis).
