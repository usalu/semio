# Audit round 2: Beat2_GridRandomSampling (`🎲️sampling`)

Read-only audit. Scope: all 33 English keyframes and 11 German keyframes (`de-parameters`, `de-run_2`, `de-grid_2`, `de-grid_4`, `de-growth_3`, `de-random`, `de-random_3`, `de-gaps`, `de-evaluate`, `de-compare_2`, `de-limits_4`) in `🗑️generated/frames/Beat2_GridRandomSampling/`, the scene `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py` (modified 17:10, frames 18:15 to 18:17, so the stills reflect the current code), the model `🧮️model/🐍️.py` and `🔣️.json`, and six energy reference stills. Numbers were recomputed by `T/check-r2-sampling.py` (run with the cache Python, 145 checks, 4 mismatches). The earlier audit (`T/audit-sampling.md`) is checked item by item in section 5.

## 1. Verdict

The clip is about two thirds of the way to the bar: almost every number on screen reproduces from the model, the room sections with sun rays, the stacked bars and the compare tiles are close to energy-clip depth, and most of the earlier layout and German defects are fixed. It still fails the owner's test where it matters most: lighting and daylight are never drawn (lighting is 42 % of the 522 kWh/a bar, yet no daylight or shading meter appears on screen), and no screen shows a substituted equation or a live meter with physical ends, which the energy clips use on every beat. Two arithmetic defects are blockers (stacked-bar parts that do not add to the printed total on `evaluate_2` and `limits_4`), and the closing card and the random table carry a count error and untranslated German headers.

Severity counts: blocker 2, major 10, minor 17 (29 defects, section 3).

## 2. Per-beat table

Key | What is on screen | Narration (EN) | Defects (section 3)

