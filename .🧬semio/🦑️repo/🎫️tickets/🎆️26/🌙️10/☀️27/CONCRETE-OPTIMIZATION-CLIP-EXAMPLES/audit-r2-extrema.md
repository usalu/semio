# Audit r2: `Beat1_LocalGlobalExtrema` (topic `📉️extrema`)

Scope. Scene `C/📉️extrema/🐍️.py` (27 NARRATION keys, file 16:47), model `C/🧮️model/🐍️.py` and `🔣️.json` (16:55 / 16:40). Frames `T/🗑️generated/frames/Beat1_LocalGlobalExtrema/` are dated 18:15 to 18:18, so they show the current code. Reviewed: all 27 English frames (end states) and 8 German frames (formulate, response, sweep_3, global_2, maximize, negate, feasible_2, scope_2) plus both layout reports. Depth benchmark: six energy frames in `T/🗑️generated/energy-frames/` (Beat1_LeistungUndEnergie-summe, Beat8_Hauptgleichung-chain, Beat1_LastZuerstSenken-envelope, AnlagenVerluste-sum, Beat5_Verschattung-raffstore, Beat3_IsolateAirflow-compare) and their sources under `C/…/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` (chain_src), `…/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` (chain_marks), `…/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` (substitute step). Motion was not judged; end states only. Number check: `T/check-r2-extrema.py`, log `T/🗑️generated/r2/check-r2-extrema.log` (66 screen values match, 0 fail, 23 derived facts).

## 1. Verdict

The r1 blockers are fixed (the 90° point now has its marker, the local clause shows the room at 315°, and the summer sun at 360° is 0 h), and every on-screen number I recomputed matches the model except one caption: sweep_3 calls 270° "the total peak" while E keeps rising to 1,146 kWh/a at 282°. The clip is still far from the bar: only the façade bearing ever moves, window share, overhang and insulation are fixed chips that change no drawn consequence, and no heating, cooling or lighting value is derived on screen the way the energy clips derive theirs (the energy clips substitute every input into a visible formula). Layout is not sign-off ready either: about eight keys in English and at least two in German have text running into the section drawing, axis ticks, bars or the person glyph, and both layout reports still say "none" for many of them.

## 2. Per-beat table (English frame; defect IDs refer to section 3)

