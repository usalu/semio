# 📓️ Report WP H1 — art: sunny, cloudy, windy, flamy, kettly

Five species documents, hand-written, each starting with `"$schema": "../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json#/$defs/Species"` and then the `Species` fields in the order of the TypeScript type.

| Pet | File | Size (w×h) | Gait | Speed | Hover | Temperament (energy / sociability / curiosity) |
|---|---|---|---|---|---|---|
| sunny | `🎓️teaching/🏛️architecture/🐾️pets/☀️sunny/🔣️.json` | 44×48 | float | 26 | 10 | 0.7 / 0.85 / 0.5 |
| cloudy | `🎓️teaching/🏛️architecture/🐾️pets/☁️cloudy/🔣️.json` | 58×40 | float | 16 | 14 | 0.35 / 0.6 / 0.7 |
| windy | `🎓️teaching/🏛️architecture/🐾️pets/💨️windy/🔣️.json` | 42×56 | walk | 28 | – | 0.9 / 0.55 / 0.6 |
| flamy | `🎓️teaching/🏛️architecture/🐾️pets/🕯️flamy/🔣️.json` | 28×45 | walk | 20 | – | 0.4 / 0.3 / 0.65 |
| kettly | `🎓️teaching/🏛️architecture/🐾️pets/🫖️kettly/🔣️.json` | 46×44 | walk | 30 | – | 0.85 / 0.65 / 0.45 |

Directory code points were checked with the `readdirSync` one-liner of the brief: `☀️sunny 2600 fe0f`, `☁️cloudy 2601 fe0f`, `💨️windy 1f4a8 fe0f`; flamy and kettly were written with the emoji copied from the brief and appear in the listing as `🕯️flamy`, `🫖️kettly`.

Sheets (tool output, all under `🗑️generated/preview/`):

- `wp-h1.png` — all five, every clip at four phases, light and dark (scale 2, very tall).
- `wp-h1-rest.png` — the five rest rows side by side (light, dark, gaze, mood, blink, mirrored).
- `wp-h1-family.png` — the five next to housy, radiatory, solary, battery, pumpy, waly (family resemblance, relative size).
- `sunny.png`, `cloudy.png`, `windy.png`, `flamy.png`, `kettly.png` — one full sheet per pet (scale 3).
- `wp-h1-engine-<id>.png` — frames taken from the real `🎪️stage` (`advance` + `frameOf`), four moments per activity; this is what the engine actually composes (idle loop underneath, clip on top).

## Things the lead should know first

1. **The stage layers clips differently from design §4.7.** `🎪️stage/🟦️.ts` (`poseOf`, `weightOf`, `replaces`) loops the *first idle clip underneath every activity*; `fidget`, `greet`, `cuddle`, `squabble`, `sulk` are **added on top** of it (offsets and rotations add, scales multiply), `walk`, `hop`, `fall`, `land`, `sleep` cross-fade it away; every clip fades in over 8 ticks and out over the last 8. Consequences I authored for (and the other art packages should check):
   - a non-looping clip must end on rest values, otherwise the fade-out sweeps back to rest in 0.125 s (a rotation held at 120° or 720° unwinds visibly);
   - a `land` clip must hold its squash for the first ~8 ticks, otherwise the fade-in eats it (all five `touchdown` clips hold until phase 0.3 of 0.4 s);
   - the idle loop must not contain motion an additive clip would have to cancel. This is why windy's rotor only rocks in idle: a rotor that turns continuously in idle keeps turning under the sulk (drooping blades going round) and cannot be stopped by a fidget.