| Key | On screen | Narration (EN) | Defects |
| --- | --- | --- | --- |
| parameters | Elevation 8 m × 3 m, chips "WWR 45 % · 10.8 m²" and "d 0.6 m", sliders WWR 10–80 % and d 0.0–1.2 m; summer section with person; "1 m" and "6 m" dimensions | Which window share and overhang depth suit this south façade? | D17 (unexplained "1 m"), D16 (d 0.6 vs 0.60) |
| parameters_2 | Plane with marker chip "WWR 45 % · d 0.6 m"; coloured summer section with chip "WWR 45 % · d 0.60 m" | Each pair is one design: point and room move together. | D16, D17 |
| run | Winter 19° and summer 66° sections with rays; plane marker; note "0.25 CHF/kWh" | One simulation runs through a whole year of sun, heat and daylight. | D18 (two solstice days only, no daylight), D01 |
| run_2 | Bar Heating 58 / Cooling 245 / Lighting 219 = 522 kWh/a; "Cost 131 CHF/a" under bar; lamps unchanged | Winter sun cuts heating electricity, summer sun adds cooling, daylight cuts lighting. | D01 (blocker: daylight not drawn), D03 (cost label), D10 (no formula), D28 (DE word) |
| run_3 | Ramp 495 to 846 with grey "kWh/a"; point tag "522 / kWh/a" at 45 % / 0.6 m | One number per design: electricity in kWh/a, carried by each point. | D27 (grey 14 px unit) |
| budget | Nine pips "Run 1 / 9" under design chip; bar and sections | But only nine simulations are affordable. Which nine designs? | none |
| grid | Nine grey ghosts, column chips 10 % / 45 % / 80 %, y ticks 0.0 to 1.2 | A full grid uses three levels per axis and combines them all. | D23 (10 % column straddles y-axis) |
| grid_2 | Runs 1 to 5 coloured; tags 808, 809, 846, 637, 522 with "kWh/a"; board "522.1"; bar 637 = 58 + 408 + 171, cost 159 | Simulate them one by one: room, sun rays and electricity bar for each. | D16 (522.1 vs 522) |
| grid_3 | Runs 1 to 9; tags 547, 616, 499; "Run 9 / 9" | The ninth simulation empties the budget. | D23 |
| grid_4 | Green ring on 80 % / 1.20 m (499 kWh/a); outlined ghost bar of worst (846) at x 608 to 662 px; room of best; bar 499 = 34 + 304 + 161, cost 125 | The best of the nine is ringed; its room shows why. | D08 (why not shown), D09 (ghost bar overlaps tile) |
| growth | Four factor columns: window 10 / 45 / 80 %, overhang 0.0 / 0.6 / 1.2 m, orientation 90° E / 180° S / 270° W / 360° N (glyphs now correct), insulation 10 / 20 / 30 cm; "9 simulations" | More design variables: orientation with four levels, insulation with three. | none (glyph fix verified) |
| growth_2 | 9 of 108 blocks lit; tokens "3 · 3 · 4 · 3 = 108" | Levels multiply: 3 · 3 · 4 · 3 makes 108 simulations. | none (verified) |
| growth_3 | Blocks turn orange; chips "99 over budget", "12 × budget" | Every extra level multiplies the simulations: here twelve times our budget. | none (verified) |
| random | Nine ghosts on the plane; table "#, WWR in %, d in m" with 0.1-scale ghost thumbnails and bare numbers; chip "Recorded with the study" | Random sampling draws nine independent designs, recorded so everyone sees the same points. | D07 (abstract table), D05 (DE headers English) |
| random_3 | All nine coloured; bar 707 = 129 + 233 + 345, cost 177; board Grid 499.2, Random 561.8; chip WWR 18 % · d 0.08 m | Same budget, same model: nine more simulations. | none (verified) |
| gaps | Dashed orange empty box, chip "WWR 35 % – 70 % · d 0.15 m – 1.20 m"; "?" probe at box centre with chip "WWR 52 % · d 0.68 m"; "not simulated" bar | This box stays empty: no draw combines a large window share with a deep overhang. | D06 (caption describes another region), D15 (0.675 tie) |
| gaps_2 | Closest pair (33 / 0.72 and 35 / 0.78) ringed in orange, rings overlap; chip "Δ WWR 2 pp · Δ d 0.06 m · Δ 6 kWh/a" (verified); room and bar of 35 / 0.78: 562 = 75 + 202 + 285, cost 140 | Elsewhere draws crowd: two nearly identical rooms, two simulations. | D22 (ring overlap), D20 (only one room shown), D16 (cost 140 vs 562 × 0.25) |
| gaps_3 | Green ring on best random draw; chip "best of nine draws 561.8 kWh/a" | Equal probability does not guarantee even coverage in a small sample. | D19 (caption and chip disagree), D22 |
| lhs | Nine strata lines; green tick squares on both axes; chip "9 strata per axis: 7.8 pp · 0.13 m" (verified); "not simulated" bar; board row Latin hypercube | Latin hypercube sampling: nine strata per axis, exactly one sample in each stratum. | D24 (ticks unexplained) |
| marginals | Green vertical strip 41.1 to 48.9 % with chip "1 point" (verified); chosen draw 45 % / 0.33 m | This vertical strip is one window-share band with exactly one design. | none |
| marginals_2 | Orange horizontal strip d 0.27 to 0.40 m "1 point" (verified); strip drawn over the chosen ghost | The horizontal strip also holds exactly one: each axis is covered evenly. | D24 (strip dims the ghost) |
| cells | Nine cells filled of 81; dashed "largest empty box" with label only; "not simulated" bar | But the plane has 81 cells and only nine points: empty regions remain. | D13 (no numbers on the box) |
| evaluate | Nine ghosts; chip "WWR 14 % · d 0.73 m"; "not simulated" bar; "Run 0 / 9" | Until simulated, a point is only a position, with no result. | none |
| evaluate_2 | All nine coloured; room at 76 % / 0.60 m; bar total 599 with parts 34 + 424 + 140 = 598; cost 150; board row Latin hypercube 495.9 | Each point becomes a full building and runs through the same model. | D02 (blocker, parts sum), D10 |
| reveal | Heat map 495 to 846 with contours 500 / 600 / 700 (labels now clear of markers); counter "Dense grid: 41 × 41 = 1,681 simulations"; probe room at 80 % / 0.00 m with bar 775 = 33 + 630 + 112 (verified), probe not marked on the plane | To judge the point sets, reveal the surface with a dense reference grid. | D12 (probe detached from plane) |
| reveal_2 | Star at 66 % / 0.99 m; chip "WWR 66 % · d 0.99 m · 494.8 kWh/a"; room and bar 495 = 40 + 273 + 182 (verified) | Darker means less electricity. The lowest point lies inside the range. | D22 (star, triangle and ring cluster), D16 (494.8 chip vs 495 bar) |
| compare | Three tiles: Grid 499.2 (80 % / 1.20 m), Random 561.8 (35 % / 0.78 m), Latin hypercube 495.9 (68 % / 1.00 m); each with room, bar on one scale; colour key | Compare each point set's best design with the optimum of the dense reference grid. | D16 (1 dp tiles vs integer tags) |
| compare_2 | Optimum tile 494.8 kWh/a (66 % / 0.99 m); Δ 4.4 kWh/a · 0.9 %, Δ 67.0 · 13.5 %, Δ 1.1 · 0.2 % (all verified); Δ bars | Each best design is the best of nine, yet none reaches the optimum. | D26 (sliver bars) |
| compare_3 | Chip "9 vs. 1,681 simulations" at plane top; Δ values highlighted | The remaining gap is the price of having only nine simulations. | none |
| limits | Green ≤ +5 % outline (unfilled) over the north-east of the plane; chip "≤ +5 % of the optimum · ≤ 519.6 kWh/a"; optimum room returns | Sampling shows coverage and promising regions to search next. | D25 (outline only, label touches it), D14 (519.6 vs 519.5) |
| limits_2 | Cards "1,681 simulations → optimum 494.8 kWh/a" and "9 simulations → best of nine 495.9 kWh/a" | A finite sample proves no global optimum. | D04 (count), D21 (naming) |
| limits_3 | Closing chip "Coverage informs the search. It proves no optimum."; note "Simplified annual teaching model · Zurich-like climate" | The model itself still needs validation. | none on EN; DE not re-viewed |
| limits_4 | Answer chip "best of nine 495.9 kWh/a · Δ 1.1 kWh/a" (set not named); room at 68 % / 1.00 m; bar total 496 with parts 39 + 281 + 177 = 497; cost 124 | Nine simulations found a good south façade design, no proven optimum. | D02 (blocker, parts sum), D21 |

