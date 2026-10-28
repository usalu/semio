# Verification: Beat6_EvolutionarySearch (Algorithms and Evolutionary Search), Round 2

Read-only verification of `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py` (class `Beat6_EvolutionarySearch`, 43 keys, English and German). Ticket `T` = `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES`.

## 1. Verdict

**FAIL.** Defects: **1 blocker, 6 major, 13 minor** (20 in total).

The blocker is the arithmetic of the substituted ledger. Six printed lamp rows and one cooling row do not reproduce from their printed factors (section 3, V1), and the owner's rule says arithmetic on screen must be correct. The clip is much improved over the audit: probes, the mutation_2 counter, the stop-rule labels, the close frame, the German overlaps and the sum of the bar segments are fixed.

## 2. Scope and method

- Scene file mtime 22:00 (unchanged since). Room toolkit `🎨️visuals/🏠️room/🐍️.py` changed at 22:42, 22:52 and 23:26. Model `🧮️model/🐍️.py` and `🔣️.json` changed at 23:24 to 23:25; the printed numbers did not move (see section 5).
- Captures: `capture.py` run at 22:53 (after the 22:52 toolkit state) produced 43 EN and 43 DE PNGs. **All 86 were read at full size** with the Read tool. A second capture at 23:36, after the 23:26 toolkit change, was compared pixel by pixel with the 86 read frames (`T/🗑️generated/verify-algorithms/frame-diff.out`, backup in `T/🗑️generated/verify-algorithms/frames-2252/`). 70 of 86 PNGs changed, and every change lies in room drawings, sun rays and card thumbnails (back-wall heating arrows removed, short heat-loss arrows added below the glass in the small rooms). No caption, chip, bar, counter or ledger text changed, except a 2 px shift of the "Change" pill in overshoot_2. Representative full-size re-reads of the new set: en `pattern_2`, en `generations`, and crops of `task_3`, `overshoot_2`, `pattern_4`, `mutation_2`. A full re-read of the 70 changed PNGs after 23:26 was **not** repeated; the verdict relies on the 22:53 reads for text and on the crops for the room changes.
- Layout reports (`T/🗑️generated/frames/Beat6_EvolutionarySearch/…-en-layout.md`, `…-de-layout.md`): both list "3 strict findings in 2 of 43 frames" (map_2 and overshoot). The map_2 finding is a false positive: the contour passes under the opaque "map minimum" chip and is not visible. The overshoot finding is real (V9).
- Check scripts: `T/check-r2-algorithms.py` (108 OK, 3 MISMATCH, see section 6), `T/check-algorithms-narration.py` (43 clauses, 0 textual problems, 0 failed facts), `T/verify-r2-arithmetic.py` (new: recomputes every substituted row, slope and nudge from the model; output `T/🗑️generated/verify-algorithms/arithmetic.out`, identical before and after the model change).

## 3. Defects

Severity: blocker = owner's rule broken (numbers on screen not true, or arithmetic not as printed); major = a reviewer would reject it; minor = polish or wording.

### Blocker

| Key | Sev. | Keys and location | Defect | Concrete fix |
| --- | --- | --- | --- | --- |
| V1 | blocker | task_2 (117), gradient_4 (cooling 413, lamps 153), map_2 (182), overshoot_2 (205), gradient_result (189), confirm (189); EN and DE. Scene `_ledger`, lines 268–277. | Printed rows do not reproduce from their own printed factors. Lamps: 0.43 × 1,500 × 18 % = 116.1 (printed 117); × 24 % = 154.8 (153); × 28 % = 180.6 (182); × 32 % = 206.4 (205); × 29 % = 187.1 (189). Cooling: 1,237 ÷ 3.0 = 412.3, printed 413. Exact model values are 117.2, 153.2, 181.6, 205.2, 189.8 and cooling 412.4, so the printed integers are largest-remainder apportionments that look wrong on screen. Cause: power rounded to 2 decimals (line 276, 0.432 → 0.43), share to 0 decimals (line 277), and the results use `parts_of=group` (lines 273, 275, 277), so the value after ÷ is the apportioned part. | Print each factor at the precision it is computed (power 0.432 kW; demand, COP and EER to one decimal; lamp share to 0.1 %), and print every row and the bar parts with `self.parts(..., digits=1)`. At one decimal the rows close: for gradient_result the exact parts 42.79 / 263.84 / 189.76 print as 42.8 + 263.8 + 189.8 = 496.4, the total the bar shows. The demands have to be printed to one decimal from the model as well, so that demand ÷ COP and demand ÷ EER reproduce the printed parts. Verify with `verify-r2-arithmetic.py` after the change. |

