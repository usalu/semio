# 📓️ Report WP O1 — art polish in motion: sunny, cloudy, windy, flamy, kettly, radiatory, pumpy, boily, thermy, chilly

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02. `TK` = this ticket folder, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `OUT` = `TK/🗑️generated/wp-o1`.

Only the ten species documents `AP/{☀️sunny, ☁️cloudy, 💨️windy, 🕯️flamy, 🫖️kettly, ♨️radiatory, 🌀️pumpy, 🔥️boily, 🌡️thermy, ❄️chilly}/🔣️.json` were edited (Edit tool, small anchored edits, valid JSON after every save), plus three tools and this report under `TK`. No git command that modifies anything, no cargo, no e2e gate, no deploy check. The repo MCP server was down; no ticket was opened or closed.

## 1. How the pets were judged

| Tool | What it does |
|---|---|
| `TK/wp_o1_stage_sheets.ts` (bun) | Plays every activity of a species on the **real stage** (`openStage`, `advance`, `frameOf` from the pets barrel) with scripted events and draws the frames with the product's `depictionMarkup` and the product's `🎨️.css` into one HTML sheet per species and theme. Rows: rest with the pointer left / right / up / down-right, mirrored, sad and happy mouth (at scale 2 and at scale 1); blink; idle loop; both fidgets; greet, cuddle, squabble (running on into the real sulk) and sulk next to a partner 6 px away; sleep and waking; walk over two cycles with ground marks every 8 px; turning round; coming to rest; fall from 70 px and landing; hop onto a ledge. Also writes numbers the eye cannot judge: pupil travel, one-shot clips that do not end at rest, the world position of every foot over a walk cycle (slip), the largest move of any bone in one tick (pops). Paths are found by their ASCII names, no emoji is typed. `--rows <word,…>` limits the rows. |
| `TK/wp_o1_shoot.mjs` (node + Playwright) | Screenshots the pages of those sheets (`<id>-<theme>-<page>.png`) and measures per row how far the drawing reaches around the feet, stroke included (`getPointAtLength` through the screen matrix), against the declared `size` → `<id>-extent.txt`. |
| `TK/wp_o1_live.mjs` (node + Playwright) | Films the **sandbox of the stories gallery** (the real `PetLayer`, survey, pacing) and writes per species a contact sheet of crops around the pet, one per sample, with time, place and facing. |

The gallery ran on the private port **6237** (`PETS_STORIES_PORT=6237 PETS_MENAGERIE=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts bun ./📜️script.ts dev` in the React package). I stopped it by PID only (listener 33092; its launcher 56644 ended with it); port 6237 is free again. Nothing else was stopped.

Every sheet page of every pet was looked at on the light theme, the first page (rest, gaze, 1× scale, blink) also on the dark theme, and the live sheets of the four quiz scenes (2× light) and three scenes (1× dark).

**Things in the engine that changed while I worked** (other agents; the sheets follow the current code): `Actor.faced` replaced the turn-by-`since`; the head now leans after the gaze (`leant`: 5° and 1.5 px per unit of gaze), which is why the "rest, gaze" rows reach up to 2–3 px beyond the box on the side looked at; pupil travel is `radius − pupil − 0.25`.

## 2. What was wrong everywhere, and the common fixes

