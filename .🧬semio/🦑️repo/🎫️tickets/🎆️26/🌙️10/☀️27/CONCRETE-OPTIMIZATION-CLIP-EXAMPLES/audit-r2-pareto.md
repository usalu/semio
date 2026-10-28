# Audit r2: Beat5_ParetoFront (Pareto Optimality)

Scope: all 32 English keyframes in narration order (`T/🗑️generated/frames/Beat5_ParetoFront/Beat5_ParetoFront-en-<key>.png`), 10 German keyframes (`de-decision_2`, `room`, `payback`, `third_2`, `walk_2`, `approximate`, `dominated_2`, `knee_3`, `weights`, `rule_2`), 6 energy stills (`T/🗑️generated/energy-frames/`), 12 magnified crops (`T/🗑️generated/r2/crops/`), the number recheck `T/check-r2-pareto.py` (log `T/🗑️generated/r2/check-r2-pareto.log`), and the scene `C/🏆️pareto/🐍️.py` with the model `C/🧮️model/🐍️.py`. No source file was edited.

## 1. Verdict

The clip is about two-thirds of the way to the bar: the tile, the room section, the cost chip, the stacked bar and the cards now show one design at a time, and 86 of 95 recomputed screen values match the model to rounding (grid, front, dominated pair, extremes, refinement and picks are correct). It still falls short in three places: the daylight objective and the payback and weight encodings remain abstract (daylight barely changes the room, payback and the iso-score line have no drawn physical meaning or shown arithmetic), the central knee sentence ("after it, almost none") and the "each step saves less" and "a deeper overhang dominates" claims contradict the model, and eight checks fail: four colour-ramp end labels that no plotted tile reaches, two bar sums that give 723 and 775 instead of 724 and 776, one rounded delta, and the "each step saves less" claim. The depth is comparable to the energy clips in visual density but not in derivation: no frame shows a substituted formula or a physical scale of the kind the energy stills use, and every layout report says "none" although at least ten collisions exist.

## 2. Per-beat table

"Defects" refers to the IDs in section 3.

