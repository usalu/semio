# Audit R2: Beat4_NormalizedIndex (topic ⚖️index)

Scope: all 35 English keyframes viewed in narration order; 8 German keyframes (variants, units_2, ruler_2, matrix, limits, range_3, sweep_2, next); 6 energy benchmark frames (AnlagenVerluste-sum, Beat1_LastZuerstSenken-reserve, Beat5_Verschattung-raffstore, Beat8_Hauptgleichung-chain, Beat1_LeistungUndEnergie-summe, Beat3_IsolateAirflow-compare); scene `⚖️index/🐍️.py` (NARRATION, `screen_*` methods, `screen_limits`, `switch_shares`, close-call drawing); model `🧮️model/🐍️.py` and `🔣️.json`. Layout reports: EN no conflicts; DE one (`limits`, line through "nichterlaubt"). Number check: `check-r2-index.py`, output `🗑️generated/audit-r2/check-r2-index.out`. Keyframes show the state at the end of each clause, so transitional content (the carbon ruler and its cloud panel, the "was 1st" tag fade) is audited from the code, not from a frame.

## 1 Verdict

The numbers hold: 160 of 162 model-derived strings reproduce, and the index, the cost-driven re-ranking, the sweep switches at 4 % and 39 %, the six-design range and the Pareto front are all correct, so the chain is a sound teaching sequence. It is still well short of the bar: carbon and daylight values are shown without a drawn physical meaning (blockers), 11 of 35 frames have no room panel at all, and the room drawing is about 180 × 100 px, so most screens are still tables, thumbnails and rulers. Against the energy clips it is shallow (no written arithmetic for the index score, heating or lighting, no cost or carbon build-up, no standards footer), and the one hard constraint (21,000 CHF) is computed from the winner instead of being an authored requirement; this needs rework of the room panel, the cards and the carbon/daylight rulers, not a polish pass.

## 2 Per-beat table