1. **Every walk slid.** Legs are children of `root` and only rotated about a fixed hip, so both feet travelled ±3…5 px around a fixed point while the body moved 10…31 px per cycle. Measured before (world x of a foot over one cycle, should be constant while it stands): radiatory 6.2 → 20.2 monotonous, pumpy 16.9 → 36.7, boily 11 → 27, thermy 21.6 → 40.1; windy / flamy / kettly were not even measured by the first tool version (path legs) and slid the same way. Fix: a `x` track per leg makes the hip travel with the stride — `foot = hip x + leg length × sin(rotation)` runs backwards at exactly the walking speed while the foot stands and forwards while it swings — and a `y` track lifts the swinging foot 1.5…2 px. After (stance values of one foot): windy 17.8 17.8 17.7 17.7 · swing · 34.4 34.4 34.4 (16.8 px per cycle = 28 px/s × 0.6 s); flamy 12.5 ×6; kettly 20.3 ×6; radiatory 15.0 ×7; pumpy 29.8 ×7 (all four feet); boily 21.5 ×6; thermy 32.9 ×6. Legs stay attached (the hip only moves ±0.5…2.2 px behind the body).
2. **Landings.** `land` replaces the idle loop and is faded in over 8 ticks; the H3 clips reached their squash at 0.3 × 0.4 s = tick 8 and left it at once, the H1 clips started squashed (a jump the fade hid) and held to tick 8. All ten `land` clips are now 0.55 s (35 ticks), start at rest, reach the squash at tick 4, **hold it to tick 17**, rebound at tick 27 and end at rest. Squashes that pushed drawings into the ground (root scale on floaters, see sunny and cloudy) were rebuilt on `body`.
3. **Spins.** `loop-seam` modulo 360° is used everywhere: no sawtooth is left. One-shot spins end on a held angle `A` with `A ÷ 8` a multiple of the symmetry of the shape, so the 8-tick fade-out (`A × k⁄8`) shows the same picture at every tick: windy `gust` 1920° (3 blades, 240° per step), pumpy `rev` 1920° and `exchange` 960°, chilly `flurry` 960° (six-armed flake, 120° per step), sunny `ray-spin` ±480° (six rays, 60° per step). The numbers file reports these as "pops" of 10…18 px (it measures bone frames, not pictures); they are not visible.
4. **A spin in the idle loop also runs under the additive clips** (greet, cuddle, squabble, sulk, fidget). Where the thing must stand still (sulk, cuddle) the clip carries the opposite spin of the same length, so the sum is constant; clips that only repeated the idle spin on top of it (double speed for no reason, and a visible unwinding at the end) lost that track.
5. **Props that "appear"** (pumpy's streaks, boily's smoke ring, chilly's breath, kettly's steam) ended their one-shot at scale 0.05 away from home, so the fade-out dragged a dot back across the pet (pops 5…14 px). They now travel home while invisible (scale 0.05, behind the body) and grow back there; kettly's steam started at scale 0.01 at tick 0 (a pop) and now shrinks first.
6. **Missing `hop` and `fall` clips** (radiatory, pumpy, boily, thermy, chilly) meant a stiff rest pose in the air. Each got `leap` (leaning forward, legs tucked back, arms up, the signature part reacting) and `tumble` (legs and arms flailing), mapped in the repertoire.
7. **`size`** was made the true rest box (stroke included, idle loop running) so pets do not overlap the card edge they stand beside; measured rest extents are in §4.

## 3. Per pet

### sunny (float, 46×51; was 44×48)

- Changed: `size` (the rays reached 2.5 px beyond the box at rest and 4.2 px in idle; nothing was redrawn, the box was wrong). `bob`: bob 3 → 2 px and ray pulse centred on rest (0.97…1.05, 0.93…1.07), so idle stays inside the box and above the hover line. `ray-spin` was a 60° twist there and back — with six rays that reads as a wobble; it is now a real counter-rotating spin (long ring +480°, short ring −480°, eased in and out, long rays swelling 10 %). `ray-wave` amplitudes trimmed (−3 px reach). `touchdown`: was a `root` squash that pushed the lower rays 2.4 px through the perch line; now the disc drops 5 px into its hover (10) and squashes on `body`, rays flare, hold, rebound.
- Seen: rays pulse out of phase in idle; the spin reads at 1× scale; the face stays clear of the rays in every row; mirrored copy fine; sad mouth and cheeks readable on both themes. Live (physics and cooling scene): hovers 10 px over the footer line and over card edges, glides without wobble.
- Still bothers me: `beam` (greet) and `flare` (squabble) reach 6 px above and 3 px below the hover line at their peaks — it is a floater, nothing is touched, but beside a card tab the rays overlap its border for a moment. Yellow on the light page still lives from its outline.

### cloudy (float, 58×43; was 58×40)