## 3. Defect list

Tags: [abstract], [numbers], [layout], [sync], [depth], [de]. Coordinates are approximate pixels on the 1280 × 720 still.

### Blockers

**D01 [abstract] blocker. Daylight and lighting are never drawn (run_2, run_2 DE, run_3).**
The caption says "daylight cuts lighting" (DE: "Tageslicht senkt Beleuchtungsstrom"). The lighting segment (219 kWh/a, 42 % of 522) has no physical cause on screen: the lamps in the room are identical in every frame, no daylight is drawn, and the model's daylight autonomy (0.66) and the summer shading (0.89) appear nowhere. The owner's rule is that each plotted value is shown by its meaning on the same screen.

**D02 [numbers] blocker. Stacked-bar parts do not add to the printed total.**
- evaluate_2 (76 % / 0.60 m): parts 34 + 424 + 140 = 598, total printed 599. Unrounded parts 34.41 / 424.24 / 139.89 give 598.54, so the total is right and the parts are rounded wrongly.
- limits_4 (68 % / 1.00 m): parts 39 + 281 + 177 = 497, total printed 496. Unrounded 38.58 / 280.60 / 176.68 give 495.87.
The AGENTS rule "signs and arithmetic shown on screen must be correct" is violated on two screens, one of which is the closing answer.

### Major

**D03 [numbers] major. "Cost 131 CHF/a" is the electricity bill, not the façade cost.**
It is kWh/a × 0.25 CHF/kWh (verified: 522.1 × 0.25 = 130.5 → 131). The model's façade cost for the same design (20,359.2 CHF, baseline) never appears, and the bill's arithmetic is not shown. The label "Cost" invites the wrong reading. Shown on run_2, run_3, grid_2, grid_4, random_3, gaps_2, evaluate_2, reveal (194 CHF/a), reveal_2, limits_4. Location: below each bar, x 810 to 940, y 580 to 595.

