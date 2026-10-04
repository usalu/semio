# 📓️ Work package B3 (second round) — projection: the second-round frame

Ticket `2026/10/02/QUIZ-PETS`, second round, design-v2 §15, §17, §19–§21, §23. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Tool output: `TK/🗑️generated/b3/` (every log named below is there).

**State: done (TypeScript).** `frameOf` projects every field of the second round. Work ran 01:20–02:21 (reading, waiting for R0) and 12:19–13:40 after the rate-limit cut; R0's contract and B1/B2's integrations were in flight in the same files the whole time, so every shared file was re-read right before each anchored edit. The Rust projection is **phase D** (not cheap: it needs Rust twins of `✨️effects`, `🪄️mischief`, `🧗️climbing`, `🪢️swing`, `📏️spacing` that do not exist yet); only the Rust schema twin and what keeps the crate compiling were changed.

## 1. Every frame field and how it is computed (`P/🔨️modules/🎥️projection/🟦️.ts`)

### 1.1 Per actor (`ActorFrame`)

| Field | Computed as | Still stage |
|---|---|---|
| `x`, `y` | where the rig's origin is placed **before** the drawing turns: `placed(feet, facing, tilt, pivot)` = `feet + R(tilt)·p − p` with `p = (facing·pivot.x, pivot.y)`, `R` the clockwise turn; the turn about the pivot then carries the rig's feet exactly onto the actor's feet (gl-matrix judges it to 1e-9). While `tilt` is 0, `x, y` are the feet. | the feet |
| `footing` | `actor.footing` | same |
| `state` | `stateAt(species, actor.state, actor.stateSince, tick)`; during the first half of `STATE_BLEND` (8 ticks) after a change the **former** state, so the tint switches in the middle of the blend | the present state |
| `mood`, `intensity` | `settled(actor.feeling, species.mood, tick)` — the raw mood and intensity (a target that wants "is it grumpy?" thresholds at `MOOD_REST`, i.e. `shownMood`) | `atRest(species.mood)`: resting mood at 0.25 (time-invariant) |
| `spirits` | `spiritsOf(settled feeling)` = `faceOf(...).bend` (the mouth) | spirits of the resting feeling |
| `eyes[].lid` | `faceLid + (1 − faceLid) × lidAt(tick − blink)` (the mood's resting lid height with the blink on top), 1 asleep | 0 |
| `eyes[].x/y` | as before (gaze × `pupilReach`) | 0 |
| `bones` | `solveRig` of `poseOf` (§1.3), then `drooped` (the bone of the first eye sinks by `faceOf(...).drop`), then `leant` outside quiet, then the squeeze while turning | rest pose in the look of the state (state clip at phase 0), no drop |
| `tilt` | `actor.tilt` (B1: hanging from the hand = `leanOf(grip, bob, grip)`, tumbling, righting itself; all about the scruff) plus, on footing `wall`, `±WALL_LEAN` (0.04 turns) towards the wall it clings to (nearest surveyed wall whose clinging place lies within half a width; else the way it faces), given up over a `mantle` by `1 − smoothstep(elapsed/span)` | 0 |
| `pivot` | the scruff `{0, −grip}`; on footing `rope` the hands `{MUZZLE_FORWARD·w, −MUZZLE_HEIGHT·h}` (A4's hold, where the rope attaches) | as computed |
| `tools` | in A9's layering order: `chute {open: chute.open, sway: leanOf(canopy, feet, CHUTE_ROD·h)}`; `ladder {lean: facing·CARRY_LEAN (0.22), length: CARRY_LENGTH (2)·h}` while `carry`; rope out: while the hook flies (`tick − rope.since < hookTicks(muzzle, hook, HOOK_SPEED)`) `rope`+`hook` at `hookStep(...)` and `gun {aim: atanTurns(hook − muzzle)}`, after the bite `rope`+`hook` at `shot.hook`, slack 0; `aim` without a rope: `gun {aim: −AIM_RISE}` facing right, `AIM_RISE − ½` facing left | `[]` |
| `body` | the clearance body: `extentOf(actor, kind)` of `📏️spacing` (B1; size box, hover, tilt about the scruff, canopy, margin 4) as `{x: x0, y: y0, width, height}` | same |

### 1.2 Stage (`Frame`)

| Field | Computed as |
|---|---|
| `actors` | computed in menagerie order, then sorted back to front by the frame's own `y`, then species id (the `whole` law reads the frame's `y`) |
| `particles` | for every actor in drawing order, every plume in `actor.emitters`: the emitter at its bone — `(e.x, e.y)` through the bone's matrix of this frame (after squeeze), mirrored, turned and placed like the drawing (`staged`) —, `particlesOf(emitter, origin, facing, plume.since, plume.until, tick, emitterKey(stage.seed, speciesIndex, emitterIndex, plume.since))`; all of them through `capped(·, STAGE_CAP = 160)` (the youngest stay, earlier in drawing order wins ties); opacity × the actor's frame opacity; a plume of an unknown emitter shows nothing; none on a still stage |
| `ladders` | per `Stage.ladders`: `{x0, y0: foot, x1, y1: top, rungs: ladderRungs (⌊length ÷ 10.5⌋), opacity}`, opacity `min(smoothstep((tick − since)/8), smoothstep((until − tick)/8))`, not drawn at 0; whole on a still stage while standing |
| `lifts` | `Stage.lift` → `liftAt(since, tick, side, room, span, unit)` with the fixture id, from `since` to `since + LIFT_TICKS`; none on a still stage |
| `puffs` | **new** (`PuffFrame {x, y, width, height, phase}`, all three twins): every `Stage.puffs` entry with `phase = (tick − puff.tick) ÷ PUFF_TICKS` in [0, 1); none on a still stage |
| `held` | the first actor (menagerie order) with `hang !== null`; `null` on a still stage |

### 1.3 Pose composition (the coordinator's two notes)

1. `idle` = the first idle clip on the stage clock (`tick + stream × 37`), or rest.
2. Activity clip `top` (when it is not the idle clip): `weight` as before (fade in/out 8 ticks; walk **and scoot** by distance; flights **hop, fall, tumble** end hard); `over = blendPose(rest, top at tick − since, weight)`.
3. Replacing activities (`walk hop fall land sleep` + `tumble scoot carry climb slide mantle`) lay **only the idle loop** to rest: `blendPose(idle, rest, restingOf)`, where `restingOf` = the weight, except `land`, which starts with the loop at rest and only hands it back as it ends (`clamp((until − tick)/8)`) — no jump between a flight and its landing.
4. **The look of the state** (`lookedOf`): every (bone, channel) the state's clip keys takes that clip's value (sampled on the same stage clock as the idle loop); every other channel keeps the underlay; blended channel by channel from the former look over `STATE_BLEND` = 16 ticks (`smoothstep`), `from + (to − from)·blend`, exactly `to` once done. The look stays under replacing clips (H3 §7.2).
5. `layer(look, over)` (offsets add, scales multiply), then `drooped` (mood posture), `leant` (gaze), squeeze.

The former state: `Actor.former` (new, all three twins), set by `📝️draft.enter` to the state the actor was in just before `since` (`stateAt(stored, since − 1)`); when a lasting state has given way since the stored anchor, the frame derives it the same way. Arrival: `former = state`.

### 1.4 The face (`faceOf`)

`bend` → `spirits`; `lid` → combined into `EyeFrame.lid`; `drop` → the pose. **`slant` is not a frame number:** it is `faceOf({mood, intensity, since: 0}).slant` of the frame's `mood` and `intensity` (degrees; positive lowers the end of the lid towards the other eye). Decided against `EyeFrame.slant`/`ActorFrame.slant`: one face table in the core, nothing duplicated in the frame, and no churn of every `EyeFrame` literal in `PR` while C1 works there.

## 2. Rate and wake

| Source | Rate | Wake (at rate 0) |
|---|---|---|
| an actor not on a perch (B1's rule), walking, hopping, falling, landing, scooting, **carrying**, with its **rope out**, during a **state blend** (16 ticks, when either look has a clip) | 64 | — |
| a **brisk plume** (a `burst`, or `speed ≥ BRISK` = 32 px/s: particles move ≥ ½ px per tick) begun and not yet at `emitterEnds` | 64 | — |
| a **slow plume** (`speed < 32` px/s, drifting) | 32 | — |
| a looping state clip (like the idle loop) | 32 (16 asleep) | — |
| the lifted copy moving (`liftWake(since, tick) === tick + 1`), a ladder in its fade window, a puff of dust spreading | 64 | — |
| resting lift | 0 | `liftWake` (start of putting back), `liftEnds` |
| ladder standing | 0 | its `since` (future) or `until − LADDER_FADE` |
| plumes | (live ones keep ≥ 32) | a plume's `since` (future) and `emitterEnds` |
| a lasting state | | `stateEnds` of the **present** standing (was the stored anchor) |
| still stage | 0, no wake, no particles at all | — |

Kept from B1/B2: `footing !== "perch"` → 64, `hoverBusy` → 64, restless while `settlesAt > tick`, the press due, puffs' end, `beatTick`. The wake gate also opens for a stage with ladders or a lift and no actor.

**`🗓️schedule` and `🕰️clock` were not changed:** particles, lifts, ladders, puffs and looks are pure functions of the tick, so the stage need not step for them (a lull that jumps over them still ends in the same frame); only frame horizons (rate/wake) need them, and they live in `🎥️projection`. Whoever ends a lift or takes a ladder away at a tick (B4/B5) adds that stage change to `lull`.

## 3. Order and depth

Actors back to front as before (by the frame's `y`). Tools in A9's order (back group: parachute, carried ladder; front group: rope, hook, gun). **No `depth` for particles:** A9's painter has one particle root (`stageEffects`, the layer's last child), so every particle is drawn in front of every actor; an orbit's front/back reads through its scale. If behind-particles are wanted later, the frame needs a per-particle depth and the painter a second root.

## 4. Digests (stage-trace)

The TypeScript adapter (`P/🧪️tests/🎪️stage-trace/🟦️.ts`) folds, after the first-round numbers of each actor: the indices of footing (`FOOTINGS`), state (among the species' states) and mood (`MOODS`), intensity, tilt, pivot x/y, the number of tools and per tool its kind index (chute 0, rope 1, hook 2, gun 3, ladder 4) and its numbers, body x/y/width/height; then per frame the ladders (x0, y0, x1, y1, rungs, opacity), the particles (species index, emitter index, x, y, scale, rotation, opacity), the lifts (dx, dy, tilt, opacity — the fixture id, a string, is left out), the puffs (x, y, width, height, phase), and the held species index (−1). Every checkpoint also records `drawn: {particles, ladders, lifts, puffs, held}`. The Python keeper (`🐍️.py`, `verify_scenery`) checks per checkpoint: whole non-negative counts, ≤ 160 particles, ≤ 1 lifted copy, nothing (no dust, nobody held) on a still stage, and a held pet that is on stage with footing `hand`. The feature text says so. The Rust adapter and the Rust stage suite's `fold_frame` follow in phase D.

**Which digests moved and why — all of them, in two steps:**

| Script | HEAD (first round) | 13:07 (B2's re-record, with my fold) | final (13:27) |
|---|---|---|---|
| calm-home | 1758177737 | 3523489951 | 1993928575 |
| lively-card | 3055031182 | 594139635 | 4013533459 |
| moving-card | 3448150708 | 2268799631 | 2798023599 |
| quiet-run | 811069282 | 4264546285 | 2250908941 |
| still-and-back | 2032106604 | 2376239238 | 272470662 |
| scene-change | 1529883008 | 261852663 | 1067382583 |
| narrow-stage | 1073395578 | 917808373 | 650297045 |
| crowded-strip | 2275531786 | 3892383556 | 3495807588 |
| tab-and-footer | 1444943760 | 3898719359 | 2204155139 |
| pointer-rest | 3777510391 | 2762664196 | 1058712548 |
| pointer-attention | 3326711053 | 1730361224 | 3866680776 |
| new-ground | 3213029212 | 1997825983 | 3028183423 |
| clicks-and-glances, clicks-and-purrs, circles-and-states, chemistry-and-moods, drag-and-drop, throw-into-a-crowd, chute-landings, heads, crowded-reseat, cancel-and-permit (new scripts of B1/B2) | — | 885294772, 345130795, 347135987, 2091597923, 1915040480, 1391873144, 1388082075, 2757314553, 2618097657, 667328318 | 3058049844, 2031924555, 4000356723, 700410868, 3192043328, 2717229240, 3376849051, 3856985, 1727052441, 937263518 |

Why: (1) every frame now folds the new fields (so every digest moves even where nothing new happens); (2) the frame's content changed for every actor — lids carry the mood's resting height (`lidded` in 100 % of open-eyed frames), spirits come from the feeling (B2), the head sinks or lifts by the resting mood (mossy, pebble, misty …), bodies are clearance bodies, a landing starts from the rest pose; (3) B1/B2 behaviour and new scripts; (4) the final step (13:07 → 13:27) is the puff count folded into every frame plus B1/B2 changes in between (`tab-and-footer`, `chemistry-and-moods` changed lids/rates, i.e. behaviour). What the 22 scripts exercise (`bun TK/b3_trace_survey.ts`, log `trace-survey-2.log`): particles in 16 scripts (up to 1867 frames in chemistry-and-moods; the cap is never reached in a trace), states other than resting in 3, chute tools in 6, held pets in 4, tilt in 3, footings air/chute/hand/head; **no rope, wall, ladder, lift or puff yet** (B4/B5 and poofs do not occur) — those fields are covered by the unit suite.

## 5. Performance (`bun TK/b3_measure.ts 4000`)

10 actors (blobby clones), 8 of them running one emitter each (count 32, life 1 s: fall, rise, drift, orbit, fall, rise, drift, burst), so ≈ 250 particles are computed and the cap of 160 bites; `frameOf` on consecutive ticks after a 200-frame warm-up:

| Run | no plumes | 8 plumes, cap reached |
|---|---|---|
| 13:07 (host loaded by siblings) | 123.7 / 134.6 µs | 551.8 / 360.2 µs |
| 13:33 (final) | **35.2 / 32.5 µs** | **118.1 / 136.3 µs** |

So the particles cost ≈ 85–100 µs per frame at the cap (≈ 0.6 % of a 16 ms frame at rate 64).

## 6. Commands and real results

From the repository root unless a directory is named.

| Command | Result |
|---|---|
| `P/📦️packages/🟦️typescript`: `bun ./📜️script.ts typecheck` | exit 0 (`final-typecheck.log`) |
| same: `bun ./📜️script.ts test` | exit 0, `Test Files 16 passed (16)`, `Tests 1078 passed (1078)`, 10.8 s (`final-test.log`). An earlier run at 12:52 was killed by the 15 s budget while siblings' stage-suite work was in flight (the stage suite alone needed 20 s then); the final run fits. |
| same: `bun ./📜️script.ts test quick` | exit 0, `16 passed`, `1078 passed`, 27.4 s (`final-test-quick.log`) |
| same: `bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🎥️projection"` | `Tests 46 passed (46)` (`test-projection-3.log`) |
| `PR/📦️packages/🟦️typescript`: `bun ./📜️script.ts typecheck` / `test` | exit 0 / `Test Files 8 passed (8)`, `Tests 166 passed (166)` (`final-react-*.log`) |
| `TEST`: `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🎪️stage-trace"` | `executed=0 … not-exercised=1` — "recorded no-oracle decision pets-stage-trace — its evidence is discharged by the subject phase" (the Python keeper runs inside the generator, see below) |
| `TEST`: `… subject exhaustive --implementation typescript … --case "🎪️stage-trace"` | `executed=3 passed=3 failed=0 errored=0` |
| `TEST`: `… oracle exhaustive … --case "✨️particle-motion"` | `executed=6 passed=6 failed=0 errored=0` |
| `TEST`: `… subject exhaustive --implementation typescript … --case "✨️particle-motion"` | `executed=6 passed=6 failed=0 errored=0` |
| `TEST`: `… parity exhaustive --implementation typescript … --case "🎪️stage-trace"` (extra) | 3/3 passed, but exit 1: "claims the independent-implementations substitute but only one implementation ran" — expected until the Rust twin is re-synced (phase D) |
| `.venv/Scripts/python.exe -X utf8 TK/generate_behavior_vectors.py --rerecord` | `wrote …🎪️stage-trace\🔣️.json` (the keeper's `verify` passed on all 22 scripts); the other two fixtures `unchanged` (`generate-6.log`). A first attempt at 13:16 failed inside a sibling's half-saved `🎯️choice` (`decide is not defined`) and wrote nothing. |
| same without `--rerecord` (second run) | `traces as committed: 22 of 22 scripts`, `unchanged …🎪️stage-trace\🔣️.json` (`generate-7.log`) |
| `RUSTC_WRAPPER="" cargo test -p semio-framework-pets --features sut --offline` | `203 passed; 2 failed` — the two failures are the stage-trace replays (`the_calm_home_replays…`, `quick::every_script_replays…`: the Rust stage lacks B1/B2/B3, phase D); the schema twin tests pass with `former` and `PuffFrame` (118 `$defs`) |
| `rustfmt --check` on the five touched Rust files | my lines clean; the only diffs are two pre-existing long lines of the schema test (not mine) |
| `node TK/b3_code_rules.mjs` | every B3 file clean (docstring emojis unique, no console/`[DEBUG]`, no inner comments, no forbidden maths in the projection); the 2 findings are the first round's `▭️` in both schema twins (U+25AD is no pictograph; not mine) |

## 7. What the Rust twins must mirror (phase D)

1. **Schema (done in Rust already):** `Actor.former: Slug`; `PuffFrame {x, y, width, height, phase}`; `Frame.puffs`.
2. **`📝️draft::enter`:** `former = if since > state_since { state_at(kind, state, state_since, since - 1).state } else { state }` before the state is overwritten (when B2's mind is mirrored). Arrival: `former = state` (done).
3. **`🎥️projection` (all of §1–§2), in this exact arithmetic order:** `replaces` (+ tumble, scoot, carry, climb, slide, mantle); `weight_of` (flights hop/fall/tumble end hard; walk and scoot by distance); `resting_of` (`land`: `clamp((until − tick)/8, 0, 1)`); `look_of`; `keys_of` (channel index x 0, y 1, rotation 2, scaleX 3, scaleY 4); `looked_of` (`done = blend >= 1 || former == present` by identity of the clip; `from + (to − from) * blend`, `to` when done); `pose_of(kind, actor, tick, stream, present, former, blend)` with `blend = smoothstep((tick − since) / 16)`; `drooped`; lids `face_lid + (1 − face_lid) * lid_at(tick − blink)`; `looking_of`; `pivot_of`; `wallward` (best starts at `w/2`, `gap <= best`); `tilt_of`; `placed` (`feet.x + (cos * across − sin * pivot.y) − across`, `feet.y + (sin * across + cos * pivot.y) − pivot.y`, `across = facing * pivot.x`; identity when tilt is 0); `staged`; `tools_of` (order chute, ladder, rope, hook, gun; `0 − AIM_RISE`, `AIM_RISE − 0.5`, `facing * CARRY_LEAN`); `held_of`; `sparks_of` / `particles_from` (drawing order, `capped`, opacity product); `plume_pace`; `ladders_of`, `lifts_of`, `puffs_of`, `scenery_pace`, `scenery_wake`; `pace_of` (carry, rope, state blend, plumes, looping looks); frame order by the frame's `y`; `state` switch at `tick − since < 8`; still: `at_rest` face, look at phase 0, no tilt/tools/particles/lifts/puffs/held. Needs the Rust twins of `particles_of`, `capped`, `emitter_ends`, `emitter_key`, `lift_at`, `lift_wake`, `lift_ends`, `hook_step`, `hook_ticks`, `ladder_rungs`, `lean_of`, `face_of`, `state_ends`, `extent_of`.
4. **Stage-trace `🦀️.rs` adapter and the Rust stage suite's `fold_frame`:** fold exactly as §4 (tool kind indices, `-1` for none) and record `drawn` per checkpoint.

## 8. What C1 needs to know to draw it

- **Placement:** `tiltedPlacement(frame.x, frame.y, facing × size, size, frame.tilt, frame.pivot)` — `x, y` are the rig's origin **before** the turn, the feet only while `tilt` is 0; `paintTools(equipment, frame)` takes the actor frame as it is (`x, y, facing, tilt, pivot, tools` are A9's `GearBearer`).
- **Hit tests:** `frame.body` (the solid box, margin included), not `x, y` + size.
- **Tint:** by `frame.state` (it switches in the middle of a 16-tick look blend); the look itself is already in the bones.
- **Face:** the mouth reads `spirits` (wired by R0); lids are already at the mood's height in `eyes[].lid`; the **lid slant** is `faceOf({ mood, intensity, since: 0 }).slant` (exported by `@semio-tech/pets`), degrees, positive lowers the end towards the other eye; one-sided marks can use `shownMood(mood, intensity)`.
- **Particles:** `paintEffects(effects, kinds, frame.particles, size)` — all in front of the actors; at most 160.
- **Ladders:** `paintLadders(rack, frame.ladders, size)`; **lift:** `frame.lifts` (at most one) for `🪞️lifting`; **held:** `frame.held` → grab cursor and pointer capture.
- **Dust (new):** `frame.puffs` — draw a domain-neutral dust cloud at `(x, y)` of about `width × height`, spreading and fading with `phase` 0 → 1 (0.75 s); A9 has no painter for it yet.
- `rate` and `wake` already cover every moving tool, particle, ladder fade, lift and puff.

## 9. Other changes, decisions, deviations

- **Shared files touched (anchored edits, re-read before each):** `P/🧬️schema/{🟦️.ts, 🔣️.json, 🦀️.rs}` (`Actor.former`, `PuffFrame`, `Frame.puffs`, docstrings); `P/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` (fixtures, `$defs` 117 → 118); `P/🔨️modules/📝️draft/🟦️.ts` (`enter`); `P/🔨️modules/👥️population/{🟦️.ts, 🦀️.rs}` (arrival `former`); `P/🔨️modules/🧠️behavior/🧪️tests/🔬️unit/🦀️.rs` (an actor literal); `P/🔨️modules/🎥️projection/🦀️.rs` (`puffs: Vec::new()`); `P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` (blinking measured against the mood's resting lid; the frame test's body = `extentOf`; the lean test includes the mood's drop; the circling test reads the former state before the mid-blend; the empty frame has `puffs`); `P/🧪️tests/🤏️pet-handling/🟦️.tsx` (C1's fake frame gets `puffs: []`); `P/README.md` (layout row, frame paragraph); `TK/📓️design-v2.md` (new §24.1 "As built — projection").
- **Decisions:** frame `x, y` = rig origin before the turn (A9's painter, B1's scruff tilt); body = B1's clearance extent; particles keep no depth; slant derived, not a frame number; still frames are time-invariant (resting face, look at phase 0); `BRISK` 32 px/s decides 64 vs 32; carried ladder 2 heights, aim raised ⅛ turn without a shot; the wall lean is drawing-only (B4 may move it into `Actor.tilt` — then remove the wall branch of `tiltOf`); rope slack is 0 (taut) also while the hook flies; a miss/return of the hook is not projected (B4 has no miss state yet).
- **Not done / open:** the Rust projection and adapters (phase D, §7); a dust painter in `PR` (C1); the schema catalog (`bun ./📜️script.ts schema generate`) was not regenerated after `former`/`PuffFrame` (repo-wide, it rewrites other tickets' entries — coordinator's call); traces exercise no rope, wall, ladder, lift or puff yet (B4/B5 scripts will).
- **Ticket tools:** `TK/b3_measure.ts` (performance), `TK/b3_trace_survey.ts` (what the traces exercise), `TK/b3_code_rules.mjs` (code rules). No git command was run; nothing was installed.
