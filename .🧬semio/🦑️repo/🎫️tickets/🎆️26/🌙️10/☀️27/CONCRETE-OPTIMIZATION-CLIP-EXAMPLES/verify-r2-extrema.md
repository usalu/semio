# Verification r2: `Beat1_LocalGlobalExtrema` (topic `📉️extrema`)

Scope. Scene `C/📉️extrema/🐍️.py` (file 21:53, 29 NARRATION keys). Frames `T/🗑️generated/frames/Beat1_LocalGlobalExtrema/` (21:54 to 22:02, newer than the scene, so no re-capture was needed): all 29 English and all 29 German PNGs read at full size, zoomed crops in `T/🗑️generated/verify-extrema/`. Strict layout reports `…-en-layout.md` and `…-de-layout.md` read. Model `C/🧮️model/🐍️.py` and toolkit `C/🎨️visuals/🏠️room/🐍️.py` read at the lines cited. Energy benchmark: `T/🗑️generated/energy-frames/Beat8_Hauptgleichung-chain.png`. Checks: `T/verify-check-extrema.py` (log `T/🗑️generated/verify-extrema/check-verify.log`), `T/verify-crops-extrema.py`. The older `T/check-r2-extrema.py` is stale (see m13).

## 1. Verdict

**FAIL.** 2 blockers, 4 major, 13 minor.

The clip is much better than round 1. The model maximum at 282° is marked, the neighbour tables show the heating, cooling and lighting parts, the sections carry live daylight and lamps, the formulas are substituted live, the scope beat has real tiles, and the captions fit two lines. Almost every printed number matches the model (about 110 checks, section 4). One printed arithmetic line does not close (M2).

It still fails the owner's bar in two places:

- The sun drawing contradicts its own labels. At 360° the summer panel draws yellow direct-sun rays through the room while the label says "direct sun 0 h", and the winter panel at 315° does the same (B1).
- The physics of the three parts is shown only as a last division step. The demand values 152 and 1,510 kWh/a and the 21 % lamp share are bare numbers. The energy clips derive exactly this kind of term (B2).

**Owner's question, 360° summer rays: misleading, yes.** The rays are the direct-sun colour, they fill the room, and the narration of the same frame says "the sun only grazes the glass". A second, unlabelled sun disc also sits outside the panel. The fix is in B1.

## 2. Defects

Tags: [abstract], [numbers], [layout], [sync], [depth], [de]. Severity: blocker, major, minor.

### 2.1 Blockers

