# 📓️ Report WP H3 — art: radiatory, pumpy, boily, thermy, chilly

## What exists

| Pet | File | Size (w×h) | Gait, speed | Temperament (energy / sociability / curiosity) |
|---|---|---|---|---|
| radiatory | `🎓️teaching/🏛️architecture/🐾️pets/♨️radiatory/🔣️.json` | 54×51 | walk, 26 px/s | 0.35 / 0.85 / 0.4 (warm, cuddly, sleepy) |
| pumpy | `🎓️teaching/🏛️architecture/🐾️pets/🌀️pumpy/🔣️.json` | 50×45 | walk (four-legged trot), 34 px/s | 0.55 / 0.3 / 0.45 (calm, hard-working introvert) |
| boily | `🎓️teaching/🏛️architecture/🐾️pets/🔥️boily/🔣️.json` | 48×52 | walk, 24 px/s | 0.4 / 0.35 / 0.2 (grumpy veteran) |
| thermy | `🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json` | 32×51 | walk (long strides), 38 px/s | 0.6 / 0.45 / 0.85 (nervous, precise, curious) |
| chilly | `🎓️teaching/🏛️architecture/🐾️pets/❄️chilly/🔣️.json` | 52×43 | walk (skating clip), 44 px/s | 0.45 / 0.25 / 0.35 (aloof, grumpy-cool) |

Directory names were checked for emoji + U+FE0F (`2668 fe0f`, `1f300 fe0f`, `1f525 fe0f`, `1f321 fe0f`, `2744 fe0f`). Every document starts with the `$schema` key; names, nouns and palette values are exactly those of `📓️explore-topic-pets.md` §2 and §6.3.

Preview sheets (tool output, `TK/🗑️generated/preview/`): `wp-h3.png` (all five, every clip, scale 2), `wp-h3-rest.png` (the five at rest on both themes, side by side — the sheet to look at for family resemblance), `<id>.png` per pet, and the larger `<id>-a.png` / `<id>-b.png` halves that were used for judging.

## Shared rig decisions

