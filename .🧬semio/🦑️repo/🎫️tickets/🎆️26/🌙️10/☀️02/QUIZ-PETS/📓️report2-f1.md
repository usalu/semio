# Report F1: overlap while panning, multi-leg gear routes, circling, see-through, and gear on the real pages

Work package F1 of the second round, 2026-10-03. Abbreviations: `P` is `🧰️framework/🛍️products/🐾️pets`, `TK` is this
ticket folder, and `PR` is `P/🎯️targets/⚛️react`. Every number below comes from a run in this session. The tool output is
in `TK/🗑️generated/f1/`; delete that folder when the ticket closes.

## 0. Summary

| Defect / request | Cause | Fix | Proof |
|---|---|---|---|
| 1. Overlap on the home overview while the panorama pans | A climber's wall lean (`WALL_LEAN` 0.04 turn) was drawn, but the clearance body and the trip claims stayed upright. The drawing poked out of the body (fuzz: `windowy wall/climb tilt -0.0400 × kettly hand/hang`). | The lean is now part of the body: `📏️spacing.wallLeanOf` (positional: full at the cling, none half a width past it) feeds `extentOf`, the trip claims (`bodyAt`) and the projection's `tiltOf`. | Lean test: fails before (2), passes after. Pan fuzz: 7 579 933 actor-ticks with 0 overlaps and 0 drawn overlaps. New sampled stage test for a panning overview. |
| 2. Multi-leg routes and gear on the real pages | Routes had a single leg. On the 1440 × 900 pages no ladder (≤ 3.6 heights) and no rope (≤ 3.2 heights) reached. Card bodies, which the survey grows by 4 px, blocked ladder lines. A rope from below always ran into the card under the edge, or swung into the card and below the footer. | Two-leg routes are planned and claimed as a whole. `ladderTo` (a ladder against the lower end of a wall, then the wall). Telescopic ladder and rope (8 heights each). `flanking` keep-outs. Hooks bite the corner. A slanted rope whose swing would hit a keep-out under the hook is hauled in straight. Shooters step back (`aims`). Explorers: a lively walk turns into a gear trip up. Mischief slips over only after every prospect was tried with its gear. | Stage test on the measured heating page: a pusher reaches its task row's station by ladder then wall, with no puff (fails before, passes after). Rope test on the overview's card geometry (fails before, passes after). Browser: a ladder on a quiz page and a rope on home, in both topologies (§5). |
| 3. Circling holds attention | `optionsOf`, `sociable` and `spread` ignored the gesture. | `👆️gesture.circled(hover, tick)` holds walks, hops, gear trips, encounters, pranks and spreading while the pointer circles the pet. | Circling test: fails before (2), passes after. |
| 4. See-through per design-v2 §17 | A pet turned see-through under a resting pointer. | No see-through under the pointer (the pet perks up). A pet becomes see-through (0.35) only in the air, on a wall, a ladder or a rope in front of a keep-out (`presenceOf(keepouts, …)`). | See-through block: fails before (12), passes after. React `🫥️decorative-layer` tests rewritten. |
| Leftovers | — | Dizzy after a hard bump against the edge of the stage (`bumped`). The crate's `Cargo.toml` description now lists the modules. | Bump test: fails before, passes after. |
| Poofs (coordinator) | Falls through the bottom edge and mischief slip-overs were not counted. | `dusted` counts every puff. | Bottom-exit test and mischief test: fail before, pass after. Fuzz tables count all poofs. |

## 1. Defect 1: overlap while panning