| ID | tag | keys (en / de) | location | evidence | concrete fix |
| --- | --- | --- | --- | --- | --- |
| B1 | [sync] [abstract] | sweep_4, local (winter panel only), local_2, constraint, constraint_2, feasible, feasible_2 (both languages) | toolkit `room_section` in `C/🎨️visuals/🏠️room/🐍️.py` lines 591 to 594 (`sun_state`, `strength`, `blocked`, `front`), line 613 (`sun_at`), line 617 (`opacity`). Scene `_season_note` in `C/📉️extrema/🐍️.py` line 249 counts hours with `sun_samples`, but the rays come from the noon `sun_state` and `front`, so the two disagree. | At 360° the summer panel shows "direct sun 0 h, strength 44 %" beside yellow direct-sun rays that fill the room (crop `en-sweep_4-summer.png`). The same panel also shows a sun disc on the façade side and a second, unlabelled yellow disc outside the panel at about x 448, y 362 px (crop `en-sweep_4-disc.png`). The winter panel at 315° (local, "direct sun 0 h", crop `en-local-winter.png`) draws the same strong rays. Winter at 360° draws a fainter burst at the glass with 0 h. Narration in sweep_4 says "the sun only grazes the glass", so text, label and drawing contradict each other. | Draw direct-sun rays and the sun disc only when `len(_model.sun_samples(θ, season)) > 0`. Use the count, not `front`, for the decision. When the count is 0, draw only the cream sky light and write "no direct sun, diffuse light only" (or "direct sun 0 h") in the panel. Remove the back-side disc unless it is labelled "sun behind the façade". Keep ray opacity tied to the hours, or show the strength legend (see M3). Re-capture the 7 key pairs above. |
| B2 | [depth] | response, sweep, sweep_2, sweep_3, sweep_4, local, local_2 (derivations); fixed (chips); scope (the only beat that moves window or overhang) | scene `_variant` lines 233 to 238, `_causes` lines 335 to 341, `_phase_fixed` lines 310 to 328, `_phase_scope` lines 661 to 703 | Each part is derived only from its last step: heating "152 kWh/a ÷ 3.5 = 43", cooling "1,510 kWh/a ÷ 3.0 = 503", lighting "0.43 kW × 1,500 h/a × 21 % = 135". The demand values 152 and 1,510 kWh/a have no derivation on screen, and the 21 % lamp share is never linked to the daylight factor or the window. Window, overhang and insulation are text in `fixed` and change no printed number in the main example. The energy benchmark (Beat8 Hauptgleichung chain) derives the solar term as "G · A · F · g · F_sh = 400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W" with a bar against the maximum. | Add one demand line per part with the substituted terms from the model: heating `(transmission + ventilation losses − η · (solar + internal gains))`, cooling the same with the summer terms, and solar `G · A · g · (1 − f · F_sh)`. For lighting show `daylight factor → autonomy → lamps on = 1 − autonomy`. This needs the model to expose the loss and gain terms: `evaluate` returns `solar_winter`, `solar_summer` and the demand totals, but not the transmission, ventilation or internal-gain terms, so extend the model and its schema first. In `fixed`, show the same θ with overhang 0 m and 0.6 m side by side (shading_summer 0.00 → 0.89, cooling bar falls), so the chips change a drawn quantity. |

### 2.2 Major

| ID | tag | keys (en / de) | location | evidence | concrete fix |
| --- | --- | --- | --- | --- | --- |
| M1 | [layout] | constraint, constraint_2, feasible, feasible_2 (en and de) | readout chip from `_marker` (scene lines 387 to 398, placed at the top-right corner of the plot); dashed limit `limit` at line 623 | The chip "θ 360° · 882 kWh/a" spans about x 718 to 910 px and covers the 270° limit line at x about 734 px from y 298 to 330 px (crop `en-constraint-chip.png`). The rule "nothing overlaps a line" is broken. The strict checker reports none because it tests boxes, not the limit line. | Keep the readout chip right of the limit in constraint frames: pass `x_min = axes.c2p(lo, 0)[0] + 0.15` into `_marker` and clamp the chip, or start the dashed limit below the chip (`axes.c2p(lo, y_range[1] - .45)`). |
| M2 | [numbers] | sweep_4 (en and de), derivation at 360° | toolkit `derivation("lighting")` (`C/🎨️visuals/🐍️.py`, see `toolkit-r2.md` section 3) called from `_causes` / `_variant("lighting")` | Printed "0.43 kW × 1,500 h/a × 25 % ≈ 164 kWh/a". 0.43 × 1,500 × 0.25 = 161, not 164 (crop `en-sweep_4-derivation.png`). The model gives 25.3 % and 163.8 kWh/a. The displayed "≈" hides the error instead of fixing it. The power is exactly 9 W/m² × 48 m² = 0.432 kW, which the rounded 0.43 cannot reproduce. | Print the power as 0.432 kW and the share with one decimal. Then 0.432 × 1,500 × 25.3 % = 163.8 → 164 and 0.432 × 1,500 × 20.8 % = 134.8 → 135 (the 180° line, currently "21 %", must also change to 20.8 %). Keep "=" only when the printed inputs reproduce the printed result. |
| M3 | [abstract] [numbers] | every frame with the two solstice panels: response to feasible_2 (24 keys per language) | `_season_note` in scene lines 240 to 255 (line 252 prints "strength N %" from `sun_<season>`); toolkit opacity line 617 | "strength 57 %", "strength 100 %", "strength 44 %" carry no meaning on screen. The model value is `sun_<season> = irradiance(φ) / peak` (model line 173): the façade's seasonal irradiance relative to the sunniest bearing. A bare percentage without its reference breaks the owner's rule "no bare number without its object and unit". | Name what the percentage is and what it is relative to, for example "sun on glass 57 % of best bearing", and add the one-line definition under the section (or in the chip row). Alternatively remove the number and keep the ray opacity as the legend. |
| M4 | [numbers] [sync] | scope (en and de), the teal overhang curve and the tile "θ* = 354°" | `_phase_scope` lines 681 to 699, `_optimum` lines 654 to 659 | The 0.6 m overhang curve has steps. E(354°) = 742.6 and E(355°) = 930.9, a jump of 188.4 kWh/a. The same jump returns at 365° → 366°. There are also steps of 87 kWh/a at 309/310° and 410/411° (log `check-verify.log`). The optimum "354°" is the last point before a step, so the "direction" claim rests on a shading switch, not on a smooth optimum. The model report already notes that shading switches in steps (hourly sun samples). The curve reads as a glitch, and the narration says "sometimes its direction". | Keep the claim only with its cause shown: draw the curve as a step, put a marker at the switch (about 354.5°) with the label "shading switches here: sun enters the room (hour X)", and write the caption as "…or, at a shading switch, its direction". Or choose a case whose optimum moves continuously and say so. |

