# Explore 2: species content for the twenty pets (states, tricks, purr, moods, gear, mischief, chemistry)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02 by a read-only explorer. The only file written is this one. No build, test, server or modifying git command was run.

**Read for this report:** the twenty species documents `AP/<species>/🔣️.json` (bones, parts, face, clips, repertoire, locomotion, temperament, grounds), the ensemble `AP/🔣️.json` and `AP/README.md`, the pets product (`README.md`, `🧬️schema/🔣️.json`, `🧠️behavior` for `moodOf`, `ENCOUNTERS`, `rapportAfter`, `🏞️terrain` for the hop limits, `🎪️stage` for `poke`), the four energy quizzes (all 9 tasks, 109 items), the ticket notes `📓️explore-topic-pets.md` (§3 sheets, §4 social graph), `📓️report-wp-o1.md` and `📓️report-wp-o2.md` (`AP` = `🎓️teaching/🏛️architecture/🐾️pets`).

**Layout:** §0 what the rigs allow and the vocabulary used below; §A the shared vocabularies (moods, triggers, particles, gear); §S the twenty species (each: 1 States, 2 Tricks, 3 Purr, 4 Moods, 5 Gear and traversal, 6 Topic mischief); §7 Chemistry (64 physical rules R01-R64 and six mood rules G1-G6); §8 Click escalation; §9 Build cost and the four art work packages.

---

## 0. What the rigs allow, and the words used below

### 0.1 Facts about the current rigs that shape every proposal

- **Channels.** A bone animates `x`, `y`, `rotation`, `scaleX`, `scaleY` only; a part is rigid on one bone and painted with a palette token (`body`, `accent`, `detail`, `ink`, `paper`, `none`). There is no per-part opacity and no visibility switch. So (a) a state that changes colour needs a palette override, and (b) a prop that is not always visible has to be hidden today by occlusion at rest (the convention of cloudy `drop`, boily `puff`, chilly `breath`: parked behind a body part, scale 0.05 while travelling home). Everything below that needs more is marked as an engine primitive in 0.2.
- **Mood is one number.** `moodOf(activity)` eases a scalar from -1 to 1 (cuddle 1, greet .7, fidget and hop .5, walk .4, idle .3, land .2, sleep .1, fall -.2, sulk -.6, squabble -.8) and the face only bends the mouth with it; the lids are the blink. A real mood vocabulary therefore needs lid height, lid slant and a posture offset (§A1).
- **Activities are a closed list of eleven** (`idle fidget walk hop fall land sleep greet cuddle squabble sulk`); one clip is drawn per activity and every clip is additive on top of the idle loop (O1 §2.4: a spin in `idle` also runs under greet, cuddle, squabble, sulk). A state overlay is therefore exactly "a second idle loop"; the engine pattern already exists.
- **Encounters** are `greet / cuddle / squabble` drawn from the affinity (`encounterShares`); a squabble always ends in a sulk and a sulk in mending; rapport drifts +.05 (greet), +.1 (cuddle), -.15 (squabble), +.1 on mending, and fades by .1 in ten minutes. Chemistry (§7) plugs into exactly these numbers.
- **The only learner input today is `poked`** (nearest grounded actor within 1.2 x its height greets). The layer never takes pointer events and every pet is `aria-hidden`. Everything in this report that needs pointer input (clicks beyond the first, circling, stroking, holding, shaking) is an opt-in extension of the layer that must stay decoration: no pet carries information, nothing is needed to finish a quiz, `still`, reduced motion and forced colours switch all of it off, and keyboard-only learners lose nothing.
- **Hop limits** (`🏞️terrain`): a hop rises at most `HOP_HEIGHT - HOP_CLEARANCE` = 72 px and spans at most `HOP_DISTANCE` = 160 px; a fall reaches terminal speed 900 px/s after 225 px. So gear (§A4) takes over for climbs above 72 px, gaps above 160 px and drops above 60 px.
- **Walk cycles are slide-free** (O1/O2: stride per cycle = speed x seconds). Every new clip that moves the root (climb, vault, glide) must keep that discipline; size boxes were measured, so props that leave the box (coronas, ladders, canopies) are ephemeral.
- **Eyes ride on `body`** in most species, so the gaze lean (5 degrees and 1.5 px per unit of gaze) already leans whole bodies; mood posture offsets (§A1) are applied on the same bone and inherit that.
- **Palettes are three colours** plus theme ink and paper. State tints below are given as `body/accent/detail` hex triples; they keep the species' hue family and are meant to stay close to the original's lightness. They are authored by eye and **have not been contrast-tested** against the base colours `#f7f3e3` and `#001117` (9.4, point 4).
- **No text in the art.** The pets are language-free (no default language): numbers are shown as bar glyphs (a horizontal `meter`), never as digits or letters.
- **No sound.** The layer is silent; a whistle, a purr or a hum are drawn (rings, lines, tremor).

### 0.2 Engine primitives this report assumes (added once in the product, schema-first; species documents only supply data)

| Primitive | Meaning | Per-species data |
|---|---|---|
| `state` | A persistent physical condition of one pet, one of 2-4, one is the default. Held for a time (`hold`) and eased back to the default unless its cause persists. | `overlay` clip (a looping additive "second idle"), `tint` (palette override), `show` (props visible), `lean` (mood weights), `hold` seconds |
| `ladder` | The ordered subset of states that circling climbs (CW) or descends (CCW). | list of state ids |
| `trick` | A new activity `trick`: a once-clip with optional emitters and a result state. | `clip`, `emit`, `leaves`, `needs` |
| `purr` | A new looping activity for contentment. | `clip`, `emit` |
| `prop` | A part group on its own bone, shown by a state, a trick or a gear; hidden at rest by the engine (no occlusion trick needed). A prop paints with the species palette, plus two **kit paints** shared by all species so that insuly's coat looks the same on housy and waly: `wool` `#e0617f` and `wool-light` `#f7f3e3` (a schema addition to `Paint`). | parts, bone, `shownBy` |
| `emit` | A particle emitter (§A3): shape, paint token, count, motion, life. Pure function of the stage seed; at most 12 live particles per pet, 40 per stage. | emitter list |
| `gear` | Traversal activities `climb`, `descend`, `dangle` (§A4) with their props. | gear id, clips, props |
| `trigger` | Pointer gestures turned into events: `clicked(n)`, `circled(cw/ccw)`, `stroked`, `held`, `shaken` (§A2). | none (shared) |
| `mood` | A categorical mood with strength, drawn on the shared face (§A1). | `lean` (2-3 prone moods) |
| `mischief` | A host adapter that nudges quiz items (`[data-quiz-item=<id>]`) by a transform and gives the pet a `push` clip. | item ids, push style |
| `rule` | A chemistry row (§7), evaluated by the stage for pairs in range. | ensemble-level data |

### 0.3 Notation

- Clip sketch: `name 1.6s once` or `loop`, then `bone.channel value@phase` with phase 0..1; `scale` means scaleX = scaleY; angles in degrees clockwise on screen; px at scale 1; "hold" keeps the last value for the phase span given. "Existing" clips are the ones in the species document today.
- `+x` is something that does not exist yet: a new part, bone, clip or emitter. Marked (M) must-have or (N) nice-to-have where useful; §9 collects them.
- Quiz codes, as in `📓️explore-topic-pets.md` §1.3: tasks `P1 P2 P3` (physics), `H1 H2` (heating), `C1 C2` (cooling), `D1 D2` (demand); an item is `P2:sun`. Chemistry rules are `R01..R64` and `G1..G6` (§7); clicks are `L1..L6`.
- Triggers: `L1..L6` the n-th left click of a streak, `CW` / `CCW` the pointer circling the pet clockwise / counter-clockwise, `STR` stroking, `HOLD` pressed and lifted, `SHK` shaken while held, `AUTO(mood)` its own initiative while in that mood, `DUET(pet)` towards another pet.
- Hex triples are `body/accent/detail`. Default palettes: sunny `#fccf05/#fa9500/#ff344f`, cloudy `#63939a/#34d1bf/#c4e4d5`, housy `#b87f46/#a60009/#dbbea1`, solary `#2c5fb0/#c4e4d5/#c4c4b9`, radiatory `#ff344f/#fa9500/#f7f3e3`, pumpy `#1e9b8d/#34d1bf/#fa9500`, windowy `#7b827d/#bfe6ea/#f7f3e3`, waly `#a3472f/#dbbea1/#4c5756`, battery `#3e9c64/#00d492/#c4c4b9`, windy `#8282dd/#f7f3e3/#4c5756`, boily `#705c54/#fa9500/#fccf05`, roofy `#5d7185/#a3472f/#334041`, insuly `#e0617f/#f7f3e3/#a64a63`, shady `#8a5fb8/#c4c4b9/#001117`, venty `#68952c/#f7f3e3/#c4e4d5`, chilly `#1f95c8/#f7f3e3/#bfe6ea`, kettly `#808d98/#ff344f/#f7f3e3`, flamy `#de6a00/#fccf05/#c4c4b9`, thermy `#2f7fae/#ff344f/#e8e2c8`, servy `#406670/#00d492/#ff344f`.
- Every species section starts with its rig facts so the art agent sees which bones and parts carry what.

---

## A. Shared vocabularies

### A1. One mood vocabulary for all pets (nine moods)

Mood is a category plus a strength 0..1. The face is shared by every species, so the mapping is written once in the product and costs no species art. The columns are the face and body channels the engine sets: `mouth` is the existing curve (-1 sad .. 1 happy; a width factor below 1 draws a small round "o"); `lid` is a resting top-lid coverage 0 (wide open) .. 1 (shut) under which blinks still run; `tilt` is the slant of the lid edge over each eye in degrees (positive: inner end lower = hooded or angry; negative: inner end higher = arched or worried), "one-sided" means only the eye on the side the pet faces; `posture` is an additive offset on the bone that carries the first eye (y in px, rotation in degrees, scaleY); `tempo` multiplies the playback speed of the idle and overlay loops. Nothing changes colour: moods are colour-neutral so states can own the palette.

| id (en / de) | Caused by (examples) | Holds | mouth | lid | tilt | posture | tempo |
|---|---|---|---|---|---|---|---|
| `content` / zufrieden | the default; every other mood fades to it; a friend nearby; a good state | baseline | +.3 | .10 | 0 | none | x1.0 |
| `happy` / froh | L1 hello; purr; cuddle; a bond-warming chemistry rule; the pet's best state; a correct answer on its topic | 40-90 s, linear fade | +.8 | .12 (cheek squint) | 0 | y -1.5, scaleY 1.04 | x1.15 |
| `playful` / verspielt | CW circles; clicks 2-3; a high-energy neighbour (battery, kettly, windy); start of a mash streak | 30-60 s | +.6 grin | 0 | one-sided -10 | rotation +4 (leans in), idle bounce y +-1 | x1.3 |
| `curious` / neugierig | a new quiz item drawn; a pointer lingering; a neighbour changing state; any unknown pet | while the cause lasts +10 s | +.2, width x.7 | 0 | -10 both | rotation 5 towards the target, y -1 | x1.0, gaze gain x1.3 |
| `sleepy` / schläfrig | long idle; low energy; sunset and ccw winding; a warm cuddle | until sleep or a poke | +.1 | .55 | +4 | y +1.5, scaleY .96 | x0.7 |
| `grumpy` / mürrisch | squabble; hostile chemistry (open window vs radiator); mash; its topic chip taken back | 20-60 s | -.5 | .35 | +12 | rotation -3, scaleX 1.03 | x1.1, jerky |
| `sad` / traurig | a loss state (empty, cold, dim, wet, snuffed); alone for 3 min; redundant (boily next to a wrapped house) | 60-120 s; a cuddle or purr ends it | -.8 | .25 | -10 | y +2, rotation 6 away from the target, scaleY .96 | x0.8 |
| `scared` / erschrocken | dropped; held in the air; shaken; rain on flamy; a storm; second mash level | 3-8 s, then `sad` or `grumpy` | -.4, width x.5 | 0 | -14 | y +2, scaleY .92, tremor x +-.6 at 12 Hz | x1.4 |
| `proud` / stolz | a finished trick; its best state; wrapping or being wrapped by a friend; a correct answer on its topic | 30-60 s | +.6 smirk | .15 | one-sided +6 | rotation -2 (chest out), y -1.5, scaleY 1.06 | x0.95 |

- **Resolution.** Several causes may push at once: priority `scared > grumpy >= sad > proud >= happy > playful > curious > sleepy > content`; strengths decay with the "Holds" time; `moodOf(activity)` stays as the valence floor so the existing mouth keeps working (its value is the mouth column here).
- **Effect on meetings.** The affinity that `encounterShares` reads gets a mood bias, the mean of the two partners: happy +.15, playful +.10, proud +.05, curious +.05, content 0, sleepy -.05, scared -.05, sad -.10, grumpy -.25 (never below the existing floor of -.6). So two grumpy pets squabble even when they are fond of each other, and two happy ones cuddle even when they are neutral.
- **Per-species lean.** Each species names 2-3 prone moods (weights x2 when a random mood impulse fires, x1.5 on decay). They follow `temperament` as a guide (high energy leans `playful`, low sociability `grumpy`, high curiosity `curious`, low energy `sleepy`, high sociability `happy` and `sad`) and the character sheets of `📓️explore-topic-pets.md` §3; each species section gives the final list.
- **Posture budget.** Offsets are small (at most 2 px and 6 degrees) so the measured size boxes (O1 §4, O2 §4) stay valid.

### A2. Triggers

| Code | Gesture | Detection (tunable constants) | Means |
|---|---|---|---|
| `L1`..`L6` | left click on the pet | the existing `poke` (nearest actor within 1.2 x its height); a streak is clicks at most 4 s apart | L1 hello, L2 trick 1, L3 trick 2, L4-L6 purr (§8) |
| `CW` / `CCW` | the pointer circles the pet | swept angle >= 270 degrees within 2.5 s at a radius of 0.8..3 x the pet's larger side; the sign is the direction on screen | trick 3 / trick 4: one step up / down the pet's state `ladder` (the sun brightens or sets) |
| `STR` | stroking | >= 3 direction reversals within 1.5 s over the pet at < 450 px/s | `purr` |
| `HOLD` | press and hold, then move | press > 0.6 s on the pet | `dangle` (§A4) |
| `SHK` | shaken while held | >= 4 reversals in 1 s at > 600 px/s | the pet's shake outcome (§8) |
| `AUTO(m)` | its own initiative | drawn like a fidget; weight x3 in mood `m` | trick 5 |
| `DUET(p)` | towards another pet | an encounter with pet `p` in a given state (§7) | duet trick |

Clockwise winds a pet **up** (more of what it is: brighter, hotter, fuller, faster), counter-clockwise winds it **down**. At the end of a ladder a further turn plays the step clip as a flourish without a state change. After a state change by gesture the new state holds at least 20 s, then decays towards the default over the state's `hold`.

### A3. Particle kit (emitters are data, shapes and paints are shared)

