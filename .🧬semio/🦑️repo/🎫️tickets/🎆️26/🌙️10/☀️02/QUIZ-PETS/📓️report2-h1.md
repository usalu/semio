# 📓️ Work package H1 (second round): sky and charge — sunny, cloudy, windy, flamy, battery

Ticket `2026/10/02/QUIZ-PETS`, second round, written 2026-10-03. `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `OUT` = `TK/🗑️generated/h1`. CONTENT = `📓️explore2-species-content.md`.

**Status: done.** The five placeholders of A2 are replaced by real states, tricks, purrs, emitters, gear, grips, reaches, a canopy and every clip the new activities ask for. Only the five species documents were edited (Write/Edit), plus one new tool `TK/h1_measure.ts` and this report. No git command that modifies anything, no `bun install`, no cargo, no e2e gate, no server.

## 0. Decisions that hold for all five

1. **Chemistry ids kept exactly** (CONTENT §7): sunny `shining dim blazing sunset`, trick `rainbow`; cloudy `fluffy wispy heavy raining`, tricks `rain thunder gust snow`; windy `turning becalmed rated feathered`; flamy `burning ember high snuffed`, tricks `relight look-up`; battery `charged empty charging full`, trick `power-vs-energy`. The resting state (first) is CONTENT's default, not `resting`.
2. **Ladders through `from`** (`rungsOf`): every `circle`/`countercircle` trick lists the rungs bottom-up in `from` and has no `to`, so the resting state can sit in the middle of the ladder (sunny `sunset < dim < shining < blazing`). The authored order of `states` puts the resting state first and the step up from rest second, so even `ladderOf` steps up correctly from rest.
3. **Every state comes back**: each non-resting state `lasts` and names `then` (chains: sunset → dim → shining; heavy → raining → fluffy; snuffed → ember → burning; empty → charging → full → charged). Every state is reachable from rest by a cue (circle/countercircle/click/shake) and by `lasts` chains, chemistry aside.
4. **Tricks end on rest values and states show themselves through overlays and tints.** The engine does not yet say whether a trick's `to` takes effect at its start or its end (B2/B3 not landed: no stage module reads `stateAfterTrick` or a state clip yet). Clips that would only look right under one of the two orders (multiplying a hidden part back up against an overlay) were avoided.
5. **Props "hidden at rest by scale 0"**: impossible as such — bones have no rest scale and poses multiply, so a 0 in the idle loop would hide the part under every clip. The equivalent used: the prop is **authored tiny** (sunny's corona: unit-size star behind the disc; battery's clamps: drawn at 1/20 with stroke 0.05) and clips scale its bone up (×19…29, ×20). At rest it is invisible (sub-pixel, or behind the disc).
6. **Trick count**: CONTENT's must-haves plus chemistry make cloudy 6, flamy 7 and battery 6 tricks. Cued tricks stay ≤ 6; chemistry-only tricks have `cues: []` (sunny `rainbow`, cloudy `snow`, flamy `look-up`, battery `power-vs-energy`), state-bound ones have `from` (flamy `relight` only in `snuffed`, `blow-out` never from `snuffed`).
7. **Grip on the axis**: `grip` is a height, the pivot is `(0, −grip)`. So "held by the wisp" (cloudy) or "tip of a long ray" (sunny) became the top of the middle puff and the top ray tip, which lie on the axis.
8. **`reach`** = the measured right edge of greet/cuddle/squabble played over the idle loop beyond the half width (`OUT/measure.txt`), rounded up by ≈ 1 px for the gaze lean.
9. **Emitters**: shapes are authored for their motion (burst and drift particles point the way they travel, so streaks and bolts are drawn along +x; a bolt meant to point down in a burst is authored along −x); origins sit outside faces (twinkles, rainbow and heat were moved off the face after the first sheets). At most 8 particles alive per emitter; a pet runs one emitter at a time per source (state, trick, purr).

## 1. sunny (float, 46×51, grip 48 = top ray tip, reach 6, mood content, no gear)

- **Bones/parts added**: `corona` (12-spiked star, unit size, behind the disc; scaled ×17…29 by clips: spikes between the rays), `cheek-left`/`cheek-right` (the cheeks on their own bones, for blushing).
- **States**: `shining` (rest) · `blazing` Gleißend (tint #ffe45c/#ff7a00/#ff344f, overlay `blaze`: rays out, corona spikes pulse, short rays flicker; emitter `heat` waves; 30 s → shining) · `dim` Gedimmt (tint #d9c47a/#c98c3c/#d8788a, overlay `overcast`: lowered, rays shrunk, cheeks pale; 60 s → shining) · `sunset` Abendrot (tint #ff9a3c/#e0482a/#ffd0a0, overlay `low`: sunk to the horizon, rays flattened into a fan; 45 s → dim).
- **Tricks**: `corona` [click, whim] `corona-burst` + `rays` (radial drift streaks) · `prominence` [click, shake, show] + `plasma` (a fountain of red blobs arcing back) · `high-noon` [circle, rungs sunset/dim/shining/blazing] counter-spinning rays (±480°, symmetric end) + `rays` · `sundown` [countercircle] sink to the horizon + `afterglow` (flat ring of glow dots orbiting at the horizon) · `rainbow` [] `beam` + `rainbow` (a red-rimmed arch rising beside it; R07 duet).
- **Purr** `glow-hum` (two breaths per loop, rays in counter-phase, corona breathing, cheeks blushing) + `glow` (rising warm dots).
- **New activity clips**: `dangle-ray` (hang: stretched from the top ray, which stays at the grip; short rays flutter; flushed), `spin-tumble`, `sink` (fall: the floater sinking with folded rays), `float-rise` (hop), `push-beam` (leaning in, rays thrusting in two shoves), `daze` (rays out of phase), `shrug`, `sidle` (scoot).
- **Contrast** (against #f7f3e3 / #001117): palette body 1.34/12.87; blazing body 1.14/15.11; dim 1.56/11.10; sunset 1.90/9.10. The 2 px ink disc outline carries it on the light page as in round one.
- **Bothers me**: corona-burst and high-noon reach 6.8 px beyond the box (a flaring sun); the static sheet cannot show the counter-spin of high-noon. The rainbow is a single two-tone arch (one fill, one stroke), not three bands.

## 2. cloudy (float, 58×43, grip 37 = top of the middle puff, reach 7, mood content, no gear)

- **States**: `fluffy` (rest) · `wispy` Hauchdünn (tint #9fc0c6/#8fe3d8/#eef6f0, overlay `thin`: flatter, smaller puffs, longer wisp; 60 s → fluffy) · `heavy` Schwer (tint #46606a/#2fa89a/#8ea9a8, overlay `sag`: fatter, lower, a raindrop bead hangs from the belly; 40 s → raining) · `raining` Regnend (tint #3b5560/#2bc2b0/#9fb9b6, overlay `wring`: the middle puff wrung; emitter `drizzle`; 12 s → fluffy).
- **Tricks**: `rain` [click, shake] → raining: squeeze, the drop bone falls and splashes, `downpour` · `thunder` [click] → heavy: two puff flashes and a rumble shake, `bolt` (two lightning bolts under the belly) · `condense` [circle, rungs wispy/fluffy/heavy/raining] puffs inflate in turn, `vapour-in` rising from the perch · `evaporate` [countercircle] puffs shrink, wisp drifts off, `vapour-out` · `gust` [show] cheeks puff, blow, `gust` streaks forward (R10) · `snow` [] (R63) shiver and sag, `flakes`.
- **Purr** `soft-patter` (puffs breathe with a phase lag) + `virga` (one drop that fades before it lands).
- **New activity clips**: `dangle-top` (stretched from the top, puffs droop, a nervous bead hangs), `tumble-roll`, `fog-sink` (fall), `updraft` (hop), `push-shove` (leans in, front puff squashed in two shoves), `swirl` (dizzy), `shrug` (side puffs rise like shoulders), `sidle`.
- **Contrast**: palette body 3.06/5.65 (the only body ≥ 3:1 on both); wispy 1.74/9.91; heavy 6.01/2.88; raining 7.11/2.43 — the 4 px rim outline carries every tint.
- **Bothers me**: rain falls as a fan from one point under the belly (an emitter has one origin), so it reads a little like a leak; there is no splash (one emitter per source). Heavy and raining tints are close; the drizzle and the wringing tell them apart.

## 3. windy (walk, 48×56, grip 43 = top of the hub, reach 6.5, mood content, gear parachute)

- **States**: `turning` (rest) · `rated` Nennleistung (tint #9a9aff/#ffffff/#4c5756, overlay `rated`: rotor +360° per 0.9 s on top of the idle turn, blades stretched, head into the wind; emitter `wind`; 12 s → turning) · `becalmed` Flaute (tint #6a6ab0/#d8d4c2/#4c5756, overlay `still-air`: rotor stopped by cancelling the idle turn, blades drooping, head hanging; 45 s → turning) · `feathered` Sturmstellung (tint #5a5aa0/#d8d4c2/#4c5756, overlay `feather`: rotor stopped, blades edge-on (scaleX .3), crouched and leaning into the storm; emitter `gale`; 8 s → turning).
- **Tricks**: `full-power` [click, whim] → rated (existing `gust`, `gale`) · `yaw-turn` [click] the nacelle turns through edge-on and back (`wind`) · `spin-up` [circle, rungs becalmed/turning/rated/feathered: past rated the storm cut-out feathers it] rotor accelerates to 960° · `wind-down` [countercircle] rotor decelerates to a stop, head droops, `hum` squeak rings · `pinwheel` [whim, show] a twirl on its toes (head yaws twice, tower narrows), `trail` dots orbit it.
- **Purr** `blade-hum` (7.2 s: rotor at half speed, blades pulse in turn, nods) + `hum` ring.
- **New activity clips**: `dangle-hub` (rotor freewheels, legs paddle), `autorotate` (glide under the plain canopy), `push-lean` (tower leans 13…17°, back leg braced, rotor blowing), `wobble` (dizzy: rotor rocks back and forth), `shrug`, `sidestep` (scoot). `tumble` kept.
- **Contrast**: palette body 3.05/5.66; rated 2.24/7.73; becalmed 4.39/3.94 (≥ 3:1 on both); feathered 5.55/3.12 (≥ 3:1 on both). Outline carries the rest.
- **Bothers me**: a stopped rotor can only be made by cancelling the idle spin (−360° per 3.6 s). The sulk (`becalmed`) and cuddle (`nuzzle`) clips of round one cancel it too, so **sulking or cuddling while becalmed or feathered turns the rotor backwards at idle speed** (e.g. R12: solary comforting a becalmed windy). A clean fix needs the engine to let a state overlay replace the idle loop for a bone. `wind-down` ends with one lazy forward turn (a once clip must end on a multiple of 960° or 0). The plain canopy is drawn in windy's accent (cream), so on the light page it is outlines only.

## 4. flamy (walk, 28×46, grip 16 = cup rim, reach 5, mood curious, gear parachute + own canopy)

- **Gear decision**: CONTENT refuses the nylon parachute but proposes a sky lantern for going down; flamy owns `parachute` with a **paper-lantern canopy** (`M -11 0 L -11 -20 Q -11 -26 0 -26 Q 11 -26 11 -20 L 11 0 Z` + two ribs; vertical sides so the cords take the rim's lower corners). With grip 16 the lantern floats just above the flame tip — the flame fills it.
- **States**: `burning` (rest) · `ember` Glimmt (tint #b84a00/#e68a00/#8c8c84, overlay `smoulder`: flame .55, the face shrinks with it; 40 s → burning) · `high` Lodert (tint #ff7a00/#ffe45c/#c4c4b9, overlay `blaze-up`: flame ×1.3, tip curling; emitter `crackle`; 15 s → burning) · `snuffed` Ausgepustet (tint #5a5a54/#8c8c84/#c4c4b9, overlay `out`: a grey drooping wisp at .7 so the face stays readable; emitter `smoke`; 20 s → ember).
- **Tricks**: `spark-dance` [click, whim] (`flicker` + `sparks`) · `ember-whoosh` [click] → high (`ember` + `crackle`) · `stoke` [circle, rungs ember/burning/high] breath-in then flare · `shield` [countercircle] leans away and shrinks, smoke · `relight` [click, from snuffed] → burning: a pop with white `twinkle`s above the tip · `blow-out` [shake] → snuffed: whipped about, shrinks to a third, smoke · `look-up` [] (R60) on its toes, leaning up, twinkles.
- **Purr** `candle-glow` (flame breathes, core in anti-phase, tip sways, leaning in) + `warm-dot`.
- **New activity clips**: `dangle-cup` (flame sways against the swing, legs dangle), `lantern-ride` (glide: low guttering flame, dangling legs), `push-warm` (cup leans into it, flame trails back), `wobble`, `shrug`; scoot uses the existing `scoot` walk.
- **Contrast**: palette body 3.06/5.65; ember 4.70/3.68 (≥ 3:1 on both); high 2.35/7.36; snuffed 6.24/2.77. The 4 px flame rim carries the rest.
- **Bothers me**: `relight` under the snuffed tint pops a grey flame (the tint changes only with the state); `blow-out` regrows to rest before the snuffed overlay shrinks it again. Seven tricks (two are state-bound, one chemistry-only).

## 5. battery (walk, 42×52, grip 49 = cap, reach 8, mood playful, gear climb, ladder, grapple)

- **Bones/parts added**: `clamp-left`, `clamp-right` on the arm tips: jump-lead crocodile clamps (accent with ink), authored at 1/20 and scaled ×20 by the climbing clips; jaws bite by scaleY.
- **States**: `charged` (rest, overlay `three-bars`: bar 4 off) · `empty` Leer (tint #5a6b5f/#8c9a90/#a8a89e, overlay `flat`: one flickering bar, slumped, arms hanging, cap askew; 90 s → charging) · `charging` Lädt (tint accent #ffd23f, overlay `fill`: yellow bars fill bottom-up, a lightning bolt flashes on the cap; emitter `current`; 8 s → full) · `full` Voll (tint #2fb36c/#7dffcf/#e6fff4, overlay `brim`: bars swell, proud; emitter `star`: a sparkle burst on entering; 40 s → charged).
- **Tricks**: `bar-count` [click, whim, from charged/full] → full (`recharge` + `count-dots` in the window) · `zap` [click, show] → charging (`zap` + `bolt`s from the cap) · `charge-up` [circle, rungs empty/charged/full] bars fill in turn, cheer, `orbit` dots · `discharge` [countercircle] bars drain top-down, sag, `fallout` sparks · `slosh` [shake] → charged, bars slosh · `power-vs-energy` [] (R18) bars flick off fast, refill slowly, arms on hips.
- **Purr** `trickle-charge` (bars light in a rising pulse, rocking) + `trickle`.
- **New activity clips**: `clamp-climb` (clamps out, arms reach and bite in turn, legs kick, body against the wall), `clamp-mantle`, `clamp-slide`, `haul` (carry), `aim-gun`, `reel-in`, `push-bump` (a real hip bump: top away, hip into the element), `dangle-cap` (bars flicker, limbs flail), `tumble`, `daze`, `shrug` (arms up), `shuffle` (scoot).
- **Contrast**: palette body 3.07/5.62; empty 5.10/3.39 (≥ 3:1 on both); full 2.43/7.13; charging accent 1.30/13.31. Outline carries.
- **Bothers me**: the window is `ink`, cream on the dark page, so mint/yellow bars have ~1.8:1 there (O2's open point; a fourth palette colour or a `pupil` paint for parts would fix it). The coil (lead-bungee) of CONTENT was not built: sliding is done with the clamps. On the wall the casing still stands ~1 px off the line at some phases; the reel arms are short stubs (the rope hangs from the cap).

## 6. Validation and tests (real output)

```
$ bun TK/validate_species.ts <the five>          (OUT/validate.txt)
ok   🎓️teaching/🏛️architecture/🐾️pets/☀️sunny/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/☁️cloudy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/💨️windy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🕯️flamy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔋️battery/🔣️.json
validate exit 0