### 2.3 Minor

| ID | tag | keys (en / de) | location | evidence | concrete fix |
| --- | --- | --- | --- | --- | --- |
| m1 | [de] [layout] | all frames with an E or S axis title (domain to scope); argument, argument_2; negate; fixed | `_build_axes` / `y_label` in scene lines 377 to 385; formula `S(θ) = E(90°) − E(θ)` line 590; chip `E(θ*) = …` lines 539 to 540; `W/(m²·K)` line 320 | The scene font draws "(" and ")" as "[" and "]": "E [kWh/a]", "S [kWh/a]", "S[θ] = E[90°] − E[θ]", "E[θ*]", "W/[m²·K]" (crop `en-argument-chip.png`). The toolkit already avoids this in `INTENSITY_UNIT`, but the scene does not. Readers may take the brackets as notation. | Use the bracket-free form everywhere: "E, kWh/a", "E(θ*)" written as "E at θ*", "W per m²K". Apply the toolkit's `INTENSITY_UNIT` rule to the scene labels too. |
| m2 | [de] | de formulate, formulate_2, fixed (WWR); de response, sweep, sweep_2, sweep_3 (COP, EER) | toolkit section chip "WWR" (`room_section`); derivation word "COP" / "EER" | English abbreviations in German chips: "WWR 60 % · 14,4 m²" and "d 0,0 m"; "Heizbedarf ÷ COP", "Kühlbedarf ÷ EER". | German chip "Fensteranteil 60 %" in the section; "COP" → "JAZ" (Jahresarbeitszahl), "EER" → "JAZ Kälte". Keep "d" as a symbol only with its German name in the card. |
| m3 | [de] | de response and response_2 (formula row), de response caption, de global_2 and de local (tables) | `PARTS` in scene line 24; formula word list in toolkit `electricity_formula`; narration row response line 42 | Three German term sets: formula row "Heizen / Kühlen / Licht / Summe", derivation results "Heizen / Kühlen / Licht", caption "Heizung, Kühlung und Beleuchtung", tables "Heizstrom / Kühlstrom / Beleuchtung". "Licht" also means light in general. | One German set for the three parts: "Heizstrom / Kühlstrom / Beleuchtung" (also in the formula row and derivations), caption "Heizstrom, Kühlstrom und Beleuchtung". Use "Ost" or "O" consistently (the compass ring uses "O", tags and ticks "Ost"); pick one. |
| m4 | [layout] | local_2, compare_2 (en and de) | `_phase_extrema` step `local_2` (scene line 509) does not fade `m["start"]` (line 501); compare_2 adds a second start ring (line 563) | The start ring and its "start" tag stay in local_2 beside the local ring and cross the curve (crop `en-compare_2-route.png`). This is audit r2 D14, still open. | Add `_mn.FadeOut(m["start"])` at the start of the `local_2` step; in `compare_2` keep only the route start. Put the "start" tag above its ring, clear of the curve. |
| m5 | [layout] [de] | de response, de response_2 | `_causes` line 340 (derivation width 5.5) | "= 135 kWh/a" ends about 4 px from the bar-scale axis at x about 1090 px (crop `de-response-lighting.png`). | Shorten the German label ("Leistung × Stunden × Lampen an = Licht") or reduce the width to 5.2 for German. |
| m6 | [layout] [sync] | en local ("electricity."), en global ("range."), en argument ("kWh/a."), en scope_2 ("360°."); de global ("Bereich."), de global_3 ("CHF.") | captions, narration lines 50 (local), 52 (global), 55 (argument), 67 (scope_2) (en) and 52 (global), 54 (global_3) (de) | Orphan words on the second caption line (audit D24, still open). | Shorten by two or three words, for example en local "Every neighbour is worse, so we stop at 360°, north." and de global_3 "Der Halt im Nordtal verschwendet jedes Jahr Strom und Geld." |
| m7 | [abstract] | response to maximize (bar column) | `_bars` / `_ghost_bar` lines 180 to 195 | The bar column has an axis line without ticks, and the ghost bars name their bearing but not their segments (audit D25, D5, still open). | Add ticks at 0, 500 and 1,000 kWh/a beside the bar column, or drop the axis line. Label the ghost segments with their values in the ghost colour. |
| m8 | [layout] | en fixed, de fixed | section chip "WWR 60 %" in toolkit `room_section` | The chip sits on the sun rays and the light beam (crop `en-fixed-wwr.png`). | Move the chip above the glass, out of the ray band. |
| m9 | [sync] | scope (en and de) | narration line 66 | "Change window share, overhang or climate…": climate is never varied and nothing on screen shows a climate case. | Either show a climate case as a fourth tile, or drop "or climate" from both languages. |
| m10 | [sync] | maximize, negate_2 (en and de) | `_phase_maximize` line 584 (bracket at `BAR_AT[0] + .95`) and `_bracket` lines 610 to 612 | The saving bracket has no horizontal guides from the bar tops (1,131 and 681) (audit D27, still open). `global_3` and `feasible_2` have the guide, so the frames differ. | Add two dashed guides from the 1,131 and 681 bar tops to the bracket, as in `global_3`. |
| m11 | [abstract] | local, local_2 (en and de) | `_neighbours` lines 465 to 477 | Neighbours are numbers in a table: the room, section and bar at 310°, 320°, 355°, 365° are not drawn (audit D3, partly open). The table itself closes. | Add a mini plan at each neighbour, or one small section per neighbour with its sun hours, so the ΔE has a drawn cause. Optional if B1 and B2 are fixed first. |
| m12 | [layout] [process] | both layout reports (`…-en-layout.md`, `…-de-layout.md`) and `T/scene-extrema.md` ("0 findings in 29 frames") | `T/layout_strict.py` (used by `T/capture.py`) | The reports say "0 findings" in every frame, but M1, m4 and m5 are visible overlaps. The strict check tests boxes and text, not limit lines, polygon edges or proximity of 4 px. The scene report overstates the result. | Extend the strict check with text against lines and polygon edges, and a 6 px minimum clearance. Correct the claim in `scene-extrema.md` until the check passes. |
| m13 | [process] | `T/check-r2-extrema.py` | line asserting "sweep_3 'the total peak' at 270 deg" | The old checker now fails on the current scene because the caption moved to 282° (the scene is right, the checker is stale). Use `T/verify-check-extrema.py`. | Retire `check-r2-extrema.py` or update its peak assertion to 282°. |

