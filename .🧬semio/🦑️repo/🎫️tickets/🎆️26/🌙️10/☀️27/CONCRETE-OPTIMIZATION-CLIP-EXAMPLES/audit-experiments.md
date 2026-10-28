# Audit: Beat3_FactorialExperiments (`🧪️experiments`)

Read-only audit. Scope: the 34 English stills (every key, in narration order), 9 German stills spread across the clip (question_2, plan_2, response, saving_70, main, difference, model_3, confirm_2, purpose_4), both layout reports, the source `🧪️experiments/🐍️.py`, `plan.md`, `agent-rules.md`, `storyboard-experiments.md`, the energy idioms (sections 1 and 5), and five energy reference stills (Beat1 summe, Beat8 chain, Beat5 raffstore, AnlagenVerluste sum, Beat1 reserve). Stills were judged at 1280 px; no zoomed crops were made, so small-text claims are approximate.

## Verdict

Not ready for delivery. The first half (question, factors, 2x2 plan, response, plot, savings) meets the owner's test: each value (313, 190, 567, 288 kWh/a; room 298 kWh/a) sits next to a live room, numbers agree across cards, bars, table and plot, and the German captions and decimal formatting are clean. From the model beat onward the clip becomes formulas, chips and tiny thumbnails, and the constant (339 kWh/a), the coefficients and the fraction values lose their room. Three defects must be fixed first: the curvature beat shows a room bar reading 0 kWh/a, the printed subtraction "427 - 252 = +176" does not add up, and the model formula stage has no room at all. Savings labels lack units, value tags cross highlight frames, and the fraction beat does not explain why insulation raises cooling. The clip is also shallower than the energy clips: small repeated rooms, no heat flows, no colour-matched formula fragments.

Count: 3 high, 16 medium, 18 low (37 findings).

## Findings

