# 📓️ Work package H4 (second round): glass, air and cold — windowy, solary, venty, chilly, servy

Ticket `2026/10/02/QUIZ-PETS`, second round, phase C, written 2026-10-03. `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `OUT` = `TK/🗑️generated/h4`.

**State: done.** The five species documents carry real states, tricks, purrs, emitters, gear, grip, reach, canopies, resting moods and every clip of the new activities. They follow the coordinator's engine rules for state clips and trick timing (§3 point 7). All five are valid (`validate_species` exit 0), and the site suite is green (5 files, 138 tests; `🐾️pet-cast` alone 80 tests); both were rerun after the last change. Only the five `AP/<species>/🔣️.json` files were edited, always with Edit, and each step was validated straight after the save. In `TK` I added one tool, `h4_measure.ts`, and fixed one line of the preview tool. No git command was run, nothing was installed, and no server was started.

## 1. How I worked and judged

| Tool | What it does |
|---|---|
| `TK/render_species_preview.mjs` (A9) | `--states`, `--tricks`, `--gear`, `--clips` sheets at 3× and 2× scale (1× for the final roster), on the light and dark themes. Every sheet was looked at with the Read tool, usually several times per pet. |
| `TK/h4_measure.ts` (new) | Uses the product's own `sampleClip` and `solveRig`. Reports WCAG contrast for every state's colours against `#f7f3e3` and `#001117`, and the drawn extent of any clip at every tick (stroke included, curves sampled). `reach` is measured from the encounter clips, both as authored and leant by the stage's gaze lean (`leant` of `👀️attention`: 1.5 px and 5° on the first eye's bone). It also lists one-shot clips that do not start and end at rest. |

**Hidden props.** Bones have no rest scale. An additive clip multiplies scale, so a prop hidden by an idle track of scale 0 could never be shown again. I therefore used two techniques:
- **Occlusion:** the prop rests behind a part that is drawn later, and the clip slides it out. This covers windowy's cups (behind the openings), doodle and glazing lines (behind the sill), solary's hooks (behind the panel) and chilly's axes (behind the body).
- **Tiny geometry:** fill-only props are authored at 1/1000 size (invisible) and scaled ×1000 by the clips. This covers solary's three cell rows and venty's bypass flap. It works only without a stroke, because a stroke would scale too.

I swept every first-round clip of windowy, solary and chilly (`OUT/sweep-*.png`): no prop peeks out. While doing so I moved solary's hooks inwards, because the 22° tilt of `sunbathe` uncovered them, and chilly's axes downwards, because the squash of `skid` uncovered them.

**Climbing geometry.** I followed the engine (`🧗️climbing`): the box edge touches the wall (`clingOf`), and one climb cycle covers two holds of 0.3 heights. Each climbing prop is planted for half a cycle and moves down linearly in the body's frame by 0.3 h, so nothing slides on the wall. Climbing props hang on `root`, so the idle breathing of `body` cannot shift a planted cup. `mantle` lasts `MANTLE_TICKS` (0.4375 s) and deliberately starts from the climbing pose.

## 2. Per pet

### windowy (40×52, hops; gear climb + parachute; grip 49; reach 8; mood `curious`)

- **States:** `shut` (resting) · `tilted` "Tilted open / Gekippt" (overlay `tilt`: the right sash drops to 0.86 so a dark slot opens at the top, slight lean; emitter `draught`; 25 s → shut) · `open` "Wide open / Offen" (overlay `wide`: both sashes edge-on, dark openings, draught sway; emitter `gust`; 20 s → tilted) · `fogged` "Fogged up / Beschlagen" (tint accent `#e8f1f3`, so the panes turn white; overlay `fog`: a smiley drawn in the fog in the upper left pane, a dimmed glint, an occasional shiver; emitter `fog-drops`; 12 s → shut).
- **Tricks:**

  | Trick | Cues | Does | Emitter | From | To | Mood |
  |---|---|---|---|---|---|---|
  | `crack-open` | click | swings the right sash open and shivers | `draught` | all states | tilted | playful |
  | `fog-draw` | click | breathes on itself, draws the smiley | `fog-drops` | shut, tilted | fogged | proud |
  | `swing-wide` | circle | sashes fly edge-on with flaps and a skip | `gust` | rungs shut, tilted, open | — | playful |
  | `close-up` | countercircle, shake | sashes slam with an overshoot, clatter | `snap` (3 stars burst off the frame) | same rungs | — | — |
  | `glaze-show` | whim, show | stands proud while one, two, three glazing lines appear in the pane; the glint sweeps | `twinkle` | shut, tilted, open | shut | proud |