**D04 [numbers] major. "9 simulations → best of nine 495.9 kWh/a" (limits_2, limits_4 chip).**
The scene code takes `min` over the best of all three sets (grid 499.2, random 561.8, Latin hypercube 495.9), so the value is the best of 27 simulations, labelled as nine. The value is correct for the Latin hypercube set only, and no set is named.

**D05 [de] major. Random table headers are hard-coded English (random, DE frame).**
`_random` uses literal `"WWR in %"` and `"d in m"` (no `self.tr`). In `de-random` they stand at x 680 to 768, y 266 and x 680 to 732, y 302. The table is also the only place where "#" appears unlabelled in DE.

**D06 [sync] major. The gaps caption describes a different region than the drawn box (gaps, DE and EN).**
The caption says no draw combines a large window share with a deep overhang. Verified: no draw has WWR ≥ 55 % and d ≥ 0.8 m (count 0), and the drawn box is WWR 35 to 70 % by d 0.15 to 1.20 m (chip at x 205 to 533, y 170 to 203). The box is empty (verified, 0 draws inside), but the caption names a corner the box does not cover. A viewer who reads the chip sees 35 % called "large".

**D07 [abstract] major. The random table is numbers with 0.1-scale ghosts (random).**
Nine (WWR, d) pairs appear in a grid of bare numbers (x 815 to 1230, y 230 to 302). The unit sits only in the row header, the thumbnails are about 40 px, and the close pair #3/#4 overlaps. No room is readable and no electricity is shown until the sweep, so this screen is the most abstract in the clip.

**D08 [abstract] major. "Its room shows why" is not shown (grid_4).**
Only the best room is drawn. The reason is not on screen: the best design's cooling is 304 kWh/a and the worst (10 % / 1.20 m, 846 kWh/a) has no room or segment values. The ghost bar carries only the total.

**D09 [layout] major. The worst-design ghost bar overlaps a plane tile (grid_4, DE and EN).**
The outlined ghost bar spans x 608 to 662, y 436 to 558. The 80 % / 0.0 m thumbnail spans x 540 to 622, y 435 to 505, so the bar cuts into it and runs past the plane edge. Its label "worst 846 kWh/a" (x 562 to 710, y 404 to 420) floats over the 80 % column.

**D10 [depth] major. No equation with substituted values anywhere.**
The energy references show the mechanism as a formula whose symbols are coloured and whose numbers are substituted (Beat8: Φ_solar = G · A · F_f · g · F_sh, then 400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W against a 768 W reference bar; Beat3: 0,126 / 0,061 ≈ 2,1 chip with units). The sampling clip has no such band; its electricity bar is never derived. The baseline numbers reproduce exactly, so the band can be bound to the model (verified: 203.9 ÷ 3.5 = 58.3; 735.6 ÷ 3.0 = 245.2; 9 W/m² × 48 m² × 1,500 h × (1 − 0.6627) = 218.6; the three parts sum to the 522.1 kWh/a total).

**D11 [depth] major. No live shading or daylight meter (energy Beat5 pattern).**
Beat5 shows F_sh with a scale from "ganz verschattet" (0) to "ohne Verschattung" (1). The sampling clip has the same physics in the model (summer beam-weighted shading 0.89 at 0.6 m, daylight autonomy 0.66) but shows neither. Because the overhang is what changes the summer bar, the reader never sees the cause.

**D12 [abstract] major. The reveal probe is not marked on the plane (reveal).**
The room and bar sweep through 80 % / 0.00 m (775 kWh/a, verified), but the plane has only the grid corner marker there, so the room is detached from the surface it moves over. The caption "reveal the surface" then has no moving pointer.

### Minor

**D13 [numbers] minor. The largest empty box of the Latin hypercube has no numbers (cells).** Verified box: WWR 13.9 to 76.1 %, d 0.467 to 0.867 m. The box is labelled "largest empty box" only.

