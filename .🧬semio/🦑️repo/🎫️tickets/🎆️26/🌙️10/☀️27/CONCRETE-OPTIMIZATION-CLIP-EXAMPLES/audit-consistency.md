# 🔎 Cross-clip consistency audit: six optimization clips

> **Partly superseded (2026-10-27, `title-convention.md`).** Statements that treat the persistent title chip, or the title and subtitle chip pair, as the convention to keep are marked **[superseded]**. The convention is one `_fonts.section_heading` per beat; the chapter name stays in the presentation window chrome and `TITLE` stays as transcript metadata.

Read-only audit. Sources: `T/🗑️generated/frames/<SceneClass>/*-en-*.png` (8 English frames per clip), `*-de-*.png` (2 German frames per clip), `T/🗑️generated/energy-frames/` (Beat1_LeistungUndEnergie-summe, AnlagenVerluste-sum, Beat8_Hauptgleichung-chain), `T/agent-rules.md` Quality Bar, `T/energy-idioms.md` sections 1-2. Frames are 1280 x 720 captures from before the current fix pass.

Clip keys: Beat1 = extrema (📉️extrema), Beat2 = sampling (🎲️sampling), Beat3 = experiments (🧪️experiments), Beat4 = index (⚖️index), Beat5 = pareto (🏆️pareto), Beat6 = algorithms (🧬️algorithms).

## 1. Issues per clip