- **Purr:** `glass-hum` (sashes breathe alternately, the glint slides across, it leans into the hand, one foot taps) + `twinkle`.
- **Emitters (5):** `draught` (3 ink streaks, drift), `gust` (5 ink streaks, drift), `fog-drops` (2 grey drops, fall), `snap` (3 paper stars, burst), `twinkle` (1 star, drift).
- **New bones:** `cup-left`, `cup-right` (on root), `doodle`, `pane-1..3` (on `sash-left`).
- **New parts:** the two cups (accent dome with an ink rim and handle), `doodle`, `pane-1..3`.
- **Canopy:** an arched window (dome with a cross of mullion and transom).
- **New clips (19):** `tilt`, `wide`, `fog`, `crack-open`, `fog-draw`, `swing-wide`, `close-up`, `glaze-show`, `glass-hum`, `hang` (legs paddle, the free sash flaps, the glint flashes), `tumble`, `glide`, `climb` (suction cups hand over hand, they pop and squish), `mantle`, `slide`, `push` (sill against the chip, right sash swung out like a door, braced back leg, tremble), `dizzy`, `shrug`, `scoot` (the hop at 1.5×, the same stride, so it is slide-free).
- **Contrast:** body `#7b827d` gives 3.54 / 4.88 in every state (the fog tint changes only the sashes). Both pass 3:1, and the 2 px ink outline adds to it.
- **Still bothers me:**
  - `tilted` is subtle at 1×: a 4 px slot plus streaks.
  - On the dark theme the `ink` openings turn cream, so the open window shows bright openings instead of dark ones. This is the same class of problem as battery's window, and the palette has no fourth colour to fix it.
  - The fog tint comes in only when the trick ends, because a tint belongs to a state.
  - While the `fog` overlay blends in, the doodle slides up from the sill for 8 ticks.

### solary (46×52, walks; gear climb + ladder + parachute; grip 49; reach 7; mood `happy`)

- **States:** `generating` (resting) · `shaded` "Shaded / Verschattet" (tint `#23467f/#8fb1a0/#9a9a90`; overlay `shade`: the panel slumps and the arms droop; 30 s → generating) · `peak` "Peak power / Spitzenleistung" (tint `#3b78d8/#e8f7ee/#fff5b0`; overlay `peak`: lit cell rows pulse top to bottom, panel and sparkle swell, arms up; emitter `shine`; 40 s → hot, as rule R06 says) · `hot` "Too hot / Modul heiß" (tint `#5a5fb0/#e8eef5/#e08a50`, so the frame turns orange; overlay `hot`: droop, limp arms, a shiver, a dimmed glint; emitter `sweat`; 20 s → generating).
- **Tricks:**

  | Trick | Cues | Does | Emitter | From | To | Mood |
  |---|---|---|---|---|---|---|
  | `sun-track` | click, whim | tilts towards the sun, then gleams; the sparkle pops | `sparkle-burst` | all states | — | happy |
  | `cell-wave` | click | rows light top to bottom and back; the glint sweeps; little hops | `glimmer` | all states | — | playful |
  | `power-up` | circle | stands upright and tall, arms up, cells fill bottom to top | `sparkle-burst` | rungs shaded, generating, peak | — | proud |
  | `duck` | countercircle, shake | folds forward, covers up, the glint goes out | — | same rungs | — | — |
  | `feed-battery` | show | flash and throw of packets, then a wave | `packets` (accent dots, burst) | all states | — | proud |

