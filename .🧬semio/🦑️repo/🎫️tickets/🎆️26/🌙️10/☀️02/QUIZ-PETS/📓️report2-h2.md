# 📓️ Work package H2 (second round): heat and steam — radiatory, pumpy, boily, kettly, thermy

Ticket `2026/10/02/QUIZ-PETS`, second round, written 2026-10-03 (night). `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `OUT` = `TK/🗑️generated/h2`.

**Status: done.** The five species documents `AP/{♨️radiatory, 🌀️pumpy, 🔥️boily, 🫖️kettly, 🌡️thermy}/🔣️.json` carry real states, tricks, purrs, emitters, gear, grip, reach, canopies and 63 new clips. Edited only with Edit (anchored insertions; valid JSON after every save). New ticket tool: `TK/h2_measure.ts`. No git command that modifies anything, no `bun install`, no cargo, no servers. The repo MCP was unreachable, so no ticket bookkeeping was done from this package.

## 1. Conventions used for all five

- **State ids, trick ids** are exactly those of CONTENT §S/§7 (chemistry writes against them): radiatory `warm cold hot`; pumpy `heating standby cooling defrost`; boily `burning pilot roaring`; kettly `cold lukewarm heating boiling`; thermy `mild cold hot`; tricks named in §7: `flow-temperature` (pumpy), `efficiency-bars`, `relight` (boily), `boast` (kettly), `measure` (thermy).
- **First state = resting state.** The CW/CCW ladder tricks list the ladder in `from` (low → high), so `rungsOf` steps along it even where the resting state sits in the middle (radiatory, boily, thermy, pumpy).
- **Click order** = authored order of `click` tricks (L2, L3 of CONTENT §8.2).
- **No emitter on any resting state** (a resting emitter would keep a still stage awake).
- **Props hidden at rest by occlusion** (parked behind the body, as the first round did): radiatory `towel`, pumpy `frost`, boily `meter`/`meter-fill`. A rest scale of 0 is not expressible (bones have no rest scale, and an idle-loop scale would multiply into every clip).
- **One-shot clips end on rest values**; one-shot fan spins end on a multiple of 960° (÷8 = 120°, invisible on a three-blade fan during the 8-tick fade).
- **`scoot` clips** = the walk cycle at ⅔ of its seconds (the stage scoots at `SCOOT_HASTE` 1.5 × walk speed), so stride per cycle stays slide-free. **`carry` clips** reuse the walk's leg tracks and seconds (assumes carrying at walk speed — the engine does not fix a carry speed yet).
- **Climb clips** are one hold-cycle (two hands); the engine phases them by distance.
- `grip` sits just inside the top outline on the centre axis (where the grabbing cursor pinches); `reach` = how far `greet`/`cuddle`/`squabble` (layered over the idle loop) extend beyond half the width, measured by `TK/h2_measure.ts`, rounded up to 0.5.
- Particles stay ≤ 9 alive per pet in the worst case (state emitter + trick emitter); bursts carry `BURST_GRAVITY`, so bursts are short and aimed away from the face.

## 2. Per pet

### ♨️ radiatory (54×55, walk) — gear `climb, ladder` (refuses grapple, parachute) · grip 39 · reach 4.5 · mood `content`

| States | tint (body/accent/detail) | overlay clip | emitter | lasts → then |
|---|---|---|---|---|
| `warm` Warm/Warm (rest) | — | — | — | — |
| `cold` Cold/Kalt | `#6a8bb3/#6aa0c9/#e8eef5` | `chill` (shimmers parked, arms down, knob to the left stop, outer fins sag, shiver bursts) | — | 60 s → warm |
| `hot` Hot/Heiß | `#ff3b1f/#ffb000/#fff3d0` | `glow` (tall wobbling shimmers, travelling fin wave, knob +120) | `heat-wave` | 40 s → warm |