**D14 [numbers] minor. Threshold chip does not follow from the printed optimum (limits).** 494.8 × 1.05 = 519.54 prints 519.5, while the chip shows 519.6 (correct from the unrounded optimum). A reader who checks against the 494.8 on the cards gets a 0.1 difference.

**D15 [numbers] minor. Tie rounding of the probe centre (gaps).** The box centre is 52.49 % (chip 52, correct) and d 0.675 m exactly; the chip prints 0.68 (half up) while float formatting gives 0.67. Use one convention or show the box edges.

**D16 [numbers] minor. Precision is mixed on the same screens.**
- grid_2: board "522.1" vs tag and bar "522".
- reveal_2: chip "494.8 kWh/a" vs bar "495".
- compare: tiles "499.2" vs tag "499".
- gaps_2: bar cost 140, while 562 × 0.25 = 140.5 reads as 141 on the same screen (the unrounded value is 140.45).
- Overhang: "d 0.6 m" (parameters, parameters_2 plane chip), "d 0.60 m" (run onward), "d 0.00 m" (grid_2), axis "0.0".

**D17 [numbers] minor. Unexplained "1 m" dimension on every room section (parameters, run, run_2 and later).** It is not a model quantity (the room is 8 × 6 × 3 m) and no meaning is given. The overhang d (0.6 m, the variable the clip is about) is never dimensioned on the section.

**D18 [sync] minor. Caption says "a whole year" while the screen shows two solstice days (run).** The two sections show 19° and 66° noon sun; daylight and the annual sums appear only as bars. Caption: "One simulation sums a year of heat, sun and daylight; two solstice days are shown."

**D19 [sync] minor. gaps_3 caption is about coverage, the chip and ring are about the best draw.** Caption "Equal probability does not guarantee even coverage" vs chip "best of nine draws 561.8 kWh/a" on the crowded pair. Suggest: "The best draw sits in the crowded pair: nine draws do not guarantee coverage."

**D20 [sync] minor. gaps_2 says "two nearly identical rooms" but only one room and bar are shown.** The 33 / 0.72 draw (568 kWh/a) exists only as a ring; its room and value never appear in the still.

**D21 [sync] minor. Set and naming on the closing screens (limits_2, limits_4).** limits_4 chip does not say the Latin hypercube set. limits_2 labels the 41 × 41 grid minimum "optimum" while the caption says a finite sample proves no optimum. Use "dense-grid optimum" and "Latin hypercube: best of nine".

**D22 [layout] minor. Ring and marker clusters (gaps_2, gaps_3, reveal_2).** Orange pair rings (r 0.25, x 272 to 332, y 292 to 350) overlap each other and a neighbour ghost; gaps_3 adds a green ring over them. In reveal_2 the star (x 497, y 275), the Latin hypercube triangle with its green ring (x 510, y 270) and a diamond form one blob (x 485 to 530, y 255 to 290).

**D23 [layout] minor. The 10 % column touches the y-axis (grid, grid_2, grid_3).** Thumbnails span x 117 to 197, the y-axis line is at x 125, y 197 to 505.

**D24 [layout] minor. Unexplained ticks and strip z-order (lhs, marginals_2).** Green tick squares sit on both axes (x-axis y about 498, x 178 to 560; y-axis x about 126, y 245 to 455) with no legend. The orange strip in marginals_2 is drawn over the chosen ghost (45 % / 0.33 m) and dims it.

**D25 [layout] minor. The ≤ +5 % region is an outline and its label touches it (limits).** The "500 kWh/a" contour label (x 378 to 482, y 288 to 318) touches the green outline (x 352 to 380). The region is not filled, so "inside" has no area.

**D26 [layout] minor. Δ bars are unreadable slivers (compare_2).** Grid Δ 4.4 is a 0.03-unit bar (x 735 to 742, y 348 to 356) and LHS Δ 1.1 is a sliver (x 735, y 588). Neither has a scale; the Random bar is the only usable one.