### Major

| Key | Sev. | Keys and location | Defect | Concrete fix |
| --- | --- | --- | --- | --- |
| V2 | major | gradient_3 (EN, DE). Scene `_slope_rows`, lines 504–507. | The overhang slope strip prints `(743.05 − 746.33) / 0.024 m = −136`. From the printed values that is −136.7 (rounds to −137). The model value −136.4 is correct only from unrounded probes. The check at line 506–507 compares with the unrounded slope within half a unit, so it accepts −136.67. | Print the two probes with three decimals (743.053 and 746.327 give −3.274 / 0.024 = −136.4), and test the rounded printed values, not the unrounded slope. |
| V3 | major | crossover, crossover_2 (EN, DE). Scene `_breeding`, line 779 (path), line 790 (`set_design(self.focus, crossed, 1.4)`). | Counter reads 8 of 50, but the bar shows 583 kWh/a, the ledger reads "86 + 191 + 306 = 583 kWh/a" and the room shows the child (30.3 %, 0.69 m). 583 is the crossover-only what-if, which the mutation frame itself labels "(what-if)" and the model has not simulated. The child is first simulated at run 9 (563). | Keep the focus (bar, ledger, room) on the last simulated design until the child is counted. Show the crossover child only as the dashed what-if card, as `_mutation_cards` already does. |
| V4 | major | compare, compare_2, stopping_2 (EN, DE). Scene `_compare`, lines 891–952. | Best-so-far curves are plotted against run count, but the room thumbnails show only the final designs. No live marker moves along a curve, so at run n the kWh/a value has no design on screen (audit B5, still open). | One ValueTracker on the run count; at each curve tip a live marker and a room thumbnail of that method's best design so far (`self.d["best"][key][n - 1]`); draw the 1 % band (494.8 to 499.8) as a shaded band. |
| V5 | major | task_2 onward (EN, DE). Scene `_flows`, line 313 (glass loss 1,134 kWh/a, gross); `_ledger` lines 272–273 (heating demand 122 kWh/a, net). | "Heat loss through glass 1,134 kWh/a" stands beside "Heating demand 122 kWh/a" with no bridge. The reader cannot see that wall, ventilation and the solar and internal gains account for the difference (audit D2, conversion hidden). | Add a bridging row computed from the model: glass loss + wall and ventilation − gains = heating demand, with each term substituted. Or relabel the glass figure "gross glass loss before gains" and show the gains. |
| V6 | major | genetic and pattern beats (all frames), depth check (6). Scene `_breeding` (line 749), `_pattern_band` (592), `_stopping` (953), `_ledger` size 14 (line 278). | Depth is below the energy clips (audit D1, open). The only symbolic formula band is the gradient difference quotient (gradient_3, gradient_4). Selection, crossover, mutation, halving and the stop rules are chips with no symbols and no live substitution. Chips and formula text are 13 to 14 px; the energy stills (`T/🗑️generated/energy-frames/Beat5_Verschattung-raffstore.png`, `Beat8_Hauptgleichung-chain.png`) use symbolic formulas with coloured live numbers at about 20 px. | One symbolic strip per method with live numbers, e.g. gradient x' = x − η·∇E (already partly there), crossover child = (w of parent B, d of parent A), mutation (±Δ), halve the step when no neighbour is below E*, stop when min(budget, tolerance). Set at 18 px or larger. |
| V7 | major | pattern_5 (EN, DE). NARRATION row `pattern_5` (line 103). | The narration says the steps shrink "toward the valley floor". The pattern search stops at 75 % / 1.12 m (496.7 kWh/a), not at the valley floor 66 % / 0.99 m (494.8 kWh/a) that map_2 and close label as the floor. The claim is false, and the checker does not test it. | EN: "Round after round the steps shrink until no neighbour is better." DE: "Runde um Runde werden die Schritte kleiner, bis kein Nachbar besser ist." |