- Changed: `size` height (the 4 px rim of the top puff is 1.9 px above the old box, 3.2 px in idle). `touchdown` rebuilt like sunny's (the root squash made the 58-wide cloud 67 wide and jump at tick 0): body sinks 6 px into the hover (14), squashes, the middle puff sags, rebound.
- Seen: `drip` is the best fidget of the ten — wring, drop swells, falls exactly the hover height, splashes flat on the perch line and is gone; `billow` rolls through the three puffs back to front. Eyes (radius 4.2, travel 2.05 px) read at 1×. Outline stays closed while puffs move.
- Still bothers me: when a sulk ends while the drop is in the air it is pulled back up over 8 ticks (2.4 px per tick; rare). `bluster` reaches 5.6 px beyond the box towards the partner — a small shove, as intended. The drop's flight depends on `hover` = 14.

### windy (walk, 48×56; was 42×56)

- Changed: **rotor**: `breeze` (idle, now 3.6 s) is a plain `0 → 360` loop — the turbine always turns, slowly; `tiptoe`, `whirl`, `leap` `0 → 360`, `tumble` `0 → −360`; `gust` spins up to 1920° and settles (eased, 6 px per tick at the blade tips at its peak). Blades shortened 0.8 px and `size` widened to the swept circle (radius 22.9 with stroke; the old box was only right for one blade position and blades poked 2…5 px out as soon as anything moved). `lull` redone for a turning rotor: the wind dies (rotor decelerates against the idle spin to a stop), the hub looks up, the tower sags, a foot taps three times, then a little hop and the rotor swings back into the wind (the old blade-scratch only worked on a rotor that stood still). `wave` (greet): hop + lean as before, but the "wave" is now the three blades stretching in turn as they go round (the old ±32° rock fought the idle spin). `nuzzle` (cuddle, 3.6 s): rotor stopped (−360 against idle), blades pulled in to 0.8 so they do not poke the partner, three nuzzles per loop. `becalmed` (sulk, 3.6 s): rotor stopped, blades at 0.86, head hanging; the tower now leans 4° forward (it leant −5°, i.e. towards the partner once the stage has turned the sulker away). Walk: no slide (§2.1), lean reduced. Eyes: pupil 1.45 → 1.4, moved 0.3 px apart (they touched).
- Seen: idle rotor step 1.56° per tick, smooth, never the same picture twice in the sheet; gust reads as a spin-up at 1× dark; stopped rotor in sulk and cuddle is the clearest mood signal of the ten. Mirrored walk fine.
- Still bothers me: at the end of a sulk / cuddle / sleep the rotor snaps from its stopped angle to the idle angle within 8 ticks (≤ 4.8 px per tick at the tips) — reads as a kick-start, not as a glitch, but it is not eased. The idle rotor turns clockwise when facing right and anticlockwise when mirrored. Light theme: cream blades are outline only (unchanged).

### flamy (walk, 28×46; was 28×45)

- Changed: `size` height (rim of the tip). Eyes: they overlapped by 0.5 px on the dark theme (cream on cream); radius 3.2 → 3.0, pupil 1.45 → 1.25, 0.1 px further apart — whites no longer touch, travel 1.5 px. `scoot`: no slide (§2.1), cup bob 1.5 → 1 px. `touchdown`: starts at rest, holds, the flame also widens while flattened.
- Seen: `flicker` (tip curls left and right while the flame stretches) and `ember` (shrinks to a spark with the face on it, sputters twice, whooshes back taller) both read at 1×; the flame lags behind in the walk; gaze reads on both themes.
- Still bothers me: `flare-up` and `ember` reach 7…10 px above the box (a flame flaring; nothing beside it is touched). In `ember` the eyes shrink with the flame to 0.42 — at 1× they are two dots for a second. The faint shoulder between tip and bulb is still there.

### kettly (walk, 46×44)