| key | severity | problem | concrete fix suggestion |
| --- | --- | --- | --- |
| question | low | Room is drawn 6 m deep only. The 8 m facade width and 3 m height of the study room never appear. "t 16 cm" is a bare letter; "Sun 66°" is unexplained. | Dimension facade 8 m and height 3 m; label "t 16 cm · insulation" and "Sun 66° · summer altitude". |
| question_2 | low | Caption repeats the question card word for word. The chip "Response · cooling electricity" uses statistics jargon. | Shorten the caption to a lead-in, or rename the chip "Cooling electricity, kWh/a (result)". |
| question_3 | low | "Sun · room · weather stay fixed" is 17 px grey on near-black (low contrast). Storyboard icons are missing. | Use INK colour or a chip; add the four small icons or drop that storyboard item. |
| factors_3 | med | Coding rule is never shown. The storyboard lines "A = (WWR − 50 %) / 20 %" and "B = (d − 0.5 m) / 0.5 m" are absent, so A and B are never defined on screen. | Add both coding lines under the sliders and keep them visible through plan and model. |
| plan | med | Thumbnails are about 24 px and unreadable, so "miniature facade" fails. Run order is (−,−), (−,+), (+,−), (+,+) (B varies fastest), not the storyboard order. Headers "A", "B" are bare letters. | Enlarge thumbnails to at least 60 px or drop them here. Use standard order (A fastest) or state the order. Headers "A · window", "B · overhang". |
| plan_2 | low | The thumbnail pulse is invisible at this size, so "the miniature facade shows what changed" is not conveyed. | Colour the changed element (window or overhang) on each thumbnail, or drop the pulse. |
| response | low | Dimension labels "1 m" and "6 m" in every card are about 9–10 px. | Remove them from the cards or raise to at least 14 px. |
| response_2 | med | Highlight frames cut through value tags: "190 kWh/a" crosses the green frame edge, "567 kWh/a" runs past the orange frame edge, "313 kWh/a" touches the orange frame's left edge. | Make each frame include its tag (frame wider than card + tag) or move the tag inside the card. Same defect recurs in main, main_2 and difference. |
| plot | low | Legend block sits directly beside the "288" tag and reads as crowded. | Move the legend below the x-axis title or to the upper-left of the plot area. |
| plot_2 | med | Caption says each point is "the room beside it: orange … green …", but after the pulse no room carries an orange or green cue, so the link is not visible at rest. | Keep a persistent colour frame or leader per card in the marker colour (orange no overhang, green 1 m). |
| saving_30 | med | "-122" has no unit, on the card and on the plot arrow. Chip "Sun admitted: 612 → 245 kWh/a" shows admitted sun, not the blocked sun (367 kWh/a) that explains the −122. Storyboard's dimmed blocked rays with crosses are not discernible at 1280 px. | Write "−122 kWh/a". Show "blocked 367 kWh/a → cooling −122 kWh/a" in the chip. Dim and cross the blocked rays at the overhang. |
| saving_30 (caption) | low | "Saves little" is an absolute statement; the relative saving is 39 % here and 49 % at 70 %. | Say "saves 122 kWh/a, less than at 70 %", or print both percentages. |
| saving_70 | med | "-279" has no unit (card and arrow). Chip "Sun admitted: 1,428 → 591 kWh/a" omits the blocked sun (837 kWh/a). | Write "−279 kWh/a". Show "blocked 837 kWh/a → cooling −279 kWh/a"; the ratio (about 2.3x) then shows the proportionality. |
| interaction | low | The dashed "parallel" line runs through the "−279" label. The "difference 157 kWh/a" figure is not carried into the ½ formula. | Move "−279" clear of the dashed line. Reuse 157 in the difference strip ("½ × 157 = 78.5"). |
| interaction_2 | low | Caption on the overhang effect depending on window size has no on-screen text; only arrows pulse. | Add a chip "overhang saves 122 at 30 %, 279 at 70 % kWh/a". |
| main | high | Printed arithmetic does not add up: "A effect = mean at 70 % - mean at 30 % = 427 - 252 = +176 kWh/a" (427 − 252 = 175). The underlying means are 427.5 and 251.5, rounded in opposite directions. Hyphen-minus used instead of minus. German strip has the same error. Tags for 190 and 313 cross the cyan and yellow frames. | Show means to one decimal (427.5 − 251.5 = +176.0), or compute the effect from the displayed rounded means. Use the true minus sign. Fix frames as in response_2. |
| main_2 | low | The storyboard's dashed mean lines at 650 and 1,050 are replaced by two circles at 50 % with no value labels. Orange frame edge cuts the "1 m / 5 m" dimension labels above it. | Print "mean 440" and "mean 239 kWh/a" beside the circles. Keep frames clear of dimension labels. |
| difference | low | ½ × [(−279) − (−122)] = −78.5 is shown as "−78". Nested form "[ [-279] - [-122] ]" is hard to parse. The storyboard's numeric substitution step is replaced by a meaning chip. | Show −78.5 (or −79). Write "(−279 − (−122)) ÷ 2". Keep the substitution line before the chip. |
| model | high | Formula "Cooling = 339 + 88·A − 100·B − 39·A·B" stands alone. Plot and room link are gone, the four cards do not change, and "339 = mean of the four runs" appears on no room or plot. A and B are not named on this screen. | Keep the plot and four points; draw 339 as a level line. Name A ("window") and B ("overhang") beside their terms. Animate the cards to show each term's contribution. |
| model_2 | med | The constant's room is a ghost thumbnail of about 30 px ("50 % · 0.5 m") with no value. The storyboard's hollow star on the plot is missing. | Show a ghost bar at 339 kWh/a next to a 50 % / 0.5 m room, linked to the plot's mean point. |
| model_3 | med | "half of the effect +176 kWh/a" reads as if the half were 176. The "÷ 2" arrows from effects to coefficients are absent. Effects appear only as small grey text. | Write "effect +176 kWh/a ÷ 2 = coefficient +88". Draw the arrows from effect to coefficient. |
| curvature | high | The big room shows "0 kWh/a" with an empty bar and "model 339" above it. The room has not been evaluated yet, so 0 is a false value on screen. | Hide the room's bar and value until the confirm step, or show "not run yet". |
| curvature (lens) | med | The two alternative curves form a closed lens between the same ends and read as an ellipse, not as a sagging and a bulging alternative. End dots at 50 % (about 439 at 0 m, 239 at 1 m) are model values drawn like measurements, with no label. | Draw one sagging and one bulging dashed curve from the start dot. Make end dots hollow and label them "model". |
| confirm | low | Plot shows the hollow model circle and the yellow room dot without value tags. The gap line is about 0.2 units and barely visible. | Add tags "339" and "298" and a clear gap bracket on the plot. |
| confirm_2 | med | Storyboard's gap segment inside the bar (339 vs 298) is missing. Caption says "higher cooling demand" while the axis says cooling electricity; demand and electricity are not the same quantity. | Draw the gap in the bar and label "−41 kWh/a (−12 %)". Use "cooling electricity" in EN and "Kühlstrom" in DE, not "Kühlbedarf". |
| confirm_3 | low | Storyboard's dashed 1.0 m outline in the room is missing. Chip "0.5 m already delivers 69 % of the saving of 1.0 m" does not say at which window size (50 %). | Add the dashed 1.0 m outline. Write "at 50 % window" in the chip. |
| repeat | low | The big-room bar repeats the 298 kWh/a bar of the "Run it again" pair. Correct but redundant. | Drop one, or mark the big bar "same run". |
| repeat_2 | med | "Measure it again" bars are scaled from 298 kWh/a times (1 ± wobble) on the same kWh scale, while the card says "no simulated values". Storyboard icons are text only. Grey 14 px captions have low contrast. | Draw the wobble without a value scale and label it schematic. Add the icons. Raise caption contrast. |
| purpose | med | Checklist uses bare coded letters: "A +176 · B −201 · A·B −78 kWh/a". Five thumbnails of about 50 px carry values but no run or setting label. The pale 567 thumbnail's colour meaning is unexplained. | Write "window +176 · overhang −201 · window × overhang −78 kWh/a". Label each thumbnail with its run and setting, or drop the thumbnails. |
| purpose_2 | med | Same design, two values: 30 % · 1.0 m is 190 kWh/a at 16 cm (2x2) and 186 kWh/a at 10 cm (fraction), with no on-screen insulation change. Table shows 316.5, 186.3, 564.8, 289.4 while cards show 317, 186, 565, 289. Headers "A", "B", "C" are bare letters. | Show the insulation basis (16 cm baseline vs 10/30 cm) on the cards or in a chip. Use one precision throughout. Headers "A · window", "B · overhang", "C · insulation". |
| purpose_2 (cube, wall) | low | Storyboard's 2³ cube sketch is missing. The 10 cm and 30 cm wall layers are barely distinguishable at card scale. | Add the cube with four filled corners, or drop it from the storyboard. Make the insulation layer visibly thicker per level. |
| purpose_3 | low | Identical C and A·B columns are framed well. Storyboard's "runs 1 and 4 marked" on the cards is missing. Table headers are bare letters. | Mark runs 1 and 4 on the cards in the frame colour. Name the columns. |
| purpose_4 | med | Contrast arithmetic checks out (−78.3 + 5.7 = −72.6). But: (a) C = +5.7 kWh/a means thicker insulation raises summer cooling, which is never explained (storyboard expected a decrease); (b) the confounding error is 5.7 of 72.6 kWh/a, so the lesson barely shows; (c) the reveal is small grey text and the storyboard's final chip "document the confounding" is missing. | Explain the sign on screen (for example reduced night-time loss) or check the insulation term. Choose levels where the alias error is visible. Enlarge the reveal and show "error 5.7 kWh/a". |
| all frames (layout) | med | Both layout reports say "none" for all 34 keys, yet text crosses frame lines, tags are unreadable and a value shows 0. The checker does not test text against highlight frames, minimum font size or text inside cards. | Extend layout_conflicts with text-vs-frame and minimum-size checks. Do not rely on "none". |
| all frames (depth) | med | Against the energy stills (Hauptgleichung chain with colour-matched factors, Anlagenverluste stacked column, Verschattung scale), this clip has small repeated room cards, no heat flows or ripples at the floor patch, no colour-matched formula fragments, and single-bar cooling readouts. | One large live room per formula clause. Colour-match formula factors to rays, bars and overhang. Show heat flow or ripples where sun lands. |
| all DE frames | low | Bar caption "Kühlen" is a verb (question, confirm and purpose frames). Captions "Kühlbedarf" (confirm_2) conflicts with the "Kühlstrom" axis. | Use "Kühlstrom" for the bar caption and the caption. |
| all formulas | low | Hyphen-minus "-" in strips and chips ("-122", "-78", "-201", "-279", "-72.6") next to "−" elsewhere. | Use the true minus sign everywhere. |