| key | on screen | narration | defects |
| --- | --- | --- | --- |
| formulate | Plan 180° S with sun arcs labelled "summer 66°", "winter 19°"; facts card 8 × 6 × 3 m, 48 m², one 24 m² façade; section "WWR 60 % · 14.4 m²", "d 0.0 m" | One glazed façade: which compass direction should it face? | D21, D17 |
| formulate_2 | Strip "Design variable θ", East 90° … East 450°, pointer at 180°; plan 180° | θ: east 90°, south 180°, west 270°, north 360° | D21 |
| response | "E = 43 + 503 + 135 = 681 kWh/a", three cards, winter section "direct sun 7 h", bar 681 kWh/a ≈ 170 CHF/a, chips 60 % / 0.0 m / 16 cm | Objective: electricity for heating, cooling, lighting, kWh/a | D2, D18, D25 |
| response_2 | "Goal: minimize E, lower is better", "Tariff 0.25 CHF/kWh" | Minimize; values from a simplified model | D2 |
| domain | Axes E 500–1,300 with θ ticks; marker at East 90° with chip "θ 90° · 1,131 kWh/a"; bar 1,131 ≈ 283 CHF/a | Turn from east 90° through all orientations | D26, D17 |
| sweep | Curve 90° to 180°, marker 681 at 180°; direct-sun strips for summer and winter; winter section 7 h | Low winter sun heats for free: heating falls | D4 |
| sweep_2 | Summer section "direct sun 9 h"; bar still shows cooling 503 at 180° | High summer sun: cooling drops to the lowest so far | D2 (claim verified on 90–180) |
| sweep_3 | Marker 270° at 1,131; summer section 4 h; bar 181 / 803 / 147 | Low evening sun: cooling and "the total peak" | D1, D2 |
| sweep_4 | Marker 360° at 882; summer section "direct sun 0 h", caption text "no direct sun by day" across the person; bar 311 / 407 / 164 | Cooling least, heating rises | D10 (claim verified: cooling minimum is at 360° on 90–450°) |
| sweep_5 | Marker 450° at 1,131; curve closes; curve rises above the dashed line near 438° without comment | At 450° the façade faces east again | D1 |
| local | Start ring at 315° (1,042); plan 315° NW; summer section 3 h; lens "neighbours ±5°" (+25 at 310°, −26 at 320°) over the x ticks | Start at 315°, always turn toward less electricity | D3, D8 |
| local_2 | Local ring at 360° (882); plan 360° N; lens (+3 at 355° and 365°) over "West 270° / North 360° / East 450°"; start ring still visible; "no direct sun by day" over person | Every neighbour is worse: local minimum at 360° | D3, D8, D10, D14 |
| global | Ghost bar "N 360°" 882 beside live bar "S 180°" 681; green square at 180°; legend; dashed level line | South, 180°, is lower still: global minimum | D5, D9 |
| global_2 | Chip "882 − 681 = +201 kWh/a · +29.5 % · +50 CHF/a"; bracket "+201 kWh/a" | Stopping in the north valley wastes electricity and CHF | D9, D24 |
| argument | Chips "θ* = 180° · south" → "E(θ*) = 681 kWh/a", tags design / value | Optimal design θ* = 180°; E(θ*) in kWh/a | D17, D18 |
| argument_2 | Same chips, indicated in turn | Keep them apart: θ* is a direction, E(θ*) is kWh/a | D24 |
| compare | Scan line at 180°; cards "North 360° · 882 kWh/a", "South 180° · 681 kWh/a · best" | Scan every degree, compare all valleys | D17 |
| compare_2 | Descent route 315° → 360° drawn; start ring with tag crossing the curve | A search can end in the wrong valley | D14 |
| maximize | Dashed east 90° reference room; orange east reference line 1,131; S arrow 1,131 → 681; chip "1,131 − 681 = 450 kWh/a ≈ 112 CHF/a"; ghost bar "E 90°" | How many kWh/a and CHF/a do we save against east? | D19, D16, D9 |
| negate | S curve 0–500, peak 450 at 180°; tags "S = 0: east façade", "S < 0: worse than east" placed over the bars; "min E ⇔ max S" | Minimizing E is maximizing S | D7, D5 |
| negate_2 | Marker 180° "θ 180° · S 450 kWh/a"; bracket and chip indicated; no guides | Highest saving at the same orientation: θ* = 180° | D27 |
| constraint | Courtyard sector on plan, street band south; plot band 90°–270° "infeasible: street noise"; chip "Constraint: 270° ≤ θ ≤ 450°"; room 360° N; bar 882 ≈ 221 CHF/a; "no direct sun by day" over person | Street south is loud: face the courtyard, north ± 90° | D10, D13 |
| constraint_2 | Cross on the 180° ghost inside the band; same plan and section | A constraint adds no objective and changes no E | D10 |
| feasible | Green square at 360° (882); legend "feasible minimum / excluded"; ghost bar "S 180°" 681 at left; bars 14 / 18 kWh/[m²·a] | Among allowed orientations, north is the global minimum | D22 |
| feasible_2 | Chip "Courtyard price: 882 − 681 = +201 kWh/a · +50 CHF/a" at top, bracket +201; "no direct sun by day" over person | The courtyard has a price | D13, D10 |
| scope | Card Direction / Domain / Scope; plan with courtyard sector; three curves (80 %, 60 %, 30 %); tiles and ramp 832–966 kWh/a; section still present | Change window share or climate: value, perhaps direction | D6, D20, D10 |
| scope_2 | One curve (60 %), answer ring at 360° "882 kWh/a", "≈ 221 CHF/a"; section still overlapping the plot axis | With the courtyard rule the answer is north, 360° | D6, D10, D24 |