- Changed: steam puffs are small three-lobed clouds (were plain circles: rings on the light theme) with a 1.25 px outline; they no longer pop at the start of `boil` and `sigh` (§2.5). Power lamp radius 1.5 → 1.7. `waddle`: no slide. `touchdown`: starts at rest, the lid jumps and stays up while the body is squashed, then drops.
- Seen: `boil` — rattle builds, cheeks blush, lamp glows, lid hops four times, two clouds rise from the spout one after the other; `sigh` — breath in, long breath out, lid opens, one cloud from under the lid, spout droops. Lid rattles on every step of the walk. Face clear of handle and spout, also mirrored.
- Still bothers me: the handle loop is the box (left −23.2); it cannot touch a border while kettly stands inside its perch, but it is what touches first. The steam of `boil` and `boil-over` rises 8…9 px in front of the spout, i.e. over a partner's head in a squabble (wanted). The lamp is still small at 1×.

### radiatory (walk, 54×55; was 54×51)

- Changed: **eyes on the dark theme**: two round red sockets (`eye-socket-left/right`, radius 5.8, body colour, no stroke) are drawn over the fin dividers behind the eyes, so the cream whites (radius 3.5 → 3.6, travel 1.75 px) sit on red with a 1.5 px ink ring around them on both themes; on the light theme the dividers simply stop short of the eyes. `size` height 51 → 55: the shimmer was 3.5…5 px above the box (the box now holds the idle shimmer; idle shimmer travel trimmed 2 → 1.5 px and its pulse centred). `shuffle`: no slide. `thud`: starts at rest, sinks 2.5 px, holds, shimmer snuffed and flaring back. New `leap` and `tumble`.
- Seen: `ripple` (a wave through the five fins there and back, the shimmer above flaring as it passes) and `thermostat` (arm up, knob turned up — shimmer and body swell — then down) are both clear; `cool-off` with the shimmer almost out is a good sulk. Live on the dark page at 1×: eyes read as eyes.
- Still bothers me: the mouth sits on the middle fin between two dividers and is 5.5 wide — fine, but the sad mouth nearly touches them. Shimmer in `hug`, `huff`, `thermostat` rises 7…8 px above the box.

### pumpy (walk, 46×46; was 50×45)

- Changed: **greet** is `nod` (was `paw`, the kick): two little hops with wagging tail, then two clear bows towards the other (9° about the feet), tail wagging throughout; no leg leaves the ground. **Fan**: blades are now paper-cream with an orange hub (were body-teal on accent-teal, visible only by their outline — the "fan spinning" was hard to see at 1×). `rev` spins up to 1920° (was 1080°) with the body vibrating at full speed, then the proud hop. `standby` (sulk, 4 s) really stops the fan (−360 against the idle turn; it kept turning before). `purr` and `nod` lost their extra fan track (§2.4). **Box**: the tail made the drawing reach −25…+20; the body and legs were moved 2.5 px towards the face, so the pet is centred on its feet (−22.5…+21.5 at rest) and `size` shrank to 46 wide; height 46 holds the idle breath. `exchange` and `bluster` streaks reach 3…6 px less far and return hidden (§2.5). `trot`: no slide on any of the four feet, diagonal pairs lift. `clunk`: holds, body sinks onto its legs. New `leap` (galloping stretch, fan spinning) and `tumble`.
- Seen: the fan is now the first thing one sees, on both themes and at 1×; nod reads as a nod; `exchange` — cool teal streaks drawn in on the back, body swells, warm orange streaks leave at the front.
- Still bothers me: the streaks of `exchange` and `bluster` still leave the box by 5…9 px (that is the air moving); in a squabble they reach the partner. The tail is 1…2 px outside the box when it hangs in `standby` / `sleep-mode`.

### boily (walk, 48×53; was 48×52)

- Changed: `puff`: the smoke ring rises 25 px (was 29) and comes home hidden (§2.5; the old end pop was 5 px). `plod`: no slide. `clang`: holds, body sinks 2 px. New `leap` (flue rattling back, flame streaming, needle up) and `tumble`. `size` height +1 (flue cap stroke).
- Seen: `whoomp` — the flame flickers faster, ducks, bursts with a jolt, arms fly up, needle swings; `puff` — needle climbs in jerks, body swells, the flue kicks and a ring pops out and widens. The fire window is a dark porthole on light and a cream one on dark; the flame reads on both. Brows keep it grumpy when the mouth smiles.
- Still bothers me: the gauge needle is 2 px long — its swings are invisible at 1×. The smoke ring is a plain circle (ring on the light theme, cream ball on dark).