2. **`greet`, `cuddle`, `squabble`, `sulk` are loops** in all five pets. These activities last 2…6 s (design §5.4); a one-shot clip would hold its last key for the rest of the span.
3. **`loop-seam` forbids a continuous rotation.** Windy's rotor spins in `tiptoe`, `whirl`, `leap`, `tumble` as a sawtooth: `0 → 0.999: n·120 − ε → 1: 0`. A looping clip is sampled at phases `k ÷ ticks` with `k < ticks` (`sampleClip`), so the reset between 0.999 and 1 is never sampled for clips up to 15 s. `gust` (not a loop) spins 720° and resets between two ticks (keys at 0.878 and 0.882; 2.4 s = 154 ticks, ticks 135 and 136 lie at 0.8766 and 0.8831). If `gust.seconds` is retuned, those two keys must be moved between two ticks again. Checked against the real sampler: largest visible rotor step per tick (modulo the 120° symmetry) is 3.2° (`tiptoe`), 11.3° (`gust`), 6.2° (`whirl`), 7.5° (`leap`), 4.6° (`tumble`) — no jump. I believe the rule itself is right; a "periodic value" notion (seam modulo a symmetry) would be the clean alternative if more rotating things appear.
4. **`$schema` is accepted** by the owned validator and by the JSON schema.
5. The three walkers also carry `hop` (`leap`) and `fall` (`tumble`) clips; the floaters do not (they fall back to their idle loop, as `clipsOf` does).
6. None of the five has arms. Rays, puffs, blades, the flame tip, lid, spout and handle do the gestures; the siblings' pets mostly have line arms. Legs are round-capped 3 px ink lines with a small foot pointing right (the facing cue), children of `root` so they stay planted while the body rocks; whole-pet hops and landing squashes are keyed on `root`.

## Per pet

### sunny — the sun (float)

- Bones: `root`, `body` (disc centre, 24 above the origin), `rays-long`, `rays-short` (rest rotation 30°) — both children of `body`.
- Parts: two ray rings (six rounded rays each, accent, 1.5 px ink), disc (body, 2 px ink), two cheeks (detail). Face on `body`, shifted 0.4 px to the right.
- Clips: `bob` (idle + walk, 3.2 s loop: bob, slight sway, the two rings pulse out of phase) · `ray-wave` (fidget, 1.8 s) · `ray-spin` (fidget, 3 s) · `beam` (greet, 1.8 s loop: two hops, head tilt, rays flare) · `bask` (cuddle, 2.4 s loop: leans in, long rays pulled in so it does not poke, short rays beat twice like a heart) · `flare` (squabble, 0.8 s loop: shakes, rays bristle) · `dim` (sulk, 3.2 s loop: sinks, rays shrink to nubs, sighs) · `doze` (sleep, 5 s loop: lower, tilted, rays small, slow breathing) · `touchdown` (land, 0.4 s).
- Fidgets: `ray-wave` — the long and the short ring swell alternately twice, a wave travelling around the disc. `ray-spin` — the rings counter-rotate by one ray pitch (60°), hold, and turn back.
- Not satisfied with: on the light theme the brand yellow carries only 1.34:1 against the page (as the brief says, it relies on the outline). No glow part (the schema has no opacity). `flare` and `ray-wave` leave the size box by up to 5 px.

### cloudy — the cloud (float)

- Bones: `root`, `drop` (child of `root`, hidden behind the belly at rest), `body`, `wisp`, `puff-left`, `puff-middle`, `puff-right` (children of `body`).
- Parts: raindrop (accent, drawn first), then the cloud twice: `…-rim` parts (fill body, 4 px ink stroke) and on top the same shapes without stroke. The fill pass hides the inner strokes, so the union of belly, three puffs and wisp shows one 2 px outline that stays closed while the puffs move. `shine` (detail) on the belly. Face on `body`, set 4 px right of centre under the big puff.
- Clips: `drift` (idle + walk, 3.6 s loop) · `drip` (fidget, 2.4 s) · `billow` (fidget, 2.4 s) · `puff-wave` (greet, 1.8 s loop: the right puff waves three times) · `snuggle` (cuddle, 2.4 s loop: leans in, fluffs up) · `bluster` (squabble, 0.9 s loop: winds up, thrusts forward, flattens) · `drizzle` (sulk, 3.2 s loop: sags, puffs deflate, one drop falls per loop) · `doze` (sleep, 5 s loop) · `touchdown` (land, 0.4 s).
- Fidgets: `drip` — the cloud wrings itself, a teal drop swells below the belly, falls the hover height, splashes flat and is gone (it travels back at scale 0.01 and regrows behind the belly). `billow` — the three puffs swell one after the other from back to front.
- Not satisfied with: widest of my five (58). The splash height is the hover of 14; if the hover is tuned, the `drop` y keys (20 / 23 / 24.4) must follow. When a sulk ends while the drop is in the air, the 8-tick fade pulls it back up into the cloud. The wisp is only a fourth small puff. No cheeks, so "puffs its cheeks and blows" became the body thrust of `bluster`.

