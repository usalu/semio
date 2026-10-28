# Scene report: `📉️extrema` · `Beat1_LocalGlobalExtrema`

File: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py` (class, `TITLE`, `construct()` unchanged; `present_extrema` and its helpers added). Storyboard: `storyboard-extrema.md` (superseded by this report where they differ).

## What the clip shows

One seminar room (8 m × 6 m × 3 m, Zurich-like climate, window 60 %, no overhang, 16 cm insulation) whose glazed façade bearing θ (E 90° … N 360° … E 450°) is the design variable and whose annual electricity E(θ) (heating + cooling + lighting, kWh/a) is the objective. One `θ` tracker drives the plan with sun arcs, the winter/summer section, the stacked bar and the plot marker together; every number on screen comes from `orientation_study()` / `evaluate()` through the toolkit formatters.

Screen: left column plan (top) and section (bottom), middle column E(θ) plot with compass tick row and live marker, right column stacked heating/cooling/lighting bar with live kWh/a. Intro beats use a large plan with the room facts and the unrolled bearing strip; the summary uses the plan with the courtyard sector and a three-row card.

## Beats (27 clauses)

| Key | Visual state and animation |
| --- | --- |
| `formulate` | large plan with sun arcs and dimensions, room facts; the room turns E → S → W → N → S |
| `formulate_2` | unrolled bearing strip "Design variable θ" with a pointer that follows the room |
| `response` | plan to the left column, winter section, stacked bar, objective row `E = Heating + Cooling + Lighting` in the bar colours |
| `response_2` | "Goal: minimize E ↓ lower is better", model disclaimer, fixed design row |
| `domain` | axes with compass tick row, y ticks, titles; room turns to east (90°), bar and section show the east values |
| `sweep` | marker 90° → 180°, curve drawn behind it, winter section: low sun, deep patch, heating segment shrinks |
| `sweep_2` | section switches to the summer sun: steep rays, cooling stays modest |
| `sweep_3` | 180° → 270°: low summer sun floods the room, cooling segment dominates, curve peaks |
| `sweep_4` | 270° → 360°: sun behind the façade, heating segment large |
| `sweep_5` | 360° → 450°: plan equals the 90° state, curve closes |
| `local` | white "start" ring at 315°; greedy ±5° descent from the model (`study["descent"]`) to the north minimum |
| `local_2` | circle at 360°; chip "local minimum 355° +… kWh/a, 365° +… kWh/a" and dots at the neighbours |
| `global` | marker to 180°, green square, legend, dashed level line, ghost bar of the north design beside the live south bar |
| `global_2` | bracket and chip "+201 kWh/a, +29.5 %, +50 CHF/a" between the two valleys |
| `argument` | chips "design θ* = 180° (south)" → "value E(θ*) = 681 kWh/a" |
| `argument_2` | both chips indicated in turn (° versus kWh/a) |
| `compare` | scan line over the whole range with the room turning; cards "North 360° …" and "South 180° … best" |
| `compare_2` | the descent from 315° drawn again, ending in the north valley |
| `maximize` | ghost east room, east reference line, ghost east bar, formula `S(θ) = E(90°) − E(θ)` |
| `negate` | curve reflects into S(θ) on the savings axis, "min E ⇔ max S" |
| `negate_2` | marker on the peak, bracket between the two bars with "450 kWh/a, 112 CHF/a" |
| `constraint` | street/courtyard on the plan, infeasible south half ring and plot band 90°–270°, room turns to north |
| `constraint_2` | south optimum crossed out; the room tries to turn back, stops at 270° and returns |
| `feasible` | green square at 360°, legend "feasible optimum / excluded" |
| `feasible_2` | bracket and "Courtyard price +201 kWh/a, +50 CHF/a" |
| `scope` | plan with courtyard sector and the card Direction / Domain / Scope (window, overhang, insulation, climate, model) |
| `scope_2` | scope row indicated; smaller and larger window pictograms with "θ* = ?" |

The storyboard clauses `domain_2` and `maximize_2` were folded into `domain` and `maximize` to keep the running time near four minutes.

## Numbers and the narration

`NARRATION` is a literal list of `[key, en, de]`. It contains only the compass constants and bearings that the model fixes (90°, 180°, 270°, 315°, 360°, 450°, 270°–450°, ±90°); every kWh, CHF and percent value is drawn on screen from the model, so model retuning cannot make the captions wrong. `T/check-extrema-numbers.py` compares each clause with `orientation_study()` (bearings per key, no other numbers, minima, constraint, descent, heating/cooling orderings per sweep clause, west peak, saving peak) and passes (66 checks). `T/timing-extrema.py` prints the estimated spoken time (en about 2:44, de about 2:54).

## Verification

- `T/capture.py 📉️extrema Beat1_LocalGlobalExtrema en|de`: one PNG per clause (27 each), layout reports are `none` for every clause in both languages. Frames are in `T/🗑️generated/frames/` (copies of the first iterations in `T/🗑️generated/extrema/run1`, `run2`).
- `T/render-smoke-extrema.py <en|de>` plays the whole scene for real (320 × 180, 3 fps, no skipped animations) to prove every fade, transform and travel runs; German run: 253.7 s of scene time (4:14), English similar; both finish without errors.
- Clip contract tests of `🧪️tests/🐍️.py` for narration and captions (`test_each_scene_has_bilingual_narration_and_native_discovery`, `test_all_clauses_fit_both_languages_and_audiences`, curriculum and entry points): 4 tests, OK.

## Frames reviewed and fixes made on the way

Reviewed every frame of both languages, repeatedly. Fixed: tiny section (probe box dominated by labels, now `labels=False` and fitted to its corner), clipped bar labels (bar column moved left, label size 12), overlapping readout/axis title (headroom in the y range), tags crossing drop lines (replaced by a plot legend and a header row in the neighbour chip), price chips against the live readout (chip right of the bracket), top-row texts against the y title, ghost bar against the last tick label (plot 4.2 wide), zero-line tag against the curve (own strip below the zero line), scope card against the compass.

## Findings for the toolkit and the coordinator

1. **`FadeIn` on a live widget crashes in real playback** (`ValueError: zip() argument 2 is longer than argument 1` in `Animation.interpolate_mobject`) when the widget's child count depends on the design, for example `plan(arcs=True)` while a tracker moves: `Animation.get_all_mobjects_to_update` also runs the copied updater of the animation's starting copy, which rebuilds that copy. The capture harness skips animations and hides it. The scene works around it with `_Appear` (a `FadeIn` whose `get_all_mobjects_to_update` returns `[]`) and `FadeOut` only after `clear_updaters()`. The toolkit should offer the same as `LiveFade`/`fade_in`.
2. Real playback cost: the smoke render takes about 0.27 s per frame at 320 × 180; expect about two hours for 1080p30 of this scene (live section and plan redraw every frame).
3. `room_section` fits only small in the 4 × 1.7 unit corner; with the toolkit's larger default extent it would need its own band.
4. The model's baseline shift (geometric shading) does not touch this clip: the orientation study has d = 0 and its numbers are unchanged.

## Open issues

- Running time: narration alone is below three minutes, but `step` adds the animation time on top of the full reading hold (`hold_for(used=0)`), so the played scene is 4:14 (smoke render, German). Shortening more would mean removing `compare_2` or `scope_2`.
- The scope pictograms (`thumbnail`, ghost state) are plain grey outlines; a coloured ramp tile would need a shared ramp range.
- No audio/VO timing exists yet for this clip.

## Round 2

Audit: `T/audit-r2-extrema.md` (28 defects). Scene: `C/📉️extrema/🐍️.py` only. Checks: `T/check-extrema-numbers.py` (narration vs model, now incl. peak, local_2, sweep_4, global_2 claims), `T/check-extrema-r2-tables.py` (every table, formula and chip recomputed from the model; every printed sum closes; all `ok`), `T/layout-strict.md` findings from `capture.py` (final en and de reports: see Verification).

### Changes

- **Layout rebuilt around the room.** Left column: two live toolkit `room_section` drawings (summer on top, winter below, 0.38 per metre) with a live note left of each façade (season, hours of direct sun from `sun_samples`, sun strength from `sun_<season>`, the same number that sets the ray opacity). Centre: live formula zone above a 3.9 × 2.3 plot with a zero baseline (D26). Right: compass plan, the stacked bar with CHF/a below it. The old tiny hand-drawn section and the bearing × hour strips (D4) are gone; the sections now carry the physics, daylight zone and lamps included (toolkit r2).
- **The plot is the decomposition.** The curve is the top edge of a stacked area, heating (red) + cooling (teal) + lighting (yellow), so every bearing shows its three parts at the marker, in the colours of the bar and the formula.
- **Live substituted formula (toolkit `electricity_formula` / `derivation`).** `Heating + Cooling + Lighting = Total` with the live numbers is always on; the focus part is derived below it: heating `152 kWh/a ÷ COP 3.5 = 43`, cooling `1,510 ÷ EER 3.0 = 503`, lighting `0.43 kW × 1,500 h/a × 21 % lamps on = 135`. All three are shown together in `response`, one at a time while sweeping. Parts are largest-remainder rounded by the toolkit, my tables use `self.parts`, so every printed sum closes (`check-extrema-r2-tables.py`).
- **D1:** sweep_3 travels to the model's maximum (`orientation_study()["local_maxima"][0]` = 282°, 1,146 kWh/a) with a "maximum 282°" tag and drop line; caption and checker follow the model.
- **Why the minima are minima (D3, D5).** `local` and `local_2` show the neighbours ±5° with ΔE split into heating, cooling and lighting (315°: ΔE +25.1 = −7.1 + 33.3 − 1.1 at 310°, −26.2 at 320°; 360°: +2.6 = −0.2 + 2.8 + 0.0 on both sides: cooling rises faster than heating falls). `global_2` shows north to south: heating −268, cooling +96, lighting −29 = −201 kWh/a (the free winter sun wins), `maximize` the same table for the saving against east (138 + 300 + 12 = 450 kWh/a ≈ 112.5 CHF/a, D19: printed arithmetic closes, D27: tariff stated in `negate_2`).
- **Fixed WWR, overhang, insulation (D2).** New beat `fixed`: the big labelled section with the chips WWR 60 %, d 0.0 m, t 16 cm drawn on the window, overhang and wall layer, plus a card saying what each means (14.4 m² of glass in the 24 m² façade, no shading, U-value 0.19). The chips stay top right afterwards.
- **Scope with a real moving direction (D20).** The tiles compare the study design (θ* 360°, 882 kWh/a), a 0.6 m overhang (θ* 354°, 743 kWh/a) and a 30 % window (θ* 360°, 832 kWh/a), all computed on the 1° grid inside the courtyard domain; the caption now says "sometimes its direction", which is true for the overhang.
- **Other defects.** D6 (section removed in scope), D7/D27 (negate tags below the zero line, no text on bars), D8 (lens removed), D9 (ghost bars moved right), D10 (no text over the person), D11/D12/D13/D14 (legend row in the formula zone, start tag above its ring), D15/D23 (German terms in one set, "Ost" instead of "O" in bar tags, `Ost-Referenz`), D16 (CHF/a under the bar), D17 (all labels ≥ 14 px), D18 (units stay in the energy-clip bracket convention), D21 (sun altitude spelled out: "Summer sun, noon altitude 66°"), D22 (caption "…is now the best one"), D24 (captions shortened), D25 (bar scale rule only, bar labels replaced by the formula in the same colours), D28 (the coordinator's strict check now exists).
- Narration: new clauses `fixed`, `global_3` (old `global_2` price line), changed `sweep_3`, `sweep_4`, `local_2`, `global_2`, `feasible`, `scope`, `scope_2`; 29 clauses, German and English complete, all captions fit two lines.

### Final beat list

`formulate` (plan with sun arcs turning, room facts, façade elevation) · `formulate_2` (θ strip) · `fixed` (big section with chips + card) · `response` (three derivations with icons, formula, bar, CHF/a) · `response_2` (goal, tariff) · `domain` (plot with zero baseline appears, marker at east) · `sweep` (winter section framed, heating derivation, area builds to south) · `sweep_2` (summer section framed, cooling derivation) · `sweep_3` (to the 282° maximum) · `sweep_4` (north, lighting derivation) · `sweep_5` (curve closes) · `local` (start ring, neighbours table at 315°) · `local_2` (descent to 360°, neighbours table with decomposition) · `global` (south square, ghost north bar, legend) · `global_2` (north to south decomposition table) · `global_3` (gap bracket, price chip) · `argument`, `argument_2` (θ* vs E(θ*)) · `compare`, `compare_2` (scan, route) · `maximize` (east reference, saving table) · `negate` (S curve, zero and negative tags) · `negate_2` (S × tariff) · `constraint` (courtyard on the plan, infeasible band) · `constraint_2` (south excluded) · `feasible` (green square, legend) · `feasible_2` (courtyard price) · `scope` (card, three designs with tiles) · `scope_2` (answer).

### Verification

- `T/capture.py` en and de: every PNG of both languages viewed (29 frames each). Final strict reports: **en 0 findings in 29 frames, de 0 findings in 29 frames** (`T/🗑️generated/frames/Beat1_LocalGlobalExtrema/…-layout.md`); the classic checker's remaining lines are bounding-box noise (a ring-shaped sector polygon under the plan's centre label).
- Tests: `C/🧪️tests/🐍️.py` with `SEMIO_TEACHING_OUTPUT_ROOT=T/🗑️generated/output`: 63 tests OK (narration, captions fit in two lines in both languages and audiences, curriculum, entry points).
- Real playback: `T/render-smoke-extrema.py en` plays the whole scene (no skipped animations, 320 × 180) without error: 278 s render time, 293 s scene time (4:53; was 4:14: the three added beats and the longer table beats).
- Numbers: `T/check-extrema-numbers.py` (narration vs model) and `T/check-extrema-r2-tables.py` (formula parts at 90/180/225/282/315/360/450°, derivations demand ÷ COP/EER and 648 kWh/a × lamps-on share, neighbour tables, north-to-south and east-to-south tables, saving 449.8 kWh/a × 0.25 = 112.5 CHF/a, courtyard price 201 kWh/a = 50 CHF/a = 29.5 %) all `ok`.

### Remaining risks

- The section drawing is the toolkit's: the "WWR" chip leader and the short light-ray burst at the glass in `fixed` are its own geometry; at 360° the summer rays are drawn although "direct sun 0 h" (the model's sun strength 44 % is the diffuse/low sun at the façade edge, the note says both).
- The overhang curve in `scope` is rippled: the model counts hourly sun samples, so shading switches in steps near north. It is the model's behaviour, shown as is.
- Rendering cost: five live widgets redraw per frame (two sections, formula, derivation, bar), expect a slow 1080p render (`smoke` time in the log).
- `WWR`/`t`/`d` chip words in the section stay English abbreviations in German (toolkit).

## Round 3

Verifier: `T/verify-r2-extrema.md` (2 blockers, 4 major, 13 minor). Scene file only: `C/📉️extrema/🐍️.py`. Correction first: the Round 2 statement "0 strict findings" overstated the result. The strict check tests boxes, not limit lines or edges (verifier m12); the verifier's M1, m4 and m5 were real. Round 3 results below were checked by eye on every PNG as well.

### What changed

- **B1 (rays at 0 h).** Fixed in the toolkit's `room_section` (direct sun and disc only with `sun_samples` hours > 0, otherwise cream sky light). Re-captured; sweep_4, local, constraint…feasible_2 now show no direct rays at 0 h. Narration aligned: sweep_4 "Facing north, only diffuse light reaches the glass…" / de "…erreicht nur diffuses Licht das Glas…".
- **B2 (depth).** Heating and cooling are now derived the way the energy clips do it, from the model's own terms, live: heating demand `1,877 kWh/a (losses) − 0.63 (useful share η) × 2,748 kWh/a (gains) ≈ 152`, then `152 ÷ COP 3.5 = 43`; cooling demand `1,724 (solar + internal gains) − 1.00 × 214 (removable heat) = 1,510`, then `1,510 ÷ EER 3.0 = 503`. Lighting: `daylight factor 4.8 % → daylight autonomy 74.7 % → lamps on 25.3 %`, then `0.432 kW × 1,500 h/a × 25.3 % = 164`. The terms (`u_wall`, areas, `u_window`, `h_other`, degree hours, `solar_<season>`, internal gains, utilisation exponent) come from `evaluate()` and the study JSON, the model was not touched: `check-extrema-r2-tables.py` rebuilds both demands from the same terms for five bearings and requires equality to 1e-6. The model keeps these terms private; exposing them (`losses`, `gains`, `eta`) from `evaluate` would let the toolkit's `derivation('heating' | 'cooling')` print the same lines instead of the scene (open question for the coordinator). In `sweep` the heating balance and conversion are shown, in `sweep_2` the cooling ones, in `sweep_4` the daylight chain and the lamp product; `response` shows the three final conversions with icons.
- **M1.** The 270° limit line now ends 0.5 units below the top of the plot, clear of the readout chip.
- **M2.** Printed power 0.432 kW and one-decimal lamp shares: 0.432 × 1,500 × 25.3 % = 163.9 → 164 and × 20.8 % = 134.8 → 135 (check script), "=" only when the printed inputs reproduce the printed result, otherwise "≈" (the heating balance prints "≈" because η is shown with two decimals).
- **M3.** "strength N %" replaced by "N % of best" below the season and the hours of direct sun (N = `sun_<season>`: the façade's seasonal sun relative to the sunniest bearing).
- **M4 (354°/355° jump).** Investigated: at 354° (overhang 0.6 m) exactly one hourly sun sample (16:00, sun at 265°, 88.9° off the façade normal) still reaches the glass and is fully shaded, so `shading_summer = 1.0`; at 355° no sample remains and the model defines the shading as 0.0 while the irradiance term stays, so cooling jumps 222 → 410 kWh/a. That is an artefact of the model's shading definition, not physics, so the scene no longer builds on it: the scope tiles now compare window shares 30 %, 60 % and 80 % (θ* = 360° in all three: 832, 882, 966 kWh/a, smooth curves), the caption says "Another window share moves the optimum's value, here not its direction; other changes need a new search" (also resolves m9, "climate" is no longer named as shown). Model owner: a smooth treatment of the last sample (or a shading of 1 for "no direct sun") would remove the step.
- **Minors.** m1 no parentheses in labels ("E, kWh/a", "S, kWh/a", "E at θ*", "S at θ = E at 90° − E at θ", "W/m²K"); m2/m3 one German term set (Heizstrom, Kühlstrom, Beleuchtung; JAZ / Kälte-JAZ instead of COP/EER; "Ost" in tags; the compass ring and the section chip "WWR" are the toolkit's); m4 the start ring fades at the start of `local_2`, the tag sits above the ring; m5 German lamp line shortened ("Leistung × Stunden × Lampen an"); m6 orphan words removed by shorter captions (local, global, global_3, argument, scope_2, scope); m7 the segment values 43/503/135 are printed beside every bar segment (live bar and ghost bars); m10 dashed guides from both bar tops to the saving bracket; m12 the claim above is corrected; m13 `check-r2-extrema.py` now expects the maximum at 282°. m8 (chip on rays in `fixed`) was fixed by the toolkit (chips leave the ray corridor). m11 (drawn neighbours) stays optional: the neighbour tables carry the decomposition.
- Docstring emojis are unique again (66 docstrings).

### Verification

- Re-capture of both languages after the toolkit's B1 fix and all scene changes (29 frames each): `capture.py` strict reports **en 0 findings, de 0 findings**. Every PNG was read; the changed frames (response, sweep, sweep_2, sweep_3, sweep_4, local_2, global, negate_2, constraint, compare_2, scope, scope_2, and the German counterparts) were read again after the final capture. The strict check still tests boxes, not lines (see Round 2 correction), so the eye check stays the reference.
- Caption wrapping (`T/extrema-r3-captions.py`): the one-word orphans are gone (response, global de, scope_2 en and de, global_3 de now fit one line); two-word endings remain in a few captions ("curve flips.", "east façade?", "in kWh/a.").
- Numbers: `T/check-extrema-numbers.py` and `T/check-extrema-r2-tables.py` have 0 failures; the latter now rebuilds heating and cooling demand from the printed terms at 90°, 180°, 282°, 315°, 360° (equality to 1e-6) and the lamp product at 90°, 180°, 360° (0.432 kW × 1,500 h/a × 22.7 / 20.8 / 25.3 % = 147 / 135 / 164 kWh/a).
- Tests: `C/🧪️tests/🐍️.py` with `SEMIO_TEACHING_OUTPUT_ROOT=T/🗑️generated/output`: 64 tests OK (narration, two-line captions in both languages and audiences, curriculum, entry points, model).

### Remaining risks

- The zone above the plot is dense (sum, balance, conversion): three live formulas at 15 to 18 px; lines are 0.1 to 0.2 units apart.
- Window, overhang and insulation still change no drawn quantity in the main example (θ is the variable); only the scope tiles vary the window.
- The heating balance is honest about rounding ("≈"): η is printed with two decimals.

## Round 4

- **No physics in the scene any more.** The in-scene rebuild of the balance terms (`_eta`, `_flows`, constants from the study JSON) is deleted. The live lines read model keys only: heating `heating_losses − heating_eta × heating_gains → heating_demand`, then `÷ cop`; cooling `cooling_gains − cooling_eta × cooling_removable → cooling_demand`, then `÷ eer`; daylight chain `daylight → autonomy → lamps_on`; lamp product `lighting_power × lighting_hours × lamps_on`. The scene still decides only the printing (digits, "=" or "≈" from the rounded inputs, German words). Printed numbers are identical to Round 3 (152 / 43, 1,877 − 0.63 × 2,748 ≈ 152, 1,724 − 1.00 × 214 = 1,510, 0.432 kW × 1,500 h/a × 20.8 % = 135, 25.3 % → 164). `check-extrema-r2-tables.py` now checks the balances through the model keys (`heating_used`, `cooling_removed`, `cop`, `eer`, `lamps_on`) at five bearings and the lamp product at three: 0 failures; `check-extrema-numbers.py`: 0 failures.
- **Scope beat shows the overhang again, with the model's own argmin.** With the continuous shading the 0.6 m overhang curve has no step: over the courtyard domain 270°–450° its minimum is **329°, 870.0 kWh/a**, while the façade facing north (360°) would use 928.4 kWh/a; the curve has local minima at 329°, 360° and 391° (the last mirrors the first). The study design (no overhang) stays at 360°, 882.0 kWh/a, window 30 % at 360°, 832.0 kWh/a. The beat therefore teaches the stronger lesson and keeps it true: a different window share changes the optimum's value only, an overhang also moves its direction, from 360° to 329° (31° towards the west, 58 kWh/a less than at 360° for that design, 12 kWh/a less than the study design's 882). Caption: "Another window share moves the optimum's value; an overhang moves its direction as well." (de: "…ein Vordach verschiebt auch die Richtung."). The three tiles are the study design, window 30 % and overhang 0.6 m (80 % dropped), each with its thumbnail and its argmin from the 1° grid.
- **Verification.** Re-capture en and de (29 frames each): strict reports 0 findings in both; changed frames read again (scope, sweep, response in en and de). Tests: 70 tests OK.
- **Remaining risk.** The overhang curve is smooth but wavy (three local minima), as the model computes it from hourly sun samples; the scope plot shows it unsmoothed.

## Round 5

Source: `T/final-verify-extrema-sampling.md` (extrema: X1, M1 to M4, m-a to m-p). Scene `C/📉️extrema/🐍️.py`; toolkit `C/🎨️visuals/🏠️room/🐍️.py` edited with small, API-compatible changes (below).

- **X1 (blocker).** Every balance line is printed with the digits at which it closes, "=" only then: the scene searches amount digits 0 to 2 and η digits 2 to 6 per frame. Examples: heating `1,877 kWh/a − 0.6276 × 2,748 kWh/a = 152 kWh/a`; cooling at 180° `1,724 − 1.00 × 214 = 1,510`; cooling at 282° `2,548.93 − 0.99995 × 214.47 = 2,334.47` (the former "≈ 2,334 / 2,335" case, m-a). `check-extrema-r2-tables.py` now checks the printed inputs (five lines, all `ok`).
- **M1.** The derivation block is set off from the sum row by a rule and 0.4 units of air (plot shortened to 1.9 units); each term name stays in its own line, directly over its value. **M2.** The sky arrows of `room_section` (diffuse light) are now pale sky blue instead of the cream of the sun, so no yellow sun-like burst remains at the glazing at 0 h (toolkit edit, colour only: `SKY`). **M3.** Left of each section: `direct sun N h`, `sun on glass N %`, `of the best bearing`; German `Glasstrahlung N %`, `der besten Lage`. **M4.** `negate_2` prints S = 450 kWh/a in the formula and the chip (450 × 0.25 = 112.5 closes).
- **Minors.** m-b the maximum tag carries its value (`maximum 282°` / `1,146 kWh/a`); bar ticks stay out (segment values and totals label the bar, a tick scale would collide with the ghost-bar values). m-c the change column has its unit in the header. m-d the gap chips name both bearings and the base (`North 360° − South 180°: 882 − 681 kWh/a = +201 kWh/a · +29.5 % of South · +50 CHF/a`). m-e the limit line ends 0.62 units below the plot top. m-f the start tag sits beside its ring. m-g the legend glyph for "excluded" is the plotted square with the orange cross. m-h each section header says what its arrows are (`heat removed →`, `← heat lost`). m-i the scope tile ramp is titled "electricity at θ*", a note says the curve ripples come from hourly sun samples. m-j `Goal: minimize electricity E`, `Objective E: …`, and the section label reads "Sun altitude 19°" (toolkit). m-k the compare chips use "North/South", S stays the saving. m-l the S = 0 label moved right of the axis ticks. m-m German chips capitalised (Zulässig, Ausgeschlossen, Lokales/Globales Minimum, Kleiner ist besser, Beste), German section chips "Fenster", "Vordach", "Dämmung" (toolkit), minus signs are U+2212 (the font draws them short), "O" stays on the toolkit compass ring while ticks and tags spell "Ost". m-n captions rebroken (sweep, sweep_5, response_2, maximize, negate, argument), no caption ends in a one or two word line any more. Bill under the bar prints one decimal (170.3, 220.5, 286.5 CHF/a: 681, 882 and 1,146 kWh/a × 0.25).
- **Not done (named).** m-o density (one beat would have to be split, no time budget for a new beat), m-p drawn neighbour rooms (optional since round 3), bar-axis ticks (see m-b).
- **Verification.** Re-capture en and de (29 frames each): strict reports 0 findings in both; changed frames read (sweep, sweep_2, sweep_3, sweep_4, feasible_2, negate_2, scope, fixed, constraint). 70 tests OK after the toolkit edit; `check-extrema-numbers.py` and `check-extrema-r2-tables.py`: 0 failures.
- **Readability pass on the balance lines.** Each balance is now two short exact lines, as in the energy clips. Line 1 (share): `Useful gains = 62.8 % of 2,748 kWh/a ≈ 1,725 kWh/a` (η with one decimal, "≈" because it rounds a product). Line 2 (balance): `Losses 1,877 − useful gains 1,725 = Heating demand 152 kWh/a`. The whole numbers come from the same largest-remainder rounding (`parts`) as the bars, so the subtraction closes exactly as printed at every bearing. Cooling at 282°: `Removed heat = 100.0 % of 214 kWh/a ≈ 215 kWh/a`, then `Gains 2,549 − removed heat 215 = Cooling demand 2,334 kWh/a` (η = 99.995 % prints as 100.0 %). The conversion line (`÷ COP 3.5 = Heating 43`) is a third row; in the response frames the short form omits the leading demand name to stay above 11 px. `check-extrema-r2-tables.py` tests the printed lines at 90, 180, 225, 282, 315, 360, 450° for heating and cooling: subtraction exact, inputs within 1 of the model, share line within 1.5 of η × input: all ok. Re-capture en and de: strict 0 of 29 frames in both; sweep, sweep_3 (en), sweep_2, response (de) read. 70 tests OK; `check-extrema-numbers.py`: 0 failures.
- **Every "≈" line is verifiable by eye.** `balance_lines(found, part)` (scene module) searches the fewest decimals (kWh/a first, then percent) at which printed `share % × printed base` rounds to the printed result and `top − result = demand` closes; "=" is printed when the product is exact. The earlier "62.8 % of 2,748 ≈ 1,725" was also wrong by that test (62.8 % × 2,748 = 1,725.7), so heating prints `62.76 % of 2,748 ≈ 1,725` (whole kWh/a, still the 3-term line `1,877 − 1,725 = 152`). Cooling at 282°: no whole or one-decimal print closes (η = 99.995 %, base 214.47, true result 214.46), so it prints two decimals on both lines: `99.995 % of 214.47 ≈ 214.46 kWh/a`, `2,548.93 − 214.46 = Cooling demand 2,334.47 kWh/a`, and the conversion line uses 2,334.47 ÷ 3.0 = 778. The coordinator's "100.0 % of 214.5 ≈ 214.5" cannot close the next line (2,548.9 − 214.5 = 2,334.4, true 2,334.5). Over the 5° grid from 90° to 450° plus the seven named bearings, whole kWh/a suffices for most bearings, 1 to 3 decimals appear where the cooling η is near 100 %. The base value carries no unit (the result does), and German cooling uses "Abgeführt" for the removed heat so the line stays above 11 px. `check-extrema-r2-tables.py` asserts at all those bearings, for heating and cooling: rounded product equals printed result, "=" only when exact, printed top − result = demand, printed numbers within one unit of the model keys: 344 ok, 0 fail. Re-capture en + de: strict 0 of 29 each; sweep and sweep_3 read; 70 tests OK; docstring emojis unique (🎚️).
