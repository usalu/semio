# Final verification: Beat5_ParetoFront (🏆️pareto) and Beat6_EvolutionarySearch (🧬️algorithms)

Read-only verification, English and German, 1280 x 720 keyframes. No source file was edited.

Ticket: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES`

## 1. Verdicts

| Clip | Verdict | Blockers | Majors | Minors |
| --- | --- | --- | --- | --- |
| 🏆️pareto, Beat5_ParetoFront | **FAIL** (en and de) | 2 | 5 | 14 |
| 🧬️algorithms, Beat6_EvolutionarySearch | **FAIL** (en and de) | 1 | 10 | 14 |

Both clips fail on arithmetic the owner's rule forbids: printed equations that do not evaluate as printed, and printed values that do not belong to the state on screen. The rest of the defects are presentation (precision, collisions, wording).

## 2. Scope, freshness and method

- Frames: all 32 keys per language for pareto, all 43 keys per language for algorithms, read as PNG with the Read tool.
- Freshness: pareto frames 01:23 to 01:26, scene `🏆️pareto/🐍️.py` 23:47, toolkit `🏠️room/🐍️.py` 00:48. Algorithms frames 00:49 to 00:58, scene `🧬️algorithms/🐍️.py` 00:49:11, toolkit 00:48. No re-capture was needed.
- Reading: four fresh readers, one per language and clip, each reported that every frame of its set was viewed (32/32 and 43/43, none left unviewed). My own first pass over the English pareto frames returned "[media removed: request limit]" for most reads, so I did not use my own notes from that pass as evidence.
- My own checks on the blocker rows: recomputed every printed equation and every Δ row from the printed values (section 4).
- Reader-reported frame collisions (legend against room wall, guide against legend title, label overprints) are listed as reported; crops are in `final-verify/pareto-en`, `final-verify/pareto-de`, `final-verify/algo-en`, `final-verify/algo-de`. I did not re-inspect every crop.
- One reader claim was rejected: that the winter "davon durch Glas" value should change with the overhang. That value is a transmission loss (U times window area times degree hours), so it depends on window area only; 1.134,0 to 1.209,6 kWh/a at WWR 80 % is consistent with that.

## 3. Check scripts

| Script | Result |
| --- | --- |
| `check-pareto-r2.py` | 0 failed (exit 0). Log `final-verify/check-pareto-r2.log` |
| `verify-check-pareto.py` | 2 FAIL: `walk_2` heating change printed -0.1 vs model 0.000; knee step 7 cooling change prints +50.4 but the model gives +50.455 (rounds to +50.5). Log `final-verify/verify-check-pareto.log` |
| `algorithms-r3-arithmetic.py` (run with PYTHONUTF8=1) | 88 designs; 63 ledger rows print "≈" where the last digit differs; the check accepts this policy. See A-M3 for the row that is not honest. Log `final-verify/algorithms-r3-arithmetic.log` |
| `check-r2-algorithms.py` | 0 mismatches (exit 0). Log `final-verify/check-r2-algorithms.log` |
| `check-algorithms-narration.py` | 43 clauses, 0 textual problems, 0 failed facts. It does not test the "jetzt" state (A-B1) or the valley-floor wording beyond the earlier fix |

## 4. Blockers and recomputed evidence

- **P-B1 (pareto third, third_2, third_3, third_4; en and de) DF equation does not evaluate as printed.** Printed: `DF = 0.6 · 7.2 m² · 75° / 180 m² · 0.75 = 2.4 %`. Read left to right it gives 1.35 %. The third_2 version (19.2 m²) gives 3.6 %, not 6.4 %. The value 2.4 and 6.4 hold only with the denominator bracketed, `/ (180 m² · 0.75)`. Fix: print `(180 m² · 0.75)` (German: `(180 m² · 0,75)`), or print the room surfaces as 135 m².
- **P-B2 (pareto walk_2, walk_3, payback, payback_2; en and de; knee_2, knee_3 de reader, knee_3 en reader) Δ rows do not recompute.** The Heat row shows 97.7 then 97.6 with "±0.0" (the same model heat, 97.649 kWh/a, both sides; the printed value changes but the Δ does not). The Total row prints -64.9 where 583.9 - 648.9 = -65.0. In knee_3, Light prints -57.4 where 240.9 - 298.2 = -57.3, and Total prints -50.9 where 533.1 - 583.9 = -50.8. Fix: take each Δ from the printed one-decimal totals (or print every row at the same precision), and make the Heat row identical in both frames when the model is unchanged.

Algorithms blocker:

- **A-B1 (algorithms pattern_5; en and de) state mixes before and after a move.** The chip and dot say 75.0 % · 1.12 m and the bar says 497 kWh/a, which is the destination. The text "now 497.8 kWh/a" (de: "jetzt 497,8") is the pre-move centre, which is drawn at 1.16 m and is never shown; 497.8 would also print as 498. Fix: show the centre with its value before the move, label the destination "afterwards", and keep chip, bar and room on the same state.

## 5. Majors

Pareto:

- **P-M1 (all keys, en and de) same quantity, two precisions in one frame.** Bar label integer (for example "649 kWh/a") against legend "Total 648.9 kWh/a"; room_2 "11 kWh/m²a" against "11.1" in the derivation. The coordinator's rule is one precision per quantity per frame. Affected keys (reader lists): room_2, objectives, objectives_2, grid, grid_2, extremes, dominated, dominated_2, dominated_3, rule, rule_2, front, front_2, approximate, sets, walk, walk_2, walk_3, payback, payback_2, knee, knee_2, knee_3, third, third_2, third_3, third_4, weights, weights_2, decision_2. Fix: print the bar total at one decimal wherever the legend uses one.
- **P-M2 (knee, en and de) step Δ shown two ways.** Legend total change -0.3 kWh/a against the equation "0.25 kWh/a × 0.25 CHF/kWh". Fix: show the step change at two decimals in the Δ column, or use the same precision in both places.
- **P-M3 (knee, knee_2, knee_3, payback_2; de) lifetime label overprints the x tick labels.** The green "30 a Lebensdauer" sits on the tick labels 20 and 30. Also the English tick reads "30 years" on payback and "30 a lifetime" on payback_2 and knee (unit inconsistent). Fix: place the lifetime label above the plot or beside the dashed line; use "a" in both languages; label every tick.
- **P-M4 (objectives to sets, all keys with the legend column; en and de) legend column touches the room wall.** The "Light/Licht" line starts at x about 1041, where the room's bottom-right corner is at about 1045, y 352. Fix: move the legend about 8 px down, or move the room about 10 px left.
- **P-M5 (third_2, third_3; en and de) dashed front guide under the legend title.** The guide at y about 229 runs under "Tile colour: daylight factor" / "Kachelfarbe: Tageslichtquotient" (the first-round M1 residual). In de third_3 the "Front 30 von 36" box touches the legend title. Fix: stop the guide at the legend's left edge or move the legend up; add 6 px spacing.

Algorithms:

- **A-M1 (pattern_3; en and de) "now 744.7" is the pre-move value** next to a chip, room and bar (649) already at the destination; bar 649 against text 648.8. Fix: write "was 744.7, moved to 648.8" and print one precision.
- **A-M2 (gradient_3; en and de) slope arithmetic and undefined symbol.** The equation uses 743.053 and 746.327, but the cards show 743.1 and 746.3; from the cards the slope is -133, not -136. The step rule "Schritt = -η · Steigung" never defines η. Fix: print the cards with three decimals (or use card values in the equation), state the rule's η, and print the result that follows from the printed values.
- **A-M3 (gradient_4, overshoot; en and de) quotient is truncated.** "165.1 ÷ COP 3.5 ≈ 47.1": 165.1 ÷ 3.5 = 47.17, which rounds to 47.2. The total "744.7 - 132.0 = 612.7" is built on 47.1. The ≈ policy accepts last-digit differences, but 0.07 is a truncation, not an approximation. The DE reader rated this a blocker. Fix: print 47.2 and make the total follow (612.8) or print a consistent total on the bar.
- **A-M4 (crossover, crossover_2; en) bar shows an unnamed breakdown.** The bar reads 643 kWh/a with "107.7 + 174.9 + 360.0 = 642.6", equal to parent A, while the frame says the child is "not simulated yet". Fix: label the bar as parent A, or hide it; use one precision.
- **A-M5 (population; en and de) stale chip.** "windows 75.0 % · overhang 1.12 m" stays on screen with "0 of 50 simulations" and no current design dot. Fix: hide until a genetic design is simulated.
- **A-M6 (close; en and de) unexplained façade cost.** "façade cost 21,260 CHF" appears nowhere in the narration or the derivations. The value is the model cost of the map minimum design, so it is correct, but it is unexplained. Fix: remove it, or introduce it with its meaning.
- **A-M7 (gradient_2, gradient_3; en) "× 11" magnifier label has no object.** Fix: "zoom ×11" or remove.
- **A-M8 (task_2; repeats in map_2, gradient_4, overshoot_2, gradient_result(_2), pattern_result, confirm; en) lamp factor unnamed.** "Lamps … · 18.090 %" has no name; later rows carry 28.028 %, 23.644 %, 31.673 %, 29.284 %, 25.920 %. Fix: name it once, for example "lamps on 18.090 % of hours".
- **A-M9 (pattern_2, pattern_3, pattern_5; en) "limit: no move" on moved probes.** "windows +5 %" (80.0 %) and "overhang -0.10 m" (0.00 m) are at the bound but were simulated, so "no move" is wrong. Fix: "at limit 80 %" / "at limit 0.00 m"; reserve "no move" for clipped zero displacement.
- **A-M10 (compare_2, pattern_result, stopping_2, proof, population_2; de) number split from unit at a caption line break.** Examples: "3,3 / kWh/a", "nach 16 / Simulationen", "mit 1.681 / Simulationen". Fix: no-break spaces (U+00A0) between number and unit in captions (the shared wrapper splits on U+00A0, see toolkit-r2 section 8).
- **A-M11 (all kWh/a bar labels; en and de) systemic precision.** Bar integer against ledger or legend one-decimal values in about 20 frames (for example "745" against 744.7, "613" against 612.7, "496" against 496.4, "643" against 642.6, "548" against 548.3). Same fix as P-M1.

## 6. Minors (short list)

Pareto (en and de unless noted):

- P-m1 payback: "65 kWh/a × 0.25 CHF/kWh ≈ 16.2" (16.25 should print 16.3 half-up); knee_2 "51 × 0.25 ≈ 12.7" (12.75 → 12.8).
- P-m2 payback: the payback column "1" over an underlined "12" reads as a fraction; add step labels.
- P-m3 approximate: caption "dominates a front member" against callout "beats 2 front façades"; delta table "d 0.60 → 0.99 m" against card "d 0.6 m"; print "d 0.6 → 0.99 m".
- P-m4 third_3, third_4: the ring on the "Front 30 of 36" design is teal, while the legend says green.
- P-m5 bar intensity (room_2 "11", derivation "11,1") — covered by P-M1; listed for completeness.
- P-m6 grid overlay box "6 window shares × 3 insulations × 2 overhangs" nearly touches the "Electricity" bracket (box top about 6 px higher than needed).
- P-m7 front: legend "grey: a cheaper façade is better" is ambiguous ("beaten by a cheaper façade"); the orange cost line at the axis end is unlabelled.
- P-m8 extremes: the green X marks the unattainable best corner (19,210 CHF, 520 kWh/a), labelled "no façade"; label it "best of both (not a façade)".
- P-m9 room: the "80 %" WWR entry ends about 5 px before the "d 0.6 m" tag on the same row; move the tag about 10 px right.
- P-m10 de third, third_2: English "DF" abbreviation in the German frame; use "TQ" (Tageslichtquotient).
- P-m11 de extremes header "sparsamste minus günstigste" (lowercase nouns) → "Sparsamste minus Günstigste".
- P-m12 de rule_2, third_2, third_4: caption line breaks leave orphans ("beiden.", "Strom.", "Zielgröße."); rebalance.
- P-m13 de objectives_2: the "besser" arrow crosses the dashed cost guide; shorten it.
- P-m14 de third_2 / third_3 / rule and rule_2: the German "Vergleich" box and the legend sit close to the room's 6 m dimension; spacing.

Algorithms (en and de unless noted):

- A-m1 pattern_result: caption "after 16 simulations" while the frame counter reads 49 and the 16-run point is not drawn; the chip "within 1 % after 16" is present. Draw the 16-run point or label it.
- A-m2 stopping_2 (de reader): caption says the genetic run "stopped after 15", but its curve still runs to 50; dim the curve after 15.
- A-m3 mutation, mutation_2, crossover (de): the what-if values 583.0 and 629.2 are for designs never simulated; mark them counterfactual.
- A-m4 compare, compare_2, stopping (de): "bis 499,8 kWh/a" is 494.8 × 1.01 = 499.75 (rounds to 499.7 or 499.8 depending on rounding); state the threshold as 1 % of 494.8.
- A-m5 mutation, mutation_2 (de): "Fenster ±10 % , Vordach ±0,2 m" has spaces before commas (crossover: "[Fenster von B , Vordach von A ]").
- A-m6 mutation, mutation_2: the "mutate" box touches the bottom edges of the result cards (crop checked by the reader).
- A-m7 children, survival: green frame on the best card is unexplained (en).
- A-m8 generations, genetic_result and children: chip "56.6 %" against card "57 %", "35.9 %" against "36 %" (one precision).
- A-m9 population_2, population_3, tournament, tournament_2: chip "26.7 %" against card "27 %".
- A-m10 gradient_result, pattern_result, close: chip "61.0 %" / "75.0 %" / "66.0 %" against captions "61 %" / "75 %" / "66 %".
- A-m11 Green star (map minimum) is unlabelled in several frames (gradient_result(_2), pattern_result, genetic_result(_2), proof); label "map minimum".
- A-m12 overshoot_2: "Change +6 kWh/a" has no reference value or design.
- A-m13 task (en): bar "745 kWh/a" appears before its parts (task_2); label it or delay.
- A-m14 stopping (en): unlabelled orange and red brackets; counter "50 of 50" while the runs stop at 15, 21 and 23; relabel the counter as budget.

## 7. Prior blockers and majors: status

Pareto round-2 verifier (`verify-r2-pareto.md`):

- M1 (rays through chips): resolved. The legend and guide collision remains (P-M5), so M1's second location is open.
- m1 (bar-legend deltas): open, now as P-B2.
- m2 (d 0.99 precision): resolved.
- m3 (payback axis unit and lifetime label): partly. Unit and "30 a" are present, but the label overprints the ticks (P-M3).
- m4 (intensity area 48 m²): resolved in room_2; bar precision remains (P-M1).
- m5 (symbol explanations): resolved (card explains WWR, t, d).
- m6 (number-unit wrap): resolved for the numbers; orphans remain (P-m12).
- m7 (margins): resolved.
- m8 (grid tile clusters): not flagged by any reader.
- m9 (quadrant chip): resolved.
- m10 (delta column header): resolved.

Algorithms round-2 verifier (`verify-r2-algorithms.md`):

- V1 (ledger arithmetic, blocker): partly. Rows now print one decimal and most close; gradient_4 and overshoot still truncate 47.2 to 47.1 (A-M3).
- V2 (slope rows): partly. Precision mismatch between cards and equation (A-M2).
- V3 (crossover what-if): resolved as a what-if; a new unnamed bar value remains (A-M4).
- V4 (compare live): resolved.
- V5 (glass loss bridge): resolved (loss, gains, demand shown).
- V6 (symbolic formulas): partly. Chips are symbolic, but η is undefined (A-M2).
- V7 (pattern_5 narration): narration fixed; the frame state is wrong (A-B1).
- V8 (nudge decimals): resolved.
- V9 to V20: resolved or minor as listed in section 6.

## 8. Clip verdicts

- **🏆️pareto: FAIL (en and de).** Two blockers: the DF equation as printed (P-B1) and the Δ rows (P-B2). Five majors, of which P-M1 (precision) is systemic. The remaining narration claims hold on the frames reviewed, and the numbers the readers recomputed (cost sums, ratios, payback, knee, frontier) hold as printed.
- **🧬️algorithms: FAIL (en and de).** One blocker: pattern_5 state (A-B1). Ten majors, of which A-M11 (precision) is systemic. The ledger rows, counters, tolerance claims and narration arithmetic otherwise hold as printed, and check-r2-algorithms and check-algorithms-narration pass.

Depth against the energy clips was not scored separately; the readers found derivations on screen comparable to the energy stills for pareto, and no depth defect was raised for algorithms beyond the symbolic formulas (V6 residual).

## 9. Files

- Report: `final-verify-pareto-algorithms.md` (this file)
- Check logs: `🗑️generated/final-verify/*.log`
- Crops by readers: `🗑️generated/final-verify/pareto-en`, `pareto-de`, `algo-en`, `algo-de`
- My first-pass notes (superseded; not evidence): `🗑️generated/final-verify/notes-*.txt`
- Crop helper: `crop-final.py` in the ticket folder