$ bun ./📜️script.ts test     (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript; OUT/site-test.txt)
 Test Files  5 passed (5)
      Tests  138 passed (138)
exit 0
```

`🐾️pet-cast` (ajv + product validation per document, the assembled menagerie, grounds, casts) passes with the five new documents. Numbers per clip, tint and emitter: `bun TK/h1_measure.ts <files>` → `OUT/measure.txt`.

During the work the preview tool failed once to bundle (`tintPalette` moved from the React `✨️effects` to `🖌️depiction` by another package); the entry was fixed by its owner minutes later and all sheets were re-rendered with the current product modules.

## 7. Sheets (`OUT`)

- **Final side-by-side of the five with their states**: `five-states-x1.png` (pets at page size, both themes) and `five-states-x2.png`.
- Tricks and purrs of all five: `five-tricks-x2-1.png` … `-4.png` (light).
- Per pet: `<id>-st-*.png` (states, tricks, purr; both themes) and `<id>-gear*.png` (hang, tumble, glide, aim, reel, climb, mantle, slide, carry, push, dizzy, shrug, scoot; cloudy and windy sheets also hold `fog-sink`/`updraft` and `pinwheel`).

## 8. For the engine packages

- Assumed: a state's overlay clip is layered on top like a second idle loop (additive, scales multiply) on the stage clock; trick `to` is applied when the trick ends. Both work with the art either way except the cosmetic points named above.
- The `fall` and `hop` repertoire of the floaters now have clips (`sink`, `fog-sink`, `float-rise`, `updraft`) for the floater's sinking and lane rise.
- No tool clip draws rope, hook, gun or ladder; battery's arms only point at where the renderer draws them.