| key | what is on screen | what the narration says | defects |
| --- | --- | --- | --- |
| room | Section at WWR 40 %, d 0.6 m (sun 66°, rays stopped by the slab), t 12 cm; dimensions "1 m" and "6 m"; no plot, no bar | Which window share, insulation and overhang suit this seminar room's glazed façade? | D32 |
| room_2 | Same room; chip "Façade cost 19,829 CHF"; bar 533 kWh/a (241 / 219 / 73); intensity "11 kWh/[m²·a]" | A simplified annual teaching model gives every façade its cost and its electricity. | ok (verified) |
| objectives | Empty axes (19,000–22,000 CHF; 500–900 kWh/a), "better" arrow to the lower left; room and bar | We want both small: cost to the left, annual electricity at the bottom. | ok |
| objectives_2 | One ring at 19,829 CHF / 533 kWh/a; chip "19,829 CHF · 533 kWh/a" sits on the y-axis | Every façade becomes one point: cost along the horizontal, electricity up. | D20 |
| grid | 36 tiles; orange ring at 21,576 / 775 (WWR 80 %, d 0, t 20); chip "6 WWR × 3 t × 2 d = 36 designs"; bar 775 (112 / 631 / 32) | We simulate every combination of window share, insulation and overhang. | D14, D16, D26 |
| grid_2 | Ring at 19,402 / 584 (WWR 30 %, d 0.6, t 12); room and bar 584 (298 / 188 / 98) | Point at any tile: room, cost and electricity follow. | ok (verified) |
| extremes | Cards: cheapest 19,210 CHF / 649 kWh/a, most frugal 21,552 CHF / 520 kWh/a with thumbnails; gap "+2,342 CHF", "−129 kWh/a"; room WWR 50 %, d 0.6, t 30; bar 520 (200 / 275 / 45) | The cheapest and the most frugal façade lie far apart: none is both. | D11 (gap has no arithmetic); numbers ok |
| dominated | Room WWR 70 %, d 0, t 30 (21,696 / 724); compare thumbnails WWR 70 → 30; bar "724" with segments 121 / 568 / 34 | Compare two façades: this one has a larger window share and more insulation. | D04 |
| dominated_2 | Quadrant 19,000–21,696 CHF × 450–724 kWh/a; winner ring 19,210 / 649; legs "−2,486 CHF", "−75 kWh/a" (inside the box, among tiles); room = winner, bar 649 (241 / 310 / 98) | That one is cheaper and uses less electricity. | D25 |
| dominated_3 | Red cross on the 21,696 / 724 tile; room, chip and bar show 19,210 CHF, WWR 30 % | So the first is dominated: nobody would pick it. | D13 |
| rule | Check rows "cost 19,210 ≤ 21,696 CHF", "electricity 649 ≤ 724 kWh/a" with winner room | Dominance: no worse in cost and no worse in electricity. | ok |
| rule_2 | Third row "strictly less: −2,486 CHF · −75 kWh/a" (ends 10 px from the frame edge) | And strictly better in at least one of them. | D14, D24 |
| front | Orange sweep line at the plot's right edge; green staircase over the 8 front tiles; grey = dominated; 3-line key; room 21,552 / 520 | Sweep from cheap to expensive: keep only façades whose electricity beats all cheaper ones. | D27 |
| front_2 | 8 front tiles highlighted; ramp "Tile colour: electricity" labelled "495 kWh/a" … "846 kWh/a"; caption "Pareto front of the 36 simulated designs" | What remains is the Pareto front of the simulated designs: no gain without extra cost. | D03 |
| approximate | Ghost ring "not in the grid · 21,260 CHF · 495 kWh/a · beats 2 front façades"; 11 px sub-label; room WWR 66 %, d 1.0 m, t 16 (bar 495 = 182 / 273 / 40); pair card vs WWR 50 / d 0.6 / t 30 (21,552 / 520); rows "21,260 < 21,552 CHF", "495 < 520 kWh/a", "−292 CHF · −25 kWh/a" | Off the grid, a deeper overhang dominates a front member: the search clip finds such designs. | D06, D19, D22, D23 |
| sets | Labels "OBJECTIVE SPACE: cost · electricity" on the plot, "DESIGN SPACE: window share · insulation · overhang" under the room, arrow "simulation" from room to plot | The room is design space, the plot is objective space; simulation maps one onto the other. | D30 |
| walk | Step card "Step 1 of 7": WWR 30 %, d 0.0 → 0.6 m (19,210 / 649 → 19,402 / 584); dashed slab outline; chip "d 0.0 → 0.6 m" | Walk along the front, starting at the cheapest façade. | D22 |
| walk_2 | Leaders "+192 CHF" (orange) and "−65 kWh/a" (white); bar legend "Lighting 298 kWh/a +57", "Cooling 188 kWh/a −122", "Heating 98 kWh/a"; chips "Extra cost 192 CHF", "Saved 65 kWh/a" | Step one: a 0.6 m overhang costs 192 CHF more and saves 65 kWh/a. | D18, D21, D22, D35 |
| walk_3 | Chips "Extra cost 192 CHF", "Saved 65 kWh/a", "Price 3.0 CHF per kWh/a saved" | Divide the two: 3.0 CHF for every kWh/a saved. | D07, D11 |
| payback | Bar "12" for step 1; axis "Payback [years]" with x = front step 1 … 7 and no y scale; chip "Tariff 0.25 CHF/kWh" | At 0.25 CHF per kWh the savings repay this step in 12 years. | D07 (numbers ok: 11.8 → 12) |
| payback_2 | Dashed "lifetime 30 a" line; bar 12 | A façade lasts about 30 years: only steps that pay back within that lifetime are worth it. | D07 |
| knee | Step 7 of 7: WWR 40 → 50 %, 21,384 → 21,552 CHF, 520 → 520 kWh/a; room deltas "−41 / +51 / −10"; bars 12, 34, then ">90" five times | Walk on: each step saves less, until payback exceeds the lifetime. | D05, D17, D33 |
| knee_2 | Step 1 of 7; ring 19,402 / 584 labelled "knee"; bars 12 (green) and 34 (red) | Here is the knee: before it, money buys much saving; after it, almost none. | D01 |
| knee_3 | Tariff chip "0.40 CHF/kWh"; step 2 of 7 (19,402 → 19,829); bars 7 and 21; ring on 19,829 / 533; room deltas −57 / +31 / −25 | A higher tariff moves the knee: same front, different best choice. | D07 (numbers ok: 7.4 → 7, 21.0 → 21) |
| third | Daylight ramp "1 %" … "7 %"; chips "Lamps on 37 % of the time", "Daylight factor 2.4 %"; room WWR 30 %, d 0, t 12; bar 649 | Add daylight as a third objective: tile colour shows the daylight factor in %. | D02, D03 |
| third_2 | Loser ring WWR 80 % (21,346 / 776 / 6.4 %); thumbnail of 19,210 / 649 / 2.4 %; rows "cost 21,346 > 19,210 CHF" (cross), "electricity 776 > 649 kWh/a" (cross), "daylight factor 6.4 > 2.4 %" (tick); bar "776" with 112 / 629 / 34 | In 2D it loses: a dimmer room is cheaper and needs less electricity. | D02, D04, D15 |
| third_3 | As third_2, plus chip "Front 8 → 30 · 2 → 3 objectives" and green rings on 30 tiles | With daylight counted, nobody beats it on all three objectives. | D15, D37 (numbers ok: 8 and 30) |
| third_4 | Stage pixel-identical to third_3; only the caption band differs (pixel difference confined to y 638–685) | A two-dimensional picture can hide a trade-off: check every objective. | D09 |
| weights | Slider "Cost 50 % · Electricity 50 %"; orange iso-line "orange line: equal weighted score"; pick chip "Pick 19,829 CHF · 533 kWh/a"; ramp 495 … 846 | To pick one façade, weigh the objectives, as in the index video. | D03, D08 |
| weights_2 | Slider at 2 % / 98 %; horizontal iso-line through the 21,552 / 520 tile; pick chip; room WWR 50 %, d 0.6, t 30 | The winner always sits on the front and slides along it with the weights. | D03, D08 (pick ok) |
| decision | Three cards: Cheapest 19,210 / 649 (orange ring), "Knee at 0.25 CHF/kWh" 19,402 / 584 with "lifetime 30 a" (green ring), "Balanced weights" 19,829 / 533 (white ring); thumbnails without labels; long leaders | The front removes dominated designs; it does not choose: tariff, lifetime and weights do. | D28, D29 |
| decision_2 | Room WWR 30 %, d 0.6, t 12; ring 19,402 / 584 on the plot; bar 584 (298 / 188 / 98) | For this room, the knee: 30 % window share, 0.6 m overhang, 12 cm insulation at 0.25 CHF/kWh. | D11 (payback 12 a and lifetime not on this frame) |