**D27 [layout] minor. Secondary units are grey and small (run_3, reveal, compare).** Ramp unit "kWh/a" (x 470 to 530, y 135 to 150) and point-tag units are grey at 14 px; the plane tick labels are 14 px. Make the units INK at 15 px.

**D28 [de] minor. Word mismatch between narration and bar (run_2 DE).** Narration "Beleuchtungsstrom", bar label "Beleuchtung 219 kWh/a". Use one word.

**D29 [layout] minor (tooling). The layout reports say "none" for all 33 keys in both languages.** They do not detect the overlaps in D09, D22, D23 or the chip against tile cases; treat "none" as no information until the checker tests ring, marker, tile and ghost intersections.

## 4. Fix proposals (implementable, per defect)

- **D01.** In `run_2` and in the two sections, add a daylight band: an interior gradient from the glazing, depth proportional to the model's daylight factor (`daylight`, baseline 2.7 %), bound to the design trackers so it moves with WWR and d. Replace the static lamp glyphs with circles whose fill opacity is `1 − autonomy` (always_redraw). Add a LIGHT-coloured chip beside the lighting segment: "Daylight autonomy 66 % → lamps 34 % on" (DE "Tageslichtautonomie 66 % → Lampen 34 % an"). Source values: `evaluate()["autonomy"]`, `shading_summer`.
- **D02.** Show bar parts and total at one decimal everywhere. This sums exactly: evaluate_2 34.4 + 424.2 + 139.9 = 598.5 (total 598.5); limits_4 38.6 + 280.6 + 176.7 = 495.9 (total 495.9). Alternatively use largest-remainder rounding in `_room.parts` so parts sum to the rounded total (evaluate_2 35 + 424 + 140 = 599; limits_4 38 + 281 + 177 = 496). The one-decimal option is preferred because it keeps the model value visible.
- **D03.** Rename the label to "Energy cost 131 CHF/a" (DE "Energiekosten"), and add a grey 14 px formula chip under it: "522 kWh/a × 0.25 CHF/kWh". Keep the façade cost out of this clip or show it once as a separate chip ("Façade 20,359 CHF", model `cost`).
- **D04.** In `_limits` bind the card to `bests["lhs"]`: "Latin hypercube: best of nine 495.9 kWh/a". Name the set in the answer chip of `limits_4` too (D21).
- **D05.** Replace the literal headers in `_random` with `self.tr("WWR in %", "Fenster-WWR in %")` and `self.tr("d in m", "Vordach d in m")`. Move the table's first column left or shorten the German to "Fensteranteil %" and "Vordach m" so the header fits before x 815.
- **D06.** Either rewrite the caption to the box ("No draw falls inside this box: WWR 35 to 70 %, overhang 0.15 to 1.2 m.", DE "Kein Entwurf liegt in dieser Box: Fensteranteil 35 bis 70 %, Vordach 0,15 bis 1,2 m."), or draw the corner the caption names (WWR 55 to 80 %, d 0.8 to 1.2 m, verified empty) with its own dashed outline. The first option is simpler and fits the chip.
- **D07.** Put one 0.15-scale ghost above each column (as now at 0.1), write units into each cell ("14 %", "0.95 m"), and move the close pair (#3/#4) apart by the ghost offset already used in `draw`. Keep the table at 15 px.
- **D08.** Keep the best room and add the worst room (10 % / 1.20 m) at the same scale beside it, or label the segments of both bars ("cooling 304" vs the worst's cooling from `evaluate`). Change the caption only if the room cannot be added: "The best of the nine is ringed; its cooling is the lowest of the three bars."
- **D09.** Move `_worst_bar` to its own slot right of the live bar (x 1.2 units further right), keep it off the plane (x > 585 px), and place "worst 846 kWh/a" above the ghost, not over the plot.
- **D10.** Add a formula band under the bars in `run_2` (and reuse on grid_4): heating `203.9 ÷ 3.5 = 58.3`, cooling `735.6 ÷ 3.0 = 245.2`, lighting `9 W/m² · 48 m² · 1 500 h · (1 − 0.6627) = 218.6`, total `= 522.1 kWh/a`, cost `522.1 × 0.25 = 130.5 → 131`. Colour the symbols as the energy Beat8 clip does (heating HEAT, cooling COOL, lighting LIGHT). All values come from `evaluate` and `levels`.
- **D11.** In each season section add a meter with the energy convention: F_sh from 0 ("ganz verschattet") to 1 ("ohne Verschattung"), value 1 − shading (baseline summer 0.11), and a daylight meter 0 to 100 % with the autonomy value. Bind both to the design trackers.
- **D12.** Draw a ring or crosshair on the plane bound to the WWR and overhang trackers during the sweep, and label it with the nearest surface value in kWh/a. Keep the probe room and bar as now.
- **D13.** Add a chip beside the dashed box: "largest empty box: WWR 13.9–76.1 %, d 0.47–0.87 m", from `sampling_diagnostics("lhs")`.
- **D14.** Compute the threshold from the printed optimum (`round(optimum, 1) * 1.05`, giving 519.5) or print both numbers in the chip.
- **D15.** Show the box edges in the probe chip (or 0.675 m with three decimals), so no tie rounding remains.
- **D16.** Rule: integers on plane tags and bars, one decimal on scoreboards, tiles and cards; overhang always two decimals in chips and one on axes; cost from the unrounded value (already so, label clarity only).
- **D17.** Remove the "1 m" dimension from the section (or label the exterior zone "außen"), and add a dimension arrow from the glazing plane to the slab tip labelled "d 0.60 m".
- **D18.** Replace the run caption with "One simulation sums a year of heat, sun and daylight; two solstice days are shown." (DE: "Eine Simulation summiert ein Jahr aus Wärme, Sonne und Tageslicht; gezeigt sind zwei Sonnwendtage.")
- **D19.** Change the gaps_3 caption to "The best draw sits in the crowded pair: nine draws do not guarantee coverage." and keep the chip.
- **D20.** In `gaps_2`, show the two values beside their rings ("568" and "562 kWh/a") and, if space allows, the second room at 0.15 scale.
- **D21.** limits_4 chip: "Latin hypercube: best of nine 495.9 kWh/a · Δ 1.1 kWh/a". limits_2 first card: "dense-grid optimum 494.8 kWh/a".
- **D22.** Pair rings: radius 0.25 → 0.18 and one bracket with one chip per pair. reveal_2: ring radius 0.2 → 0.14 and offset the LHS ring from the star by one marker width; or drop the LHS ring at this moment.
- **D23.** Start the x-axis at 0 % (range 0 to 85) or shift the 10 % column +0.12 units; the ghosts then stop at the axis.
- **D24.** Add the chip "one tick per sample" beside the strata chip. Lower the marginals_2 strip z-index below the ghosts (z −2) and dim the ghost with a ring instead of the strip.
- **D25.** Fill the ≤ +5 % outline (`fill_opacity` 0.12, GOOD) and move the "500 kWh/a" label to a free contour spot by passing the outline points to `_contour_labels` as `avoid`.
- **D26.** Draw Δ bars only when gap ≥ 3 kWh/a, or add one scale "0 – 70 kWh/a" with ticks under the tiles.
- **D27.** Units ("kWh/a") in INK at 15 px in the ramp legend and in point tags.
- **D28.** Narration `run_2` DE: "Beleuchtungsstrom" → "Beleuchtung" (bar label already uses it).
- **D29.** Extend the layout checker to ring-versus-marker, label-versus-tile and ghost-versus-tile intersections; until then the "none" reports carry no weight.

## 5. Status of the earlier audit (`T/audit-sampling.md`)

Fixed in the current frames: orientation glyphs now rotate correctly (growth, growth_2, growth_3, EN and DE); the letter formula is gone; overhang and insulation thumbnails are coloured at 0.15 scale; the "8 m" label no longer collides; parameters_2 room is coloured; run_3 point tag has a unit; the budget pip row is clear of the chip; grid_3 sun disc is clear; gaps_3 ring now has a chip; the Latin hypercube screens no longer keep the random room and bar (not-simulated state shown); cells has a label on the box; reveal contour labels avoid markers; reveal caption no longer says "true surface"; reveal_2 dotted line removed; compare tiles have no dimension overprint; Δ values now match (4.4 / 67.0 / 1.1); the best-of-nine card has a value (but see D04); Latin-Hypercube naming is consistent; run_2 DE uses "Kühlstrom"; range dashes are en dashes.

Still open: the daylight claim (D01), "its room shows why" (D08), the random table (D07), the gaps caption (D06), gaps_2 ring overlap (D22), tick legend (D24), cells numbers (D13), the evaluate_2 total (D02, now the parts), the reveal_2 cluster (D22), the ≤ 5 % region (D25), compare_2 slivers (D26), 10 % column on the axis (D23), the layout checker (D29).

Not re-verified in this round: DE `limits_2` and `limits_3` (the earlier audit reported the DE closing chip ends at about x 1270 of 1280); DE `parameters_2`, `budget`, `marginals`, `cells`, `reveal` and `compare` were not viewed here. The English set was viewed in full.

## 6. Depth against the energy references

Energy frames viewed (6): `Beat5_Verschattung-raffstore` (building section, slats, a live F_sh meter with labelled ends and DIN references), `Beat1_LeistungUndEnergie-jahr` (power over a year, units on both axes), `Beat3_IsolateAirflow-compare` (bars with units, ratio chip with arithmetic), `Beat8_Hauptgleichung-chain` (coloured formula, substituted values, result against a reference bar), `Beat1_LastZuerstSenken-natural` (building section with numbered levers and a cooling-load bar), `Beat1_VerlustZuGewinn-gain` (building section with rays on roof, walls and window, Φ_solar label). Sources, for the coordinator to open: `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`, `…/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`, `…/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` (found by grep for "ganz verschattet", "Verschattungsfaktor", "Hauptgleichung der solaren Gewinne").

Where the sampling clip already matches: live room sections with sun altitudes (19° and 66°, verified), rays that change with the overhang, stacked bars with units, the colour ramp shared by tags, tiles and contours, and the compare tiles. Where it falls short: no substituted equation (D10), no meter with physical ends (D11), no daylight (D01), a screen density of 6 to 10 numbers where the energy stills carry 1 to 3, and no time or annual structure beyond two solstice days (D18).

## 7. Verification

Script: `T/check-r2-sampling.py` (imports `🧮️model/🐍️.py` by path, stdlib only). Run: `cd /c/git/semio && "C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe" "<T>/check-r2-sampling.py"`. Result: 145 checks, 4 mismatches. Mismatches: evaluate_2 parts (D02), limits_4 parts (D02), threshold 519.5 vs 519.6 (D14), probe centre d tie (D15). All other numbers reproduce: sun altitudes 19 and 66, tariff 0.25, baseline bar 58 / 245 / 219 = 522 and cost 131, grid tags 808 to 499, growth 108 / 99 / 12, random table (all nine WWR and d values from the recorded draws), random_3 bar 707, gaps box 35 to 70 % by 0.15 to 1.20 m, closest pair #3/#4 with Δ 2 pp, 0.06 m, 6 kWh/a, gaps_2 bar 562, LHS 7.8 pp and 0.13 m, marginal strips 41.1 to 48.9 % and 0.27 to 0.40 m, reveal 41 × 41 and optimum 494.8 at 66 % / 0.99 m, ramp 495 to 846, compare Δ 4.4 / 67.0 / 1.1 and percentages, limits_2 optimum value. Narrative claims checked: no draw inside the empty box (0); no draw with WWR ≥ 55 % and d ≥ 0.8 m (0); Latin hypercube best (495.9) below random best (561.8); every set's best above the optimum.

Frames checked: English 33 of 33 (the keyframes in `🗑️generated/frames/Beat2_GridRandomSampling/`); German 11 (listed in the header). Layout reports `…-en-layout.md` and `…-de-layout.md` both say "none" for all keys and are not trusted (D29).