## 3. Per-key result

Frame reading only. "ok" means no defect in that frame beyond the listed IDs. All 29 keys are present in both languages with the same structure.

| key | en | de | defects |
| --- | --- | --- | --- |
| formulate | ok | ok | m2 (de WWR chip) |
| formulate_2 | ok | ok | m2 (de WWR chip) |
| fixed | ok | ok | m1 (W/[m²·K]), m8 (en/de), B2 (chips change no number) |
| response | ok | ok | B2, M3, m3, m5 (de) |
| response_2 | ok | ok | M3, m5 (de) |
| domain | ok | ok | m1, m7 |
| sweep | ok | ok | B2, M3, m1, m2 (de COP) |
| sweep_2 | ok | ok | B2, M3, m2 (de EER) |
| sweep_3 | ok | ok | M3, m2 (de COP/EER) (the 282° peak is correct and marked) |
| sweep_4 | not ok | not ok | B1 (rays at 0 h, disc), M2 (164 vs 161), M3 |
| sweep_5 | ok | ok | M3 |
| local | not ok | not ok | B1 (winter rays at 0 h), M3, m1 |
| local_2 | not ok | not ok | B1, m4, m11 |
| global | ok | ok | m1, m7 |
| global_2 | ok | ok | m3 (de), m7 |
| global_3 | ok | ok | m6 (de "CHF.") |
| argument | ok | ok | m1 (E[θ*]), m6 (en "kWh/a.") |
| argument_2 | ok | ok | m1 |
| compare | ok | ok | none beyond shared items |
| compare_2 | ok | ok | m4 |
| maximize | ok | ok | m7, m10 |
| negate | ok | ok | m1 (S[θ], E[θ]) |
| negate_2 | ok | ok | m10 |
| constraint | not ok | not ok | B1 (summer rays/disc at 360°), M1 |
| constraint_2 | not ok | not ok | B1, M1 |
| feasible | not ok | not ok | B1, M1 |
| feasible_2 | not ok | not ok | B1, M1 |
| scope | ok | ok | M4, m9 |
| scope_2 | ok | ok | m6 (en "360°.") |

