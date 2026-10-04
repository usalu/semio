# 📓️ Work package C2 (second round): the stories gallery of the second round, the dust painter, state attributes

Ticket `2026/10/02/QUIZ-PETS`, second round, phase C; design-v2 §18, §22, §24. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `TK` = this ticket folder. All results below are from runs on 2026-10-03 between 13:50 and 14:55 (Windows, bun 1.4.2) while B4, C3 and C4 edited the same tree. Tool output: `TK/🗑️generated/c2/`.

**State: done.** The dust of a poof is painted, every pet's `<svg>` names its footing, state and mood, and the gallery shows everything the second round adds: a page per species (states, tricks, purr, every clip of getting around with its gear, the poof) and a sandbox with walls, gutters, prop rows, the host's switches, the hand, the deeds and an inspector that reads every frame and forces states, tricks, moods, activities and footings. React package: typecheck exit 0, 8 files / 168 tests pass; quiz `🐾️pet-companions` 45 pass; site 228 pass; browser check without a failed expectation and with an empty console.

## 1. Files

| File | What |
|---|---|
| `PR/🔨️modules/✨️effects/🟦️.ts` | **dust**: `paintPuffs`, `puffShape`, `EffectPuff`, `PuffShape`, `PUFF_CLOUDS` (8), `PUFF_RING` (7), `PUFF_RISE` (¼), `PUFF_OUTLINE` (3); `Effects.dust`; root scaling shared (`scaled`); `strikeEffects` also forgets the dust |
| `PR/🔨️modules/🖌️depiction/🟦️.ts` | `paint` writes `data-pet-footing`, `data-pet-state`, `data-pet-mood` (only on change; 12 actor memo slots); `depictionMarkup` states them |
| `PR/🔨️modules/🫧️layer/🟦️.tsx` | anchored edits in the `Scenery` region: `stage(frame: Pick<Frame, "ladders" \| "particles" \| "puffs">)` paints the dust; the effects root is inserted with the first particle **or puff** |
| `PR/🟦️.tsx` | barrel: the dust names |
| `PR/📖️stories/🟦️.tsx` | the gallery, rewritten (§3) |
| `PR/📖️stories/🎨️.css`, `PR/📖️stories/🌐️.html` | the gallery's look; the document hands over the director |
| `PR/🏗️builder/🌐️vite/🟦️.ts` | `DIRECTOR_MODULE`, `directorVitePlugin()` (§3.3), registered in the dev config |
| `P/🧪️tests/🎆️effect-painting/🟦️.tsx` | +2 tests (dust geometry against gl-matrix; pooling, no-write, cap, strike), 2 extended (no writes while nothing lives; CSP spies) |
| `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` | attributes asserted in 4 tests (root, tint/state writes, no-write, markup); the scenery test draws dust and passes `puffs` |
| `P/🔮️oracles/🔣️.json` | `pets-react-gl-matrix` rationale: one sentence for the dust (capability `pets-react-effect-painting` already listed) |
| `P/README.md` | three anchored edits: the attributes, the gallery paragraph, `paintPuffs` |
| `TK/📓️design-v2.md` | new §24.2 "Dust, state attributes and the gallery (C2)" |
| `TK/c2_gallery_check.mjs` | Playwright drive of the gallery (species pages, close-ups, poof, sandbox), report JSON |
| `TK/c2_frame_probe.mjs`, `TK/c2_poof_probe.mjs` | frame-time probe of a species page; what the poof specimen holds |
| `TK/c2_code_rules.mjs`, `TK/c2_crop.py` | docstring/comment/console/forbidden-write rules; cutting tall screenshots for looking |

No launch entry changed: the dev target's arguments are the same (`bun ./📜️script.ts dev`, `PETS_STORIES_PORT`, `PETS_MENAGERIE`). No new taxonomy name in the product (new files only in `TK`).

## 2. Painters and attributes

### 2.1 Dust (`paintPuffs`, closes B3's open item)