- **Purr:** `inverter-hum` (panel sways, the glint slides diagonally, rows pulse in alternation) + `glimmer`.
- **Emitters (5):** `sparkle-burst`, `glimmer`, `shine`, `sweat`, `packets`.
- **New bones:** `cell-row-1..3` (on panel), `hook-left`, `hook-right` (on root).
- **New parts:** three tiny-geometry cell rows, two accent bracket hooks with an ink cord.
- **Canopy:** a solar wing (a flat trapezoid with a cell grid), so solary hang-glides.
- **New clips (20):** `shade`, `peak`, `hot`, `sun-track`, `cell-wave`, `power-up`, `duck`, `feed-battery`, `inverter-hum`, `hang` (legs and arms flail, the glint flashes, cells flicker), `tumble`, `glide` (holding the wing), `climb` (bracket hooks bite the wall, the arm reaches up hand over hand; the same clip on a ladder), `mantle`, `slide`, `carry` (stroll with arms up and a forward lean, slide-free), `push` (leans its frame in, braced arm, the glint flashes like a mirror), `dizzy` (sparkle spins), `shrug`, `scoot` (stroll at 1.5×, 0.2917 s).
- **Contrast:**

  | State | Body (glass), light / dark | Frame (detail), light / dark |
  |---|---|---|
  | generating | 5.60 / 3.09 | 1.58 / 10.93 |
  | shaded | 8.38 / **2.06** | 2.55 / 6.77 |
  | peak | 3.88 / 4.45 | 1.00 / 17.34 |
  | hot | 5.11 / 3.39 | 2.38 / 7.25 |

  The glass sits inside the frame, so the outline against the page is frame plus 2 px ink. On dark, shaded glass inside its frame is fine. On light, peak's pale frame relies on the ink outline.
- **Still bothers me:**
  - The arms are 4.5 px stubs drawn behind the frame, so raised arms (glide, carry, climb) mostly hide.
  - The push leans 8 px out of the panel's footprint to reach a fixture at the box edge.
  - At peak the lit cells cover most of the blue, so solary looks pale.
  - On dark the grid lines and glints read; the eyes and mouth stay on top of the lit cells.

### venty (44×41, floats, hover 3; gear none, since it refuses all four; grip 35; reach 4.5; mood `proud`)

- **States:** `nominal` (resting) · `low` "Basic airflow / Grundlüftung" (tint `#5f8428/#d8d4c2/#b5d4c8`; overlay `low`: the fan turns slowly, one turn per 6.4 s, short jets, sinks, ducts droop; reached by `air-down`, no time limit) · `boost` "Boost / Intensivlüftung" (tint `#7fc03a/#ffffff/#e6f8ee`; overlay `boost`: the fan blurs, jets ×1.7, exhaust streaks, a buzz; emitter `airflow`; 15 s → nominal) · `bypass` "Summer bypass / Sommer-Bypass" (tint `#4f9c6a/#f7f3e3/#9ed8e6`; overlay `bypass`: the mint flap slides on the casing, the fan turns back; emitter `cool-air`; 30 s → nominal).
- **Tricks:**

  | Trick | Cues | Does | Emitter | From | To | Mood |
  |---|---|---|---|---|---|---|
  | `spin-up` | click | the existing clip | `airflow` | all states | boost | playful |
  | `heat-recovery` | click | stale air out of the back duct, fresh air into the front duct, proud swell | `stale` puffs | all states | — | proud |
  | `air-up` | circle | longer jets, fan speeds up | `airflow` | rungs low, nominal, boost | — | playful |
  | `air-down` | countercircle, shake | jets shrink, ducts droop, a sigh | `sigh` | same rungs | — | — |
  | `night-cool` | whim, show | flap slides open, cold air leaves the front, a shiver | `cool-air` | low, nominal, boost | bypass | curious |