## 3. Defect list

Severity: blocker (breaks the owner's rule or a central claim), major, minor. Tags: [abstract], [numbers], [layout], [sync], [depth], [de].

| ID | tag | severity | keys | defect |
| --- | --- | --- | --- | --- |
| D01 | [sync][numbers] | blocker | knee_2 (en, de) | "after it, almost none" is false. The step after the knee saves 50.9 kWh/a, 78 % of step 1 (64.9 kWh/a), for 427 CHF. Saving drops to 5.5 kWh/a only from step 3 on. |
| D02 | [abstract] | blocker | third, third_2, third_3, third_4 (de equal) | Daylight is shown only as tile colour, two chips and a faint room gradient. Measured on the keyframes: near-window room luminance 92 → 104 (of 255) between 2.4 % and 6.4 %; lamp zone 144 → 149, i.e. the lamps do not dim. No physical meaning of "2.4 %" is drawn in the room. |
| D03 | [numbers] | major | front_2, weights, weights_2 (electricity ramp); third, third_2, third_3, third_4 (daylight ramp); all de equivalents | Ramp end labels do not match the plotted designs. The 36 tiles span 519.8 to 775.9 kWh/a and 1.8 to 6.4 %, but the ramps say 495 to 846 kWh/a (the 41 × 41 surface range) and 1 to 7 %. No tile reaches either end. |
| D04 | [numbers] | major | dominated (loser), third_2, third_3, third_4 (loser) | Bar labels do not add up: loser 121 + 568 + 34 = 723 against total 724 (exact 723.7); 112 + 629 + 34 = 775 against 776 (exact 775.9). |
| D05 | [sync][numbers] | major | knee (de: Weiter … spart weniger) | "each step saves less" is false on the model: step 6 saves 2.29 kWh/a after step 5's 2.00. The statement that does hold is "each step costs more per kWh/a saved": 2.96, 8.40, 77.2, 83.7, 156.1, 240.9, 669.9 CHF per kWh/a (strictly rising). |
| D06 | [sync] | major | approximate (en, de) | The causal claim is wrong. Only the overhang change (0.6 → 0.99 m) costs +125 CHF and saves 5 kWh/a, so it dominates nothing by itself. The ghost dominates through a path: deeper overhang (+125 CHF, −5.4 kWh/a) → larger window 66 % (+269 CHF, −21.4 kWh/a) → thinner insulation 16 cm (−685 CHF, +1.8 kWh/a). Net −292 CHF, −25 kWh/a. |
| D07 | [abstract][depth] | major | payback, payback_2, walk_3, knee, knee_2, knee_3 (de equal) | The payback bars have no y scale, the lifetime line has no physical unit on screen, and the division "3.0 CHF per kWh/a ÷ 0.25 CHF/kWh = 12 a" is never shown. The 12 years are not visible as savings. |
| D08 | [abstract] | major | weights, weights_2 (de equal) | The iso-line is a min–max normalised score (0 best … 1 worst), with no physical unit. The slider "50 % / 50 %" does not tell the viewer what is traded. At 50/50 the line means 18.2 CHF per kWh/a (range 2,342 CHF over 129 kWh/a), which is never shown. |
| D09 | [sync] | major | third_4 | Stage pixel-identical to third_3; only the caption differs. "check every objective" has no per-row indication (audit item from before, still open). |
| D10 | [layout][de] | major | de payback, de knee_3 | Title "Amortisation [Jahre]" (x≈923–1087, y≈440–456) touches the chip "Tarif 0,25 / 0,40 CHF/kWh" (x≈1083–1266, y≈410–441). "Lebensdauer" in the lifetime label ends at x≈1277, 3 px from the frame edge. |
| D11 | [depth] | major | all derived-number frames (extremes, walk_3, payback, third_2, approximate, decision_2) | No substituted arithmetic and no physical scale anywhere. The energy stills show "400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W", "0,126 / 0,061 ≈ 2,1" and a 0 … 1 gauge with words ("ganz verschattet" / "ohne Verschattung"). |
| D12 | [layout] | major | all 32 en and 10 de frames | Both layout reports (`…-en-layout.md`, `…-de-layout.md`) say "none" for every key. The collisions in D10, D20, D21, D22, D23 and D24 are not detected. |
| D13 | [sync] | minor | dominated_3 | "So the first is dominated" but the room, the chip and the bar show the dominator (19,210 CHF, WWR 30 %). The loser with the red cross (21,696 CHF, WWR 70 %) is not in the room. |
| D14 | [sync] | minor | rule_2 (de equal) | "strictly better in at least one" is shown as "strictly less" in both criteria. The "at least one" case is not demonstrated (no tie exists in the 36 designs). |
| D15 | [sync] | minor | third_2, third_3, third_4 (de equal) | Check rows use ">" for cost and electricity (loser worse) and for daylight (loser better). The same symbol means opposite things. |
| D16 | [sync] | minor | grid (de equal) | "every combination of window share, insulation and overhang" is wrong: the grid is 6 × 3 × 2 = 36 level combinations. |
| D17 | [numbers] | minor | knee (step 7), knee (de) | Cooling delta "+51" is the difference of rounded labels (275 − 224). The exact change is +50.45 kWh/a, which rounds to +50. |
| D18 | [numbers] | minor | walk_2 (de equal) | Bar-legend deltas "+57", "−122" have no unit and sit beside "298 kWh/a", so they read as part of the total. |
| D19 | [numbers] | minor | approximate | Ghost design shown as "d 1.0 m"; the refinement design is 0.99 m (surface optimum). |
| D20 | [layout] | minor | objectives_2 | Chip "19,829 CHF · 533 kWh/a" (x≈86–322, y≈350–384) touches the y-axis line at x≈87. |
| D21 | [layout] | minor | walk_2 | Leaders do not span the two tiles: the orange "+192 CHF" line ends on the top-right corner of the 19,210 tile; the white "−65 kWh/a" line ends at the upper edge of the 19,402 tile. The step itself is not drawn. |
| D22 | [layout] | minor | approximate (de equal), third_2, third_3, third_4, walk, walk_2 | Sun rays cross dimension chips: "d 1.0 m" (x≈640–712, y≈157–187), "d 0.0 m" (x≈636–716, y≈156–186). Rays run through the chip borders. |
| D23 | [layout] | minor | approximate (en, de) | Sub-label "= optimum a search finds [algorithms clip]" (x≈150–395, y≈236–250) is about 11 px grey; de "= Optimum der Suche [Algorithmen-Video]" likewise. Unreadable at this size. |
| D24 | [layout][de] | minor | rule_2 (en, de) | Check row ends at x≈1270, 10 px from the frame edge. |
| D25 | [layout] | minor | dominated_2 (de equal) | The quadrant has no label (what its designs are). "−75 kWh/a" (x≈153–256, y≈292–312) sits inside the box among tiles. Pair-card grey text (≈14 px) is low contrast. |
| D26 | [layout] | minor | grid, front, front_2, weights | Tiles overlap in clusters: 21,200–21,600 CHF × 520–560 kWh/a (4–5 tiles), 20,000–20,300 CHF × 620–660 kWh/a. Front and grid tiles cannot be told apart. |
| D27 | [layout] | minor | front | The orange sweep line at x≈527 touches the chip "6 WWR × 3 t × 2 d = 36 designs" (right edge x≈523). |
| D28 | [layout] | minor | decision | Leaders from the three cards cross most of the plot (the "Cheapest" leader runs from x≈565 to the ring at x≈115). |
| D29 | [abstract] | minor | decision | Thumbnails carry no design labels (WWR, d, t), so the three options are not named in physical terms. |
| D30 | [layout] | minor | sets | "DESIGN SPACE" line floats under the room (x≈693–1090, y≈457), not attached to glass, wall or overhang; the "simulation" arrow ends at the plot's x-axis labels. |
| D31 | [sync] | minor | room family, all frames with the room | "Sun 66°" is a fixed summer label while heating (winter) segments are shown (open since the earlier audit). |
| D32 | [layout] | minor | room, room_2 (de equal) | The "1 m" dimension under the ground line has no stated meaning. |
| D33 | [layout] | minor | knee, knee_2 | Five repeated "> 90" labels above bars 3–7; the "knee" label overlaps grey tile outlines (x≈145–190, y≈315–340). |
| D34 | [de] | minor | walk_2 (de) | Chip "Eingespart 65 kWh/a" does not match "Mehrkosten 192 CHF" (noun vs adjective form). Use "Ersparnis 65 kWh/a". |
| D35 | [de] | minor | walk_2 (de), all de captions | The caption wraps between "65" and "kWh/a." (de-walk_2, line 2). Numbers and units need a non-breaking space (U+00A0). |
| D36 | [de] | none (checked) | decision_2, room, third_2 (de) | Numbers are formatted correctly (19.210, 0,25, 3,0, 6,4 %, 0,6 m) and the English "vs" is gone. No untranslated strings found in the 10 de frames checked. |
| D37 | [layout] | minor | third_3 | Chip "Front 8 → 30 · 2 → 3 objectives" is cryptic and the green rings have no legend. |

## 4. Fix proposals

- **D01** Rewrite knee_2. EN: "Here is the knee: the next step would take 34 years to pay back, longer than the façade lasts." DE: "Hier ist das Knie: Der nächste Schritt amortisiert sich erst in 34 Jahren, länger als die Fassade hält." Bind 34 to `pareto()["steps"][1]["payback"]` (33.6) and 30 to `service_life`; assert both in `check-pareto-narration.py`.
- **D02** In `_pareto_daylight` draw the visible sky fan from the window centre. Its angle is `sky_angle × (1 − 0.5 · d² / (d² + 0.6²))`: 75° at d 0, 56.3° at d 0.6 m, 45° at d 1.2 m. Show the daylight factor as the substituted model formula, for example "DF = 0.6 · 7.2 m² · 75 / (180 m² · 0.75) = 2.4 %" for WWR 30 %, d 0 (the same formula gives 6.4 % at WWR 80 %, d 0, and 1.8 % at WWR 30 %, d 0.6 m). Set lamp-glyph opacity to 1 − autonomy (lamps on 37 % at 2.4 %, 17 % at 6.4 %) and tint the floor zone with the same value. Verify each number in the check script.
- **D03** Bind both ramps to the min and max of the 36 plotted designs, then print their endpoints from the same values: "520 kWh/a" … "776 kWh/a" and "1.8 %" … "6.4 %" (de "1,8 %", "6,4 %"). Let the tile colours use the same range. Add the two ends to `check-r2-pareto.py`.
- **D04** Show bar labels with one decimal in all frames: 121.1 + 568.3 + 34.3 = 723.7 and 112.5 + 629.4 + 34.0 = 775.9. Parts then add to the total within 0.1.
- **D05** Replace "each step saves less" by "each step costs more per kWh/a saved" (DE: "jeder Schritt kostet mehr je eingesparter kWh/a"). Print the ratio on the payback bars or in a chip row ("3.0 → 8.4 → 77 → 84 → 156 → 241 → 670 CHF per kWh/a"). Add a strict-monotonicity check of `ratio` to the script.
- **D06** Replace the clause by "Off the grid, a bigger window shaded by a deeper overhang, paid for by thinner insulation, dominates a front member." (DE: "Abseits des Rasters dominiert ein größeres Fenster mit tieferem Vordach und dünnerer Dämmung ein Frontmitglied.") On screen, add a three-step strip under the pair card: "+ Vordach 0.99 m: +124.8 CHF, −5.4 kWh/a → + Fenster 66 %: +268.8 CHF, −21.4 kWh/a → − Dämmung 16 cm: −685.4 CHF, +1.8 kWh/a = −291.8 CHF, −25.0 kWh/a" (shown rounded as −292 CHF, −25 kWh/a; integers would not add up). All values come from `evaluate()` along that path; the check script asserts the path ends at the refinement.
- **D07** Add a worked line for walk_3 and payback: "192 CHF ÷ 65 kWh/a = 3.0 CHF per kWh/a" then "3.0 CHF per kWh/a ÷ 0.25 CHF/kWh = 12 a". Add a 30-cell lifetime strip: one cell is one year of savings, 65 kWh/a × 0.25 CHF/kWh = 16.2 CHF/a, so 12 cells fill to 192 CHF. Give the payback chart a y scale in years (0, 10, 20, 30) with the lifetime line at 30.
- **D08** Replace the slider's abstract percentages by the trade it implies: at the pick, "1 kWh/a is worth 18.2 CHF" at 50/50, computed as range_cost / range_electricity over the front (2,342 CHF / 129 kWh/a). Draw the iso-line in those physical units and state the range basis once ("ranges of the front"). Keep the slider labels as they are.
- **D09** In third_4, indicate the three rows in turn (cost ✗ pulse, then electricity ✗, then daylight ✓), with the caption kept. Use plain words instead of symbols if fonts lack the glyphs.
- **D10** Move the "Tarif …" chip up by about 0.35 units so it clears the title, or move the title left; keep the lifetime label's right edge at x ≤ 1240 (≥ 40 px margin) by placing it left of the dashed line's end.
- **D11** Add one equation chip per derived number, with substituted model values, in the energy style: walk_3 and payback (as D07), extremes gap ("21,552 − 19,210 = 2,342 CHF; 520 − 649 = −129 kWh/a"), third_2 ("6.4 − 2.4 = 4.0 %"), approximate (the path strip from D06), decision_2 ("12 a ≤ 30 a").
- **D12** Extend the layout check: text-box vs axis/tick/line overlap, rays vs labels, leaders vs tiles, and a 12 px minimum from the frame edge. Regenerate both layout reports and run it on de and en. The check must fail on the current frames (D10, D20, D22, D24).
- **D13** In dominated_3 move room, chip and bar to the loser (21,696 CHF, WWR 70 %, t 30 cm, 724 kWh/a) and keep the winner as the reference thumbnail with its numbers.
- **D14** Replace the caption by "And strictly better in at least one of them; here, in both." (DE: "Und in mindestens einer echt besser; hier in beiden.") so the screen matches the words.
- **D15** Write the rows as differences in words: "+2,136 CHF — higher cost" (✗), "+127 kWh/a — more electricity" (✗), "+4.0 % — more daylight" (✓). No ">" for two opposite meanings.
- **D16** Caption: "We simulate every combination of six window shares, three insulation thicknesses and two overhangs." (DE: "Wir simulieren jede Kombination aus sechs Fensteranteilen, drei Dämmstärken und zwei Vordächern.")
- **D17** Compute deltas from unrounded values and show one decimal for values below 100 (for example +50.4 or +50.5 for cooling, with lighting −41.0 and heating −9.7); the three deltas must sum to the displayed total change (−0.3 kWh/a, shown as 0).
- **D18** Add a column header "Δ kWh/a" above the delta column and write "+57 kWh/a", "−122 kWh/a".
- **D19** Show the refinement as "d 0.99 m" (two decimals only for this design), or "d ≈ 1.0 m (0.99)".
- **D20** Move the chip right by about 0.15 units (about 20 px) so its left edge clears the axis by at least 15 px, or place it above the dot.
- **D21** Draw the step as one bracket between the two tiles: "+192 CHF" as a horizontal dimension under the segment and "−65 kWh/a" as a vertical one beside it; leaders end at the tile centres.
- **D22** Draw dimension chips above the sun-ray bundle (z-order) with an opaque fill, or start the ray bundle outside the chip box. Do not place a chip inside the ray band.
- **D23** Set the sub-label at 16 px on its own line (or move it to the caption): "= optimum a search finds" and "(algorithms clip)" on two lines; DE the same.
- **D24** Split the check row into two lines ("strictly less: −2,486 CHF" / "· −75 kWh/a") or start the rows 40 px further left; both languages.
- **D25** Label the quadrant with its meaning (EN: "designs that could dominate the loser"; DE: "Entwürfe, die den Verlierer dominieren könnten"). Move "−75 kWh/a" beside the vertical leg, outside the box. Set the grey pair-card text to 16 px with at least 4.5:1 contrast.
- **D26** Shrink grid tiles to about 0.052 units, or add short leaders for tiles that overlap by more than 30 %. Keep the front tiles on top.
- **D27** End the sweep line at x ≤ 520 or move the chip 0.1 units left.
- **D28** Route card leaders as elbows: horizontal from each card to the plot's top margin, then to the tile. Or place the cards closer to their tiles.
- **D29** Put a 16 px line under each thumbnail with its design: cheapest "30 % · 0 m · 12 cm" (19,210 CHF), knee "30 % · 0.6 m · 12 cm" (19,402 CHF), balanced "40 % · 0.6 m · 12 cm" (19,829 CHF).
- **D30** Label the room parts (glass 30 %, wall 12 cm, overhang 0.6 m) and keep "design space" as one caption above the room. End the arrow at the plot's bottom border.
- **D31** Label the sun "Summer sun 66° at noon" (DE: "Sommersonne 66° mittags") in every frame that shows heating.
- **D32** Say what the 1 m measures or remove the dimension.
- **D33** Merge the five "> 90" labels into one bracket above bars 3–7 ("> 90 a"). Move the "knee" label into clear space above-left with a 0.2-unit leader.
- **D34** DE: "Ersparnis 65 kWh/a" to match "Mehrkosten 192 CHF".
- **D35** Use U+00A0 between every number and its unit in captions and chips (both languages).
- **D36** No change needed; keep the number format.
- **D37** Chip "2D front 8 → 3D front 30 of 36", and a legend line "green ring = front in all three objectives" beside it.

## 5. Status of the earlier audit (`audit-pareto.md`, 12:24)

| earlier finding | status now |
| --- | --- |
| third / third_2 daylight never reaches the room (high) | Open (D02). Chips and a gradient exist; lamps and the room barely change. |
| walk_2 step consequence not shown (high) | Fixed: delta chip, dashed outline, bar deltas. Leader anchor still ambiguous (D21). |
| third_2 (de) English "vs" (high) | Fixed. |
| third_2 dominator not on screen (med) | Fixed: thumbnail with numbers. |
| third_3 / third_4 identical (med) | Open (D09). |
| dimension and chart labels collide (med) | Mostly fixed in en; new collisions in de (D10). Layout report still says "none" (D12). |
| rule_2 "at least one" not demonstrated (med) | Open (D14). |
| payback x-axis unlabelled (med) | Fixed ("front step"). |
| colour-ramp legends without units (med) | Units fixed; values wrong (D03). |
| decision card hard-codes the tariff (med) | Lifetime chip added; the tariff text is correct, but the source was not re-read to confirm it is built from the model. |
| approximate shows only the cost chip (med) | Fixed (readout, "beats 2", pair card); attribution wrong (D06). |
| extremes shows only the frugal room (med) | Fixed (two cards and gap). |
| grid cursor hidden under tile (med) | Fixed (ring visible). |
| grid tile clutter (med) | Open (D26). |
| bar shows only the total (med) | Mostly fixed; segment sums off (D04). |
| "Sun 66°" fixed summer sun (low) | Open (D31). |
| room_2 "Cost" without noun (low) | Fixed ("Façade cost"). |
| objectives_2 chip on the axis (low) | Open (D20). |
| grid caption levels not shown (low) | Fixed (chip). |
| grid_2 pointed tile without ring (low) | Fixed. |
| dominated_2 "−75 kWh/a" among tiles (low) | Open (D25). |
| walk_3 chip "Price 3.0 CHF/(kWh/a)" (low) | Fixed. |
| knee label overlaps outlines (low) | Open (D33). |
| third_3 chip "Front 8 → 30" unexplained (low) | Open (D37). |
| weights iso-line unlabelled (low) | Fixed label; still abstract (D08). |
| sets DESIGN SPACE floats (low) | Open (D30). |
| decision leaders cross the plot (low) | Open (D28). |
| front staircase unlabelled (low) | Fixed ("green steps"). |
| walk insulation layer barely visible (low) | Not re-checked: the walk frames change only the overhang. |

## 6. Recomputed values

`check-r2-pareto.py` ran with `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`; log `T/🗑️generated/r2/check-r2-pareto.log`. Result: 86 checks ok and 9 flagged lines = 8 distinct mismatches: four colour-ramp ends (D03), two bar sums (D04), the cooling delta (D17) and the "saves less" narration (D05). The knee sentence (D01) and the overhang attribution (D06) are checked by the path probe and the table in section 2, not by the script.

Verified on screen (matches the model to rounding): all 8 front members and their count (8 in 2D, 30 in 3D); 36 designs; cost of the grid cards 19,210 / 21,552 / 19,829 / 19,402 / 21,346 / 21,696 / 21,260 / 21,576; electricity 649 / 520 / 533 / 584 / 776 / 724 / 495 / 775; the segment values of every shown bar except the two sums above; "+2,342 CHF" and "−129 kWh/a"; "−2,486 CHF" and "−75 kWh/a"; "192 CHF" and "65 kWh/a"; 3.0 CHF per kWh/a; payback 12 / 34 at 0.25 CHF/kWh and 7 / 21 at 0.40; knee = WWR 30 %, d 0.6 m, t 12 cm at 0.25 and WWR 40 % at 0.40; weighted picks at 0, 0.5 and 0.98; dominated pair; refinement cost 21,260 and electricity 495 (surface minimum 494.8); "−292 CHF · −25 kWh/a"; daylight 2.4 % and 6.4 % with lamps on 37 % and 17 %; sun 66.04° at summer noon; intensity 11 and 16 kWh/(m²·a).

## 7. Depth benchmark (energy clips)

Energy stills checked: `Beat5_Verschattung-raffstore`, `Beat1_LastZuerstSenken-envelope`, `Beat1_LeistungUndEnergie-summe`, `Beat3_IsolateAirflow-compare`, `Beat8_Hauptgleichung-chain`, `Beat1_VerlustZuGewinn-gain`. They have a physical drawing that changes with the physics (raffstore slats, rays, heat flows), a named variable, a substituted equation with the units on each value ("400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W"), a ratio chip ("0,126 / 0,061 ≈ 2,1") and a 0 … 1 gauge with words at both ends. The energy source `⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` builds the same kind of 0 … 1 gauge with words at both ends, and `⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` uses "≈" substitution lines. The Pareto frames are denser in linked objects (room, plot, chips, bars, cards) but show no derivation of their derived numbers (D07, D11), no physical scale for daylight (D02), and no physical meaning for the iso-line (D08).

## 8. Coverage and limits

- All 32 English keyframes were viewed; 10 German ones (listed in the scope line). The remaining German frames were not viewed individually, so German findings beyond the 10 are not claimed.
- Crops for the collision claims: `T/🗑️generated/r2/crops/` (for example `en-approximate-600-140.png`, `en-walk_2-100-150.png`, `de-knee_3-900-420.png`, `en-objectives_2-60-330.png`).
- Brightness probe: `T/🗑️generated/r2/daylight-probe.py` (zone means; the lamp zone includes the white shade, so it is a weak measure of dimming).
- Path probe for D06: `T/🗑️generated/r2/approximate-path.py`. Single-variable probe: `T/🗑️generated/r2/approximate-probe.py`.
- Pixel-difference probe (third_3 vs third_4 differs only in y 638–685): `T/🗑️generated/r2/diff-probe.py`.
- `check-pareto-narration.py` was not re-run. The `repo` and `semio` MCP servers were not connected and were not needed for this read-only audit.