- **Where:** inside the effects root (`svg.pet-effects`, the layer's last child), in a lazily created `<g data-pet-puffs="">` that is its **first** child, so dust lies in front of the actors and behind every particle. The layer's scenery appends the root with the first particle or the first puff.
- **Look** (domain-neutral, a puff belongs to no species): per puff a cloud `<g data-pet-puff visibility opacity transform>` of 8 blobs, drawn twice — 8 outline circles `pet-fill-ink pet-stroke-ink` (stroke 3) under 8 fill circles `pet-fill-paper pet-stroke-none` — so the ink line runs round the union of the blobs. `puffShape(puff)`: middle blob ½ of the mean half-size, a ring of 7 (0.36 / 0.28 of it, alternating) on the ellipse of the body's half-width and half-height, spreading from 0.45 of it to the outline (`1 − (1 − phase)²`), turning ¹⁄₂₀ turn, every blob shrinking to half; the cloud rises ¼ of the body's height; opacity `1 − phase²`.
- **Rules:** pooled clouds (built only when more puffs are in the air than ever before, at most `PUFF_CLOUDS` = 8, never removed while the layer lives), writes only where a rounded value changed (place 0.01, opacity 0.001), `visibility` only on entering/leaving use, nothing written while no puff lives, no ids, attribute writes only.

### 2.2 State attributes on every pet

`<svg class="pet" data-pet data-pet-activity data-pet-footing data-pet-state data-pet-mood>` — footing and mood by their index in `FOOTINGS`/`MOODS`, the state together with the tint (index in the species' states; the attribute is removed for a state the species does not have, which a valid frame never names). Order of first writes = order in `depictionMarkup`. C4's `S/🧪️tests/🐕️pet-walk` already reads `data-pet-footing` and `data-pet-state` (seen in its file at 14:24).

## 3. The gallery and how to use it

Open: VS Code launch `🛠️dev🐾️pets📖️stories` (sample menagerie, http://127.0.0.1:6069/) or `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` (http://127.0.0.1:6074/); Claude preview `pets-stories` / `architecture-pets-stories`; CLI `PETS_STORIES_PORT=… PETS_MENAGERIE=… NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-react:dev`. Header: Species / Sandbox, language (en/de), dark page. Every label in English and German.

### 3.1 Species pages

- **Controls** (all specimens): mood (the nine moods) and intensity (`faceOf`: mouth bend, lid height, head drop), lid, blink, face left, speed (0 pauses the clock), size.
- **Roster** at the top; the chosen species (`#<id>`) gets its page — one page at a time (≈100 specimens per species, 2 028 for the twenty).
- **Sections**, each on a light and a dark ground, captions in monospace:
  1. *At rest and every clip* — as in round one, with the activities that play each clip.
  2. *States* — per state its tint, its look (the state's overlay clip on the channels it keys, over the breathing idle loop, as the stage draws it) and its emitter's particles running; caption: name, id, tinted tones, clip, emitter (motion × count), `lasts … → then`.
  3. *Tricks and the purr* — per trick its clip replayed with its particles thrown once per replay, in the tint of its first `from` state; caption: cues, `from → to`, clip, emitter; then the purr (clip looping, emitter running).
  4. *Getting around and being handled* — per clip of `hang` (drawing swings ±20° about the grip; red dot = the hand, in front), `tumble` (turns about its middle), `glide` (the species' canopy or the plain one, swaying), `aim` (gun at `−AIM_RISE`), `reel` (rope from the hands to a hook on a ledge), `climb`/`slide` (wall line on the facing side, `WALL_LEAN`), `climb` on a standing ladder (species with `ladder` gear), `mantle` (wall corner), `carry` (ladder at `CARRY_LEAN`, 2 heights), `push` (a block), `dizzy`, `shrug`, `scoot`; a line names the activities without a clip; last the **poof** (shown 1.5 s, vanishes in the dust for 0.75 s, arrives anew).
- Drawing = the product's `depict`/`paint` and the layer's `stageScenery` (tools, ladders, dust, particles) — the same seam the layer uses. Specimens are built when they first come into view and painted every frame while in view; all stages are measured before any is painted.

### 3.2 Sandbox

- **Terrain:** six columns with 48 px gutters and a 72 px page gutter; cards of different heights (side walls), a button on an edge, a keep-out note on an edge, a scrolling shelf, and a **Task** card with up to five `li[data-pet-prop]` rows keyed with grounds of the scene's cast (one per species in turn; e.g. home: `physics/powers/sun`, `physics`, `heating/heating-load-and-demand`, `physics/power-or-energy/pv-module-peak`, `heating/u-values/window-passive-house`).
- **Switches:** scene, mode (incl. off; default lively), quiet, **play**, **mischief**, capacity (default 5), scale, **tempo** (×⅛…×8, a new tempo starts a new show), seed, new seed, scroll the cards, rearrange, take a card away.
- **Bench** (right, above the layer): *The hand* (how to pick up, throw, give back, click, circle, stroke); *Inspect a pet* (§3.3); *Live* readout: tick, rate, frames per second, **poofs**, **overlaps now**, **frames with an overlap**, frames, **particles alive**, dust clouds, ladders standing, lifted fixtures, held; a table of the actors in menagerie order. For tools every number carries `data-reading`/`data-value` and every actor row `data-species`, `data-footing`, `data-activity`, `data-state`, `data-mood`, `data-body` (solid box in viewport px).

### 3.3 Inspector and its seam (dev server only)

- **Seam:** `directorVitePlugin` serves `pets-stories:director`: the whole core plus a mutable `director {advance, frameOf}`; the **layer module's** import of `@semio-tech/pets` (or its alias path) resolves to it, every other import to the core. The document passes `director` to `mountStories`. The product's code is unchanged; no release build (quiz, site, package exports) contains any of it.
- **Reading:** `frameOf` is wrapped: per frame the stage's own invariant `overlaps(bodies)` over the frame bodies, frame counts per show, frames per second; published at most 5×/s.
- **Forces** (folded in after the next `advance` by the stage's own draft functions): state → `enter`; mood → `feeling = {mood, intensity, since: now}` (decays as moods do); trick → `release` + `perform`; activity (pets on a perch) → idle `settle`, greet `hail`, purr `purr`, shrug `shrug`, trick its first, dizzy/fidget/sleep/push `shift` + a clip once. A pet not on a perch refuses tricks and activities ("not now"). The news line says what was applied at which tick.
- **Footings, the learner's way:** the gallery presses the pet with a pointer of its own (`pointerId` 47) and carries it: *Pick up (hand)* (holds 2 s, lets go), *Drop (air)* (120 px up), *Drop from high (chute)* (to the top of the room), *Drop on a neighbour (head)*; *Toss* and the other deeds go through the layer's handle. Walls, ladders and ropes come only with the pets' own routes (B4).

## 4. What I saw (screenshots looked at with the Read tool)

Gallery for the architecture menagerie on 6253, headless Chromium 1440 × 900. Files in `TK/🗑️generated/c2/browser/` (final run), `…/sandbox-again/`, `…/sample/`.

- **Species pages** (`species-<id>.png`, cut into `-partN.png`; looked at chilly in full, the close-ups `close-{sunny,housy,thermy,solary,chilly,battery}-{states,tricks,around}-{light,dark}.png`): rigs, tints and looks per state (housy cold grey with breath, wrapped pink, hot orange; sunny blazing with rising heat, dim, sunset), trick particles (rays, plasma, afterglow orbit, rainbow, count-dots, bolt), purr particles; hang swinging with the hand dot on the scruff, tumble spinning, glide under the own canopies (chilly's snowflake, solary's slats, thermy's striped dome), gun, rope to the hook on its ledge, wall climbs leaning in, the standing ladder against a wall, the mantle corner, carried ladders, push blocks. Ink lines turn paper-coloured on the dark ground. Captions wrap under their stage.
- **Poof** (`poof-<id>-{0,1,2}.png`, clock frozen at three moments): a solid cloud with one ink outline at the start, blobs spreading and separating, fading rings at the end.
- **Sandbox** (`sandbox-*.png`): five pets on card tops, none on the button or the note; radiatory hanging in the hand (`held radiatory`), thrown pumpy under its parachute, sunny blazing with its heat after the circle, dark page.
- **Numbers** (final full run, `check-final.log`): 20 species pages, 2 028 specimens, every one painted; tools seen per species as their gear says (e.g. housy gun/hook/ladder/rope, thermy chute/gun/hook/rope); dust seen on all six close-up species. Sandbox: **overlaps 0 in every frame** (3 962 frames; 4 046 in the extra run), **poofs 0**; three clicks on radiatory → `greet`, `trick`, `trick`; pumpy thrown high → `hand/hang` → `air/tumble` → `chute/glide` → `perch/idle` (extra run; in the final run it left the stage and arrived anew); circling sunny → `greet`, then `trick` → state `shining → blazing` (extra run, mode calm; twice in lively at 14:22/14:31 too; in two lively runs sunny walked away mid-circle and nothing was cued); forced state, mood `grumpy`, trick, `dizzy` all drawn; drop from high → `hand → air/tumble → perch/land` (sunny has no parachute); pick up → `hand/hang`; drop on a neighbour → `head/slide` → `air/fall` → `perch/land`; toss → `hand → air → perch/land → perch/dizzy`.
- **Console:** empty in every run. Only the dev tooling printed (listed apart): Vite's `[vite] connecting...`/`connected.` (debug) and react-dom's development hint to install its DevTools (info). No page error.
- **Frame time** (`c2_frame_probe.mjs`, sunny page): median 16.7 ms with 38 and 71 specimens painted (900/2 400 px viewport); p90 33 ms with 88 painted (5 000 px).

## 5. Commands and real results

| Command (directory) | Result |
|---|---|
| `bun ./📜️script.ts typecheck` (`PR/📦️packages/🟦️typescript`) | exit 0, no output (`typecheck-final.log`) |
| `bun ./📜️script.ts test` (same) | exit 0, `Test Files 8 passed (8)`, `Tests 168 passed (168)` (`test-final.log`; baseline before my work: 8 / 166) |
| `bun x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" effect-painting pet-depiction` (same) | `Test Files 2 passed (2)`, `Tests 25 passed (25)` |
| `bun ./📜️script.ts test pet-companions` (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript`) | exit 0, `Test Files 1 passed (1)`, `Tests 45 passed (45)` |
| `bun ./📜️script.ts test` (`🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`) | exit 0, `Test Files 5 passed (5)`, `Tests 228 passed (228)` |
| `PETS_STORIES_PORT=6253 PETS_MENAGERIE=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-react:dev` (root, background) | `VITE v7.3.6 ready in 401 ms`, `http://127.0.0.1:6253/` → 200 |
| `node TK/c2_gallery_check.mjs --part all --close sunny,housy,thermy,solary,chilly,battery` | exit 0, `failed: []`, `console: []`, `errors: []` (`check-final.log`, `browser/report.json`) |
| `node TK/c2_gallery_check.mjs --part sandbox --out …/sandbox-again` | exit 0, `failed: []`, console empty (`sandbox-again/report.json`) |
| `PETS_STORIES_PORT=6254 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-react:dev` + `node TK/c2_gallery_check.mjs --url http://127.0.0.1:6254/ --part species --roster 3` | sample menagerie: exit 0, 3 pages, 162 specimens all painted, console empty (`sample/report.json`) |
| `node TK/c2_code_rules.mjs` | every file `clean`, exit 0 |
| `node -e "JSON.parse(…🔮️oracles/🔣️.json…)"` | parses |

Both servers were stopped by the PIDs of the chains I started (6253: 56796 … 53672; 6254: 60328 … 20200); both ports are free. Others edited the core during the runs (Vite logged repeated reloads for `🧗️climbing`); the check therefore holds the dev server's `full-reload`/`update` messages back from its page (3 in the final run) so a run keeps the code it loaded.

## 6. Decisions

1. **Dust in `✨️effects`, behind the particles, ink and paper.** The effects root is the layer's stage-level painter (C1b's seam), and a puff has no species to take paints from.
2. **Two circles per blob** (outline under fill) instead of one stroked circle: one ink line round the whole cloud, not a ring per blob.
3. **`data-pet-state` absent for an unknown state** — the attribute names what the drawing shows (the palette then), and an index memo cannot tell two unknown names apart.
4. **The inspector's seam lives in the dev server** (a virtual module replacing the layer's import of the core), not in the layer: no prop, handle or hook was added to the product. Forces use the core's own draft functions, imported by path from the stories only (`📝️draft`, `👀️attention`, `🎥️projection` constants, `🚶️locomotion.PUFF_TICKS`).
5. **Footings are reached the learner's way** (a synthetic pointer and the toss deed), not written into the stage: a written footing would skip the stage's planning and clearance and could break its invariant.
6. **Forced activities only on a perch and only the standing ones** (idle, fidget, sleep, greet, trick, purr, dizzy, shrug, push); motions and partner activities belong to the stage.
7. **A page per species** instead of all species stacked (DOM and paint cost); specimens built lazily, measured before painted.
8. **The first round's spirits slider became mood + intensity** through `faceOf` (lid and head drop included), as the frame now carries mood and intensity.
9. **Sandbox defaults** mode `lively`, capacity 5, tempo ×1; the bench is sticky above the layer (z 50).
10. **Playwright holds HMR messages back** and lists dev-tooling console lines apart; it never hides a page error.

## 7. Open

- **B4:** wall, ladder and rope footings cannot be forced (only the pets' routes reach them); the gallery's species pages show their drawings.
- **B5:** prop rows are on the page and surveyed; nothing was lifted in any run (`lifted fixtures 0`).
- **B1 (observed, not judged):** a fast upward throw sometimes ends with the pet leaving the stage and arriving anew (pumpy, final run); a 490 px drop of sunny (no parachute) landed without `dizzy` once, a toss with `dizzy` once — the threshold is the touch speed (≥ 780 px/s).
- **Gestures in lively:** pets often walk off while the pointer circles them; the check circles in calm and follows the pet.
- **Gallery simplifications:** trick specimens show the trick clip alone (the look of the state is shown in the states section); `tumble` spins steadily; particles of a specimen ignore its tilt.
- Tools kept in `TK` (inputs): `c2_gallery_check.mjs`, `c2_frame_probe.mjs`, `c2_poof_probe.mjs`, `c2_code_rules.mjs`, `c2_crop.py`. `TK/🗑️generated/c2/` holds the final logs, reports and screenshots; it goes with the ticket's generated folder.