| clip | key | issue | fix |
|---|---|---|---|
| Beat1_LocalGlobalExtrema | all | Reference room is 60 % WWR, 0.0 m overhang, 16 cm insulation (681 kWh/a south). The shared reference used elsewhere is 45 % / 0.6 m south. | Use 45 % / 0.6 m south as the single reference room in every clip and rebuild the sweep and minimum on it. |
| Beat1_LocalGlobalExtrema | all DE | No German PNG exists in `frames/Beat1_LocalGlobalExtrema/`, only `-de-layout.md` (all "none"), although `Beat1_LocalGlobalExtrema-de.log` reports "captured ... de 27 beats". | Re-capture DE for Beat1 and verify the PNG count before closing. |
| Beat1_LocalGlobalExtrema | formulate ... sweep_2 | "12 h" and "16 h" sun-hour labels overprint each other in the sun-path diagram. | Offset or drop one label. |
| Beat1_LocalGlobalExtrema | sweep_2, global | Stacked bar and "882 kWh/a" / "N 360°" readouts cover the "East 450°" x-axis label. | Keep the bar column clear of the axis; move readouts into the right column. |
| Beat1_LocalGlobalExtrema | argument | "value" caption chip overprints the "Window 60 %" chip; "6 m" dimension sits under the caption band. | Move the formula chip row below the header chips; keep dimensions above CONTENT_TOP. |
| Beat1_LocalGlobalExtrema | negate_2 | "S < 0: worse than east" text crosses the stacked bar. | Put the sign note in the plot margin. |
| Beat1_LocalGlobalExtrema | constraint | "Courtyard" label sits behind the title chip; "no direct sun by day" overprints the lamps. | Move the compass label below the chip; put the note above the lamps. |
| Beat1_LocalGlobalExtrema | scope_2 (closing) | Closing caption is generic ("the optimum changes") and the room shows no concrete answer. | End on the courtyard design (360° N, 882 kWh/a, about 221 CHF/a) as a room card. |
| Beat2_GridRandomSampling | parameters (opening) | No question is stated; no beat subtitle chip (energy clips use one). | Add subtitle chip and a caption naming the question, "Which nine designs?" |
| Beat2_GridRandomSampling | grid_4 | Value boxes collide ("547 kWh/a" next to "499"); "499 kWh/a" bar total overprints "Lighting"; unit lines about 9 px at 720p. | One value per cell; move bar total above the bar; labels of 13 px minimum at 720p. |
| Beat2_GridRandomSampling | limits_3 (closing) | Closing is a caveat ("the model itself still needs validation"), no design answer in caption. | Caption ends on the 66 % / 0.99 m room (494.8 kWh/a) and the validation note as a second line. |
| Beat2_GridRandomSampling | budget, cells | Dimension labels "1 m" / "6 m" about 9 px. | Raise to 13 px minimum. |
| Beat3_FactorialExperiments | question (opening) | Right 55 % of the stage is empty; single teal cooling bar instead of the stacked heating/cooling/lighting bar. | Use the stacked bar and place the 2x2 plan beside the room. |
| Beat3_FactorialExperiments | factors, plan, purpose_4 | Factors are named A, B, C and coded -1/+1; the Quality Bar forbids "A/B/C". | Name factors by physical quantity ("Window 30 -> 70 %", "Overhang 0 -> 1 m"); keep coding only in a footnote. |
| Beat3_FactorialExperiments | saving_70 | "-122 kWh/a" label sits on the room 2 section; "-279 kWh/a" sits over the room 4 top. | Place deltas in the plot margin next to the arrows. |
| Beat3_FactorialExperiments | confirm_2 | Red "-41 kWh/a" label overprints "model 339". | Move the delta under the bar. |
| Beat3_FactorialExperiments | purpose_4 (EN and DE) | Value labels "186,3 / 289,4 kWh/a" run past the red highlight frames; closing is a method statement, not a design. | Close with the best cheap design in the room ("1.0 m overhang, 70 % window: 288 kWh/a"). |
| Beat3_FactorialExperiments | all rooms | Person silhouette is grey; every other optimization clip uses the orange person. Lamps are absent. | Use the shared room primitive (orange person, lamps). |
| Beat4_NormalizedIndex | weights, ruler, mapped, index | Normalized 0-1 axes ("0 = best of the set, 1 = worst") and an invented index score (0.40) from invented weights (40/30/20/10). The Quality Bar forbids normalized axes and invented scores. | Show the physical ruler (kWh/(m²·a), CHF, kg CO2e) and the summed cost in the room; drop the 0-1 index or label it as a method note only. |
| Beat4_NormalizedIndex | variants (opening) | Opening has no question and no room in focus; the header chip is grey and not in the DIN footnote style. | Start with the seminar room (45 % / 0.6 m) and the question "Which façade?" |
| Beat4_NormalizedIndex | next (closing) | Closing is a hand-off to Pareto; the preview front uses different designs than the Pareto clip (see mismatch M4). | End on the winner room, reconcile the preview with Beat5. |
| Beat4_NormalizedIndex | colour | Teal = electricity, red = carbon, yellow = daylight, orange = cost. In energy and other clips teal = cooling, red = heating, yellow = lighting. | Keep the energy palette for energy; use a separate criteria palette. |
| Beat4_NormalizedIndex | units_2, winner | Text about 9 px ("Window 25 % · overhang 0.0 m · insulation 16 cm"); mixed units kWh/(m²·a) vs CHF without "/a". | 13 px minimum; label cost as "Façade cost CHF". |
| Beat5_ParetoFront | room (opening) | Room is small and centred-left; the right 45 % of the stage is empty. | Scale the room to the stage and put the front chart beside it. |
| Beat5_ParetoFront | walk_2 | Caption repeats the unit: "saves 65 kWh/a a year". | Reword to "saves 65 kWh/a". |
| Beat5_ParetoFront | knee | The "knee" label crowds the tile and the orange circle. | Move the label outside the front. |
| Beat5_ParetoFront | decision_2 | Three room thumbnails of about 120 px; insulation and overhang unreadable. Caption generic ("the front shows what is possible"). | Enlarge the three rooms with d, WWR, t labels; caption names the knee choice (19,402 CHF, 584 kWh/a). |
| Beat5_ParetoFront | room, walk (t) | Insulation baseline 12 cm; other clips use 16 cm. | Use 16 cm in the design set or state 12 cm as scope. |
| Beat5_ParetoFront | de room, de decision | "Dachüberstand" where other DE clips use "Vordach". | Use "Vordach" everywhere. |
| Beat6_EvolutionarySearch | task (opening) | Left 60 % of the stage is bare axes; room is small. | Show the start room larger and fade in the map at the start. |
| Beat6_EvolutionarySearch | compare_2 | Caption "all three lie within 2 kWh/a" contradicts its own chip "gap 3.2 kWh/a" (genetic). The "41 runs" chip hides the gradient label. | Change caption to the true gap (within 4 kWh/a) and move the run chip. |
| Beat6_EvolutionarySearch | map_2 | "750 kWh/a" contour label is covered by markers and leader line. | Move the label clear of the markers. |
| Beat6_EvolutionarySearch | map_2 | Room shows "d 1.0 m" for the 494.8 kWh/a optimum; sampling shows "d 0.99 m". | Show 0.99 m everywhere. |
| Beat6_EvolutionarySearch | gradient, pattern, population | Card labels (for example "10 % · 0.74 m / 810 kWh/a") about 11 px. | 13 px minimum. |
| Beat6_EvolutionarySearch | close (closing) | Closing room shows 57 % / 0.87 m (498 kWh/a); the map minimum is 66 % / 0.99 m. Caption is generic. | End on the surface optimum (66 % / 0.99 m, 494.8 kWh/a) or name why 498 kWh/a is accepted. |
| all six | title and chips | Subtitle chip used only in extrema; energy clips use it in every clip. DIN footnote (top right) only in extrema. | One chip system: title, subtitle, DIN footnote, caption band. **[superseded: no persistent title chip; one `section_heading` per beat, see `title-convention.md`]** |
| all six | stage fill | Opening frames of Beat1 (formulate), Beat3 (question), Beat5 (room), Beat6 (task) leave 45-60 % of the stage empty; Beat1 sweep_2 and Beat6 gradient are crammed. | Fill the stage with the room at a readable size; split crammed frames across two clauses. |
| all six | formula band | Only extrema (argument, maximize), experiments (contrast) and index (index formula) show a formula chain. Energy clips end each beat with a coloured formula band (Q_E, Phi_solar). | Add a formula band with coloured factors to each beat's conclusion. |