### windy — the wind turbine (walk)

- Bones: `root`, `leg-left`, `leg-right` (children of `root`), `body` (the tower, pivot at its base), `head` (the hub with the face), `rotor`, `blade-a`, `blade-b` (120°), `blade-c` (240°).
- Parts: legs, tower (body colour) with a shade strip and a base flange (detail), three blades (accent, 1.5 px ink) behind the hub, hub (body, 2 px ink). Face on `head`.
- Clips: `breeze` (idle, 3.2 s loop: tower sways, rotor rocks, hub breathes) · `tiptoe` (walk, 0.6 s loop: leg swing, bob, forward lean, rotor turns 120° per loop) · `gust` (fidget, 2.4 s) · `lull` (fidget, 2.8 s) · `wave` (greet, 1.8 s loop: little hop, rotor rocks three times like two waving arms) · `nuzzle` (cuddle, 2.4 s loop: tower bends over, head nuzzles, blades fold) · `whirl` (squabble, 0.9 s loop: one revolution per loop, stamps a foot) · `becalmed` (sulk, 3.2 s loop: tower leans away, head hangs, blades droop) · `doze` (sleep, 5 s loop) · `touchdown` (land, 0.4 s) · `leap` (hop, 0.5 s loop) · `tumble` (fall, 0.4 s loop, rotor runs backwards).
- Fidgets: `gust` — a gust pushes the tower back, it leans into it and the rotor spins two revolutions. `lull` — the rotor swings to a stop, one lower blade lifts and scratches the head three times while the tower sags.
- Not satisfied with: it is 56 tall, not the "XL" of the sheet (design §9 caps the height), so the hub is large against the tower and the blades are short and leaf-like — it reads as a turbine through tower + three blades, but less slender than a real one. On the light theme the cream blades are outline only. The rotor does not turn in idle (see point 1). At the end of `whirl` and `tiptoe` the fade-out multiplies the accumulated angle by the weight, so the rotor unwinds for 0.125 s; it is spinning anyway and I could not see it in the engine frames.

### flamy — the tea light (walk)

- Bones: `root`, `leg-left`, `leg-right`, `body` (the tin cup), `flame` (pivot at the wick), `core` (inner flame), `flame-tip`.
- Parts: legs, cup (detail, 2 px ink), wax (paper), then the flame as bulb + tip with the same rim/fill technique as the cloud so the tip can curl without opening the outline, inner flame (accent) carrying the mouth. Face on `flame` — the flame is the body of the character, the cup its seat.
- Clips: `glow` (idle, 2.8 s loop: flame breathes, tip sways, core pulses) · `scoot` (walk, 0.5 s loop: leg swing, cup rocks, flame lags behind) · `flicker` (fidget, 1.6 s) · `ember` (fidget, 3 s) · `tip-wave` (greet, 1.8 s loop: little hop, the tip waves) · `warm` (cuddle, 2.4 s loop: leans in and grows) · `flare-up` (squabble, 0.8 s loop: tall, jittering, cup rattles) · `gutter` (sulk, 3.2 s loop: small, turned away, tip hanging) · `doze` (sleep, 5 s loop: a low, wide pilot flame) · `touchdown` (land, 0.4 s) · `leap` (hop) · `tumble` (fall).
- Fidgets: `flicker` — the flame stretches and squashes in quick beats while the tip curls left and right. `ember` — it shrinks to an ember (face and all), sputters twice, then whooshes back taller than before and settles.
- Not satisfied with: small and narrow (28 wide) next to the others, on purpose (the smallest power of the quiz) but it will look tiny beside cloudy. Because the eyes sit on the scaling `flame` bone, the face shrinks with the ember (intended, but pupils get very small at 0.42). There is a faint shoulder where the tip meets the bulb. Gait is `walk` on two small legs, not the hop of the character sheet (the brief names walk and float only).

### kettly — the kettle (walk)

