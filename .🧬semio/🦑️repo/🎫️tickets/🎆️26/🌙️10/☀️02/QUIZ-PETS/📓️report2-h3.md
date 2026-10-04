# 📓️ Work package H3 (second round, art): the envelope — housy, waly, roofy, insuly, shady

Ticket `2026/10/02/QUIZ-PETS`, second round, written 2026-10-03. `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `OUT` = `TK/🗑️generated/h3`.

**Status: done.** The five species documents carry real states, tricks, purrs, emitters, gear, grip, reach, canopies and the clips of every new activity (98 new clips in all). All five pass the product's validator (exit 0) and the site suite is green (5 files, 138 tests; `🐾️pet-cast` alone 80 tests). Only the five `AP/<pet>/🔣️.json` files were edited, with Edit only; tools and output live under `TK`. No git command that modifies anything, no `bun install`, no cargo, no e2e gates, no servers. The repo MCP was not used (unreachable in this session); no ticket bookkeeping.

## 1. Conventions this package fixed

1. **Props hidden at rest.** The schema has no rest scale on a bone, so a hidden prop is a part on its own leaf bone whose geometry is authored at **1/1000** of its size (stroke widths too, e.g. `0.002` for a 2 px outline); a clip shows it by scaling that bone to **1000** (×1000 = its real size, a 0.04 px speck at rest). A 1/10⁶ part on a child of such a bone needs both scales (waly's insulation stripe inside the cut-away, §3.2).
2. **Who may scale a hidden bone up.** State props (housy `coat`, `icicles`; waly `coat`, `coat-outer`, `section-wool`; roofy `snow`, `chimney-snow`, `sheen`; insuly `batt2`) are scaled up **only by their state's overlay clip**; trick props (housy `jacket`, `label`; waly `section`, `meter`; insuly `blanket`; shady `shadow`) **only by their trick clip**. Other clips may translate a state prop or scale it by a factor ≤ 1 (housy `air-out` drops the coat, roofy `sun-bake` melts the snow), never up — layering multiplies scales.
3. **Engine rules from the coordinator (applied):** a state clip overrides the idle loop per keyed channel (so overlays key breathing where they replace a breathing channel — added to housy `chill`/`fan`, waly `glow`, roofy `snow-shiver`, insuly `squash`/`bulk`); a trick's `to` applies when the trick ends, its emitter starts with it (housy `pull-on-coat` ends with the jacket where the `wrapped` coat appears; `air-out` drops the overlay's coat before `cosy` takes over); the first state is the resting one (CONTENT's default ids: `cosy`, `bare`, `dry`, `fluffy`, `lowered`).
4. **Ladders.** Circle tricks list their rungs in `from` (lowest first) and leave `to` out, so the engine steps one rung (`rungsOf`/`stepRung`); where one clip cannot serve every step, a step gets its own trick with `from` + `to` (housy `warm-through` cold → cosy beside `pull-on-coat` cosy → wrapped). `whim`/`show` only where CONTENT has AUTO or DUET; duet-only tricks have **no cues** (chemistry asks for them): waly `u-value-duel`, shady `sun-block`.
5. **Particles** use one kit in the species paints with a thin ink outline so they read on both themes: puff, donut ring, drop, chip, vertical heat-wave band, streak, star4, dot. At most 8 live per emitter (bursts), typically 1–4; one emitter runs per state/trick/purr.
6. **Contrast.** Every body colour (palette and tint) was measured against `#f7f3e3` and `#001117` (`TK/h3_measure.ts`, WCAG ratio). Three CONTENT tints failed 3:1 and were adjusted within their hue: housy `cold` `#a88f73` → `#9a8775`, housy `hot` `#d99a55` → `#c8702c`, roofy `wet` `#3f5265` → `#4d6178`. Every body colour now has ≥ 3:1 on both pages (§4); accents/details below 3:1 keep the 2 px ink outline as before.
7. **The shared wool coat:** insuly's pink `#e0617f` with paper (`#f7f3e3` = wool-light) batting waves. Since `Paint` has no wool tokens, the wrapped states tint one slot to `#e0617f` and the coat is filled with it: housy tints `body` (coat fill `body`), waly tints `detail` (coat fill `detail`, so its brick layer can stay brick-red). Same puffy outline, same batting on both.
8. **reach** = how far the encounter clips (greet, cuddle, squabble) reach beyond the half-width on the facing side, measured over every tick laid over the idle loop, + ~2 px for the engine's gaze lean, rounded up to 0.5. **grip** = the scruff on x = 0 (housy roof ridge, waly cap, roofy ridge, insuly top tufts, shady rail).

