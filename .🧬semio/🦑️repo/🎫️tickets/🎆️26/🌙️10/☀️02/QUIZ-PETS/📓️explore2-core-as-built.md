# Explore 2 - the pets core as built (read-only survey for the next simulation round)

Ticket `2026/10/02/QUIZ-PETS`. Written 2026-10-02 by a read-only explorer (nothing run: no build, test, server or cargo; nothing but this file written). Everything below was read from the files named; where a figure comes from a ticket report instead of from running something, the report is named.

Abbreviations (every emoji in a path is the emoji plus U+FE0F; copy, never retype):

| Short | Path |
|---|---|
| `P` | `🧰️framework/🛍️products/🐾️pets` |
| `SCH` | `P/🧬️schema/🟦️.ts` (types), `SCH.json` = `P/🧬️schema/🔣️.json`, `SCH.rs` = `P/🧬️schema/🦀️.rs` |
| `STG` | `P/🔨️modules/🎪️stage/🟦️.ts` (1728 lines, 94 functions); `STG.rs` = `.../🦀️.rs` (2215 lines, 96 functions) |
| `BEH` | `P/🔨️modules/🧠️behavior/🟦️.ts` (292 lines); `TER` = `.../🏞️terrain/🟦️.ts` (185); `ANI` = `.../🎞️animation/🟦️.ts` (169); `RIG` = `.../🦴️rig/🟦️.ts` (137); `RND` = `.../🎲️randomness/🟦️.ts` (128); `TRI` = `.../📐️trigonometry/🟦️.ts` (86); `VAL` = `.../✅️validation/🟦️.ts` (426) |
| `STU` | stage unit suite `P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` (1784 lines); other suites live at `<module>/🧪️tests/🔬️unit/🟦️.ts` |
| `TRC` | `P/🧪️tests/🎪️stage-trace/` (`🟦️.ts` 220 lines, `🐍️.py` 179, `🦀️.rs` 310, `🥒️.feature` 66) |
| `LAY` | `P/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx` (442); `SUR` = `.../📡️survey/🟦️.ts` (333); `DEP` = `.../🖌️depiction/🟦️.ts` (291); `PAC` = `.../⏲️pacing/🟦️.ts` (159); `CSS` = `P/🎯️targets/⚛️react/🎨️.css` |
| `TK` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS` |
| `GEN` | `TK/generate_behavior_vectors.py` (604 lines) |

---

## 0. The facts that matter most

1. The core is a pure fold with exactly two public functions beyond `openStage`: `advance(menagerie, stage, events)` (`STG:1588`) and `frameOf(menagerie, stage)` (`STG:1669`). Everything else in `STG` is private. The barrel `P/📦️packages/🟦️typescript/🟦️.ts` is an explicit list of `export *` lines per module, so a new sub-module that is not listed stays internal.
2. **Actor identity is the species id.** `draftOf` keeps at most one actor per species (`STG:130-137`, `break` after the first match), `partner` is a species slug, the random stream of an actor is the species' index in `menagerie.species`. Two pets of one species cannot exist; a rope, ladder or parachute owned by "a pet" is owned by a species slug.
3. **`Actor.perch === null` means "in the air or without ground"** and is overloaded: hop flight, fall, float glide, crowded-out/leaving without a perch, and orphan states. Poke (`STG:1553`), pairing (`STG:955`), perk (`STG:911`), the `perched`/`clear`/`apart` laws (`TRC/🟦️.ts:101`, `:114-121`) all treat `perch === null` as "not grounded, exempt".
4. **The spacing guarantee is grounded-only, same-perch-only, x-only.** Law `apart` (`TRC/🟦️.ts:114-121`, mirrored in `TRC/🐍️.py` and `TRC/🦀️.rs`) and the helper `overlapping` in `STU:1020-1029` only compare actors with equal non-null `perch`. Airborne actors (hop, fall, float glide, up to 256 ticks), actors on different perches, and any y overlap are unchecked. Float glides additionally ignore keep-outs (`STG:621-629`).
5. **The activity set is a closed 11-element enum whose order is load-bearing in at least ten places** (schema TS/JSON/Rust, `Repertoire` struct and JSON object, `BEH` tables, `activityWeights` literal arrays, Python oracles, `GEN`, the digest of the stage trace, the depiction slot, the validation test of the sample menagerie, the strongly-connected-graph test). Appending is cheaper than inserting: the digest folds `ACTIVITIES.indexOf(activity)` (`TRC/🟦️.ts:61`), so appended activities leave all 13 committed traces bit-identical while nothing uses them.
6. **`frameOf` can only describe actors**: `Frame = { tick, actors, rate, wake }`, `ActorFrame` = feet, facing (+-1), activity, opacity, 6 numbers per bone, eyes, mood. No rotation of the whole body, no per-part opacity or palette, no props. The renderer (`LAY:192-216`, `DEP:193`) iterates `next.actors` only, one SVG per species.
7. **Three places must agree whenever something moves or is scheduled**: `lull` (`STG:1085`), `paceOf` (`STG:1644`) and the `wake` horizon in `frameOf` (`STG:1710-1725`). A change that is not listed in all three is either drawn at the wrong rate or jumped over. The cut-invariance tests (`STU:1644`, `TRC` scenario `determinism`) are the safety net.
8. **Pointer model is minimal**: one point (`Stage.pointer`), the tick of the last move (`Stage.pointed`), no press state, no buttons, no velocity, no history; `poked` is one `pointerdown` that hit an actor box (`SUR:305-312`). The layer takes no pointer events (`CSS:9`, `LAY:348`) and its window listeners are passive (`SUR:319`), so nothing can be grabbed, dragged or suppressed today.
9. **Mood is output only**: eased towards `moodOf(activity)` (`STG:873-878`), +0.3 on a poke, drawn as mouth bend. It feeds no decision. Decisions read needs (energy, curiosity), mode, quiet, crowd, roam/hops, watched.
10. **The Rust twin is a hand-made 1:1 mirror** (same regions, same function names in snake_case, same expression order). Its proof is bit parity on 13 scripts (128 000 ticks) plus a 20-minute long run (48 sessions, 57 600 checkpoints) kept outside the repo gates in ticket tools. There is no code generation; every core change is two edits. Rust stage unit tests: 31 (TS: 93 `it` registrations, 90 of them inside `describe.each` over two menageries).
11. **The refactor "split `STG`" is provably behaviour-free**: the 13 traces are regenerated from the TS subject and a second run must print `unchanged`. Section 5 proposes 11 modules with a verified acyclic dependency graph (checked by script on the call graph of section 2).
12. Several as-built rules directly contradict the new wishes (section 7): quiet ignores pokes, pets turn see-through under the pointer (including the one being held), the layer never takes pointer events, poke needs a grounded actor, one encounter at a time, only `idle` decides.

---

## 1. Schema inventory

`SCH` (302 lines) restates `SCH.json` (681 lines, draft-07, `$id` `https://json.schemas.assets.semio-tech.com/framework/product/pets/schema.json`, objects closed). `SCH.rs` is the serde twin. A unit test pins `Object.keys(SCHEMA.$defs) == exported type names of SCH` (`VAL` suite, `describe("contract twins")`, line 275-280): **every new exported type needs a `$defs` entry**, and the Rust twin needs the struct/enum. The Stage/Actor/Frame types are *not* covered by the `schema-conformance` Protocol v2 case (grep: no Actor/Stage/Frame there); they are covered by that `$defs` test, the Rust schema unit tests (`SCH` test file `🧬️schema/🧪️tests/🔬️unit/🦀️.rs`, `actor()` line 78, `stage()` line 82, round-trip tests at 167 and 281) and indirectly by the trace fixtures (events are decoded in Rust by serde with `deny_unknown_fields`).

Legend for the *Class* column: **doc** = authored document type (menagerie side), **geo** = geometry/terrain value, **ev** = stage event, **state** = runtime state of the fold, **frame** = output.

### 1.1 Scalars and constants (`SCH:11-71`)

| Export | Class | Content |
|---|---|---|
| `Slug` | doc | kebab-case id string (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1..64) |
| `Text` | doc | `{ en, de }` (no default language); `LANGUAGES = ["en","de"]` |
| `Degrees`, `Turns`, `Ticks`, `Color` | doc/state | number aliases; `Ticks` whole ticks; `Color` lowercase `#rrggbb` |
| `TICKS_PER_SECOND = 64` | const | one tick = 1/64 s (exact binary fraction) |
| `PAINTS` / `Paint` | doc | `body, accent, detail, ink, paper, none` |
| `CHANNELS` / `Channel` | doc | `x, y, rotation, scaleX, scaleY` (the only animatable channels: no opacity, no colour) |
| `GAITS` / `Gait` | doc | `walk, hop, float` |
| `ACTIVITIES` / `Activity` | doc+state | `idle, fidget, walk, hop, fall, land, sleep, greet, cuddle, squabble, sulk` (index order is load-bearing) |
| `PET_MODES` / `PetMode` | state | `still, calm, lively` |
| `MENAGERIE_SCHEMA`, `ENSEMBLE_SCHEMA` | doc | `semio.pets.menagerie/v1`, `semio.pets.ensemble/v1` |

### 1.2 Authored documents (`SCH:73-180`)

| Type | Fields | Notes |
|---|---|---|
| `Bone` | `id, parent?, x, y, rotation?` | parents first; the first bone is the root at the feet |
| `PathShape`, `EllipseShape`, `RectShape`, `LineShape`, `Shape` | `d` / `cx cy rx ry` / `x y width height radius?` / `x1 y1 x2 y2` | SVG-ish; **the core never parses `d`** (no path bounds anywhere in the core) |
| `Part` | `id, bone, shape, fill, stroke, strokeWidth?` | drawn back to front; no visibility/opacity field |
| `Eye` | `id, bone, x, y, radius, pupil` | pupil < radius |
| `Mouth` | `bone, x, y, width` | bends with `mood` (renderer only) |
| `Face` | `eyes[], mouth?, above?` | |
| `Ease`, `Key`, `Track`, `Clip` | `[x1,y1,x2,y2]`; `at,value,ease?`; `bone,channel,keys`; `id,seconds,loop,tracks` | values are offsets from rest; `loop-seam` rule |
| `Repertoire` | optional `Slug[]` per **activity name** (11 properties in JSON, an 11-field struct in Rust) | `clipsOf` falls back to hop clips (walk of a hop gait), then idle clips |
| `Size` | `width, height` | the rest box, stroke and idle loop included; the **only** body extent the core knows |
| `Palette` | `body, accent, detail` | set once per depiction as CSS custom properties |
| `Locomotion` | `gait, speed (px/s), hover?` | `hover` exactly when `float` |
| `Temperament` | `energy, sociability, curiosity` in [0,1] | seeds `Needs`, scales drains/gains |
| `Species` | `$schema?, id, name, thing, grounds[], size, palette, bones[], parts[], face, clips[], repertoire, locomotion, temperament` | closed object; 20 hand-made species in the architecture menagerie |
| `Bond`, `Cast` | `between[2], affinity`; `scene, core[], rotation[]` | |
| `Menagerie`, `Ensemble` | `schema, id, title, species, bonds, casts` (+`$schema?`) | `assembleMenagerie` drops the `$schema` hints |

### 1.3 Terrain values (`SCH:182-194`)

| Type | Fields | Class |
|---|---|---|
| `Point` | `x, y` | geo |
| `Rect` | `x, y, width, height` | geo |
| `Surface` | `id, x0, x1, y` (a **top edge**; no vertical edges exist in the model) | geo |
| `Perch` | `surface, x0, x1, y` (a free stretch of a surface; derived by `measure`) | geo/state |

### 1.4 Events (`SCH:196-225`), the only inputs of the fold