Verdict per key is "not ok" where a blocker or a major applies. The text-only keys (captions) are clean on numbers and on sync, except for the items listed.

## 4. Checks performed

Script `T/verify-check-extrema.py`, log `T/🗑️generated/verify-extrema/check-verify.log`.

- Bars and parts (6 bearings, 90°, 180°, 282°, 315°, 360°, 450°): heating, cooling, lighting, E and CHF/a, 30 values, all match the model after rounding. Shown part sums close with the shown totals (for example 218 + 778 + 150 = 1,146).
- Derivations: 152 kWh/a demand (180°), 1,510 kWh/a cooling demand (180°), 2,334 kWh/a cooling demand (282°): model matches. ÷ 3.5 and ÷ 3.0 steps close. Lighting: 180° "0.43 × 1,500 × 21 % = 135" closes; 360° "25 %" does not (M2).
- Sun: direct-sun hours (7 h winter and 9 h summer at 180°; 4 and 3 h at 90°, 282° and 450°; 3 h summer and 0 h winter at 315°; 0 and 0 at 360°): all match `sun_samples`, 14 values. Strength %: 14 values match `sun_<season>`.
- Neighbour tables: 310°, 320° (heating, cooling, lighting, ΔE) and 355°, 365°: all match to 0.1 kWh/a, and every printed row sums to its ΔE. The "0.0" lighting cell at 355° and 365° is the largest-remainder share of −0.07, so the row closes with 2.6.
- Global and east tables: north → south changes −268, +96, −29, −201 match. East → south saving 450 kWh/a matches; the parts 138, 300, 12 are the differences of the printed rounded part values (181 → 43 etc.), so they close on screen; the raw model difference of heating is 137.3 (note only).
- Maximum and minimum: E maximum on 262–300° at 282° (1,145.9 kWh/a), E(270°) = 1,131.1, S(282°) = −14.8, S minimum −14.8 at 438°. Matches the captions.
- Scope tiles: window 30 %, 60 %, 80 % minima 832, 882, 966 at 360° match. Overhang 0.6 m minimum 743 at 354° matches, but the curve steps (M4).
- Chips: 882 − 681 = 201, 29.5 %, 50 CHF/a; 449.8 kWh/a × 0.25 = 112.5 CHF/a; 2,334 ÷ 3,0 = 778; 1,510 ÷ 3,0 = 503; 152 ÷ 3,5 = 43; 882 × 0,25 = 221 CHF/a. All close.
- Captions versus model claims: "cooling lowest so far" (sweep_2, 90–180°) true; "cooling least" at 360° (sweep_4) true; "curve ends where it began" (sweep_5) true; "every neighbour is worse" (local_2) true; "cooling rises faster than heating falls" (local_2) true (−0.2 vs +2.8); "global minimum" (global) true; "north 29.5 % worse" and "+50 CHF/a" true; "highest saving at θ* = 180°" (negate_2) true; "courtyard price" (feasible_2) true; "north is best among allowed" (feasible) true; scope_2 "north 360°" true; scope "climate moves the optimum" not shown (m9).