| Shape | Drawn as | Typical use |
|---|---|---|
| `dot` | filled circle r 1-1.6 | sparks, bubbles, fibres, embers, packets |
| `drop` | teardrop r 1.6 x 3 | rain, sweat, drips, puddle splash |
| `star4` | four-point sparkle 3-5 px | glint, ice chip, impact, pop |
| `ring` | stroked circle growing 4 -> 14 px | hum, smoke ring, sound-free "purr ring", shock |
| `streak` | 6-10 px stroke | wind, air flow, radial rays, speed lines |
| `flake6` | six-armed flake 4 px | snow, frost dust |
| `puff` | three-lobed cloud 5-8 px (as kettly's steam) | steam, smoke, exhaled air |
| `zigzag` | 3-segment bolt 8-14 px | lightning, electric spark, power |
| `wave` | 8 px sine stroke | heat shimmer (as radiatory's shimmer) |
| `chip` | small rect 3 x 2 | ice cube, brick bit, dust of brick, tile |
| `arc` | dots along a parabola | prominence, rainbow, water stream, thrown hook |

Motions: `fall` (gravity, splash on the perch), `rise` (buoyancy with sideways wander), `burst` (radial, ease-out), `orbit` (circle around a bone), `sweep` (along a bone's path), `drift` (slow wander), `arc` (ballistic between two points). Paints are the tokens `body accent detail ink paper`, so particles follow state tints and the light/dark theme. Life 0.4-1.6 s. Budget: 12 live particles per pet, 40 per stage; none in `still`.

### A4. Gear: how pets get up, across and down (shared mechanics)

- **When.** The stage plans a climb above 72 px (hop limit), a gap above 160 px, or a descent of more than 60 px as a gear trip; the pet uses its primary gear, or its alternative, and never one it refuses (each §5 lists refusals with the physical reason). The pet takes a gear trip at its own initiative (weight of a hop), when the learner presses it upwards (`HOLD` and release above a higher card), and when summoned.
- **Phases.** `ready` (0.4 s: the prop comes out of a pocket on the pet), `go` (the traversal clip, loop), `arrive` (hops onto the perch, mood `proud` or `happy` for 10 s), `stow` (0.3 s). A rope, ladder or cord is drawn as 1.5-2 px ink lines in the layer; it vanishes on `stow`. Nothing may cross a keep-out; the learner's pointer never needs to avoid anything.
- **Speeds.** Climbing 14-46 px/s (slower for the heavy, faster for the light); lifts 40-60 px/s; sinking under a canopy 20-45 px/s; a plain fall is the existing one.
- **Dangle.** `HOLD` lifts the pet by a grab point (named per species); it plays `dangle` (a loop on the species' own bones, swinging pendulum-like), mood `scared` for 3 s then `curious`; 3 s later it asks to be put down by `HOLD` release.
- **Dropped.** Release above a perch: below 60 px a plain `fall` and `land`; 60-120 px the descent gear deploys at once; above 120 px the descent gear plus a bigger `land`; species without descent gear get the big `land` and a mood from §5. A pet dropped onto a keep-out walks to the nearest perch.
- **Refusal** is character: a refusing pet that is asked (by hold-release above a card) walks round to a place it can hop, or looks up and sighs (`sulk` 2 s).

---

## S. The twenty species

### S.0 Shared rules for topic mischief (item 6 of every species)

- **What a push is.** A transform on the element `[data-quiz-item=<id>]` of an item listed in the species' `grounds` (or, for a task-level ground, an item of that task whose own label is about the thing). The pet stands on the neighbouring card surface (chips are keep-outs, pets never stand on them) and plays a `push` clip towards the item: lean, kick, blow, shine. The chip moves at most 14 px and 4 degrees over 0.6 s, holds 6 s, then returns by itself, at once on `pointerdown` or focus. It never changes order, layout, answers, tab order or focus, never covers a control, and never targets the chip being dragged or focused. One push per 45 s per stage.
- **No answers leak.** The direction is drawn from the stage seed; it never depends on the item's value or its correct place. A pet that pushes `tea-light` towards the front of "From Tea Light to Sun" would be giving the answer; a pet that pushes it a seed-drawn 12 px sideways is playing.
- **Taken back.** The learner drags or clicks a pushed chip: it springs home and bumps the pet ("thrown out": the pet slides off the card edge, plays `fall` or its descent gear, `land`, takes the mood named in its §6 line, and comes back after 20 s). This covers both readings of the owner's sentence (the element is taken back, and the pet is thrown out).
- **Open product decision (flag for the owner).** Today the layer is `quiet` on the run screen (`quiet = step.screen === "run"` in the quiz pets module), and run screens are the only place with `[data-quiz-item]` chips. Mischief needs either a new preference "pets may play during a run" (default off) or a milder `quiet` that keeps pushes but nothing else (no leaning, no greeting). The `🐕️pet-walk` gate also asserts pets are never the target of a click; the click, circle, stroke, hold and shake gestures change that contract and the gate.

### 1. sunny: Sunny, the sun / Sonne

Floats at 26 px/s, hover 10 px; 46 x 51; temperament energy .70, sociability .85, curiosity .50.
**Rig.** Bones `root > body > rays-long, rays-short` (rest 30 degrees); parts `rays-short`, `rays-long` (accent), `disc` (body, r 14.5), `cheek-left`, `cheek-right` (detail, both on `body`, no bones of their own); eyes and mouth on `body`. Existing clips: `bob` (idle, walk), `ray-wave`, `ray-spin` (fidgets), `beam` (greet), `bask` (cuddle), `flare` (squabble), `dim` (sulk), `doze` (sleep), `touchdown` (land).
**Physics hooks.** 1,361 W/m2 solar constant, 1,000 W/m2 at a clear noon (P2:sunlight-square-metre, the test irradiance of P1:pv-module-peak), 3.8e26 W, 174 PW on the Earth, 592 EJ a year delivered in about an hour, roof skin 60-70 degrees C.

**1 States** (`ladder`: sunset < dim < shining < blazing)

| id (en / de) | Physics | Look in this rig | In / out |
|---|---|---|---|
| `dim` Dimmed / Gedimmt | overcast: 100-300 W/m2 instead of 1,000 | existing `dim` clip as overlay (body y +4, rotation -9, scaleY .94, rays x.8); tint `#d9c47a/#c98c3c/#d8788a` | in: a `heavy` or `raining` cloudy in front (R01), CCW from `shining`, sulk. Out: the cloud leaves (+3 s), CW, L1 hello. Hold 60 s |
| `shining` (default) Strahlend | clear sky | existing `bob`, default palette | the default |
| `blazing` Gleißend | clear noon, full irradiance | +loop `blaze` 0.8s: rays-long.scale 1.2<->1.1, rays-short.rotation +-14, body.y -2; +part `corona` on `body` behind the rays (ellipse r 22, accent, scale pulse .95-1.15); emit `wave` accent x3 rising 18 px every 0.6 s; tint `#ffe45c/#ff7a00/#ff344f` | in: CW from `shining`; `proud` mood after L2; a cheering solary. Out: decays after 30 s (no overtime), a cloud in front, CCW |
| `sunset` Abendrot | low sun, long path through the air, a red 0-200 W/m2 | +loop `low` 3.2s: body.y +7, rays-long scaleY .6 and scaleX 1.15 (a flat fan), rays-short .8, body.rotation +-3; tint `#ff9a3c/#e0482a/#ffd0a0`; mood `sleepy` | in: CCW from `dim`; idle for 3 min. Out: CW or L1 (sunrise), else `doze` after 40 s |

**2 Tricks**
- `corona` Corona / Korona (L2): 1.6s once: rays-long.scale 1->1.35@.25, hold to .6, back@1; +corona.scale .8->1.5@.3->.8; body.y 0->-2@.3->0. Emit 8 `streak` (accent), `burst` radial 14->30 px, life .9 s. Leaves the state unchanged.
- `prominence` Protuberanz / Sonneneruption (L3): 1.4s once: body.rotation -10@.2 -> +6@.5 -> 0; rays-short.rotation +20@.3. Emit 6 `dot` (detail) on an `arc` from the rim up 22 px and back onto the disc, life 1.1 s. Mood `proud`.
- `high-noon` Mittagshöhe (CW, steps up): 1.8s once: existing `ray-spin` (rays +480 / -480), then `blaze` settles in; steps `sunset -> dim` (sunrise, 1.8 s), `dim -> shining` (1.2 s, `bob` blend), `shining -> blazing`. Emit 12 `streak` (accent) burst on the last step.
- `sundown` Sonnenuntergang (CCW, steps down): 2.4s once: body.y 0->+7 (ease out), rays-long.scaleY 1->.6, rays-short fold; emit 5 `ring` (detail, flat, wider than high) spreading 10->26 px along the perch line (horizon glow). Steps `blazing -> shining`, `shining -> dim`, `dim -> sunset`.
- `rainbow` Regenbogen (DUET cloudy `raining`, or AUTO(proud) with one near): 2.6s once: existing `beam`; emit three `arc` bands (detail, accent, body), 36 px wide over the cloud, life 2.2 s. Both pets `happy`. Reason: refraction in the drops.
- Shake outcome: `spark-shower`: 12 `star4` (accent) burst, then `dizzy` 2 s (rays spin out of phase).

**3 Purr** `glow-hum` 2.4s loop: body.scale 1<->1.04 (1.2 s breaths), rays-long.scale .97<->1.06 half a period behind, rays-short.rotation +-5, corona held at .85 ("a halo that breathes"); (N) the two cheeks as own bones pulse 1->1.3. Emit 1 `dot` (detail) rising 14 px every 0.9 s.

**4 Moods** lean: `proud` (vain), `happy`, `playful`; baseline `content`; sulks fast and forgives fast (grumpy decay x2).

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | no hardware: it rises along a quarter circle to above the edge at 40 px/s (`bob` with y -> perch + hover), rays-long wave; no prop |
| Down | `sink`: the rays fold (rays-long .6), the disc sinks at 40 px/s like a balloon and turns `sunset` on the way down; on landing a `touchdown` with a corona flash is the "sunrise" |
| Dangle | grab at the tip of one long ray: body pendulum rotation +-14 over 1.1 s, rays-short .85, cheeks flushed (mouth -.4) |
| Dropped | above 60 px it sinks; mood `happy` ("again!"), `sad` if it was `dim` |
| Refuses | ladder (nothing to hold on a ball of plasma), grappling gun (the line burns through in a `puff` at 4 px), climbing, parachute (it is its own balloon) |

**6 Topic mischief** Items: P2 `sun`, `sunlight-on-earth`, `sunlight-square-metre`; P3 `world-primary-energy` ("the Sun delivers this much in about an hour"); P1 `pv-module-peak`; C2 `attic-flat`, `office-1970s` (solar gains). Style: **shines** on it. A soft spot (accent radial, 1.2 s) lands on the chip, the chip warms (+3 px lift, scale 1.03) and slides a seed-drawn 14 px: sunlight really does push (radiation pressure, the principle of a solar sail). Sunny plays `beam`, then `bask`. When the learner takes the chip back it springs home, sunny `dim`s for 4 s (mood `sad`), then tries a different item once a minute.

### 2. cloudy: Cloudy, the cloud / Wolke

Floats at 16 px/s, hover 14 px; 58 x 43; energy .35, sociability .60, curiosity .70.
**Rig.** Bones `root > body > wisp, puff-left, puff-middle, puff-right`, and `drop` (on `root`, parked behind the belly); parts `drop` (accent), outlined and filled puffs plus `belly`, `shine` (detail); eyes and mouth on `body`. Existing clips: `drift` (idle), `drip`, `billow` (fidgets), `puff-wave` (greet), `snuggle`, `bluster`, `drizzle` (sulk), `doze`, `touchdown`.
**Physics hooks.** A cloud is named by no item; it is implied by the clear-sky square metre (P2), the weather-averaged 950 kWh/kWp (P1:pv-annual-yield), cooling loads on a "hot design day" (C2) and the steam-looks-like-a-cloud bond with kettly.

**1 States** (`ladder`: wispy < fluffy < heavy < raining)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `wispy` Zart / Schleierwolke | thin high cirrus, lets ~90 % of the light through | +loop `thin` 3.6s: puffs .85, body.scaleY .8, wisp.scale 1.3 and x +-3; tint `#9fc0c6/#8fe3d8/#eef6f0` | in: CCW from `fluffy`; evaporation. Out: CW |
| `fluffy` (default) Schönwetterwolke | cumulus, fair weather | existing `drift` | default |
| `heavy` Schwer | nimbostratus, lets ~10 % through | +loop `sag` 3.2s: body.y +3, body.scaleY 1.12, puffs 1.1; the `drop` hangs as a bead at the belly (drop.y +8, scale .5); tint `#46606a/#2fa89a/#8ea9a8` | in: CW; trick `thunder`; a rule (R05). Out: CCW; after 40 s it rains |
| `raining` Regnend | precipitation | `heavy` plus emit 3 `drop` (accent) per second over the belly width, `fall` to the perch with a flat `ring` splash; +loop `wring` 1.2s on puff-middle (scale .9/1.08); tint `#3b5560/#2bc2b0/#9fb9b6` | in: L2 rain, CW from `heavy`. Out: holds 12 s, then `heavy`, then `fluffy` |

**2 Tricks**
- `rain` Regen (L2): 3.0s once: puffs squeeze to .8, belly sags, `drop` bone swells and falls; emit 14 `drop` (accent) from the belly width +-14 px at 60 px/s with splash rings; leaves `raining` 5 s then `fluffy`.
- `thunder` Gewitter (L3): 2.0s once: body darkens by the `heavy` tint for 0.2 s, flashes twice; emit 1 `zigzag` (paper) 14 px from the belly at @.3 and @.55, life .15 s; existing `bluster` shake. Leaves `heavy`; scares flamy, windowy and thermy within 90 px (mood `scared`, 3 s).
- `condense` Kondensieren (CW, steps up): 1.4s once per step: the `billow` roll; emit 6 `dot` (paper) rising from the perch and gathering into the body (vapour in). Steps `wispy -> fluffy -> heavy -> raining`.
- `evaporate` Verdunsten (CCW, steps down): the same clip reversed: puffs shrink and 6 `dot` (paper) leave upwards. Steps `raining -> heavy -> fluffy -> wispy`.
- `snow` Schneefall (AUTO(playful) near a `cooling` or `frosted` chilly, or DUET chilly): 3.0s once: puffs pale to the `wispy` tint, existing `drizzle`; emit 10 `flake6` (paper) falling 24 px/s with a sideways drift. The perch gets a dusting (a flat paper strip, 1 px, fades in 8 s). Chilly `happy`.
- `gust` Windstoß (DUET windy): 1.8s once: cheeks puff (puff-left, puff-right scale 1.2), then existing `bluster`; emit 6 `streak` (paper) towards windy. Windy goes `rated` (R10).
- Shake outcome: `downpour`: 20 `drop` (accent) in 1.5 s from the belly, the puffs shrink to `wispy` size, mood `sad`, then `fluffy` after 6 s.

**3 Purr** `soft-patter` 2.8s loop: puffs breathe 1.0<->1.06 with phase lag .2, body.y +-.6; emit a single `drop` (accent) that falls from the wisp every 1.2 s and **fades before it reaches the perch** (virga, rain that evaporates in the air).

**4 Moods** lean: `curious`, `playful` (cheeky: it drifts in front of sunny "by accident"), `sleepy`; baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | thermal: puffs inflate in sequence, body rises 36 px/s on a gentle S-curve (a warm updraught, `condense`-style `dot` rising under it) |
| Down | `fog-sink`: it flattens (body.scaleY .85, wisp trailing) and sinks 30 px/s as drizzle: 3 `drop` fall faster than it does |
| Dangle | held by the `wisp` (a thread of mist): the belly sags (scaleY 1.2), puffs droop, 2 `drop` drip; pendulum +-8 degrees |
| Dropped | above 60 px the fog-sink; it lands with `touchdown` and `puff-wave`; mood `playful` (nothing hurts a cloud) |
| Refuses | ladder, grapple (the hook goes through the mist), suction, climbing: no hands, no weight |

**6 Topic mischief** Items: P2 `sunlight-square-metre` ("clear summer day"), P1 `pv-annual-yield` (weather-averaged), C2 `school-new-build` ("hot design day", the summer holidays), `attic-flat`. Style: **blows** (cheeks puffed, 6 `streak` (paper)); the chip flutters 4 degrees and slides a seed-drawn 12-20 px; or a single `drop` splashes at its edge so the chip droops 3 degrees for 3 s ("wet"). Taken back: shrugs, `drizzle` sulk 4 s, tries from the other side.

### 3. housy: Housy, the house / Haus

Walks at 26 px/s; 48 x 52; energy .35, sociability .85, curiosity .45.
**Rig.** Bones `root` (legs `leg-left`, `leg-right` hang on `root`), `body` (with `arm-left`, `arm-right`, `door`, `roof`), `roof > chimney > puff`; parts `puff` (paper), legs and arms (ink lines), `chimney` (detail), `wall` (body), `doorway` (paper), `door` (detail), `knob`, `roof` (accent); eyes under the eaves on `body`, mouth at the door. Existing clips: `breathe`, `waddle`, `smoke-ring`, `shiver` (fidgets), `welcome` (greet), `shelter` (cuddle), `rattle`, `mope`, `doze`, `touchdown`.
**Physics hooks.** Design heating load 18 kW (nine kettles) for 110 m2; annual demand 33 MWh, 303 kWh/(m2·a); energy-certificate class H above 250 kWh/(m2·a); 10-160 W/m2 across H2; houses "breathe" through ventilation; solar gains help in winter and hurt in summer.

**1 States** (`ladder`: cold < cosy < wrapped)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `cold` Ausgekühlt | heat leaks out: an unrenovated old house on a design day | +loop `chill` 1.9s: the existing `shiver` blend (door.scaleX flaps, body.x +-.6, arms hugging), breath `puff` from the door every 2 s; +part `icicle` x2 under the eave (paper, small paths on `roof`); tint `#a88f73/#8c2a30/#bcc6c9` | in: CCW from `cosy`; trick `heat-loss`; windowy `open` near (R26). Out: a warm radiatory near (R30), CW |
| `cosy` (default) Behaglich | design indoor 20 degrees C reached | existing `breathe` | default |
| `wrapped` Gedämmt | insulation coat: load falls from 160 to 20 W/m2 | +prop `coat` (rect 40 x 28 on `body`, kit paint `wool` with `wool-light` batting stripes; windows and door cut out); calm `breathe` at tempo x.8; the chimney smokes less (puff scale .5) | in: CW from `cosy` (pulls on a coat) or an insuly tuck-in near (R41). Out: CCW (air-out strips it) |
| `hot` Überhitzt | summer: no shading, solar gains | +loop `fan` 1.6s: arms rotating as fanning (arm-left/right -> +-35), door.scaleX .5, 2 `drop` (accent) sweat; tint `#d99a55/#c40010/#f0d4b4` | in: sunny `blazing` near in the cooling scene (R36). Out: shady near, or 30 s |

**2 Tricks**
- `energy-label` Energieausweis (L2): 2.4s once: the door swings open and a label strip slides out (+prop `label`: 8 stacked bars on `door`, each 2 px high and growing from 6 to 13 px in `accent`, like the arrows of a real energy certificate; the bar of its class turns `detail` and an ink arrow points at it: `wrapped` near the top, `cosy` in the middle, `cold` and `hot` at the bottom). No digits, no letters, and no green-to-red ramp (the palette has no such colours).
- `smoke-rings` Rauchringe (L3): 2.0s once: existing `smoke-ring` (chimney stretches) three times; emit 3 `ring` (paper) rising 28 px and widening; the colour follows the state (`cold`: ring tokens `ink`, more smoke; `wrapped`: one faint ring).
- `pull-on-coat` Mantel anziehen (CW): 1.8s once: arms up, the `coat` slides in from the right in three tugs; steps `cold -> cosy` (shiver stops, 1 s), `cosy -> wrapped`.
- `air-out` Stoßlüften (CCW): 1.6s once: door.scaleX 1->.2, windows open, a gust of 6 `streak` (paper) leaves through the door (the heat goes); steps `wrapped -> cosy` (coat drops off), `cosy -> cold`.
- `heat-loss` Wärmeverlust (AUTO(sad)): 2.2s once: the roof lifts 8 px and tilts 15 degrees; emit 5 `wave` (accent) rising out of the top and fading; leaves `cold`.
- Shake outcome: `rattle-roof`: the roof jumps off by 10 px and is caught, 5 `chip` (accent) of dust, `scared`.

**3 Purr** `chimney-content` 3.0s loop: body.scaleY 1<->1.03, roof.rotation +-1.5, door ajar 10 %, lids .25; emit a small `ring` (paper) from the chimney every 1.5 s, and one warm `dot` (accent) from the door.

**4 Moods** lean: `content` (host), `sad` (worrier about draughts), `happy`; proud when `wrapped`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `chimney-cannon`: the chimney tips forward 40 degrees, a "whoomp" of 3 `puff` (paper), a hook (accent triangle, 4 px) flies on a 1 px ink line to the card edge; a winch on the roof (`chimney` rotation) reels the house up at 30 px/s, legs dangling, door flapping |
| Alt | `roof-ladder`: a ladder (two ink rails, rungs every 6 px) lowered from the right arm; for rises below 120 px |
| Down | `downpipe`: a zinc pipe (rect 4 x 40, `detail`) unrolls from the eave; it slides down it with both arms around it, a splash `drop` at the bottom |
| Dangle | grab at the chimney tip: pendulum +-10 degrees, legs +-14, `door` flaps, roof tilts; mouth -.4 |
| Dropped | above 60 px existing `rattle`, roof falls like a hat and is caught, big `touchdown` with 4 `dot` (ink) dust; mood `scared` then `sad`, unless it lands beside an insuly (then `happy`) |
| Refuses | parachute (bricks do not float; the roof is nailed to the walls), suction cups (a house has no flat palms) |

**6 Topic mischief** Items: all houses in H2 (`plattenbau`, `gruenderzeit`, `sfh-1960s`, ...), D1 and D2 houses (`unrenovated-old-building`, `old-house-gas`, ...), P1 `heating-load`, `heating-demand-house`, `energy-certificate`, P2 `heating-load-old-house`, P3 `heating-demand-house`. Style: **shoulders** the chip with its door side: a waddle of two steps, a lean, the chip slides a seed-drawn 14 px (the owner's example: a question about houses is pushed out of the stack). Taken back: the chip bumps the house off the card; it falls, lands (`rattle`), `mope`s 5 s, and later knocks politely (a door-knock `ring` pair) before trying the next house.

### 4. solary: Solary, the solar panel / Solarmodul

Walks at 30 px/s; 46 x 52; energy .70, sociability .75, curiosity .60.
**Rig.** Bones `root > body > leg-left, leg-right, panel (rest -5 degrees)`, `panel > arm-left, arm-right, sparkle, glint`; parts legs (ink), `hips` (detail), arms, `sparkle` (accent star path), `frame` (detail), `glass` (body), `mullion-upper/lower` and `rail-upper/lower` (detail lines: the 2 x 3 cell grid), `glint-long`, `glint-short` (accent); eyes and mouth on `panel`. Existing clips: `bask` (idle), `stroll`, `sunbathe`, `gleam` (fidgets), `wave`, `snuggle`, `bicker`, `dim` (sulk), `doze`, `touchdown`.
**Physics hooks.** 0.43 kWp for a 2 m2 module at 1,000 W/m2 and 25 degrees C (IEC 60904-3); 20 % efficiency (200 W of the kilowatt); 950 kWh/kWp a year; a plus-energy roof exports; a shadow on a cell drops output; hot modules lose about 0.35 % per kelvin.

**1 States** (`ladder`: shaded < generating < peak)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `shaded` Verschattet | no direct light, a cloud or a blind | existing `dim` as overlay (panel.scaleY .94, panel.rotation 8, arms droop); tint `#23467f/#8fb1a0/#9a9a90` | in: heavy or raining cloudy over sunny (R03), shady near (R38), CCW. Out: the shade leaves, CW |
| `generating` (default) Erzeugt Strom | normal light | existing `bask`; the glint drifts across slowly | default |
| `peak` Spitzenleistung | 1,000 W/m2: the standard test condition | +loop `peak` 1.6s: panel.scale 1.04, +parts `cell-row-1..3` (on bones `cell-row-1..3` of `panel`: each one path of two rects 9 x 13 over the glass, accent, brightness pulse from top to bottom via scale .9<->1.0), sparkle scale 1.3; tint `#3b78d8/#e8f7ee/#fff5b0` | in: a `blazing` sunny near (R02), CW. Out: after 40 s it heats to `hot`; sunny leaves |
| `hot` Modul heiß | the cell temperature climbs, output falls | `peak` slowed to x.6, arms limp, 2 `drop` (accent) sweat; tint `#5a5fb0/#e8eef5/#e08a50` | in: `peak` held for 40 s (R06). Out: shade, a cloud, or 20 s |

**2 Tricks**
- `sun-track` Der Sonne nachdrehen (L2): 2.4s once: existing `sunbathe` (tilt 17 degrees towards sunny, or towards the top right of the screen when none) then `gleam`; emit 5 `star4` (accent) from `sparkle`. Leaves the state.
- `cell-wave` Zellenwelle (L3): 2.0s once: the three `cell-row`s light one after another from top to bottom (0.4 s apart) and back, the glint sweeps; leaves `peak` when sunny `shining` or `blazing` is near, else `generating`.
- `power-up` Hochfahren (CW, steps up): 1.6s once: the panel stands tall (rotation 0, arms up), cells fill; steps `shaded -> generating -> peak`.
- `duck` Wegducken (CCW, steps down): 1.4s once: the panel folds forward 12 degrees and the glint goes out; steps `peak -> generating -> shaded`.
- `feed-battery` Akku füttern (DUET battery, or AUTO(happy) near it): 2.0s once: existing `wave`, then emit 5 `dot` (accent) on an `arc` into the battery's bars (+1 bar). Both `proud`.
- Shake outcome: `sparkle-shower` (8 `star4`), then `shaded` 3 s.

**3 Purr** `inverter-hum` 1.6s loop: panel.rotation +-1.5, the glint slides across the glass diagonally in 1.2 s, cell brightness pulses in two phases like a chess board; emit one `star4` (accent) every 1.0 s at the sparkle.

**4 Moods** lean: `happy`, `sad` (shy in the dark; sulks under a cloud), `curious`; baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `rail-hooks`: two roof hooks (accent L-paths, 6 x 8 px, "Dachhaken") on the arms are hung on the card edge; she swings and pulls the panel up hand over hand (arm rotations -120 -> -20), legs dangling, 24 px/s |
| Alt | leaning on a friend's ladder |
| Down | `wing-glide`: the panel tilts to ~70 degrees and she holds the frame corners like a hang glider; sinks 35 px/s with a lateral drift +-40 px; the sparkle winks |
| Dangle | grab at the top frame corner: pendulum +-16 degrees, the glint flashes at every swing, cells flicker |
| Dropped | above 60 px wing-glide at once; below it `shaded` for 3 s; mood `scared` then `happy` |
| Refuses | grappling gun ("no holes may be drilled in glass") |

**6 Topic mischief** Items: P1 `pv-module-peak`, `pv-annual-yield`, P2 `sunlight-square-metre`, D1 `plus-energy-house`, D2 `kfw-55-gas-solar`. Style: **mirrors**: she tilts and bounces a glint (a `star4` line) onto the chip, then shoulders it with her frame; slides a seed-drawn 12 px. Taken back: the panel goes flat (`shaded` mope, 4 s), then she waves.

### 5. radiatory: Radiatory, the radiator / Heizkörper

Walks at 26 px/s; 54 x 55; energy .35, sociability .85, curiosity .40.
**Rig.** Bones `root` (legs), `body` > `arm-left/right` (rest -25 / +25), `fin-1..5` (x -15.2 .. 15.2), `knob`, `shimmer-1..3`; parts legs and arms (ink lines), `valve` (line on `knob`), fin edges and fins (body), two `eye-socket` ellipses (body, no stroke), `knob` (detail), `knob-mark`, three `shimmer` strokes (accent); eyes and a 5.5 px mouth on the middle fin. Existing clips: `simmer` (idle), `shuffle`, `ripple`, `thermostat` (fidgets), `wave`, `hug`, `huff`, `cool-off`, `doze`, `thud`, `leap`, `tumble`.
**Physics hooks.** 10-160 W/m2 across H2; 18 kW for the 1960s house; 20 degrees C design inside and 32 K at -12 degrees C; low flow temperatures with a heat pump versus boiler flow temperatures; a passive-house window (0.8) makes the radiator redundant, single glazing (5.8) makes it work overtime; air in the system leaves the top cold.

**1 States** (`ladder`: cold < warm < hot)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `cold` Kalt | valve closed or no heat arriving | shimmers parked (scaleY .1), body.scaleY .97, arms down, knob turned to the left stop; tint `#6f8fb5/#6aa0c9/#e8eef5` | in: CCW from `warm`; a `cold` window near. Out: CW, a pumpy `heating` near with flow temperature (R48) |
| `warm` (default) Warm | design condition | existing `simmer` | default |
| `hot` Heiß | thermostat on 5, high flow temperature | +loop `glow` 1.2s: shimmer-1..3 scaleY 1.7 and x wobble +-2, fins y +-.6 in a wave, body.scale 1.02; emit `wave` (accent) x3 rising 20 px; tint `#ff1f3d/#ffb000/#fff3d0` | in: CW; trick `glow-wave`. Out: holds 40 s then `warm`; an open window near makes it huff (R21) |

**2 Tricks**
- `bleed` Entlüften (L2): 2.4s once: the right arm turns the vent screw on `knob` (rotation 90), the top fin hisses; emit 3 `dot` (paper) bubbles rising from the top fin and a `puff` (paper) hiss; the shimmer flares, then steadies; mood `happy` (relief). Leaves `warm`.
- `glow-wave` Wärmewelle (L3): 2.0s once: the existing `ripple` through five fins and back, shimmer flaring as it passes; emit 4 `wave`; leaves `hot` 6 s.
- `turn-up` Aufdrehen (CW, steps up): 2.4s once: existing `thermostat` (arm up, knob +120, shimmer and body swell); steps `cold -> warm -> hot`.
- `turn-down` Zudrehen (CCW, steps down): the same clip reversed (knob -120, shimmer fades); steps `hot -> warm -> cold`.
- `towel-dry` Handtuch wärmen (AUTO(content)): 3.0s once: a towel (+prop `towel`, rect 24 x 8 `paper` with ink stripe, hung on both arms) lies over its top; emit 4 `puff` (paper) steam; it purrs under it. A radiator's other job (towel radiator) and a ladder-like silhouette.
- Shake outcome: `gurgle`: 6 `dot` (paper) bubbles, mood `grumpy`.

**3 Purr** `convection-hum` 2.6s loop: fins y +-.8 in a travelling wave (lag .2), shimmers scaleY .6->1 in three phases, body.scale 1.0<->1.015, knob jiggles +-3; emit a warm `dot` (accent) from the top every 0.8 s, rising 12 px.

**4 Moods** lean: `content`, `sleepy`, `grumpy` (huffy at open windows and at chilly), `happy` with pumpy; `sad` when `cold`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `iron-ladder`: a slender cast-iron ladder (two ink rails, rungs every 6 px) swings out from its back; stiff rung-by-rung climb at 12 px/s, arms alternating; each rung a small `thud` squash, the ladder bends 2 degrees |
| Down | slides down the same ladder, arms around the rails (the fireman) |
| Dangle | grab at the `knob` (top right): hangs tilted 25 degrees, legs swing, the shimmers curl **upwards** against gravity (hot air rises), the valve turns |
| Dropped | big `thud`: all five fins ring (y +-2), shimmer snuffed; mood `grumpy`, a dent `star4` pair |
| Refuses | grappling gun (a cast-iron section weighs ~14 kg and would pull the card down), parachute (no canopy would carry it) |

**6 Topic mischief** Items: H1 `window-passive-house` ("no radiator is needed below the window"), `single-glazing`; H2 items (loads); P1 `heating-load`; P2 `heating-load-old-house`; D2 items. Style: **warms and shoulders**. The warmed chip lifts 3 px on a `wave` and slides 12 px. Character gag: it is jealous of `window-passive-house` and pushes it **away** (grumpy, huff) but hugs `single-glazing` (work for the radiator); the direction rule of S.0 still holds because it is the *pose*, not the direction. Taken back: huff and steam; if it was `window-passive-house` the mood is `sad` ("redundant").

### 6. pumpy: Pumpy, the heat pump / Wärmepumpe

Walks (trots) at 34 px/s; 46 x 46; energy .55, sociability .30, curiosity .45. No arms.
**Rig.** Bones `root` (four legs `leg-back-far`, `leg-back-near`, `leg-front-far`, `leg-front-near` hang on `root`), `body` > `tail`, `intake`, `outlet`, `fan`; parts `intake` (accent stroke), `outlet` (detail stroke), legs (ink), `tail` (ink), `tail-pipe` (detail), `body` (rect r 7), `louvres`, `well` (accent), `blades` (paper), `hub` (detail); eyes and mouth on `body`. Existing clips: `hum` (idle), `trot`, `rev`, `exchange` (fidgets), `nod` (greet), `purr` (cuddle), `bluster`, `standby` (sulk), `sleep-mode`, `clunk`, `leap`, `tumble`.
**Physics hooks.** Seasonal performance factor 2.7-3.5: 3 kWh of heat per kWh bought, more than two thirds from the environment; the ground is warmer than winter air; a heat pump is a chiller run backwards; it serves low flow temperatures; defrost cycles in cold air; GEG 2024; the compact unit with ventilation in the passive house.

**1 States** (`ladder`: standby < heating; `cooling` and `defrost` come from tricks and rules)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `standby` Bereitschaft | compressor off | existing `standby` as overlay: fan stopped, body.scaleY .97, louvres shut; tint `#4a7f78/#5c9c96/#c46a1a` | in: CCW from `heating`; a long idle. Out: CW, L1 |
| `heating` (default) Heizen | the normal winter job | existing `hum`; warm streaks (detail) leave at the outlet | default |
| `cooling` Kühlen | the cycle reversed: it heats the outdoors | +loop `reverse` 3.2s: fan rotation 0 -> -360 (turns the other way), intake and outlet strokes swap paint (accent out, detail in); tint `#2a8fb5/#7ee6f5/#4bc8e8` | in: trick `reverse-cycle`; a `cooling` chilly near (R52); the cooling scene. Out: the same trick; 40 s |
| `defrost` Abtauen | frost on the evaporator melts by running the cycle backwards for a moment | +loop `defrost` 2.4s: fan slow reverse; +part `frost` (a paper path across the louvres); emit 4 `puff` (paper) rising every 0.8 s; tint `#7fb8b5/#e8f7f5/#fa9500` | in: AUTO(proud) every ~3 min after 60 s of `heating`, the heating scene; trick `defrost-shake`. Out: 8 s |

**2 Tricks**
- `three-for-one` Eins zu drei (L2): 2.8s once: existing `exchange` with a counted flow; emit 1 `zigzag` (detail) entering at the tail (electricity), 2 `streak` (accent) entering at the intake (environment), then 3 `streak` (detail) leaving at the outlet; body.scale 1.04@.5. Leaves `heating`. Seasonal performance factor 3, drawn.
- `reverse-cycle` Kreislauf umkehren (L3): 1.8s once: fan stops, restarts the other way (rotation 0 -> -720), intake and outlet swap paint for the duration; toggles `heating` and `cooling`; mood `proud` ("a heat pump is a chiller run backwards").
- `ramp-up` Hochmodulieren (CW): `standby -> heating`: existing `rev` (fan 1920, body vibrates, the proud hop); a flourish at `heating`.
- `whisper` Flüstermodus (CCW): `heating -> standby`: fan slows over 1.2 s, lids .3, emit 2 `ring` (paper) shrinking 14 -> 4 px (sound dampening, drawn).
- `flow-temperature` Vorlauftemperatur (DUET radiatory): 2.6s once: the `tail-pipe` extends towards radiatory's valve (tail.scale 1.6), emit 6 `dot` (detail) sweeping along a line across the gap; radiatory `warm -> hot`; both `happy`, pumpy `proud`.
- `defrost-shake` Abtauen (AUTO(proud), and the entry into `defrost`): 2.4s once: shakes like a dog (body.rotation +-6 at 6 Hz), fan reversed; emit 6 `chip` (paper) flying sideways and 4 `puff` rising.
- Shake outcome: `rattle`: fan 1080 degrees, a short `dizzy`.

**3 Purr** `compressor-purr` 1.2s loop: body.x +-.5 at 8 Hz, fan 0 -> 360 per loop, tail +-8, louvres micro-shift; emit a `ring` (detail) from the outlet every 0.6 s, 6 -> 14 px.

**4 Moods** lean: `content`, `proud`, `grumpy` (an introvert; boily sets it off); baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `pipe-vault`: the tail pipe stiffens (compressor pressure; `tail-pipe` straight rod 22 px, tail.rotation -90), plants at the foot of the card edge, pole-vaults: body.rotation -70 over the edge, 1.2 s, up to 110 px |
| Alt | `rev-leap`: existing `leap` with the fan at 1920 degrees, for 72 px and below |
| Down | `fan-brake`: the fan spins backwards at full speed, 4 `streak` (paper) under the body; sinks 45 px/s with legs tucked |
| Dangle | grab at the `outlet` (the carry handle): hangs level, four legs paddling at 2 Hz, fan idling, tail +-20 |
| Dropped | above 60 px fan-brake at once; lands with `clunk`; mood `proud` (it worked) |
| Refuses | ladders (four legs: a quadruped cannot climb rungs), suction cups (no hands), grappling gun (the tail pipes are refrigerant lines, a rope would kink and leak) |

**6 Topic mischief** Items: D2 `kfw-40-heat-pump`, `deep-retrofit-heat-pump`, `old-house-heat-pump`; D1 `kfw-40`, `passive-house`, `plus-energy-house`; H2 `geg-2024`. Style: **blows** a warm column from the outlet (3 `streak` (detail)): the chip slides a seed-drawn 14 px, "moving heat from here to there". (N) a rival tease: with grumpy boily on stage it may blow boily's `old-house-gas` item (outside its own grounds, so only as a duet). Taken back: `bluster`, then a `nod`.

### 7. windowy: Windowy, the window / Fenster

Hops at 40 px/s; 40 x 52; energy .65, sociability .80, curiosity .90.
**Rig.** Bones `root` (legs), `body` > `sash-left` (x -13), `sash-right` (x 13) > `glint`; parts legs, `frame` (body), `opening-left/right` (ink), `sash-left/right` (accent), `transom-left/right` (body lines), `glint-long/short` (detail), `socket-left/right` (body ellipses behind the eyes), `sill` (detail); eyes at y -18.5, mouth at y -8.5, both on `body`. Existing clips: `breathe`, `hop` (walk and hop), `air`, `gleam` (fidgets), `flap` (greet), `open-up` (cuddle), `rattle`, `shut` (sulk), `doze`, `clatter` (land).
**Physics hooks.** U-values 5.8 (single), 4.3 (aluminium), 2.7 (box-type), 1.3 (GEG), 0.8 (passive house), 0.5 (triple glass alone); window ventilation struggles at 4.8 1/h (classroom); unshaded glazing is the chiller's biggest load; roof windows leak heat and gain sun; larger glazing raises the plus-energy house's cooling demand; winter solar gains help.

**1 States** (`ladder`: shut < tilted < open; `fogged` from a rule or a trick)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `shut` (default) Geschlossen | closed, tight | existing `breathe` | default |
| `tilted` Gekippt | tilt position: a slot of air at the top | +loop `tilt` 2.4s: sash-left.scaleX .9 and rotation +4, a dark 2 px strip along the top of the opening, body.rotation 2; emit 3 `streak` (paper) slipping in at the top every 1.2 s | in: L2, CW. Out: CCW, 25 s |
| `open` Offen | fully open: the air change is big and so is the loss | +loop `wide` 2.0s: both sashes scaleX .12 (swung edge-on), the openings dark (ink), body.x +-.8 (a draught sway); emit 5 `streak` (paper) crossing every 0.8 s | in: CW from `tilted`. Out: CCW, 20 s |
| `fogged` Beschlagen | warm moist air on cold glass | tint `#7b827d/#e8f1f3/#f7f3e3` (the sashes whiten), emit 2 `drop` (accent) running down the pane every 2 s | in: `fog-draw`; a `cold` radiatory beneath (R23). Out: L1 wipes it (a flap), 12 s |

**2 Tricks**
- `crack-open` Kippen (L2): existing `air` 2.4 s: sash-right.scaleX 1 -> .25@.3, a shiver (body.x +-.8), back; emit 4 `streak` (paper) entering. Leaves `tilted`, 25 s.
- `fog-draw` Beschlag-Zeichnung (L3): 3.0s once: it breathes on the pane (body.scaleY 1.04, mouth round), the `fogged` tint comes in; then +part `doodle` (an ink 1.5 px smile path on `sash-left`, scale 0 -> 1 in 0.8 s) is drawn; both fade after 2 s. Leaves `shut`.
- `swing-wide` Weit öffnen (CW, steps up): 1.6s once: `air` with overshoot (sash scaleX 1 -> .12 with a flap on the end); steps `shut -> tilted -> open`.
- `close-up` Zuschlagen (CCW, steps down): 1.2s once: sashes snap back with a `clatter`; emit 3 `star4` (paper) at the frame corners; steps `open -> tilted -> shut`.
- `glaze-show` Verglasungs-Show (AUTO(proud), DUET waly): 3.0s once: one, two, three hairlines (ink, 1 px) appear in the pane while the leak `streak`s on the left side thin out from 6 to 2 to 1 (5.8, 1.3, 0.8 W/(m2·K)), no digits. Leaves `shut`; `proud`.
- Shake outcome: `rattle` plus a glint flash (6 `star4`).

**3 Purr** `glass-hum` 2.2s loop: sashes scaleX 1 <-> .96 alternating, the glint slides across once per loop, body.scaleY 1 <-> 1.02; emit 1 `star4` (detail) at the glint every 1.5 s.

**4 Moods** lean: `curious`, `playful` (chatty, a show-off: it cannot hide its feelings), `scared` (draughts); baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `suction-cups`: two cups (accent circles r 3 with an ink rim) at the sash tips; it climbs the card side like a window cleaner, left cup pops with a `ring`, then the right; 0.4 s per step, 20 px/s, body tilted 4 degrees |
| Down | `sash-glide`: both sashes swing out to +-70 degrees as wings; 40 px/s with a body roll of +-12 |
| Dangle | grab at the top of one sash: hangs diagonal, the other sash flaps (scaleX 1 <-> .2 at 2 Hz), legs paddle, the glint flashes; mouth round |
| Dropped | above 50 px the sashes snap open by themselves; below it `clatter`; mood `scared`, then `curious` |
| Refuses | grappling gun (a glass latch cannot take a point load and would shatter), ladders (rungs scratch the pane) |

**6 Topic mischief** Items: H1 `single-glazing`, `aluminium-window-1970s`, `box-type-window`, `window-geg`, `window-passive-house`, `triple-glazing`; C1 `classroom`; C2 `attic-flat`, `office-1970s`; D1 `plus-energy-house`. Style: **swings a sash** like a door onto the chip and pushes it 16 px; for `classroom` it opens both sashes wide and waves them, to no effect (window ventilation struggles at 4.8 1/h). Taken back: slams (`clatter`), `shut` mood sulk for 5 s.

### 8. waly: Waly, the wall / Wand

Walks at 20 px/s; 48 x 46; energy .30, sociability .40, curiosity .25.
**Rig.** Bones `root` (legs), `body` > `brick` (x 13.5, y -29.25, at the top right), `arm-left` (rest -60) > `forearm-left`, `arm-right` (rest 60) > `forearm-right`; parts `brick` (body), legs, `wall` (body), `mortar` (accent path), `shade` (detail line), `cap` (accent), arm lines; the face is drawn above `cap`. Existing clips: `stand`, `march`, `loose-brick`, `knock` (fidgets), `salute` (greet), `lean-on` (cuddle), `stomp`, `stonewall` (sulk), `doze`, `thud`.
**Physics hooks.** U 1.5 (half-timbered), 1.0 (perforated brick), 0.6 (WSchVO 1982), 0.28 (GEG, about 12 cm), 0.15 (passive, about 24 cm); thermal mass; Plattenbau sandwich panels; Gründerzeit brick with party walls; windows leak 4.6 times more (1.3 against 0.28).

**1 States** (`ladder`: bare < warm; `wrapped` and `passive` only through insuly, R42)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `bare` (default) Ungedämmt | a wall without a layer, U about 1.0 | existing `stand` | default |
| `warm` Aufgewärmt | heat stored in the mass, released over hours | tint `#bd5a3a/#e8cba8/#6a5a56`, `stand` at tempo x.6, emit 1 `wave` (accent) rising 10 px every 2 s | in: CW; a hot radiatory near for 20 s (R46); sunny near. Out: CCW; it releases over 90 s (thermal inertia) |
| `wrapped` Gedämmt | 12 cm of insulation, U 0.28 | +prop `coat` (rect 40 x 30 over the wall below the cap, kit paint `wool` with `wool-light` batting stripes), `stand` at tempo x.8; the mortar is hidden | in: an insuly tuck-in near (R42). Out: hold 120 s after insuly leaves |
| `passive` Passivhaus-gedämmt | 24 cm, U 0.15 | `coat` doubled (a second layer, 4 px wider each side), body.scaleX 1.08, tempo x.5 | in: a second tuck-in by a `thick` insuly while `wrapped`. Out: 120 s |

**2 Tricks**
- `brick-swap` Stein zurechtrücken (L2): existing `loose-brick` 2.0 s: the brick pops out (+5 px), the arm pushes it back; emit 3 `chip` (accent) of dust; `proud`.
- `layer-reveal` Schichtenschnitt (L3): 2.4s once: the front opens to a cut-away; +prop `section` (an inset 14 x 20 on `body`, stacked stripes: plaster `accent`, brick `body`, and as many pink insulation stripes as the state has: none, one, two); `forearm-right` points at it, 1.2 s hold, then closes. The one trick that shows a number (layer thickness) as a picture.
- `warm-up` Aufheizen (CW): 2.0s once: leans towards the nearest heat source (sunny, radiatory) or the screen top, arms spread; emit 3 `wave` (accent); step `bare -> warm`.
- `cool-down` Auskühlen (CCW): 2.0s once: shivers, folds its arms, emit 3 `wave` (accent) leaving upwards; step `warm -> bare`. When `wrapped` or `passive`, a CCW turn is a `salute` and nothing else.
- `u-value-duel` U-Wert-Duell (DUET windowy): 3.0s once: two `meter` bars (+prop `meter`, rect 4 x 20) appear above the pair, windowy's 4.6 times as long as waly's; waly `stonewall`s smugly, windowy rattles. Both `grumpy` 15 s; bond -.05.
- Shake outcome: `crumble`: 4 `chip` (body) fall, the brick drops out; `grumpy`.

**3 Purr** `thermal-mass-hum` 4.0s loop: body.scaleY 1 <-> 1.02, lids .3, the whole drawing 6 % lighter at the peak (tint ramp); emit 1 slow `dot` (accent) rising from the cap every 2 s.

**4 Moods** lean: `grumpy` (stubborn), `proud`, `content`; rarely `happy`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `scaffold-ladder`: both arms set a scaffold ladder (two ink poles, `detail` boards) against the card edge; climbs at 8 px/s, every rung a `thud` and 2 `chip`. For rises up to 36 px: `brick-stairs`, the loose brick laid as a step |
| Down | backs down the same ladder |
| Dangle | grab at the `cap`: hangs rigid, the arms keep the `stand` pose, the loose brick wiggles out and falls 40 px (and regrows); mouth -.5 |
| Dropped | `thud` twice (the second small), brick pops, 4 `chip`; `grumpy`, then `proud` ("it is a wall") |
| Refuses | grappling gun (mortar cannot take a point load), parachute (a cubic metre of brick weighs about 1.8 t; no canopy would open), suction cups (porous brick) |

**6 Topic mischief** Items: H1 `half-timbered-wall`, `hollow-brick-wall-1970s`, `masonry-wall-1980s`, `wall-geg`, `wall-passive-house`; H2 `plattenbau`, `gruenderzeit`. Style: **leans its shoulder**, the slowest push of all: 10 px over 1.2 s. Taken back: `stonewall` (arms crossed) for 20 s, and it refuses to leave the spot.

### 9. battery: Battery, the battery / Akku

Walks at 46 px/s; 42 x 52; energy .90, sociability .80, curiosity .60.
**Rig.** Bones `root` (legs), `body` > `arm-left` (rest 20), `arm-right` (rest -20), `spark`, `cap`, `bar-1..4` (y 16, 12.25, 8.5, 4.75); parts legs, arms, `spark` (accent), `cap` (detail), `casing` (r 8), `window` (ink), four `bar` rects (accent); eyes above the window. Existing clips: `hum`, `bounce`, `recharge`, `zap` (fidgets), `wave`, `snuggle`, `bicker`, `drained` (sulk), `standby` (sleep), `touchdown`.
**Physics hooks.** A phone charge is 15 Wh, an EV 60 kWh (5.5 h at 11 kW), a petrol tank 440 kWh (seven times the battery, a quarter usable); a 2 kW kettle would drain a phone charge in under a minute: amount against rate.

**1 States** (`ladder`: empty < charged < full; `charging` is transient)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `empty` Leer | no energy stored | existing `drained` as overlay (bars scaleY .05, body.scaleY .94, y +2, arms hang, legs wobble); tint `#5a6b5f/#8c9a90/#a8a89e` | in: CCW; the kettly skit (R18). Out: CW; solary `peak` or windy `rated` near (R58) |
| `charged` (default) Geladen | partly stored | `hum` with bar-4 held at scaleY .05 (three bars) | default |
| `charging` Lädt | a rate fills an amount | +loop `fill` 2.0s: bars fill bottom to top in turn, the next one blinks, spark flashes every 0.5 s; `bounce` tempo x1.3; accent `#ffd23f` | in: R58, trick `zap`. Out: reaches `full` after 8 s |
| `full` Voll | 100 % | all four bars; one `star4` on entering; tint `#2fb36c/#7dffcf/#e6fff4`; proud bounce | in: CW; 8 s of charging. Out: 40 s later back to `charged` (self-discharge) |

**2 Tricks**
- `bar-count` Balken zählen (L2): existing `recharge` 3.2 s: bars drain top-down, a sag, pop back bottom-up, springs up; emit 4 `dot` (accent) climbing inside the window. Leaves `full` 12 s.
- `zap` Funke (L3): existing 1.6 s: the spark jumps cap -> arm and back; emit 1 `zigzag` (accent) 10 px between cap and arm tip. Leaves `charging`.
- `charge-up` Aufladen (CW): 1.8s once: bars fill, 3 `dot` (accent) orbit r 16 around the body for 1 s; steps `empty -> charged -> full`.
- `discharge` Entladen (CCW): 1.8s once: bars drain top-down, a sag, sparks fall; steps `full -> charged -> empty`.
- `power-vs-energy` Leistung gegen Energie (DUET kettly): 3.0s once: kettly's lamp flashes and the four bars flick off one after another in 1 s (a rate); then they refill slowly over 2 s (an amount) while a long `meter` bar (+prop `meter`) grows beside it; both `grumpy` 10 s; bond -.05.
- Shake outcome: `slosh`: the bars slosh (scaleY alternating), `scared`, then `charged`.

**3 Purr** `trickle-charge` 2.0s loop: bars light in a slow rising pulse (bar-1..4, 0.25 s apart), body.scaleY 1 <-> 1.025, cap.y +-.5, arms rock +-4; emit 1 `dot` (accent) climbing from the window to the cap every 0.6 s.

**4 Moods** lean: `playful`, `happy`, `sleepy`; `sad` when `empty` for long.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `clamp-climb`: the arms hold two jump-lead clamps (+parts `clamp` x2: ink crocodile jaws with accent tips); they bite onto the card edge in turn with a `star4`; fast, 46 px/s |
| Down | `lead-bungee`: a coiled lead (+prop `coil`, ink spring spiral, four loops) anchored on the edge; it drops, the spiral stretches x2.5, rebounds twice, then lowers it (energy stored in a spring, again an amount) |
| Dangle | grab at the `cap`: bars flicker with the swing, arms and legs flail, a spark bursts at every swing; mouth round |
| Dropped | bounces on landing (squash and a rebound twice as high as the others), a `zap`; `playful` ("again!"); `empty` lies flat, `sad` |
| Refuses | parachute (nylon sticks to its terminals: static) |

**6 Topic mischief** Items: P1 `ev-battery`, `phone-charge`, `wallbox`; P3 `ev-battery`, `phone-charge`, `petrol-tank`; P2 `wallbox`. Style: a **hip bump** for small chips (`phone-charge`: 14 px); a failed lift for `petrol-tank` (it strains, one bar drops, the chip moves 4 px: "seven times my capacity"). Taken back: the bars drop to `empty` for 6 s until solary arrives or a click refills it.

### 10. windy: Windy, the wind turbine / Windenergieanlage

Walks (tiptoes) at 28 px/s; 48 x 56; energy .90, sociability .55, curiosity .60. **No arms.**
**Rig.** Bones `root` (legs `leg-left`, `leg-right` hang on `root`), `body` (the tower) > `head` (y -27) > `rotor` > `blade-a`, `blade-b` (rest 120), `blade-c` (rest 240); parts legs (paths), `tower`, `tower-shade`, `flange`, three blades (accent), `hub` (r 9.5, body); eyes and mouth on `head`. Existing clips: `breeze` (idle: a rotor turn per 3.6 s), `tiptoe`, `gust` (spin to 1920 degrees), `lull` (fidgets), `wave` (greet, blades stretch), `nuzzle` (cuddle), `whirl`, `becalmed` (sulk), `doze`, `touchdown`, `leap`, `tumble`.
**Physics hooks.** 5 MW at rated wind (the 635 turbines of 2024 averaged 5.1 MW); 10 GWh a year is about 2,000 full-load hours; the ICE's 8 MW is 1.6 turbines; wind is driven by the Sun heating the atmosphere; in a storm a turbine feathers and stops (cut-out, general physics).

**1 States** (`ladder`: becalmed < turning < rated; `feathered` from a storm)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `becalmed` Flaute | no wind | existing `becalmed` as overlay (rotor stopped at 40 degrees, blades .86, head drooping, tower leaning forward 4); tint `#6a6ab0/#d8d4c2/#4c5756` | in: CCW. Out: CW, a gust (R10), a blazing sunny near (R11) |
| `turning` (default) Dreht | breeze | existing `breeze` | default |
| `rated` Nennleistung | rated wind speed, rated power | +loop `rated` 0.9s: rotor 0 -> 360 per 0.9 s, blades scaleY 1.05, head lean 3, tower sway +-2; emit 3 `streak` (paper) from the wind side every 0.5 s; tint `#9a9aff/#ffffff/#4c5756` | in: CW, `full-power`, R10. Out: 12 s, or it feathers (see next) |
| `feathered` Sturmstellung | cut-out: blades turned edge-on, rotor stopped | blades scaleY .25, rotor still, body.scaleY .92 (crouching); tint `#5a5aa0/#d8d4c2/#4c5756` | in: a `thunder` near; `rated` held for 25 s with a storm cloud near. Out: 8 s |

**2 Tricks**
- `full-power` Volle Leistung (L2): existing `gust` 2.4 s: the rotor spins up to 1920 degrees and settles; emit 8 `streak` (paper) left to right. Leaves `rated`, 8 s.
- `yaw-turn` In den Wind drehen (L3): 1.8s once: head.scaleX 1 -> -1@.5 -> 1 (the nacelle yaws to face the wind), body.rotation -4 -> +4; emit 4 `streak` from the pointer's side. Leaves `turning`.
- `spin-up` Hochdrehen (CW, steps up): the rotor accelerates 3.6 s -> 1.8 s -> 0.9 s per turn, 1.6 s per step; steps `becalmed -> turning -> rated`.
- `wind-down` Auslaufen (CCW, steps down): like `lull`: the rotor decelerates, a squeak (a `ring` pair), stops; steps `rated -> turning -> becalmed`.
- `gust-ride` Böe reiten (DUET cloudy `gust`, or AUTO(playful) in `rated`): 2.4s once: leans into the wind, lifts a foot, tiptoe-skates 14 px; emit 6 `streak`; `happy`.
- `pinwheel` Windrädchen (AUTO(playful)): 2.0s once: a full pirouette on its toes at `rated` speed; emit a `dot` (paper) trail along a spiral `arc`.
- Shake outcome: `dizzy`: the rotor spins back and forth, `scared`, then `becalmed`.

**3 Purr** `blade-hum` 3.0s loop: rotor at half its idle speed, blades scaleY 1 <-> 1.08 one after another, head nods +-2, tower sways +-1.5; emit a soft `ring` (paper) from the hub every 1.5 s.

**4 Moods** lean: `playful`, `happy`, `sad` (restless in a calm); baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `propeller-lift`: the head tips back 20 degrees, the rotor spins up (1920), the body rises 50 px/s for up to 60 px with `streak`s going down: a generator run backwards (as pumpy is a chiller run backwards). A near battery adds 20 px ("jump-start") |
| Down | `autorotation`: the rotor freewheels one turn per 0.7 s, it sinks 30 px/s with a lateral drift (a sycamore seed) |
| Dangle | grab at the hub: the tower hangs, the rotor spins freely from the swing, blades flutter, legs paddle; mouth round, `curious` |
| Dropped | autorotation above 30 px; lands with the rotor still turning for 1 s; `happy` |
| Refuses | ladders, climbing, suction cups and the grappling gun: it has no arms and two thin legs |

**6 Topic mischief** Items: P2 `wind-turbine`, `ice-train`; P3 `wind-turbine-year`. Style: **blows**: leans back, rotor to `rated`, 8 `streak` from the left; the chip flutters 4 degrees and slides 16 px. For `ice-train` it races it: 16 px out and back in 1 s ("chuff"). Taken back: `becalmed` for 4 s, waits for the next gust.

### 11. boily: Boily, the boiler / Heizkessel

Walks at 24 px/s; 48 x 53; energy .40, sociability .35, curiosity .20.
**Rig.** Bones `root` (legs), `body` > `arm-left` (rest -25), `arm-right` (rest 25), `flue` (y -35) > `puff`, `flame` (y -4.5), `needle` (rest -40); parts legs and arms (ink), `puff` (paper), `flue-pipe`, `flue-cap`, `body` (path), `brows` (ink), `window` (ink ellipse r 7), `flame` (accent), `flame-core` (detail), `gauge` (paper), `needle`; eyes above the window, mouth at y -20.5. Existing clips: `simmer` (idle), `plod`, `whoomp`, `puff` (fidgets), `tip-flue` (greet), `warm-up` (cuddle), `rumble`, `grumble` (sulk), `pilot-light` (sleep), `clang`, `leap`, `tumble`.
**Physics hooks.** Efficiencies .8 (old), .85 (low-temperature), .9 (condensing); hot water divided by .5 (storage and circulation losses); 409 against 122 kWh/(m2·a) for the same old house with boiler or heat pump; a litre of heating oil is 10 kWh; five gas houses in D2; an oversized boiler cycles on and off.

**1 States** (`ladder`: pilot < burning < roaring)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `pilot` Zündflamme | standby, only the pilot flame | existing `pilot-light` as overlay: flame scale .3, body.scaleY .97, needle low; tint `#5f4f48/#3a7fd0/#9fc4ee` (a blue flame) | in: CCW; summer; a wrapped housy near (R31). Out: CW |
| `burning` (default) Brennt | normal load | existing `simmer` | default |
| `roaring` Volllast | the design day, full output | +loop `roar` 0.6s: flame.scaleY 1.6, flame.rotation +-6, body.x +-.5 (rumble), needle +40; emit 3 `dot` (detail) embers rising from the flue every 0.5 s; tint `#8a6454/#ff6a00/#ffe45c` | in: CW; `whoomp`; a `cold` housy near (boily wants to help). Out: 20 s |

**2 Tricks**
- `whoomp` Wumm (L2): existing 1.8 s: the flame ducks (scaleY .5@.15) and bursts (1.9@.3), arms fly up, needle swings; emit 6 `dot` (detail) embers, `burst` 14-24 px. Leaves `roaring` 5 s.
- `smoke-ring` Rauchring (L3): existing `puff` 2.4 s: needle climbs in jerks, the flue kicks; emit 3 `ring` (paper) in a row rising 25 px and growing 5 -> 12 px. Leaves `burning`.
- `fire-up` Anfeuern (CW, steps up): a gas valve click (needle jump), the flame turns from blue to orange in 1 s (tint ramp); emit 4 `dot`; `pilot -> burning -> roaring`.
- `bank-down` Drosseln (CCW, steps down): the flame shrinks and cools to blue, boily yawns (mouth round) and blows one `puff`; `roaring -> burning -> pilot`.
- `efficiency-bars` Wirkungsgrad-Balken (DUET pumpy, or AUTO(grumpy)): 3.0s once: a `meter` bar (+prop `meter`) over boily filled to .8 of its frame, pumpy's overflows to 3.0; boily crosses its arms; both `grumpy`; bond -.05.
- `relight` Feuer geben (DUET flamy `snuffed` or `ember`): 2.0s once: boily bends, a spark `dot` (detail) jumps on an `arc` to flamy's wick; flamy `burning` and `happy`, boily `proud`; bond +.05.
- Shake outcome: `cough`: 5 `puff` (ink) from the flue, the gauge needle spins, `grumpy`.

**3 Purr** `pilot-purr` 3.2s loop: the flame breathes .9 <-> 1.1 and shifts between blue and orange (tint ramp 30 %), needle ticks +-4, body.scaleY +-.015, lids .3; emit two bubbles (`dot`, paper) rising in the window, and one `dot` from the flue every 2 s.

**4 Moods** lean: `grumpy` (a veteran), `proud`, `sad` (it feels redundant); `content` next to flamy.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `chain-hoist`: a hook on a chain (ink links, accent hook) swings from the flue to the card edge; it hauls hand over hand at 10 px/s with a `clang` at every tug (installed by crane, as in life) |
| Down | `chain-abseil`: the same chain run backwards; flame turned to the blue pilot for safety |
| Dangle | grab at the flue cap: the flame leans upward along the swing (flame.rotation), the needle swings +-60, the ring `puff` bobs, legs kick |
| Dropped | big `clang`, gauge spins, the flame goes out and the pilot relights itself after 3 s; above 120 px a ring of smoke is coughed; `grumpy` |
| Refuses | parachute (an open flame under nylon), suction cups (no flat palms) |

**6 Topic mischief** Items: D2 `kfw-55-gas-solar`, `enev-2014-gas`, `wschvo-1995-gas`, `gruenderzeit-gas`, `old-house-gas`; D1 `unrenovated-old-building`, `wschvo-1995`, `enev-2014`; P1 and P3 `heating-oil-litre`. Style: **rolls it with its belly**: a chugging push, the flue smoking; 12 px; for `heating-oil-litre` it sniffs it first (nostalgia). Taken back: `grumble`, drops to `pilot` for 5 s.

### 12. roofy: Roofy, the roof / Dach

Walks (scampers) at 46 px/s; 56 x 42; energy .55, sociability .45, curiosity .50.
**Rig.** Bones `root` (legs), `body` > `arm-left`, `arm-right` (x +-22), `chimney`, `tiles-low`, `tiles-high`, `tile` (rest 50.66); parts legs and arms (ink), `chimney`, `chimney-pot` (accent), `roof` (path, body), `tiles-low`, `tiles-high` (detail strokes), `tile` (body rect); eyes and mouth on `body`. Existing clips: `breathe`, `scamper`, `loose-tile`, `ripple` (fidgets), `hat-tip` (greet), `shelter` (cuddle), `clatter`, `hunker` (sulk), `roost` (sleep), `plop`.
**Physics hooks.** U 2.1 (concrete, 1950s), .20 (GEG), .10 (passive, 35 cm); the roof skin reaches 60-70 degrees C (sol-air temperature); attic flat 50 W/m2; roof windows leak; the roof carries the PV (950 kWh/kWp).

**1 States** (`ladder`: snow-capped < dry < sun-baked; `wet` from a rule)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `snow-capped` Schneebedeckt | cold, snow load | +part `snow` (paper path cap on the ridge, 40 x 8, with 2 drips), body.scaleY .96 shivering, a breath `puff`; no tint | in: CCW from `dry`; a `frosted` chilly near (R62). Out: CW; sunny near melts it in 6 s (4 `drop`) |
| `dry` (default) Trocken | normal | existing `breathe` | default |
| `sun-baked` Sonnenheiß | skin temperature 60-70 degrees C | +loop `bake` 1.6s: body.scaleY .96, arms limp, tiles flutter; emit 3 `wave` (accent) rising 16 px from the ridge and 3 `drop` (paper) sweat from the eaves; tint `#7a6a74/#d4552e/#3a3030` | in: CW; a blazing sunny near (R37). Out: 30 s, or CCW |
| `wet` Nass | rain | tint `#3f5265/#8c3b26/#1f2a2c`, emit 4 `drop` (paper) from the eaves and 2 `streak` (paper) sheen strokes | in: a raining cloudy over it (R44). Out: 15 s |

**2 Tricks**
- `tile-flip` Ziegel-Flip (L2): existing `loose-tile` 1.9 s boosted: the tile lifts to 62 degrees, flips like a coin (tile.scaleX 1 -> -1 -> 1) and lands back; emit 1 `star4` (paper) glint. `proud`.
- `tile-wave` Ziegelwelle (L3): existing `ripple` 1.8 s: the tile rows jump 3.2 px in sequence, the chimney bounces; emit 4 `chip` (accent) hopping off and back.
- `sun-bake` Sonnenbad (CW, steps up): 2.0s once: turns to the sun, arms spread, shimmer builds, sweat appears; `snow-capped -> dry -> sun-baked`.
- `shake-off` Abschütteln (CCW, steps down): shakes like a dog (body.rotation +-8 at 6 Hz), flinging 8 `drop` or `flake6` (paper) sideways; `sun-baked -> dry -> snow-capped`.
- `carry-pv` PV tragen (DUET solary within 40 px, or AUTO(proud)): 3.0s once: roofy crouches, solary hooks onto the ridge, roofy stands up carrying her; both `happy`; bond +.05 (the roof carries the PV).
- Shake outcome: `clatter`, the tile pops off and returns.

**3 Purr** `tile-purr` 2.4s loop: `tiles-low` and `tiles-high` alternate y +-.8 (like ribs), tile +-4, chimney pot nods, arms rest; emit one `puff` (paper) from the chimney every 1.8 s.

**4 Moods** lean: `content`, `scared` (storms), `proud`; baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `roof-ladder`: a hooked roof ladder (two ink rails, a big hook at the top that grabs the card edge like a ridge), scampering arms alternate; 24 px/s |
| Down | `umbrella-roof`: the roof tips back, body.scaleX 1.2, legs dangle below, it sinks 30 px/s and sways +-6 (a roof used as a parasol) |
| Dangle | grab at the chimney: the roof hangs sideways 15 degrees, the loose tile swings +-40, legs dangle; `scared` |
| Dropped | umbrella-roof at once; without it a `plop` and a tile pops off (and returns); `sad` |
| Refuses | grappling gun (tiles crack under a point load; real roofers anchor to the ridge), suction cups |

**6 Topic mischief** Items: H1 `solid-roof-1950s`, `roof-geg`, `roof-passive-house`; C2 `attic-flat`; D1 `unrenovated-old-building`, `plus-energy-house`; P1 `pv-annual-yield`. Style: **flips with the loose tile as a lever**: the tile snaps, the chip tips 4 degrees and slides 12 px; with `pv-annual-yield` it carries the chip on its ridge for 3 s (a PV roof) and sets it down. Taken back: `hunker` 5 s, the tile pops.

### 13. insuly: Insuly, the insulation / Dämmung

Hops at 22 px/s; 48 x 40; energy .25, sociability .80, curiosity .35.
**Rig.** Bones `root` (legs), `body` > `tail`, `arm-left`, `arm-right`, six tuft bones (`tuft-side-left/right`, `tuft-top-left/right`, `tuft-top-mid-left/right`); parts `tail` (rect, body), `foil` (accent line), legs and arms (ink), six static `fluff` ellipses, six tuft ellipses (on their bones), `wool` (rect), `batting` (detail path), three `sheen` strokes (accent). Existing clips: `breathe`, `hop`, `fluff-up`, `tuck-in` (fidgets), `wave` (greet), `snuggle` (cuddle), `bristle`, `deflate` (sulk), `snooze`, `flump`.
**Physics hooks.** Conductivity .035 W/(m·K); 12 cm gives U .28, 17 cm .20, 24 cm .15, 35 cm .10 (against 2.1 uninsulated); a deep retrofit cuts demand from 303 to 54 kWh/(m2·a); wet or compressed insulation loses its air (general physics).

**1 States** (`ladder`: compressed < fluffy < thick; `soaked` from a rule)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `compressed` Zusammengedrückt | the air is squeezed out, conductivity rises | +loop `squash` 3.4s: body.scaleY .78, scaleX 1.12, tufts .7, arms tucked | in: CCW; shaken. Out: CW |
| `fluffy` (default) Flauschig | dry, loose | existing `breathe` | default |
| `thick` Dick | a thick layer: 24-35 cm | tufts 1.2, body.scale 1.12, +part `batt2` (a larger scalloped outline behind the body); content | in: CW; a finished tuck-in. Out: 60 s |
| `soaked` Durchnässt | wet insulation insulates badly | tint `#b04a63/#c8d6df/#7a3548`, tufts y +3 (drooping), body.scaleY .92, 2 `drop` (accent) drip | in: a raining cloudy over it (R44). Out: `dry-shake`, or roofy shelter (R45); 40 s |

**2 Tricks**
- `fluff-burst` Flaum-Wolke (L2): existing `fluff-up` 2.0 s: tufts swell in turn, a shake; emit 6 `puff` (paper) fibres floating up 18 px. Leaves `thick` 10 s.
- `tuck-in` Einpacken (L3): existing 2.3 s extended: +prop `blanket` (rect 30 x 12, kit paint `wool` with a `wool-light` stripe) slides over the nearest wrappable pet within 40 px for 1.4 s, two pats; housy, waly and roofy change state (R41-R43); with nobody near it tucks itself in (`sleepy`).
- `bulk-up` Aufplustern (CW, steps up): `compressed -> fluffy -> thick`, tufts growing, 1.6 s per step.
- `flatten` Plattdrücken (CCW, steps down): presses itself flat with a sigh; on `soaked` any circle plays `dry-shake`.
- `dry-shake` Trockenschütteln (AUTO(sad) when `soaked`): 2.4s once: shakes like a wet dog; emit 8 `drop` (accent) flung sideways; leaves `fluffy`.
- Shake outcome: `itch`: 10 `dot` (paper) of fibre burst out, `grumpy` (mineral wool itches).

**3 Purr** `fibre-purr` 3.4s loop: the tufts breathe in a ripple (top-left to top-right, lag .2), body.scale +-3 %, tail rolls +-6, arms hug itself; emit one `dot` (paper) floating up every 1.6 s.

**4 Moods** lean: `sleepy`, `content`, `sad` (wet); `proud` while wrapping.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `fibre-cling`: arms and body flatten (scaleX 1.2, scaleY .85) and cling to the card side like a burr; an inchworm: body.scaleY stretches 1.25 then contracts, 6 px per 0.8 s |
| Down | `dandelion-fluff`: tufts x1.5, body x1.25, sinks 20 px/s drifting: low density and a large area make it its own canopy |
| Dangle | grab at a top-middle tuft: hangs pear-shaped (body.scaleY 1.15), the foil tail swings, legs dangle; `curious`, serene |
| Dropped | always the dandelion fluff; `flump`; `happy`; nothing hurts it |
| Refuses | ladders (tuft arms cannot grip rungs), grappling gun (fibres snag and tangle the line) |

**6 Topic mischief** Items: H1 `wall-geg`, `roof-geg`, `wall-passive-house`, `roof-passive-house`, `masonry-wall-1980s`; H2 items; D1 `wschvo-1995`; D2 `deep-retrofit-heat-pump`. Style: a **soft nudge** with a tuft (8 px, the gentlest push) followed by a pink outline around the chip for 6 s (it "wraps" it; nothing changes in the layout). Taken back: the outline falls, `deflate`, `sad`, then it tries again with another wall item.

### 14. shady: Shady, the external sun shading / Sonnenschutz

Walks at 22 px/s; 44 x 48; energy .25, sociability .45, curiosity .30.
**Rig.** Bones `root` (legs `leg-left`, `leg-right`), `body` (y -38) > `blind` > `slat-1..6`, `cord`, `shades`; parts legs, `curtain` (ink), six slats (body), `cord`, `tassel` (accent), `rail` (accent), sunglasses (`temple-left/right`, `bridge`, `lens-left/right` detail with accent stroke, two glints); the face is above `rail`, eyes on `body`, mouth on `slat-5`. Existing clips: `lounge`, `shuffle`, `slat-wave`, `roll` (fidgets), `peek` (greet), `chill` (cuddle), `clatter`, `shut` (sulk), `siesta`, `touchdown`.
**Physics hooks.** External shading: 6 W/m2 shaded against 100 W/m2 unshaded; DIN 4108-2 summer protection; unshaded roof windows; the roller-shutter box leaks at 3.6; in winter blinds should be up (solar gains help).

**1 States** (`ladder`: raised < tilted < lowered)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `raised` Hochgezogen | the blind is rolled up: sun comes in (right in winter) | blind.scaleY .15, slats compressed, sunglasses pushed up to the rail (shades.y -9) | in: CCW; the heating scene; cloud overhead (R40). Out: CW |
| `tilted` Gewendet | slats angled 40 degrees: daylight in, glare out | slat-1..6 rotation 40, existing `chill` | in: L2; CCW from `lowered`. Out: CW/CCW |
| `lowered` (default) Heruntergelassen | closed, sun blocked | existing `lounge` | default |

**2 Tricks**
- `slat-wave` Lamellenwelle (L2): existing 2.4 s: the slats tilt in a wave top to bottom; emit 3 `streak` (paper) light shafts through the gaps. Leaves `tilted`.
- `roll-down` Herunterrollen (L3): existing `roll` 2.6 s: rolls up, then drops with a bounce, glasses flash; emit 2 `star4` (paper) on the lenses. Leaves `lowered`.
- `lower` Absenken (CW, steps up): the cord is pulled (cord.y +4) and the blind falls with a concertina bounce, 1.6 s; `raised -> tilted -> lowered`.
- `raise` Hochziehen (CCW, steps down): the reverse; the sunglasses are pushed up with a puff; `lowered -> tilted -> raised`.
- `sun-block` Schatten spenden (DUET sunny `blazing`, with chilly near): 2.4s once: the lenses darken, +part `shadow` (a dark triangle, 22 x 10, `detail`) extends on the perch towards the sunny; the sun dims (R35), chilly relaxes; `proud`.
- `pose` Cooler Auftritt (AUTO(proud)): 1.6s once: leans back, chin up, glasses glint twice (2 `star4`). `proud`.
- Shake outcome: `clatter`, the glasses fall off and are caught.

**3 Purr** `cool-purr` 3.0s loop: the slats flutter in a slow cascade (slat-1..6, +-3, lag .15), cord sways +-4, body.y +-.6; emit one `star4` (paper) on the lens every 2 s; a smirk.

**4 Moods** lean: `content` (cool), `sleepy` (naps in the sun it blocks), `proud`; baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `cord-haul`: it throws its cord up (cord.scaleY 3, the tassel as hook) over the edge and hauls by rolling itself up: blind.scaleY .3 (concertina) while body rises 18 px/s ("pull the cord") |
| Down | `awning-chute`: a striped half-round awning (+prop `awning`: arc path 28 px wide, ink and accent stripes) pops from the rail like a parasol (a Markise); sinks 28 px/s |
| Dangle | grab at the cord's tassel: the blind hangs under the rail and clatters like a venetian blind, the glasses slip to the nose, a frown (mouth on `slat-5`) |
| Dropped | a concertina `touchdown`, the glasses jump; `grumpy`, then the `pose` |
| Refuses | suction cups, grappling gun (nothing to mount a gun on; the cord is its grappling hook) |

**6 Topic mischief** Items: C2 `passive-house-home`, `new-home-geg`, `office-passive-house`, `office-geg-shading`, `attic-flat`, `office-1970s`; H1 `roller-shutter-box`; D1 `passive-house`. Style: **shades**: it pulls its blind over the chip (a 30 % dark overlay for 3 s) and nudges it 12 px "out of the sun"; the overlay applies to every chip it touches, so it says nothing about the value. Taken back: it sighs, `raise`s and pushes the glasses up, sulks 5 s.

### 15. venty: Venty, the ventilation unit / Lüftungsgerät

Floats at 28 px/s, hover 3 px; 44 x 41; energy .50, sociability .60, curiosity .70.
**Rig.** Bones `root`, `body` (y -20) > `streak-upper`, `streak-lower`, `puff`, `jet-back`, `jet-middle`, `jet-front`, `duct-back`, `duct-front`, `fan`; parts streaks and jets (ink lines), `puff` (detail), two ducts with collars (detail), `casing` (r 6), `fan-window` (detail), `fan-blades` (body), `fan-hub` (accent); eyes at x 6 and 14.5, mouth at x 10.3. Existing clips: `hover` (idle), `glide`, `spin-up`, `exhale` (fidgets), `nod` (greet), `cosy` (cuddle), `fuss`, `stale` (sulk), `settle`, `touchdown`.
**Physics hooks.** Air change rates from .13 to 540 1/h; .3 in a passive-house flat, .5 nominal, 4.8 in a classroom, 540 in an ISO 5 cleanroom (HEPA); heat recovery of at least 75 %; ventilation heat loss 6.5 against 61 kWh/(m2·a); night ventilation keeps a passive-house office cool; window ventilation loses what the unit recovers.

**1 States** (`ladder`: low < nominal < boost; `bypass` from a trick or a rule)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `low` Grundlüftung | .3 1/h, basic ventilation | fan one turn per 6 s, jets x.6, body.y +1; tint `#5f8428/#d8d4c2/#b5d4c8` | in: CCW; night. Out: CW |
| `nominal` (default) Nennlüftung | .5 1/h | existing `hover` | default |
| `boost` Intensivlüftung | purge: several air changes per hour | +loop `boost` 0.8s: fan one turn per 0.8 s (a blur), jets scaleY 1.8, streaks x3, body.y -1; tint `#7fc03a/#ffffff/#e6f8ee` | in: CW; `spin-up`; a `hot` servy or thermy near (R54). Out: 15 s |
| `bypass` Sommer-Bypass | night or summer cooling: the heat exchanger is bypassed, no recovery | +part `flap` (rect 8 x 3, detail) slides across the casing; the incoming `streak` stays cold (detail) instead of warming; tint `#4f9c6a/#f7f3e3/#9ed8e6` | in: `night-cool`; a hot room near. Out: 30 s |

**2 Tricks**
- `spin-up` Hochdrehen (L2): existing 2.4 s: the fan turns three times, two streaks shoot out of the back, it rises on longer jets. Leaves `boost` 6 s.
- `heat-recovery` Wärmerückgewinnung (L3): existing `exhale` 2.6 s extended: a stale `puff` (ink) leaves through the back duct, a fresh stream enters through the front duct, inside two `streak`s cross in an X; the incoming stream changes paint from `detail` (cold) to `accent` (warm) at the middle (75 % recovery). Emit 3 `puff` out, 3 `streak` in. `proud`.
- `air-up` Mehr Luft (CW, steps up): the jets lengthen and the fan speeds up, 1.4 s per step; `low -> nominal -> boost`.
- `air-down` Weniger Luft (CCW, steps down): the reverse with a sigh `puff`.
- `filter-tap` Filter klopfen (AUTO(grumpy) when the air is stale, DUET servy): 2.4s once: a filter (+prop `filter`, rect 10 x 8, `paper` with pleat lines) slides out 10 px, a duct taps it three times, emit 6 `dot` (ink) of dust, slides back; mood `proud` ("clean air"); ends a stale spell.
- `night-cool` Nachtkühlung (AUTO(curious) in the cooling scene or beside a hot room): 3.0s once: the `flap` slides, cold `streak`s (detail) enter, warm `puff`s leave. Leaves `bypass` 30 s.
- Shake outcome: `gasp`: the jets sputter sideways, the fan reverses, 6 `streak`; `scared`.

**3 Purr** `quiet-fan` 3.0s loop: the fan one turn per 3 s, the jets pulse +-.2 in a ripple back to front, body.y +-1, ducts nod +-3; emit one `streak` (detail) trailing from the back every 1.2 s.

**4 Moods** lean: `curious`, `proud` (clean air), `grumpy` (stale air); baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `jet-lift`: the jets scaleY 1.8, the fan a blur, it rises 40 px/s for up to 60 px (a mini drone) and settles |
| Down | the normal glide with feathered jets (x.5) and a `puff` brake; 40 px/s |
| Dangle | grab at `duct-front`: tilts nose-down, the jets sputter sideways, the fan windmills backwards, exhaust `streak`s trail; mouth round |
| Dropped | the jets burst and right it in the air; lands softly; `proud` ("I am a hovercraft") |
| Refuses | ladders and climbing (hollow ducts cannot grip), grappling gun (a rope would be sucked into the intake), parachute (its thrust does the job) |

**6 Topic mischief** Items: C1 (all fifteen air change rates), C2 `office-passive-house`; D1 items; D2 `kfw-40-heat-pump`, `passive-house-direct-electric`; H2 `kfw-40`, `gruenderzeit-retrofit`. Style: **blows**, always the same 10 px. Design trap avoided: pushing harder at a larger air change rate (a cleanroom's 540 against a warehouse's .13) would put the answer into the animation, so strength, flutter and duration are constants. Taken back: a `fuss`, then a `filter-tap`.

### 16. chilly: Chilly, the chiller / Kältemaschine

Walks (skates) at 44 px/s; 52 x 44; energy .45, sociability .25, curiosity .35.
**Rig.** Bones `root` (legs), `body` > `arm-left`, `arm-right` (rest -25 / 25), `crystals`, `breath`, `frost`, `emblem`; parts `breath` (detail, ink stroke), legs (paths), arms, `crystals` (detail), `body` (r 6), `frost` (accent), `vent` (ink), `snowflake` (accent stroke on `emblem`); eyes at y -18.5, mouth y -12.5. Existing clips: `chill` (idle), `glide` (skating), `frost-over`, `flurry` (fidgets), `nod` (greet), `thaw` (cuddle), `glare`, `cold-shoulder` (sulk), `hibernate`, `skid`, `leap`, `tumble`.
**Physics hooks.** Cooling loads from 6 to 1,000 W/m2; every watt of IT power is heat (8,000 full-load hours); hospital 90 kWh/(m2·a); the 26 degrees C cooling line; external shading spares the chiller; unshaded glazing is its biggest load; siblings with the heat pump.

**1 States** (`ladder`: standby < cooling < frosted; `overloaded` from a rule)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `standby` Bereitschaft | no load | frost scaleY .2, crystals .6, emblem still, lids .3; tint `#6a97b0/#d8e2e6/#cfe0e4` | in: CCW; shady lowered near (R35). Out: CW |
| `cooling` (default) Kühlt | normal | existing `chill` | default |
| `frosted` Vereist | the evaporator ices up | `frost` scaleY 1.6 over the front, crystals x1.5, shivers; emit 1 `flake6` (paper) falling every 1.5 s; tint `#5ab6e0/#ffffff/#e4f6fb` | in: CW; `ice-crown`. Out: CCW; thaws in 30 s |
| `overloaded` Überlastet | peak load: the sun on glass, a server hall | +loop `strain` 0.8s: body.scale 1.03, emblem spin x3, breath `puff` quick; emit 3 `drop` (detail) sweat; tint `#1f7fb0/#f7f3e3/#ffb48a` | in: a blazing sunny near without shade (R34); a hot servy near (R55). Out: shady relief (R35); 30 s |

**2 Tricks**
- `ice-crown` Eiskrone (L2): existing `frost-over` 2.8 s: frost creeps down behind the face, the crystals grow (crystals.scaleY 1 -> 1.8), a shiver, then part of it melts; emit 4 `star4` (paper) on the crystals. Leaves `frosted`, 30 s.
- `flurry` Schneegestöber (L3): existing 2.4 s: the emblem spins up (960 degrees) and grows, it leans back and blows; emit 8 `flake6` (paper) `burst` forward 30 px and 1 `chip` (detail, an ice cube) on a 14 px `arc`. Leaves `cooling`.
- `cool-down` Runterkühlen (CW, steps up): crystals grow, 1.6 s per step; `standby -> cooling -> frosted`.
- `thaw` Abtauen (CCW, steps down): existing `thaw`; emit 6 `drop` (detail) from the frost and a flat `ring` puddle; `frosted -> cooling -> standby`.
- `chill-out` Abkühlen (DUET servy, thermy, a `hot` pet): 2.6s once: the breath flows to the partner, 5 `streak` (detail) sweep across; the partner's heat waves shrink, thermy's column drops a step. Chilly `proud`, partner relieved.
- `skate-spin` Pirouette (AUTO(playful)): 2.0s once: a pirouette on the blades (body.rotation 0 -> 720, skates lifting); emit a spiral of 8 `flake6` (paper) on the ground.
- Shake outcome: `snow-globe`: 14 `flake6` burst, a shiver; `scared`.

**3 Purr** `compressor-sigh` 3.6s loop: the emblem turns slowly (60 degrees/s), the crystals twinkle (scaleY +-8 %, staggered), body.scale +-1.5 %, a small breath `puff` drifts 8 px every 1.8 s; emit one `star4` (paper) on the frost every 2 s.

**4 Moods** lean: `grumpy` (aloof), `content` (the cold shoulder), `sad` (secretly lonely); `proud` near shady.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `ice-axes`: the arms grow axes (+parts `axe` x2: ink handle, accent blade, 7 px) and the skates turn into crampons; alternating bites with 3 `star4` chips each, 30 px/s. For a low step it freezes a block (a `chip` 6 x 4, detail) under its skate |
| Down | `snowflake-chute`: the emblem swells x3 above its head (a six-armed snowflake canopy, accent stroke, ink strings) and turns slowly; sinks 24 px/s (snowflakes fall slowly because of drag) |
| Dangle | grab at the crystals: hangs, the frost sheds 4 `flake6`, the skates swing, the arms flail; mouth -.4, `grumpy` |
| Dropped | above 50 px the snowflake chute; otherwise `skid`; `grumpy` |
| Refuses | ladders (a metal rung would stick to its frost), grappling gun (the rope freezes brittle) |

**6 Topic mischief** Items: C2 `passive-house-home`, `school-new-build`, `hospital`, `office-1970s`, `data-centre`. Style: **skates it**: a kick, the chip slides 14 px along a frost trail (4 `flake6`) as if on ice, with a 2 px overshoot; for `data-centre` it frosts the chip's corner for 6 s. Taken back: `glare`; the frost patch melts in 6 s.

### 17. kettly: Kettly, the kettle / Wasserkocher

Walks at 30 px/s; 46 x 44; energy .85, sociability .65, curiosity .45.
**Rig.** Bones `root` (legs `leg-left`, `leg-right`), `lamp` (on `root`), `body` > `steam-a`, `steam-b`, `handle`, `spout`, `lid`, `cheek-left`, `cheek-right`; parts steam puffs (detail, ink), legs (paths), `handle-rim`, `handle` (accent), `spout`, `knob`, `lid`, `base` (rect on `root`), `lamp` (accent), `body`, two cheeks (accent). Existing clips: `simmer` (idle), `waddle`, `boil`, `sigh` (fidgets), `hat-tip` (greet), `cosy`, `boil-over`, `lukewarm` (sulk), `doze`, `touchdown`, `leap`, `tumble`.
**Physics hooks.** 2 kW (230 V x 16 A allows 3.7 kW); one litre from 20 degrees C to boiling is 93 Wh, about 3 minutes; 57 times flamy's 35 W; nine kettles are the 18 kW heating load; the quiz's point: a rate (kW) is not an amount (kWh).

**1 States** (`ladder`: cold < lukewarm < heating < boiling)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `cold` (default) Kalt | cold water | existing `simmer`, lamp small | default |
| `lukewarm` Lauwarm | after the cut-off | existing `lukewarm` as overlay (body.scaleX 1.04, spout drooping, cheeks fading) | in: after `boiling` (auto cut-off); CCW. Out: 40 s to `cold`; CW reheats |
| `heating` Heizt | 2 kW going in | +loop `heat` 1.4s: lamp.scale 1.6, cheeks 1.3, body.x +-.4 growing, lid +-2; emit one `dot` (paper) from the spout every 0.6 s; tint `#8e8a8e/#ff344f/#f7f3e3` | in: CW; trick `boil`. Out: it boils after 6 s |
| `boiling` Kocht | 100 degrees C | continuous steam: `steam-a/b` loop, lid hops +-3 px at 6 Hz; emit 3 `puff` (paper) per second; tint `#a39a9c/#ff1f3d/#ffffff` | in: `boil`, `whistle`, 6 s of heating. Out: the automatic **click-off** after 3 s (lamp.scale .4 and a `star4` snap) into `lukewarm` |

**2 Tricks**
- `boil` Aufkochen (L2): existing 2.8 s: the rattle builds, the cheeks blush, the lamp glows, the lid hops four times, two steam puffs. Leaves `boiling`, then the cut-off.
- `whistle` Pfeifen (L3): 2.2s once: lid up (lid.rotation -25), spout forward; emit 3 `streak` (detail), horizontal 12 px lines from the spout in sequence (the whistle, drawn) and 2 `puff`. Leaves `boiling`.
- `heat-up` Aufheizen (CW, steps up): "three minutes in two seconds": cheeks fill, lamp brightens, 1.6 s per step; `cold -> lukewarm -> heating -> boiling`.
- `cool-down` Abkühlen (CCW, steps down): a sigh, the steam fades; `boiling -> heating -> lukewarm -> cold`.
- `boast` Angeben (DUET flamy or battery, or AUTO(proud)): 2.4s once: chest out (body.scaleY 1.06), lamp flash, a tall column of 4 `puff` (paper) twice flamy's height. Flamy `sad`, battery `grumpy`.
- `pour` Eingießen (AUTO(playful)): 2.4s once: tips the spout (body.rotation 30), a thin stream of 6 `drop` (paper) pours to the perch with a puddle `ring`; `happy`.
- Shake outcome: `boil-over`: 8 `puff` plus `drop`, `scared` and embarrassed.

**3 Purr** `singing-kettle` 1.6s loop: lid rattles +-2 at 8 Hz, body.x +-.4, cheeks pulse, lamp a soft glow; emit two tiny `puff` (paper) per second rising 10 px. A kettle sings just before it boils.

**4 Moods** lean: `playful`, `grumpy` (hot-headed), `proud` (boasts); baseline `content`.

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `steam-jet`: the spout turns down and back (spout.rotation 40), 4 `puff` shoot out and it rises 50 px/s for up to 40 px with the lid rattling; afterwards `lukewarm` for 4 s (tank empty). Alt `handle-hook`: the handle (rotation) hooks the card edge and it hauls itself up at 20 px/s |
| Down | `lid-chute`: the lid pops up on a thin tether (ink line) and swells (lid.scale 2.2) above its head as a little parachute; sinks 32 px/s |
| Dangle | grab at the `handle` (what it is for): the body hangs level, 2 `drop` from the spout, the lid rattles, legs paddle; `scared` and thrilled |
| Dropped | lid-chute; lands with `touchdown` and `hat-tip`; `playful` ("again!") |
| Refuses | ladders (short legs and one handle for an arm), suction cups |

**6 Topic mischief** Items: P1 `kettle`; P2 `kettle`, `heating-load-old-house` ("nine kettles"); P3 `boil-water`. Style: a **belly bump behind a steam cloud** (3 `puff`): the chip slides 12 px. As a duet only: it bumps `phone-charge` (battery's) and `tea-light` (flamy's) with a boast. (N) for `heating-load-old-house` nine tiny kettle glyphs (+prop `kettle-row`) line up for 2 s. Taken back: `boil-over` as a sulk, then `lukewarm`.

### 18. flamy: Flamy, the tea light / Teelicht

Walks (scoots) at 20 px/s; 28 x 46; energy .40, sociability .30, curiosity .65. **No arms.**
**Rig.** Bones `root` (legs `leg-left`, `leg-right`), `body` (the cup) > `flame` > `core`, `flame-tip`; parts legs (paths), `cup` (detail), `wax` (paper ellipse 12 x 2.6), outlined `flame-tip-rim` and `flame-rim` with their fills, `core` (accent); eyes and mouth on `flame`. Existing clips: `glow` (idle), `scoot`, `flicker`, `ember` (fidgets), `tip-wave` (greet), `warm` (cuddle), `flare-up`, `gutter` (sulk), `doze`, `touchdown`, `leap`, `tumble`.
**Physics hooks.** About 35 W (30-40 W): 12 g of paraffin at 12 Wh/g burn in about 4 h; the smallest of the powers against the Sun's 3.8e26 W; the same family as the heating-oil burner (10 kWh per litre) and the gas flame; a flame needs oxygen; rain and gusts snuff it.

**1 States** (`ladder`: ember < burning < high; `snuffed` from a rule or a shake)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `ember` Glimmt | starved of oxygen: small flame | flame.scale .45 (the face shrinks with it; the eyes become dots), `gutter` as overlay; tint `#b84a00/#e68a00/#8c8c84` | in: CCW; a heavy cloudy near (R14). Out: CW |
| `burning` (default) Brennt | normal | existing `glow` | default |
| `high` Auflodernd | draught, extra oxygen | flame.scaleY 1.35, tip curled, flame.rotation 6, a wisp of smoke (`dot`, ink) from the tip; tint `#ff7a00/#ffe45c/#c4c4b9` | in: CW; the `ember-whoosh` trick; a rated windy near (R14, a gentle gust). Out: 15 s |
| `snuffed` Ausgepustet | the flame is out | flame.scale .15, `core` hidden, eyes shut (lid 1); +prop `smoke` (three `dot`, ink, rising), cold wax; tint `#5a5a54/#8c8c84/#c4c4b9` | in: a raining cloudy near (R13); `blow-out`. Out: L1 relights it, boily or kettly relight it (R15, R16), else a match after 20 s |

**2 Tricks**
- `spark-dance` Funkentanz (L2): existing `flicker` 1.6 s: the tip curls left and right while the flame stretches; emit 4 `dot` (accent) sparks hopping up 10 px and fading.
- `ember-whoosh` Glut (L3): existing `ember` 3.0 s: it shrinks to a spark with the face on it, sputters twice, whooshes back taller; emit 3 `dot` (accent). Leaves `high` 6 s.
- `stoke` Anfachen (CW, steps up): the flame grows after a breath-in pause, 1.4 s per step; `ember -> burning -> high`.
- `shield` Abschirmen (CCW, steps down): it leans away from the draught and shrinks; `high -> burning -> ember`, never snuffed by a gesture.
- `look-up` Hochschauen (AUTO(curious) with a sunny near): 2.4s once: on its toes, the flame leans towards sunny, 2 `star4` (accent) twinkle between them; `happy`, bond +.05.
- `relight` Entzünden (L1 on `snuffed`; the DUET with boily or kettly): 1.2s once: a spark `dot` (detail) re-ignites the wick, flame.scale .15 -> 1 with a pop; 3 `star4` (accent).
- Shake outcome: `blow-out`: the shake snuffs it; `scared`.

**3 Purr** `candle-glow` 2.4s loop: the flame breathes .96 <-> 1.06, `core` in anti-phase, the tip sways +-4, the wax shimmers (wax.scaleX +-1.5 %); emit one warm `dot` (accent) drifting up 10 px every 1 s.

**4 Moods** lean: `curious`, `scared` (rain, gusts), `sad` (when snuffed).

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `sky-lantern`: a paper lantern (+prop `lantern`: a cylinder 18 x 26, `paper` with an ink outline, scaleY 0 -> 1 over the flame in 1 s, an accent glow inside) fills with hot air and lifts it 60 px/s with a sway of +-5 (a Himmelslaterne) |
| Down | the lantern sinks 20 px/s, the flame low and guttering |
| Dangle | grab at the cup rim: the flame still points up and gutters with the swing (flame.rotation +-15), it may go `ember`; `scared`; a shake snuffs it |
| Dropped | 20 % chance to snuff (always if a cloudy is within 60 px), otherwise `flicker` then `ember`; `scared`, then `sad` |
| Refuses | ladders and the grappling gun (no arms, and a rope would burn), parachute (nylon melts), suction cups |

**6 Topic mischief** Items: P1 `tea-light`; P2 `tea-light` (the task "From Tea Light to Sun"). Style: a **warm nudge**: 6 px with a heat shimmer; the direction is seed-drawn so it never hints at the order. Taken back: it gutters to `ember` for 5 s and looks up (sad).

### 19. thermy: Thermy, the thermometer / Thermometer

Walks at 38 px/s; 32 x 52; energy .60, sociability .45, curiosity .85.
**Rig.** Bones `root` (legs), `body` > `arm-left`, `arm-right` (rest -20 / 20), `tube` > `column` > `warmth`; parts legs and arms (ink), `tube` (detail), `column-warm` (accent, on `warmth`), `column-cold` (body, on `column`), `ticks`, `bulb` (body); eyes at y -11 on `body`. Existing clips: `steady` (idle), `stride`, `reading`, `tap` (fidgets), `salute` (greet), `glow` (cuddle), `fluster`, `cold-read` (sulk), `nap`, `boing`, `leap`, `tumble`.
**Physics hooks.** 20 degrees C inside and -12 degrees C outside (32 K); the 26 degrees C cooling line; 25 degrees C overheating limit and test condition; 60-70 degrees C roof skin; 100 degrees C boiling water; U-values per kelvin.

**1 States** (`ladder`: cold < mild < hot)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `cold` Kalt | a low reading | `column` scaleY .5 (a short blue stub), shivering as overlay (existing `cold-read` blend), breath `puff`; tint `#2f6fae/#ff344f/#e8e2c8` | in: CCW; a cooling chilly near (R57, R64). Out: CW |
| `mild` (default) Mild | 20 degrees C design | existing `steady` | default |
| `hot` Heiß | a high reading | `warmth` scaleY 1.8 (a long red segment), bulb flushed, faster `glow` overlay, 2 `drop` (paper) sweat; tint `#d8663f/#ff344f/#e8e2c8` | in: CW; hot neighbours (R25, R56, R57, R61). Out: CCW; 20 s |

**2 Tricks**
- `self-reading` Eigenmessung (L2): existing `reading` 3.2 s: the column climbs to the top and turns red while it fans itself, drops to a blue stub while it shivers, then settles; emit 6 `dot` (ink) popping along the tick marks. Leaves `mild`.
- `tap-check` Klopfprobe (L3): existing `tap` 2.0 s: three taps at the base of the tube, the column jiggles, then a `star4` (accent) tick. `proud`.
- `warm-read` Hochmessen (CW, steps up) and `cool-read` Runtermessen (CCW, steps down): the column rises or falls by a third, the bulb flushes or pales, 1.4 s per step.
- `measure` Messen (AUTO(curious), DUET any neighbour): 2.8s once: turns to the nearest pet, holds its tube out; the column goes to the neighbour's thermal class (§7, R57): **hot** (sunny `blazing`, servy `hot`, kettly `boiling`, radiatory `hot`, roofy `sun-baked`, boily `roaring`), **cold** (chilly `cooling` or `frosted`, cloudy `raining`, radiatory `cold`, roofy `snow-capped`), otherwise **mild**; emit 4 `dot` (ink). Hot partners are `proud`, cold ones `grumpy`.
- Shake outcome: `slosh`: the column sloshes red, `scared`; afterwards it taps itself ("still accurate").

**3 Purr** `steady-hum` 2.6s loop: the column breathes .8 <-> 1.0 about the middle, the bulb pulses +-2 %, the tube leans +-2, arms rest; emit a faint `ring` (paper) from the bulb every 1.3 s.

**4 Moods** lean: `curious`, `scared` (nervous at extremes), `proud` (precise).

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `thread-gun`: the tube tip is the muzzle; it shoots a hook on a fine thread (ink 1 px line, accent hook 3 px) over the edge and reels itself up hand over hand at 40 px/s |
| Down | `chart-chute`: a sheet of graph paper (+prop `chart`: a `paper` arc canopy 24 px wide with ink tick marks) as the parachute; 26 px/s |
| Dangle | grab at the top of the tube (thermometers hang from a loop): hangs, the column slides to the lower end by gravity (the reading flips), arms flail |
| Dropped | above 60 px the chart-chute, otherwise `boing`; it taps itself to check for cracks; `proud` ("still accurate") |
| Refuses | climbing the card side and ladders (a long thin glass tube is fragile; the mercury would split) |

**6 Topic mischief** Items: H2 `sfh-2000s`; C2 `attic-flat` and the task; P3 `boil-water`; the topics heating and cooling. Style: **measures**: holds its tube against the chip, then flicks it with the scale tip, 8 px; the chip gets a neutral 2 px outline for 2 s whose colour is seed-drawn, not a temperature (so no value is hinted). Taken back: `fluster`, then it reads `mild` again.

### 20. servy: Servy, the server rack / Serverschrank

Rolls at 20 px/s; 46 x 56; energy .80, sociability .30, curiosity .85.
**Rig.** Bones `root` (`caster-back`, `caster-front` at x +-9), `body` > `cable`, `arm-left`, `arm-right` (rest 20 / -20), `heat-back`, `heat-front`, `led-1..4` (x 10, y -3 to 20.6); parts casters (ink) with hubs, `cable` (ink path), `plug` (accent), arms, two heat strokes (detail), `cabinet`, four shelves, four slots, LEDs (three accent, `led-4` detail); eyes at y -17.5. Existing clips: `hum` (idle), `roll`, `count`, `overheat` (fidgets), `ping` (greet), `sync` (cuddle), `glitch`, `throttle` (sulk), `standby`, `touchdown`.
**Physics hooks.** 1,000 W/m2 and 8,000 kWh/(m2·a) in the data centre: every watt of IT power turns into heat, around the clock; it needs a chiller and filtered moving air.

**1 States** (`ladder`: idle < busy < hot; `throttled` on the way down)

| id (en / de) | Physics | Look | In / out |
|---|---|---|---|
| `idle` (default) Leerlauf | low load | existing `hum`, LEDs blink slowly | default |
| `busy` Rechnet | high load, more heat | +loop `crunch` 0.8s: LED waves faster (led-1..3 scaleY, lag .2), cabinet x +-.3, cable sways; mood `curious` | in: CW, `led-wave`. Out: 20 s |
| `hot` Überhitzt | the cooling is not keeping up | `led-4` (detail, red) scale 1.4 and stays on, `heat-back/front` waves rising, 2 `drop` (detail) sweat; tint `#5a5a64/#ffb000/#ff344f` | in: CW; a boiling kettly near (R20); the `overheat` trick. Out: chilly near (R55), venty near (R54), CCW |
| `throttled` Gedrosselt | the clock is cut to protect the chip | LEDs amber (tint accent `#fccf05`), body.y +1, tempo x.6, a sluggish roll; tint `#44666a/#fccf05/#ff344f` | in: CCW from `hot`. Out: cools to `idle` in 20 s |

**2 Tricks**
- `led-wave` LED-Welle (L2): 2.4s once: LEDs light in a wave `led-1 -> led-4` and back, three passes (derived from `count` and `sync`); emit 3 `dot` (accent) packets travelling right along the top edge. Leaves `busy` 10 s. (The owner's wave of LEDs.)
- `overheat` Überhitzen (L3): existing 2.8 s: `led-4` pulses red, two heat waves rise; emit 3 `drop` (detail) sweat and 2 `wave` (detail). Leaves `hot`, 8 s.
- `load-up` Last hochfahren (CW, steps up): 1.4 s per step; `idle -> busy -> hot`.
- `throttle-down` Drosseln (CCW, steps down): `hot -> throttled -> idle`, `busy -> idle`.
- `reboot` Neustart (AUTO(grumpy) after `hot`, and after a shake): 3.0s once: a yawn (mouth round), all LEDs off for 1 s (scale .1), then back one by one (led-1..4, 0.3 s apart) with a `ring` (accent) pulse. Leaves `idle`, mood `content`.
- `count` Binärzählen (AUTO(proud)): existing 2.6 s: LEDs count 0-7 while the arms type; emit `dot` bits. `ping` (existing greet) as a duet: a `dot` (accent) packet from the plug to the partner on an `arc`; the partner glows once.
- Shake outcome: `glitch` then `reboot`.

**3 Purr** `fan-hum` 2.0s loop: the LEDs breathe in staggered soft pulses (scaleY .8 <-> 1.1), the cabinet vibrates x +-.3 at 8 Hz, the cable sways +-3; emit one `dot` (accent) ping travelling right from `led-1` to the edge every 1.2 s.

**4 Moods** lean: `curious`, `grumpy` (always overheated), `proud` (perfectionist); it never sleeps (`standby` instead of `sleepy`).

**5 Gear and traversal**

| Move | How |
|---|---|
| Up | `cable-plug`: the cable tail (bone `cable`, scaleY 3) flies to the card edge and plugs in (the accent `plug`); the cable reels in and hoists the rack 18 px/s, casters idling. Alt `ramp`: a folding loading ramp (+prop `ramp`, a hinged rect 24 x 3, ink with an accent edge) unfolds from the casters for steps up to 36 px |
| Down | `cable-abseil`: the cable pays out through the plug, 16 px/s |
| Dangle | grab at the top of the cabinet: LEDs flick to red ("connection lost"), casters spin free, arms flail, the cable swings; mouth -.4 |
| Dropped | `glitch` and the `reboot` sequence (a data-loss gag); `grumpy` |
| Refuses | parachute ("a fall is a reboot"), ladders (casters cannot climb rungs), suction cups |

**6 Topic mischief** Items: C2 `data-centre` (and `hospital`, the next heaviest load). Style: **rolls into it**: a bumper nudge with LEDs flashing, 10 px; heat `wave`s rise off the chip for 3 s. Taken back: it throttles, then `reboot`s.

---

## 7. Chemistry: state- and mood-dependent interactions

### 7.1 How a rule works

- **Proximity.** `adj`: same perch or cards touching, within 90 px; `over`: A hovers above B (|dx| <= 28 px, A higher); `front`: A's picture overlaps B's middle; `touch`: the 6 px meeting gap of an encounter. A rule is checked every 0.5 s for each ordered pair in range; it has a `delay` (the state must have held that long, 0 unless noted), a `cooldown` of 20 s per pair, and acts only on pets that are on stage.
- **Effects** (any mix): (a) set B's or A's **state** (with that state's own `hold`); (b) set a **mood** (strength .6; the mood table of §A1 decides how long it shows); (c) **rapport** drift, same scale as the engine's +.05 greet, +.1 cuddle, -.15 squabble (here -.05 to +.1, never below the -.6 floor); (d) multiply the **encounter shares** `[greet, cuddle, squabble]` by 1.5 or 2 for 30 s (so "more likely" is a number); (e) queue a **duet trick** for the next encounter; (f) play a one-shot **vignette**.
- **Determinism.** Rules are ensemble data (schema-first), evaluated in a fixed order (ascending id); every random choice (which of two rules wins, how long a delay jitters) draws from the stage stream, so the same seed and events give the same frames in the TypeScript and Rust cores.
- **Budget.** One `blazing` sunny per stage at a time; a state caused by a rule never lasts longer than 40 s after the cause ends; a pair is never in two squabbles at once; a pet in `scared` ignores all rules except G4.
- **Reading the table.** `A` acts on `B`; "both" lists the moods; states are the ids of §S. The last column is the physics or the quiz line behind the rule.

### 7.2 Physical rules (R01-R64)

| ID | When A is ... | ... near B ... | Then | Why |
|---|---|---|---|---|
| R01 | cloudy `heavy` or `raining`, in `front` of | sunny (any lit state) | sunny -> `dim` and `grumpy`; cloudy `playful`; rapport -.05 | irradiance falls from 1,000 to 100-300 W/m2 (bond -.4, the tease) |
| R02 | sunny `blazing` | solary (not `shaded`) | solary -> `peak`; both `happy`; rapport +.05 | 1,000 W/m2 is the standard test irradiance of the peak rating |
| R03 | cloudy `heavy`/`raining` in front of sunny | solary within 90 px | solary -> `shaded`, `sad`; a battery within 90 px comforts her (battery `content`, her `sad` ends 3 s sooner); rapport solary-cloudy -.05 | module output follows irradiance (bond -.7); the battery bridges the gap |
| R04 | sunny `shining` | solary | solary -> `generating`, `happy` | PV turns sunlight into electricity |
| R05 | sunny `blazing` for 30 s | cloudy `fluffy` | cloudy -> `heavy`; cloudy `curious`, sunny `proud` | sunshine drives convection, and convection builds towering clouds |
| R06 | solary `peak` for 40 s | alone | solary -> `hot`, `grumpy`; back to `generating` 20 s later | a hot module loses about 0.35 % per kelvin |
| R07 | cloudy `raining` | sunny `shining`/`blazing` within 90 px | duet `rainbow` once per 3 min; both `happy`; rapport +.05 | refraction of sunlight in the drops |
| R08 | cloudy `wispy`/`fluffy`, in front of | sunny | no state change; sunny `grumpy` 10 s, cloudy `playful` | a thin cloud lets ~90 % of the light through: only teasing |
| R09 | windy `rated` | cloudy | cloudy drifts x1.6 for 10 s and is `playful` | wind drives clouds (bond +.6) |
| R10 | cloudy plays `gust` or `bluster` towards | windy | windy -> `rated` for 8 s, `happy`; rapport +.05 | a gust spins the rotor |
| R11 | sunny `blazing` | windy `becalmed` | windy -> `turning`; windy `happy` ("thanks"); rapport +.05 | the Sun heats the air and drives the wind |
| R12 | windy `becalmed` | solary `generating`/`peak` | windy `sad`; solary comforts it (cuddle share x1.5); both `content` after 20 s | complementary renewables: sun by day and in summer, wind by night and in winter |
| R13 | cloudy `raining`, over/adj | flamy `burning`/`high` | flamy -> `snuffed`, `scared`; cloudy `sad` ("oops"); rapport -.1 | rain snuffs a flame (bond -.5) |
| R14 | cloudy `heavy`, or windy `rated`, adj | flamy | `heavy`: flamy -> `ember`; `rated`: flamy -> `high` then 30 % `snuffed`; flamy `scared` | gusts and damp air starve or blow out a flame |
| R15 | boily `burning`/`roaring` | flamy `snuffed`/`ember` | duet `relight`: flamy -> `burning`, `happy`; boily `proud`; rapport +.05 | the same family of burners (bond +.5) |
| R16 | kettly `heating`/`boiling` | flamy `snuffed` | kettly flashes its lamp; flamy -> `burning`; kettly `proud` and sheepish | an electric spark relights a wick (the ticket's vignette 6) |
| R17 | kettly `boiling` | flamy any | duet `boast`: flamy `sad`, kettly `proud`; rapport -.05 | 2 kW against 35 W: 57 times (bond -.3) |
| R18 | kettly `heating`/`boiling` | battery any | duet `power-vs-energy`: both `grumpy` 10 s; rapport -.05 | a rate against an amount: a 2 kW kettle empties a 15 Wh phone charge in under a minute |
| R19 | kettly `boiling` | cloudy `wispy`/`fluffy` | cloudy one step up the ladder; both `happy`; rapport +.05 | the kettle's steam looks like a baby cloud (bond +.3) |
| R20 | kettly `boiling` | servy `busy`/`hot` | servy -> `hot`; both `content`; cuddle share x1.5 | fellow heaters: every watt ends as heat (bond +.3) |
| R21 | radiatory `warm`/`hot` | windowy `open` | radiatory `grumpy` ("heating the street") and huffs; windowy `playful`; squabble share x1.5; rapport -.05 | an open window throws away what the radiator delivers |
| R22 | radiatory `warm`/`hot` | windowy `tilted` | radiatory mildly `grumpy`; windowy closes to `shut` within 6 s | a tilted window still leaks |
| R23 | radiatory `cold` | windowy `shut` | windowy -> `fogged`, `sad` | warm moist air condenses on cold glass |
| R24 | radiatory `warm` | windowy `shut`/`fogged` | `fogged` -> `shut`; windowy `happy`; radiatory `content` | a radiator under the window warms the pane |
| R25 | radiatory `hot` | thermy | thermy -> `hot` and flusters; radiatory `proud` | the thermometer reads the heat it keeps on the 20 degrees C line |
| R26 | windowy `open` for 12 s | housy `cosy`/`wrapped` | housy -> `cold` (`cosy`) or stays (`wrapped`), shivers; housy `sad`; windowy `playful` | ventilation heat loss |
| R27 | windowy `open`/`tilted` | venty `nominal`/`boost` | venty `grumpy` and fusses; windowy `sad` ("I was only airing"); rapport -.05 | window ventilation loses the heat the unit recovers (75 %; bond -.4) |
| R28 | waly any | windowy any, adj | squabble share x1.5; the duet `u-value-duel` once per 3 min | 1.3 against 0.28 W/(m2·K): 4.6 times (bond -.5) |
| R29 | boily `roaring` | radiatory `cold` | radiatory -> `warm`; both `content`; rapport +.05 | an old couple: radiators at boiler flow temperatures |
| R30 | radiatory `warm`/`hot` | housy `cold` | housy -> `cosy`, `happy`; radiatory `proud` | the radiator delivers the heating load |
| R31 | housy `wrapped` | boily any | boily -> `pilot`, `sad` ("0.8"); rapport -.05 | insulation shrinks the boiler's job (303 -> 15 kWh/(m2·a)) |
| R32 | chilly `cooling`/`frosted` | radiatory `warm`/`hot` | squabble share x2 for 30 s; both `grumpy` after a squabble; rapport -.1 | heating and cooling at once wastes energy (bond -.7) |
| R33 | thermy within 120 px of both | chilly and radiatory squabbling | the squabble becomes a greet; both `content`; thermy `proud` | thermy takes orders from both lines, 20 and 26 degrees C |
| R34 | sunny `blazing` | chilly, no `lowered` shady within 120 px | chilly -> `overloaded`, `grumpy`; sunny `proud` | solar gains are the chiller's workload (50-100 W/m2; bond -.4) |
| R35 | sunny `blazing` | shady `lowered` | shady `proud` and strikes a `pose`; sunny `grumpy` ("the sunglasses"); a chilly within 150 px -> `cooling` or `standby`, `happy` | 6 against 100 W/m2; the blind blocks the Sun's gains (bond -.5) |
| R36 | sunny `blazing` | housy | cooling scene: housy -> `hot`, `sad`; heating scene: housy `happy` | summer gains overheat, winter gains heat |
| R37 | sunny `blazing` | roofy | roofy -> `sun-baked`, `grumpy`; sunny `proud` | the roof skin reaches 60-70 degrees C |
| R38 | shady `lowered` | solary | solary -> `shaded`, `sad`; shady raises its blind for 5 s ("sorry"); rapport -.05 | a shadow on a module drops its output (bond -.3) |
| R39 | shady `lowered`/`tilted` | windowy `shut` | windowy `proud`/`content`; rapport +.05 | the shading sits on the window's head and spares the glass (bond +.6) |
| R40 | cloudy (any), over/adj | shady `lowered` | shady -> `raised`, `sleepy` | a cloud is free shade: a day off for the blind (bond +.4) |
| R41 | insuly `tuck-in` | housy `cosy`/`cold` | housy -> `wrapped`; both `proud`; rapport +.1 | insulation is the house's coat (bond +.6) |
| R42 | insuly `tuck-in` | waly `bare`/`warm` | waly -> `wrapped` (a second tuck-in by a `thick` insuly: `passive`); both `proud`; rapport +.1 | U 1.0 -> 0.28 -> 0.15 W/(m2·K) (bond +.9) |
| R43 | insuly `tuck-in` | roofy any | roofy `proud`, `sun-baked` -> `dry`; rapport +.1 | 35 cm give U .10 instead of 2.1 (bond +.8) |
| R44 | cloudy `raining`, over | insuly any; roofy `dry` | insuly -> `soaked`, `sad`; roofy -> `wet` | wet insulation insulates badly; roofs get wet |
| R45 | roofy `dry`/`wet`, over/adj | insuly | roofy shelters it: insuly stays `fluffy`, `happy`; roofy `proud` | the roof keeps the insulation dry |
| R46 | radiatory `hot`, or sunny, for 20 s | waly | waly -> `warm`, `content` | thermal mass stores heat (bond +.3) |
| R47 | sunny `shining`/`blazing` | windowy `shut` | heating scene: windowy `happy`; cooling scene: windowy `scared` and squints | winter solar gains help, summer ones hurt (bond +.2 flips) |
| R48 | pumpy `heating` | radiatory | duet `flow-temperature`: radiatory `warm` -> `hot` cheaply; both `happy`; pumpy `proud`; rapport +.1 | low flow temperatures with a heat pump (bond +.9) |
| R49 | pumpy any | boily any, adj | squabble share x2; duet `efficiency-bars`; both `grumpy` | seasonal performance 3 against efficiency .8-.9 (bond -.6) |
| R50 | pumpy `heating` | solary `peak` | pumpy `proud`, fan quieter; solary `happy`; rapport +.05 | the PV roof feeds the heat pump (plus-energy house) |
| R51 | pumpy `heating` | insuly `thick`, or housy `wrapped` | pumpy -> quieter, `proud` | a deep retrofit cuts the purchase from 122 to 23 kWh/(m2·a) |
| R52 | pumpy `cooling` | chilly `cooling` | siblings: both `content`, they mirror each other's fan and emblem; cuddle share x1.5 | a heat pump is a chiller run backwards |
| R53 | venty any | pumpy `heating` | both hum in step (a cuddle); venty `proud`; rapport +.05 | the compact passive-house unit |
| R54 | servy `hot`, or thermy `hot`, adj | venty | venty -> `boost`; servy -> `busy` (relieved) | server halls live on moving, filtered air; a hot room is purged |
| R55 | servy `hot` | chilly `cooling` | chilly -> `overloaded` (works harder); servy -> `busy`; both `content` after 20 s; rapport +.05 | the chiller works around the clock for the data centre (bond +.6) |
| R56 | servy `hot` | thermy | thermy -> `hot`, flusters; servy `grumpy` ("stop looking") | the rack runs hot and the thermometer fusses (bond -.3) |
| R57 | thermy (always, every 2 s) | the hottest or coldest pet within 100 px | thermy mirrors its **thermal class**: hot (sunny `blazing`, servy `hot`, kettly `boiling`, radiatory `hot`, roofy `sun-baked`, boily `roaring`, flamy `high`), cold (chilly `cooling`/`frosted`, cloudy `raining`, radiatory `cold`, roofy `snow-capped`), else mild; the hot partner turns `proud`, the cold one `grumpy` | a thermometer reads what is around it |
| R58 | solary `peak`, or windy `rated`, adj | battery `empty`/`charged` | battery gains a bar every 6 s -> `charging` -> `full`; both `happy`; rapport +.05 | the battery stores the surplus (bonds +.6, +.5) |
| R59 | solary `peak` | battery `full` | solary `sad` (curtailed), battery `proud` | a full store takes no more |
| R60 | sunny any | flamy | flamy's `look-up` odds x3, flamy `happy` | the smallest and the largest of the powers (bond +.5) |
| R61 | sunny `blazing` | thermy | thermy -> `hot`; squabble share x1.5 | the Sun drives the mercury up (bond -.3) |
| R62 | chilly `frosted` | roofy | roofy -> `snow-capped`, shivers; chilly `proud` | cold air and snow |
| R63 | chilly `cooling`/`frosted` | cloudy `fluffy`/`heavy` | duet `snow`; chilly `happy` | overcast days lower the cooling load (bond +.3) |
| R64 | chilly `cooling` | thermy | thermy -> `cold`; chilly `proud`; rapport +.05 | the chiller takes orders from the 26 degrees C line (bond +.4) |

### 7.3 Mood rules for every pair (G1-G6)

| ID | When | Then |
|---|---|---|
| G1 | a `grumpy` pet and another `grumpy` pet, adj | the squabble share rises further (their mean bias is already -.25); both stay `grumpy` +10 s; a third pet in `content` (thermy is the natural mediator, R33) turns it into a greet |
| G2 | a `happy` or `playful` pet, adj to any pet | the neighbour's mood strength moves towards `happy` by .1 per 5 s; greet and cuddle shares x1.3 |
| G3 | a `sad` pet and a friend (affinity >= .4) in `content` or better within 90 px | the friend walks over and cuddles (cuddle share x2); the `sad` ends 20 s sooner |
| G4 | a `scared` pet and a sheltering pet (housy, roofy, waly or insuly) within 90 px | the shelterer plays its `shelter`/`lean-on`/`snuggle` clip over it; the scared pet calms to `content` in 4 s |
| G5 | a `sleepy` pet next to a purring pet | it falls asleep within 10 s; `sleepy` spreads to the purrer after 20 s |
| G6 | a `proud` pet next to a `curious` one | the proud pet repeats its last trick for it (an audience); both `happy` |

### 7.4 Vignettes the rules assemble

1. **Eclipse.** A raining cloudy slides in front of sunny: R01 (sunny `dim`, `grumpy`), R03 (solary `shaded`, battery comforts), R13 if flamy stands by (snuffed), R15/R16 (boily or kettly relights), then R07 (the rainbow when the cloud thins).
2. **Heat-loss squabble.** Windowy `open` beside waly and housy: R26 (housy `cold`), R28 (the U-value duel), R42 (insuly wraps waly), R39 (shady stands in as peace offering), R21 (radiatory huffs).
3. **The temperature dispute.** Radiatory and chilly in one cast: R32, R33 (thermy mediates), R34/R35 (the sun and the sunglasses decide who is right).
4. **Retrofit.** Insuly tucks in housy and waly: R41, R42, R31 (boily sad), R51 and R48 (pumpy quiet and proud, radiatory warm cheaply).
5. **Power or energy.** Kettly boils beside battery and flamy: R17, R18, then R16 (kettly relights flamy sheepishly).
6. **Renewables day.** Blazing sunny, solary, battery, windy `rated`: R02, R58, R59, R11, R12, R50.

### 7.5 Cost note

The rules are data (one row each); the only art they need is already in §S (the `rainbow`, `relight`, `boast`, `u-value-duel`, `power-vs-energy`, `flow-temperature`, `efficiency-bars`, `snow`, `sun-block`, `measure` and `carry-pv` duets are species tricks). Checking them needs a stage test per rule (a fixture with two pets and the expected states) and a Python or Rust oracle, as the repo's rules require.

---

## 8. Click escalation

### 8.1 The shared scheme

A **streak** is a series of clicks on one pet, each at most 4 s after the one before. Pointer extras are decoration (see 0.1): `still`, reduced motion and forced colours switch them off.

| Click | What happens | Mood |
|---|---|---|
| L1 | **hello**: the existing `greet` towards the pointer (the species' own greet clip); a fresh pet that was `sad` or `sleepy` wakes up | `happy` 20 s |
| L2 | **trick 1**, the signature of the species (what the thing does: shine, rain, whistle, LED wave); plays once; the result state of §S applies | `playful` |
| L3 | **trick 2**, the second signature (a different effect: prominence, thunder, bleed, tap check) | `playful`, or `proud` if it changes a state |
| L4 | **purr** starts (the species' `purr` loop, §S 3); the pet leans into the pointer (rotation 4 degrees) | `content` becoming `happy` |
| L5, L6 | each click extends the purr by 5 s (up to 20 s); at L6 the pet closes its eyes (lid .55) | `happy` |
| L7+ in the streak | an **encore**: with probability 1/3 a ladder trick (the species' trick 3 or 4, whichever has a legal step), otherwise the purr is extended | `proud` |
| a pause of more than 4 s | the streak ends; the next click starts over at L1 but the pet remembers it has been petted (a purr starts one click earlier for 2 min, "rapport with the learner", ephemeral and local) | |

- **Circling and stroking** bypass the count: `CW` / `CCW` play trick 3 / 4 as ladder steps at any time (they take priority over a running streak), `STR` starts the purr at once and can run alongside the streak.
- **Mash.** More than 5 clicks within 2 s (or 8 within 3 s) is a mash. Level 1: the pet plays its **mash reaction** for 3 s (below), sets `grumpy` (or `scared`, per table) and ignores clicks for a **12 s cooldown** during which a click only makes it shake its head (body.rotation +-5, twice). Level 2 (a second mash in the cooldown): it turns away, plays `sulk` and leaves to a perch at least 80 px away for 10 s. Nothing is ever lost, no pet leaves the stage.
- **Dizzy** is one shared overlay (no species art): 3 `star4` (paper) orbit 14 px above the eye bone for 3 s while the body wobbles +-6 degrees; used by every species whose mash reaction is `dizzy`.
- **Refractory.** After a trick, 0.6 s of no new trick (a click then queues exactly one).
- **State gating.** A trick that needs a state or a neighbour (`measure`, `snow`, `carry-pv`) is skipped for the next legal one in order; L2/L3 never fall through to nothing.

### 8.2 Per-species order

L2 and L3 are the signature tricks (trick 1 and 2); `CW`/`CCW` are the ladder steps; `Shake` is the outcome when the pet is held and shaken; `Mash` is the level-1 reaction; `Grab` is where `HOLD` takes the pet (and what swings).

| Pet | L2 | L3 | CW (up) | CCW (down) | Shake | Mash | Grab |
|---|---|---|---|---|---|---|---|
| sunny | `corona` | `prominence` | `high-noon` | `sundown` | `spark-shower` | dizzy | tip of a long ray |
| cloudy | `rain` | `thunder` | `condense` | `evaporate` | `downpour` | grumpy | the `wisp` |
| housy | `energy-label` | `smoke-rings` | `pull-on-coat` | `air-out` | `rattle-roof` | grumpy | chimney |
| solary | `sun-track` | `cell-wave` | `power-up` | `duck` | `sparkle-shower` | dizzy | top frame corner |
| radiatory | `bleed` | `glow-wave` | `turn-up` | `turn-down` | `gurgle` | grumpy | the `knob` |
| pumpy | `three-for-one` | `reverse-cycle` | `ramp-up` | `whisper` | `rattle` | dizzy | the `outlet` |
| windowy | `crack-open` | `fog-draw` | `swing-wide` | `close-up` | `rattle` | scared | top of a sash |
| waly | `brick-swap` | `layer-reveal` | `warm-up` | `cool-down` | `crumble` | grumpy | the `cap` |
| battery | `bar-count` | `zap` | `charge-up` | `discharge` | `slosh` | dizzy | the `cap` |
| windy | `full-power` | `yaw-turn` | `spin-up` | `wind-down` | `dizzy` | dizzy | the hub |
| boily | `whoomp` | `smoke-ring` | `fire-up` | `bank-down` | `cough` | grumpy | the flue cap |
| roofy | `tile-flip` | `tile-wave` | `sun-bake` | `shake-off` | `clatter` | scared | the chimney |
| insuly | `fluff-burst` | `tuck-in` | `bulk-up` | `flatten` | `itch` | grumpy | a top-middle tuft |
| shady | `slat-wave` | `roll-down` | `lower` | `raise` | `clatter` | grumpy | the cord tassel |
| venty | `spin-up` | `heat-recovery` | `air-up` | `air-down` | `gasp` | dizzy | `duct-front` |
| chilly | `ice-crown` | `flurry` | `cool-down` | `thaw` | `snow-globe` | grumpy | the `crystals` |
| kettly | `boil` | `whistle` | `heat-up` | `cool-down` | `boil-over` | grumpy | the `handle` |
| flamy | `spark-dance` | `ember-whoosh` | `stoke` | `shield` | `blow-out` | scared | the cup rim |
| thermy | `self-reading` | `tap-check` | `warm-read` | `cool-read` | `slosh` | scared | top of the tube |
| servy | `led-wave` | `overheat` | `load-up` | `throttle-down` | `glitch` | dizzy | top of the cabinet |

Dizzy species are the ones with something that spins (sunny's rays, solary's glints, pumpy's and venty's fans, windy's rotor, battery's bars, servy's LEDs); `scared` ones are the fragile or timid (windowy, roofy, flamy, thermy); the rest are `grumpy`.

---

## 9. Build cost, and four art work packages

### 9.1 Scale and what is not counted

Cost points: new part or prop shape 1, new bone .5, new clip 2 (reusing an existing clip with a different overlay, rate or direction costs 0), emitter 1 (data only; shared shapes), tint .5 (three hex values). **M** is the minimum that delivers what the owner asked for: at least two visible states, three tricks including the signature one, a purr, the climb, descent and dangle of §5 and one push clip per pet. **N** is everything else in §S. Engine and schema work (states, tricks, purr, props, emitters, mood face, gestures, gear mechanics, chemistry rules, the mischief adapter, and the Rust twin of each) is not counted per species; its tests are the language-agnostic cases with oracle vectors the repo's rules require.

Shared art kits (counted once, in work package B): the **hook and line** (a 4 px hook, a 1 px ink line) used by housy, boily, thermy, servy and chilly; a **ladder** (two ink rails, rungs every 6 px) in two variants (iron, scaffold) used by radiatory, waly and roofy; the **canopy** (an arc path 24-28 px with strings) with five skins (awning, snowflake, chart, lid, roof); the **meter bar** glyph (rect 4 x 20) used by waly, battery, boily; the **wool coat and blanket** (the two kit paints `wool` and `wool-light`) used by insuly, housy, waly. Estimated 9 points.

### 9.2 Per species: what is added

(M = must-have, N = nice-to-have. Names are the ids used in §S. "tints" counts state palettes.)

| # | Pet | M: new parts / props | M: new bones | M: new clips | M: emitters | M: tints | M pts | N: further items | N pts |
|---|---|---|---|---|---|---|---|---|---:|
| 1 | sunny | `corona` | none | 8: `blaze`, `low`, `corona-burst`, `prominence`, `glow-hum`, `dangle-ray`, `sink`, `push-beam` | 4: heat waves, ray streaks, prominence arc, glow dot | 3 | 22.5 | bones `cheek-left/right`; clips `sundown`, `float-rise`, `spark-shower`; emitters horizon rings, rainbow arcs, star shower | 10 |
| 2 | cloudy | none | none | 10: `thin`, `sag`, `wring`, `rain`, `thunder`, `gust`, `soft-patter`, `dangle-wisp`, `fog-sink`, `push-blow` | 6: rain with splash, bolt, vapour dots, flakes, gust streaks, virga drop | 3 | 27.5 | clip `downpour` | 2 |
| 3 | housy | `coat`, `label`, `icicle`, `downpipe`, `hook` | `downpipe`, `hook` | 10: `chill`, `fan`, `energy-label`, `pull-on-coat`, `air-out`, `chimney-content`, `cannon-climb`, `downpipe-slide`, `dangle-chimney`, `push-shoulder` | 4: breath puff, sweat, smoke ring, dust | 2 | 31 | prop `ladder`; clips `heat-loss`, `roof-ladder-climb`, `rattle-roof`; emitter heat waves | 8 |
| 4 | solary | three `cell-row` parts, two `hook` parts | `cell-row-1..3`, `hook-left/right` | 7: `peak`, `cell-wave`, `inverter-hum`, `hook-climb`, `wing-glide`, `dangle-frame`, `push-mirror` | 2: sparkle stars, sweat | 3 | 25 | clips `power-up`, `duck`, `feed-battery`, `sparkle-shower`; emitter packet arc | 9 |
| 5 | radiatory | `ladder` | `ladder` | 8: `cold`, `glow`, `bleed`, `convection-hum`, `ladder-climb`, `ladder-slide`, `dangle-knob`, `push-warm` | 3: shimmer waves, bubbles, warm dot | 2 | 21.5 | prop `towel`; clips `towel-dry`, `gurgle` | 5 |
| 6 | pumpy | `frost` | none | 8: `reverse`, `defrost`, `reverse-cycle`, `compressor-purr`, `pipe-vault`, `fan-brake`, `dangle-outlet`, `push-blow` | 3: electricity and flow streaks, outlet rings, steam puffs | 3 | 21.5 | clips `defrost-shake`, `flow-temperature`; emitter ice chips | 5 |
| 7 | windowy | `doodle`, two `cup` parts | `cup-left/right` | 8: `tilt`, `wide`, `fog-draw`, `glass-hum`, `cup-climb`, `sash-glide`, `dangle-sash`, `push-sash` | 2: draught streaks, fog drops | 1 | 22.5 | clip `glaze-show`, three hairline parts; emitter star | 6 |
| 8 | waly | `coat` (two layers), `section` (three stripes), `ladder` | `ladder`, `section` | 6: `layer-reveal`, `warm-up`, `thermal-hum`, `ladder-climb`, `dangle-cap`, `push-shoulder` | 2: heat waves, brick dust | 1 | 21.5 | clips `u-value-duel`, `crumble`; `meter` prop; brick-step clip | 7 |
| 9 | battery | two `clamp` parts, `coil` | `clamp-left/right`, `coil` | 6: `fill`, `clamp-climb`, `lead-bungee`, `trickle-charge`, `dangle-cap`, `push-bump` | 3: zigzag spark, orbit dots, climbing dot | 3 | 21 | clips `power-vs-energy`, `slosh`; `meter` prop | 5 |
| 10 | windy | none | none | 8: `rated`, `feathered`, `yaw-turn`, `blade-hum`, `propeller-lift`, `autorotation`, `dangle-hub`, `push-blow` | 3: wind streaks, hub ring, trail dots | 3 | 20.5 | clips `gust-ride`, `pinwheel` | 4 |
| 11 | boily | `chain`, `hook` | `hook` | 5: `roar`, `pilot-purr`, `chain-hoist`, `dangle-flue`, `push-belly` | 3: embers, smoke rings, bubbles | 2 | 16.5 | clips `efficiency-bars`, `relight`, `cough`; `meter` prop | 7 |
| 12 | roofy | `snow`, hooked `ladder` | `ladder` | 8: `snow-shiver`, `bake`, `tile-flip`, `tile-purr`, `ladder-climb`, `umbrella-sink`, `dangle-chimney`, `push-tile` | 3: heat and sweat, drips, flakes | 2 | 22.5 | clips `carry-pv`, `shake-off` | 4 |
| 13 | insuly | `batt2`, `blanket` | none | 8: `squash`, `tuck-in` (with blanket), `dry-shake`, `fibre-cling-climb`, `dandelion-sink`, `dangle-tuft`, `fibre-purr`, `push-tuft` | 2: fibre puffs, drops | 1 | 20.5 | clip `itch` | 2 |
| 14 | shady | `awning` (2), `shadow` | none | 7: `tilt-slats`, `raised`, `cord-haul`, `awning-sink`, `dangle-cord`, `cool-purr`, `push-shade` | 2: light shafts, lens glint | 0 | 19 | clips `sun-block`, `pose` | 4 |
| 15 | venty | `flap`, `filter` | none | 8: `low`, `boost`, `heat-recovery`, `quiet-fan`, `jet-lift`, `glide-brake`, `dangle-duct`, `push-blow` | 3: streaks, puffs, dust | 3 | 22.5 | clips `night-cool`, `filter-tap` | 4 |
| 16 | chilly | two `axe` parts, `ice-block`, `canopy` | `axe-left/right`, `canopy` | 6: `strain`, `ice-axe-climb`, `snowflake-chute`, `compressor-sigh`, `dangle-crystals`, `push-skate` | 3: flakes, ice chips, drips | 3 | 22 | clips `chill-out`, `skate-spin` | 4 |
| 17 | kettly | `tether` | none | 8: `heat`, `boiling`, `whistle`, `singing-kettle`, `steam-jet`, `lid-chute`, `dangle-handle`, `push-bump` | 3: puffs, whistle streaks, drops | 2 | 21 | clips `boast`, `pour`, `handle-hook`; prop `kettle-row` | 7 |
| 18 | flamy | `lantern` (2), `smoke` | `lantern` | 7: `high`, `snuffed`, `relight`, `candle-glow`, `lantern-lift`, `dangle-cup`, `push-warm` | 3: sparks, smoke, glow dots | 3 | 22 | clip `look-up` | 2 |
| 19 | thermy | `chart`, `hook` with thread | `hook` | 7: `hot`, `measure`, `steady-hum`, `thread-climb`, `chart-sink`, `dangle-tube`, `push-flick` | 3: tick pops, sweat, ring | 2 | 20.5 | clip `slosh` | 2 |
| 20 | servy | none | none | 9: `crunch`, `throttled`, `led-wave`, `fan-hum`, `cable-plug-climb`, `cable-abseil`, `dangle-cabinet`, `push-roll`, `reboot` | 3: packets, sweat, ring | 2 | 22 | prop `ramp` and its clip | 3 |
|  | **Sum** | | | | | | **443** | | **100** |

Notes: (a) many tricks cost 0 new clips because they re-key an existing fidget (`rain` is new, but `condense` is `billow`, and `tuck-in` is the existing `tuck-in`); (b) `dangle` could be one species-neutral pendulum for a first release (a rotation of `root` about the grab point plus the swing of one named bone), which removes 20 clips = 40 points from M at the price of generic motion; (c) housy and cloudy are the most expensive because they carry the most distinct states and props; (d) the points are my estimate from reading the rigs, not a measurement.

### 9.3 Four balanced work packages

| Package | Theme | Pets | M points | Why together |
|---|---|---|---:|---|
| **A: Sky and charge** | light, weather, wind, spark | sunny, cloudy, windy, flamy, battery | 113.5 | floaters and rotors; the particle-heavy pets (corona, rain, wind, flame, spark) share one emitter style; three of the five have no arms and their gear is mostly body motion |
| **B: Heat and steam** + the shared gear kit | heat shimmer, steam, readings | radiatory, pumpy, boily, kettly, thermy (+9 for the hook, ladder, canopy, meter and coat kits) | 101 + 9 = 110 | all five read as "warm things": shimmer, steam, tick marks and the same tint ramps; the kit is built here because the ladder, hook and canopy are first needed by radiatory, boily, thermy and kettly |
| **C: Envelope** | coats, tiles, slats, bricks | housy, waly, roofy, insuly, shady | 114.5 | one shared wool coat and ladder look; the wrapped states cross-reference each other (R41-R45), so one artist keeps them consistent |
| **D: Glass, air and cold** | glass, fans, frost, LEDs | windowy, solary, venty, chilly, servy | 114 | glints, fans, frost and LED rows are all small repeating details; the pets they talk to in the other packages (shady, sunny) are done by then |

Totals: A 113.5, B 110, C 114.5, D 114 (sum 452 = 443 + 9 for the kit; the spread is about 4 %). Suggested order: engine primitives and the shared mood face first (they unblock every package), then B's kit with one pet from each package as a pathfinder (sunny, radiatory, housy, windowy), then the rest in parallel. N items (100 points) are worth doing in the order: signature duets (`rainbow`, `relight`, `boast`, `flow-temperature`, `u-value-duel`, `carry-pv`), then shake outcomes, then the extra gear variants.

### 9.4 Risks and open decisions for the owner

1. **Pointer contract.** Everything beyond `poke` (counted clicks, circling, stroking, holding, shaking) needs pets to receive pointer input; today the layer never does and the `🐕️pet-walk` gate asserts pets are never the target of a click. The proposal keeps pets decorative (no information, no required use, off in `still`, reduced motion and forced colours); the gate and the product README need rewriting.
2. **Runs.** Mischief needs the layer to play while `[data-quiz-item]` chips are on screen, which is exactly when the quiz sets `quiet` (the run screen). Options: a new preference "Pets may play during a run" (default off), or a milder quiet.
3. **No leaks.** Every push is constant-strength and seed-directed (S.0); the thermometer outline colour is drawn from the seed, not from a value; venty does not blow harder for larger air change rates. Any new mischief needs the same review.
4. **Colour.** All state tints are authored hexes and have not been checked against the 3:1 contrast target of the earlier notes (`📓️explore-topic-pets.md` §6.2); the existing palette tooling should run over the 44 state tints before art starts. Battery's dark-theme window problem (O2 §3) is still open and gets worse with `empty`.
5. **Performance.** 12 live particles per pet and 40 per stage are proposals, not measured. The stage already asks the pacer for 64, 32 or 16 ticks per second and for 0 when nothing moves, so emitters must be able to run at 16 and must not keep a still stage awake.
6. **Scale of the chemistry.** 70 rules are 70 fixture tests; if that is too much, R01-R05, R13-R16, R21-R24, R32-R35, R41-R43, R48-R49, R54-R57 are the ones the learner will actually see in the four quiz scenes.

### 9.5 What this report did not check

No clip was played, no tint was contrast-tested, no rule was run on the stage, and the numbers in 9.2 are estimates from reading the rigs, not measurements. Physical values are quoted from the quiz files and the ticket notes; statements marked "general physics" (storm cut-out of turbines, wet insulation, defrost cycles, temperature coefficient of modules, snowflake drag) are textbook facts that no quiz item states.

