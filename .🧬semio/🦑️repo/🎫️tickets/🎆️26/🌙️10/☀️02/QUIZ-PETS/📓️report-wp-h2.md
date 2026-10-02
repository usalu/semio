# 📓️ Report: work package H2 (art: housy, waly, windowy, roofy, insuly)

Ticket `2026/10/02/QUIZ-PETS`. Paths: `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder.

## 1. What exists

| File | Pet | Size (w×h) | Gait, speed | Bones | Parts | Clips |
|---|---|---|---|---:|---:|---:|
| `AP/🏠️housy/🔣️.json` | Housy, the house / das Haus | 48×52 | walk, 26 px/s | 10 | 11 | 10 |
| `AP/🧱️waly/🔣️.json` | Waly, the wall / die Wand | 48×46 | walk, 20 px/s | 9 | 11 | 10 |
| `AP/🪟️windowy/🔣️.json` | Windowy, the window / das Fenster | 40×52 | hop, 40 px/s | 7 | 14 | 10 |
| `AP/🛖️roofy/🔣️.json` | Roofy, the roof / das Dach | 56×42 | walk, 46 px/s | 10 | 10 | 10 |
| `AP/🧶️insuly/🔣️.json` | Insuly, the insulation / die Dämmung | 48×40 | hop, 22 px/s | 13 | 23 | 10 |

Directory code points were checked with the `bun -e` one-liner of the brief: `1f3e0 fe0f`, `1f9f1 fe0f`, `1fa9f fe0f`, `1f6d6 fe0f`, `1f9f6 fe0f`.

Every document starts with `"$schema": "../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json#/$defs/Species"`, then the `Species` fields in type order. Palettes are exactly the hex values of `📓️explore-topic-pets.md` §6.3.

Sheets (tool output, to be deleted with the ticket's `🗑️generated`):

- `TK/🗑️generated/preview/wp-h2.png` — all five, every clip at four phases (scale 2; 1500×10921 px, open the `.html` beside it for a comfortable view).
- `TK/🗑️generated/preview/wp-h2-rest.png` — all five, rest rows only, light and dark: the side-by-side for family resemblance and relative size.
- `TK/🗑️generated/preview/{housy,waly,windowy,roofy,insuly}.png` — one full sheet per pet.

## 2. Important: the files contain edits that are not mine

While I was iterating, another agent edited the same five files. The harness flagged it ("changed on disk since you last read it") and the content confirms it. I did not revert anything; my later edits and the foreign ones are both present. What I found that I did not author:

- walk clips: foot planting through `leg-*.y` tracks and a different body bob; `waddle` 0.64 s → 0.5 s, `march` 0.7 s → 0.6 s, larger leg swing;
- hop clips (windowy, insuly): an added `root.x` track (−3.5 … +3.5);
- every land clip lengthened from 0.4…0.45 s to 0.55…0.6 s with an extra hold key;
- waly: my fidget `flex` (both arms pumping) was replaced by `knock` (2.2 s: knocks on itself, the brick creeps out with each knock and is pushed back);
- roofy: `ripple` and `loose-tile` amplified (tile lifts to −66°, tile rows also squeeze in `scaleX`), `hunker` arm value changed;
- insuly: `tuck-in` and `snuggle` tail stretch reduced;
- speeds: housy 30 → 26, waly 24 → 20.

Everything below describes the files as they are on disk now. The sheets and the validation runs of §5 were made after those edits.

## 3. Per pet

Common rig decisions: `root` at the feet, `body` at the base of the body (so scale squashes towards the feet and rotation leans from the hips), legs are children of `root` (feet stay on the ground while the body breathes or squashes), 3 px round-capped `ink` lines for limbs, eyes radius 3.3…3.5 with pupil 1.5…1.6, mouth width 6.5, all on `body`. Parts that must be invisible at rest are hidden behind an opaque part and slide out (the schema has no opacity).

### 3.1 housy (kind host, calm; energy 0.35, sociability 0.85, curiosity 0.45)

- Look: tan wall block, dark-red pitched roof with overhang, sand chimney on the right slope, sand door with knob on the left of the wall, face on the right of the wall under the eaves (eyes are not windows).
- Bones: `root`, `body`, `leg-left`, `leg-right` (on root), `arm-left`, `arm-right`, `door` (hinge on its left edge), `roof` (hinged at the left eave), `chimney` (on roof), `puff` (on chimney, hidden behind chimney and roof at rest).
- Clips: `breathe` (idle, 3.6 s), `waddle` (walk, 0.5 s), `smoke-ring`, `shiver` (fidgets), `welcome` (greet), `shelter` (cuddle, loop), `rattle` (squabble, loop), `mope` (sulk, loop), `doze` (sleep, loop), `touchdown` (land).
- Fidgets: `smoke-ring` — the house inhales, the chimney squashes and pops, a paper-white ring rises out of it, grows and vanishes. `shiver` — a draught: the door blows open (lit doorway), the house shivers, clutches its walls with both arms, the roof rattles, the door shuts.
- Other signature motion: `welcome` tips the roof like a hat, waves and opens the door; in `rattle` the roof clatters like a lid; in `mope` and `doze` the roof sinks over the brow.
- Not satisfied: the door sits off-centre (the brief's sheet puts it in the lower middle as the mouth; with a schema mouth the two collided, so the face went right and the door left). In `touchdown` the squashed roof touches the top of the eyes for a few frames. The smoke puff shrinks to scale 0.05 to disappear, which is a trick, not a fade.

### 3.2 waly (stoic, stubborn, slow; energy 0.3, sociability 0.4, curiosity 0.25)

- Look: brick-red slab with four courses of sand mortar in running bond, sand coping stone with a slate shadow line, sturdy 4 px legs, two-segment arms. The bond is laid out so the eyes sit in the third course over its joints and the mouth in the middle of a brick.
- Bones: `root`, `body`, `leg-left`, `leg-right` (on root), `brick` (hidden behind the wall, slides out to the right), `arm-left` → `forearm-left`, `arm-right` → `forearm-right`.
- Clips: `stand` (idle, 4 s), `march` (walk, 0.6 s), `loose-brick`, `knock` (fidgets), `salute` (greet), `lean-on` (cuddle, loop), `stomp` (squabble, loop), `stonewall` (sulk, loop), `doze` (sleep, loop), `thud` (land).
- Fidgets: `loose-brick` — a brick pops out of the top right, wiggles, the right arm comes up and it is shoved back. `knock` (not mine, see §2) — knocks on itself three times, the brick creeps out a little more with each knock, then is pushed in.
- Other: `stonewall` folds both arms in front of the wall; `stomp` jumps and slams, the brick jolts out; `thud` jolts the brick too.
- Not satisfied: the popping brick is a separate brick that slides out from behind the wall; the brick pattern on the face does not change, which is a cheat that holds at 48 px. The arms are drawn in front of the wall (needed for the folded arms), unlike the other four pets. I judged `knock` only on the sheet, not while crafting it. The flexing arms asked for in the brief survive only in `stomp`.

### 3.3 windowy (curious, chatty show-off; energy 0.65, sociability 0.8, curiosity 0.9)

- Look: grey frame, two pale-blue casement sashes with a transom each, a double glint on the right upper pane, cream sill, two feet. Eyes in the lower panes with grey sockets, mouth on the bottom rail. No arms: the sashes are the arms.
- Bones: `root`, `body`, `leg-left`, `leg-right` (on root), `sash-left`, `sash-right` (hinged at the outer edges; `scaleX` swings them open over an `ink` opening), `glint` (on `sash-right`).
- Clips: `breathe` (idle, 3 s), `hop` (gait, 0.6 s loop), `air`, `gleam` (fidgets), `flap` (greet), `open-up` (cuddle, loop), `rattle` (squabble, loop), `shut` (sulk, loop), `doze` (sleep, loop, one sash ajar), `clatter` (land).
- Fidgets: `air` — the right sash swings open, the window shivers in the draught and slams it shut. `gleam` — it tilts its glass, the glint sweeps to the left pane and back with a twinkle at each end.
- Not satisfied: the eyes stay on `body`, so an opened sash leaves the eye over the opening. On the light theme that is a white eye on a dark opening and reads well; on the dark theme the opening is cream, the white of the eye merges with it and only the grey socket ring and the pupil remain. The glint is cream on pale blue (the given palette) and is faint at scale 1. In `gleam` the glint crosses the mullion for a few frames. The grey sockets were added because cream eyes with a cream outline vanished on the glass in the dark theme; they also act as lids in the blink.

### 3.4 roofy (protective, a little shy; energy 0.55, sociability 0.45, curiosity 0.5)

- Look: slate gable with rounded corners, scalloped tile rows in dark slate (kept clear of the face), brick chimney with pot on the back slope, one loose tile lying on the front slope, thin 2.5 px legs, eave tips as hands.
- Bones: `root`, `body`, `leg-left`, `leg-right` (on root), `arm-left`, `arm-right` (at the eaves), `chimney`, `tiles-low`, `tiles-high`, `tile` (hinged at its upper end, rest rotation 50.66° along the slope).
- Clips: `breathe` (idle, 3.4 s), `scamper` (walk, 0.5 s), `loose-tile`, `ripple` (fidgets), `hat-tip` (greet), `shelter` (cuddle, loop, leans over the partner), `clatter` (squabble, loop), `hunker` (sulk, loop, sinks onto its legs), `roost` (sleep, loop, sits on the ground with the legs hidden), `plop` (land).
- Fidgets: `loose-tile` — the tile flaps up and flutters, roofy leans to look, reaches for it, then hops and squashes to slam it flat. `ripple` — a wave runs up the tile rows twice, lifts the loose tile and pops the chimney.
- Not satisfied: the arms are only small nubs at the eave corners and are nearly invisible at rest. At 42 px it is the lowest of the five and at 56 px the widest; that follows the sheet ("S wide") but it needs the longest perch.

### 3.5 insuly (gentle, motherly, sleepy; energy 0.25, sociability 0.8, curiosity 0.35)

- Look: a pink batt with a scalloped outline (twelve overlapping puffs behind a strokeless core), the batt-insulation zigzag across the belly in the fold-shadow colour, cream sheens on the top puffs, a loose end with a cream foil stripe at the back, tiny arms and feet.
- Bones: `root`, `body`, `leg-left`, `leg-right` (on root), `tail`, `arm-left`, `arm-right`, six animated puffs `tuft-side-left`, `tuft-side-right`, `tuft-top-left`, `tuft-top-right`, `tuft-top-mid-left`, `tuft-top-mid-right` (six more puffs are static parts on `body`).
- Clips: `breathe` (idle, 4 s, the puffs swell in a travelling wave), `hop` (gait, 0.8 s loop, squishy), `fluff-up`, `tuck-in` (fidgets), `wave` (greet), `snuggle` (cuddle, loop, puffed up, loose end unrolled), `bristle` (squabble, loop, puffs alternate), `deflate` (sulk, loop), `snooze` (sleep, loop, sits flat), `flump` (land).
- Fidgets: `fluff-up` — the puffs pop up one after another from back to front, it shakes itself and settles. `tuck-in` — the loose end unrolls behind it, it leans back and pats it flat three times, then rolls it in again.
- Not satisfied: without context it is "a pink fluffy cushion"; a first, rounder version read as a brain and was replaced by the rectangular batt. Scaling a puff scales its outline, so the puffs are limited to about 1.35, which makes "puffing up" modest. On the dark theme the cream sheens merge with the cream outline.

## 4. Decisions worth knowing

- **Hop gait** (windowy, insuly): the repertoire maps both `walk` and `hop` to the one `hop` clip, because the design does not say which entry the stage reads for a hopping species during the `walk` activity. The vertical arc is in the clip (`root.y`).
- **Greet** is a one-shot (about 1.6 s); cuddle, squabble, sulk and sleep are loops. All one-shot clips start and end on rest values on every track, all loops close their seams (checked by script, zero findings).
- **Grounds**: every string exists in the quiz files (checked by script against the four `🔣️.json`). Coverage per cast: housy heating, demand, physics; waly heating; windowy heating, cooling, demand; roofy heating, cooling, demand, physics; insuly heating, demand.
- I judged everything on stills (four phases per clip, light and dark, mirrored copy). Nothing was seen in motion; the stories gallery was not available to me.

## 5. Verification (real output, after the last change to any of the five files)

```
$ bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/validate_species.ts" <the five files>
ok   🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🧱️waly/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🪟️windowy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🛖️roofy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🧶️insuly/🔣️.json
validator exit=0
```

The validator accepts the `$schema` key. Additionally, ajv (from the repo's `node_modules`, `strict: false`) against `🧬️schema/🔣️.json#/$defs/Species`: all five `ok`. Grounds check: `unknown: none` for all five.

## 6. Open points and slips

- Two rule slips of mine: one `sed -i` on `AP/🧱️waly/🔣️.json` (shorter arms) before I switched to the Edit tool only; the file was checked afterwards (valid JSON, no CR, `$schema` emoji intact). Four throwaway check scripts lived in the OS temp folder, not in the ticket; they are deleted.
- Chromium screenshots of the preview tool failed sporadically (`Unable to capture screenshot`) while other agents rendered; a retry always worked.
- For the polish pass: windowy's eye over an open sash on the dark theme, roofy's arms, insuly's recognisability, and a look at all walks and hops in motion.