## 2. Mismatch list (same design, different numbers or labels)

- M1 Reference design. The brief expects 45 % / 0.6 m south. Seen: 60 % / 0.0 m, 681 kWh/a (extrema); 45 % / 0.6 m, 522 kWh/a (sampling, the only match); 50 % / 0.5 m, 298 kWh/a cooling only (experiments); 40 % / 0.6 m, t 12 cm (pareto opening); 50 % / 0.6 m, 524 kWh/a (index winner); 75 % / 0.1 m, 745 kWh/a (algorithms start).
- M2 Surface optimum. Sampling dense grid 66 % / 0.99 m = 494.8 kWh/a (matches brief, 495). Algorithms map minimum 494.8 kWh/a is labelled "d 1.0 m". Rounding only, but the label must match.
- M3 Algorithms compare_2. Caption "all three lie within 2 kWh/a"; chip for genetic says "gap 3.2 kWh/a" (498.1 vs 494.8). Caption is wrong.
- M4 Pareto front vs other clips. Pareto front_2 ends at 21,552 CHF / 520 kWh/a as the lowest-electricity front point. Algorithms close shows 498 kWh/a at 20,873 CHF, which dominates it. Index next preview labels a front point near 21.5k CHF at 10.4 kWh/(m²·a) (= 501 kWh/a), which also dominates it. Either the design sets differ (must be labelled) or the "Pareto front" claim is wrong.
- M5 Insulation baseline. 16 cm in extrema, experiments, index and algorithms; 12 cm in pareto.
- M6 Electricity unit. kWh/a in extrema, sampling, algorithms and pareto; kWh/(m²·a) in index and in the pareto bar caption. Numbers reconcile at 48 m² (501 kWh/a = 10.44 kWh/(m²·a)), but the axis changes between clips.
- M7 "Cost" meaning. Operating CHF/a in sampling ("Cost 125 CHF/a") and extrema ("170 CHF/a"); investment CHF in pareto ("Façade cost 19,402 CHF"), algorithms ("Façade cost 20,873 CHF") and index ("20,544 CHF", no label); "Cost 30 %" is also a weight in index.
- M8 Near-duplicate designs. Index winner 50 % / 0.6 m (524 kWh/a) and sampling 45 % / 0.6 m (522 kWh/a) are the same overhang family with different labels.
- M9 Checked and consistent (no action). Bar split 40 + 273 + 182 = 495 (algorithms map and sampling compare). Best of nine 499.2 kWh/a (sampling grid_4 and compare). Tariff 0.25 CHF/kWh used for 170 CHF/a (681 kWh/a), 125 CHF/a (499 kWh/a), payback 12 years (192 CHF / 16.25 CHF/a). 1,681 simulations. Sun 66 deg and 19 deg. Extrema 1,131 - 681 = 450 kWh/a. Experiments 837 / 3.0 = 279 kWh/a and model 339 vs room 298 = -12.1 %. Pareto walk 192 CHF and 65 kWh/a, with lighting +57 and cooling -122. Index scores reproduce from the normalized bars and weights (punched windows: 0.40 x 0.71 + 0.30 x 0.00 + 0.20 x 1.00 + 0.10 x 0.00 = 0.48; ribbon window = 0.40).
- M10 Colour ramp 495 to 846 kWh/a and contour labels 500 to 700 kWh/a are identical in sampling, algorithms and pareto. Good.

