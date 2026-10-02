# 📓️ Report WP H4 — art: solary, battery, shady, venty, servy

Five species documents, hand-written with Write/Edit only, each starting with `"$schema": "../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json#/$defs/Species"`:

| Species | File | Size (w×h) | Gait | Speed | Temperament (energy / sociability / curiosity) |
|---|---|---|---|---|---|
| solary | `🎓️teaching/🏛️architecture/🐾️pets/🔆️solary/🔣️.json` | 46×52 | walk | 34 | 0.7 / 0.75 / 0.6 |
| battery | `🎓️teaching/🏛️architecture/🐾️pets/🔋️battery/🔣️.json` | 42×52 | walk | 46 | 0.9 / 0.8 / 0.6 |
| shady | `🎓️teaching/🏛️architecture/🐾️pets/😎️shady/🔣️.json` | 44×48 | walk | 22 | 0.25 / 0.45 / 0.3 |
| venty | `🎓️teaching/🏛️architecture/🐾️pets/🌬️venty/🔣️.json` | 44×36 | float, hover 6 | 28 | 0.5 / 0.6 / 0.7 |
| servy | `🎓️teaching/🏛️architecture/🐾️pets/🖥️servy/🔣️.json` | 46×56 | walk | 20 | 0.8 / 0.3 / 0.85 |

Directory code points verified with the `readdirSync` one-liner: `🔆️ 1f506 fe0f`, `🔋️ 1f50b fe0f`, `😎️ 1f60e fe0f`, `🌬️ 1f32c fe0f`, `🖥️ 1f5a5 fe0f`.