### Minor

| Key | Sev. | Keys and location | Defect | Concrete fix |
| --- | --- | --- | --- | --- |
| V8 | minor | mutation (EN, DE). Scene `_mutation_cards`, line 744 (nudge computed from unrounded values). | "nudge +0.9 %" beside a child moving from 30.3 % to 31.1 %, which reads as 0.8 on screen. | Print the window shares of the cards with two decimals (30.27 → 31.13 gives +0.86, which reproduces "+0.9 %"). |
| V9 | minor | overshoot (EN, DE). Scene `_gradient`, lines 463–554. | The dashed valley line at 66 % ends exactly on the right border of "valley floor 66 % · 495 kWh/a" and runs at the edge of the "%" of "overshoot 10.7 %" (layout checker: real, although not a crossed glyph). | End the dashed line at the valley chip's bottom, or move both chips left of 60 %. |
| V10 | minor | gradient_result, pattern_result (EN, DE). Scene `_result`, lines 449–451. | The three result chips stack with frames overlapping by 2 to 4 px. The strict checker does not catch frame-to-frame overlaps. | Leave a 0.1-unit gap between the chips. |
| V11 | minor | population (EN, DE). Scene `_genetic`, lines 705–707 (`first_dots`). | Eight solid map dots appear while the counter reads 0 of 50. | Draw them as dashed ghosts until each design is counted. |
| V12 | minor | population (DE, line 681), mutation and mutation_2 (DE, line 744), formulas (EN, DE). | DE "Bestwert noch keiner" is poor German; DE "Schub" for nudge conflicts with the caption's "Mutation". Hyphen-minus is used in "-136", "-0,07 m" and "-0,13 m". | DE: "noch kein Bestwert" and "Mutation" (or "Verschiebung" throughout). Use the typographic minus "−" in all signed values. |
| V13 | minor | stopping_2 (EN "after 23 / 21 / 15", DE "nach 23 / 21 / 15"). Scene line 977. | The unit is missing; `_result` (line 449) prints it. | Add `self.tr("simulations", "Simulationen")` after the run count. |
| V14 | minor | mutation_2 (EN, DE). NARRATION row `mutation_2` (line 115). | "Later a nudge can also hurt" compares the simulated child (648) with the untested crossover what-if (629). The chip says "(what-if)", the caption does not. | Name the baseline in the caption: "Later a nudge can also hurt: against the untested crossover, the simulated child is worse." |
| V15 | minor | compare, compare_2, stopping (EN, DE). `_compare` (891–952), `_stopping` (953–982). | The 1 % band is only a chip and a dotted line; the tolerance rule ("gain below 1 kWh/a") is only text, not drawn on the curve (audit B6 residual). | Shade the 1 % band on the chart; draw a bracket for the tolerance window on each curve. |
| V16 | minor | population, survival, children (EN, DE). `_card` line 322 (thumbnail scale 0.13); card order in `_children` (804). | Cards are not sorted by kWh/a; elite stars sit on the card corners; thumbnails are too small to read window share (audit L11, L12). | Sort the slots by kWh/a within a generation; put the star in the card's inside corner; raise thumbnail scale to about 0.19. |
| V17 | minor | pattern_4 (EN, DE). Card note at `edge` (line 356). | A clamped probe is labelled "windows 0 %" with "at the limit" (audit N7 residual). | Label it "no move: at the limit". |
| V18 | minor | summer rooms, all frames with cooling flow (EN, DE). Toolkit `room_section` (`C/🎨️visuals/🏠️room/🐍️.py`), scene `_mode_a`, line 985. | Cooling arrows now scale in length (593 longest in task; 273 and 253 shortest in map_2 and overshoot_2; checked in `T/🗑️generated/verify-algorithms/crops/cooling-arrows-stack.png`), so audit B2 is partly fixed. The count stays at three and the scale is not labelled. | Label the arrow scale ("1 arrow length = X kWh/a") or let the count follow cooling demand. |
| V19 | minor | confirm → close (EN, DE). `_proof` (winner note), `_close` (1063–1083). | The switch from "winner" (61 %, 0.91 m) to "map minimum" (66 %, 0.99 m) is not announced (audit S6, residual). | In close, add a short arrow from the winner chip to the star and the chip text "full map minimum (1,681 runs)". |
| V20 | minor | gradient_result, pattern_result (EN, DE). `_result`, line 444. | No start and result room side by side at the same scale (audit D3); only proof has three rooms. | Add a start room at the same scale next to the result room. |