### thermy (walk, 32×52; was 32×51)

- Changed: eyes: pupil 1.45 → 1.3 (travel 1.5 → 1.65 px; the pupil nearly filled the small white) and 0.4 px further apart (the whites touched and read as one goggle on the dark theme). `stride`: no slide (the long strides stay long: 20.9 px per cycle). `boing`: holds the squash while the liquid sloshes up. New `leap` (tube trailing, column up) and `tumble` (column sloshing red). `size` height +1.
- Seen: `reading` — the column climbs to the top and turns red while it fans itself, then drops to a blue stub while it shivers; `tap` — three taps at the base of the tube, the column jiggles. Both read at 1×. Sulk with the tube drooping away and the column low and blue is clear.
- Still bothers me: the bulb is small, so the face is the smallest of the ten (white radius 3.2); on the dark theme the whites still cover much of the bulb. Enlarging the whites would cover it entirely, so I shrank the pupil instead.

### chilly (walk with a skating clip, 52×44; was 52×43)

- Changed: `flurry`: the snowflake spins up to 960° (was 720°) and the breath cloud is blown 19 px (was 26) and returns hidden (§2.5; old end pop 14 px). `frost-over`, `nod`, `thaw` lost their extra snowflake spin (§2.4; `frost-over` ended on a 7.5 px unwind). `cold-shoulder` (sulk, 4 s) stops the snowflake. `glare` breath 4 px shorter. `glide`: the pushing skate swung 1.8 px **below the ground line**; a `y` track lifts it while it kicks back (0.1 px now). `skid`: holds, skates slide apart (was a leg rotation that dug the blades in). New `leap` and `tumble`. `size` height +1.
- Seen: the skating gait is the most characterful walk — one skate carries, the other pushes back and up, arms balance. `frost-over` (frost creeps down behind the face, crystals grow, shiver, melts back) and `flurry` (flake spins up and grows, leans back, blows a frosty cloud) both show the thing doing its thing. The flake turning in idle is visible at 1× on both themes.
- Still bothers me: skating means the carrying foot glides — by design it is not planted (world x of the standing skate advances 2.8 px per tick-step in the numbers). The breath cloud of `glare` and `flurry` is 8…9 px outside the box in front of the face.

### Kept as it was

Ids, names, grounds, palettes, gaits, speeds, hovers and temperaments of all ten are unchanged. Clip ids are unchanged except pumpy's greet (`paw` → `nod`) and the ten new `leap` / `tumble` clips; nothing outside the species documents refers to clip ids.

## 4. Numbers (final state)

Rest box against the drawing, stroke included, idle loop running (`OUT/extent-final.txt`; "rest" rows include the head leaning after the pointer, which adds up to 3 px on the side looked at):

| Pet | `size` | idle loop x | idle loop y (0 = feet / hover line) |
|---|---|---|---|
| sunny | 46×51 | −21.4 … 22.2 | −50.4 … −1.5 |
| cloudy | 58×43 | −28.5 … 28.8 | −43.2 … 0.4 |
| windy | 48×56 | −23.0 … 23.8 | −56.3 … 0 |
| flamy | 28×46 | −13.0 … 13.0 | −46.1 … 0 |
| kettly | 46×44 | −23.2 … 21.9 | −44.3 … 0 |
| radiatory | 54×55 | −26.9 … 27.8 | −55.0 … 0 |
| pumpy | 46×46 | −22.5 … 23.8 | −46.5 … 0 |
| boily | 48×53 | −23.5 … 24.3 | −53.1 … 0 |
| thermy | 32×52 | −15.4 … 16.1 | −52.3 … 0 |
| chilly | 52×44 | −25.5 … 26.2 | −44.7 … 0 |

Nothing of a walker is below the ground line at rest or in idle; walks touch it by the stroke of the foot only (≤ 0.5 px).