| key | On screen | Narration (EN) | Defects |
| --- | --- | --- | --- |
| variants | Five cards with ~95 px section thumbnails; room chip "Seminar room 8 × 6 × 3 m · 48 m² · Zurich · simplified annual teaching model"; no room panel | Which of five façade designs suits this seminar room best? | D03, D09, D10 |
| variants_2 | Same cards with spec lines (Window 25 %, Overhang 0.0 m, Insulation 16 cm); lower 40 % of each card empty (y ≈ 400–594 px) | Only window share, overhang and insulation differ. | D03, D10 |
| criteria | Values per card: 13.73 kWh/[m²·a] and 19,428 CHF with ↓ arrows; no room | Electricity intensity and cost: lower is better. | D03, D15 |
| criteria_2 | Daylight 2.00 % (↑) and carbon 2,520 kg CO₂e (↓); "%" wraps under the number; "embodied" runs into the arrow; no room, carbon has no physical meaning | Daylight factor: higher is better. Embodied carbon: lower is better. | D01, D03, D20 |
| best | Green frames on 10.44 (Shaded), 19,428 (Punched), 5.60 (Bare), 2,520 (Punched); no room | No design is best everywhere. | D03 |
| units | "13.73 kWh/[m²·a] + 19,428 CHF = ?" with a "no unit" stamp; Punched thumbnail; caption wraps with orphan "unit." | Kilowatt-hours per square metre per year, plus francs? That sum has no unit. | D03, D25 |
| units_2 | Bars of unlike sums 21,964 (Punched) … 26,625 (Shaded); "Sum of the printed numbers"; chip "Units: CHF · kg" | Added anyway, the biggest numbers decide, not the importance. | D03 |
| units_3 | kCHF/t sums 37.7, 38.3, 39.5, 40.4, 44.5; re-ranked (checked against the model); caption orphan "changes." | Count cost in kCHF and carbon in tonnes instead, and the ranking changes. | D25 |
| ruler | Electricity ruler 10–16 with five thumbnails; Shaded room bars Lighting 181, Cooling 281, Heating 39 (sum 501 kWh/a); "501 kWh/a ÷ 48 m² = 10.44 kWh/[m²·a]"; scale label "10 kWh/[m²·a]" under the baseline; "1 m" dimension outside the façade | Give each criterion its own ruler, starting with electricity. | D09, D14, D15, D26 |
| ruler_2 | Shaded ring; room unchanged; DE room labels are verbs (Licht, Kühlen, Heizen); "501 kWh/a" label touches the axis line | Each design sits at its real value; the room shows what the value means. | D25, D27 |
| shift | Chip "- 10.44 kWh/[m²·a]" (hyphen-minus); ticks 0, +2, +4, +6 without unit | Step one: subtract the best of these five values. | D14, D15 |
| shift_2 | Normalized ruler 0.00 … 1.00 with 0.10, 0.64, 0.71, 1.00; chip "÷ 4.67 kWh/[m²·a]"; legend "0 = best of the set · 1 = worst of the set" | Step two: divide by the span from best to worst. | D11 |
| mapped | Bare glass wall room (Lighting 121, Cooling 567, Heating 37; sum 725 kWh/a); "725 kWh/a ÷ 48 m² = 15.11 kWh/[m²·a]"; scale "15 kWh/[m²·a]" under baseline; chip "compared set: 5 variants" | Zero is the best of these five, one the worst of these five. | D04, D14, D15 |
| cost | Cost ruler 0.00 … 1.00 (Ribbon 0.55, Super 0.74, Bare 0.81, Shaded 1.00); Punched elevation 8 × 3 m with four openings and "no overhang" chip; "19,428 CHF"; "façade 24 m² · window 6.0 m²" | Façade cost gets the same ruler: the cheapest façade becomes zero. | D08, D15 |
| daylight | Mirrored ruler (Bare 0.00, Shaded 0.62, Ribbon 0.72, Super and Punched 1.00); Bare room with yellow sun rays entering; "factor 5.60 %"; "Autonomy: lamps off 81 % of lighting hours"; lamps identical in every room | Daylight factor: more is better, so the ruler is mirrored. | D02, D07 |
| matrix | 5 × 4 normalized table with the physical value under each bar; column chips; legend; Punched room with 13.73, 19,428, 2.00, 2,520 as text; carbon cloud is only in the transition | Embodied carbon follows the same rule: its five values now lie between zero and one. | D01, D21 |
| weights | Equal-weights strip (25 % each); column chips above, not centred on their columns | Weights say how much each criterion matters; they add up to 100 %. | D21 |
| weights_2 | Strip 40 / 30 / 20 / 10 %; bars resized to weights; caption two lines | Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %. | D21 |
| weights_3 | Columns widened by weight; green frame on the Electricity column; dashed orange frame on the 10 % Carbon column | A wider column means a stronger pull on the result. | D24 |
| index | Index bars 0.48, 0.40, 0.52, 0.70, 0.65 (weighted segments); axis 0 … 1; Punched room; no score arithmetic | Slide the filled parts together: their length is the index. | D06 |
| index_2 | Ribbon row ① framed with index 0.40; Ribbon room (window 50 %, overhang 0.6 m, 10.92 kWh/[m²·a], 20,544 CHF, 3.00 %, 3,840 kg CO₂e); no score arithmetic | The shortest bar has the lowest index and ranks first. | D06, D09 |
| winner | Ribbon room (section with sun rays stopped by the overhang, lamps, person, "1 m" and "6 m" dimensions); index table | Its room: not best everywhere, but best overall. | D09, D26 |
| winner_2 | Orange ring on the Daylight segment; chip "weakest: Daylight 0.72" | Strengths elsewhere outweigh its weakest criterion. | D18 |
| sensitivity | Weight chips 23 / 60 / 11 / 6 % (checked); cost-driven client coin icon; bars widened; Ribbon room | A cost-driven client puts more weight on cost. | D21 |
| sensitivity_2 | Rows re-sorted: Punched 0.28 ①, Ribbon 0.46 ②, Super 0.72, Bare 0.72, Shaded 0.73; dashed separators; grey bracket marks at x ≈ 32 and 932; orange double arrow; "was 1st" tag next to the room title | The bars change and the ranking reorders. | D24 |
| sensitivity_3 | Punched room (winner at 0.28), cost-driven icon, green frame on the Punched row | Another façade wins, though no design changed. | none material |
| sweep | Cost-weight axis 0–100 %; top strip Shaded 0–4 %, Ribbon 4–39 %, Punched 39–100 %; chips "4 %", "39 %", "cost weight 60 %"; Punched room | Sweep the cost weight to see where the winner switches. | D23 |
| sweep_2 | Close-call bands drawn ≈ 0–11.5 % and 36–44 % (model: 0–10.0 % and 35.9–42.5 %); legend "close call: index gap below 0.03"; "cost weight 5 %"; first top thumbnail unlabelled; "rank" touches the axis origin | Close calls are fragile: test the ranking before you decide. | D12, D23, D29 |
| range | Cost ruler with Premium off the axis at 1.30 (orange dot), chip "1.30 > 1: beyond the old range"; Premium room 22,080 CHF; chips "Window 70 % · 16.8 m²" and "Overhang 1.2 m" abut | The rulers depend on the designs compared: add a sixth façade. | D17, D22 |
| range_2 | Six-design cost ruler: Punched 0.00, Ribbon 0.42, Super 0.57, Bare 0.63, Shaded 0.77, Premium 1.00; orange worst frame on Premium (fixed); ghost dots at old positions; "1" tick and "1.00" label stacked | Best and worst move, so all other normalized values shrink. | D17, D22 |
| range_3 | Six-design board: Ribbon 0.36 ①, Shaded 0.45, Punched 0.48, Premium 0.52, Bare 0.60, Super 0.65; "rank old → new" column; Ribbon frame touches row 2; orange double arrow without label; no room | The order of the original five can flip, though they did not change. | D03, D24 |
| record | Decision card: weights 40 / 30 / 20 / 10 %; ranges 10.44 … 15.11, 19,428 … 21,475 CHF, 5.60 … 2.00 %, 2,520 … 5,136 kg; five thumbnails; result "Ribbon window · Index 0.40"; no room | Document weights, compared designs and value ranges with every decision. | D03 |
| limits | Cost ruler 19,000–22,000 CHF; dashed "limit 21,000 CHF"; Bare and Shaded struck out "not allowed"; board Ribbon 0.40, Punched 0.48, Super 0.70; Ribbon room; chip "20,544 CHF < 21,000 CHF"; DE leader crosses "nicht erlaubt" | Strengths can offset weaknesses; mandatory limits stay constraints. | D05, D13, D19 |
| pointer | Pareto scatter, cost 19,000–22,000 CHF against electricity 8–16; five thumbnails; dashed front Punched → Ribbon → Shaded; chip "Pareto front"; no room; Super and Bare not marked as dominated | To see trade-offs before weighting, look at Pareto fronts. | D03, D16 |
| next | Ribbon room with "Sun 66°", "d 0.6 m", "WWR 50 %", "t 16 cm"; "Cooling demand 816 kWh/a heat ÷ EER 3.0 = 272 kWh/a electricity"; stack Lighting 200, Cooling 272, Heating 52 (= 524 kWh/a); scale "11 kWh/[m²·a]" under baseline; unexplained "*" box in the room; chip "Index 0.40 · lowest of five" | For this room at these weights, the winner has the lowest index. | D06, D07, D14, D26 |