Not verified: the German translations of the narration beyond what is on screen (the tests `C/🧪️tests/🐍️.py` cover length and bilingual presence, not meaning); audio timing; motion (end states only).

## 5. Audit r2 status (`T/audit-r2-extrema.md`)

| ID | status |
| --- | --- |
| D1 (sweep_3 caption at 270° vs 282° peak) | resolved: marker and caption at 282°; the stale checker still says 270° (m13) |
| D2 (depth, blocker) | open: B2 |
| D3 (neighbours only as numbers) | partly: decomposition tables; no drawn cause (m11) |
| D4 (strips without hour ticks) | resolved: strips removed |
| D5 (ghost segments unnamed) | open: m7 |
| D6 (scope section and plot overlap) | resolved: section removed from scope |
| D7 (negate tags on bars) | resolved: tags below the zero line |
| D8 (lens over x ticks) | resolved: lens removed |
| D9 (ghost bar on East 450° tick) | resolved |
| D10 ("no direct sun by day" over figure) | resolved: text removed |
| D11 (section titles on dimension) | resolved: labels at left, tight but clear |
| D12 (legend under the curve peak) | resolved |
| D13 (price chip over title chip) | resolved; the new overlap with the limit line is M1 |
| D14 (start ring in local_2) | open: m4 |
| D15 (German "+Beleuchtung" touching) | resolved; the German derivation now touches the bar axis (m5) |
| D16 (bar unit labels touching) | resolved |
| D17 (labels below 14 px) | resolved: smallest labels 14 px |
| D18 (brackets for parentheses) | open: m1 |
| D19 (112 vs 112.5 printed arithmetic) | resolved: "449.8 × 0.25 = 112.5" |
| D20 (optimum moves: direction shown?) | changed: direction now shown by a step artefact (M4) |
| D21 (sun altitude read as bearing) | resolved: "noon altitude 66°" / "Mittagshöhe 66°" |
| D22 (global vs feasible optimum wording) | resolved |
| D23 (German term sets) | open: m3 (and "O" vs "Ost") |
| D24 (orphan words) | open: m6 |
| D25 (bar axis without ticks) | open: m7 |
| D26 (y axis from 500) | resolved: zero baseline |
| D27 (bracket guides, tariff) | partly: tariff stated; guides still missing in maximize and negate_2 (m10) |
| D28 (layout checker "none") | open: checker still misses lines and edges (m12) |

## 6. Files

- Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py`
- Toolkit room drawing: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py`
- Toolkit derivations and formulas: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🐍️.py`
- Model: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py`
- Frames: `T/🗑️generated/frames/Beat1_LocalGlobalExtrema/`
- Crops and logs: `T/🗑️generated/verify-extrema/` (`check-verify.log`, `check-stock.log`, the crop PNGs)
- Check scripts: `T/verify-check-extrema.py`, `T/verify-crops-extrema.py`