Pupil travel (`radius − pupil − 0.25`): cloudy 2.05, sunny 1.85, kettly 1.75, radiatory 1.75, pumpy 1.65, boily 1.65, chilly 1.65, thermy 1.65, windy 1.55, flamy 1.5 px — all ≥ 1.5.

Pops at the end of an activity (largest move of a bone in one tick over the last 9 ticks and the 12 after; `OUT/numbers-final.txt`): every fidget and greet ≤ 1.8 px, except the symmetric spins of §2.3 (invisible; `land` clips now end on rest values, their largest moves of 2…4 px per tick are the squash going in and the rebound) — before: pumpy `paw` 7.4, `purr` 7.6, `exchange` 14.0, chilly `frost-over` 7.5, `flurry` 14.4, `nod` 7.4, `thaw` 7.1, boily `puff` 5.0. Sulk, cuddle and sleep of the three spinners end with the kick-start described under windy (2.5…4.8 px). The squabble rows end in the turn-round of the real sulk (not a pop).

## 5. Validation (real output, final state)

```
$ bun TK/validate_species.ts <the ten files>      (from /c/git/semio)
ok   🎓️teaching/🏛️architecture/🐾️pets/☀️sunny/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/☁️cloudy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/💨️windy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🕯️flamy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🫖️kettly/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/♨️radiatory/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌀️pumpy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔥️boily/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/❄️chilly/🔣️.json
validate exit 0

$ bun ./📜️script.ts test      (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript)
 Test Files  5 passed (5)
      Tests  131 passed (131)
exit 0
```

(`OUT/validate-final.txt`, `OUT/site-test.txt`.) The stage sheets were generated for all ten on the final files without an error; the seven live recordings report no page error and no console error.

Not run: cargo, parity, the e2e gate, deploy checks, the pets product's own unit suites (no code was changed).

## 6. Final sheets

All under `OUT` = `TK/🗑️generated/wp-o1/`:

- `<id>-light-<n>.png`, `<id>-dark-<n>.png` — the stage sheets (8…10 pages per pet and theme; page 0 = rest, gaze, 1× scale, blink; then idle and fidgets, greet, cuddle, squabble → sulk, sulk, sleep, walk, turning, coming to rest, fall and land, hop). `<id>-light.html` / `<id>-dark.html` are the pages they were shot from.
- `<id>-numbers.txt`, `numbers-final.txt` (all ten), `numbers-before.txt` — eyes, repertoire, rest seams, foot trails, pops.
- `<id>-extent.txt`, `extent-final.txt`, `extent-before.txt` — reach of the drawing per row against the box.
- `live/<scene>-light-2x-<id>-<part>.png` (physics, demand, cooling, heating; lively, seed 3, scale 2) and `live/<scene>-dark-1x-<id>-<part>.png` (physics, demand, cooling; dark page, scale 1, seed 5) — the real layer in the gallery sandbox; `live/<name>.json` holds the places per sample.

## 7. Open

1. **`AP/README.md`** was not touched (outside my files); if it lists sizes or clip names of these ten, it is stale for: the nine changed sizes (all but kettly), pumpy's greet `nod`, the five new `leap` / `tumble` pairs.
2. The head lean after the gaze (engine) adds up to 3 px beyond the box on one side; boxes were sized for the idle loop, not for the lean.
3. Cuddles lean into the partner by design (pumpy +6 px, radiatory +5.6 px, flamy +4.5 px beyond the box): bodies touch. If cuddles should only nearly touch, the stage's meeting gap is the knob, not the art.
4. Squabble props (pumpy's and chilly's streaks, kettly's steam) cross the 6 px gap to the partner — intended, but they overlap its outline.
5. A spin that restarts after sulk / cuddle / sleep is a kick over 8 ticks (windy, pumpy, chilly); easing it needs a blend in the stage that knows about angles modulo the symmetry, not something a clip can express.
6. The idle loop is the only place a continuous spin can live; rotors therefore turn at one fixed slow speed whenever nothing else plays. A "wind" notion (speed varying over a longer cycle) would need a second, longer loop the schema does not have.