## 3. Defects

### 3.1 Status of the r1 findings (`T/audit-extrema.md`)

| r1 finding | status in r2 |
| --- | --- |
| domain: plot empty at 90°, no marker | Closed: marker, chip and curve start visible in `domain` |
| local: room shown at 360° while caption says 315° | Closed: `local` shows the plan, section and bar at 315° |
| sweep_4: low summer sun at 360° contradicts caption | Closed: summer section at 360° reads "direct sun 0 h" |
| "Reference" label collides with the design row (global, global_2, maximize, negate_2) | Closed: design row is clear; ghost bars named by bearing; east reference labelled in plot |
| local_2 probes hidden under marker | Partly: replaced by a lens box that now covers the x ticks (D8) and carries no physics (D3) |
| "summer 66°", "winter 19°" read like bearings | Open (D21) |
| Compass letters overdrawn by sun dots | Open (formulate: S; sweep_3: W) |
| Yellow carries several meanings | Partly: local ring is beige now; yellow still marks sun arcs, sun dots and lighting |
| scope: empty lower-right, no E tie | Changed: plot and tiles added, but new overlaps (D6) |
| scope_2 ramp unlabelled, tiles without kWh/a | Closed: tiles show kWh/a; ramp ends labelled 832 and 966 |
| arithmetic not shown | Closed for sums and differences (chain, global_2, maximize, feasible_2) |
| sweep_2 / sweep_4 comparisons | Closed in substance: both claims verified; curve not shown as comparison |
| maximize: gap S not drawn, CHF missing | Closed: S arrow and "≈ 112 CHF/a" present |
| negate_2: no guides; tariff not stated | Open (D27) |
| "none" layout sign-off | Open (D28): checker now lists overlaps for local, scope, negate but misses glyph and tick overlaps |
| feasible_2 chip touches legend | Changed: chip now touches the title chip (D13) |
| constraint chip straddles 270° limit | Closed |
| argument tags 13 px; "[south]" | Partly: "·" fixed; tags still 14 px (D17) |
| global legend under dashed line | Closed |
| negate: S dips below zero unexplained | Partly: tag added, but it sits over the bars (D7) |
| feasible: "global optimum" vs "feasible optimum" | Open (D22), now "global minimum" vs "feasible minimum" |
| response: bar axis without ticks | Open (D25) |
| orphan words in captions | Open (D24) |
| beat subtitle and footnote | Closed: subtitle chip and top-right model note present |
| axis title brackets; y axis from 500 without break | Open (D18, D26) |

### 3.2 Defect list

Tags: [abstract], [numbers], [layout], [sync], [depth], [de]. Severity: blocker, major, minor.