- `root` at the feet, `body` at the hip line (so breathing and squash grow upwards from the feet). **Legs are children of `root`, not of `body`**: their upper ends are hidden behind the body, so body bob, lean (`body` rotation) and squash never lift or bury the feet. Arms and all signature parts are children of `body`.
- Every loop closes its seam. Fans and the snowflake spin with a two-key `rotation` track `0 → 360` in looping clips (the amended `loop-seam` rule, modulo 360°); non-looping fidgets spin `0 → 720` / `0 → 1080` with an ease (spin-up and spin-down) and end on a pose identical to rest because the shapes have three- and six-fold symmetry.
- `greet`, `cuddle`, `squabble` and `sulk` clips are **loops** (their activities last 2…6 s, longer than any one-shot gesture): greet loops start and end at rest with a pause after the wave; cuddle, squabble and sulk hold a pose with motion in it. `fidget` and `land` clips are one-shots that start and end at the rest values. If the stage should rather play one-shot encounters, only the `loop` flag needs flipping on the greets.
- "Appearing" props (pumpy's air streaks, boily's smoke ring, chilly's breath cloud) cannot be hidden by opacity, so they rest **behind the body** (drawn first) and clips slide them out; they shrink to 0.05 at the end of a one-shot instead of sliding back.
- Repertoire of every pet: `idle`, `fidget` (two clips), `walk`, `land`, `sleep`, `greet`, `cuddle`, `squabble`, `sulk`.

## Per pet

### radiatory (15 bones, 20 parts, 10 clips)

- Bones: `root`, `body`, `leg-left`, `leg-right`, `arm-left`, `arm-right`, `fin-1` … `fin-5`, `knob`, `shimmer-1` … `shimmer-3`.
- Look: five round-capped ribs with a scalloped silhouette, thermostat knob with a coral mark on a valve stub top right, three orange heat-shimmer lines above (the ♨️ sign), pipe-stub legs. Each fin is drawn twice (a 3.5 px ink backing and a fill with a 1 px line), which gives a 2 px outer outline but thin dividers.
- Clips: `simmer` (idle 3.2 s: breathing, shimmer wafts out of phase), `shuffle` (walk 0.6 s: stiff steps, shoulders rock, shimmer trails), `ripple` (fidget: a wave runs through the fins to the right and back, the shimmer above each fin flares as the wave passes), `thermostat` (fidget: the right arm reaches up, the knob turns up — shimmer grows, body swells — then down — shimmer shrinks), `wave` (greet), `hug` (cuddle: both arms open, leans in, extra heat), `huff` (squabble: fins bristle alternately, knob on full, shimmer flares), `cool-off` (sulk: leans away, knob turned down, shimmer almost out, outer fins droop), `doze` (sleep), `thud` (land).
- Deviation from the sheet: **five fins instead of six**. With six, the fin dividers ran through the eyes; with five, each eye sits on its own fin and the mouth on the middle one.
- Not fully satisfied: on the dark theme the eye whites and the fin dividers are both cream, so the eyes touch the dividers of their fin; the pupils keep the face readable. The shimmer rises about 1 px above the size box.

### pumpy (10 bones, 13 parts, 10 clips)

- Bones: `root`, `body`, `leg-back-far`, `leg-back-near`, `leg-front-far`, `leg-front-near`, `tail`, `intake`, `outlet`, `fan`.
- Look: rounded outdoor unit, eyes and mouth above a round fan well (accent) with three blades and a hub, louvres left and right of the well, four stubby legs, a curled refrigerant pipe with a copper core as tail.
- Clips: `hum` (idle 4 s: the fan turns once per loop, breathing, slow tail wag), `trot` (walk 0.6 s: diagonal leg pairs, bob, rocking, fan at 600°/s), `rev` (fidget: the fan spins up and down through three turns, the body vibrates, then a proud little hop), `exchange` (fidget: cool streaks are drawn in on the left, the body swells, warm orange streaks leave on the right), `paw` (greet: two hops with wagging tail, then the front paw kicks up), `purr` (cuddle), `bluster` (squabble: stomps and blows warm streaks at the other), `standby` (sulk: fan stopped, tail hangs), `sleep-mode` (sleep: sits down on its legs, fan stopped), `clunk` (land).
- Not fully satisfied: the blades are body-teal on an accent-teal well and live mostly from their outline; the paw can only show below the body, so the greeting reads as a kick more than a wave. The tail makes the box asymmetric (the size box has about 5 px of air on the right).

### boily (10 bones, 14 parts, 10 clips)

- Bones: `root`, `body`, `leg-left`, `leg-right`, `arm-left`, `arm-right`, `flue`, `puff`, `flame`, `needle`.
- Look: round-shouldered cast-iron body, flue with collar on the back shoulder, grumpy brows, round fire window with an orange flame and yellow core, small pressure gauge with a needle.
- Clips: `simmer` (idle 3.6 s: flame flickers, needle trembles), `plod` (walk 0.7 s: the flue rattles on every step), `whoomp` (fidget: the flame flickers faster, ducks, then bursts with a jolt, arms fly up, needle swings), `puff` (fidget: the needle climbs in jerks, the body swells, then a smoke ring pops out of the flue and rises), `tip-flue` (greet: tips the flue like a hat while raising an arm), `warm-up` (cuddle: bigger flame, open arms), `rumble` (squabble: shaking, big flame, a smoke ball pumping above the flue), `grumble` (sulk: small flame, flue tilted back), `pilot-light` (sleep: tiny flame, sits down), `clang` (land).
- Notes: the window is painted `ink`, so it is a dark porthole on the light theme and a cream one on the dark theme; the flame reads on both. The brows are a static part, so boily looks grumpy even when its mouth smiles (wanted: "secretly soft").

### thermy (9 bones, 9 parts, 10 clips)

- Bones: `root`, `body` (the bulb), `leg-left`, `leg-right`, `arm-left`, `arm-right`, `tube`, `column`, `warmth`.
- Look: blue bulb with the face, glass tube (detail colour) with scale ticks on its right, a liquid column that is blue below and red on top. Limbs are 2.5 px instead of 3 px (thin stick limbs per the sheet).
- Rig of the gradient: `column` (anchored inside the bulb) carries the blue part, its child `warmth` sits on the blue part's top and carries the red part; `column.scaleY` is the level, `warmth.scaleY` how much of it is red. Cold poses are short and blue, hot poses tall and mostly red.
- Clips: `steady` (idle 3 s: the level trembles, the tube sways), `stride` (walk 0.55 s: long strides, tube lags behind), `reading` (fidget: the column climbs to the top and turns red while thermy fans itself, then drops to a blue stub while it shivers), `tap` (fidget: taps the base of its tube three times, the tube leans in and the column jiggles), `salute` (greet: wave and a bow of the tube), `glow` (cuddle), `fluster` (squabble: hopping, column at the top), `cold-read` (sulk: tube droops away, column blue and low), `nap` (sleep: tube nods forward), `boing` (land: the liquid sloshes).
- Not fully satisfied: 51 px tall instead of the sheet's 64 (the art direction caps at 56), so the face is the smallest of the five (eye radius 3.2); on the dark theme the eye whites cover much of the bulb.

### chilly (10 bones, 10 parts, 10 clips)

- Bones: `root`, `body`, `leg-left`, `leg-right`, `arm-left`, `arm-right`, `crystals`, `breath`, `frost`, `emblem`.
- Look: wide blue cabinet, a scalloped frost fringe under its top edge, three ice shards on the back of the roof, a cream snowflake emblem and a vent grille on the belly, ice-skate feet with curled tips.
- Clips: `chill` (idle 4 s: the snowflake turns once per loop, crystals pulse), `glide` (walk 0.7 s: skating — alternating push-offs, forward lean, balancing arms), `frost-over` (fidget: the frost creeps down over the front behind the face, the crystals grow, a shiver, then it melts back), `flurry` (fidget: the snowflake spins up and grows, chilly leans back and blows a frosty cloud forward), `nod` (greet: a curt arm lift), `thaw` (cuddle: frost and crystals shrink), `glare` (squabble: bristling crystals, frost spreads, an icy cloud is blown at the other), `cold-shoulder` (sulk: frosted over, leaning away), `hibernate` (sleep), `skid` (land).
- Deviations from the sheet: no red/blue coolant pipes (the palette has no red); the gait is `walk` with a skating clip, because `float` would require a hover height.

## Verification (real output)

`bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/validate_species.ts" <the five files>`:

```
ok   🎓️teaching/🏛️architecture/🐾️pets/♨️radiatory/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌀️pumpy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔥️boily/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/❄️chilly/🔣️.json
validate exit 0
```

- The `$schema` key is accepted by the owned validator.
- A throw-away ajv run of `P/🧬️schema/🔣️.json#/$defs/Species` against the five files also reported `ok` for all five.
- Grounds: a throw-away script resolved every ground string against the four quiz files — all 34 exist. radiatory carries `demand/final-energy` (task level) because it is in the rotation of the `demand` cast and no demand item names a radiator; thermy carries `heating` and `cooling` plus three temperature items and one task.
- Every pet and every clip row was looked at on the preview sheets (rest, sad, happy, blink, mirrored, both themes; four phases per clip).

## Open

- The pose pop at the end of a looping encounter clip depends on how `🎪️stage` leaves an activity; the clips keep their held offsets small (≤ 8° lean) for that reason.
- No `hop` or `fall` clip is mapped; those activities show the rest pose.
- Nothing was run in the stories gallery or the real layer; the judgement is from the static contact sheets only.