| Type (`kind`) | Fields | Effect (details in section 2.3) |
|---|---|---|
| `Ticked` (`ticked`) | `ticks` | `pass`: per-tick work with lull jumping |
| `Pointed` (`pointed`) | `x, y` | store pointer + tick |
| `Unpointed` (`unpointed`) | - | pointer = null (the tick `pointed` stays) |
| `Glanced` (`glanced`) | `points[]` | store (other people's cursors) |
| `Surveyed` (`surveyed`) | `width, height, surfaces[], keepouts[]` | store; recut perches; ride; spawn; spread |
| `Summoned` (`summoned`) | `species[]` | wanted list; leavers; arrivals |
| `Tuned` (`tuned`) | `mode` | freeze / thaw / mover limit |
| `Hushed` (`hushed`) | `quiet` | flag only |
| `Poked` (`poked`) | `x, y` | nearest grounded actor within 1.2 heights greets |
| `StageEvent` | union of the nine | `apply` (`STG:1573`) dispatches with an if-chain on `kind` |

### 1.5 Runtime state (`SCH:227-280`)

| Type | Fields |
|---|---|
| `Gaze` | `x, y, vx, vy` (pupil spring state, units of the unit disc) |
| `Needs` | `energy, sociability, curiosity` in [0,1] |
| `Actor` | `species, perch, x, y, vx, vy, facing, faced, activity, since, until, goal, partner, clip, gaze, blink, mood, needs, opacity, leaving, draws` |
| `Rapport` | `between[2], drift` |
| `Stage` | `seed, tick, mode, quiet, width, height, pointer, pointed, glances, surfaces, keepouts, perches, wanted, actors, rapports, met, draws` |

### 1.6 Frame (`SCH:283-302`)

| Type | Fields |
|---|---|
| `EyeFrame` | `x, y` (pupil offset in px from the white's centre), `lid` (0 open .. 1 shut) |
| `ActorFrame` | `species, x, y, facing, activity, opacity, bones[] (6 per bone), eyes[], mood` |
| `Frame` | `tick, actors[], rate (0\|16\|32\|64), wake (Ticks\|null)` |

### 1.7 What the fields mean in practice

**`Actor`** (all numbers are in "species units" = px at scale 1; the layer divides the page by its draw scale, `LAY:90-100`):

| Field | In practice |
|---|---|
| `species` | the identity (one actor per species); also keys `partner`, the random stream (index in `menagerie.species`) and the frame order tie-break |
| `perch` | surface id of the perch it stands on; `null` while hopping, falling, gliding, crowded out or orphaned |
| `x`, `y` | feet. `y = perch.y - hover` when grounded. The body box is `[x - w/2, x + w/2] x [y - h, y]` (the hover is added to nothing else) |
| `vx`, `vy` | px/s. Used by hops (set at launch, `vx` constant, `vy` integrated by `hopStep`), falls (`vy` only; `plunge` never integrates `vx`), float glides (both constant). 0 otherwise |
| `facing` | +-1, the way it faces **at once**; `faced`: the tick from which the drawing has finished turning (it sweeps over the 8 ticks before) |
| `activity`, `since`, `until` | what it does, the tick it began (clip phase `tick - since`, needs settlement, blend-in), the tick it ends (idle dwell; walk = 1920-tick patience; hop = landing tick; fall = 640 ticks; waiting for a partner = `now + 1920`) |
| `goal` | x target: walk goal, hop target x (launch.x), otherwise `= x`; moved along with a riding surface |
| `partner` | slug of the encounter partner or `null` (symmetric links; `release` cuts both) |
| `clip` | id of the clip played on top of the idle loop for this activity (or the idle clip), `null` when none/still |
| `gaze` | pupil spring; goal from `gazeGoal` |
| `blink` | tick at which the next (or current) blink begins; lid = `lidAt(tick - blink)` |
| `mood` | scalar in [-1,1], eased 1/32 per tick to `moodOf(activity)`; output only |
| `needs` | energy/sociability/curiosity, **settled lazily** over the span of an activity when it ends (`shift`) |
| `opacity` | 0 at arrival, +1/16 per tick towards `presence` (1, or 0.35 under the pointer); leavers -1/16 per tick, removed at <= 0 |
| `leaving` | summoned away / crowded out: walks to the perch end when close, then fades; never concludes activities |
| `draws` | counter of the actor stream `[seed, streamIndex, draws]`, u32, wrapping; starts at the **arrival tick**, not 0 |

**`Stage`**: `seed` u32; `tick`; `mode`, `quiet` (a run is in progress); `width/height` the stage box; `pointer` (last stage position or `null`), `pointed` (tick of the last pointer event, also used for "has it rested"); `glances` (peer cursors); `surfaces`/`keepouts` as surveyed; `perches` derived by `measure`; `wanted` slugs in wish order; `actors` in `menagerie.species` order (this order is normative: step order, draw order, frame tie-break); `rapports` (drift per unordered pair, lazily faded); `met` (tick of the last change of any rapport, 0 = never, anchors the encounter gap and the warm-up); `draws` stage-stream counter.

**`ActorFrame`**: `x, y` feet; `facing` mirror sign (the squeeze while turning is folded into the matrices, not into `facing`); `activity` (render hook `data-pet-activity`); `opacity`; `bones` = world matrix `a b c d e f` per bone relative to the feet origin (rotation/scale of the whole body is only possible through the root bone's clip tracks); `eyes` = pupil offsets in px; `mood`.

**`Frame`**: `rate` = ticks per second the motion needs (64 fading/walking/flying/landing/turning/blending, 32 loops/blinks/pupils/mood, 16 only sleeping, 0 nothing); `wake` = tick of the next scheduled change, only at rate 0.

---

## 2. Stage anatomy (`STG`)

### 2.1 File map (region markers, `STG`)

Header/normative order 1-51 | imports 53-59 | Constants 61-107 | Draft 109-183 | Lookups 185-313 | Gaze 315-371 | Activities 373-528 | Rapport 530-581 | Decision 583-749 | Motion 751-950 | Encounters 952-1072 | Time 1074-1139 | Events 1141-1594 | Frame 1596-1728. The Rust twin has the same regions in the same order (`STG.rs` 67-2211).

### 2.2 Every top-level function in file order

Callers come from a scripted call graph over the file (appendix A repeats it compactly). "-" = exported / entry.

| Function | Lines | What it does | Called by |
|---|---|---|---|
| (types) `Body`, `Draft`, `Room`, `Launch` | 110-116 | mutable actor copy; working copy of a stage plus parallel `kinds[]`/`streams[]`; a perch with free stretches; a hop launch | |
| `openStage(seed)` | 119-121 | empty stage: tick 0, `calm`, not quiet, seed u32 | - (export) |
| `draftOf` | 124-159 | copy actors (shallow), sorted into `menagerie.species` order, **first actor per species only**, unknown species dropped; keeps `kinds`, `streams` | `advance` |
| `sealed` | 162-182 | draft back to a `Stage` | `advance` |
| `hoverOf` | 187-189 | hover of a float gait, else 0 | touch, hopsOf, plunge, fly, measure, carry, arrive |
| `indexOf` | 192-195 | index of the actor of a species, -1 | headingOf, gazeGoal, release, reconcile, meet, arrivalTick, measure, spawn |
| `surfaceOf` | 198-201 | surface by id | carry, widened, ride |
| `clipsOf` | 204-211 | repertoire clips for an activity; walk of a hop gait falls back to hop clips; anything else to idle clips; else none | clipAt |
| `clipAt` | 214-219 | uniform pick among those by a unit | settle, crowdOut, drop, touch, attend, sulk, decide, stride, fly, hail, reachable, approach, meet, arrive, leave, tune |
| `clipOf` | 222-226 | clip by id | breathOf, touch, paced, decide, stride, poseOf, paceOf |
| `breathOf` | 229-232 | the first idle clip (the "breathing" loop that runs under everything) | poseOf, paceOf |
| `facingTo` | 235-237 | `towards >= x ? 1 : -1` | headingOf, hail, approach |
| `blinkAt` | 240-242 | `now + 128 + floor(256 * unit)` | wink, arrive, tune |
| `actorKey` | 245-250 | next actor draw key `[seed, stream, draws++]` | settle, sulk, decide, wink, hail, tune |
| `stageKey` | 253-257 | next stage draw key `[seed, STAGE_STREAM, draws++]` | pair, meet, arrive |
| `watched` | 260-267 | pointer within 1.5 x height of the body middle | decide, perk, act, lull |
| `presenceOf` | 270-276 | 0.35 while the pointer is inside the box grown by 4 px, else 1 | perk, act, lull, paceOf, frameOf |
| `headingOf` | 279-294 | the way an actor ought to face at a tick: toward goal (walk/hop), partner (toward; away when sulking), or - standing idle, alone, grounded, not leaving, not quiet, pointer < 256 ticks old, 56 ticks since last turn - toward a pointer more than `width/2 + 12` px aside; else 0 | swivel, lull |
| `turn` | 297-302 | face the new way at once, `faced = now + 8 - left` (a turn back takes as long as the turn has run) | swivel, hail |
| `shoulders` | 305-307 | `(w1 + w2) / 2`: feet distance at which bodies touch | vacancy, hopsOf, clearway, hindered, seat, roomsFor |
| `beatOf` | 310-312 | ticks of one hop clip for a hop gait, else 1 | paced, stride |
| `gazeGoal` | 322-350 | where pupils want to be: sleep centre; sulk (0.3 x facing, 0.5 down); pointer if not walking/hopping/falling and < 256 ticks old; partner; nearest glance; fall = down 0.8; else ahead 0.3 | gazeRests, look |
| `gazeRests` | 353-358 | pupils on the goal without velocity | lull, paceOf |
| `look` | 361-370 | one `springStep` per axis; snaps and rests within 1/4096 / 1/64 | act |
| `shift` | 375-380 | settle needs over the span of the old activity, set new activity and `since` | settle, crowdOut, drop, touch, stroll, attend, sulk, decide, stride, fly, hail, meet, leave, freeze |
| `settle` | 383-393 | come to rest: idle, new dwell + idle clip (2 words), no partner, goal = x, v = 0 | release, conclude, stride, act, summon, tune |
| `release` | 396-404 | cut both partner links, settle the other unless sulking/leaving | vanish, crowdOut, drop, conclude, leave, tune |
| `remove` | 407-411 | splice out of draft | vanish, act, seat, spread, summon, freeze |
| `vanish` | 414-418 | release + remove, always returns false ("arrives anew") | plunge, carry |
| `crowdOut` | 421-433 | no room: release, `leaving = true`, `perch = null`, fade where it stands | touch, seat |
| `vacancy` | 436-457 | nearest place on a perch >= `shoulders + 8` from everybody else on that surface, or null | touch |
| `drop` | 460-469 | lose perch: release, `fall` (vy 0, until +640) | carry |
| `touch` | 472-490 | land on a perch: `vacancy` near `x` (within one body width) else crowded out; `land` for its non-looping clip or 19 ticks | plunge, fly |
| `paced` | 493-500 | hop gait: floor the way to whole hops (`speed * beat / 64`); other gaits unchanged | decide, reachable, leave |
| `stroll` | 503-509 | start `walk`: goal, until +1920, clip | decide, approach, leave |
| `attend` | 512-518 | wait for the partner: idle, until +1920 | stride, approach |
| `sulk` | 521-527 | `sulk` for a drawn 3..6 s with a drawn clip (2 words) | conclude |
| `recall` | 532-543 | fade every rapport over the ticks since `met`, drop zeros, `met = now` | reconcile, meet |
| `bond` | 546-567 | step a pair's drift by the encounter (`rapportAfter`), add/forget | reconcile, meet |
| `reconcile` | 570-580 | end of a sulk: when the partner no longer sulks, mend (+0.1); always cut the own link | conclude, poke |
| `soars` | 585-599 | the hop arc touches no keep-out above both ends and stays below the stage top | hopsOf |
| `hopsOf` | 602-639 | per other perch the place nearest to the actor, >= half a body from the ends, free of actors at launch time; `hopOf` + `hopLanding` + `soars` for walkers/hoppers; **float gaits: straight glide at 2x speed when within `HOP_DISTANCE`, `HOP_HEIGHT` and 256 ticks, with no landing or keep-out check** | decide |
| `clearway` | 642-664 | the stretch of the own perch walkable without coming closer than `gap` to anybody else (a walker counts for its whole way) | decide, reachable, leave |
| `decide` | 667-730 | an idle actor with an expired dwell picks (see 2.5) | conclude |
| `conclude` | 733-748 | expiry of `until`: idle -> `decide` (or give up waiting), squabble -> `sulk`, sulk -> `reconcile`, walk/hop/fall do nothing (they end by motion), all else `settle` | act |
| `hindered` | 753-765 | somebody ahead on the same perch would be closer than `shoulders + 6` after the next beat | stride |
| `stride` | 768-795 | one tick of a walk (see 2.6) | act |
| `aim` | 798-810 | perch that carries `x`, nearest in y | fly |
| `plunge` | 813-827 | one tick of a fall: `fallStep`, swept `landingOf`, below the stage box -> `vanish` | fly, act |
| `fly` | 830-862 | one tick of a hop/glide; last tick touches down on the aimed perch or falls | act |
| `wink` | 865-870 | after a blink's 12 ticks draw the next (2..6 s, or 1/6 a double 7 ticks later); sleepers draw nothing | act |
| `cheer` | 873-878 | mood 1/32 of the way to `moodOf(activity)`, snap within 1/1024 | act |
| `swivel` | 881-892 | turning: `true` while turning (a walker must not stride); starts a turn toward `headingOf` | act |
| `hail` | 895-904 | greet toward `x`: shift, `turn`, dwell + clip (2 words), goal = x, no partner | perk, poke |
| `perk` | 907-915 | idle actor greets a pointer that has rested 32 ticks within 1.5 heights but not on it, if curiosity >= 0.5; costs 0.6 curiosity | act |
| `act` | 918-949 | one tick of one actor (2.4) | step |
| `sociable` | 954-956 | idle, no partner, not leaving, opacity exactly 1, has a perch | pairingTick, pair |
| `pairingTick` | 959-971 | first whole second a pair may be drawn, or -1 (no encounters in mode, quiet, a partnership exists, < 2 sociable, warm-up 1280 / gap not over) | pair, lull, frameOf |
| `pair` | 974-1008 | on a whole second: candidate pairs within 12 mean widths / 3 high, nobody between; one stage draw weighted `(0.25 + |authored affinity|) * mean sociability`; chance `rate * social`; `approach` | step |
| `parted` | 1011-1022 | somebody stands between two actors of one perch | pair |
| `reachable` | 1025-1030 | where an actor may walk to meet its partner | approach |
| `approach` | 1033-1048 | set partners; walk to `apart = widths/2 + 6`: both walk (room for 2 movers) or only the more eager; others `attend` | pair |
| `meet` | 1051-1071 | both partners idle: `recall`, one stage draw (4 words: kind, span, clip, clip), `shift` both, same `until`, `bond` | step |
| `arrivalTick` | 1076-1082 | next whole second a wanted-but-absent species may arrive, or -1 (still, no perches, nobody waits) | lull, step, pass, frameOf |
| `lull` | 1085-1109 | how many upcoming ticks change nothing but the tick (0 while anything moves, fades, turns, springs, eases) | pass |
| `step` | 1112-1119 | one stage tick (2.4) | pass |
| `pass` | 1122-1138 | tick by tick, jumping over lulls, still stages and empty stages | apply |
| `measure` | 1143-1152 | recut perches: `clearance` = tallest of wanted/on-stage + hover, `minimum` = widest | survey, summon |
| `carry` | 1155-1195 | ride one grounded actor with its surface (x0 delta, new y), hold it inside the nearest perch of that surface (fade anew if pulled farther than its width); no perch on the surface: other surface exactly under the feet, else `vanish` (uprooted/still) or `drop` | ride |
| `seat` | 1198-1250 | after a ride: per perch the members must fit with 8 px gaps, crowd out the most strained until they do, then two passes setting them apart | ride |
| `widened` | 1253-1256 | a surface id not in `before` (new ground) | survey |
| `ride` | 1259-1277 | carry everyone, then seat | survey, summon |
| `carve` | 1280-1292 | stretches without an open interval | vacancy, roomsFor |
| `crowdOn` | 1295-1299 | actors on a perch | roomsFor, spread |
| `roomsFor` | 1302-1319 | per perch the stretches a species could stand on, `shoulders + 8` from everyone there, and the crowd | decide, arrive |
| `quarters` | 1322-1335 | rooms with the fewest actors; the ground (lowest perches of the stage) only when no raised one is as empty | arrive |
| `arrive` | 1338-1383 | one stage draw (perch, place, facing), actor draw (dwell, clip, blink); opacity 0 (1 when still); inserted in stream order | spawn |
| `spawn` | 1386-1392 | every wanted species that is absent arrives while `actors < wanted` | step, survey, summon, tune |
| `spread` | 1395-1422 | over new ground, idle partnerless actors beyond the first on a perch leave for perches that hold nobody (not quiet) | survey |
| `survey` | 1425-1438 | store; `widened`; `measure`; `ride`; `spawn`; when uprooted `spread` + `spawn` | apply |
| `leave` | 1441-1462 | start leaving: release; walk to the nearer perch end if within 3 widths and clear, else fade where it stands | decide, spread, summon |
| `summon` | 1465-1487 | new wanted list; unwanted leave (gone at once when still), wanted leavers stay; measure, ride, spawn | apply |
| `freeze` | 1490-1510 | still: remove leavers/airborne, everybody idle at rest, opacity 1, centred gaze | tune |
| `tune` | 1513-1544 | still -> freeze; from still -> fresh dwell/clip/blink (3 words); fewer movers -> excess walkers settle | apply |
| `poke` | 1547-1570 | still/quiet: ignored; nearest grounded non-leaving actor within 1.2 x height of its middle: mood +0.3, if free (or sulking: reconcile first) `hail` | apply |
| `apply` | 1573-1585 | if-chain over `event.kind` | advance |
| `advance` | 1588-1593 | no events -> same stage object; else draft, apply each, seal | - (export) |
| `replaces` | 1598-1600 | walk, hop, fall, land, sleep cross-fade the idle loop out; others lie on top | poseOf |
| `weightOf` | 1603-1608 | blend weight of the activity clip: in over 8 ticks, out over the last 8 (of the span, or of the way to the goal when walking; hop/fall: no out) | poseOf |
| `layer` | 1611-1616 | offsets add, scales multiply | poseOf |
| `poseOf` | 1619-1629 | pose of one actor (2.10) | frameOf |
| `leant` | 1632-1636 | the first eye's bone leans: +5 deg and +1.5 px per unit of gaze across, +1 px per unit down | frameOf |
| `squeezeOf` | 1639-1641 | x-scale while turning: from -1 through 0 to 1 over 8 ticks | frameOf |
| `paceOf` | 1644-1658 | the rate one actor needs | frameOf |
| `frameOf` | 1669-1727 | the projection (2.10) | - (export) |

### 2.3 `advance`: control flow per event

`advance` (`STG:1588`): no events -> returns the same stage object (test `STU:1682` asserts `toBe`); otherwise `draftOf`, `apply` per event in order, `sealed`. The input is never mutated (test `STU:1673` deep-freezes it), so **new nested state must be replaced, not mutated in place**.

| Event | Handler | Chain |
|---|---|---|
| `ticked` | `pass(ticks)` | loop: still or (no actors and nobody waits) -> `tick += left`, done; else `skip = lull()`; `skip > 0` -> `tick += skip`; else `step()`; `left -= 1` |
| `pointed` | inline (1575-1577) | `pointer = {x,y}`, `pointed = tick` |
| `unpointed` | inline (1578) | `pointer = null` (`pointed` unchanged) |
| `glanced` | inline (1579) | `glances = points` |
| `surveyed` | `survey` | store box/surfaces/keepouts -> `widened(before)` = `uprooted` -> `measure` -> `ride(before, uprooted)` (per actor `carry`, then `seat`) -> `spawn`; if uprooted: `spread` -> `spawn` |
| `summoned` | `summon` | store `wanted`; for each actor: wanted leaver stays (`settle`), unwanted -> `leave` (or `remove` when still); `measure`; `ride(surfaces, false)`; `spawn` |
| `tuned` | `tune` | same mode: nothing; to still: `freeze` + `spawn`; from still: fresh dwell/clip/blink for everyone; otherwise excess walkers settle |
| `hushed` | inline (1583) | `quiet = event.quiet` only; its effects are read where they matter (list in 2.9/section 7) |
| `poked` | `poke` | see table |

### 2.4 One tick: order of phases (`step`, `STG:1112-1119`; header `STG:3-12`)

`tick += 1`, then for each actor in `menagerie.species` order (`act`, `STG:918-949`; the loop does not advance its index when `act` removed the actor, `STG:1115`):

1. **Fade/presence**: leaving and not walking: `opacity -= 1/16`, removed at <= 0; otherwise `opacity` moves towards `presenceOf` (up +1/16, down -1/32).
2. **Turning**: `swivel` (true while the drawing turns; on the tick the turn ends a walker's `since = now`).
3. **Motion by activity**: `walk` -> `stride` unless turning; `hop` -> `fly`; `fall` -> `plunge`; `sleep` -> `settle` when `watched`; `idle` -> `perk`; all others nothing.
4. `look` (gaze spring), 5. `wink`, 6. `cheer`.
7. **End of activity**: `!leaving && now >= until` -> `conclude`.

Then the stage: `meet` (pairs whose partners both wait), `pair` (a new pair on whole seconds), and `spawn` when `arrivalTick == now`.

Actors are processed sequentially and see the already updated state of earlier actors and the old state of later ones in the same tick (no double buffering). Random draws are keyed by actor, so the order does not change what an actor draws.

### 2.5 How activities start and end

* **Start by choice** - only `idle` decides (`conclude` -> `decide`, `STG:733-748`, `667-730`): `activityWeights` (section 3.1) over `Situation { mode, quiet, movers, fidgeters, roam, hops, crowd, watched }`; one `actorKey` draw: word 0 = pick (`randomPick`), words 1/2/3 = dwell unit, clip unit, place unit (`randomWords(key, 4)` re-derived from the same key). `hopsOf` is evaluated only when `!quiet && movers < limits.movers && limits.hop > 0 && on a perch`. `restless && no launch && another perch has room` -> `hops = true` and the "hop" turns into `leave` (wander off and arrive anew).
  * `walk`: goal = `low + (high - low) * place` clamped to +-`stroll * width`; at least `0.75 * width` away (else pushed out); snapped by `paced`; no valid goal -> stays idle.
  * `hop`: `launches[floor(place * n)]`; `perch = null`, `vx, vy`, `goal = launch.x`, `until = now + launch.ticks`.
  * `fidget`/`sleep`/`idle`: `body.activity = activity` set directly after `shift(idle)` (no second `shift`), clip drawn, `until = now + clipTicks` for a non-looping fidget, else `dwellOf`.
* **Start by event**: `poke`/`perk` -> `greet` (`hail`); `carry` -> `drop` -> `fall`; `fly` end with no perch -> `fall`; `plunge`/`fly` landing -> `touch` -> `land`; `meet` -> `greet|cuddle|squabble`; `conclude` -> `sulk` after `squabble`; `approach`/`decide`/`leave` -> `stroll` -> `walk`; `settle`/`attend` -> `idle`; `freeze` -> `idle`.
* **End**: dwell expiry via `conclude` (idle, fidget, land, sleep, greet, cuddle, squabble, sulk); `walk` at goal / hindered / patience (`stride`); `hop` on landing (`fly`); `fall` on landing (`plunge`); sleep also on `watched` (pointer within 1.5 heights); a leaver never concludes.
* **Clip choice**: `clipAt(kind, activity, unit)`, uniform among `clipsOf` (see table); fidget: the whole clip length when non-looping, else 96 ticks.

### 2.6 Locomotion in detail

* **Walking** (`stride`): `stroll` set goal/until/clip. `swivel` first turns the actor toward the goal (8 ticks, walk clip weight 0 while `tick < faced`, `STG:1625`). On every *beat* (every tick; a hop gait: the start of each hop, `(now - since) % beat == 0`) the walk ends if `|goal - x| <= speed/64` (snap to goal) or `hindered(beat)`; otherwise `strideTo`. A hop gait hindered in mid-hop (`hindered(1)`) sets `goal = x` and finishes the hop. At the end: leaver -> idle+fade; partner -> `attend`; else `settle`.
* **Hop gaits** walk in whole hops: `paced` floors the goal to `floor(way / (speed * beat / 64))` hops; `beat = clipTicks(hop clip)`.
* **Hopping between perches**: `hopsOf` per other perch: target x clamped to half a body inside the perch, no actor within `shoulders + 8` at launch, `hopOf(from, to)` (ballistic, apex above both ends, <= 84 high/160 far/48 ticks, no faster than 900 px/s landing), `hopLanding` must equal the target perch, `soars` clear of keep-outs. `fly`: `hopStep` + swept `landingOf` each tick (first perch crossed while descending -> `touch`); last tick: `aim` at `goal` else `fall`.
* **Floating**: `hopsOf` float branch: straight line at 2 x speed within `HOP_DISTANCE` x `HOP_HEIGHT` and <= 256 ticks (`GLIDE_TICKS`); `fly` moves `x += vx/64`, `y += vy/64` with no landing or keep-out test; last tick `aim`. Walking floaters glide at hover height on the perch with the same `strideTo`.
* **Falling**: `drop` (perch vanished) sets `vy = 0`; `plunge`: `fallStep` (GRAVITY 1800, FALL_SPEED 900), `landingOf(x, y + hover, y' + hover)` highest perch crossed at the current `x` (x never changes), `touch`; `y - height > stage.height` -> `vanish`.
* **Landing** (`touch`): `vacancy` within one body width else `crowdOut`; `land` lasts the non-looping clip or 19 ticks; `vx = vy = 0`.
* **Turning**: `headingOf` -> `turn` (sets `facing` now, `faced = now + 8 - left`) -> `squeezeOf` in `frameOf`. Pointer-driven turns need >12 px beyond the body and 56 ticks since the last turn.
* **Spreading/quarters/uprooted**: `quarters` picks among the emptiest perches (ground last); `widened` marks a survey that brought a new surface id; `carry` then makes ground-less actors `vanish` (arrive anew) instead of `drop`; `spread` sends idle co-habitants to empty perches.
* **Quiet/still** gates: see section 7 for the full list.
* **Leaving**: `summon` -> `leave` (walk to the nearer end within `3 * width` if the way is clear, else fade in place); `act` fades and removes. **Appearing**: `arrive` at opacity 0 (1 when still).

### 2.7 Encounters: pairing, staging, ending

* Gate (`pairingTick`, `STG:959-971`): mode has `encounterGap > 0`, not quiet, nobody has a partner (so **at most one encounter at a time**), >= 2 `sociable` actors, and `tick >= (met == 0 ? 1280 : met + gap)`; evaluated only on whole seconds.
* `pair` (974-1008): mover budget (`movers < limits.movers`), candidates within `12 * meanWidth` x `3 * meanWidth` with nobody between, one `stageKey`: word 0 picks the pair, word 1 is the chance against `rate * meanSociability`. No candidates -> no draw. `approach`: walkers go to `apart = widths/2 + 6` (`MEET_GAP`) clamped to the clear way; others `attend` (until `now + 1920`).
* `meet` (1051-1071) when both partners idle: `recall`; one `stageKey` of 4 words [kind via `encounterOf(affinityOf(...))`, span, clip1, clip2]; both `shift` to the kind with one shared `until`; `bond` moves the rapport at once.
* End: greet/cuddle -> `settle`; squabble -> `sulk` for each (own span 3..6 s, backs turned); a sulk's end -> `reconcile` (the second to end mends the rapport +0.1); a patient waiter gives up after 1920 ticks; a partner that falls/leaves/freezes releases the other.

### 2.8 Spacing between actors: what exists, what does not

Margins (`STG:62-107`): `COMFORT_GAP = 8` (arrival, landing, goal choice, seat capacity, hop target check), `MEET_GAP = 6` (walker stops, encounter approach, leave clearway), `CONTACT = 1/128` (slack of the law/`hindered` equality), `shoulders = (w1 + w2)/2`.

| Mechanism | Function (lines) | Covers |
|---|---|---|
| arrival | `roomsFor` 1302, `quarters` 1322, `arrive` 1338 | grounded actors of the same perch; arrival waits off stage when no stretch is free |
| goals | `clearway` 642 (walkers count for their whole way) | same perch |
| walking | `hindered` 753 (+6 after the next beat), `stride` 768 | same perch, ahead only |
| landing | `vacancy` 436, `touch` 472 | places the lander at the nearest free place within one body width, else crowds it out *after* the flight |
| ride | `carry` 1155, `seat` 1198 | squeezes grounded actors apart after a survey; crowds out the most strained |
| hop target | `hopsOf` 616-620 | actors on the target perch **as they stand at launch** |
| encounter | `approach`/`reachable` | stop 6 px apart; partners on different perches "act across the gap" |

**Not covered**: (a) airborne actors in any pair (hop flight <= 48 ticks, fall, float glide <= 256 ticks): nothing tests overlap during flight; (b) actors on *different* perches regardless of y distance (the laws and `overlapping` compare `perch` strings); (c) y overlap of any kind (everything is x-distance on a shared surface); (d) hop landing slot reserved only at launch, not kept: a third actor can walk onto the target while the hopper is in flight, repaired afterwards by `vacancy` (teleport up to one body width) or `crowdOut`; (e) float glides cross keep-outs and other actors; (f) encounter poses and fidgets reach a few px beyond `Species.size` (known art limit; `CONTACT` and `MEET_GAP` absorb it only for the rest box); (g) newcomers fading in at opacity 0 are normal actors for spacing (good), but a *leaver* that was crowded out (`perch = null`) is invisible to spacing; (h) bounds of the *drawn* body: the core has no path parser, so only `Species.size` can define "overlap".

### 2.9 The pointer: every use

| Use | Where | Rule |
|---|---|---|
| gaze | `gazeGoal` 329 | pointer moved < 256 ticks ago and actor not walking/hopping/falling; `lookOffset(eye, pointer, 32)`; eye = 0.6 x height above the feet |
| turn | `headingOf` 290-293 | idle, alone, grounded, not leaving, not quiet, pointer < 256 ticks old, >= 56 ticks since `faced`; pointer > `width/2 + 12` px aside |
| lean | `frameOf` 1698 + `leant` | not still, not quiet |
| hail/perk-up | `perk` 907-915, called from `act` 943 | not quiet, `now == pointed + 32` exactly, idle, no partner, grounded, `watched` (within 1.5 heights), `presence == 1` (not on it), curiosity >= 0.5 -> `hail`, costs 0.6 curiosity |
| poke | `poke` 1547-1570 | `poked` event (one `pointerdown` over an actor box that is not a control): not still, not quiet; nearest grounded non-leaving actor within 1.2 x height of the body middle; mood +0.3; free or sulking -> `greet` (sulker reconciled first) |
| wake | `act` 941 | sleeping actor settles when `watched` |
| sleep weight | `activityWeights` | `watched` removes the sleep weight |
| see-through | `presenceOf` 270, `act` 929-932, `frameOf` 1701 | opacity 0.35 while the pointer is in the box +4 px; eased down 1/32, up 1/16 per tick; instant on a still stage |
| lull/wake | `lull` 1099-1102, `frameOf` 1717-1720 | horizons: turn-rest end, perk tick, pointer interest end (`pointed + 256`) |

The layer adds: pointer only matters to a *still* stage while it is on or beside a pet (`LAY:296-307`); touch has no hover (`SUR:313-318`: a lifted finger sends `unpointed`).

### 2.10 `frameOf`: composing the frame (`STG:1669-1727`)

* Actors are re-sorted into species order, then rendered **back to front by `y`, then species id** (`:1685`).
* **Pose** (`poseOf`, 1619-1629): `under` = the first idle clip sampled at `tick + stream * 37` (BREATH_STAGGER, loops forever, so nothing is frozen); `over` = `blendPose(rest, sampleClip(top clip, tick - since), weightOf)`; if the activity `replaces` the idle loop (walk, hop, fall, land, sleep) `under` is first blended towards rest by the same weight; `layer(under, over)` adds offsets and multiplies scales. A walk that still turns weighs 0. Still stage: rest pose.
* **Lean** (`leant`): outside still and quiet, applied to the bone of the first eye.
* `solveRig` -> 6 numbers per bone; **squeeze**: while turning, entries 0, 2, 4 (`a`, `c`, `e`) of every matrix are multiplied by `squeezeOf` (2 * smoothstep(...) - 1).
* **Eyes**: pupil = gaze x `pupilReach(eye)` (x mirrored with the drawn side); lid = `lidAt(tick - blink)`, 1 asleep, 0 still.
* **opacity**: `actor.opacity`, on a still stage `min(opacity, presence)`.
* **rate**: max of `paceOf` over actors (0 when still). `paceOf` (1644-1658): 64 if leaving, opacity != presence, turning, walk/hop/fall/land, or a layered clip that is non-looping and young, or blending in/out (first/last 8 ticks); else 32 if mood != target, pupils not at rest, blinking, or loops; sleepers: 32 if restless, 16 if they have anything to breathe, else 0.
* **wake** (only at rate 0, not still, and actors or arrivals exist): min of each actor's `until`, blink (start or end), turn-rest end (pointer present), perk tick, pointer-interest end, `pairingTick`, `arrivalTick`; at least `tick + 1`.

### 2.11 Randomness keys

One key per draw: `[seed, stream, counter]` hashed by `randomWords` (numpy `SeedSequence`, pool of 4). Actor stream = species index; counter = `actor.draws` (u32, wraps; starts at arrival tick). Stage stream `STAGE_STREAM = 0xffffffff`, counter `stage.draws`. Reserved: `CAST_STREAM = 0xfffffffe` (`castOf`), `ROTATION_STREAM = 0xfffffffd` (the layer's rotation clock). Word layout (header `STG:29-35`): actor arrival/thaw `[idle dwell, idle clip, first blink]` (arrival uses `randomWords([seed, stream, tick], 3)` directly and sets `draws = tick + 1`); rest `[dwell, clip]`; decision `[pick, dwell, clip, goal-or-target]`; blink `[gap, double]`; hail `[dwell, clip]`; sulk `[dwell, clip]`; stage arrival `[perch, place, facing]`; pairing: word 0 pick, word 1 chance; encounter `[kind, span, clip1, clip2]`. A new draw on an existing code path shifts every later draw of that stream and so every committed trace; a new feature should draw from a **new stream** (reserve it in `RND`) or only on newly reachable paths.

### 2.12 Constants (`STG:62-103`, in file order)

| Constant | Value | Meaning |
|---|---|---|
| FADE_STEP | 0.0625 | opacity per tick in/out (16 ticks) |
| POINTER_TICKS | 256 | a pointer is worth a look for 4 s |
| GAZE_REACH | 32 | px at which the pupil is half way out (`lookOffset` reach) |
| GAZE_REST / GAZE_CALM | 1/4096 / 1/64 | snap distance / snap speed of pupils |
| GAZE_AHEAD | 0.3 | pupil x towards facing when nothing to look at |
| GAZE_SULK / GAZE_FALL | 0.5 / 0.8 | pupil down while sulking / falling |
| EYE_HEIGHT | 0.6 | eye height as a fraction of species height |
| BLINK_LOW / BLINK_HIGH / BLINK_AGAIN | 128 / 384 / 7 | next blink 2..6 s ahead; double blink 7 ticks after (1 in 6) |
| MOOD_EASE / MOOD_REST | 1/32 / 1/1024 | mood easing and snap |
| POKE_REACH / POKE_CHEER | 1.2 x height / 0.3 | poke radius from the body middle / mood gain |
| WAKE_REACH | 1.5 x height | a pointer this close wakes a sleeper |
| BLEND_TICKS | 8 | clip blend in/out |
| BREATH_STAGGER | 37 | idle loop phase offset per stream |
| PATIENCE | 1920 | a waiting partner gives up after 30 s |
| WARMUP | 1280 | no encounter in the first 20 s |
| ENCOUNTER_REACH / ENCOUNTER_RISE | 12 / 3 | mean widths horizontally / vertically for pairing |
| PAIR_WEIGHT | 0.25 | base weight of any pair |
| MEET_GAP / COMFORT_GAP | 6 / 8 | px gaps (above) |
| LEAVE_REACH | 3 | widths a leaver will walk to the perch end |
| STROLL_LEAST | 0.75 | least walk in body widths |
| GLIDE_TICKS | 256 | longest float glide |
| SHY_OPACITY / SHY_STEP / SHY_REACH | 0.35 / 1/32 / 4 | see-through level / fade-down step / margin around the box |
| TURN_TICKS / TURN_REST / TURN_CLEAR | 8 / 56 / 12 | turn duration / rest between pointer turns / dead zone px |
| LEAN_TURN / LEAN_REACH / LEAN_NOD | 5 / 1.5 / 1 | degrees, px across, px down per unit gaze |
| PERK_LINGER / PERK_URGE / PERK_COST | 32 / 0.5 / 0.6 | rest ticks / min curiosity / curiosity spent |
| CONTACT | 1/128 | equality slack |
| (literals) | | arrival opacity 0; glide speed factor 2; `GAZE`/`BLINK_TICKS` in `ANI`; double-blink chance `unit*6 < 1`; facing at arrival `unit < 0.5 -> 1` |

---

## 3. Behaviour, terrain and the small modules

### 3.1 `BEH` (292 lines)

* `type Limits` (24-36) and `MODE_LIMITS` (39-43):

| | movers | fidgeters | idle dwell (ticks) | fidget | walk | hop | sleep | stroll (widths) | encounterGap | encounterRate |
|---|---|---|---|---|---|---|---|---|---|---|
| still | 0 | 0 | 0..0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| calm | 1 | 1 | 384..1280 | 0.4 | 0.2 | 0.12 | 6 | 4 | 5760 (90 s) | 0.04 |
| lively | 2 | 2 | 192..640 | 1 | 0.6 | 0.3 | 3 | 6 | 1920 (30 s) | 0.12 |

* `type Situation` (48-57): `mode, quiet, movers, fidgeters, roam, hops, crowd, watched`.
* `activityWeights(actor: Actor, species: Species, situation: Situation): number[]` (73-88): ACTIVITIES order, `[1, fidget, walk, hop, 0, 0, sleep, 0, 0, 0, 0]` (a literal 11-array); `drive = 0.5 + 0.5*energy`, `urge = 0.25 + curiosity`, `tired = clamp((0.6 - energy)/0.6, 0, 1)`; `fidget = limits.fidget*drive*(0.5+curiosity)` if awake, `fidgeters < limit`, species has a fidget clip; `walk = limits.walk*drive*urge` if awake, `movers < limit`, `roam`; `hop = limits.hop*energy*urge*(1+crowd)` if awake, free, `hops`; `sleep = limits.sleep*tired^2*(quiet ? 3 : 1)`, 0 when `watched`.
* `dwellOf(activity, mode, unit): Ticks` (99-104): `low + floor((high-low)*unit)`; tables `DWELL_LOW = [0,96,1920,96,640,19,1280,128,128,128,192]`, `DWELL_HIGH = [0,96,1920,96,640,19,3840,320,320,320,384]` (idle from the mode).
* `moodOf(activity)` (109): `MOODS = [0.3, 0.5, 0.4, 0.5, -0.2, 0.2, 0.1, 0.7, 1, -0.8, -0.6]`.
* `followersOf(activity)` (134-146): the activity graph (`AFTER_*` arrays 115-125): idle -> idle, fidget, walk, hop, fall, sleep, greet, cuddle, squabble; fidget -> idle, walk, fall, greet; walk -> idle, fall, greet, cuddle, squabble; hop -> idle, fall, land; fall -> idle, land; land/sleep/sulk -> idle, walk, fall, greet; greet/cuddle -> idle, walk, fall; squabble -> idle, walk, fall, sulk. Tests demand one strongly connected component, ACTIVITIES order, `idle` in every list.
* Encounters (151-176): `ENCOUNTERS = [greet, cuddle, squabble]`; `encounterShares(affinity)`: friends (>= 0.4) cuddle `0.5 + 0.4a`, never squabble; rivals (<= -0.3) squabble `0.55 - 0.5a` (max 0.95), never cuddle; between: cuddle `0.5a` for a > 0, squabble `0.08 - 0.5a`; `encounterOf(affinity, unit)` = `weightedIndex(shares, unit)`.
* Bonds (181-222): `AFFINITY_FLOOR = -0.6`, `RAPPORT_SPAN = 0.5`; `affinityOf(menagerie, rapports, a, b)` = clamp(authored + drift, -0.6, 1), 0 for unlisted/self; `rapportAfter(drift, activity)` with `RAPPORT_STEPS = [0,0,0,0,0,0,0,0.05,0.1,-0.15,0.1]` (greet, cuddle, squabble, sulk-mend) clamped to +-0.5; `rapportFaded(drift, ticks)`: toward 0 by `0.1 * ticks / 38400`.
* Needs (226-253): rates per second per activity - `ENERGY_RATES = [-0.001,-0.01,-0.012,-0.02,0,0,0.02,-0.006,-0.004,-0.012,-0.001]` (drains scaled by `1.5 - temperament.energy`), `SOCIABILITY_RATES = [0.002,0.002,0.002,0.002,0,0.002,0.002,-0.12,-0.12,-0.12,-0.02]` (gains scaled by `0.5 + temperament.sociability`), `CURIOSITY_RATES = [0.01,-0.08,-0.06,-0.1,0,0.01,0.01,0,0,0,0.01]`; `needsOf(temperament)` = energy `0.5+0.5*t`, others as authored; `needsAfter(needs, activity, ticks, temperament)` clamps to [0,1].
* `castOf(cast, capacity, epoch, seed)` (276-291): see design §13.3; the stage never calls it (the layer does, `LAY:163`).

### 3.2 `TER` (185 lines)

Constants (26-44): `GRAVITY 1800`, `FALL_SPEED 900`, `HOP_CLEARANCE 12`, `HOP_STEEPNESS 0.375`, `HOP_HEIGHT 84`, `HOP_DISTANCE 160`, `HOP_TICKS 48`. Types `Fall`, `Hop`, `Flight`.

* `perchesOf(surfaces, keepouts, width, height, clearance, minimum): Perch[]` (80-92): per surface with `clearance <= y <= height`, clip to `[0, width]`, subtract the x-extent of every keep-out that intersects the band `[y - clearance, y)` (area > 0), keep stretches >= `minimum`; order: surface order then x. The stage passes clearance = tallest (height + hover) of all wanted or present species and minimum = widest (`measure`, 1143).
* `perchAt` (95), `nearestPerch` (101) (unused by the stage), `strideTo(x, goal, speed)` (119: step `max(speed,0)/64`, never overshoots).
* `fallStep(y, vy)` (128): `vy' = min(vy + 1800/64, 900)`, `y' = y + vy'/64`. `landingOf(perches, x, fromY, toY)` (134): highest perch crossed at `x`, ends inclusive, never while rising (one-way platforms).
* `hopOf(from, to): Hop | null` (155-166): `rise = max(12 - min(dy,0), |dx|*0.375)`; refuses `rise > 84`, `|dx| > 160`, `ticks > 48`, landing faster than 900; `ticks = floor((sqrt(2 rise/g) + sqrt(2(rise+dy)/g))*64 + 0.5)`; `vy`, `vx` solved so the *per-tick sum* ends on `to`. `hopStep` (169): fall step for y, `x += vx/64`, last tick writes the target. `hopLanding` (175): runs the loop ahead of time and returns the perch really landed on.
* Used by the stage: `HOP_DISTANCE`, `HOP_HEIGHT`, `fallStep`, `hopLanding`, `hopOf`, `hopStep`, `landingOf`, `perchAt`, `perchesOf`, `strideTo`, types `Flight`, `Hop`. **Vertical edges, ropes, wall surfaces and horizontal flight with gravity do not exist in `TER`.**

### 3.3 `ANI`, `RIG`, `RND`, `TRI`, `VAL`

* `ANI`: `easeBezier` (30, 48 bisections), `sampleTrack` (62), `clipTicks` (77, `floor(seconds*64+0.5)` >= 1), `sampleClip(species, clip, ticks): Pose` (90; loops wrap, others hold), `blendPose(from, to, amount)` (109), `springStep(position, velocity, target, stiffness, damping)` (148, semi-implicit Euler dt 1/64), `GAZE_STIFFNESS 512`, `GAZE_DAMPING 32`, `BLINK_TICKS 12`, `lidAt(ticks)` (164).
* `RIG`: `Affine`, `compose`, `invert`, `transform`, `BonePose`, `Pose`, `restPose` (69), `solveRig(species, pose): number[]` (88; local = translate x rotate x scale, world = parent x local), `lookOffset(eye, target, reach)` (123), `pupilReach(eye)` (133; radius - pupil - 0.25, >= 0).
* `RND`: `randomWords(key, count)` (74), `unitOf` (89), `randomUnit` (94), `randomBetween` (99), `weightedIndex(weights, unit)` (104; -1 when none positive), `randomPick` (125), `STAGE_STREAM/CAST_STREAM/ROTATION_STREAM` (24/27/30).
* `TRI`: `sinTurns`, `cosTurns` (50/60, fdlibm kernels, exact odd/even), `clamp` (71), `lerp` (77), `smoothstep` (82). **No atan2, pow, exp, hypot**: an angle-based gesture or a pendulum must use cross/dot products, `sqrt`, `sinTurns`.
* `VAL`: `speciesIssues` (391), `menagerieIssues` (396), `ensembleIssues` (410), `assembleMenagerie` (423); 13 rule codes + 7 structural; no schema library. Closed objects: any new `Species` field must be added to `SPECIES_FIELDS` (27) and checked.

---

## 4. Extension seams and hazards

### 4.0 Cross-cutting hazards (apply to every capability)

| # | Hazard | Where |
|---|---|---|
| X1 | Closed `Activity` list; order-coupled tables in 3 languages + Python (see 4.a) | `SCH`, `SCH.json` (`Activity`, `Repertoire`), `SCH.rs` (`Activity` 96-130, `ACTIVITIES` array len 11, `Repertoire` struct 303), `BEH` tables, `GEN:34`, `P/🧪️tests/🧠️behavior-choice/🐍️.py` (ACTIVITIES, DWELL_*, MOODS, FOLLOWERS, `weights`), `🤝️bond-dynamics/🐍️.py`, `🧬️schema-conformance/🐍️.py`, `DEP:211` (`ACTIVITIES.indexOf`), `📖️stories/🟦️.tsx` |
| X2 | Identity = species; one actor per species | `STG:130-137`, `indexOf`, `partner`, `wanted: Slug[]` |
| X3 | `perch === null` overloaded | see 0.3 |
| X4 | Time horizons live in three functions | `lull` 1085, `paceOf` 1644, `frameOf` wake 1710 |
| X5 | Frame shape: no body rotation/tint/props/per-part opacity | `SCH:288-301`, `DEP:193-260`, `LAY:192-216` |
| X6 | Pointer model: one point, no press/hold/velocity/history | `SCH:200-222`, `SUR:296-332`, `LAY:296-307` |
| X7 | Laws restated in three adapters | `TRC/🟦️.ts:86-123`, `TRC/🦀️.rs`, `TRC/🐍️.py:60-140` + the feature text |
| X8 | Existing paths must not gain draws (trace stability) | 2.11 |
| X9 | `quiet`/`still` gating is scattered (about 20 sites) | section 7 list |
| X10 | Rust mirror: every TS edit is a second edit; `Rate`, `Facing` are enums on wire numbers (`SCH.rs:606`, `747`) | `STG.rs`, `SCH.rs` |
| X11 | Digest folds activity index, not new fields; blind spots exist (needs, gaze velocity) | `TRC/🟦️.ts:57-67` |
| X12 | No nested in-place mutation of inputs; deep-frozen input test | `STU:1673-1683` |
| X13 | Forbidden maths: `sin cos tan atan2 exp pow hypot log random Date performance console` (grep tests in every module suite, e.g. `STU:1778-1784`, `BEH` suite 589) | all modules |
| X14 | New module directories need taxonomy registration (`members-of-modules`), unique docstring emoji per file, no banned name stems | design §11-12 |

### 4.a Airborne/vertical activities: climbing a vertical edge, hanging on a rope and reeling in, parachute descent, a ladder that stays

**What must change**

* *Terrain input*: `Surveyed` knows only top edges (`Surface`) and rectangles. A vertical edge is derivable from `keepouts` but nothing exposes it; a ladder/rope anchor needs a place that moves with the host element. Needs a new survey product (e.g. vertical edges/anchors with ids) in `SCH`, `SCH.json`, `SCH.rs`, `Surveyed`, `Stage`, `survey` (`STG:1425`), `measure` (1143), and the React survey `SUR` (only emits top edges, keep-outs, floor).
* *Actor state*: no attachment (what it holds, where, how long), no orientation. Candidates: new `Actor` fields (anchor/tether/length, side) or an activity-specific blob. `vx`, `vy`, `goal` exist but are used only by hop/fall.
* *Activities*: new enum members (hang, reel, climb, glide/parachute...). Append at the end of `ACTIVITIES` to keep the 13 trace digests unchanged. Update: `BEH` (`MOODS`, `DWELL_*`, `ENERGY/SOCIABILITY/CURIOSITY_RATES`, `RAPPORT_STEPS`, the `AFTER_*` graph and `followersOf`, the literal arrays of `activityWeights`), Python oracles, schema (3 files), `Repertoire` (JSON object, Rust struct, `VAL` repertoire check uses `ACTIVITIES` automatically), `GEN` and the case Python files, sample menagerie generator (`generate_schema_vectors.py`), 20 species JSON files (clips per activity, or fall back via `clipsOf`).
* *Decision*: `decide` (667) chooses only among idle, fidget, walk, hop, sleep via `activityWeights`; `Situation` has no field for "a climb is available". New activities would also need a trigger (an event: e.g. a grapple "fired", a ladder "placed") because a pet that ends a climb mid-air must `conclude` somewhere; today `conclude` returns for walk/hop/fall (`STG:736`) because motion ends them.
* *Motion*: add branches in `act` (`STG:935-943`), add the activity to `lull`'s "moving" test (`:1091`) and to `paceOf`'s 64 rate (`:1649`), add `riding` support in `carry` (`:1155` ignores `perch === null`) so an actor on a rope/ladder follows its scrolling anchor, and decide whether the activity counts as a "mover" (`decide:679`, `pair:978`, `tune:1535`, `pairingTick`, `lawsOf` `paced`, Python `MOVERS`).
* *Pose*: no whole-body rotation in `ActorFrame`; a root-bone rotation track can rotate the rig about the feet only; hanging from a hand or sliding down a wall needs either a frame-level angle/pivot or bones authored for it.
* *Gaze*: `gazeGoal` (`:329`) looks at the pointer only when not walking/hopping/falling; the new activities need a rule (`moving` list).
* *Spacing*: the new states are airborne (see 4.c).
* *Keep-outs*: climbing a card edge puts the body next to a surface box that is itself a keep-out inflated sideways by 4 px (survey rule); law `clear` is exempt for `perch === null` (`TRC/🟦️.ts:101`), so nothing would stop a climber covering text.

**Pinned (would need to change)**

* STU: `only ever changes activity along the activity graph` (315-343); `names what every actor is doing` loops `ACTIVITIES` (1720-1726); `asks for 64 ticks per second while anyone fades, walks, falls or lands` (1700); `lets the walkers beyond the limit come to rest` (864); `lets whoever was in the air arrive anew` (828).
* BEH suite: `answers in ACTIVITIES order and only ever lets an idle pet choose idle, fidget, walk, hop or sleep` (114); `leaves a still pet nothing but idle` (136, literal 11 zeros); `leaves a quiet pet only idle and sleep` (140); `gives every activity the mood of the design` (273); `the activity graph` (284-313: reach all, order, idle in every list, only squabble -> sulk, only hop/fall -> land, committed graph with 1 component); `needs` and `bonds` vectors.
* VAL suite: `the sample menagerie covers ... every activity` (108-115) pins the 11 repertoire keys of the sample species.
* Protocol v2: `🧠️behavior-choice` scenarios `weights`, `decisions`, `dwells`, `moods`, `reachability` (graph must stay one component), `limits`; `🤝️bond-dynamics` `rapport-steps`, `needs`, `days`; `🧬️schema-conformance` (sample menagerie); `🎪️stage-trace` `traces` (digests), `laws` (`perched`, `clear`, `apart`, `paced`, `paired`, `graphed`, `whole`), `determinism`.
* React: `DEP` slot `ACTIVITIES.indexOf(frame.activity)` (`:211`) and the `data-pet-activity` attribute; suite `🖌️pet-depiction`.

### 4.b An actor held by the user (position driven by the pointer, then released with a velocity)

**What must change**

* *Events*: a grab/drag/release set (press with target, move while held, release with velocity) - `Pointed`/`Poked` carry only x/y and the layer sends `poked` on `pointerdown` only (`SUR:305-312`); `pointerup` is read for touch only (`SUR:316-318`). Velocity needs either the layer to measure it or the stage to remember the previous sample (`Stage` keeps only `pointer` and `pointed`). The layer folds `ticked` first and then `waiting` events (`LAY:228-252`) and replaces earlier `pointed` events of one frame (`LAY:304`), so samples are one per animation frame (<= 8 ticks apart).
* *Layer*: `pointer-events: none` on the layer and listeners that are `passive` (`SUR:319`); grabbing needs a hit test (exists: `hit`, `LAY:272-282`, a bounding box of `size`, not the drawing), the right to swallow the press (so the page beneath does not also click), `touch-action` handling, capture until release.
* *Stage state*: a "held" marker + grip offset; position = pointer - offset each `pointed`; `perch = null`; on release `vx, vy` and a fall. `plunge` (`STG:813`) ignores `vx` (x constant: the unit test `lets %s fall when its perch vanishes` expects `x: 550` unchanged and `vy: 0` at the start); `TER.fallStep` is vertical only; `hop` is the only ballistic motion with `vx`, and `hopStep` writes a target on the last tick. A thrown pet needs horizontal flight with gravity, wall/floor contact and landing on any perch crossed (reuse `landingOf` sweep with x movement).
* *Interactions to cut*: `release` the partner; exempt from `presenceOf` (otherwise a held pet fades to 0.35 because the pointer is on it, `STG:270-276`, `:929-932`); exempt from `watched` wake logic; `lull` must return 0; `paceOf` 64; `gazeGoal` rule; `poke`/`perk` guards (`:1553`, `:911`); `carry`/`ride`/`seat` ignore it (they do: `perch === null`); `spread` (`:1410` idle only); `tune` still/quiet semantics (is dragging allowed in `still` or during a run? the layer drops pokes in `still`, `LAY:299-303`).
* *Spacing*: a held pet can be dragged through others; see 4.c.
* *Keep-outs*: dropping a pet on top of text/controls: `soars`, `perchesOf` and the `clear` law cover grounded actors only; after release it lands on a perch (perches are keep-out-free by construction) but while held it covers anything.

**Pinned**: `STU` `a poke in $name` (954-1011: greet/facing/mood/reach 1.2/nearest/wake sleeper/stop walker/busy partner/reconcile), `a still stage ... later = advance(... poked ...)` (822-825: a poke on a still stage changes nothing), `the pointer's courtesy` (1548-1610: see-through under the pointer incl. fade-in clamps), `falling and landing` (554-578: x constant and `vy = 0`), `determinism` (frozen inputs); trace scripts `pokes-and-glances` (21 pokes), `pointer-rest` (see-through), `pointer-attention`, `calm-home`; React: `🫥️decorative-layer` (pointer-events none: tests at 277 and 302), `📡️surface-survey` pointer cases, site spec `🐕️pet-walk` tests at 463 (never takes a click, `pointer-events: none` asserted at 469) and 655 (see-through).

### 4.c A hard no-overlap guarantee between any two actors at every tick

**What exists**: grounded x-spacing (2.8). **What must change**

* Define "overlap": only `Species.size` (rest box incl. stroke and idle loop) is available; poses can exceed it (art limit in the closing summary, item 2). Either accept the box as the contract or add authored per-clip bounds; the core cannot compute drawn bounds (no `Shape` path parser, matrices only).
* Prevention at the source for airborne motion: `hopsOf` (`:602`) reserves nothing; `fly` (`:830`), `plunge` (`:813`) and the float glide never look at anyone. Options: (1) block/abort (shorten or cancel flights when the next position would intersect; needs a defined fallback, since a hop must end on a perch and `hopStep` lands exactly), (2) a final deterministic separation pass at the end of `step` (and at the end of `survey`/`summon`/`tune`/`poke` handlers) that pushes airborne actors apart, (3) reservations/occupancy timelines of arcs (complex; arcs are deterministic from the launch, so other deciders could test them).
* Cases to handle explicitly: airborne vs grounded, airborne vs airborne, different perches with overlapping x and close y, newly arrived (opacity 0) actors, leavers crowded out (`perch = null`), held actors, `seat`'s squeeze (uses x only), pairs "acting across the gap" on different perches, float vs walker on one perch (hover raises the floater's feet: boxes can overlap in x yet be separated in y).
* Resolution order: actors are processed sequentially, so any "at most overlap `CONTACT`" check must hold for the *end of the tick*; `hindered` today sees partly updated neighbours. A separation pass after `meet`/`pair` in `step` is the least invasive place.
* Laws: `apart` must become a 2D box law for all pairs; three adapters (`TRC/🟦️.ts:114-121`, `TRC/🦀️.rs`, `TRC/🐍️.py:117-124`) and the unit helper `overlapping` (`STU:1020`) change; the feature text (`TRC/🥒️.feature:35-37, 58`) too.

**Pinned**: STU `keeping their distance` (1013-1213, 8 tests), `arrival ... keeps newcomers a comfortable gap apart` (167), `never lets anyone stand inside a keep-out` (247), `walking and riding ... is held on its perch when the perch shrinks` (539), `falling ... comes down beside whoever stands where it lands` (1169, exact expected x `550 + shoulders + 8`), `hopping between perches` (1215), `a hopping gait` (1501-1546); trace scripts `crowded-strip`, `narrow-stage`, `tab-and-footer`, `moving-card`, `new-ground`, `lively-card`; law `apart`. A stricter law will likely change committed digests wherever the new pass moves somebody.

### 4.d Species-specific states (sun: dim / shining / blazing) and tricks

**What must change**

* Data first (the framework is domain-neutral): the species document needs states and tricks (names, transitions or triggers, per-state repertoire/clip overrides, dwell, effect on mood/needs/appearance). `Species` is a closed object in `SCH`, `SCH.json`, `SCH.rs` (`deny_unknown_fields`) and `VAL` (`SPECIES_FIELDS`, 27; structural checks, rule codes, a new vector per rule: the unit test `every rule of the design has a vector` at `VAL suite:133`); 20 species JSONs under `🎓️teaching/🏛️architecture/🐾️pets` (`AP/<emoji><id>/🔣️.json`) to author.
* Runtime state: `Actor` has no species-specific slot (a state id + since). Add fields (`Actor` in 3 schema files + `draftOf`/`sealed` need nothing since they copy actors; the Rust `Actor` struct and test fixture `SCH` unit test `actor()` at line 78 change).
* Behaviour: `activityWeights` takes `Species` but reads only `repertoire.fidget`; state-dependent weights/trick selection need `Situation`/weights extended and the Python oracle restated.
* Appearance: the frame cannot express "blazing" except through clip values on existing channels (x, y, rotation, scale). There is no per-part visibility/opacity (`Part` has none; design §13.7: "a part cannot be hidden by opacity") and no palette in the frame (the palette is applied once, `DEP:150-190`). A glow/brightness state needs a new channel or frame field; `ActorFrame.bones` is a flat 6-per-bone array and a test pins `bones.length === 6 * bones` (`STU:1743`).
* Draws: trick selection should use a dedicated stream or new words after existing ones.

**Pinned**: validation sweeps (`VAL suite` four sweeps against ajv on the sample menagerie), `SCH` contract-twin test, `schema-conformance` vectors (generated by `generate_schema_vectors.py`), STU `draws for every actor from its own stream` (1685), behaviour vectors.

### 4.e A richer mood than the present scalar

* Where mood lives: `Actor.mood` (`SCH:251`), eased by `cheer` (`STG:873`) to `moodOf(activity)` (`BEH:109`), +0.3 by `poke` (`STG:1566`), reset by `arrive` (`:1372`) and `freeze` (`:1506`), copied to `ActorFrame.mood` (`:1701`) and drawn as a mouth bend only (`DEP:253`, `bendOf`).
* It enters `lull` (`:1092`, `mood !== moodOf(activity)` forces ticking) and `paceOf` (`:1654`, rate 32 while mood is not at target), so a mood with its own slow dynamics keeps the stage ticking and the frame at 32 unless the rest condition is extended.
* A richer model (valence/arousal, named emotions, memory) touches: `SCH` (`Actor.mood`, `ActorFrame.mood`, JSON bounds -1..1), 3 twins, `moodOf`/`MOODS`, `cheer`, `poke`, `freeze`, `arrive`, `frameOf`, renderer mouth/eyes, the digest (`TRC/🟦️.ts:64` folds `actor.mood`), Python `MOODS` and `moods` vectors; and, if it is to feed decisions, `activityWeights`/`Situation` and all weight vectors.

**Pinned**: BEH `gives every activity the mood of the design` (273); STU poke tests (mood `moodOf("idle") + 0.3` to 12 digits, `-0.2` cheer), encounters `one.mood < 0.3` during squabble (686), rate tests (1717: mood 0.5 -> rate 32), `finite` frame check (1743: `|mood| <= 1`); `🧠️behavior-choice` `moods`; stage-trace digests.

### 4.f Per-click escalation (hello -> trick -> purr)

* There is no per-actor click memory. `poke` (`STG:1547`) is stateless: cheer +0.3, then `hail` (always `greet`, 2..5 s via `dwellOf(greet)`, clip `greet`). Repeated pokes simply restart the greeting.
* Needs: a counter + window per actor (state in `Actor`, decays with ticks - must be settled lazily like needs or included in `lull` horizons), new activities or clip kinds for "trick" and "purr" (see 4.a for the activity cost; "purr" is closest to a solo `cuddle`, but `cuddle` requires a partner in `headingOf`/`meet`), per-species trick clip mapping (4.d), and handling of fast sequences: `poked` events are not deduplicated by the layer (only `pointed`/`unpointed` are, `LAY:304`), so several can arrive in one frame at the same stage tick.
* Guards to revisit: busy-with-partner branch (`:1567`), sleeper wake, `quiet` (ignored), `still` (ignored), `perch === null`, `PET_CONTROLS` target exclusion in `pressed` (`SUR:310`).

**Pinned**: all `a poke in $name` tests (954-1011), trace scripts with pokes, `🐕️pet-walk` (no poke test by name, but `greet` activity observed in the "lively pets walk" test).

### 4.g Gestures such as the pointer circling an actor

* The stage keeps `pointer` and `pointed` only; a circle needs a trajectory: a short ring of recent samples or running accumulators (e.g. signed area / winding relative to the actor centre) held in `Stage` or `Actor`, updated in the `pointed` branch of `apply` (`STG:1575`), with a timeout (more `lull`/`wake` horizons, 4.0 X4).
* Arithmetic: no `atan2`; use cross and dot products or quadrant counting. Samples arrive at most once per animation frame (`LAY:304`), possibly 8 ticks apart at high load; still stage: the layer forwards only pointers on or beside a pet (`LAY:299-303`).
* Result of a gesture: an event-like effect inside the fold (a new activity or mood effect), keyed by actor; per 4.f.
* Do not break `perk` (it fires on the exact tick `pointed + 32` while the pointer rests, `STG:911`) or the turn logic, which read the same pointer.

**Pinned**: `attending to the pointer in $name` (1329-1500), `the pointer's courtesy` (1548-1610), trace scripts `pointer-attention`, `pointer-rest`, `STU:1478` (`jumps over the time in which a resting pointer changes nothing`).

### 4.h Stage entities besides actors (ropes, ladders, parachutes, rain) that must appear in the frame

* `Stage` and `Frame` have no entity list. Add runtime state (e.g. a prop list with kind/owner/anchor/since/until) and frame entries (geometry, opacity, z); particles like rain can be closed forms of `(seed, emitter, tick)` to avoid per-particle state; they need their own random stream (reserve in `RND`).
* `frameOf` (`STG:1669`) re-derives actor lists and returns `{ tick, actors, rate, wake }`; props must join `rate` (`paceOf`), `wake`, `lull`. The frame order is "back to front by y"; a rope behind and a parachute in front of its owner needs a z rule.
* Renderer: `LAY:show` (192-216) creates one SVG per `species` and reuses it by species id; `DEP` has no notion of anything else. The layer suites assert what the layer contains (one static element, nothing interactive: `P/🧪️tests/🫥️decorative-layer`, `🖌️pet-depiction`).
* The digest and the laws fold `frame.actors` only (`TRC/🟦️.ts:57-67`, `whole` law: `frame.actors.length === stage.actors.length`).
* Rust: new structs in `SCH.rs`, `Rate`-like enums for kinds, adapters.
* Ownership is by species slug (X2): a rope/parachute owner reference is a slug.

### 4.i The stage steering a displacement of host UI elements ("fixtures") and reacting when the user touches one

* Data flow today is one-way: the host surveys, the core folds, the layer paints inside its own layer only. The design promises "never changes layout or scroll" (design §6.5; `LAY:7-8`). A fixture contract is a new product decision.
* Core: a fixture description in `Surveyed` (id, box, movable?), stage state for displacement (offsets/velocities, spring back), a frame output (`fixtures: id -> offset`), and events for the host's side (user touched/dragged a fixture). Surfaces already ride moving cards frame-accurately via `carry` (`STG:1155`) because the host re-surveys (`LAY` re-measures every 8 ticks while the rate is 64, once a second while it is 32 or 16, and at rate 0 only when something marks the survey stale, `LAY:234`; scroll/resize/mutation/focus/transition make it urgent). A displaced element that the stage itself moved will be reported back by the next survey: the stage must separate "rest position" from "applied offset" or it double counts.
* Pets pushing UI during a run contradicts `quiet` (nothing may ask for a click) and WCAG target-size/focus constraints; `PET_CONTROLS`/focused element keep-outs make controls un-standable but not immovable.
* The survey observes attributes `class, hidden, inert, open` only (`SUR:225`), not `style`, so the layer's own transforms do not trigger mutation surveys; layer transitions are ignored.

**Pinned**: `📡️surface-survey` suite and `generate_survey_vectors.py` (15 scenes), `🫥️decorative-layer` (`follows a still stage's surfaces`, `hands other things worth a look`), site spec cases 626 (stand on what the cards show, keep distance), 673 (bottom of a drawing meets the top of a tab), 804 (phone); trace scripts `moving-card`, `tab-and-footer`, `new-ground`, `narrow-stage`.

---

## 5. Size and coupling: can `STG` be split without changing behaviour?

Yes. The file already carries its seams as region markers (2.1), the call graph is shallow, and there is no module-level mutable state (Rust unit tests grep for `static`, `thread_local!`, maps and print macros). The only cross-region cycle risks are `attend` (used by `stride` and `approach`) and `poke` (needs `hail` and `reconcile`); both resolve by placement. A script checked the proposal below against the real call graph: **no cycle** (edges at the end).

Proposed modules (names are placeholders; each needs an emoji from the taxonomy convention, a `members-of-modules` entry, unique docstring emojis, no banned stems; line counts include docstrings):

| Module | Functions | Lines (est.) |
|---|---|---|
| `draft` | `draftOf, sealed, hoverOf, indexOf, surfaceOf, clipsOf, clipAt, clipOf, breathOf, facingTo, blinkAt, actorKey, stageKey, shoulders, beatOf, shift, settle, release, remove, carve, crowdOn` + types `Body, Draft, Room, Launch` + constants shared | ~185 |
| `spacing` | `vacancy, clearway, hindered, soars, roomsFor, quarters` (+ `COMFORT_GAP, MEET_GAP, CONTACT`) | ~110 |
| `schedule` | `sociable, pairingTick, arrivalTick` (what `frameOf` and `lull` need to know about future events) | ~26 |
| `attention` | `watched, presenceOf, headingOf, turn, gazeGoal, gazeRests, look, wink, cheer, swivel, hail, perk, leant, squeezeOf` | ~150 |
| `locomotion` | `vanish, crowdOut, drop, touch, paced, stroll, attend, stride, aim, plunge, fly, hopsOf, leave` | ~230 |
| `sociability` | `sulk, recall, bond, reconcile, pair, parted, reachable, approach, meet, poke` | ~175 |
| `choice` | `decide, conclude` | ~80 |
| `population` | `measure, carry, seat, widened, ride, arrive, spawn, spread, survey, summon, freeze, tune` | ~310 |
| `clock` | `act, lull, step, pass` | ~85 |
| `projection` | `replaces, weightOf, layer, poseOf, paceOf, frameOf` | ~115 |
| `stage` (facade, existing name) | `openStage, apply, advance`, re-export of `frameOf`, the NORMATIVE ORDER header | ~25 + header |

Dependency direction (each arrow = "imports"; verified acyclic): `stage -> clock, population, sociability, draft, projection`; `clock -> attention, choice, locomotion, population, schedule, sociability, draft`; `choice -> attention, locomotion, sociability, spacing, draft`; `sociability -> attention, locomotion, schedule, spacing, draft`; `population -> locomotion, spacing, draft`; `locomotion -> spacing, draft`; `projection -> attention, schedule, draft`; `attention, spacing, schedule -> draft`; `draft -> behavior, randomness, schema`.

Notes for the split:

* `export *` in the barrel collides on equal names; keep the sub-modules out of the barrel (only `stage` is listed) and the internals unexported to the package. Internal cross-module functions must be exported from their files (TS) / `pub(crate)` (Rust; precedent: `larger`/`smaller` in `TER`, `stage` uses them).
* Rust: one `🦀️.rs` per new module, `#[path]` mounts in `P/📦️packages/🦀️rust/🦀️.rs`, `Draft<'a>` is shared with lifetimes; `P/🦀️.rs` re-exports only the public surface. The Rust file is a literal mirror (names, regions, order), so the split is two parallel edits and must land together.
* Tests: the vitest `include` is `modules/*/🧪️tests/🔬️unit/🟦️.ts` (`P/🧪️tests/🎚️config/🟦️.ts:32`), so new top-level modules need no config change; nested folders would. The stage suite's grep test (`STU:1778`) reads only the facade; it must scan every new file. The suite's helpers (`STU:24-127`) would move to a shared test-support file (name without banned stems).
* **Proof of "no behaviour change"**: the 13 committed traces (digest at every 640 ticks over 128 000 ticks), the unit suites (510 tests at the fundamental level), `generate_behavior_vectors.py` printing `unchanged` for all three fixtures, Rust `parity` for `🎪️stage-trace` and the long run (0 mismatches). A split followed by any behaviour change is easy to attribute because the digests either stay or move.
* The Rust twin file is 2215 lines, the same shape; its 31 unit tests mirror a subset of the TS suite. Splitting Rust first would break the 1:1 diff method used so far (`ported-from` copies; baseline today is `git show HEAD:<path>`, commit `83025db9517`, the only commit that touches the stage files).
* Which capabilities would live where: climbing/hanging/parachute = `locomotion` + new activity tables; held = `attention` + `population` + `locomotion`; no-overlap pass = `spacing` (+ a call in `clock.step`); states/tricks/mood/escalation = a new module fed by `choice` and `sociability`; props = `schema` + `projection` + a new module; fixtures = `population`/`survey` side. Shared hot files that every capability touches and that therefore serialize parallel work: `SCH` (3 files), `BEH` tables, `ACTIVITIES` order, `activityWeights`, `TRC` adapters (3 languages), `GEN` and the Python oracles, `DEP`/`LAY`. A refactor-first sequence (split, then add) avoids merge fights in `STG`.

---

## 6. Test and oracle machinery

**Unit suites (TypeScript)**: vitest, one suite per module (`<module>/🧪️tests/🔬️unit/🟦️.ts`), picked up by the glob in `P/🧪️tests/🎚️config/🟦️.ts:32`; run with `bun ./📜️script.ts test [quick|long|exhaustive]` in `P/📦️packages/🟦️typescript` (`📜️script.ts` resolves the level and calls `runVitest`). 8 files, 510 tests at every level except 514 at `exhaustive` (report Q). Stage suite: 93 `it` registrations, 90 of them played on two menageries each (`SAMPLE` = the sample menagerie of the schema-conformance fixture: a walker, a hopper, a floater; `TROUPE` = the five blobs of the stage-trace fixture), `STU:38-42`. Rust unit suites: 31 stage / 39 behaviour / 31 terrain / 23 animation / 14 rig / 15 randomness / 16 trigonometry / 11 validation / 9 schema (counts of `#[test]`, 189 total in report M).

**Test levels**: `P/🧪️tests/🎚️config/🟦️.ts:12-17`: `LEVEL` from `SEMIO_TEST_LEVEL` (set by `resolveTestLevel`, `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts:30`, default `fundamental`); `sampled(few, full, more)` returns `few` at fundamental, `more` at exhaustive, `full` at quick and long. Only amounts are sampled; every assertion runs at every level. Budgets (`...budget/🟦️.ts:7-10`): 15 s / 300 s / 900 s / 1800 s (`SEMIO_TEST_BUDGET_MS` overrides). Measured by report Q: fundamental 2.8-3.8 s wall on a quiet host (stage 1.29 s of test time), sometimes more under load (one budget kill in 24 loaded runs). A new feature's long sessions must be written with `sampled` (the stage suite does, e.g. `STU:267`, `:1104`, `:1198`); the slowest test was 81 ms after Q.

**Protocol v2 cases** (`P/🧪️tests/<case>/{🥒️.feature, 🐍️.py, 🟦️.ts, 🦀️.rs}`, vectors `P/🧫️fixtures/<case>/🔣️.json`, registry `P/🔮️oracles/🔣️.json`): run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` with `bun ./📜️script.ts {oracle|subject|parity} <level> --owner "🧰️framework/🛍️products/🐾️pets" --case "<dir with emoji>"` (the bare slug selects nothing). Cases touched by the stage/behaviour:

| Case | Scenarios (id, level) | Oracle |
|---|---|---|
| `🎪️stage-trace` | `traces` (fundamental, differential), `laws` (fundamental, property), `determinism` (quick, property) | no-oracle decision `pets-stage-trace` (`🔮️oracles/🔣️.json:169`): specification vectors + metamorphic laws + independent implementations (TS vs Rust) |
| `🧠️behavior-choice` | limits, weights, picks, decisions, dwells, moods, encounters, reachability, casts (9) | numpy `cumsum`/`searchsorted`/`SeedSequence`/`roll`, scipy `csgraph` |
| `🤝️bond-dynamics` | affinities, rapport-steps, rapport-fading, histories, needs, days (6) | numpy |
| `🧬️schema-conformance` | accepted-documents, structural-rejections, rule-violations | python-jsonschema + ajv in unit suite |

**Stage-trace in detail**: fixture `🧫️fixtures/🎪️stage-trace/🔣️.json` (543 229 bytes) holds the five-blob menagerie and 13 scripts (ids, ticks): `calm-home` 11 520, `lively-card` 15 360, `moving-card` 9 600, `quiet-run` 11 520, `still-and-back` 7 680, `scene-change` 8 960, `pokes-and-glances` 9 600, `narrow-stage` 7 680, `crowded-strip` 9 600, `tab-and-footer` 11 520, `pointer-rest` 7 680, `pointer-attention` 9 600, `new-ground` 7 680 = 128 000 ticks, a checkpoint every 640 ticks (10 s). Replay: for tick 1..N fold the events of the previous tick boundary, then `ticked(1)`, `frameOf`, laws, digest (FNV-1a 32 over the bit patterns of tick, rate, wake, actor count, per actor species index, x, y, facing, activity index, opacity, every bone number, eyes, mood). Checkpoints record the digest and a `Sighting` per actor (species, perch, x, y, activity, partner, leaving, opacity). Laws per tick: `perched`, `clear`, `apart`, `paced` (grace of `HOP_TICKS + 1` after a mode change), `paired`, `graphed`, `whole`. `determinism`: second replay equal, next seed differs, uneven chunks (`1,7,64,3,500,2,19,1000`) end in the same stage. The Python file simulates nothing; it re-checks the recorded checkpoints (`verify`) when vectors are generated and answers with the committed projections.

**Generation** (`GEN`, run from the repo root `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_behavior_vectors.py`; `.venv/bin/python` elsewhere): writes `🧠️behavior-choice` and `🤝️bond-dynamics` (inputs composed in Python, expected values computed by the Python oracles in the case files, i.e. the **tables are restated in Python**) and `🎪️stage-trace` (menagerie from `TK/reference-species.json` + `extra_clips`, scripts in `scripts()`; the traces are recorded from the **TypeScript subject** by `TK/record_stage_trace.ts`, which imports `traceOf` from the TS adapter, then `verify` of the Python adapter must pass or nothing is written). A second run must report `unchanged` for all three files (reproducibility check). Other generators: `generate_schema_vectors.py` (sample menagerie, 21 accepted / 26 structural / 63 rules), `generate_survey_vectors.py` (React survey, 15 scenes), animation/terrain/kinematics generators (untouched by the stage). Duration: not measured here; the cost figures of report E (4-8 microseconds per tick plus 20-32 microseconds per frame for 10 actors) put one full TS replay of the 13 scripts in the order of several seconds; the exhaustive Rust parity run of the case takes about two minutes (report M).

**What to rerun after a core change**
1. TS unit suites (`bun ./📜️script.ts test`, plus `quick` for the statistical sessions) and `typecheck` (`@semio-tech/pets:typecheck`, 32 files incl. adapters).
2. If limits/weights/dwells/moods/graph/needs/rapport changed: edit the Python tables in `🧪️tests/🧠️behavior-choice/🐍️.py` and `🤝️bond-dynamics/🐍️.py` and `GEN`; then `GEN` (twice).
3. If the frame, the activity set or any stage rule changed: `GEN` re-records all 13 digests (they move wherever behaviour moved).
4. Schema changes: the three schema twins, `SCH.json`, `schema generate`/`schema docs` (the schema catalog), the Rust schema tests, `generate_schema_vectors.py` if documents change.
5. Protocol v2: `oracle`, `subject --implementation typescript`, `parity` per touched case; parity needs the Rust twin to be ported first.
6. Rust: port by diff, `bun nx run @semio-tech/pets-rs:test`, clippy `-D warnings`, `verify rust-warnings` on native/wasm targets, `rustfmt --check`.
7. React target and quiz suites if the frame or events change (`@semio-tech/pets-react:test`/`typecheck`, quiz `🐾️pet-companions`, site `🐾️pet-cast`, e2e `🐕️pet-walk`).

**How the Rust twin is kept in sync, and the proof** (report M): hand port, diff against the last ported TS copy; evidence = Protocol v2 parity (stage-trace 3 scenarios compared pairwise, `parity=3/3`; behavior-choice 27/27; bond-dynamics 18/18; all 12 cases 186 executed), `TK/rehearse_stage_adapters.ts` (4782 projected numbers compared without tolerance, 0 differences), the long run `TK/long_trace_digest.ts` + `.rs` via `TK/rust_scratch.sh` (48 sessions of 20 min, 76 800 ticks each, 18 883 events, 57 600 checkpoints with two digests each, 0 mismatches), and mutation checks `TK/stage_scratch.ts mutants` (18 mutants, 3 expected survivors). The ticket tools sit in `TK`; their scratch (`TK/🗑️generated`) is gone, `stage_scratch.ts prepare` recreates what a rerun needs. Known blind spot: transient quantities inside `pair` have no bit-level evidence (report M section 7).

---

## 7. As-built behaviour that contradicts the new wishes

| # | As built | Where | Pinned by |
|---|---|---|---|
| 1 | **Quiet ignores pokes** (and every interaction): `poke` returns on `draft.quiet`; no pointer turns; no lean; no perk; no new walk/hop/fidget; sleep tripled; no encounters; no spreading | `STG:1548`, `:290`, `:1698`, `:911`; `BEH:80-86`; `STG:961`; `:1396`; `STG:685` (hops not even evaluated); layer rotation freezes `LAY:237`; quiz sets `quiet` on screen `run` (`QR/🔨️modules/🐾️pets/🟦️.tsx:192`) | STU `is ignored in a time of concentration` (985), `a time of concentration` (757-801), `does not perk up ... in a time of concentration` (1464), traces `quiet-run`, `pointer-attention`; design §5.6, §13.3-13.4 (audit N7) |
| 2 | **Pets turn see-through while the pointer rests on them** - a held or clicked pet would fade to 0.35 under its own pointer; a see-through pet is not `sociable` (opacity must be exactly 1) and cannot perk | `STG:270-276`, `:929-932`, `:955`, `:911`, `:1701`, `lull :1091`, `paceOf :1649` | STU 1548-1610, trace `pointer-rest`, layer tests 414/443, spec `🐕️pet-walk` 655, design §13.3 |
| 3 | **The layer never takes pointer events**; listeners are passive; poke/hit testing is a bounding-box test on `pointerdown` and the same click also reaches the element beneath (nothing is suppressed); `pointerup` only for touch | `CSS:9`; `LAY:348`, `:272-282`; `SUR:296-332` (`:319` passive) | `🫥️decorative-layer` 277, 302; spec 463/469 (`toHaveCSS("pointer-events","none")`, `takenTargets`); design §6.5 |
| 4 | Poke needs a grounded actor (`perch !== null`); an airborne pet cannot be poked, caught or reacted to | `STG:1553` | STU `a poke` block |
| 5 | A poke has no memory: always cheer +0.3 and a fresh greet | `STG:1566-1569` | STU 959-995 |
| 6 | Only `idle` actors choose; nothing else can start a new activity on its own; a chosen walk/hop/fall runs to its end | `STG:733-748` | activity-graph tests |
| 7 | One actor per species and species-slug identity | `STG:130-137` | STU `draftOf`/Rust `a_draft_keeps_one_actor_per_known_species` |
| 8 | At most one encounter on stage; mover budget 1 (calm) / 2 (lively) counts only walk and hop; `still` allows nothing | `STG:964`, `BEH:39-43`, `STG:979` | STU 702-755, law `paired`, `paced` |
| 9 | Mood does not influence behaviour; mood is the activity's mood | `STG:873-878`, `BEH:106` | 4.e |
| 10 | Spacing guarantee is grounded/same-perch/x-only; airborne actors, different perches, y are unchecked; float glides ignore keep-outs and others | `STG:616-629`, `:830-862`; `TRC/🟦️.ts:114-121`; `STU:1020-1029` | 4.c |
| 11 | `ActorFrame` has no rotation/pivot, no tint, no props; the renderer paints actors only | `SCH:288-301`; `LAY:192-216`; `DEP:193-260` | 4.h |
| 12 | Gaze ignores the pointer while walking/hopping/falling; pointer interest lasts 4 s; one pointer only | `STG:328-329`, `:63` | STU 346-426 |
| 13 | The fall ignores horizontal velocity (x constant) and always starts at `vy = 0`; hops are the only ballistic motion and always aim at a perch | `STG:813-827`, `:460-469`; `TER:128-131` | STU 554-578 |
| 14 | Survey cadence: every 8 ticks only while the frame rate is 64; once a second at rates 32/16; at rate 0 only when marked stale (scroll/resize/mutation/focus/transition make it urgent); the survey sees top edges only and drops anything narrower than 48 px | `LAY:234`, `:48`; `SUR` `SURFACE_WIDTH = 48` | survey suite |
| 15 | `still` ignores pokes and, in the layer, every pointer event that is not on or beside a pet | `STG:1548`, `LAY:299-303` | STU 803-826 |
| 16 | The depiction pins activities by index and the sample menagerie must carry all 11 repertoire keys | `DEP:211`; `VAL suite:112` | 4.a |
| 17 | Desktop pages other than the overview offer only the footer line as a perch (35-57 px under the navigation bar vs. 40-56 px pets): a host layout fact that limits where climbing/ladders could start | design §13.4 / closing summary item 1 | - |

---

## Appendix A - call graph of `STG` (callee <- callers)

Computed by script over the file; function names as in 2.2.

```
draftOf<-advance  sealed<-advance  hoverOf<-touch,hopsOf,plunge,fly,measure,carry,arrive
indexOf<-headingOf,gazeGoal,release,reconcile,meet,arrivalTick,measure,spawn  surfaceOf<-carry,widened,ride
clipsOf<-clipAt  clipAt<-settle,crowdOut,drop,touch,attend,sulk,decide,stride,fly,hail,reachable,approach,meet,arrive,leave,tune
clipOf<-breathOf,touch,paced,decide,stride,poseOf,paceOf  breathOf<-poseOf,paceOf  facingTo<-headingOf,hail,approach
blinkAt<-wink,arrive,tune  actorKey<-settle,sulk,decide,wink,hail,tune  stageKey<-pair,meet,arrive
watched<-decide,perk,act,lull  presenceOf<-perk,act,lull,paceOf,frameOf  headingOf<-swivel,lull  turn<-swivel,hail
shoulders<-vacancy,hopsOf,clearway,hindered,seat,roomsFor  beatOf<-paced,stride  gazeGoal<-gazeRests,look  gazeRests<-lull,paceOf
look<-act  shift<-settle,crowdOut,drop,touch,stroll,attend,sulk,decide,stride,fly,hail,meet,leave,freeze
settle<-release,conclude,stride,act,summon,tune  release<-vanish,crowdOut,drop,conclude,leave,tune
remove<-vanish,act,seat,spread,summon,freeze  vanish<-plunge,carry  crowdOut<-touch,seat  vacancy<-touch  drop<-carry
touch<-plunge,fly  paced<-decide,reachable,leave  stroll<-decide,approach,leave  attend<-stride,approach  sulk<-conclude
recall<-reconcile,meet  bond<-reconcile,meet  reconcile<-conclude,poke  soars<-hopsOf  hopsOf<-decide
clearway<-decide,reachable,leave  decide<-conclude  conclude<-act  hindered<-stride  stride<-act  aim<-fly
plunge<-fly,act  fly<-act  wink<-act  cheer<-act  swivel<-act  hail<-perk,poke  perk<-act  act<-step
sociable<-pairingTick,pair  pairingTick<-pair,lull,frameOf  pair<-step  parted<-pair  reachable<-approach  approach<-pair
meet<-step  arrivalTick<-lull,step,pass,frameOf  lull<-pass  step<-pass  pass<-apply
measure<-survey,summon  carry<-ride  seat<-ride  widened<-survey  ride<-survey,summon  carve<-vacancy,roomsFor
crowdOn<-roomsFor,spread  roomsFor<-decide,arrive  quarters<-arrive  arrive<-spawn  spawn<-step,survey,summon,tune
spread<-survey  survey<-apply  leave<-decide,spread,summon  summon<-apply  freeze<-tune  tune<-apply  poke<-apply
apply<-advance  replaces,weightOf,layer<-poseOf  poseOf,leant,squeezeOf,paceOf<-frameOf
```

## Appendix B - stage unit suite blocks (`STU`, each `describe.each(COMPANIES)` except the first and the last)

`openStage` 129 | arrival 141 | default liveliness 263 | gaze 346 | blinking 428 | walking and riding 469 | falling and landing 554 | encounters 612 | a time of concentration 757 | a still stage 803 | summoning and leaving 873 | a poke 954 | keeping their distance 1013 | hopping between perches 1215 | turning round 1250 | attending to the pointer 1329 | a hopping gait 1501 | the pointer's courtesy 1548 | determinism 1612 | the rate and the wake 1695 | the arithmetic of the module 1778.

The blocks map to the proposed modules: arrival/still/summoning/distance/hopping-gait -> population+spacing+locomotion; gaze/blinking/turning/attending/courtesy/poke -> attention; encounters -> sociability; concentration/liveliness/determinism/rate -> clock+projection.

## Appendix C - what was not verified

Nothing was run. Durations of the generators and the Rust parity runs are quoted from reports E, M and Q. The call graph and the split check were computed by reading `STG` only (the Rust twin was glanced at for structure and counts). The ticket generated folder (`TK/🗑️generated`) no longer exists, so the tools that need it (`stage_scratch.ts prepare`) recreate their scratch on use.