| ID | tag | severity | keys (en / de) | evidence |
| --- | --- | --- | --- | --- |
| D1 | [numbers] [sync] | blocker | sweep_3; sweep_5 | Caption: "cooling electricity and the total peak" (de: "Strombedarf erreichen ihr Maximum"). The marker at 270° reads 1,131 kWh/a, but E is still rising there: E(275°) > E(270°), and the peak is 1,145.9 kWh/a at 282° (also 438°). Visible in sweep_5, where the curve passes above the dashed 1,131 line. Cooling peaks near 94° (804.8), cooling(270°) = 803.2, so the cooling part is nearly flat. |
| D2 | [depth] | blocker | all | Only θ moves. Heating 43, cooling 503 and lighting 135 are results with no derivation on screen. Window share, overhang and insulation are chips that never change a drawn quantity or a number. No daylight factor, no façade cost beyond tariff × kWh, no carbon. The energy clips show each term as a live formula with substituted inputs (for example solar gain "400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W", "Q_h,g = 13300 · (1/0.95 − 1) ≈ 700 kWh/a"). Lamp dimming with autonomy is not discernible across frames. |
| D3 | [abstract] | major | local, local_2 (de same) | The ±5° neighbours are shown only as ΔE numbers (+25/−26; +3/+3). No room, section or bar is drawn at 310°, 320°, 355° or 365°, so the reason 360° is a minimum is a number only. The same holds for the S values in negate (S = −14.8 near 282°). |
| D4 | [abstract] | minor | sweep onward | The two direct-sun strips ("Summer", "Winter") have no hour ticks; only the caption "8 h bottom, 16 h top" names the scale. The cursor shows θ but not the hours count, so the strip is not bound to the section's "direct sun N h". |
| D5 | [abstract] | minor | global, feasible (de same) | Ghost bars show coloured segments with no names or values per segment; only the bearing tag is shown. |
| D6 | [layout] | major | scope, scope_2 (de scope_2) | Plot title "E (kWh/a)" sits on "direct sun 0 h" (x≈247–360, y≈392–405). Tick "1,400" (x≈320–340, y≈425) and "800" (x≈305–340, y≈541) sit on the section's right wall. "Street" (x≈190–245, y≈436; de "Straße") sits on the section roof. "West 270°" sits at the section floor line. The section (x 107–340) and the plot (axis at x≈352) are not separated. |
| D7 | [layout] [sync] | major | negate, negate_2 (de negate) | "S < 0: worse than east" (de "S < 0: schlechter als Ost") runs at x≈866–1040, y≈380 across the E-90° bar (x 896–968) and the live bar (x 995–1068). The dips it explains are at x≈666 (282°, S ≈ −15) and x≈845 (438°), so the tag is detached from them. "S = 0: east façade" also crosses the vertical axis at x≈445. |
| D8 | [layout] | major | local, local_2 (de same) | The lens box (x≈620–840, y≈325–410) covers the lower part of the x tick labels "West 270°", "North 360°", "East 450°" (de "Nord 360°", "Ost 450°"). Its own labels 355° / 360° / 365° sit on its bottom edge. Checker lists 10–11 overlaps for these keys and no fix. |
| D9 | [layout] | minor | global, maximize, negate, negate_2 (de same) | "East 450°" tick label (x≈818–903, y≈413) collides with the ghost bar (x≈896–968) at its degree sign. |
| D10 | [layout] | major | sweep_4, local_2, constraint, constraint_2, feasible, feasible_2, scope, scope_2 (de feasible_2, scope_2 checked) | "no direct sun by day" (de "tagsüber keine direkte Sonne") runs from x≈107 across the lamp and the person figure (x≈240–262, y≈480–545). Eight English frames and both German frames checked. The layout report gives 0 overlaps for sweep_4 and constraint. |
| D11 | [layout] | minor | all section frames | "Summer sun" / "Winter sun" (de "Sommersonne" / "Wintersonne") end on the "6 m" dimension line, which starts at x≈118, y≈577. |
| D12 | [layout] [de] | minor | global (de), global_2 (de) | Legend "globales Minimum" (x≈514–668, y≈228) runs under the curve peak (x≈665, y≈228). English legend is clear. |
| D13 | [layout] | minor | feasible_2, constraint (de: Innenhof) | Price chip (x≈380–917, y≈97–133; de "Hofpreis") overlaps the corner of the title chip (right edge x≈388, y≈62–104). "Courtyard" / "Innenhof" (y≈110) touches the title chip. |
| D14 | [layout] | minor | compare_2, local_2 | "start" tag at 315° (x≈735–778, y≈255) lies on the curve. The start ring stays into local_2 beside the local ring. |
| D15 | [layout] [de] | minor | response (de) | "+Beleuchtung": the plus sign touches the word; the three card headers nearly fill the column. |
| D16 | [layout] | minor | maximize (de same) | Bar unit labels "24 kWh/[m²·a]" and "14 kWh/[m²·a]" touch (x≈885–1080, y≈575). "≈ 170 CHF/a" sits about 8 px from the bracket arrow at x≈1087. |
| D17 | [layout] | minor | all | Many labels are 11–14 px at 1280 px width: "6 m", "3 m", bar unit captions, "design" / "value" tags (14), "Winter 19°", compass words, sun-map caption (13), curve ticks at 14. Too small for a lecture projection. |
| D18 | [layout] | minor | all | Parentheses render as square brackets in the scene font: "Electricity E [kWh/a]", "in kWh per year [kWh/a]", "E[θ*]" appear in the PNGs while the source strings use "( )". This is the same bracket style the energy clips use for units ("P [kW]", "Q [kWh]"), so the convention is ambiguous. |
| D19 | [numbers] | minor | maximize (de same) | Chip "1,131 − 681 = 450 kWh/a ≈ 112 CHF/a": the printed product 450 × 0.25 is 112.5, which rounds to 113. The model gives 112.46 (449.8 × 0.25), so the screen arithmetic does not reproduce its own result. |
| D20 | [sync] | minor | scope (de same) | Caption "the optimum moves: its value, perhaps its direction". For window share 10–80 % in 2.5 % steps the optimum stays at 360° on 270°–450° (all three curves have their minimum there). The direction claim is not shown and not produced by this model under the window-share change. |
| D21 | [numbers] [de] | minor | formulate, formulate_2, sweep_3 (de "Sommer 66°", "Winter 19°") | Noon sun altitudes (19.16°, 66.04°) are printed with the degree sign used for bearings θ and read as bearings. The German labels do not say "Höhe" or "Sonnenhöhe". |
| D22 | [sync] | minor | feasible, feasible_2 (de same) | Caption "is now the global minimum" while the legend says "feasible minimum" (de: "globales Minimum" vs "zulässiges Minimum") for the same point. |
| D23 | [de] | minor | response, sweep, global, maximize (de bars and cards) | Terms differ within German: bars say "Heizen / Kühlen / Licht", cards and chain say "Heizstrom / Kühlstrom / Beleuchtung". Compass abbreviation "O" appears in the ghost tag "O 90°" (maximize) and plan labels, but "Ost" in ticks. |
| D24 | [layout] [sync] | minor | local, global, argument_2, scope_2, global_2 (de) | Orphan words on the second caption line: "electricity." (local), "range." (global), "kWh/a." (argument), "360°." (scope_2), "CHF." (global_2 de). |
| D25 | [layout] | minor | response, domain | The bar column axis line has no ticks or scale; "681 kWh/a" floats above the bar and "14 kWh/[m²·a]" is the only scale hint. |
| D26 | [numbers] | minor | domain to sweep_5 | Plot y axis starts at 500 kWh/a with no zero or break mark. On the 500 baseline, 681 sits at 29 % of the way to 1,131; on a zero baseline it would sit at 60 %, so the drop looks far larger than it is. |
| D27 | [sync] | minor | negate_2 | The bracket has no horizontal guides to the 1,131 and 681 bar tops, and the tariff is not stated at this clause (0.25 CHF/kWh appears only in response_2). |
| D28 | [layout] | minor | process | Layout reports say "none" for formulate*, response*, sweep*, domain, constraint, constraint_2, maximize, but the frames show D10, D11, D17. The checker misses glyph and tick overlaps (figure vs text, tick label vs panel) and does not check minimum font size. |

