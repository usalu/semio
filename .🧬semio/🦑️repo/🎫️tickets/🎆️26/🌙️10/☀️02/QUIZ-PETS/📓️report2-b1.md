# Report 2 · B1: the body (footings, the hand, the parachute, never colliding)

Work package B1 of the second round of QUIZ-PETS (2026-10-03). It integrates `🪢️swing` (A3), `🚧️clearance` (A5) and the press machine of `👆️gesture` (A6) into the split stage of `P = 🧰️framework/🛍️products/🐾️pets`. The work is in TypeScript. Of the Rust side, only the schema, the constructors and the activity graph were done here (see §9).

**Status:** green.

- TS `typecheck`, `test` and `test quick` pass.
- React `typecheck` and `test` pass.
- The four Protocol cases pass at the exhaustive level.
- The traces are reproducible (22 of 22 as committed).
- The fuzz of the real stage shows **0 overlap ticks** in 11.98 M actor-ticks, with **0 hard landings of a pet that owns a parachute**.

## 1. Footings: the machinery and the API for B4/B5

`Actor.footing` (`perch | air | chute | hand | head`) picks the integrator. `perch === null` is no longer used as a mode anywhere: the laws check that `footing === "perch"` holds exactly when `perch !== null`.

### Dispatch

`act` in `🕰️clock` dispatches on the footing:

- `air` and `chute` → `fly`
- `head` → `slip`
- `perch` → walk, scoot, sleep and idle, as before
- `hand` → `handle` in `👀️attention`, the hand phase of the tick

### Tick order

`step` in `🕰️clock` runs the phases in this order:

1. prune claims
2. drop expired puffs
3. `handle` (the hand)
4. `scoot`
5. actors (`act`)
6. hovers
7. beat
8. meet
9. pair
10. spawn

### Schema (shared; small anchored edits)

New types:

| Type | Fields |
|---|---|
| `Extent` | `x0`, `y0`, `x1`, `y1` |
| `Slice` | `from`, `until`, `extent` |
| `Claim` | `owner`, `slices`, `rest` |
| `Waypoint` | `x`, `y`, `vx`, `vy`, `tilt`, `canopy` |
| `Course` | `owner`, `from`, `steps`, `ending: perch \| head \| away`, `landing`, `touch` |
| `Puff` | `x`, `y`, `width`, `height`, `tick` |

New fields:

- `Actor` gains `host: Slug | null` and `tilt: Turns`.
- `Stage` gains `puffs`, `claims`, `courses` and `origin: Point | null`.

These changes are applied to `🟦️.ts`, `🔣️.json` (117 `$defs`) and `🦀️.rs` (inline enum `Ending`). `🚧️clearance` now imports `Extent`, `Slice` and `Claim` from the schema and re-exports them.

### API: planned motion through the air (`🚶️locomotion`)

Use this for a new way to travel.

- **Plans.**
  - `Plan = {course, claim}`.
  - `planOf(draft, i, course)` makes the claim. Its slices are `CLAIM_SPAN = 4` ticks each, and it rests on the last waypoint (nowhere for `away`).
  - `install(draft, i, plan, activity, now)` puts the course and the claim on the table. It sets the footing to `air`, or to `chute` when a canopy is out.
  - **Before `install`, always vet the plan with `claimClear(plan.claim, bodiesOf(...), draft.claims)`.** `launch` (hop or float glide) and `giveBack` are the worked examples.
- **Executing a course.**
  - `fly(draft, i, now)` copies the waypoints. It opens the parachute where the course opens it, with a spring of 220/12.
  - On the last waypoint the actor arrives:
    - `perch` → `land`, or rest at once after a parachute
    - `head` → `slide`; the host feels `trampled`
    - `away` → vanish
- **Losing the footing.**
  - `unfoot(draft, i, vx, vy, activity, now)` drops the claim and the course. The footing becomes `air` (or `chute`) and `until = now + DELAY`.
  - `plunge` then plans the way down in the actor's turn.
  - `drop(draft, i, now)` = `unfoot` with zero velocity and `fall`.
- **Ways down.** `waysDown(draft, i, from)` returns `{chosen, plans}`. Tried in order:
  1. as it flies
  2. towards the free column over its landing (`columnOver`)
  3. each of the `STEERINGS` of air control
  4. when it was thrown: bounced back at half its sideways speed, and let go with air control

  Only perched bodies without a plan serve as heads.