| Trick | cues | clip | emitter | from → to | mood |
|---|---|---|---|---|---|
| `bleed` Bleed the air/Entlüften | click, shake | `bleed` (arm to the knob, vent screw turned in steps, top fin hisses, shimmer flares, relieved sigh) | `bubbles` | → warm | happy |
| `glow-wave` Heat wave/Wärmewelle | click, show | `glow-wave` (wave through the fins, shimmers flare, ta-da) | `heat-wave` | → hot | proud |
| `turn-up` Turn up/Aufdrehen | circle | `turn-up` (knob +60, +120, shimmers and body swell) | `heat-wave` | cold/warm/hot ↑ | happy |
| `turn-down` Turn down/Zudrehen | countercircle | `turn-down` (knob −120, shimmers shrink, shiver, arm droops) | — | cold/warm/hot ↓ | content |
| `towel-dry` Warm a towel/Handtuch wärmen | whim | `towel-dry` (towel slides out and lands on top, heat goes into it, purring breaths) | `steam` | warm, hot | happy |

Purr `convection-hum` + `warm-dot`. Emitters: `heat-wave` (accent squiggle, rise ×3), `bubbles` (paper/ink, rise ×4 from the knob), `steam` (puff, rise ×4), `warm-dot` (accent dot, rise ×2).
New bone `towel` + parts `towel`, `towel-stripe` (parked behind the fins). New clips: `chill glow bleed glow-wave turn-up turn-down towel-dry convection-hum dangle-knob toss ladder-climb heave ladder-slide ladder-haul push-warm woozy shrug sidestep` → repertoire hang, tumble, climb, mantle, slide, carry, purr, dizzy, shrug, scoot, push.

### 🌀️ pumpy (46×46, walk, no arms) — gear `parachute` (refuses ladders, suction cups, grapple) · grip 43.5 · reach 9 · own canopy (fan-guard dome with grille arcs) · mood `content`

| States | tint | overlay | emitter | lasts → then |
|---|---|---|---|---|
| `heating` Heating/Heizen (rest) | — | — | — | — |
| `standby` Standby/Bereitschaft | `#4a7f78/#5c9c96/#c46a1a` | `hush` (fan stopped against the idle spin, settles, tail down) | — | 60 s → heating |
| `cooling` Cooling/Kühlen | `#2a8fb5/#7ee6f5/#4bc8e8` | `reverse` (fan turns the other way) | `cool-streak` | 40 s → heating |
| `defrost` Defrosting/Abtauen | `#4f9692/#9fd8d4/#fa9500` | `thaw` (frost cap visible on top, melting, slow reverse fan) | `steam-puff` | 8 s → heating |

| Trick | cues | clip | emitter | from → to | mood |
|---|---|---|---|---|---|
| `three-for-one` Three for one/Eins zu drei | click | `three-for-one` (cable zap, cold air drawn in at the back, body swells, three warm pulses out front) | `warm-streak` | → heating | proud |
| `reverse-cycle` Reverse the cycle/Kreislauf umkehren | click | `reverse-cycle` (fan stops with a shudder, restarts backwards, intake blows out, outlet sucks in) | `cool-streak` | heating/standby/defrost → cooling | proud |
| `ramp-up` Ramp up/Hochmodulieren | circle | `rev` (existing) | `warm-streak` | standby/heating ↑ | proud |
| `whisper` Whisper mode/Flüstermodus | countercircle | `whisper` (fan winds down to a stop, settles) | `hush-ring` | standby/heating ↓ | content |
| `flow-temperature` Flow temperature/Vorlauftemperatur | show | `flow-temperature` (outlet stretches towards the partner, tail wags) | `flow-dots` | heating | proud |
| `defrost-shake` Shake off the frost/Frost abschütteln | whim, shake | `defrost-shake` (frost cap pops up, shakes like a dog, fan reversed) | `ice-chips` | heating/standby → defrost | proud |