## 4. Checks 1 to 7

1. **Physical meaning shown at the same moment.** Passed for probes (lens, crosses and probe cards), GA children and best-so-far (cards, rooms, bar, chip), the heat-loss arrows (scale follows the loss) and the optimisation steps. Failed for the crossover what-if (V3), the compare curves (V4), the glass loss versus demand (V5). Lamp light and daylight are drawn with autonomy, but brightness against autonomy was not measured at pixel level.
2. **Numbers and arithmetic.** 108 printed values reproduce from the model (`check-r2-algorithms.py`). Every narrated figure holds (narration checker, 0 failed facts). Counters match the evaluation indices: children 14, mutation 9, mutation_2 18, stopping 23 / 21 / 15, tournament rank 5 of 8 versus rank 8. Failed: the ledger rows (V1), the overhang slope (V2), the nudge (V8). Failed narrated claim: V7 ("toward the valley floor"). Units and signs are right except V12.
3. **Layout.** No clipped text, no text outside the stage, no unreadable text at 1280 px. Overlaps: V9 and V10 (frames only). Text is 13 to 14 px for chips and formulas (V6). Empty areas are intentional (the map before the heat map appears in task).
4. **Narration and visual sync.** The counter and beat order match the narration, except V3 (value before its run) and V11 (designs shown before counting). The mutation_2 counter at 18 and the pattern_4 halving verdict are now synchronous.
5. **German.** All 43 frames carry complete German text. Number formatting is correct (1.681, 1.134, 21.260 CHF, 0,43 kW, 74,3 %, 3,3 kWh/a). Wording and signs: V12 and V13. The overlap of "bisher beste … bestes Ergebnis" in genetic_result_2 is gone.
6. **Depth comparable to energy clips.** Not met (V6, major).
7. **Audit-r2 defects.** See section 5.

## 5. Status of the audit-r2 defects