### 3.3 Number check (`T/check-r2-extrema.py`)

- 66 values printed on screen were recomputed from `orientation_study()` / `evaluate()` and all match: room 24 / 48 / 14.4 m², tariff 0.25, sun altitudes 19 / 66, bars and totals at 90°, 180°, 270°, 315°, 360°, 450° (heating, cooling, lighting, E, CHF/a, kWh/(m²·a)), direct-sun hours (7, 3, 9, 4, 0, 3, 4), lens deltas (+25, −26, +3, +3), global gap (+201 kWh/a, +29.5 %, +50 CHF/a), maximize saving (450 kWh/a), scope minima (832, 882, 966 at 360°), scope_2 bill (221 CHF/a).
- Every shown sum of parts equals the shown total (43 + 503 + 135 = 681, 181 + 803 + 147 = 1,131, 289 + 595 + 158 = 1,042, 311 + 407 + 164 = 882).
- One caption fails: D1 (sweep_3 "total peak"). Other caption claims verified: sweep_2 "lowest so far" (cooling minimum on 90–180° is at 180°), sweep_4 "cooling least" (global cooling minimum on 90–450° is at 360°), sweep_5 "curve ends where it began" (E(90°) = E(450°) exactly), negate "S = 0" at 270° (S = 0.00), negate_2 "highest saving at θ* = 180°".
- Derived facts: maxima of E at 282° and 438° (1,145.9 kWh/a); minimum of S at about −14.8 kWh/a; no window share from 10 to 80 % moves the optimum off 360° (D20).
- Rounding: 882.0 kWh/a × 0.25 = 220.5, shown as 221 (round half up); "≈ 112" for 112.46 is correct, but the printed arithmetic suggests 112.5 (D19).