Purr `compressor-purr` + `purr-ring`. Emitters: `warm-streak`, `cool-streak` (accent, so the trick's streaks are already cool before the state changes), `steam-puff`, `hush-ring`, `flow-dots`, `ice-chips`, `purr-ring`.
New bone `frost` + part `frost` (behind the body). New clips: `hush reverse thaw three-for-one reverse-cycle whisper flow-temperature defrost-shake compressor-purr fan-brake dangle-outlet toss push-blow woozy shrug scurry` → hang, tumble, glide (`fan-brake`: fan spins backwards, legs tucked), purr, dizzy, shrug (front paw raised), scoot, push (blows a warm column, braced).

### 🔥️ boily (48×53, walk) — gear `ladder, grapple` (refuses parachute, suction cups) · grip 41.5 · reach 2.5 · mood `content`

| States | tint | overlay | emitter | lasts → then |
|---|---|---|---|---|
| `burning` Burning/Brennt (rest) | — | — | — | — |
| `pilot` Pilot flame/Zündflamme | `#6b6164/#3a7fd0/#9fc4ee` (blue flame) | `pilot-glow` (tiny flame, needle low, arms droop) | — | 60 s → burning |
| `roaring` Full blast/Volllast | `#8a6454/#ff6a00/#ffe45c` | `roar` (big flickering flame, rumble, needle up, flue rattles) | `embers` | 20 s → burning |

| Trick | cues | clip | emitter | from → to | mood |
|---|---|---|---|---|---|
| `whoomp` Whoomp/Wumm | click, show | `whoomp` (existing) | `embers` | → roaring | proud |
| `smoke-ring` Smoke ring/Rauchring | click, shake, whim | `puff` (existing) | `smoke-rings` | → burning | playful |
| `fire-up` Fire up/Anfeuern | circle | `fire-up` (valve click, flame ducks and grows, fists pump) | `embers` | pilot/burning/roaring ↑ | proud |
| `bank-down` Bank down/Drosseln | countercircle | `bank-down` (yawn and stretch, flame shrinks, one puff from the flue) | — | pilot/burning/roaring ↓ | sleepy |
| `efficiency-bars` Efficiency bars/Wirkungsgrad-Balken | show (and chemistry R49) | `efficiency-bars` (meter bar pops up over the head, fills to 0.8, arms akimbo, foot stamps) | — | — | grumpy |
| `relight` Give a light/Feuer geben | — (chemistry R15 only) | `relight` (bends forward, flame flares, sparks leap ahead) | `spark` | — | proud |

Purr `pilot-purr` + `window-bubbles`. Emitters: `embers`, `smoke-rings`, `spark`, `window-bubbles`.
New bones `meter`, `meter-fill` + parts `meter`, `meter-fill` (behind the body; no digits, a bar glyph only). New clips: `pilot-glow roar fire-up bank-down efficiency-bars relight pilot-purr dangle-flue toss ladder-climb heave abseil ladder-haul aim haul push-belly woozy shrug sidestep` → hang, tumble, climb, mantle, slide, carry, aim, reel, purr, dizzy, shrug, scoot, push.

### 🫖️ kettly (46×44, walk, no arms) — gear `grapple, parachute` (refuses ladders, suction cups) · grip 39.5 (the lid knob) · reach 8 · own canopy (the lid: dome, rim lip, knob) · mood `content`

| States | tint | overlay | emitter | lasts → then |
|---|---|---|---|---|
| `cold` Cold/Kalt (rest) | — | — | — | — |
| `lukewarm` Lukewarm/Lauwarm | accent `#d9808a` (faded handle, cheeks, lamp) | `tepid` (spout droops, cheeks and lamp shrink, lid sags) | `wisp` | 40 s → cold |
| `heating` Heating/Heizt | `#938589/#ff344f/#f7f3e3` | `heat` (lamp and cheeks glow, rattle, lid jiggles) | `spout-dot` | 6 s → boiling |
| `boiling` Boiling/Kocht | `#a07a80/#ff1f3d/#ffffff` | `bubble-boil` (lid hops at 6 Hz, rattle, full cheeks and lamp) | `steam` | 4 s → lukewarm (the click-off) |

| Trick | cues | clip | emitter | from → to | mood |
|---|---|---|---|---|---|
| `boil` Boil/Aufkochen | click, shake | `boil` (existing) | `steam` | → boiling | playful |
| `whistle` Whistle/Pfeifen | click | `whistle` (lid up and fluttering, spout raised, shrill rattle) | `whistle-lines` | → boiling | proud |
| `heat-up` Heat up/Aufheizen | circle | `heat-up` (cheeks fill, lamp brightens, lid hops) | `spout-dot` | cold/lukewarm/heating/boiling ↑ | happy |
| `cool-down` Cool down/Abkühlen | countercircle | `cool-down` (a long sigh, spout droops) | `wisp` | same ladder ↓ | content |
| `boast` Show off/Angeben | show, whim (and R17) | `boast` (chest out, lamp flashes, lid lifted like a hat) | `steam-column` | — | proud |
| `pour` Pour a cup/Eingießen | whim | `pour` (tips forward on its base, pours a thin stream) | `drops` | — | happy |

Purr `singing-kettle` + `purr-puff`. Emitters: `steam`, `spout-dot`, `wisp`, `whistle-lines`, `steam-column`, `drops`, `purr-puff`.
No new bones. Glide `lid-chute`: the own lid shrinks away (scale 0.02) while the lid-shaped canopy is open — the lid became the parachute; cords meet at the knob height as a thin tether. New clips: `tepid heat bubble-boil whistle heat-up cool-down boast pour singing-kettle lid-chute aim-spout reel-handle scramble dangle-handle toss push-bump woozy shrug scurry` → hang, tumble, glide, aim, reel, mantle, purr, dizzy, shrug, scoot, push.

### 🌡️ thermy (32×52, walk) — gear `grapple, parachute` (refuses climbing, ladders) · grip 48 (top of the tube) · reach 1.5 · own canopy (graph-paper chart: grid and a reading curve) · mood `content`

| States | tint | overlay | emitter | lasts → then |
|---|---|---|---|---|
| `mild` Mild/Mild (rest) | — | — | — | — |
| `cold` Cold/Kalt | `#2f6fae/#ff344f/#e8e2c8` | `chill-read` (column a short stub, arms hugging, shiver bursts) | `breath` | 30 s → mild |
| `hot` Hot/Heiß | `#d8663f/#ff344f/#f2e2c8` | `heat-read` (long red column almost to the top, fanning itself) | `sweat` | 20 s → mild |

| Trick | cues | clip | emitter | from → to | mood |
|---|---|---|---|---|---|
| `self-reading` Self-reading/Eigenmessung | click | `reading` (existing) | `tick-pops` | → mild | curious |
| `tap-check` Tap check/Klopfprobe | click, shake | `tap` (existing) | `tap-star` | — | proud |
| `warm-read` Read warmer/Hochmessen | circle | `warm-read` (column rises a third, little cheer) | `tick-pops` | cold/mild/hot ↑ | happy |
| `cool-read` Read cooler/Runtermessen | countercircle | `cool-read` (column drops, shiver) | `breath` | cold/mild/hot ↓ | content |
| `measure` Take a reading/Messen | whim, show (R57 sets the state) | `measure` (turns its tube to the partner, column searches and settles) | `probe-dots` | — | curious |

Purr `steady-hum` + `bulb-ring`. Emitters: `breath`, `sweat`, `tick-pops`, `tap-star`, `probe-dots`, `bulb-ring`.
No new bones. New clips: `chill-read heat-read warm-read cool-read measure steady-hum chart-sink aim-tube reel-thread scramble dangle-tube toss push-flick woozy shrug scurry` → hang (column slides down), tumble, glide, aim, reel, mantle, purr, dizzy, shrug, scoot, push (tube held against the chip, flicked).

## 3. Contrast of the body colour (WCAG ratio, `OUT/measure.txt`)

| Pet / state | body | vs `#f7f3e3` | vs `#001117` |
|---|---|---|---|
| radiatory warm / cold / hot | `#ff344f` / `#6a8bb3` / `#ff3b1f` | 3.22 / 3.17 / 3.20 | 5.36 / 5.45 / 5.40 |
| pumpy heating / standby / cooling / defrost | `#1e9b8d` / `#4a7f78` / `#2a8fb5` / `#4f9692` | 3.08 / 4.11 / 3.32 / 3.09 | 5.61 / 4.21 / 5.21 / 5.59 |
| boily burning / pilot / roaring | `#705c54` / `#6b6164` / `#8a6454` | 5.64 / 5.37 / 4.68 | 3.06 / 3.22 / 3.69 |
| kettly cold, lukewarm / heating / boiling | `#808d98` / `#938589` / `#a07a80` | 3.06 / 3.17 / 3.38 | 5.66 / 5.45 / 5.12 |
| thermy mild / cold / hot | `#2f7fae` / `#2f6fae` / `#d8663f` | 3.96 / 4.71 / 3.21 | 4.36 / 3.67 / 5.39 |

Every body colour is ≥ 3:1 on both pages; the 2 px ink outline is on top. CONTENT's proposals that failed were replaced: radiatory cold `#6f8fb5` (3.01) → `#6a8bb3`; boily pilot `#5f4f48` (2.47 on dark) → `#6b6164`; kettly boiling `#a39a9c` (2.47 on light) → `#a07a80`; pumpy defrost `#7fb8b5` (2.0 on light) → `#4f9692` with the icy well `#9fd8d4` (CONTENT's `#e8f7f5` made the paper fan blades vanish on the dark page).