## 3. Ranking by visual and didactic depth (energy clips as reference)

Reference depth: energy clips tie each number to a drawn object and to a formula with coloured factors (for example Phi_solar = G·A·F_f·g·F_sh = 400·1,92·0,70·0,60·0,80 ≈ 258 W, Q_E ≈ 14 000 kWh/a, with the tank and battery as physical quantities).

1. Beat6 Algorithms: the solver loop, live map, room that moves, card rows and an explicit question at the start are the closest to the energy depth. Weak points: M3, M4, the empty opening stage.
2. Beat5 Pareto: a room per tile, a walk with derived payback (12 years), and a closing with three rooms. Weak points: small rooms, t 12 cm, no formula band.
3. Beat2 Sampling: the point-to-room link is direct, the dense reference is on screen and the numbers are consistent. Weak points: no question at the opening, caveat closing, crammed grid_4.
4. Beat1 Extrema: sun path, room, sweep chips and bars give the most physical content in the first half. Weak points: no German frames, 60 % / 0.0 m baseline (M1), overprints, generic closing.
5. Beat3 Experiments: each corner is a room with bars, and the interaction plot is clear. Weak points: A/B/C letter factors (rule violation), single cooling bar, grey person, empty opening, method closing.
6. Beat4 Index: the weakest. Normalized 0-1 axes and invented index score (rule violation), rooms only as thumbnails, criteria colours that clash with energy colours, a closing hand-off with no design answer.

Weakest two:
- Beat4 Index lacks any physical-unit decision in the room. Its score (0.40) is an invented combination of normalized values and invented weights, and the room appears only as a thumbnail beside the bars.
- Beat3 Experiments lacks a design answer at the end and uses symbol factors A/B/C instead of physical quantities. It shows one cooling bar per room and never returns to the question "which window and overhang to build".

## 4. Opening and closing check

| clip | opening shows room and question? | closing gives a concrete design in the room? |
|---|---|---|
| Beat1 extrema | Yes (room, sun, compass; question in caption 2) | Partly (courtyard 360° N card, but generic caption) |
| Beat2 sampling | Room yes, question no (question at "budget") | Partly (66 % / 0.99 m room, caveat caption) |
| Beat3 experiments | Yes (room, question in caption 1) | No (method summary) |
| Beat4 index | Five rooms, question implicit | No (hand-off to Pareto) |
| Beat5 pareto | Room yes, question implicit | Yes-partial (three rooms incl. knee 19,402 CHF / 584 kWh/a) |
| Beat6 algorithms | Yes (explicit question in caption) | Partly (57 % / 0.87 m, not the map optimum) |

## 5. Consistency of the shared style

- Title chip (top left, red outline): consistent in all six. **[superseded: the persistent title chip was removed on purpose, see `title-convention.md`]**
- Caption band (white outline, two lines max): consistent in all six.
- Stacked bar order (heating red bottom, cooling teal middle, lighting yellow top): consistent in extrema, sampling, algorithms, pareto. Experiments uses one cooling bar; index uses criterion stacks.
- Chips: WWR teal, overhang d orange, insulation t green: consistent in all six.
- Number format: EN "1,681" and DE "1.681", "494,8": consistent.
- Sun angles and climate chip "Zurich-like climate": consistent.
- Subtitle chip and DIN footnote: inconsistent (extrema only). **[superseded: the beat heading is now `section_heading`, see `title-convention.md`]**
- Person colour: grey in experiments, orange elsewhere.

## 6. Frames consulted

English: extrema formulate, sweep_2, global, argument, negate_2, constraint, scope_2 (7 frames; the eighth was not viewed); sampling parameters, grid_4, gaps_2, marginals, reveal_2, compare_2, limits_3, budget; experiments question, plan_2, response, plot_2, saving_70, interaction, confirm_2, purpose_4; index variants, units_2, ruler_2, mapped, weights_2, index_2, winner, next; pareto room, grid_2, dominated_2, front_2, walk_2, knee_2, third_3, decision_2; algorithms task, map_2, gradient_3, pattern_3, population_2, generations, compare_2, close.

German (2 per clip): sampling parameters and limits_3; experiments question and purpose_4; index variants and winner; pareto room and decision_2; algorithms task and close. Extrema: none on disk.

Energy references: energy-frames Beat1_LeistungUndEnergie-summe, AnlagenVerluste-sum, Beat8_Hauptgleichung-chain.