| Audit ID | Status now |
| --- | --- |
| B1 heating and lighting undrawn | Resolved. Heat-loss arrows through the glass scale with the loss; lamps and daylight are drawn. The link to demand is open (V5). |
| B2 cooling flow constant | Partly. Length follows cooling demand; count and scale are not labelled (V18). |
| B3 probes and slopes undrawn | Resolved (lens, crosses, probe cards, formula strip). Slope arithmetic open (V2). |
| B4 later child and what-if | Partly. mutation_2 resolved (child drawn at run 18). Crossover still shows the what-if as the child (V3). |
| B5 compare not linked to designs | Open (V4). |
| B6 cost, threshold, altitude | Partly. "façade cost 21,260 CHF" and "sun altitude 66°" resolved; threshold and 1 % band not drawn (V15). |
| S1 stop rules after the fact | Resolved. Rules labelled; stopping_2 gives the 50-run comparison. |
| S2 counter and simulation order | Resolved (mutation_2 at 18 of 50). |
| S3 tournament_2 wording | Resolved ("mid-ranked", rank 5 of 8). |
| S4 pattern_4 verdict | Resolved (halving verdict shown). |
| S5 cloud contracts | Partly. Earlier generations faded. |
| S6 confirm → close | Open, minor (V19). |
| L1 stopping badge under panel | Resolved. |
| L2 crossover chip clip | Resolved (no overlap). |
| L3 probe chips over dimension labels | Resolved. |
| L4 empty close frame | Resolved (map and trails back). |
| L5 overshoot chip contrast and line | Partly (V9). |
| L6 and E1 German genetic_result_2 overlap | Resolved. |
| L7 duplicate note chips | Resolved. |
| L8 contour chip on the start marker | Resolved. |
| L9 colour bar ticks | Resolved (600 and 700 shown). |
| L10 text size | Partly (14 px, V6). |
| L11 elite stars, card order | Open (V16). |
| L12 thumbnail size | Open (V16). |
| L13 layout checker | Partly. The checker now sees chips (finds overshoot); it misses frame-to-frame overlaps (V10) and reports a false positive on map_2. |
| L14 right margin | Resolved (chips at 1,257 px, close to the margin). |
| L15 crossover box | Resolved (dotted rectangle). |
| L16 focus room tag | Resolved. |
| D1 formula band | Open (V6). |
| D2 demand to electricity | Partly. Ledger present; arithmetic fails (V1); bridge open (V5). |
| D3 before and after rooms | Open, minor (V20). |
| N2 segment sums | Resolved (43 + 264 + 189 = 496; 83 + 193 + 287 = 563). |
| N3 step and probe rounding | Resolved (2.5 %, 72.5 %, 77.5 %). |
| N4 overhang precision | Resolved (two decimals). |
| N5 "1 m" scale bar | Resolved (removed). |
| N6 "Sun 66°" | Resolved ("sun altitude 66°"). |
| N7 signs and "edge" | Partly (V17, V12). |
| N8 generation list | Resolved. |
| N9 gradient slope arithmetic | Open (V2). |
| E2 colon spacing | Resolved. |
| E3 close names in German | Resolved. |
| E4 stopping badge | Resolved. |

## 6. Checker notes (not frame defects)

- `check-r2-algorithms.py`: 108 OK. Its 3 MISMATCH lines are stale assertions hard-coded from the old frames: "later child at run 9" (the frame now reads 18 of 50), "segments 497 vs 496" and "564 vs 563" (the frames now print 496 and 563 exactly). Update the script before reusing it.
- `check-algorithms-narration.py`: 43 clauses, 0 textual problems, 0 failed facts. It does not test the valley-floor claim (V7); add a test that the pattern endpoint is within 1 % of the map minimum before any "valley floor" wording.
- `verify-r2-arithmetic.py` (new): eight printed substitutions do not close (V1, V2, V8). Output: `T/🗑️generated/verify-algorithms/arithmetic.out`.

## 7. Files

- Scene: `C:\git\semio\🎓️teaching\🏛️architecture\🎯️optimization\🎬️clip\🧬️algorithms\🐍️.py`
- Toolkit: `C:\git\semio\🎓️teaching\🏛️architecture\🎯️optimization\🎬️clip\🎨️visuals\🏠️room\🐍️.py`
- Model: `C:\git\semio\🎓️teaching\🏛️architecture\🎯️optimization\🎬️clip\🧮️model\🐍️.py`
- Frames (23:36 capture): `T/🗑️generated/frames/Beat6_EvolutionarySearch/`
- Logs, crops, diffs and the 22:53 backup: `T/🗑️generated/verify-algorithms/`
- Check script kept in the ticket: `T/verify-r2-arithmetic.py`