## 3 Defect list

Severity: blocker = breaks the owner's rule (every plotted value shown with its physical meaning); major = wrong or missing content a reviewer would reject; minor = fix before release.

| ID | Tag | Severity | Keys | Defect (pixel positions on the 1280 × 720 PNG) |
| --- | --- | --- | --- | --- |
| D01 | [abstract] | blocker | criteria_2, matrix | Embodied carbon has no clause of its own and no drawn meaning in any keyframe. The carbon ruler and its cloud panel (`panel("carbon")`) exist only inside the `matrix` transition; the matrix room shows "2,520 kg CO₂e" as text. |
| D02 | [abstract] | blocker | daylight, ruler, mapped, next | The daylight factor (5.60 %) is drawn as direct sun rays, which is wrong for a diffuse-sky metric. Lamps are identical in every room (opacity not bound to autonomy), so the lighting value is not drawn either. Autonomy 81 % is text only. |
| D03 | [abstract] [layout] | blocker | variants, variants_2, criteria, criteria_2, best, units, units_2, units_3, range_3, record, pointer | 11 of 35 keyframes have no room panel. Every plotted or printed value there is a number with a thumbnail, not the design that produces it. The room rule of the quality bar is broken for a third of the clip. |
| D04 | [numbers] | major | mapped | "725 kWh/a ÷ 48 m² = 15.11" does not compute: 725 ÷ 48 = 15.10. The model gives 725.1 kWh/a (15.106), so the printed parts (121 + 567 + 37 = 725) are rounded and the printed equation is not. |
| D05 | [numbers] [abstract] | major | limits | The 21,000 CHF limit is computed in `screen_limits` as `ceil(winner cost / CAP_STEP) × CAP_STEP` (20,544 → 21,000). It is neither in the model nor in the narration, so it looks like a budget but is reverse-engineered from the winner; the winner passes by construction. |
| D06 | [depth] | major | index, index_2, winner, next | The index score is never written out. The bars are weighted segments, but no line reads 0.40 = 0.40 × 0.10 + 0.30 × 0.55 + 0.20 × 0.72 + 0.10 × 0.50 (= 0.399). |
| D07 | [depth] [abstract] | major | next, daylight | Heating (52 kWh/a, 39, 37) has no drawn meaning (no winter heat-loss arrows) and no arithmetic (183 kWh/a ÷ COP 3.5 = 52). Lighting 200 kWh/a has no arithmetic (9 W/m² × 48 m² × 1,500 h = 648 kWh/a; × (1 − 0.69) = 200). The room shows cooling arrows only. |
| D08 | [abstract] | major | cost, range | The cost value 19,428 CHF is shown as an elevation and a number, not as its parts: fixed 9,000 + wall 5,400 + insulation 1,728 + windows 3,300 = 19,428. |
| D09 | [layout] [depth] | major | ruler, ruler_2, winner, index_2, next | The room section in the panel is about 180 × 100 px (x ≈ 977–1153, y ≈ 208–305) with 11–12 px dimension labels ("1 m", "6 m"), so the example that carries the lesson is a thumbnail. |
| D10 | [layout] | major | variants, variants_2, criteria | Cards are sparse: the lower 40 % of each card (y ≈ 400–594) is empty in variants_2, and the thumbnails are ~95–110 px, so Punched and Super-insulated (same window, thicker wall) look alike. |
| D11 | [numbers] | minor | shift_2 | Punched shows 0.71, but the printed inputs give (13.73 − 10.44) ÷ (15.11 − 10.44) = 0.70. The exact model values give 0.71, so the screen cannot be recomputed from what it prints. |
| D12 | [numbers] | minor | sweep_2 | Close-call bands are drawn as one 2.5-unit-wide rectangle per sampled cost weight (`close.add(Rectangle(width=X(2.5) − x0 …))`), so each edge overshoots by ~1.3 points. Model bands for gap < 0.03: 0.0–10.0 % and 35.9–42.5 % (screen ≈ 0–11.5 % and 36–44 %). |
| D13 | [numbers] | minor | limits | The board's index values (0.40, 0.48, 0.70) are normalized over all five designs, not over the three allowed ones. Re-normalized over the allowed three: Ribbon 0.32, Punched 0.60, Super 0.90 (order unchanged). The screen does not say which set is meant. |
| D14 | [numbers] [layout] | minor | ruler, mapped, next, shift | Scale labels "10 kWh/[m²·a]" (ruler), "15 kWh/[m²·a]" (mapped) and "11 kWh/[m²·a]" (next) sit under the baseline (x ≈ 879–1055, y ≈ 566–580) with no tick or line at that level; in `next` the bar is 524 kWh/a = 10.92 kWh/(m²·a), which "11" does not state. Shift ticks +2, +4, +6 have no unit. |
| D15 | [numbers] [de] | minor | all screens with units | Unit notation "kWh/[m²·a]" (square brackets inside the unit) is used on every screen, while the model unit is "kWh/(m²·a)"; the shift chip uses hyphen-minus "- 10.44" instead of "−10.44". |
| D16 | [sync] | minor | pointer | The Pareto front is shown without marking dominated designs (Super 20,940 CHF / 13.41 and Bare 21,091 CHF / 15.11 are both dominated by Ribbon 20,544 CHF / 10.92). The axis reads "Electricity" while the ruler reads "Electricity intensity". No room. |
| D17 | [sync] [depth] | minor | range, range_2, record | Only the cost ruler is redrawn for six designs. Premium's electricity (10.41) is lower than Shaded's (10.44), so with six designs Premium becomes the best on electricity; this is never shown, and Premium's intensity appears nowhere on screen. The narration ("the rulers depend on the designs compared") is shown for one criterion. |
| D18 | [abstract] | minor | winner_2 | "weakest: Daylight 0.72" is the highest normalized value, which is not stated on screen. |
| D19 | [layout] [de] | minor | limits | DE: the leader from the Verschattete Glaswand thumbnail crosses "nicht erlaubt" (layout report: line through "nichterlaubt"). EN: the same leader passes about 10 px from the end of "not allowed". |
| D20 | [layout] | minor | criteria_2 | The daylight unit "%" wraps under its number in every card (x ≈ 99–115, y ≈ 513–537); "kg CO₂e · embodied" runs into the ↓ arrow (x ≈ 222–243, y ≈ 575; x ≈ 478–484 in the Ribbon card). |
| D21 | [layout] | minor | matrix, weights, weights_2, weights_3, sensitivity, index | Column chips are not centred on their columns. Matrix columns are 307–442–577–712–847 (centres 374, 509, 644, 779); chip centres are 328, 494, 659, 825, offsets −46, −15, +15, +46 px. Weights strip boundaries 523, 685, 793; the Cost chip (444–544) straddles 523. Chips touch the strip (y ≈ 180). |
| D22 | [layout] | minor | range, range_2 | Chips "Window 70 % · 16.8 m²" (x ≈ 870–1085) and "Overhang 1.2 m" (x ≈ 1087–1236) have no gutter. The "1" tick label and "1.00" value label are stacked at the same x (802). |
| D23 | [layout] | minor | sweep, sweep_2 | The "4 %" chip (x ≈ 234–287, y ≈ 190–220) sits on the green winner band and under the rank-1 dot. The first top thumbnail (0–4 %) is unlabelled. "rank" (x ≈ 203–240, y ≈ 520) touches the axis origin. |
| D24 | [layout] [sync] | minor | weights_3, sensitivity_2, sensitivity_3, range_3, matrix | Unlabelled marks: green and orange frames in weights_3; grey dashed separators and bracket ticks at x ≈ 32 and 932 in sensitivity_2 and 3; orange double arrow (x ≈ 955, y ≈ 235–290; x ≈ 1255 in range_3). The highlight frame of row 1 touches row 2's thumbnail in matrix and range_3. |
| D25 | [layout] | minor | units, units_3, ruler_2, all | Secondary text sits at 12–13 px (card spec lines, "Index: 0 = best …" axis note, "Window 50 % · …" lines in the index frames). Orphan caption words ("unit.", "changes."). "501 kWh/a" label touches the axis at x ≈ 877. |
| D26 | [abstract] [layout] | minor | ruler, winner, next | The "1 m" dimension sits outside the façade (x ≈ 915–943) and means nothing on screen; the overhang depth is dimensioned only in `next` ("d 0.6 m"). In `next` an unexplained teal "*" box sits in the room (x ≈ 507–548, y ≈ 300–317). |
| D27 | [de] | minor | ruler_2, shift, mapped, next (DE) | Room-bar labels in DE are verbs or abbreviations: "Licht 181", "Kühlen 281", "Heizen 39" (EN uses nouns: Lighting, Cooling, Heating). |
| D28 | [de] | minor | matrix, weights, weights_2 (DE) | Carbon is "Emissionen" in headers and chips but "Graue Emissionen" in the narration, captions and model title. |
| D29 | [de] | minor | sweep_2 (DE) | "Rang" touches the axis origin tick; the caption breaks with the orphan "entscheidest." |
| D30 | [depth] | minor | all | No standards or model footer. Energy clips cite DIN V 18599-5, DIN 4108-2, VDI 2078 in the corner or the caption; the index clip only has the room chip (EN and DE). |