## 4. Validation (real output)

```
$ bun TK/validate_species.ts <the five files>          (OUT/validate.txt)
ok   🎓️teaching/🏛️architecture/🐾️pets/♨️radiatory/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌀️pumpy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔥️boily/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🫖️kettly/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json
validate exit 0

$ bun ./📜️script.ts test        (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript; OUT/site-test.txt)
 Test Files  5 passed (5)
      Tests  138 passed (138)
exit 0
```

The site suite includes `🐾️pet-cast` (ajv draft-07 + the product's validators on every species, the ensemble and the assembled menagerie). One earlier run of the same command failed with "esbuild: The service was stopped" and `[budget] … exceeded 15000ms — killed` in five suites under machine load from parallel agents; the immediate rerun passed — no species file was named. Not run: cargo, e2e gates, deploy checks, the pets product's own suites (no code changed).

## 5. Final sheets (all in `OUT`)

- `five-states.png` (scale 1.5) and `five-states-1x.png` (scale 1) — the five side by side with every state, both themes.
- `final-<id>-<n>.png` for radiatory (10 pages), pumpy (8), boily (12), kettly (10), thermy (9): rest, states, tricks and purr, all new activities with their gear.
- `measure.txt` — extents of every clip over the idle loop, the reach derivation, contrast per state.

## 6. What still bothers me

1. **Engine assumptions** that the art depends on and that B1–B4 decide: state overlays are layered additively on the idle loop like activity clips (props such as pumpy's frost cap and the shimmer parking rely on it); trick emitters start with the trick (bursts fire at its first tick, so burst tricks are timed for that); `carry` moves at walk speed; the `mantle` clip is also played at the ladder exit and after a reel.
2. **At 1×** pumpy `standby` vs `heating` and kettly `lukewarm` vs `cold` differ mainly by tint, posture and (pumpy) a stopped fan that only shows in motion; boily `pilot` reads through the small blue flame only.
3. **Radiatory's arms are 6 px stubs** drawn behind the fins: climbing reads as alternating up-out arms plus stepping legs, not as hands on rungs; the "dangle at the knob" is a centre grip (grip has no x).
4. **Pumpy's fade-outs**: a loop that ends while the outlet streak is out (push) slides it home over 8 ticks; `whisper` ends 120° ahead, so the fan nudges a third of a turn before `standby` stops it.
5. **Kettly `pour` and `push-bump`** move the body off its base by a few px (the base is on `root`); fine at 1×, visible enlarged.
6. **Bursts with gravity** (pumpy `ice-chips`, thermy `tick-pops`) can drift past the face for their last frames; they are faint by then.
7. **Tricks per pet** are 5–6, one over the brief's 3–5 for pumpy, boily and kettly, because §7 names `flow-temperature`, `efficiency-bars`, `relight`, `boast` and the defrost state needs an entry trick; `relight` is chemistry-only (no cue).