- **Reproduction.** `TK/stage_fuzz.ts --pan` (§6) sets up the home overview at 1440 × 900, panning in sweeps over climbers, ladders, ropes and the hand. The tool also measures the drawn box the way `🐕️pet-walk` does (the size box turned about the feet by the frame's tilt; overlaps over 0.5 px count) and fails on any drawn overlap. On the code from before F1 it found drawn overlaps between a leaning climber and a pet held next to it.
- **Fix** (`P/🔨️modules/📏️spacing/🟦️.ts`):
  - `WALL_LEAN` moved here from the projection.
  - `wallLeanOf(kind, footing, pitch, x)` gives `(pitch.side > 0 ? −1 : 1) × WALL_LEAN × clamp(1 − |x − clingOf(pitch, size)| ÷ (w/2), 0, 1)`.
  - `extentOf` turns the size box by `actor.tilt + wallLeanOf(…)`.
  - `🚶️locomotion.bodyAt` uses the same lean for the claims of trips (`tripClaim`, `spaced`).
  - `🎥️projection.tiltOf` draws `actor.tilt + wallLeanOf(…)`.
  - The lean is positional, so it fades out over the mantle.
- **Proof.**
  - Stage test "holds the drawing of a climber that leans towards its wall inside its body": `lean-before.log` 2 failed, `lean-after.log` 2 passed.
  - New stage test "keeps every body and every drawing apart while the panorama of a home overview at 1440 × 900 pans under pets that climb …" runs at sampled amounts: 1 seed × 24 s at fundamental, 2 × 90 s at quick, 4 × 300 s at exhaustive.
  - Pan fuzz (48 seeds × 19 200 ticks per menagerie), §6.

## 2. Defect 2: multi-leg routes, ladders and ropes on the real pages

**What the real pages offer** (measured with `TK/f1_survey.mjs` and played with `TK/f1_routes.ts`):

- Quiz pages (heating, physics, cooling, demand). The three cards of the column stand about 30 px apart, so the footer (y 874) is the **only perch**. The lowest card wall ends about 404 px above it.
  - Rope: impossible, because there is no edge to hook.
  - Ladder: now possible. `ladderTo` reaches the lowest wall (radiatory, battery, solary).
  - Simulated 480 s, lively, real survey:
    - heating: radiatory 3.7k–10.2k ladder actor-ticks
    - physics: battery and solary 11.9k–14.6k
    - wall ticks 4k–9k on every page
- Home overview, **new layout**. The coordinator's relay: cards now fill their cells, gaps between columns are about 13–15 px, and the outer walls touch the screen edges.
  - 18 perches: card tops and tabs in three rows (y ≈ 96–124, 383–423, 698–732) plus the footer.
  - 14 pitches, all on tab walls.
  - The walls of the 13–15 px gaps and the screen-edge walls give no pitch, because a pet has no room beside them.
  - Gear legs:
    - grapplers zip from the footer to the bottom-row tops (x 720 → s25, x 1035 → s27) and from row 2 to row 1
    - waly can lean a ladder
  - Life over 480 s × 3 seeds: 4–14 trips, wall ticks 1.2k–3.4k, rope ticks 136–139 (battery), 0 poofs, 0 overlaps (`routes` runs in §6).
  - On the earlier layout (`survey/`), ropes from the footer reached only one tab, and only after the corner-hook and haul fixes. On the new layout they reach the bottom row, so the browser proof of the rope stays on home. The ladder proof moved to a quiz page.

**Changes** (all TypeScript; Rust in §8):

- `🧗️climbing`:
  - Telescopic ladder: `LADDER_TALL` 3.6 → 8. Grappling rope: `ROPE_LONG` 3.2 → 8. The real pages need about 8 heights above their footer.
  - `flanking(keepouts, pitch)`: keep-outs that begin within `WALL_LIP` on the element's side of the wall are the element's body. The survey grows them past the side, and the ladder leans on them.
  - `stood(low, pitch, x, y, …)`: the contact must lie between `y0 + LADDER_TUCK` and `y1`.
  - `ladderTo(low, pitch, keepouts, size)`: top at the pitch's lower end, foot at the lean, and the exit must reach `clings`.
  - `ladderHolds`: tries the tops `[y0 + TUCK, y1]` and no longer needs a rim perch.
  - `shotFor`: candidate spots are the corners (`HOOK_INSET`) and the point nearest to the feet.
  - `aimed`: a swing whose drop line under the hook hits a keep-out becomes a zip.
  - `aims`: where it stands, the nearest points, then back one width at a time up to `ROPE_LONG` heights.
  - `routeOf`: grapple from the first of `aims` that has a shot.
- `🚶️locomotion`:
  - Two-leg routes: `legsTo`, `outingsTo` (direct first, otherwise through a perch on the way, planned and claimed whole), `joined`, `ranked`, `reaches`.
  - The stand of a leg (`Stand`, `standOf`).
  - Ladder to wall (`ladderLead`, `leanOuting`) and down a standing ladder from a wall (`ladderDown` in `waysOff`).
  - Rest spots from the exit of a raised ladder (`restSpots`).
  - Footholds carry their own perch and facing. `travel` sets the footing, the perch and the goal per walk run, and turns where a foothold faces the other way. `walkEnd` and `embark` set the first goal and the first turn.
  - `halt` from a ladder or a wall leaves the pet resting there.
  - `postTo(…, slip, now)` slips over only when told to.
- `🎯️choice`:
  - `venturesOf` uses `reaches`, and adds wall ventures through `ladderTo`.
  - `decide`: in a lively stage, a walk becomes a trip to a wall rest or to a higher perch `CLIMB_SHARE` of the time (explorers).
  - `prank`: first every prospect with its gear, then the slip-over.
- `🧬️schema` (TS, JSON Schema, Rust): `Foothold.perch` and `Foothold.facing`; `Trip.facing` is gone.

**Proof:**

- Stage test "goes to its post beside a task row of a quiz's page as the site lays it out at 1440 × 900 with its gear, in two legs planned whole …". It uses the measured heating boxes and a 54 × 55 pusher with `climb` + `ladder`. The story is `perch → ladder → wall`, then `push` at y 409.6 on a `tasks-*` wall, with no puff and no poof.
  - `quizpost-before.log`: 2 failed (puff at x 700, the slip-over).
  - `quizpost-after.log`: 2 passed.
- Stage test "hooks the corner of the tab of a card of the home overview …". The rope is hauled straight beside the card, never below the footer, from x < 600.
  - `aims-before.log`: failed (no shot).
  - `aims-after.log`: passed.
- Stage test "is gone in a puff the stage counts when it falls out of the stage": before, `poofs` stayed 0.
- Simulated real pages: `TK/f1_routes.ts`, `TK/f1_rope_rounds.ts`, `TK/f1_rope_story.ts` (§6). Browser: §5.

## 3. Defect 3: circling holds attention

- `👆️gesture.circled(hover, tick)` holds while the circle has quarter turns and the last one is no older than `CIRCLE_SLOW` (40 ticks).
- Holding means:
  - `choice.optionsOf`: not restless, so no hops or ventures, and no `roam`
  - `schedule.sociable(actor, tick)`: no encounters and no pranks
  - `population.spread`: skipped
- Test: "holds still while the pointer circles it …". `circle-before.log` 2 failed, `circle-after.log` 2 passed.

## 4. Defect 4: see-through (design-v2 §17)

- `👀️attention.presenceOf(keepouts, actor, kind)` returns 0.35 only when the footing is `air`, `chute`, `wall`, `ladder` or `rope`, and the size box, inset by `WALL_LIP`, lies over a keep-out. Otherwise it returns 1.
- `SHY_REACH` is gone.
- `perk` greets a resting pointer on the pet too.
- `clock.act` and `clock.lull`, and `projection.paceOf`, use the keep-outs.
- The frame shows `actor.opacity`, so a still stage never goes see-through.
- `PR/🔨️modules/🫧️layer`: `POINTER_REACH`, `touched` and `near` are gone, and `point()` is false on a still stage.
- Tests:
  - Stage block "seeing through a pet": `see-before.log` 12 failed, `see-after.log` 12 passed.
  - `P/🧪️tests/🫥️decorative-layer`: two tests rewritten ("keeps a pet of a still/living stage whole …").
  - React suite: 173 passed.
- Docs: `P/README.md` (layer paragraph) and `🎪️stage-trace/🥒️.feature` updated.

## 5. Browser: `pets` project, private stack (site 6249, proctor 8949, WATCH off)

The gear spec `🐕️pet-walk/🟦️.ts` "a pet that cannot hop to a perch above raises its ladder or shoots its grapple there …" is no longer `fixme`. It runs at tempo 8 and lively, and waits on state through the trail of `data-pet-footing`.

1. **Ladder.** Open the quiz whose cast core holds a species with `climb` + `ladder` (chosen from the ensemble: physics, battery), then wait for `ladder`.
2. **Rope.** Back on home, rounds of up to 15 s:
   - grapplers standing on raised perches are carried to the footer line
   - a non-grappler sitting on a perch within 8 grappler heights is held in the hand
   - wait for `rope`, for at most 360 s in total
   - the spec's own timeout is 660 s

After the coordinator's relay about the new home layout:

- The ladder proof moved to the quiz page.
- "a control under a pet …" now uses the tallest card body that exists (`FRONT_LEAST` 56 px; measured 72.8 px) and lets the pet go `RELEASE_DEPTH` 6 px under its top.
- The cast flake in the teammate's rehearsal run was `net::ERR_NO_BUFFER_SPACE` on the page's stylesheet: Windows ran out of socket buffers under load. The page never laid out, so no pets appeared. This is not a pets defect; the spec already waits on state.

| run | result | gear spec |
|---|---|---|
| dev, the full project before the relay's layout change was handled (`gate-dev-final`) | 13 passed, 2 failed: card 72.8 px < 120; no ladder on the new home | — |
| dev, the full project (`gate-dev-final2`) | **15 passed** (7.0 min) | 359 s; ladder: battery (physics); rope: battery (home) |
| rehearsal (fresh `build`), the full project (`gate-rehearsal-final`) | **15 passed** (5.7 min) | 297 s; ladder: solary; rope: housy |
| dev, gear spec alone, earlier layout (`gate-dev-gear3…6`) | each passed | 18 s, 96 s, 92 s, 121 s |

In both final runs the mischief spec showed radiatory pushing a heating row and being thrown off on hover and on focus. No pet had to be held for the rope.

## 6. Commands and real results

From the repository root, unless a package folder is named.

| where | command | result |
|---|---|---|
| `P/📦️packages/🟦️typescript` | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts test` | 16 files, **1133 passed**, 7.1 s and 7.8 s (budget 15 s). Two earlier runs while other agents loaded the machine to 100 % CPU took 16.6 s and 19.7 s and were killed by the budget. |
| same | `… test quick` | 1133 passed, 20 s |
| same | `… typecheck` | exit 0 |
| `PR/📦️packages/🟦️typescript` | `… test` / `… typecheck` | 173 passed / exit 0 |
| `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript` | `… typecheck` (includes the e2e specs) | exit 0 |
| `P/📦️packages/🦀️rust` | `cargo check --tests --offline` (private build and target dirs, old layout, `RUSTC_WRAPPER=""`) | exit 0, 13 s |
| same | `cargo test --offline schema` | 11 passed (the `Foothold` changes in the Rust schema and its unit test) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` | `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "<case>"` and `subject exhaustive --implementation typescript …` | 🎪️stage-trace: oracle not-exercised (no-oracle case), subject 3/3. 🚧️clearance-proof 11/11 + 11/11. 🧗️wall-climbing 13/13 + 13/13. 👆️gesture-recognition 9/9 + 9/9. 🪜️ladder-geometry 6/6 + 6/6 (new scenario `leans`). 🎣️grapple-reach 8/8 + 8/8. |
| root | `.venv/Scripts/python.exe TK/generate_climbing_vectors.py`, run twice | exit 0, identical output (`climbing-vectors*.txt`). Placements 36 (18 standing), leans 12 (5 standing), shots 38 (25 granted), routes 27. |
| root | `.venv/Scripts/python.exe TK/generate_behavior_vectors.py --rerecord`, then without | wrote 🎪️stage-trace; second run: `traces as committed: 32 of 32 scripts`, `unchanged` |
| root | `bun TK/stage_fuzz.ts --seeds 48 --ticks 19200 --menagerie both --pan --name pan --out f1` | sample 2 747 713 and architecture 4 832 220 actor-ticks: **0 overlaps, 0 drawn overlaps, 0 outside**. Poofs 878 / 1450, of which bottom exits 770 / 1141. Wall / ladder / rope ticks 453 / 0 / 210 and 3570 / 1165 / 640. Hard landings with a parachute: 0. Order breaks 0 / 1 (survey + drag, B4's known case). |
| root | same with `--seeds 120 --ticks 6000 --walls --name walls` | 2 149 757 and 3 942 049 actor-ticks: **0 / 0 / 0**. Poofs 262 / 453 (bottom exits 244 / 382). Ticks 3479 / 499 / 1131 and 2937 / 856 / 1719. Hard landings with a parachute: 0. |
| root | same with `--mischief --name mischief` | 2 154 695 and 4 002 192 actor-ticks: **0 / 0 / 0**. Pranks 54 / 50. Slip-overs, now counted, are among the `perch/idle` poofs (26 / 25). Strays 0. Hard landings with a parachute: 0. |
| root | `bun TK/f1_routes.ts --dir TK/🗑️generated/f1/survey2 --page <home\|heating\|physics\|cooling\|demand> --seconds 480` | the numbers of §2 |
| root | `bun TK/f1_rope_rounds.ts --seeds 24` (earlier home layout) | rounds with a rope: 6 of 24, and 8 of 24 with explorers |
| root | `node TK/f1_code_rules.mjs` | `problems: 0` (findings that predate F1 listed as known) |
| root | `bash TK/f1_gate.sh <dev\|rehearsal> final 4` | §5 |

The fuzz tool now counts a landing only when the pet stays visible (`actor.opacity > 0`). A fall through the bottom edge that arrives anew in the same tick is a poof, not a hard landing. Before this fix the tool reported one false "hard landing with a parachute" (kettly, architecture seed 76).

## 7. Stage-trace digests: 23 of 32 moved (`TK/f1_trace_diff.ts`, `TK/f1_frame_diff.ts`)

The "before" digests are the committed traces from B5 (`stage-trace-before.json`). The first differing frame comes from
replaying each script on the scratch copy of the core in `TK/🗑️generated/f1/before`, taken before the defects were fixed.
That copy already drew the positional lean but kept it out of the body. First differing frame per script:

- **See-through (§4)**: opacity, and the rate changing from 64 to 32 where no fade is left. Scripts: moving-card (t 3854), still-and-back (7633), clicks-and-glances (2049), narrow-stage (3138), crowded-strip (1281), pointer-rest (1153), pointer-attention (2049), clicks-and-purrs (193), drag-and-drop (258), throw-into-a-crowd (386), heads (194), cancel-and-permit (194), ladder-up (4306, climber on a ladder in front of a keep-out), toppled-ladder (4306).
- **Lean in the body (§1)**: the body extent moves. Scripts: gutter-life (1068), gap-crossing (861), scrolled-climber (383), idle-learner-prank (982), reclaimed-prank (1408), scene-change-prank (2964).
- **Ropes (§2)**: corner hooks, `aims` and explorers. Scripts: scene-change (691, sparky now ropes up), new-ground (7405, sparky ropes instead of walking), grappling-rope (691, the hook sits on the corner).
- **Unchanged (9)**: calm-home, lively-card, quiet-run, tab-and-footer, circles-and-states, chemistry-and-moods, chute-landings, crowded-reseat, missed-hook.

Every law holds in every script.

## 8. The complete list of TypeScript stage and core changes since the last Rust-ported state (B1–B5 and F1)

Each entry gives the file under `P/🔨️modules` (or `P/🧪️tests`) and the functions. Merged from `📓️report2-b1.md` §7, `-b2` §8, `-b3` §7, `-b4` §8, `-b5` §6 and F1, and checked against what D1–D3 ported (`📓️report2-d1/-d2/-d3.md`). Already mirrored in Rust:

- the schema of B1–B5 and F1 (`Foothold.perch`/`facing`, no `Trip.facing`)
- validation
- `MISCHIEF_STREAM`, `CHEMISTRY_STREAM`, `GEAR_STREAM`
- behavior's activity graph, `FRIENDS`, `Limits.whim`
- `open_stage`
- `arrive` fields

Everything below is TypeScript only.

### 🧗️climbing (`🧗️climbing/🦀️.rs` is still at D1's snapshot of 16:16)
- [ ] B4: `chainOf` skips a pitch equal to one already in the line (value equality); `sighted` is public.
- [ ] F1: `LADDER_TALL` = 8, `ROPE_LONG` = 8 (constants of the 🪜️ and 🎣️ vectors).
- [ ] F1: `flanking(keepouts, pitch)` (keep-outs beginning within `WALL_LIP` on the element's side are ignored).
- [ ] F1: `stood(low, pitch, x, y, keepouts, size)` (contact `y0 + TUCK ≤ y ≤ y1`, `flanking`); `ladderFor` passes `y0 + TUCK`.
- [ ] F1: `ladderTo(low, pitch, keepouts, size)` (top `y1`, foot by `LADDER_LEAN` clamped, exit `clings`).
- [ ] F1: `ladderHolds` (tops `[y0 + TUCK, y1]`, no rim needed).
- [ ] F1: `shotFor` (spots `x0 + HOOK_INSET`, `x1 − HOOK_INSET`, clamp of the feet; middle below `2 × HOOK_INSET`); `aimed` (zip when steep, or when the drop line `hook → hook + length` hits a keep-out not under the hook).
- [ ] F1: `aims(x, from, to, size)` (x, three clamps, then `to.x0 − k·w` and `to.x1 + k·w` while `k·w ≤ ROPE_LONG·h`); `routeOf` grapple through `aims`, first hit.
- [ ] Rust adapters: 🪜️ladder-geometry gains the `leans` scenario (`ladder_to` + `ladder_at(ladder_exit)`); 🎣️grapple-reach is unchanged in shape.

### 📏️spacing
- [ ] B1: `extentAt` (rotated about the scruff; canopy box `CANOPY_SPAN` 0.75, `CANOPY_RISE` 0.7, `CANOPY_DOME` 0.5625), `extentOf`, `bodiesOf`, `obstaclesFor`, the 2D `roomsFor`, `clearway` aware of footings; `MEET_GAP`, `hindered`, `vacancy` removed.
- [ ] F1: `WALL_LEAN` (moved from the projection), `wallLeanOf(kind, footing, pitch, x)`, `extentOf` with `tilt + wallLeanOf`.

### 📝️draft
- [ ] B1: the new fields in `draftOf`/`sealed`, `Launch` with `y` and `surface`, `remove` drops claims, courses and `touched`.
- [ ] B2: `Draft.over`; `shift` douses trick and purr emitters and clears `trick`; the mind region (`trickOf`, `present`, `standing`, `feel`, `aired`, `ignite`, `douse`, `enter` with B3's `former`, `stand`, `perform`, `purr`).
- [ ] B3: `enter` sets `former` from `state_at(…, since − 1)`.
- [ ] B4: `Draft.menagerie`; `pitches`/`trips` copied; `TURN_TICKS`/`turn` moved here; `remove` drops the trip and frees the ladder being ridden; `astir`.

### 🚶️locomotion
- [ ] B1 (whole file at the time): `charted`, `decisionOf`, `wayDown` with braking (`BRACES` 4), `waysDown`, `planOf`, `install`, `arrive`, `fly`, `plunge`, `poof`/`vanish`, `unfoot`/`drop`, `launch` (lanes, `LANE_RAMP`), `stride` (guarded), `scoot`, `slip`, `lift` + `yielded`, `hold`, `letGo`, `giveBack`, `toss`/`tossTarget`, the region's constants.
- [ ] B4:
  - `BOUNCE`, `bounced`/`bonked`; the bottom edge `away` with a puff in `arrive`; `hold` checks the upright box.
  - The Gear region: `geared`, `tripOf`, `outingsTo`, `setOut`, `embark`, `travel`, `alight`, `rest`, `explored`, `onward`, `cling`, `halt`, `freed`, `ungear`, `spaced`, `lasting`/`REST_LEAST`, `restSpots`, `ladderOuting`, `ladderWays`, `ropeOuting`, `slideOuting`, `scaling`, `fromPerch`, `waysOff`, `reserveOf`, `walked`, `rungs`, `inside`, `tripClaim`.
  - `unfoot`/`lift` ungear; `yielded`/`leave` halt.
- [ ] B5: `POST_UNITS`, `holding`, `postTo`, `popTo`, `pushAt`, `unpush`, `unlifted`.
- [ ] F1, poofs and bumps: `dusted` (counts every puff; `vanish`/`poof`, `arrive` away, `popTo` use it); `bumped` and `arrive` (`vy` marks a daze after a hard bump).
- [ ] F1, the lean: `bodyAt` (lean in claims), `tripClaim`, `spaced`.
- [ ] F1, the stand of a leg: `Stand`/`standOf`; `wallward(leaning)`; `walked(stand, kind, to, activity)` with facing and perch per foothold; `rungs` with facing; `scaling(…, grip, from)`; `fromPerch(draft, index, stand, chain, start, hold, end, goal, mantle, foot, from)`.
- [ ] F1, ladders: `ladderLead`, `leanOuting`, `ladderDown` + `waysOff` (down ladders, `now + 1`), `lasting` (chain of the last pitch), `restSpots(…, stand, …, from, now)` with lean entries.
- [ ] F1, outings: `ladderOuting(…, stand, …, from)`, `ladderWays` facing, `ropeOuting(…, stand, …, from)`, `slideOuting` facing.
- [ ] F1, two-leg routes: `ranked`, `joined`, `legsTo`, `outingsTo` (two legs), `reaches`, `walkEnd`.
- [ ] F1, on the way: `embark` (first goal, turn); `travel` (footing and perch per step, goal per walk run, turn on facing change, rope in only after `since`); `halt` (rest on a ladder or wall); `postTo(…, slip, now)`; `popTo` (lean, foothold perch and facing).

### 👥️population
- [ ] B1: the `STRANDED` pre-pass, seat order by strain, clamped walk goals and seats; `carry` returns a bool; `ride` unfoots course owners and vanishes actors outside the stage; `freeze` resets press, `touched`, trail, origin.
- [ ] B2: arrival without spirits, with the resting state's plume; freezing resets feeling, warmth, hover.
- [ ] B4: `measure` pitches (`WALL_ROOM` 8, `WALL_LEAST` 0.5), `alike`, `holdOn`, `ride` (halt on reshape, side-edge clamp), `survey`/`summon` reshaped, `freeze` halts, `tune` counts trip movers.
- [ ] B5: `FIXTURE_SLACK`, `astray`, the survey's quiet end, `freeze` ends the lift.
- [ ] F1: `spread` skips circled pets.

### 👀️attention
- [ ] B1: the Hand region (`grasp`, `tossed`, `handle`, `TRAIL`, `DANGLE_TICKS`); `headingOf` and `perk` use the footing.
- [ ] B2: no `cheer`/`MOOD_EASE`/`MOOD_REST`; gaze manners (`sought`, `GAZE_SAD`, `GAZE_SLEEPY`), `PURR_TICKS`; `headingOf` with `trick`; `heed`, `loose`/`LOOSE`, `boxOf`, `touchedAt`, `guardsOf`, `hovers`, `unhovered`, `cued`, `shrug`; appraisal hooks, `shaken`.
- [ ] F1: `presenceOf(keepouts, actor, kind)` (§4); `SHY_REACH` removed; `perk` without the presence exclusion.

### 💞️sociability
- [ ] B1: the reach-based gap, `parted` by footing.
- [ ] B2: `reconcile` feels `mended`; `nudge`, `pledge`, `joins`, `redeem`; `meet` (`encounterBias`, affinity floor, `swayedShares`, pledge override, appraisals, show trick); `contagion`, `clicked` (with `loose`), `played`.
- [ ] B4: `pair` counts `astir`.
- [ ] F1: `pair` uses `sociable(…, now)`.

### 🗓️schedule
- [ ] B1: `sociable` needs footing `perch`.
- [ ] B2: `sightingsOf`, `beatAfter`, `dueTick`, `beatTick`, `sampling`.
- [ ] B5: `Post`, `Prospect`, `postOf`, `prospectsOf`, `circumstancesOf`, `prankTick` (every branch).
- [ ] F1: `sociable(actor, tick)` (not while circled); `pairingTick` counts with `from`; `prospectsOf(menagerie, stage, tick)`; `prankTick` passes `from`.

### 🎯️choice
- [ ] B1: hop through `launch`; `conclude` returns for footing ≠ perch; `land` with `vy` > 0 → `daze`.
- [ ] B2: `optionsOf` (needs settled by `needsAfter`, `drowsed`, weights × `moodWeights`); `decide` (forced only with weight > 0, `shift`, whims, trick branch); `conclude` (cuddle → purr, trick → `finish`); `finish`, `beat`, `react`.
- [ ] B4: `venturesOf`, ventures in `optionsOf`, `astir` movers, the hop branch on `GEAR_STREAM`, walk → climb (`CLIMB_SHARE`), `VENTURE_SHARES`.
- [ ] B5: `SHEEPISH`, `PUSH_HOLD`, `prank`, `mischief`, `reclaimed`.
- [ ] F1: `optionsOf` (`circled` holds: not restless, no roam); `venturesOf` (`reaches`, wall ventures through `ladderTo` for climb + ladder); `decide` (lively walk → ventures to walls or higher perches); `prank` (two passes: gear, then slip).

### 🕰️clock
- [ ] B1: `act` dispatch, the `lull` terms, the step order.
- [ ] B2: `act` (`stand` first, `woken`, no cheer); `lull` (`sampling`, `hoverBusy`, `stateEnds`, `calmsAt`, `beatTick`); `step` (`hovers` → `beat` after the actors).
- [ ] B4: `act` dispatch, `lull` (trips, ladder horizon, wall and ladder resters), ladder removal in `step`.
- [ ] B5: the `lull` horizon from `prankTick`; `mischief` after `pair` and before spawning; the empty-stage jump only without a lift.
- [ ] F1: `act` and `lull` use `presenceOf(draft.keepouts, …)`.

### 🎥️projection
- [ ] B1: the `paceOf` and `wake` terms.
- [ ] B2: `paceOf` (`hoverBusy` → 64; restless while `settlesAt > tick`); the frame's `state`/`mood`/`intensity`/`spirits`; `wake` with `stateEnds` + `beatTick`.
- [ ] B3: all of its §1–§2 in its arithmetic order: `replaces`, `weight_of`, `resting_of`, `look_of`, `keys_of`, `looked_of`, `pose_of`, `drooped`, lids, `looking_of`, `pivot_of`, `wallward`, `tilt_of`, `placed`, `staged`, `tools_of`, `held_of`, `sparks_of`/`particles_from`, `plume_pace`, `ladders_of`, `lifts_of`, `puffs_of`, `scenery_pace`, `scenery_wake`, `pace_of`, frame order, the still frame. It needs Rust twins of `particles_of`, `capped`, `emitter_ends`, `emitter_key`, `lift_at`, `lift_wake`, `lift_ends`, `hook_step`, `hook_ticks`, `ladder_rungs`, `lean_of`, `face_of`, `state_ends`, `extent_of`.
- [ ] B4: `clinging`, `clipTime`, `weightOf`, `tiltOf` from the pitch, the miss retract in `toolsOf`, `paceOf`.
- [ ] B5: `wake` includes `prankTick(…, tick + 1)`.
- [ ] F1: `tiltOf = actor.tilt + wallLeanOf(…)` (`WALL_LEAN` imported from spacing); `paceOf` presence from the keep-outs; frame opacity `actor.opacity` (no see-through on a still stage).

### 💗️feeling and 👆️gesture
- [x] B2 💗️feeling (`Sighting.held`/`trick`, `heldTicks`, `matches`, `barred`, `trialsOf`/`reactionsOf` with `rapports`, `Consequence.activity`, `calmsAt`): ported by D3, which reports a complete twin with Rust adapters for 💗️feeling-dynamics and ⚗️chemistry-rules. The same goes for ✨️effects and 🪄️mischief (D3), 🚧️clearance and 👆️gesture as of 01:35 (D2), and 🪢️swing and 🧗️climbing as of 16:13 (D1).
- [ ] F1 👆️gesture: `circled(hover, tick)` (after D2's port).

### 🎪️stage façade
- [ ] B1: `pressed`/`dragged`/`released`/`cancelled` → `grasp`; `played` toss; withdrawing permission aborts the press.
- [ ] B2: `openStage.over`; `pointed` sets `over`; `unpointed` → `unhovered`; the press path → `clicked`; `played` → `played`.
- [ ] B5: `permitted` without mischief → `unlifted`; `reclaimed` → `choice.reclaimed`.
- [ ] F1: docstring only (see-through and perking).

### Rust tests and adapters
- [ ] `🧪️tests/🎪️stage-trace/🦀️.rs`: `Sighting` with `footing`, `wall` and `body`; `drawn` per checkpoint; the 2D law `apart`; the scooter relaxations of `perched` and `clear`; the wall branch of `perched`; `fold_frame` exactly as B3 §4. Then the Rust replay of all 32 scripts (23 moved in F1, §7).
- [ ] The Rust stage suite's `fold_frame`, and `the_calm_home_replays_into_its_committed_trace` (23 traces moved in F1, §7).
- [ ] 🪜️ladder-geometry and 🎣️grapple-reach Rust subjects: stale until the climbing items above are ported. The constants differ (3.6 / 3.2 against 8 / 8).

## 9. Decisions recorded

- `ladderTo` puts its top on the pitch's **lower end** (`y1`), not `LADDER_TUCK` above it. That is the shortest ladder that leans on the wall, and the hands at its exit reach a pitch as short as heating's 31 px lowest card.
- Corner hooks (`HOOK_INSET` only, not half a width plus `HOOK_INSET`) override MECH §3's inset of w/2 + 4. The landing stays inside the perch through `landingFor`.
- `routeOf` keeps "from where it stands when that works". A shot that costs least, with a walk to its stand, was tried and dropped: pets walked for one or two pixels of rope.
- Explorers (lively walk → gear trip up, `CLIMB_SHARE`): the gear e2e went from about 6 min to 0.3–6 min (§5), with no stage test regressions.
- Ropes on the quiz pages: impossible, because those pages have no perch above the footer. They are proven on home. Ladders are proven on a quiz page, because the new home has no climbable body walls.

## 10. Open

- Rust: everything in §8.
- On the new home, the gear spec's rope still depends on a grappler deciding to go up: 297 s and 359 s in the final runs, within its 660 s.
- The pan fuzz poof rate excluding bottom exits is 39 and 64 per million actor-ticks. That is a new, survey-heavy setting with 62 000 panning surveys, and no baseline exists. The walls fuzz is 8.4 and 18 per million, against B4's 3.7 and 10.6.
- design-v2 §24.4 now records F1's as-built deviations.

## 11. Files

- **Created**:
  - `TK/f1_routes.ts`, `TK/f1_gear_probe.ts`, `TK/f1_aims_probe.ts`, `TK/f1_rope_story.ts`, `TK/f1_rope_rounds.ts`, `TK/f1_trace_diff.ts`, `TK/f1_frame_diff.ts`, `TK/f1_code_rules.mjs`
  - earlier in F1: `TK/f1_stack.sh`, `TK/f1_gate.sh`, `TK/f1_survey.mjs`, `TK/f1_survey_look.ts`, `TK/f1_emojis.mjs`, `TK/f1_before.config.ts`
  - this report
- **Changed (core)**: `P/🔨️modules/{📏️spacing, 🎥️projection, 🚶️locomotion, 🧗️climbing, 👆️gesture, 🎯️choice, 🗓️schedule, 💞️sociability, 👥️population, 👀️attention, 🕰️clock, 🎪️stage}/🟦️.ts`; `P/🧬️schema/{🟦️.ts, 🔣️.json, 🦀️.rs}`; `P/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`.
- **Changed (tests)**: `P/🔨️modules/{🧗️climbing, 🎪️stage, 🎥️projection}/🧪️tests/🔬️unit/🟦️.ts`; `P/🧪️tests/🫥️decorative-layer/🟦️.tsx`; `P/🧪️tests/🪜️ladder-geometry/{🐍️.py, 🟦️.ts, 🥒️.feature}`; `P/🧪️tests/🎣️grapple-reach/{🐍️.py, 🥒️.feature}`; `P/🧪️tests/🎪️stage-trace/🥒️.feature`.
- **Changed (vectors)**: `P/🧫️fixtures/{🪜️ladder-geometry, 🎣️grapple-reach, 🧗️wall-climbing, 🎪️stage-trace}/🔣️.json`.
- **Changed (React)**: `PR/🔨️modules/🫧️layer/🟦️.tsx`, `PR/🔨️modules/📡️survey/🟦️.ts`, `PR/📖️stories/🟦️.tsx`.
- **Changed (e2e)**: `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts`.
- **Changed (docs)**: `P/README.md`, `P/📦️packages/🦀️rust/Cargo.toml`, `TK/📓️design-v2.md` (§24.4).
- **Changed (ticket tools)**: `TK/stage_fuzz.ts` (pan mode, drawn boxes, poof kinds, landing fix), `TK/generate_climbing_vectors.py` (leans, new boundaries), `TK/b5_explore.ts`, `TK/b5_debug.ts`.