## 2. Tools (in `TK`)

| Tool | What |
|---|---|
| `h3_measure.ts` (bun) | contrast of every palette/tint vs both pages; rest box; props hidden at rest; per clip the reach beyond the box (left/right/top/bottom, laid over the idle loop at four offsets); `reach` needed; one-shot clips that start/end off rest. `--clips` lists every clip. |
| `h3_overlay.ts` (bun) | writes a copy of a species with `<overlay>-under-<clip>` clips (union of tracks) so the preview can show a trick played while a state is on (used to prove waly's state-dependent insulation stripe). |
| `render_species_preview.mjs` (A9's) | all sheets. It was broken twice by siblings' in-flight changes (`hail` export in `👀️attention`, `spirits` in the frame); both were fixed by others before I touched it — I did not edit it. |

## 3. Per pet

### 3.1 housy — 48×52, gear ladder + grapple (refuses parachute), grip 49, reach 8, mood content

- **States** `cosy` Behaglich (rest) · `cold` Ausgekühlt (tint `#9a8775/#8c2a30/#bcc6c9`, overlay `chill`: four icicles under the eaves, arms hugging, shiver bursts, door flaps; emitter `breath`; 45 s → cosy) · `wrapped` Gedämmt (tint body `#e0617f`, overlay `snug`: the puffy pink coat with batting, door and windows left free; 90 s → cosy) · `hot` Überhitzt (tint `#c8702c/#c40010/#f0d4b4`, overlay `fan`: arms fanning, door ajar, panting; emitter `sweat`; 30 s → cosy). The energy-label pointer moves with the state (wrapped top bar, cosy middle, hot second-lowest, cold lowest) — the overlays translate `label-pointer`.
- **Tricks** `energy-label` [click] (door opens, a paper certificate with seven stepped red arrows slides out on the left arm, the pointer wiggles; proud) · `smoke-rings` [click, whim, show] (three chimney pumps, puffs pop, `rings` rise; playful) · `pull-on-coat` [circle] from cosy/wrapped → wrapped (arms up, a quilted coat is pulled down from under the eaves in three tugs; proud) · `warm-through` [circle] from cold → cosy (existing `smoke-ring` + `rings`; happy) · `air-out` [countercircle] rungs cold/cosy/wrapped (door flung open, `gust` streaks leave the right side, a worn coat slides down and crumples; content) · `heat-loss` [whim] from cosy → cold (the roof lifts 8 px and tilts 15° like a lid, `heat-waves` escape; sad) · `rattle-roof` [shake] (roof jumps off and is caught, `dust`; scared). Click order L2 energy-label, L3 smoke-rings.
- **Purr** `chimney-content` (body swell, roof rock, door ajar, arms hug) + `purr-ring`.
- **Emitters** breath (drift puff ×1), sweat (fall drop ×2), rings (rise donut ×3), gust (drift streak ×5), heat-waves (rise wave ×4), dust (burst chip ×5), purr-ring (rise donut ×2).
- **New bones/parts** `icicles` (roof), `coat`, `jacket`, `label` > `label-pointer` (body); parts `coat`, `batting`, `jacket`, `jacket-batting`, `label`, `label-bars`, `label-pointer`, `icicles`.
- **New clips** chill, snug, fan, energy-label, smoke-rings, pull-on-coat, air-out, heat-loss, rattle-roof, chimney-content, dangle-chimney (hang), tumble, chimney-aim (aim: the chimney tips 40° as the cannon, recoil puff), winch (reel), ladder-climb, mantle, downpipe-slide (slide), ladder-carry (carry; waddle legs kept slide-free), push-shoulder (push), dizzy, shrug, scoot.
- **Push**: leans 12–16° into the fixture with the right wall, right arm braced, back leg scrabbling and re-planting, four shoves per 1.6 s with squash and a chimney huff, then a cheeky breather (roof tips, left arm waves) before the next round.
- **Bothers me**: arms are drawn behind the wall, so arms raised past ~100° vanish (pull-on-coat, rattle-roof); the jacket is cream while pulled on and the worn coat pink (it recolours at the hand-off); `heat-loss` as a whim fires only in a willing mood (the engine's `WILLING_MOODS`), not "AUTO(sad)".

### 3.2 waly — 48×46, gear ladder (refuses grapple, parachute), grip 41.5, reach 8, mood content

- **States** `bare` Ungedämmt (rest) · `warm` Aufgewärmt (tint `#c95a2c/#f5b971/#6a5a56` — glowing joints; overlay `glow`; emitter `heat`; 90 s → bare) · `wrapped` Gedämmt (tint detail `#e0617f`, overlay `wrap`: the pink coat; 120 s → bare) · `passive` Passivhaus-gedämmt (same tint, overlay `wrap-double`: coat + a wider outer coat; 120 s → wrapped). wrapped/passive are reached only through insuly (R42).
- **Tricks** `brick-swap` [click] (existing `loose-brick` + `brick-dust`; proud) · `layer-reveal` [click, show] (a cut-away slice slides out of the right side: plaster, brick with joints — and an insulation stripe that exists only when wrapped (4 px) or passive (8 px), driven by the state overlays scaling `section-wool`; the forearm points and taps; proud) · `warm-up` [circle] rungs bare/warm (arms up, basking, `warm-waves`; content) · `cool-down` [countercircle] rungs bare/warm (arms crossed, shivers, `warm-waves` leave; content) · `salute` [countercircle] from wrapped (existing `salute`, no state change — CONTENT's "a CCW turn is a salute and nothing else"; proud) · `u-value-duel` [no cue] (a meter with a short bar over its head, arms crossed, smug huff; grumpy) · `crumble` [shake] (shakes, the loose brick falls out onto the ground and regrows, `crumbs`; grumpy).
- **Purr** `thermal-mass-hum` + `hum-dot`.
- **Emitters** heat (rise wave ×1, 2 s), warm-waves (rise wave ×3), brick-dust (burst chip ×3), crumbs (burst chip ×4), hum-dot (rise dot ×2).
- **New bones/parts** `coat-outer`, `coat`, `section` > `section-wool`, `meter`; parts coat-outer, coat, batting, section-plaster, section-brick, section-joints, section-frame, section-wool, meter, meter-bar.
- **New clips** glow, wrap, wrap-double, layer-reveal, warm-up, cool-down, u-value-duel, crumble, thermal-mass-hum, dangle-cap (hang: rigid, legs barely swing, the loose brick wiggles out, falls 40 px and regrows behind the wall), tumble, ladder-climb (heavy, a thud and a brick jolt per rung), mantle, ladder-slide, ladder-carry (march legs kept), push-shoulder (the slowest push: 2 long strains per 2.4 s, shoulder at 14°, back leg scrabbling, the loose brick creeping out with the effort and slipping back), dizzy, shrug, scoot.
- **Bothers me**: `u-value-duel` draws only waly's short bar (windowy's long bar is H4's); in `passive` a countercircle does nothing (`salute` is offered in `wrapped` only, since a two-rung `from` would step passive → wrapped).

### 3.3 roofy — 56×42, gear ladder + parachute (refuses grapple), grip 40, reach 3, own canopy, mood content

- **States** `dry` Trocken (rest) · `snow-capped` Schneebedeckt (no tint; overlay `snow-shiver`: a paper snow cap with two drips on the ridge and a tuft on the chimney pot, hunched, hugging arms; emitter `breath`; 60 s → dry) · `sun-baked` Sonnenheiß (tint `#7a6a74/#d4552e/#3a3030`, overlay `bake`: limp, panting, tile rows flutter; emitter `shimmer`; 30 s → dry) · `wet` Nass (tint `#4d6178/#8c3b26/#1f2a2c`, overlay `soggy` with four paper sheen strokes on the slopes; emitter `drips`; 15 s → dry).
- **Tricks** `tile-flip` [click] (the loose tile lifts to 62° and spins edge-on like a coin, lands with a hop; `glint`; proud) · `tile-wave` [click] (existing `ripple` + `tile-chips`; playful) · `sun-bake` [circle] rungs snow-capped/dry/sun-baked (turns its face up, arms spread; melts a present snow cap; `shimmer`; content) · `shake-off` [countercircle] same rungs (dog shake ±8° at 6 Hz, `fling` drops; content) · `carry-pv` [show, whim] (crouch, then stands tall as if solary had hooked on; happy) · `clatter` [shake] (`tile-pop`: the tile pops off spinning and returns; scared).
- **Purr** `tile-purr` (tile rows breathe in opposition, tile and chimney nod) + `chimney-puff`.
- **Canopy** a terracotta gable roof with a scalloped eave and two tile rows (60 × 22): the "umbrella roof".
- **Emitters** breath, shimmer (rise wave ×3), drips (fall drop ×2), glint (burst star ×2), tile-chips (burst chip ×4), fling (burst drop ×8), chimney-puff (rise puff ×2).
- **New bones/parts** `snow` (body), `chimney-snow` (chimney), `sheen` (body); parts snow, chimney-snow, sheen.
- **New clips** snow-shiver, bake, soggy, tile-flip, sun-bake, shake-off, carry-pv, tile-pop, tile-purr, dangle-chimney (hang: the tile swings ±25°, legs dangle, arms flail), tumble, umbrella-sink (glide), ladder-climb, mantle, ladder-slide, ladder-carry (scamper legs kept), push-tile (lean into the fixture with the eave, the tile levering down at each shove, back leg scrabbling, a hop at the end), dizzy, shrug, scoot.
- **Bothers me**: `wet` is darker + sheen + drips, still the subtlest state at 1×; the lever tile sits high on the roof and does not touch a short fixture; the eave arms are tiny, so climbing reads mostly through the legs.

### 3.4 insuly — 48×40, hop, gear climb + parachute (refuses ladder, grapple), grip 36, reach 4, own canopy, mood content

- **States** `fluffy` Flauschig (rest) · `compressed` Zusammengedrückt (overlay `squash`: 0.78 × 1.12, tufts 0.7, arms tucked; 30 s → fluffy) · `thick` Dick (overlay `bulk`: a larger scalloped batt behind the body, body 1.06, tufts 1.2 breathing; 40 s → fluffy) · `soaked` Durchnässt (tint `#b04a63/#c8d6df/#7a3548`, overlay `droop`: tufts sag, body sinks; emitter `drip`; 40 s → fluffy).
- **Tricks** `fluff-burst` [click] from fluffy/thick → thick (existing `fluff-up` + `fibres`; playful) · `tuck-in` [click] (`blanket-tuck`: a pink blanket with a cream stripe unrolls from the right hand and drapes over whoever stands ahead, two pats; proud — the chemistry trick of R41–R43) · `bulk-up` [circle] rungs compressed/fluffy/thick (tufts swell one after another) · `flatten` [countercircle] same rungs (presses itself flat; sleepy) · `dry-shake` [whim, click, circle, countercircle] from soaked → fluffy (wet-dog shake, `shake-drops`, fluffs up; happy) · `itch` [shake] (scratches with arm and leg, tufts bristle, `fibre-burst`; grumpy). Click order L2 fluff-burst, L3 tuck-in (in soaked: tuck-in, then dry-shake).
- **Purr** `fibre-purr` (tufts breathe in a ripple left → right, arms hug, tail rolls) + `purr-fibre`.
- **Canopy** a cream dandelion clock (scalloped puff, 35 px, with its seed centre).
- **Emitters** drip (fall ×2), fibres (rise puff ×6), shake-drops (burst ×8), fibre-burst (burst dot ×8), purr-fibre (rise dot ×2).
- **New bones/parts** `batt2`, `blanket` (body); parts batt2 (drawn first, behind everything), blanket, blanket-stripe.
- **New clips** squash, bulk, droop, blanket-tuck, bulk-up, flatten, dry-shake, itch, fibre-purr, dangle-tuft (hang: pear-stretched, tail swings, legs dangle, serene), tumble, fibre-cling (climb: pressed to the wall, inchworm stretch 0.94 ↔ 1.18 with alternating reach), mantle, fibre-slide (slide), dandelion-sink (glide), push-tuft (push: squishes against the fixture — side tuft squashed to 0.65 —, legs pedal fast, then a cheeky hop with a tuft puff), dizzy, shrug; scoot reuses `hop`.
- **Bothers me**: `thick` is ~3 px wider each side and 4 px taller than the size box (it is a thicker layer; the clearance uses the box); `compressed` squashes the eyes into ellipses; the dandelion and the blanket stripe are cream on the cream page and rely on their outline.

### 3.5 shady — 44×48, gear ladder + parachute (refuses grapple — its cord is its hook), grip 43, reach 9, own canopy, mood content

- **States** `lowered` Heruntergelassen (rest) · `raised` Hochgezogen (overlay `rolled-up`: the rail sinks 16 px onto a blind gathered to 0.45, sunglasses pushed up onto the rail, cord a bit longer; 40 s → lowered) · `tilted` Gewendet (overlay `tilt-slats`: slats thinned to 0.4 and angled 6°, dark gaps — cream on the dark theme, so light comes through; emitter `daylight`; 40 s → lowered). No tints (CONTENT has none).
- **Tricks** `slat-wave` [click] → tilted (existing clip + `light-shafts`; playful) · `roll-down` [click] from lowered/tilted → lowered (existing `roll` + `glint`; proud) · `lower` [circle] rungs raised/tilted/lowered (cord pull, concertina drop and bounce, slats rattle; content) · `raise` [countercircle] same rungs (two cord pulls gather the blind, glasses pushed up onto the rail with a `huff`; content) · `sun-block` [no cue] (a dark shadow patch spreads on the perch ahead, glasses lowered; proud — R35's duet) · `pose` [whim, show] (leans back, chin up, glasses tilt, `glint`; proud — R35's `pose`) · `clatter` [shake] (`glasses-fumble`: the glasses fall to the lower slats and are pushed back; grumpy).
- **Purr** `cool-purr` (slats flutter in a cascade, lag 0.15) + `lens-glint` (a twinkle on the lens).
- **Canopy** a grey striped awning (Markise) with a scalloped valance, 36 × 16.
- **Emitters** light-shafts (drift streak ×3), daylight (drift streak ×1), glint (burst star ×2), huff (burst puff ×3), lens-glint (rise star ×1, speed 1 = a twinkle in place).
- **New bones/parts** `shadow` (root); part shadow.
- **New clips** rolled-up, tilt-slats, lower, raise, sun-block, pose, glasses-fumble, cool-purr, dangle-cord (hang: the blind hangs and clatters, glasses slip down), tumble, awning-sink (glide: the blind swings like a pendulum under the rail), ladder-climb (no arms: legs climb, the cord hauls), mantle, ladder-slide, ladder-carry (shuffle legs kept), push-shade (no arms: leans in and swings the blind's bottom against the fixture, concertina squash at each shove, legs scrabble, then lowers the glasses for a cool look over the rim), dizzy, shrug, scoot.
- **Bothers me**: `lower`/`raise` are gestures; the state itself changes when the trick ends (the overlay switch), which reads well only if the engine blends overlays in; `raised` lowers the body by 16 px in the overlay — if walking blends the idle (and with it the state override) away, a raised shady pops to full height while it walks; on the dark theme the `sun-block` shadow shows only by its outline.

## 4. Contrast (body colour, WCAG ratio; L = on `#f7f3e3`, D = on `#001117`)

| Pet | State: body (L / D) |
|---|---|
| housy | cosy `#b87f46` 3.06 / 5.64 · cold `#9a8775` 3.10 / 5.58 · wrapped `#e0617f` 3.05 / 5.66 · hot `#c8702c` 3.25 / 5.32 |
| waly | bare `#a3472f` 5.40 / 3.20 · warm `#c95a2c` 3.79 / 4.56 · wrapped, passive `#a3472f` 5.40 / 3.20 (coat `#e0617f` 3.05 / 5.66) |
| roofy | dry, snow-capped `#5d7185` 4.53 / 3.81 · sun-baked `#7a6a74` 4.56 / 3.79 · wet `#4d6178` 5.73 / 3.02 |
| insuly | fluffy, compressed, thick `#e0617f` 3.05 / 5.66 · soaked `#b04a63` 4.71 / 3.67 |
| shady | all `#8a5fb8` 4.28 / 4.04 |

Accents and details are listed in `OUT/measure.txt`; those under 3:1 (e.g. housy roof `#a60009` 2.40 on dark, shady rail grey 1.58 on light) keep the 2 px ink outline, which is ink on light and paper on dark.

## 5. Measurements (`OUT/measure.txt`)

- **reach**: encounter clips reach 5.96 (housy, `rattle`), 5.78 (waly, `lean-on`), 0.76 (roofy), 1.86 (insuly), 7.38 (shady, `clatter`) px beyond the half-width → documents 8, 8, 3, 4, 9.
- **Ground**: no clip of a grounded activity draws below the feet line (fixed on the way: waly `crumble` brick, insuly `dizzy` tail, shady `sun-block` shadow and `push-shade` front foot). Clips with dangling legs (hang, reel, glide) reach 0.8 px below the feet by design; insuly `bulk` reports 1.8 px only because the tool bounds curves by their control points (the curve itself ends ~1 px above the feet).
- **One-shot clips ending off rest (on purpose, hand-offs at the state change):** housy `pull-on-coat` (jacket stays on), housy `air-out` (coat stays dropped), roofy `sun-bake` (snow stays melted).
- **Hidden at rest** (speck ≤ 0.2 px): housy coat, batting, jacket, jacket-batting, label, label-bars, label-pointer, icicles; waly coat-outer, coat, batting, section-*, meter, meter-bar; roofy snow, chimney-snow, sheen; insuly batt2, blanket, blanket-stripe; shady shadow.

## 6. Verification (real results)

```
$ bun TK/validate_species.ts AP/🏠️housy/🔣️.json AP/🧱️waly/🔣️.json AP/🛖️roofy/🔣️.json AP/🧶️insuly/🔣️.json AP/😎️shady/🔣️.json
ok   …/🏠️housy/🔣️.json   ok   …/🧱️waly/🔣️.json   ok   …/🛖️roofy/🔣️.json   ok   …/🧶️insuly/🔣️.json   ok   …/😎️shady/🔣️.json
exit 0                                                     (OUT/validate.txt)

$ bun ./📜️script.ts test            (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript)
 Test Files  5 passed (5)
      Tests  138 passed (138)        exit 0               (OUT/site-test.log)
$ bun ./📜️script.ts test pet-cast
 Test Files  1 passed (1)
      Tests  80 passed (80)          exit 0               (OUT/site-test-pet-cast.log)
```

One intermediate run of the site suite (between those two green runs) exited 1: the `[budget]` guard killed vitest after 15 s while the machine was loaded by siblings ("Worker exited unexpectedly", esbuild "The service was stopped" in `📰️host-document`); no assertion failed and the immediate rerun was green.

## 7. For the engine packages and the coordinator

1. **Overlay blending at state changes.** States are carried by tints and overlay clips; tricks are gestures that return to rest (except the three hand-offs of §5). A state change at the end of a trick therefore shows as the overlay switch — please blend overlay clips in and out (and the tint) over a few ticks.
2. **Overlays during replacing activities.** If `walk`/`hop`/`fall`/`land`/`sleep` blend the idle loop — and with it the state's overrides — towards rest, states that move the body (shady `raised`, insuly `compressed`/`thick`) pop to rest pose while walking. Consider keeping the state overrides under replacing clips.
3. **Trick length.** Trick clips last 1.2–3.0 s (`u-value-duel`, `carry-pv` 3.0; `energy-label`, `smoke-rings`, `layer-reveal`, `blanket-tuck`, `dry-shake`, `sun-block` 2.4). With A2's fixed trick dwell of 128 ticks the longer ones would be cut; dwell = clip length is what they are drawn for.
4. **Chemistry ids** (exactly CONTENT's): housy states `cosy cold wrapped hot`, tricks `energy-label smoke-rings pull-on-coat warm-through air-out heat-loss rattle-roof`; waly `bare warm wrapped passive`, `brick-swap layer-reveal warm-up cool-down salute u-value-duel crumble`; roofy `dry snow-capped sun-baked wet`, `tile-flip tile-wave sun-bake shake-off carry-pv clatter`; insuly `fluffy compressed thick soaked`, `fluff-burst tuck-in bulk-up flatten dry-shake itch`; shady `lowered raised tilted`, `slat-wave roll-down lower raise sun-block pose clatter`. Cue-less (chemistry-only): waly `u-value-duel`, shady `sun-block`.
5. **Hidden-prop scales.** Any engine path that scales a bone on its own (none does today) would multiply the ×1000 of §1.

## 8. Paths

- Species: `AP/🏠️housy/🔣️.json`, `AP/🧱️waly/🔣️.json`, `AP/🛖️roofy/🔣️.json`, `AP/🧶️insuly/🔣️.json`, `AP/😎️shady/🔣️.json`.
- Tools: `TK/h3_measure.ts`, `TK/h3_overlay.ts`.
- **Final sheets** (`OUT/final/`): the five side by side with their states `envelope-states-x2.png` (2×) and `envelope-states-x1.png` (page size); per pet `<pet>-states-tricks-<n>.png` (both themes, 3×) and `<pet>-gear-<n>.png` (light, 3×).
- Working sheets and logs: `OUT/*.png`, `OUT/waly-combo.json` + `OUT/waly-combo.png` (wrapped/passive under `layer-reveal`), `OUT/measure.txt`, `OUT/validate.txt`, `OUT/site-test*.log`.