- Bones: `root`, `leg-left`, `leg-right`, `lamp` (children of `root`), `body` (pivot at the bottom of the kettle), `steam-a`, `steam-b` (hidden behind the body at rest), `handle`, `spout`, `lid` (hinge at the handle side), `cheek-left`, `cheek-right`.
- Parts: two steam puffs (detail, drawn first), legs, handle (a 5.5 px ink stroke under a 2.5 px accent stroke, two parts on one bone), spout, knob (accent), lid, power base with lamp (on `root`, so the body rocks on it), body, cheeks (accent). Face on `body`, shifted towards the spout. Spout right, handle left: it faces right.
- Clips: `simmer` (idle, 3 s loop: breathing, lid lifts a little) · `waddle` (walk, 0.5 s loop: leg swing, body rocks on its base, lid clatters on every step) · `boil` (fidget, 2.8 s) · `sigh` (fidget, 2.4 s) · `hat-tip` (greet, 1.8 s loop: bows and tips its lid like a hat) · `cosy` (cuddle, 2.4 s loop: leans in, blushes, lid purrs) · `boil-over` (squabble, 0.7 s loop: rattles, lid hops, steam shoots from the spout) · `lukewarm` (sulk, 3.2 s loop: sags, spout and handle droop, cheeks pale) · `doze` (sleep, 5 s loop: the lid lifts once per breath) · `touchdown` (land, 0.4 s: the lid jumps) · `leap` (hop) · `tumble` (fall).
- Fidgets: `boil` — it rattles harder and harder, blushes, the power lamp glows, the lid hops four times and two steam puffs rise from the spout, then it settles. `sigh` — it breathes in, sags with a long breath out, the lid opens and one puff escapes from under it, the spout droops.
- Not satisfied with: steam puffs are plain circles (rings on the light theme because detail = paper cream). The lamp is 3 px across and hard to see at scale 1. It is drawn as a classic kettle on a power base rather than a jug kettle. 46 wide because handle and spout stick out.

## Verification (commands run from `/c/git/semio`, real results)

Owned validator (final state of the files):

```
$ bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/validate_species.ts" <the five files>
ok   🎓️teaching/🏛️architecture/🐾️pets/☀️sunny/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/☁️cloudy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/💨️windy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🕯️flamy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🫖️kettly/🔣️.json
exit 0
```

That the validator is live was checked with a deliberately broken copy of sunny (outside the repository): it reported `unknown-reference at /bones/3/parent`, `loop-seam at /clips/0/tracks/0/keys/2/value`, `property-unknown at /extra`, `float-hover at /locomotion/hover`.

Further checks, run with throw-away scripts in the session scratchpad (nothing added to the repository):

- ajv against `🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json#/$defs/Species`: all five `ok`.
- Grounds: every entry resolved against the four quiz files (quiz id, task id, item id) — all `ok`. sunny has grounds in physics and cooling, cloudy carries the topic-level grounds `physics` and `cooling` plus three concrete ones, windy / flamy / kettly are grounded in physics.
- Repertoire: every clip is mapped, no clip has two tracks on one bone channel.
- Real stage, five species as one cast, mode `lively`, seed 3, 2,400 s: 76,800 frames, 0 non-finite numbers; idle, walk, both fidgets, greet, squabble, sulk, sleep, hop and land were all played. A second run over five seeds with stronger bonds also reached cuddle for every pet; its frames are the `wp-h1-engine-<id>.png` sheets.
- Rotor sawtooth against the real `sampleClip`: see point 3 above.

Judged by eye on every sheet above (light and dark theme, gaze left/right, sad/happy mouth, blink, mirrored copy, every clip at four phases, and the engine frames). Changes made after looking: rounded ray tips; cloud eyes enlarged and belly shine moved off the mouth; turbine hub enlarged, blades widened, tower shade moved to the edge; the flame's inner core enlarged so it reads as an inner flame with the face on it instead of a beak; kettle handle rebuilt as a loop with a visible hole, cheeks kept inside the outline; the largest squabble and fidget amplitudes reduced so disputes stay small.

## Open

- The renderer occasionally failed with `page.screenshot: Protocol error (Page.captureScreenshot): Unable to capture screenshot` while other agents rendered at the same time; a rerun always succeeded.
- Nothing else of mine is open. The points above under "not satisfied with" are judgement calls for the owner, not defects.