- **Purr:** `quiet-fan` (jets ripple back to front, ducts nod, the streaks trail behind) + `sigh`.
- **Emitters (4):** `airflow` (vertical ink streaks rise from the front duct), `stale` (detail puffs from the back duct), `cool-air` (mint pills drift ahead), `sigh` (one paper puff).
- **New bone and part:** `flap` (tiny geometry).
- **New clips (13):** `low`, `boost`, `bypass`, `heat-recovery`, `air-up`, `air-down`, `night-cool`, `quiet-fan`, `hang` (jets sputter sideways, the fan windmills backwards, exhaust trails), `tumble`, `push` (leans in, the front duct turns towards the chip and blows a puff, jets angled back; the strength is always the same, as CONTENT's no-leak rule asks), `dizzy`, `shrug` (ducts as shoulders), `scoot`. Its jets stay above the perch edge (3 px hover) in every new clip; the closest are the `boost` overlay at 0.04 px and `scoot` at 0.18 px.
- **Contrast:** nominal 3.19 / 5.42, low 3.92 / 4.41, bypass 3.00 / 5.76, boost **1.99** / 8.70 (boost relies on the ink outline on light).
- **Still bothers me:**
  - Under the override rule `low`, `boost` and `bypass` set the fan's speed outright (one turn per 6.4 s, per 0.8 s, and one turn backwards per 4.8 s), so the three states differ clearly in motion.
  - `air-down` ends its fan turn at −120° (the fan is three-fold, so it is seamless). The rest check flags it, but it is harmless.
  - CONTENT's `filter-tap` and its filter prop were not built (see §3).

### chilly (52×44, skates; gear climb + parachute; grip 38; reach 11.5; mood `grumpy`, i.e. aloof at rest intensity)

- **States:** `cooling` (resting) · `standby` "Standby / Bereitschaft" (tint `#6a97b0/#d8e2e6/#cfe0e4`; overlay `standby`: frost 0.2, small crystals, the emblem held still because its rotation is keyed at 0, drooping arms) · `frosted` "Iced up / Vereist" (tint `#5ab6e0/#ffffff/#e4f6fb`; overlay `frosted`: frost 1.6, crystals 1.5, shivers; emitter `snowfall`; 30 s → cooling) · `overloaded` "Overloaded / Überlastet" (tint `#1f7fb0/#f7f3e3/#ffb48a`, so the crystals turn peach; overlay `strain`: swells, the emblem spins fast, quick breath puffs, pumping arms; emitter `sweat`; 30 s → cooling).
- **Tricks:**

  | Trick | Cues | Clip | Emitter | From | To | Mood |
  |---|---|---|---|---|---|---|
  | `ice-crown` | click | `ice-crown` | `ice-chips` | standby, cooling, overloaded | frosted | proud |
  | `flurry` | click, whim, shake | existing `flurry` (a shaken chiller is a snow globe) | `flakes` (8, burst) | all states | cooling | playful |
  | `cool-down` | circle | `cool-down` | `ice-chips` | rungs standby, cooling, frosted | — | — |
  | `thaw` | countercircle | `defrost` (`thaw` is already the cuddle loop) | `drips` | same rungs | — | — |
  | `chill-out` | show | `chill-out` (breath to the partner, cold pills) | `chill` | all states | overloaded | proud |

  `chill-out` → overloaded makes `overloaded` reachable without chemistry, and it is physically true: rule R55 has the chiller working hard for a hot neighbour.
- **Purr:** `compressor-sigh` (crystals twinkle, the body breathes, breath puffs) + `glitter`.
- **Emitters (7):** `snowfall`, `flakes`, `ice-chips`, `drips`, `sweat`, `chill`, `glitter`. Flakes are filled paper six-point stars with an ink edge, so they read on both themes.
- **New bones:** `axe-left`, `axe-right` (on root).
- **New parts:** two ink handles and two accent picks.
- **Canopy:** a six-armed snowflake with branchlets.
- **New clips (17):** `standby`, `frosted`, `strain`, `ice-crown`, `cool-down`, `defrost`, `chill-out`, `compressor-sigh`, `hang` (skates swing, arms flail, the emblem spins backwards), `glide-down` (under the snowflake), `climb` (picks bite 0.5 px into the wall, swing back, crampon kicks), `mantle`, `slide`, `push` (a skate kick with a small glide forward; the kicking leg lifts so the blade stays above the ground), `dizzy`, `shrug`, `scoot`. `tumble` keeps the existing clip.
- **Contrast:** cooling 3.05 / 5.67, standby 2.83 / 6.10, frosted **2.05** / 8.42, overloaded 4.00 / 4.32. Standby and frosted on light rely on the ink outline.
- **Still bothers me:**
  - On the light theme the snowflake canopy is a cream shape on a cream page, so it reads by its outline (as the plain canopy does).
  - The ice axes are only 7 px at 1×.
  - CONTENT's `skate-spin` and `ice-block` were not built.

### servy (46×56, rolls; gear grapple; grip 53; reach 5; mood `curious`)

- **States:** `idle` (resting) · `busy` "Busy / Rechnet" (overlay `crunch`: fast LED wave, buzz, typing, cable sway; emitter `packets`; 20 s → idle) · `hot` "Overheated / Überhitzt" (tint `#5a5a64/#ffb000/#ff344f`, so the LEDs turn amber; overlay `hot`: the red LED swells, two heat waves rise endlessly, it fans itself; emitter `sweat`; 30 s → throttled, thermal throttling) · `throttled` "Throttled / Gedrosselt" (tint `#44666a/#fccf05/#ff344f`; overlay `throttled`: sinks, LEDs dimmed, arms and cable droop; 20 s → idle).
- **Tricks:**

  | Trick | Cues | Does | Emitter | From | To | Mood |
  |---|---|---|---|---|---|---|
  | `led-wave` | click, whim | three passes of LED pops from led-1 to led-4, a cheer at the end | `packets` | idle, busy, throttled | busy | playful |
  | `overheat` | click | the existing clip | `heat` (rising red waves) | idle, busy, hot | hot | — |
  | `load-up` | circle | LEDs flicker, it types, a heat wisp | `packets` | rungs idle, busy, hot | — | curious |
  | `throttle-down` | countercircle | LEDs dim one by one, it sinks | — | busy, hot | throttled | — |
  | `reboot` | shake | yawn, all LEDs off, back one by one | `spark` | all states | idle | content |

- **Purr:** `fan-hum` (LEDs breathe in staggered pulses, a 4 Hz buzz, the cable sways) + `ping` (one packet travels right).
- **Emitters (5):** `packets`, `sweat`, `heat`, `ping`, `spark`.
- **New bones and parts:** none.
- **New clips (17):** `crunch`, `hot`, `throttled`, `led-wave`, `load-up`, `throttle-down`, `reboot`, `fan-hum`, `aim` (the right arm holds the drawn gun at its side, a targeting LED sequence, leans back), `reel` (casters idle-spin, arms up the rope, cable swings), `hang` ("connection lost": green LEDs off, the red one pulses, casters spin free, arms flail), `tumble`, `push` (rolls 4 px into the chip with matching caster turns, LEDs flash, arm on the chip), `dizzy` (the cable twirls), `shrug`, `scoot` (roll at 1.5×, 0.625 s, still one caster revolution per cycle).
- **Contrast:** idle, busy and throttled 5.63 / 3.07; hot 6.13 / **2.82** (hot relies on the ink outline on dark).
- **Dark theme:** slots, shelf lines, LED rims and casters turn cream and stay readable.
- **Still bothers me:**
  - The heat waves of `hot` rise 14 px above the box. This is ephemeral art, not the body, but it is tall.
  - In `reel` the raised arms hide behind the cabinet.
  - The cable-plug idea of CONTENT is told only by the drawn gun and rope; servy has no own plug animation.

## 3. Decisions and deviations

1. **At most five tricks per pet** (the brief's 3–5). Shake outcomes are folded into existing tricks: windowy `close-up`, solary `duck`, venty `air-down`, chilly `flurry`, servy `reboot`. Not built from CONTENT §S: venty `filter-tap` (and its filter prop), chilly `skate-spin`, the `ice-block` "low step", servy `count`/`ping` as tricks and its ramp prop, and solary's separate `sparkle-shower`. No §7 rule names any of these; the duet tricks rules may target are `feed-battery`, `chill-out`, `glaze-show` and `night-cool`.
2. **State and trick ids follow CONTENT and the §7 rules exactly.** The first state is the resting one, and circle tricks name their rungs in `from`, which is necessary where the resting state sits mid-ladder (solary, venty, chilly). Every state is reachable by tricks or `lasts`/`then`, and at least one click trick is on offer in every state.
3. **Clip ids differ where an id was taken:** chilly's trick `thaw` uses clip `defrost`, and chilly's `glide` activity uses `glide-down` (`glide` is its skating walk).
4. **`reach` = the leant encounter extent, rounded up to 0.5** (windowy 8, solary 7, venty 4.5, chilly 11.5, servy 5). A2's 0.25 × width was 10–13.
5. **`grip`** is the top of the frame or cabinet (windowy 49, solary 49, servy 53), the casing top between venty's ducts (35), or chilly's top edge under the crystals (38).
6. **Particles that must read on both themes** use `paper` or `accent` fill with a thin `ink` edge; air streaks are `ink` lines. CONTENT's "paper streaks" would vanish on the light page.
7. **The coordinator's engine rules**, which arrived during the work, are followed:
   - **(1) A state clip overrides the idle loop per keyed (bone, channel).** Every state clip keys only what the state changes, so unkeyed channels keep breathing. Three clips had relied on adding to the idle loop and were changed:
     - venty `low`: the fan now keys a slow forward turn (one per 6.4 s) instead of a counter-spin, which would now have spun it backwards.
     - venty `bypass`: 4.8 s, the fan turns slowly backwards as the bypass cue, the flap slides.
     - chilly `standby`: the emblem's rotation is keyed constant 0, so it is held still.

     Every other state clip keys moving values of its own, for example solary's panel 0.95↔0.97 and servy's body y 1.2↔1.5, so nothing freezes where it should live.
   - **(2) `to` applies when the trick ends; the emitter starts with the trick.** Bursts are timed to that: windowy's `close-up` now slams at about 0.15 s, under its star burst (life 0.6 s), and solary's `feed-battery` throws its packets at the start. The fog tint of `fog-draw` arrives at the trick's end.
   - **(3) The first state is the resting one, with CONTENT's default ids** (`shut`, `generating`, `nominal`, `cooling`, `idle`).
   - The activity clips, new and old, still layer on top of the idle loop. `mantle` starts from the climbing pose.
8. **Preview tool fix:** `🖌️depiction` began reading `frame.spirits` at 01:57, and the sheets then failed with `NaN` mouths. In `TK/render_species_preview.entry.ts` I added `spirits: look.mood` beside `mood` in the one `paint` call. It is a one-line, behaviour-preserving fix that also serves the sibling art packages.

## 4. Verification (real output)

```
$ bun TK/validate_species.ts AP/{🪟️windowy,🔆️solary,🌬️venty,❄️chilly,🖥️servy}/🔣️.json      (from the repository root)
ok   🎓️teaching/🏛️architecture/🐾️pets/🪟️windowy/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🔆️solary/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🌬️venty/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/❄️chilly/🔣️.json
ok   🎓️teaching/🏛️architecture/🐾️pets/🖥️servy/🔣️.json
validate exit=0                                   (also: all twenty AP species "ok")

$ bun ./📜️script.ts test                           (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript)
 Test Files  5 passed (5)
      Tests  138 passed (138)                     exit 0
$ bun ./📜️script.ts test pet-cast
 Test Files  1 passed (1)
      Tests  80 passed (80)
```

The measurements are in `OUT/final/measure.txt`, covering contrast, reach and the extents of every new activity clip:
- Nothing new goes below the ground line on a perch.
- Venty's jets stay above its perch edge.
- Climbing props touch the wall at the box edge (chilly's pick bites 0.5 px).
- The only one-shot clips off rest are the three mantles (on purpose), chilly's existing `flurry` and venty's `air-down` (both seamless by the six-fold and three-fold symmetry of emblem and fan).

## 5. Paths

- Species: `AP/🪟️windowy/🔣️.json`, `AP/🔆️solary/🔣️.json`, `AP/🌬️venty/🔣️.json`, `AP/❄️chilly/🔣️.json`, `AP/🖥️servy/🔣️.json`.
- Tool: `TK/h4_measure.ts` (new); `TK/render_species_preview.entry.ts` (one line, see §3 point 8).
- **Final sheets:**
  - `OUT/final/five-states-x1.png`: all five with their states at 1×, light and dark.
  - `OUT/final/five-states-x2-{1,2}.png`: the same at 2×.
  - `OUT/final/<pet>-{1..4}.png`: states, tricks, purr and gear at 2×, light and dark.
  - `OUT/final/validate.txt`, `OUT/final/site-test.log`, `OUT/final/measure.txt`.
- Working sheets: `OUT/*.png` (3×), including `OUT/windowy-before-*.png` (the A2 placeholder) and `OUT/sweep-{windowy,solary,chilly}.png` (first-round clips checked for peeking props).