Counts: blocker 3, major 7, minor 20.

## 4 Fix proposals

- D01 (carbon): Add a clause and a step of its own between `daylight` and `matrix`, e.g. "Embodied carbon adds up the materials of the same room." Show a carbon panel: the wall 18 m² × (60 + 2.5 × 16) = 1,800 kg and windows 6.0 m² × 120 = 720 kg as two stacked bars in the room panel, summing to 2,520 kg; bind to `carbon.wall`, `carbon.insulation_per_cm`, `carbon.window`. Keep the cloud for two seconds with the bars. The matrix room must show the same bars, not a text value.
- D02 (daylight, lighting): Replace the direct sun rays in the daylight clause with a diffuse-sky dome and two isolines (2 % and 5 % daylight factor) in the section (`room_section`, diffuse mode). Bind lamp opacity to 1 − autonomy (Bare 0.19, Ribbon 0.31, Shaded 0.28) in all rooms, so the 181 / 121 / 200 kWh/a lighting value is visible. Keep the "lamps off 81 %" line.
- D03 (room missing): Give each frame with a card or table a room panel of the focused design (section plus 8 × 3 m elevation with window, overhang and insulation). In variants and criteria the cards become the room; in units the sum shows the Punched room; in record and pointer show the room of the highlighted design (Ribbon) on the right side. Rule: no frame without a room panel that shows the design under discussion.
- D04 (arithmetic): Print one decimal for the room parts and total and check that the printed parts add up: e.g. "725.1 kWh/a ÷ 48 m² = 15.11 kWh/(m²·a)", parts 121.x + 567.x + 37.x = 725.1. Add a test that every printed equation reproduces at its printed precision (the checker does this for all others).
- D05 (cap): Author the budget in `🔣️.json` (e.g. `constraints.facade_cost_cap_chf = 21000`, bilingual note "client budget for this façade") and draw the line from it; delete `CAP_STEP` and the `ceil` rule. Say on screen "client budget 21,000 CHF" and narrate it. Choose the value to exclude a stated set (e.g. Shaded and Bare exceed the budget), not the winner's cost.
- D06 (index arithmetic): In `index_2` and `next` print a line under the focused row: "0.40 = 0.40 × 0.10 + 0.30 × 0.55 + 0.20 × 0.72 + 0.10 × 0.50 = 0.40", bound to the normalized values and weights; extend `seg_readout` (currently one term) to all four terms.
- D07 (heating, lighting arithmetic): In `next` (and ruler frames) print "Heating 183 kWh/a ÷ COP 3.5 = 52 kWh/a" and "Lighting 648 kWh/a × (1 − 0.69) = 200 kWh/a" (648 = 9 W/m² × 48 m² × 1,500 h). Draw winter heat-loss arrows in red with count bound to the heating demand; keep cooling arrows.
- D08 (cost parts): Add a stacked bar in the cost panel: fixed 9,000 · wall 5,400 · insulation 1,728 · windows 3,300 = 19,428 CHF; the parts come from `prices.*` and the wall and window areas (all verified by the checker). Use it again when Premium is added (range).
- D09 (room size): Enlarge the room panel to at least 320 px wide in the ruler, index and next frames, with 14 px dimension labels and 14 px spec text. Move the matrix column table left if needed.
- D10 (cards): Card layout: title, 8 × 3 m elevation at ≥ 160 px (not 95 px), spec lines, four value rows (values moved from `criteria`). Fills the lower 40 %; Punched and Super-insulated become distinguishable by the wall thickness drawn in the section.
- D11 (shift_2 reproducibility): Either compute the axis from the printed two-decimal values (Punched then 0.70 on screen), or print three decimals on the chips ("13.735 − 10.438 ÷ 4.669"). Pick one and reproduce it in the checker.
- D12 (close-call bands): Draw the band from exact crossings (bisection on gap = 0.03, as `switch_shares` does for the switches), so the edges read 10.0 % and 35.9–42.5 % (print "10 %", "36 %", "43 %" as chips). Drop the per-sample rectangles.
- D13 (feasible set): Re-normalize over the three allowed designs and say "normalized over the allowed three" (values Ribbon 0.32, Punched 0.60, Super 0.90; record must match), or keep five and print "normalized over the five compared designs; the budget filters afterwards". Recommend the first, because the constraint filters the set before ranking.
- D14 (scale labels): Replace the labels under the baseline with a dashed reference inside the bar at 10 kWh/(m²·a) = 480 kWh/a (15 → 720 kWh/a) and label it there; in `next` print "524 kWh/a = 10.92 kWh/(m²·a)". Add the unit to the shift ticks: "+2 kWh/(m²·a)" at the first tick.
- D15 (notation): Use "kWh/(m²·a)" everywhere (model unit), the true minus "−10.44", and "−" in the shift chip.
- D16 (Pareto): Grey out Super and Bare with a label "dominated by Ribbon window"; add the room of the highlighted front member (Ribbon) at the right; change the axis to "Electricity intensity, kWh/(m²·a)".
- D17 (six-design electricity): Redraw the electricity ruler for six designs in `range_2` (≈ Premium 0.00, Shaded 0.01, Ribbon 0.11, Super 0.64, Punched 0.71, Bare 1.00 with the six-design min 10.41 and max 15.11) and print Premium's 10.41 kWh/(m²·a) in the range board. The narration then holds for two criteria.
- D18 (weakest): Chip text "weakest = highest normalized value: Daylight 0.72".
- D19 (limits leader): Move "nicht erlaubt" below the dashed zone (x ≈ 560–700, y ≈ 352) or route the Verschattete Glaswand leader to the zone's top edge; check against the layout report.
- D20 (criteria_2 cards): Keep "2.00 %" on one line by shrinking the unit to LABEL size or widening the card by ~0.15 units; move the carbon arrow left of its value column so "embodied" stops at the card margin.
- D21 (chips over columns): Centre each header chip on its column: matrix centres 374, 509, 644, 779 px; weights strip centres 415, 604, 739, 820 px (boundaries 523, 685, 793). Leave 4 px between chip and strip.
- D22 (range gutter): Add 8 px between "Window 70 % · 16.8 m²" and "Overhang 1.2 m"; in range_2 drop the "1" tick label and keep "1.00".
- D23 (sweep chips): Put "4 %" above the band (y + 0.25) and "39 %" clear of its dashed line; label the first top thumbnail "Shaded" or move the thumbnail row to start at 2 %; move "rank" left of the axis (x ≈ 195, y ≈ 520).
- D24 (unlabelled marks): weights_3: remove the frames, or caption "green = electricity focus"; sensitivity_2/3: label the separators "rank change" or remove them and the bracket ticks; range_3: label the orange arrow "rank change"; enlarge the row pitch by 0.08 units under the highlight frame in matrix and range_3.
- D25 (secondary text, orphans): Set secondary text to LABEL_FONT_SIZE (≥ 14 px) and check with the layout report; caption breaks: "unit." → join with "no unit." on one line; "changes." → "…and the ranking changes." on one line; move the "501 kWh/a" label 6 px right.
- D26 (dimensions): Remove the unexplained "1 m" dimension; dimension the overhang depth d in `winner`, `ruler` and `index_2` as in `next` ("d 0.6 m" with a leader to the overhang tip). Replace the "*" box by a labelled glyph ("internal gains") or remove it.
- D27 (DE nouns): "Beleuchtung 181 kWh/a", "Kühlung 281 kWh/a", "Heizung 39 kWh/a" (EN: Lighting, Cooling, Heating) in `ruler_2`, `shift`, `mapped`, `next`.
- D28 (DE carbon term): Use "Graue Emissionen" in the header and chip (widen the chip by ~40 px), or define "Emissionen (grau)" once and use it in all DE strings.
- D29 (DE sweep_2): Move "Rang" to x ≈ 195 (left of the origin); rebalance the caption so "entscheidest." is not alone on the last line.
- D30 (footer): Add a subtitle chip and a one-line footer on every screen in both languages: "simplified annual teaching model · Zurich-like climate (latitude 47.4°) · utilization after EN ISO 13790 style" (wording taken from the model's own notes), so the index clip has the same source line as the energy clips.

## 5 Earlier audit (audit-index.md): status

| Earlier finding | Status now | Note |
| --- | --- | --- |
| limits (high): constraint never shown | Resolved | Cap line, struck designs, three-design board and room are there; the cap's origin is a new problem (D05). |
| sensitivity (high): stale end state | Resolved | Rows re-sorted, "was 1st" tag. The tag sits by the room title, not the row (D24). |
| range_2 (high): worst frame on Shaded | Resolved | Orange frame on Premium. |
| cost (high): slab at d = 0, ribbon-like punched windows | Resolved | Four 1.5 m openings, no slab, "no overhang" chip. |
| variants (high): sparse, no room named | Partly open | Room chip added (EN and DE); cards still sparse, no room (D03, D10). |
| index / index_2 (high): no room | Resolved for index_2 | Ribbon room in index_2; index shows the focus row's room. No score arithmetic (D06). |
| ruler / shift / mapped arithmetic (med) | Partly open | Conversion line added; new arithmetic error in mapped (D04); notation (D15) and scale label (D14) open. |
| winner / ruler / sweep dimension "1 m" (med) | Open | D26. |
| daylight: sun rays, "81 %" undefined (med) | Partly | Autonomy defined; sun rays and lamps open (D02). |
| weights chips not over columns (med) | Open | D21. |
| range_3 highlight without gutter (med) | Open | D24. |
| weakest criterion undefined (med) | Open | D18. |
| shift_2 decimals (med) | Open | D11. |
| 0–1 legend missing on mapped and shift_2 (med) | Resolved | Legend present from shift_2 on. |
| sweep: switch shares, axis title, rank headers (med) | Resolved | 4 % and 39 % chips, axis title, "at 0 %" / "at 100 %". |
| sweep_2: legend, first thumbnail unlabelled (med) | Partly | Legend added; first thumbnail still unlabelled (D23). |
| ruler_2 (DE) verbs for room bars (med) | Open | D27. |
| unit notation "kWh/[m²·a]" (med) | Open | D15. |
| matrix column heads (med) | Resolved | Headers with unit and direction. |
| visual principle in tables (med) | Partly | Index has a room; weights, matrix and range_3 do not (D03, D21). |
| disclaimer missing (med) | Partly | Room chip in both languages; no footer (D30). |
| range Premium off axis, duplicate tick (low) | Open | D22; off-axis dot kept, as intended. |
| CHF one-off and embodied (low) | Resolved | "CHF · one-off", "kg CO₂e · embodied". |
| caption breaks (low) | Partly | Orphans remain (D25, D29). |
| Pareto preview thumbnails unlabelled (low) | Resolved | Names next to each point. |
| sensitivity_3 and range_3 unlabelled marks (low) | Open | D24. |
| weights_3 frames (low) | Open | D24. |
| weights "equal weights" label (low) | Resolved | |
| sensitivity persona (low) | Resolved | Coin icon "cost-driven client". |
| secondary text size (low) | Open | D25. |
| index strip unlabelled (low) | Resolved | Labelled "weights". |
| DE title lowercase (low) | Resolved | "normierter". |
| shift hyphen-minus (low) | Open | D15. |
| DE "Premium-Glaswand" hyphen (low) | Open, minor | Consistent in DE, EN "Premium glass wall". |
| weights_2 DE chip touching, units thumbnail size (low) | Not rechecked | No DE weights_2 frame was reviewed in this pass. |

## 6 Number check

`check-r2-index.py` recomputes every number on the frames from the model (`evaluate`, `index`, `reweigh`, `variants`, `load_study`). Result: 160 of 162 checks reproduce (EN and DE formats, cards, room bars and parts, sums in units and kCHF, normalized rulers, index, sensitivity weights and index, six-design index and cost ruler, record ranges, Pareto front, cap rule, sun altitude 66°). The two misses are real findings:

- Bare arithmetic "725 kWh/a ÷ 48 m² = 15.11": printed total gives 15.10 (D04).
- Punched normalized value 0.71 on shift_2 is not reproducible from the printed inputs (0.70) (D11).

Model facts checked beyond the screen: switches at 3.6 % (shown as 4 %) and 39.1 % (shown as 39 %); close-call bands 0.0–10.0 % and 35.9–42.5 % (D12); Premium electricity 10.41 kWh/(m²·a) (D17); feasible-set re-normalized index 0.32 / 0.60 / 0.90 (D13); winner cap 20,544 CHF, cap 21,000 CHF; Punched cost parts 9,000 / 5,400 / 1,728 / 3,300 (D08); Punched carbon parts 1,800 + 720 = 2,520 kg (D01).

## 7 Depth benchmark and German track

Energy frames (6 looked at): every value sits on a drawn object with its formula written out (e.g. "Q_h,g = 13 300 · (1/0.95 − 1) ≈ 700 kWh/a", "400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W"), each term named with its unit, a standard cited in the corner (DIN V 18599-5, DIN 4108-2, VDI 2078), and a stacked or scaled bar with labelled segments. Sources: `⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`, `⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`, `⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` (500–1,100 lines each). The index clip meets the bar on the room values and the bar chart but not on the written arithmetic, the physical drawing for heating, carbon and daylight, or the footer (D06, D07, D02, D01, D30).

German track (8 frames): number formats are correct in every checked string (13,73; 19.428 CHF; 21.964 in the units frame; 0,36 and 20.544 CHF < 21.000 CHF in the range and limits frames); captions fit two lines; the title is lowercased correctly. Remaining DE defects: verb room labels (D27), "Emissionen" vs "Graue Emissionen" (D28), the leader through "nicht erlaubt" (D19), and "Rang" at the origin (D29).

Files: this report `T/audit-r2-index.md`; checker `T/check-r2-index.py`; output `T/🗑️generated/audit-r2/check-r2-index.out`.