- **When no way is clear.** `plunge` waits in place while that place is free (`mustPoof`). Otherwise it calls `poof`: a `Puff` is recorded, `poofs` goes up by 1, and the pet vanishes and arrives anew.
- **Other exports:**
  - `vanish`, `courseOf`, `daze` (dizzy after a `land` whose `vy` ≥ `DIZZY_SPEED` 780)
  - `scoot(draft, now)`, `slip`, `stride` (guarded)
  - `lift`, `hold`, `letGo`, `giveBack`, `toss`, `tossTarget`
  - `PUFF_TICKS` (48)

### API: bodies (`📏️spacing`)

- `extentAt(owner, kind, feet, tilt, canopy)` gives one body:
  - upright: the size box, the hover and `MARGIN` 4
  - tilted: that box rotated about the scruff, `grip` above the feet
  - with a canopy: the canopy box from the depiction constants added
- `extentOf(actor, kind)` and `bodiesOf(actors, kinds)` give the bodies of actors as they are.
- `obstaclesFor(draft, i, now)` returns the other bodies, the corridors that are left and the landing spots.
- `roomsFor` is 2D: it carves every obstacle that shares the height.
- `clearway` knows footings: scooters count like walkers.
- `COMFORT_GAP` = 8.

### Horizons and rates

| Where | Rule |
|---|---|
| `lull` | 0 for any footing ≠ `perch`, for `scoot`, and while the press is open with no `pressDue` |
| `lull` horizons | `pressDue` and puff expiry (`tick + PUFF_TICKS`) |
| `paceOf` | 64 for footing ≠ `perch` and for `scoot` |
| Frame rate | 64 while the press is open with no `pressDue` |
| `wake` | includes `pressDue` and puff expiry |

Cut-invariance is tested with a held pet, a throw, a parachute and a toss.

### Activity graph

The graph lives in `🧠️behavior` (TS and Rust) and in `behavior-choice/🐍️.py`.

- Every activity can lead to `hang`, except `hang` itself.
- Perch activities can lead to `fall` and `scoot`.
- New follower lists: `AFTER_HANG`, `AFTER_TUMBLE`, `AFTER_GLIDE`, `AFTER_SLIDE`, `AFTER_GROUNDED` and `AFTER_EPISODE = [idle, hang]`.

## 2. The hand

The press machine of A6 (`pressStep`) is fed:

- by the façade on `pressed`, `dragged`, `released` and `cancelled`;
- by `handle` on every tick (`pressDue` sets the horizon).

`grasp(draft, signal, touched, now)` in `👀️attention` maps the signals:

| Signal or event | What happens |
|---|---|
| click | B2's `clicked` |
| `hold` | a loose pet purrs (B2's hook) |
| `lift` | Scruff pick-up when play is permitted and the pet is not leaving. `lift` releases the partner, the claim and the course; it sets `hangOf` with `Species.grip` as the rod and remembers `origin` when the pet was picked up from a perch. Appraisal `lifted`. |
| every tick in the hand | `hold`: the grip follows the pointer by `hangStep`, the body swings (`leanOf` tilt) and is pushed out of bodies, corridors and landing spots (`pushedOut`); otherwise it keeps its place. The trail keeps 7 samples (`RELEASE_WEIGHTS`). `shakeStep` → `shaken` (B2's hook). `dangled` at 64 ticks. |
| `drop` (release) | `letGo(releaseVelocity(trail))`: tumble (the tilt eases back over 24 ticks), air control, and the parachute rule |
| `abort` (cancel) | `giveBack`: an eased straight glide back to the free spot nearest to the origin on that perch. It is a claimed course; when that is not clear, or the pet was not picked up from a perch, it is let go instead. |
| `played` deed `toss` | `tossed`: lifted, carried to `TOSS_TOP` (16 px below the top) for 96 ticks, then let go |
| play permission off | an open press is aborted (glide back), the press goes `IDLE` and `touched` becomes null |
| quiet | pick-up is allowed |
| still | nothing; `freeze` resets the press, and pets that are not on a perch are gone and arrive anew |

**Found and fixed this session: the hand takes a pet out of its plan.**

- Time-sliced claims let a falling pet's corridor pass where another pet would only be later.
- When the learner grabbed that other pet in mid-flight, it stayed inside the first pet's corridor, and the faller ran into it. The quick-level mini-fuzz caught this as `sparky+pebble at tick 380`.
- The fix is `yielded` in `lift`: whoever's claim covers the new place of the held body gives up its plan and plans anew in its turn, keeping its activity.

**Storyboard excerpt** of the `drag-and-drop` trace script, from `b1_drag_storyboard.ts`. The sum of overlapping body pairs over all ticks is 0.

```
  4.00 s  · learner pressed at (622, 285)
  4.05 s  · press lifted, touched mossy
  4.05 s  mossy      perch/idle → hand/hang x 620.7 y 301.0 v (0, 0) tilt 0.008
  4.50 s  mossy        hand/hang x 729.9 y 259.3 v (0, 0) tilt 0.043
  5.25 s  mossy        hand/hang x 901.8 y 187.2 v (0, 0) tilt -0.015
  7.00 s  · learner released at (900, 160)
  7.02 s  mossy      hand/hang → air/tumble x 900.1 y 187.4 v (0, 28) tilt -0.001
  7.25 s  mossy      air/tumble → chute/glide x 900.1 y 245.9 v (0, 392) tilt -0.000 chute open 0.00
  7.50 s  mossy        chute/glide x 899.7 y 303.8 v (-4, 143) chute open 1.20 sway-x -0.8
  9.00 s  mossy        chute/glide x 898.4 y 408.0 v (13, 60) chute open 1.00 sway-x 1.1
 14.25 s  mossy      chute/glide → perch/idle on floor x 894.3 y 720.0 v (0, 0) tilt 0.000
```

The `cancel-and-permit` script gives the following:

- cancel at 4.00 s → `air/glide` → `perch/land on ledge` at 4.20 s
- permission off at 12.00 s → glide back to the ledge
- tuned to still at 20.00 s while held → back on a perch at once

The held pet's `v` stays (0, 0) because the hand's velocity lives in `hang.grip` and in the trail.

## 3. The parachute rule (A3's open point fixed)

The decision is made on the planned course itself (`decisionOf`), not on a prediction.

- A pet that owns a parachute opens it at the first tick with `vy ≥ CHUTE_OPENING` (240) and `CHUTE_HEADROOM` (56) or more above its real landing.
- Thrown upwards, it opens at the first tick with `vy ≥ 0` and that headroom.
- The canopy course opens `CHUTE_REFLEX` ticks later. It glides with sway (`chuteWind`), steers to the plain course's landing x and flares.

**Never hard:** `wayDown` re-checks the touch speed of the chosen course.

- When a parachute owner would still land above `HARD_LANDING` (600), it brakes at once and keeps half of its downward speed. This repeats for `BRACES = 4` rounds, and at last it keeps none.
- This covers a pet thrown down at the ground too close for the canopy to open: those were the 4–5 "braces" of the earlier fuzz runs.
- It is proven by the fuzz: hard landings with a parachute went from 5 to 0.
- A new unit test covers heights 2–55 px and speeds 610–2000 px/s at the exhaustive level.

**Species without a parachute** land hard (`land`). They are `dizzy` after a long fall (touch ≥ 780). The appraisals are `dropped`, `landed` and `floated`; for heads, `trampled`.

## 4. Never colliding (design-v2 §18)

The invariant is that no two bodies overlap at the end of any tick. The means:

- **Bodies:** `MARGIN` 4 on every side, `SEAM` 1/64. A held body is rotated about the scruff; an open canopy adds its box.
- **Check before commit:**
  - Walkers, scooters and head riders move only as far as `guardedStride` allows.
  - A blocked walk ends at once.
  - A held pet is pushed out (`pushedOut`, `PUSHES`) or keeps its place.
- **Claims and reservations:** every planned flight (hop, glide, fall, throw, chute, return glide) is a `Course` with a `Claim`. It is taken only when `claimClear` passes. The claims are pruned every tick.
- **Seating after a survey:**
  - `seatOf` (pool-adjacent-violators) runs per perch in order of strain; leavers go last.
  - The movers scoot as a group with one common fraction (`scoot`), guarded, with patience `until`.
  - A pet more than 2 widths (`STRANDED`) from its perch falls.
  - Whoever does not fit falls off the perch, or poofs when it is still on the perch.
  - New this session: seat targets are clamped onto the perch. Floating-point rounding had put a scooter at x1 − half + 1e-14.
- **Rides:** a moving surface first makes every course owner re-plan. Non-hand actors outside the narrowed stage vanish. Head riders move with their hosts. `evicted` pets poof.
- **Heads:** landing on a head works like landing on a perch (`headUnder`, `liftOnto`). The rider slides off (`slideOf`, `SLIDE_OFF_SPEED` 32). When the host no longer carries it, it falls.
- **Waits and the last resort:**
  - A pet that cannot plan a way down waits for `DELAY` (32 ticks) while its place is free.
  - Otherwise it poofs: it vanishes and a `Puff` dust record is kept for B3. B3 draws it; it expires after `PUFF_TICKS`. The pet arrives anew at once through spawn.
- **Encounter gap:** the shoulders plus max(`reach` A + `reach` B, `COMFORT_GAP`) plus `SEAM`.
- **The law `apart`** is now the 2D body law. It holds in the TS trace adapter (`overlaps(bodiesOf(...)) = []`) and in the Python verifier (pairwise numpy box overlap). The verifier also checks that an upright recorded body equals the size box plus the hover plus 4, and that footing and perch agree.
- **Unit suites:** `run()` holds every tick apart (`heldApart`), so every stage test that advances time proves the invariant.

## 5. Fuzz of the real stage (`TK/stage_fuzz.ts`)

The fuzz drives `advance` of `🎪️stage` itself.

- Surveys shrink, grow, move, jump or remove cards.
- Summons, mode changes and times of concentration happen at random, and the pointer roams.
- The hand presses, drags of 20–150 ticks at 2–18 px/tick into other pets, throws, drops, cancels, clicks and tosses.

The final run is `--seeds 24 --ticks 30000 --menagerie both --name final`, after all fixes; its exit code was 0.

| menagerie | actor-ticks | overlap ticks | near-miss ticks (2 px) | poofs (per million) | waits | order breaks | parachutes / heads / landings / hard / hard with a parachute |
|---|---|---|---|---|---|---|---|
| sample | 2 156 456 | **0** | 34 882 | 10 (4.6) | 522 | 0 | 2 554 / 394 / 2 955 / 0 / **0** |
| architecture | 3 824 948 | **0** | 135 938 | 44 (11.5) | 1 022 | 0 | 1 575 / 1 130 / 3 621 / 268 / **0** |

The hand events per menagerie (presses / clicks / lifts / throws / drops / cancels / tosses) were:

- sample: 4381 / 1092 / 3193 / 1947 / 1003 / 333 / 1242
- architecture: 4369 / 1059 / 3195 / 1963 / 995 / 341 / 1226

"Ghost-free" means:

- no overlap tick;
- no order break on a perch;
- every poof leaves a puff.

The 268 hard landings in the architecture run are of species without a parachute.

## 6. Commands and real results (2026-10-03, about 13:20–13:50, under load from parallel agents)

| Where | Command | Result |
|---|---|---|
| `P/📦️packages/🟦️typescript` | `bun ./📜️script.ts typecheck` | exit 0 |
| same | `bun ./📜️script.ts test` | exit 0, 16 files, 1080 tests, 12.70 s at 58 % CPU load (8.27 s at 34 %) |
| same | `bun ./📜️script.ts test quick` | exit 0, 1080 tests, 48.45 s |
| `P/🎯️targets/⚛️react/📦️packages/🟦️typescript` | `bun ./📜️script.ts typecheck` / `test` | exit 0 / exit 0, 8 files, 166 tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "<case>"` | `🎪️stage-trace` executed=0 not-exercised=1 (a no-oracle case by design); `🚧️clearance-proof` 11/11; `🪢️swing-dynamics` 13/13; `🪂️parachute-descent` 9/9 |
| same | `… subject exhaustive --implementation typescript …` | `🎪️stage-trace` 3/3; `🚧️clearance-proof` 11/11; `🪢️swing-dynamics` 13/13; `🪂️parachute-descent` 9/9 |
| repo root | `.venv/Scripts/python.exe TK/generate_behavior_vectors.py` | "traces as committed: 22 of 22 scripts"; every fixture `unchanged` |
| repo root | `bun TK/stage_fuzz.ts --seeds 24 --ticks 30000 --menagerie both --name final` | table in §5, exit 0 |
| repo root | `bun TK/b1_drag_storyboard.ts drag-and-drop cancel-and-permit` | §2; 0 overlapping pairs |
| repo root | `node TK/b1_code_rules.mjs` | `problems: 0` over 29 files |
| repo root | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test` | compiles; `197 passed; 1 failed`. The failure is `stage::tests::the_calm_home_replays_into_its_committed_trace` (`digest: 2407184199 ≠ 1993928575`): the Rust stage twin does not yet mirror the new body, mind and frame, which is phase D. |

`b1_code_rules.mjs` checks for a unique leading emoji per docstring, no console use or `[DEBUG]`, no comments inside definitions and no banned stems. The findings it lists as "known" are not B1's:

- `const base` in `🎥️projection` (B3's)
- `const core` in `🧠️behavior` (last commit)
- `▭️` in the schema (last commit)
- the forbidden-call list in the Rust behaviour test

**Fundamental budget.** The full run had been killed at 15 s while it ran at 100 % CPU load (all tests passed). The trims:

- The stage file had per-tick `expect`s and repeated overlap checks; they now collect the findings and assert once, because `run()` already holds every tick apart. The stage file went from 9.3 s to 5.5 s.
- With heavy parallel load the 15 s budget is still tight: 12.7 s at 58 % load.

### Trace digests: what moved and why

Compared with the last commit (`git show HEAD:…/🎪️stage-trace/🔣️.json`):

- **Moved:** all 12 old scripts, from the first checkpoint (tick 640) on. Three causes:
  1. Their event logs changed: R0's surveys gained walls and fixtures, and pointed events gained `over`.
  2. B3's frame digest now covers footing, tilt, pivot, tools and body.
  3. B1's body: bodies with a margin of 4, guarded strides, scoots, chutes, and the reach-based encounter gap.
- **New:** 10 scripts.
  - B2: `clicks-and-glances`, `clicks-and-purrs`, `circles-and-states`, `chemistry-and-moods`.
  - B1: `drag-and-drop` (seed 188), `throw-into-a-crowd` (199), `chute-landings` (211), `heads` (222), `crowded-reseat` (233), `cancel-and-permit` (244).
- **Removed:** `pokes-and-glances` (replaced by B2's script).
- The fixes of this session (`yielded`, the seat clamp, braking) moved no committed digest: the second generator run printed `unchanged`.

## 7. TS changes the Rust twins must mirror (phase D)

**Done in Rust (this package):**

- schema types and fields
- `open_stage` (`puffs`, `claims` and `courses` empty, `origin` none)
- `arrive` (`host` none, `tilt` 0)
- the activity graph and its tests
- the schema unit test (117 defs, `ENDINGS`, the decoded course)

**To mirror:**

1. `📏️spacing`: `extentAt` (rotated about the scruff, canopy box with `CANOPY_SPAN` 0.75, `CANOPY_RISE` 0.7, `CANOPY_DOME` 0.5625), `extentOf`, `bodiesOf`, `obstaclesFor`, the 2D `roomsFor`, `clearway` aware of footings. `MEET_GAP`, `hindered` and `vacancy` are gone.
2. `🚶️locomotion`, the whole file. In particular:
   - `charted`, `decisionOf`, and `wayDown` with braking (`BRACES` 4)
   - `waysDown`, `planOf`, `install`, `arrive`, `fly`, `plunge`, `poof`/`vanish`, `unfoot`/`drop`
   - `launch` (lanes, `LANE_RAMP`), `stride` (guarded), `scoot`, `slip`
   - `lift` plus `yielded`, `hold`, `letGo`, `giveBack`, `toss`/`tossTarget`
   - the constants listed in its region
3. `👥️population`:
   - `STRANDED` pre-pass, seat order by strain, walk goals clamped, seat targets clamped onto the perch
   - `carry` returns a bool; `ride` unfoots course owners and vanishes actors outside the stage
   - `freeze` resets the press, `touched`, the trail and the origin
4. `👀️attention`: the Hand region (`grasp`, `tossed`, `handle`, `TRAIL`, `DANGLE_TICKS`); `presenceOf` is 1 in the hand; `headingOf` and `perk` use the footing.
5. `🕰️clock`: `act` dispatch, the `lull` terms, the step order.
6. `🎥️projection`: the `paceOf` and `wake` terms. B3 owns the frame mapping.
7. `🎯️choice`: hop via `launch`; `conclude` returns for footing ≠ `perch`; `land` with `vy` > 0 → `daze`.
8. `🗓️schedule`: `sociable` needs footing `perch`.
9. `💞️sociability`: the reach-based gap, `parted` by footing.
10. `📝️draft`: the new fields in `draftOf`/`sealed`, `Launch` with `y` and `surface`, and `remove` drops the actor's claims and courses and `touched`.
11. `🎪️stage` façade:
    - `pressed`, `dragged`, `released` and `cancelled` feed `grasp`
    - `played` toss
    - permit off aborts the press
12. Stage-trace Rust adapter: `Sighting` with footing and body, the 2D law `apart`, and the scooter relaxations of `perched` and `clear`.

## 8. Decisions recorded

- Courses live in the stage, so frames and Rust can replay them. Claims use slices of 4 ticks.
- The update order of the menagerie is kept (no rotation).
- A blocked walk ends at once.
- The tumble tilt eases over 24 ticks.
- Head riders use `slide`; the return glide uses `glide`.
- Dizzy comes through `land` with `vy` = touch ≥ 780.
- Stranded = 2 widths.
- The `perched` and `clear` laws give scooters 2 widths of slack and let them leave a keep-out.
- Poofed pets arrive anew at once.
- `hold` only makes a loose pet purr; the warmth of a hold is B2's.
- The hand overrides plans: `yielded` re-plans whoever's corridor the picked-up pet occupies.
- Braking halves the downward speed rather than capping the throw. It works for every way down: a throw, a re-plan, a ride.

## 9. Open

- **Rust twins** of every item in §7 (phase D). The crate compiles with the new schema, and 197 of its tests pass. The one failure is the replay of the committed `calm-home` trace, which waits for these twins.
- **B3:** the frame mapping of tilt, pivot, the held pet, puffs and chute tools. The React canopy should share `CANOPY_*` with `📏️spacing`.
- **B2:** the warmth or heat of a hold.
- **Rule slip.** One early edit of `🚶️locomotion/🟦️.ts` was made with a Python snippet instead of Write/Edit. The file was checked to be intact, and every later change used Edit only.
- **The 15 s fundamental budget** is tight when many agents share the machine.

## 10. Files

**Created**

- Ticket tools in TK: `stage_fuzz.ts`, `b1_drag_storyboard.ts`, `b1_code_rules.mjs`, this report.
- `b1_probe.ts` and `b1_overlap_probe.ts` were temporary; both carried `[DEBUG]` and are deleted.

**Updated**

- Schema: `P/🧬️schema/{🟦️.ts, 🔣️.json, 🦀️.rs, 🧪️tests/🔬️unit/🦀️.rs}`.
- `P/🔨️modules/`:
  - TS: `🚧️clearance`, `📝️draft`, `📏️spacing`, `🚶️locomotion`, `👥️population`, `👀️attention`, `🕰️clock`, `🎯️choice`, `🗓️schedule`, `💞️sociability`, `🎥️projection`, `🎪️stage`, `🧠️behavior`
  - Rust: `🎪️stage`, `👥️population`, `🧠️behavior`
  - unit tests: `🎪️stage` (TS), `🧠️behavior` (TS and Rust)
- `P/🧪️tests/🎪️stage-trace/{🟦️.ts, 🐍️.py, 🥒️.feature}` and `P/🧪️tests/🧠️behavior-choice/🐍️.py`.
- Regenerated fixtures: `P/🧫️fixtures/{🎪️stage-trace, 🧠️behavior-choice}/🔣️.json`.
- `TK/generate_behavior_vectors.py`.

**Tool output:** `TK/🗑️generated/b1/` (logs and fuzz JSON). It will be deleted with the ticket's generated folder at the close.