Contact sheets (final state): `🗑️generated/preview/wp-h4.png` (all five, every clip, scale 2), `🗑️generated/preview/wp-h4-rest.png` (rest poses side by side, both themes), `🗑️generated/preview/{solary,battery,shady,venty,servy}.png`. Intermediate close-ups and dense phase strips are in `🗑️generated/wp-h4/` (left for the ticket's clean-up).

## Conventions I applied to all five (worth knowing for the stage and the gallery)

- **Rest pose is the canonical look.** `frameOf` blends every activity in from the rest pose, and `still` shows the rest pose, so nothing is hidden by a scale-0 track that would have to be present in every clip. Effects that exist only in a fidget (solary's sparkle, battery's spark, venty's streaks and puff, servy's heat waves) are listed *before* the body part and sit *behind* it at rest; a clip moves them out, shrinks them to 0 outside, snaps them back while they are at scale 0 and lets them regrow hidden.
- **Non-looping clips start and end on rest values** (0 offsets, scale 1; venty's real fan spin ends on 720° ≡ rest), so there is no pop when the next activity blends in. Checked by script for every track.
- **Looping clips:** idle, gait, cuddle, squabble, sulk, sleep. Greet, the two fidgets and land play once.
- **Leans pivot at the feet** (`root.rotation`), squashes use `root.scaleX/scaleY`, so feet stay on the ground line. Shady squashes like a concertina instead (head rail down, slat stack compressed).
- Shoulders sit just outside the body outline so a raised arm is not swallowed by the body it is drawn behind.
- Faces are shifted 0.2…0.4 px to the right (solary tilts instead, venty's face is on its front half) for "facing right".

## Per pet

### solary — the solar panel
- Grounds: `physics/power-or-energy/pv-module-peak`, `physics/power-or-energy/pv-annual-yield`, `physics/powers/sunlight-square-metre`, `demand/standard-profiles/plus-energy-house`, `demand/final-energy/kfw-55-gas-solar`.
- Bones (9): `root`, `body` (hips), `leg-left`, `leg-right`, `panel` (hinge on the hips, rest −5°), `arm-left`, `arm-right`, `sparkle`, `glint` (all four on the panel). Eyes and mouth ride on `panel`, not `body`, because the panel is what tilts.
- Look: aluminium frame (`detail`), blue glass (`body`), 2×3 cell grid in aluminium, a pale glint in the lower back cell, grey hip block on two legs.
- Clips: `bask` (idle 3.2 s), `stroll` (walk 0.6 s, panel leans forward and nods), `sunbathe` (fidget: tilts 20° back towards the sun, arms up, glint pulses), `gleam` (fidget: the glint sweeps diagonally over the cells, a sparkle pops at the top front corner, small hop), `wave` (greet), `snuggle` (cuddle), `bicker` (squabble), `dim` (sulk: panel droops forward, a shiver every loop), `doze` (sleep), `touchdown` (land).
- Not fully satisfied: the mullion of the middle row is interrupted where the mouth sits (a grid line through the mouth looked stitched), so the middle row reads as 2 cells only by continuation. Mouth on the blue glass is ink on `#2c5fb0` (3.1:1 in the light theme) — thin but readable.

### battery — the battery
- Grounds: `physics/power-or-energy/{ev-battery, phone-charge, wallbox}`, `physics/powers/wallbox`, `physics/energies/{ev-battery, phone-charge, petrol-tank}`.
- Bones (12): `root`, `body`, `leg-left`, `leg-right`, `arm-left`, `arm-right`, `spark`, `cap`, `bar-1`…`bar-4` (each anchored at its base, bottom to top).
- Look: green capsule, grey terminal cap, an `ink` charge window with four mint bars.
- Clips: `hum` (idle 3 s; the top bar dips once per loop), `bounce` (walk 0.5 s, springy bob with squash), `recharge` (fidget: bars drain top-down, it sags with drooping arms, bars pop back bottom-up, it springs up), `zap` (fidget: cap rattles, a spark jumps out of the terminal and jitters above it), `wave` (greet with two hops), `snuggle`, `bicker`, `drained` (sulk: one flickering bar, slumped), `standby` (sleep: one bar, the second breathing), `touchdown`.
- Not fully satisfied: the window is `ink`, so in the dark theme it turns cream and the mint bars have only ≈1.8:1 against it (still clearly visible by hue; in the light theme it is the strong dark display). No palette colour gives a dark backdrop in both themes.

### shady — the external sun shading
- Grounds: `cooling/cooling-load-and-demand/{passive-house-home, new-home-geg, office-passive-house, office-geg-shading, attic-flat, office-1970s}`, `heating/u-values/roller-shutter-box`, `demand/standard-profiles/passive-house`.
- Bones (13): `root`, `leg-left`, `leg-right` (children of `root`, so they stay planted while the blind rolls), `body` (at the underside of the head rail), `blind` (slat stack, anchored at the rail), `slat-1`…`slat-6`, `cord`, `shades`.
- Look: grey head rail (roller-shutter box), six violet slats on an `ink` curtain, pull cord with tassel, black sunglasses with a grey rim. `face.above` = `rail`; the sunglasses parts come after it and sit over the eyes. At rest the glasses sit low on the nose, so the eyes (radius 3) peek over the rim and the gaze stays visible; clips push them up over the eyes (cool) or tip them.
- Clips: `lounge` (idle 3.6 s: concertina breathing, cord sways), `shuffle` (walk 0.6 s: slats rattle, cord trails), `slat-wave` (fidget: it tugs its cord and the slats tilt open and shut in a wave, top to bottom), `roll` (fidget: pulls the cord, the blind rolls down by 11 px, glasses go up over the eyes, springs back), `peek` (greet: leans in, tips the glasses, cord waves), `chill` (cuddle), `clatter` (squabble: glasses up, slats rattle, cord whips), `shut` (sulk: half rolled down, glasses over the eyes, leaning away), `siesta` (sleep: rolled down a little, glasses over the eyes), `touchdown` (concertina squash).
- No arms by design (the sheet lists none); the cord is its waving limb.
- Not fully satisfied: in `clatter` the opposing slat tilts (±2.2°) read as a slightly crooked blind for the half second of the loop; that is the joke, but the stories gallery should confirm it in motion. While a slat is tilted open under the mouth, the mouth is ink on ink in the light theme for a moment.

### venty — the ventilation unit
- Grounds: `cooling/air-change-rates` (whole task), `cooling/cooling-load-and-demand/office-passive-house`, `demand/standard-profiles` (the ventilation axis of the task), `demand/final-energy/{kfw-40-heat-pump, passive-house-direct-electric}`, `heating/heating-load-and-demand/{kfw-40, gruenderzeit-retrofit}`.
- Bones (11): `root`, `body`, `streak-upper`, `streak-lower`, `puff`, `jet-back`, `jet-middle`, `jet-front`, `duct-back`, `duct-front`, `fan`.
- Gait `float` with `hover` 6 as on the character sheet (Hover class); the repertoire maps `idle` → `hover` and `walk` → `glide`.
- Look: lime casing, a round fan window (pale mint) with a three-blade wheel on the back half, the face on the front half, two duct collars on top, three air jets underneath.
- Clips: `hover` (idle 3.2 s: bob, fan rocks ±30°, jets pulse in a travelling wave), `glide` (walk 0.7 s: leans forward, fan rocks fast, two air streaks trail from the back edge), `spin-up` (fidget: the fan really spins 0 → 720°, the unit rises on longer jets and vibrates), `exhale` (fidget: back duct sucks in, casing swells, a puff of air leaves the front duct and drifts off), `nod` (greet: polite bow, the front duct waves), `cosy` (cuddle), `fuss` (squabble: ducts huff alternately), `stale` (sulk: sinks, ducts droop), `settle` (sleep: sits down into the hover gap), `touchdown`.
- **Deviation to know:** the three jets are drawn *below* the origin (y 0.75…4.25), inside the 6 px hover gap, i.e. outside the dashed size box. That is intended (they are the air cushion) and never reaches the perch at rest; in `spin-up` they stretch to about the perch line.
- Palette note: the brief puts the cream `accent` on the fan blades; cream blades on the pale window were invisible, so the blades are `body` green on the `detail` window and the cream is the hub. Air streaks outside the body are `ink` (pale mint on cream cannot be seen); the puff is `detail` with an ink outline.
- Body height 36 (+6 hover) is below the 40 px guideline, as the sheet asks (class S).

### servy — the server rack
- Grounds: `cooling/cooling-load-and-demand/data-centre`.
- Bones (13): `root`, `caster-back`, `caster-front` (children of `root`), `body`, `cable`, `arm-left`, `arm-right`, `heat-back`, `heat-front`, `led-1`…`led-4`.
- Look: slate cabinet, face in the top bay, four drive bays with a slot and a status LED each (three green, the lowest red), two casters, a cable tail with a green plug.
- Clips: `hum` (idle 3.5 s: LEDs blink off one after the other, the red one swells once, cable sways), `roll` (walk 0.6 s: leans, rattles, casters rock, cable trails, activity LEDs), `count` (fidget: the three green LEDs count 0…7 in binary while the arms type, then a small hop), `overheat` (fidget: red LED pulses, two red heat waves rise from the top, the rack wobbles and fans itself), `ping` (greet: wave, LEDs flash twice, cable wags), `sync` (cuddle: LEDs pulse in a wave, cable wags), `glitch` (squabble), `throttle` (sulk: green LEDs off, red one blinks, slumped), `standby` (sleep: one LED breathing — it "never sleeps"), `touchdown`.
- Not fully satisfied: caster rotation is hardly visible at 1× (a 0.9 px hub dot); the roll reads through the lean, the rattle and the cable. Mouth is ink on `#406670` (3.1:1 in the light theme).

## Verification (real output)

```
$ bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/validate_species.ts" <the five files>
ok   🎓️teaching/🏛️architecture/🐾️pets/🔆️solary/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔋️battery/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/😎️shady/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌬️venty/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🖥️servy/🔣️.json
exit=0
```

- The owned validator accepts the `$schema` key.
- ajv (draft-07, `…/schema.json#/$defs/Species` from `🧬️schema/🔣️.json`): all five `ok` (`🗑️generated/wp-h4/ajv_check.mjs`).
- Own script over the five documents: every ground exists in the quiz files (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`), no duplicate tracks, keys ascend 0 → 1, loop seams closed, non-looping clips start and end at rest, ease x in [0, 1], every repertoire id resolves, no unused clip: `ok` for all five (10 clips each).
- By eye on both themes: rest, look left/right, blink, mirrored copy, every clip at 4 phases, and the signature fidgets at 10 phases (`🗑️generated/wp-h4/dense-*.png`).

## Open

- Nothing was judged in real motion (no stories gallery yet); timings and amplitudes are first values to tune there.
- No `hop` or `fall` clips (the brief does not ask for them); the body stays at rest while a walker hops between perches.
- The renderer occasionally failed with `Page.captureScreenshot: Unable to capture screenshot` when several agents rendered at once; a retry always worked.