## 4. Fix proposals

| ID | concrete fix |
| --- | --- |
| D1 | Bind a second marker to `study["local_maxima"][0]` (282°, 1,145.9 kWh/a). Travel sweep_3 from 180° to 282° instead of stopping at 270°, with chip "θ 282° · 1,146 kWh/a" and a dashed guide to the peak. EN caption: "Facing west, the low evening sun floods the room: cooling stays high, and the total peaks near 282°." DE: "… und die Summe erreicht ihr Maximum bei 282°." Add 282 to the bearing set in `check-extrema-numbers.py`. Compute the peak from the model, never type it. |
| D2 | Add one derivation card per term, bound to the live design through `_room.result` / `evaluate`: heating "H = (losses − η·gains) / COP" with the current numbers, cooling "Q_c = gains − η·losses / EER", lighting "9 W/m² · 48 m² · 1,500 h · (1 − autonomy) = 135 kWh/a" and solar "A_w · g · G · (1 − f·F_sh)". Add a second design at the same θ: overhang 0 → 0.6 m, showing shading_summer 0.00 → 0.89 and the cooling bar dropping. Keep WWR, overhang and insulation as live chips and show their effect on the section window and overhang. Show CHF/a and daylight % in the chip row so the example covers cost and daylight. Pattern: energy clips `solar-gains` chain_src and `primary-energy` chain_marks. |
| D3 | Replace the lens with three mini drawings: a 3-row strip per neighbour (plan at θ, mini stacked bar with segment labels, ΔE in kWh/a with sign). Use `energy_bars(..., compact=True)` already used for ghosts. Place them in the 1,200–1,300 kWh/a band above the curve between 230° and 300°, which is empty. For negate, add the plan of the room at the dip (282°) when the S tag appears. |
| D4 | Add hour ticks at 8, 12, 16 h on the left of each strip and a readout at the cursor "7 h" (winter) / "9 h" (summer) read from `sun_samples(θ, season)`. |
| D5 | Label ghost segments with their values (for example "311 / 407 / 164") in the ghost colour, and state the bearing in the chip. |
| D6 | In `_phase_scope` remove the section from `keep` so it fades out (the scope clause needs no section). If the section must stay, move it to x ≤ −7.0 and shrink to 2.3 units wide, and move the plot's y axis to x ≥ −4.2. Move "Street" to the compass ring. |
| D7 | Anchor "S < 0: worse than east" at `axes2.c2p(282, S(282))` below the zero line, between x 270° and 300°, with a leader. Keep "S = 0: east façade" at θ = 100° so it does not cross the y axis. Remove the tag from the bar column. |
| D8 | Move the lens to the free band of the plot (1,200–1,300 kWh/a, 230°–300°, see D3) and widen its frame so the axis ticks are never covered. Keep its bottom 0.1 units above the tick row. |
| D9 | Move the ghost bar 0.06 units right, or shorten the "East 450°" label by one character width; keep 16 px gutter. |
| D10 | Move "no direct sun by day" to the upper-left inside the room, left-aligned at P(0.15, 2.1) and wrapped into two lines ("no direct sun" / "by day") so it ends before x = 3.0 m. Or drop it and show "direct sun 0 h" only. Add the same rule to the German string. |
| D11 | Move the section titles above the roof line at the left wall (x0 + 0.05, y1 − 0.14, left edge). Start the "6 m" dimension at x ≥ 1.4 m, or shorten "Summer sun" to fit before x = 1.2 m. |
| D12 | Use `_legend(..., left=...)` in the German sweeps too, or shorten "globales Minimum" to "global" by a locale key. Keep the legend below the curve peak (y < 1,100). |
| D13 | Move the price row to y = TOP_ROW − 0.3 and right of the title chip (left edge at x ≥ −0.2), or set the title chip in the subtitle row. Move "Courtyard" / "Innenhof" below the compass. |
| D14 | Fade out `m["start"]` at the start of local_2. Place the "start" tag above the ring (UP × 0.3) so it clears the curve. |
| D15 | Reduce the card spacing in German (`buff .12`) or use "Licht" in headers too (see D23). |
| D16 | Move the intensity captions under the bar as one line "kWh/(m²·a)" with values 24 / 14 at 14 px. Move the bracket 0.2 units right of the bar column and the chip 0.1 units lower. |
| D17 | Raise every label to ≥ 14 px (LABEL_FONT_SIZE) and tags to 16 px; section dimension labels to 14 px. Check on a 1280 × 720 frame. |
| D18 | Check the glyph of "(" in the scene font. If it draws "[", use "·" or put units after a comma ("Electricity E, kWh/a") and write "E at θ*" instead of "E(θ*)". Use one unit convention across both clips. |
| D19 | Show the saving to one decimal: "≈ 112.5 CHF/a" (de "112,5"), or print "449.8 kWh/a × 0.25 CHF/kWh = 112.5 CHF/a" so the arithmetic closes. |
| D20 | Either show a case where the optimum moves (test overhang and insulation settings in the model first, then add that case to the scope tiles), or drop "perhaps its direction" from the caption (de: "vielleicht seine Richtung"). |
| D21 | Use "altitude 66°" / de "Höhe 66°" for sun labels; keep the degree sign for bearings only. |
| D22 | Caption: "Among the allowed orientations, north, 360°, is the best one" (de "… ist Nord, 360°, nun das beste"). Keep the legend "feasible minimum" / "zulässiges Minimum". |
| D23 | Use one German term set: "Heizstrom / Kühlstrom / Beleuchtung" in bars, cards and chain; "Ost" in every tag (no "O"). |
| D24 | Shorten by two or three words: local "Every neighbour is worse, so we stop at 360°, north." global "But south, 180°, is lower still: the global minimum." argument "The optimal design is θ* = 180°, south; E(θ*) is in kWh/a." scope_2 "With the courtyard rule θ in 270°–450°, the answer is north, 360°." global_2 de "Der Halt im Nordtal verschwendet jedes Jahr Strom und kostet auch CHF." |
| D25 | Add ticks at 0, 500, 1,000 kWh/a on the bar column, or remove the axis line and keep the total above the bar. |
| D26 | Mark the axis break at the bottom with "axis from 500 kWh/a" in MUTED, or start the y range at 0 and keep the 681–1,131 contrast with the sun strip. Pick one and say it in the caption. |
| D27 | Draw dashed guides from the 1,131 and 681 bar tops to the bracket. Put "0.25 CHF/kWh" into the chip row at this clause. |
| D28 | Extend `layout_conflicts` with: text against figures (section, person, lamps), text against tick labels of other axes, text against curve samples, bar column against text, and minimum font size 14 px. Make the report list every visible overlap, not "none", and keep "none" only when the new checks pass. |

Priority: D1 and D2 first (factual and depth). Then D6, D7, D8, D10 (major layout). The rest are polish once the layout checker (D28) is extended, so that each fix is checked by the tool and not by eye.