## Storyboard deviations (implementation vs `storyboard-experiments.md`)

- Same 34 keys and clause order. Captions for saving_30 and saving_70 drop the numbers; several captions are reworded (e.g. difference_2, interaction_2, main_2).
- Missing: coding line (factors_3); sun/room/weather icons (question_3); blocked-ray dimming and crosses (saving_30, saving_70); dashed mean lines (main_2); hollow star on the plot (model_2); "÷ 2" arrows (model_3); the two plot lines recoloured as "model" (model); gap segment in the bar (confirm_2); dashed 1.0 m outline (confirm_3); icons (repeat_2); cube sketch (purpose_2); runs 1 and 4 marks (purpose_3); final "document the confounding" chip (purpose_4).
- Changed: run order (B fastest); confirm_3 sweeps overhang, not WWR; interaction gap is 157 kWh/a (storyboard placeholder 500); centre run 298 vs model 339 (storyboard placeholders 760 vs 850); "Sun admitted" chips and "Lower is better" chips added; curvature keeps the big room but shows 0.
- Simplified-model disclaimer chip is present (response_3), as required.

## Strengths to keep

- Numbers are consistent across the whole clip: 313, 190, 567, 288 kWh/a and the 298 / 339 pair agree on cards, bars, table, plot and chips, in EN and DE.
- The 2x2 square of live rooms reappears in every effect frame (column frames for A, row frames for B, diagonals for A·B). This geometry-as-contrast device is the clearest teaching move in the clip.
- Savings are drawn three ways at once (ghost bar with green gap, plot arrow, chip), as the storyboard requires.
- Overhang is encoded twice on the plot (orange circle at 0 m, green square at 1 m) with a legend.
- The interaction is shown by a dashed parallel line with the orange slope, so the non-parallel gap is visible.
- The B effect is drawn as two circles on the window-50 % line, consistent with the averaged lines.
- Confirmation: ghost bar at 339 vs bar at 298, and the chip "model 339 · room 298 kWh/a (−12.1 %)".
- purpose_3: the C and A·B columns framed in red with the chip "C = A·B" is the single clearest frame in the clip.
- German: captions fit in two lines in all 9 frames checked, with no truncation; decimal commas and thousands separators are correct ("1.428", "−12,1 %", "0,0 m").
- Coded −1 / 0 / +1 labels on the sliders and in the 2x2 table tie the design to the factor levels.

## Method notes

- Layout reports (`en-layout.md`, `de-layout.md`): all 34 keys read "none". Not trusted, see layout row.
- No source file edited, no git command run, no other file written.
