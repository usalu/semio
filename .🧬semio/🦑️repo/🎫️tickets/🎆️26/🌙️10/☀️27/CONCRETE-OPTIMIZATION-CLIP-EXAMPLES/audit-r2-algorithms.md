# 🧬 Round 2 audit: Beat6_EvolutionarySearch (Algorithms and Evolutionary Search)

Read-only audit. Scope: all 40 English keyframes of the current `NARRATION` (`🧬️algorithms/🐍️.py`), 8 German keyframes (task, gradient_2, pattern_2, population_2, tournament_2, stopping, genetic_result_2, close), both layout reports, six energy benchmark stills (`🗑️generated/energy-frames/`), the model (`🧮️model/🐍️.py`, `🔣️.json`), and the earlier audit `audit-algorithms.md`. Numbers: `check-r2-algorithms.py`, output `🗑️generated/r2/check-r2-algorithms.out`. Frames: `🗑️generated/frames/Beat6_EvolutionarySearch/`.

## 1. Verdict

The clip is structurally sound and its numbers are real: 108 printed values reproduce exactly from the model, and the earlier audit's worst errors (room showing demand against bar electricity, "best" chips showing unsimulated values, tick labels off the frame, a German counter off by one) are fixed. It is still far from the owner's bar: the heating and lighting segments of the bar, and the cooling segment's flow, are not drawn in the room, the gradient probes and the later genetic child appear without any drawn design, and no beat writes the arithmetic that the energy clips show with live substitutions. Three beats are visibly broken (stopping labels overlap, crossover chips clip each other, the close frame is half empty), one shows a value before its simulation is counted (mutation_2), and the layout reports say "none" for all 80 frames, so they cannot be trusted.

## 2. Per-beat table (English keyframes, narration order)

On-screen values are as printed on the PNG. "Model" means the recomputed value (see section 3, N-items, and the check output).

| key | what is on screen | what the narration says | defects |
| --- | --- | --- | --- |
| task | Empty map axes (WWR 10–80 %, overhang 0–1.2 m). Room at 75 % / d 0.1 m, label "Sun 66°", "1 m" scale bar, "6 m". Bar 745 kWh/a (legend: Heating 35, Cooling 593, Lighting 117). Chip "start design". | Seminar room, south façade: which window share and overhang need least electricity? | B1 heating and lighting undrawn; B2 cooling arrows; N5 "1 m"; N6 "Sun 66°". |
| task_2 | Pointer ring at (75 %, 0.1 m). Tape with "1 of 50 simulations". Chip "Solver". | One simulation per design, budget 50 | None significant. Tape squares ~7 px tall. |
| map | Heat map with colour bar 495 … 846 (labels only at the ends, "kWh/a" between). Contours 550/650/750. Chip "1,681 of 1,681 simulations". The "750 kWh/a" contour chip sits on the start marker. Tape gone. | The full map needs 1,681 simulations; colour = kWh/a | L8 contour chip covers start marker; L9 colour bar; counter jumps to 1,681 (story). |
| map_2 | Star at (66 %, 0.99 m) with chip "map minimum 494.8 kWh/a". Room 66 %, "d 1.0 m". Bar 495 (40 / 273 / 182). | The valley floor is the best design: less window share, deeper overhang | N4 "d 1.0 m" vs 0.99 m; B1. |
| map_3 | Fog over the map; start design; "1 of 50". | In practice we see only simulated points | None significant. |
| gradient | Red dot at start; chip "Gradient descent"; "current design". | Gradient descent starts at 75 % and 0.1 m | None. |
| gradient_2 | Four probe chips under the room: windows 74.3 % · 741, 75.7 % · 748, overhang 0.088 m · 746, 0.112 m · 743. "5 of 50". Probes are **not** drawn on the map or in the room. Chips cover the "1 m" and "6 m" dimension labels. | Four close neighbours reveal the slope: five simulations per step | B3 probes undrawn; L3 chips over dimensions. Values verified. |
| gradient_3 | Slope chips "Slope windows 5.0 kWh/a per %", "Slope overhang −136 kWh/a per m". | More window share raises the electricity here, more overhang lowers it | B3 slope has no drawn meaning. Values verified (overhang slope from unrounded probes = −136.4). |
| gradient_4 | Red arrow from (75 %, 0.1 m) to (55 %, 0.26 m). Room 55 %, "d 0.3 m". Bar 613 (47 / 412 / 153). "Change −132 kWh/a". | The step goes downhill: the room follows, the bar shrinks | N4 room "d 0.3 m" for 0.26 m; L3; "−" hyphen-minus; B2. |
| overshoot | Dashed vertical line at 66 %. Chip "valley floor 66 % · 495 kWh/a" drawn in dim colour on the fog; the dashed line runs through its "kWh/a". | The first step overshoots the valley floor, to too small a window share | L5 chip contrast and line through text; overshoot not quantified (no electricity on the far side vs the floor). |
| overshoot_2 | Zigzag of ten iterates; counter "50 of 50"; room 58 %, "d 1.0 m"; bar 504 (46 / 253 / 205); "Change 6 kWh/a" in red. Probe chips at the last iterate. | Repeating, the path zigzags; some steps even raise the electricity | Values verified (change +6.5, slopes −1.6 and +59). Sign: "Change 6" lacks "+". L3 chips over dimensions. |
| gradient_result | Chips: "best windows 61 % · overhang 0.91 m", "496.4 kWh/a after 50 simulations", "map minimum 494.8 kWh/a · gap 1.6 kWh/a". Ring at (61 %, 0.91 m), star. Bar 496 (43 / 264 / 190). Room "d 0.9 m". | After 50 simulations, gradient descent's best is within 1 % | N2 segments 43+264+190 = 497 vs total 496; N4 "d 0.9 m" for 0.91 m; L3. Gap 1.6 verified. |
| gradient_result_2 | Tape regrouped: red "step" and grey "probe" squares with a two-item legend. | Four of every five simulations probe slopes; noise or a second valley can mislead them | None significant; squares 6 px. Legend now present (resolved). |
| pattern | Pattern search chip (orange). Faint gradient trail. Start. "1 of 50". | Pattern search needs no slope | None. |
| pattern_2 | Arms from (75 %, 0.1 m): 55 % (659), 80 % "edge" (770), 0.00 m "edge" (750), 0.40 m (649). Chip "Step 20 % windows · 0.30 m overhang". "5 of 50". | From the same start it tries four designs: four simulations | "edge" not explained (N7); L3 chips over dimensions. Values verified. |
| pattern_3 | Winner ring at 0.40 m (649, green chip). "Move to the best neighbour". Room "d 0.4 m". Bar 649 (35 / 485 / 129). | The best neighbour wins and the room moves there | L3. Values verified. |
| pattern_4 | Montage of rounds up to "49 of 50". Final step chip "2 % windows · 0.04 m overhang". Probes 72 %, 78 %, 1.12 m (winner, green), 1.20 m edge. Verdict "Move to the best neighbour". Room "d 1.1 m". | If no neighbour is better, halve the step; the steps shrink | N3 step 2.5 printed "2 %", probes 72.5 / 77.5 printed "72 %" / "78 %" (differences read as 3 %); S4 verdict shows "move", not "halve". |
| pattern_result | Chips "best windows 75 % · overhang 1.12 m", "496.7 kWh/a after 49 simulations", "gap 1.9". Star. Bar 497 (36 / 293 / 168). | Pattern search first reaches within 1 % after 16 simulations, with no slope | Values verified. "16" is not marked on this frame (only in compare_2). |
| population | Eight ghost cards (dashed thumbnails, no values). Chip "genes : window share · overhang". "best so far none yet". "0 of 50". | A genetic algorithm keeps eight designs; window share and overhang are its genes | Stray space before colon (E2). Cards show "10 % · 0.74 m" without gene names. |
| population_2 | Eight cards with thumbnails and kWh/a (810 … 587). Focus room (587 design) has no WWR or overhang tag. Bar 587 (96 / 184 / 307). Two chips "best so far 587 kWh/a" and "best so far" stacked. | Each card shows one design and its simulated electricity: eight simulations | L7 duplicate chips; L16 room unlabelled; L12 thumbnails too small to read the share. Values verified. |
| population_3 | Dashed hull around the eight map dots. | Spread over the map, they explore several regions | Hull is correct. Resolved from earlier audit. |
| tournament | Two duels: 643 (23 %, 0.69 m) beats 810 (cyan ring); 598 (30 %, 0.86 m) beats 649 (orange ring). "vs" labels. | Draw two cards; the one with less electricity becomes a parent | None. Values verified. |
| tournament_2 | Duel 649 (78 %, 0.45 m) beats 810 (10 %, 0.74 m). Chip "rank 5 of 8 beats rank 8". "8 of 50". | Good designs win more often, yet even a weak card can win a duel | S3 the winner (5th of 8) is not weak; "weak" is not shown. |
| crossover | Façade thumbnails of parents: "parent 643" (cyan) and "parent 598" (orange); gene chips, faded "windows 23.1 %" and "overhang 0.86 m" (not chosen). Child row: "overhang 0.69 m" (cyan) and "windows 30.3 %" (orange) with arrows. Bar 583. Child dot at (30.3 %, 0.69 m). | The child takes window share from one parent, overhang from the other | L2 cyan "overhang 0.69 m" clipped by orange "windows 30.3 %"; the child row duplicates the parent row; faded chips unexplained. |
| crossover_2 | Dotted parent box (only its top-left corner visible). Flash on child. | On the map the child sits at a corner of its parents' box | L15 box barely visible. Geometry correct. |
| mutation | Child moved to (31.1 %, 0.56 m). Chips "crossover only 583 kWh/a", "with mutation 563 kWh/a", "nudge +0.9 % · −0.13 m" (green). Bar 563 (84 / 193 / 287). | Mutation nudges the child a little; here the change helps | N2 segments 84+193+287 = 564 vs 563; L2 duplicate chips overlap; "crossover only" is a what-if, not a simulation (B4). |
| mutation_2 | Chips "later child: crossover only 629 kWh/a", "with mutation 648 kWh/a" (red), "nudge 0.0 % · −0.07 m". Counter reads "9 of 50". Room and map still show the generation-0 child. No later design drawn anywhere. | Later a nudge can also hurt: chance keeps variety | S2 values shown at run 9 but the child is simulation #18; B4 no design drawn; N7 "0.0 %". |
| children | Six new cards (563, 548, 557, 624, 581, 572) plus elites. "14 of 50", "Generation 1 of 7", bar 548 (73 / 228 / 247), chip "best so far 548 kWh/a". | Six children, one simulation each: six simulations per generation | Values verified. Earlier "best 563 beside 548" is resolved. L7 duplicate chips. |
| survival | Elites boxed yellow with stars: 598 (30 %, 0.86) and 587 (27 %, 0.47); best child boxed green (548). Chip "kept: best two parents of generation 0". | Elitism: the two best parents survive, so the best value never gets worse | Logic correct (the two best of generation 0 are 587 and 598). Stars overlap card corners (L11); cards unsorted. |
| generations | Eight-step montage ends at "Generation 7 of 7". Chip "best per generation 587 548 524 523 514 506 498 498 kWh/a" (no generation indices). Cloud of 50 dots spread over the map. Bar 498. | Repeat for seven generations: the cloud contracts and the best card improves in jumps | N8 unlabelled list; S5 cloud does not visibly contract (all tested dots stay at full opacity). Values verified. |
| genetic_result | Chips "best windows 57 % · overhang 0.87 m", "498.1 kWh/a after 50 simulations", "map minimum 494.8 kWh/a · gap 3.3". Star. Bar 498 (46 / 252 / 200). Cards still on screen. | After 50 simulations the best card is within 1 % of the map minimum | Best card not ringed (L11). Values verified. |
| genetic_result_2 | Three bar rows: "Gradient descent 41 simulations", "Pattern search 16 simulations", "Genetic algorithm 41 simulations" (bars to scale 50). Header "Simulations until within 1 % of the map minimum". | Within 1 %: pattern search needs 16 simulations, the others more than twice as many | Values verified (41, 16, 41; 41 > 2 × 16). Clean in EN. |
| compare | Curves of best-so-far vs simulations for three methods, map minimum dashed at 494.8. Right column: three static thumbnails (final designs), chips 496.4 / 496.7 / 498.1 with designs. "50 of 50". | Same model, same budget: the best electricity so far, plotted against simulations | B5 curves not linked to designs during the run; thumbnails show final designs only. |
| compare_2 | Reach markers: pattern at 16 (orange ring), gradient and genetic at 41 (red and beige rings). Gap chips 1.6 / 1.9 / 3.3 kWh/a at the right edge. | Pattern search gets there first; all three end within 3.3 kWh/a of the map minimum | Values verified. No 1 % band drawn (the 499.7 threshold is only implied). L14 gap chips at right margin. |
| stopping | Panel: "Budget 50 simulations", "Tolerance: gain below 1 kWh/a in 5 simulations", "Stagnation: no gain in 10 simulations". Badges "tolerance · 15 sim." (beige), "tolerance · 21 sim." (orange), "tolerance · 23 sim." (red). Dashed budget line at 50. | Stop at a budget, a tolerance or stagnation, and say which | L1 badge "tolerance · 15 sim." covered by the stagnation panel; S1 the rules fire at 15, 21, 23 while the clip reports 50-run results; no stagnation for genetic. |
| proof | Full map, rings for the three solvers, star. Track of 1,681 with sliver "50 simulations · 3 %" (3 % verified). Three room sections with "Gradient 496.4 kWh/a", "Pattern 496.7 kWh/a", "Genetic 498.1 kWh/a" and designs. | None of them proves the global optimum; only the full map could show it | Resolved from earlier audit (three rooms). "1 m" scale labels repeated (N5). |
| confirm | Room 61 %, "d 0.9 m" with stamps "model result" and "check with a finer simulation" (red). Bar 496 (43 / 264 / 190). Chip "winner". | Check the winner with a finer simulation, then judge it as architecture | S6 this "winner" (61 %, 0.91 m) differs from the answer in close (66 %, 0.99 m) without explanation; N2 segments sum 497 vs 496; N4. |
| tools | Two sliders at 75 % and 0.10 m, box "Annual simulation" (room), "745 kWh/a", "Solver: next design" loop. | Parametric tools wire this up: sliders, simulation, solver | Schematic. Values verified. "1 m" scale (N5). |
| tools_2 | Sliders at 57 % and 0.87 m, "498 kWh/a", chips "smooth model: gradient", "noisy model: pattern search", "jagged, many valleys: genetic". | Choose by smoothness, noise and simulation budget; a population copes with several valleys | Resolved from earlier audit (start design). Values verified. |
| close | Room 66 %, "d 1.0 m", bar 495 (40 / 273 / 182). Four chips under the room: "66 % · 0.99 m · 494.8 kWh/a · 21,260 CHF", "Gradient 61 % · 0.91 m · 496.4 · gap 1.6", "Pattern 75 % · 1.12 m · 496.7 · gap 1.9", "Genetic 57 % · 0.87 m · 498.1 · gap 3.3". Left 60 % of the stage empty; no map. | Answer: 66 % window share, 0.99 m overhang, 494.8 kWh/a; the architect judges the rest | L4 empty half and 12 px chips; N4 "d 1.0 m"; D2 CHF 21,260 not explained (B6). All values verified. |

## 3. Defect list

Severity: blocker = the owner's rule (every plotted value drawn by its meaning, numbers true) is broken; major = visible failure a reviewer would reject; minor = polish or wording.

### Abstract (plotted value without drawn physical meaning at the same moment)

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| B1 | blocker | task … close (room frames) | The heating segment (35 to 86 kWh/a) and the lighting segment (117 to 307 kWh/a) of the bar have no drawn counterpart. The room has no heat-loss arrows through glazing and wall. The three lamp glyphs are identical from start to optimum, so the lighting share never changes on screen. |
| B2 | major | task … close (room frames) | The cooling segment is drawn only by three teal flow arrows at the room's right side, identical in every frame (593 kWh/a at the start, 273 kWh/a at the optimum). The change is visible only through the sun rays. |
| B3 | major | gradient_2, gradient_3, gradient_4 | The four probe values (741, 748, 746, 743) and the slopes (5.0 per %, −136 per m) are printed without a probe point on the map, without a tangent or slope marker on the room, and without a formula. |
| B4 | major | mutation_2 (and crossover, mutation) | The later child (629 → 648 kWh/a) has no design drawn: no point on the map, no room, no thumbnail. "Crossover only" values (583, 629) are what-if designs that the solver never simulated, but they look like outcomes. |
| B5 | major | compare, compare_2 | The three curves are not linked to designs. Thumbnails at the right are the final designs from the start of the beat; nothing moves with the curve tip as the runs count up. |
| B6 | minor | close, stopping, overshoot | "21,260 CHF" has no stated meaning (façade cost, not annual). The 1 kWh/a tolerance threshold is printed but not drawn on the curve. "Sun 66°" is not explained (see N6). |

### Numbers (wrong, inconsistent, rounded so the arithmetic does not close)

Verified exactly (108 printed values match the model): start 75 % / 0.10 m / 745 (35 / 593 / 117); map minimum 66 % / 0.99 m / 494.8 (40 / 273 / 182), cost 21,260 CHF; 1,681 runs; reach 41 / 16 / 41; gradient probes, slopes 5.0 and −136, iterate 1 613 and change −132, final 496.4 at 61 % / 0.91 m, gap 1.6; pattern probes 659 / 770 / 750 / 649, final 496.7, gap 1.9; generation 0 values 810, 654, 649, 643, 598, 765, 587, 618; elites 587 and 598; best per generation 587 548 524 523 514 506 498 498; children 563, 548, 557, 624, 581, 572; genetic final 498.1, gap 3.3; crossover 583 and mutation 563 with nudge +0.9 % and −0.13 m; duel rank 5 of 8 beats rank 8; stopping at 23 / 21 / 15; sun altitude 66.0°.

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| N2 | minor | gradient_result, confirm; mutation | Printed segments do not add to the printed total. Gradient best: 43 + 264 + 190 = 497, total 496 (exact 496.38, parts 42.79 / 263.84 / 189.76). Mutated child: 84 + 193 + 287 = 564, total 563 (exact 563.21). |
| N3 | minor | pattern_4, pattern_2 | Half-even rounding: the final step 2.5 % prints as "2 %"; probes at 72.5 % and 77.5 % print as "72 %" and "78 %", so they read as −3 and +3 around 75 %, while the printed step says 2 %. |
| N4 | minor | map_2, gradient_result, gradient_4, pattern_4, confirm, close, overshoot_2 | Room labels use one decimal for overhang ("d 1.0 m" for 0.99, "d 0.9 m" for 0.91, "d 0.3 m" for 0.26, "d 1.1 m" for 1.12); chips use two decimals. |
| N5 | minor | every room frame | The "1 m" label under the room is a fixed scale bar (code: `("1 m", -2.6, -1.6)`), not a dimension of this room. It sits under "t 16 cm" and reads as wall thickness. |
| N6 | minor | task, all room frames | "Sun 66°" is the summer-noon solar altitude (66.04°, checked). Nothing says "altitude". |
| N7 | minor | pattern_2, pattern_4, mutation_2 | "edge" (clamped to the domain) is unexplained. "nudge 0.0 %" is an exact zero (no WWR move); the sign convention (no "+" for zero, ASCII hyphen-minus for "−") is inconsistent. |
| N8 | minor | generations | "best per generation" lists eight values without generation numbers. |
| N9 | minor | gradient_4, gradient_result | The gradient overhang slope cannot be recomputed from the printed probes at two decimals (rounded chips give about −125; model central difference from unrounded values gives −136.4). Not an error, but the arithmetic does not close for the reader. |

### Sync (narration vs picture, counters, sequence)

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| S1 | major | stopping (and genetic_result, compare_2) | The stop rules are applied after the fact. Under the printed rules the gradient would stop at 23 runs, pattern at 21, genetic at 15 (genetic stagnation never fires). The clip then reports 50-run results without saying that the runs continued for comparison. |
| S2 | major | mutation_2 | The counter reads "9 of 50" while the later child values (648, 629, nudge) belong to simulation #18 (generation 1 children are simulations 15 to 20). The chips show a result before it is counted. |
| S3 | minor | tournament_2 | Caption "even a weak card can win a duel"; the winner is 5th of 8 (649), the loser 8th of 8 (810). The picture does not show a weak card. |
| S4 | minor | pattern_4 | Caption describes "halve the step"; the verdict chip reads "Move to the best neighbour". The halving verdict exists in the code for stalled rounds but is not shown. |
| S5 | minor | generations | Caption "the cloud contracts" is not visible: earlier generations stay at full opacity. |
| S6 | minor | confirm → close | Confirm labels the 61 % / 0.91 m design as "winner" (best of three solvers); close answers 66 % / 0.99 m (full map). The switch is not announced. |

### Layout (overlaps, clipping, empty screen, text size)

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| L1 | major | stopping | The badge "tolerance · 15 sim." (x ≈ 172–315 px, y ≈ 295–322) is partly under the panel "Stagnation: no gain in 10 simulations" (x ≈ 295–590, y ≈ 277–308). "sim." is cut. Same in DE ("Toleranz · 15 Sim."). |
| L2 | major | crossover, mutation | The child chip "overhang 0.69 m" (cyan, x ≈ 703–850, y ≈ 305–335) is overdrawn by the orange chip "windows 30.3 %" (x ≈ 840–985). The child row (y ≈ 363–392) repeats the parent row, so the same values appear twice. |
| L3 | major | gradient_2, gradient_3, gradient_4, gradient_result, pattern_2, pattern_3, pattern_4, pattern_result | Probe chip rows start at y ≈ 416 px and cover the top of the room dimension labels "1 m" and "6 m" (y ≈ 405–417 px). |
| L4 | major | close | Left 60 % of the stage (x 0–600 px, y 130–600 px) is empty; the map with the three trails and the star is gone. The answer chips are 12–13 px. |
| L5 | major | overshoot | The valley-floor chip (x ≈ 228–476, y ≈ 226–258) is drawn at low contrast on the fog, and the dashed valley line (x ≈ 451) runs through "kWh/a". |
| L6 | major (DE) | genetic_result_2 (de) | "bisher beste 498 kWh/a" (x ≈ 938–1143, y ≈ 552–580) overlaps "bestes Ergebnis" (x ≈ 1135–1262, y ≈ 553–585). |
| L7 | minor | population_2 … mutation_2, generations, survival (en and de) | Two note chips in the same corner ("best so far 587 kWh/a" and "best so far" / "bisher beste") say the same thing. |
| L8 | minor | map | The contour chip "750 kWh/a" (x ≈ 477–573, y ≈ 470–498) sits on the start marker at (75 %, 0.1 m), hiding its digits. |
| L9 | minor | map and all fog frames | Colour bar shows only "495" and "846", "kWh/a" floats between them; no intermediate ticks, and contour labels 550/650/750 are the only scale. |
| L10 | minor | all frames | Chip and room-label text is 12–13 px at 1280 px (dimension labels "1 m", "6 m" about 10 px). Energy clips use larger label sizes. |
| L11 | minor | survival, genetic_result, population | Elite stars sit on the card corners; the best card is not ringed in genetic_result; cards are not ordered by kWh/a. |
| L12 | minor | population, population_2, crossover | Façade thumbnails (scale 0.13) are too small to read window share or overhang. |
| L13 | major (process) | every frame | Both layout reports list "none" for all 80 frames, while L1, L2, L3, L5, L6 are visible. The checker only tests tick labels and frame edges. |
| L14 | minor | compare, stopping (en and de) | Right-column chips reach x ≈ 1252–1262 px (German "Abstand 3,3 kWh/a"), beyond the safe right margin. |
| L15 | minor | crossover_2 | The dashed parent box is a dotted L (only its corner) and is hard to see. |
| L16 | minor | population_2, children | The focus room has no WWR or overhang tag, so the room cannot be matched to a card. |

### Depth (versus the energy clips)

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| D1 | major | gradient … close | No formula band with live substitution. The energy frames write the rule with coloured symbols and the numbers in place, for example "Φsolar = 400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W" and "Q_h,s = 1,5 kWh/d · 200 d = 300 kWh/a". This clip never writes the slope, the move rule, the crossover rule or the mutation rule. |
| D2 | major | task … close | The conversion from demand to electricity is hidden. The room says "Cooling → 593 kWh/a electricity", but not that the cooling demand is about 1,778 kWh/a and the chiller EER is 3, nor that heating demand is about 122 kWh/a divided by COP 3.5. |
| D3 | minor | gradient_4, pattern_3, mutation | The room is the only physical object, and its labels are small; no before/after comparison of the room at the same scale is shown. |

### German (de)

| ID | Sev. | Keys | Defect |
| --- | --- | --- | --- |
| E1 | major | genetic_result_2 | Overlap, see L6. |
| E2 | minor | population_2 (de), population (en) | Stray space before the colon: "Gene : Fensteranteil · Vordach"; English "genes : window share". |
| E3 | minor | close (de) | Short method names "Gradient 61 %", "Muster 75 %", "Genetisch 57 %" while the rest of the clip uses "Gradientenabstieg", "Mustersuche", "Genetischer Algorithmus". |
| E4 | minor | stopping (de) | "Toleranz · 15 Sim." clipped by the stagnation panel (same as L1). |
| E5 | ok | task, gradient_2, pattern_2, population_2, tournament_2, stopping, genetic_result_2, close | Decimal commas and dot thousands are correct ("494,8 kWh/a", "21.260 CHF", "1.681"). All eight German frames fit their chips; no caption truncates. |

### Status of the earlier audit (`audit-algorithms.md`)

| Earlier finding | Status now |
| --- | --- |
| Room "Cooling X kWh/a" showed demand, bar showed electricity | Resolved: room says "→ X kWh/a electricity" and matches the bar. |
| Chip "best 497" at 0 of 50 simulations | Resolved: "best so far none yet". |
| Chip "best 563" beside a card at 548 | Resolved: "best so far 548". |
| Elite stars "not the best of the pool" | Not a defect: elitism keeps the two best parents of generation 0 (587, 598), as the code does. The remaining point is that cards are unsorted (L11). |
| Y tick labels off the left frame edge | Resolved: ticks inside the frame. |
| DE counter "9 von 50" against EN "8 of 50" | Resolved: DE "8 von 50". |
| Range chip "later child 629 to 648" with no child and counter at 0 | Partly: values shown, but no design drawn and counter wrong (B4, S2). |
| "Weak card wins" shown as 598 vs 649 | Changed: now 649 beats 810 (rank 5 of 8). Caption still overstates (S3). |
| generations_2 missing history strip | Resolved: history chip exists (no bars, N8). |
| close repeats tools_2 | Resolved as a repeat, but close now has an empty left half (L4). |
| compare_2 identical to compare | Resolved: reach markers and gap chips added. No 1 % band (compare_2). |
| stopping rings unlabelled | Resolved: badges labelled; overlap added (L1). |
| gradient_result_2 has no legend | Resolved. |
| gradient_2 probes not drawn on the map | Open (B3). |
| pattern_2 printed step vs drawn arms | Partly: "edge" labels added; rounding (N3, N7). |
| pattern_4 step "10 %" vs probes | Resolved: step now "2 %"; rounding open (N3). |
| population gene names | Partly: gene chip added; cards still unnamed (L11). |
| population_3 hull | Resolved. |
| tournament backgrounds under duels | Resolved: grid hidden during duels. |
| mutation delta tags and rings | Partly: nudge chip present; no ring or colour delta on the child dot. |
| genetic_result_2 costs as text | Resolved: bars drawn. Overlap in DE (L6). |
| Gap arithmetic does not close | Resolved: gap computed from one-decimal values. |
| overshoot not quantified, chip on fog | Open (L5, and the far side is not quantified). |
| Bar segments unlabelled | Partly: legend gives values; no segment tags (N2). |
| Room label size 9–10 px | Open (L10). |
| proof has no rooms | Resolved: three room sections. |
| No formula band | Open (D1). |
| tools starts at a non-final design | Resolved. |
| Pattern trail drawn at full red | Resolved: faint ghost. |
| Crossover arrow over chip; dashed box barely visible | Open (L2, L15). |
| confirm stamp colour and winner size | Resolved (red stamp). |
| Overhang decimals differ across screens | Open (N4). |
| "Sun 66°" unexplained | Open (N6). |
| Map colour bar and contours | Open (L9). |
| task_2 caption "a few" vs "of 50" | Resolved. |
| Counter jumps to 1,681 and tape disappears | Open (story, minor). |
| Genetic green collides with "best" green | Resolved: genetic is beige. |
| Layout checker misses chip overlaps | Open and worse: reports "none" (L13). |

## 4. Fix proposals (implementable; coordinates are 1280 × 720 px of the keyframes)

**B1 (heating and lighting drawn in the room; blocker).** Bind three elements in `room_section` through ValueTrackers with `always_redraw`:
- Heating: heat-loss arrows leaving the glazing (x of the window glass) and the wall, count or width proportional to the heating demand (about 122 kWh/a at the start, `heating × COP`). Label beside the arrows: "heat ≈ 122 kWh/a ÷ COP 3.5 = 35 kWh/a" (values from the model: `heating_demand`, `systems.cop`).
- Lighting: the lamp glyphs' brightness follows `1 − autonomy` (start autonomy ≈ 0.82, optimum ≈ 0.72), and the daylit floor patch follows `autonomy`. Label: "648 kWh/a × (1 − 0.82 daylight) = 117 kWh/a" (648 = 9 W/m² × 48 m² × 1,500 h; compute from the model, do not type it).
- Keep the bar segment colours; put the segment value next to each drawn element.

**B2 (cooling flow).** Make the three teal flow arrows (right of the room, x ≈ 885–945 px) scale in length or count with `cooling_demand`, and label them "cooling demand 1,778 kWh/a ÷ EER 3 = 593 kWh/a" (start) so that 593 → 273 is visible. The arrows are constant now.

**B3 (probes and slopes).** On the heat map draw the four probe points as small crosses in the chip colour at their exact positions (74.3 % / 75.7 % at 0.1 m; 75 % at 0.088 m and 0.112 m), and draw a thin leader line from each chip to its cross. In the room, draw the window slope as a short tangent arrow along the window band; length ∝ slope. Add a formula strip (see D1) with the difference quotient, computed from the displayed probes at one decimal (see N9).

**B4 (later child, what-if values).** Move the later child into the sequence after its simulation is counted (see S2): show its cross on the map (dashed arrow from the crossover point, label "gen 2"), its thumbnail, and its room. Draw "crossover only" as a dotted outline labelled "what-if, not simulated".

**B5 (compare linked to designs).** Add one live marker per curve (ValueTracker on the run count) at the curve tip, with a room thumbnail of that method's best-so-far design driven by `self._at(designs)`. Shade the 1 % band between 494.8 and 499.7 kWh/a as a bar on the chart.

**B6 (cost, threshold, altitude).** Write the cost chip as "façade cost 21,260 CHF (model, not annual)" or drop it from close. Draw the stopping threshold as a bracket on the curve: "gain < 1 kWh/a over 5 runs". Rename "Sun 66°" to "summer noon altitude 66°".

**S1 (stop rules).** Label the three badges "would stop here (rule)" and add the caption line "runs continued to 50 for comparison". Alternative: choose stagnation of 25 runs so that the rules fire after the budget. The first is more honest.

**S2 (counter and simulation order).** Gate the later-child chips and the mutation_2 chips on `self.runs.get_value() >= born` (the code already has `self.d["born"]`), or move mutation_2 after the generation-1 children so that the counter reads 18 or more.

**S3 (tournament_2 caption).** Replace "even a weak card can win a duel" with "even a mid-ranked card can win a duel" (EN) and "auch eine mittlere Karte kann ein Duell gewinnen" (DE), since the picture shows 5 of 8.

**S4 (pattern_4 verdict).** Choose a stalled round (the first with `moved` False, in `_pattern` the `stall` index) for the verdict chip, so "No better neighbour: halve the step" is shown and the caption matches.

**S5 (generations cloud).** Fade the dots of earlier generations to opacity 0.2 when the generation counter moves on, so the remaining cloud visibly contracts. Or reword the caption to "the tested designs cluster as the best card improves in jumps".

**S6 (confirm → close).** Confirm chip: "best of three solvers". Close chip: "full map minimum (1,681 runs)". Add the caption-free link: a short arrow from the winner chip to the star.

**L1 (stopping overlap).** Move the three-rule panel into the empty band above the curves: x 150–600 px, y 215–295 px (values 800–900 kWh/a are empty there except the 810 start at x ≈ 90–120 px). Keep the badges where they are. DE gets the same placement.

**L2 (crossover and mutation chips).** Remove the repeated child row from the parent area; keep one child row below the parents' genes at y ≈ 430–470 px with its own arrows. Shorten the orange chip to "windows 30.3 %" with width 1.35 units, and leave 0.15 units between chips so no chip covers another.

**L3 (probe rows over dimension labels).** Start the probe and step chip rows at y ≥ 435 px (currently 416 px), or raise the room by 0.2 units (`ROOM_AT` y 0.42 → 0.62) and keep the dimension labels above the chips. Apply in gradient_2 to pattern_result.

**L4 (close frame).** Restore the map (left half) with the three trails and the star, as the storyboard says. Put the four answer chips at 16 px under the room and move them to x ≥ 6.3 (right column) if the map takes the left half.

**L5 (overshoot chip).** Give the valley chip an opaque dark background and raise it above the plot top (y ≈ 1.27 units). Place the dashed line behind it (lower z-index) or stop it at the chip. Alternatively drop the fog to 0.5 during this beat.

**L6 (DE genetic_result_2).** Shorten the DE note chip to "bisher 498 kWh/a", or move "bestes Ergebnis" under the bar at x ≈ 6.2 (same column as the note), so the two chips cannot touch.

**L7 (duplicate note chips).** Hide `self.note_chip` whenever a value chip ("best so far …") is shown in the same corner; keep one.

**L8 (contour chip on the start marker).** Move the 750 contour label along its contour (for example to the lower-middle, x ≈ 60 %, y ≈ 0.25) or lower its z-index below the start dot.

**L9 (colour bar).** Add ticks at 500, 600, 700, 800 kWh/a under the bar, label "kWh/a" at the right end, and keep the three contour labels.

**L10 (text size).** Use the energy toolkit's label size (the room's `label_size(scale)`) for every chip and dimension label; enlarge dimension labels from about 10 px to 14 px.

**L11 (cards).** Sort card slots by electricity within each generation (or keep slots and show "rank k of 8" in the card). Put elite stars on the card's top-left inside corner. Ring the best card in `genetic_result`.

**L12 (thumbnails).** Raise thumbnail scale from 0.13 to about 0.19 in population and crossover, or add the WWR figure in the thumbnail's caption.

**L13 (layout checker).** Extend `layout_conflicts` to test every pair of text groups (chips, dimension labels, panels, captions) by bounding box, and report intersections of chips with other chips and with room labels. Add these as regression cases: stopping badge vs panel, crossover chips, DE genetic_result_2, gradient_2 probe chips vs "1 m"/"6 m".

**L14 (right margin).** Move right-column chips in compare_2 and stopping 0.15 units left (x 6.15 → 6.00).

**L15 (crossover box).** Draw the dashed parent box as a full rectangle (`DashedVMobject` of a closed polygon) with stroke 2.5 pt.

**L16 (focus room tag).** Add a chip under the focus room "27 % · 0.47 m · 587 kWh/a" in population_2 and children, the same design as the best card.

**D1 (formula band).** Add one formula strip above the caption band (y ≈ −1.85 units, centred, same style as the energy math panel) per method, with live numbers:
- gradient: "slope = [E(75.7 %) − E(74.3 %)] / 1.4 %-pt = (748 − 741) / 1.4 = 5.0 kWh/a per %";
- pattern: "E(0.40 m) = 649 < E(0.10 m) = 745 → move; no better neighbour → step ÷ 2";
- genetic: "child = windows of parent B (30.3 %) + overhang of parent A (0.69 m); mutation +0.9 % · −0.13 m: 583 → 563 kWh/a".
Use values from the model; show probes at one decimal so the slope recomputes on screen (N9).

**D2 (demand → electricity).** In the room labels write the division: "cooling demand ≈ 1,778 kWh/a ÷ EER 3 = 593 kWh/a" and "heating demand ≈ 122 kWh/a ÷ COP 3.5 = 35 kWh/a" (start values; computed from the model at run time).

**D3 (before/after room).** In gradient_result and pattern_result, show the start room and the final room side by side at the same scale (the proof frame already has three rooms).

**N2 (segment sums).** Print segment values with one decimal in the bar legend, or print the total as the sum of the printed integer segments, so that 43 + 264 + 190 = 497 is the total shown.

**N3 (step and probe rounding).** Print WWR step and probes with one decimal ("2.5 %", "72.5 %", "77.5 %") in the pattern chips.

**N4 (overhang precision).** Use two decimals in all room labels ("d 0.99 m", "d 0.91 m", "d 0.26 m", "d 1.12 m") to match the chips.

**N5 (1 m scale).** Either label the bar "1 m scale" or remove it from the search beats; if kept, draw it outside the façade under "t 16 cm" with a leader.

**N6 (sun).** "summer noon altitude 66°".

**N7 (signs and "edge").** Print a zero WWR nudge as "no change" and use the typographic minus "−" for negative values; explain "edge" as "limited to 80 %" / "limited to 0 m" in the chip.

**N8 (generation list).** Label the list "gen 0 … 7" or draw the eight values as a step strip with bars under the bar chart.

**N9 (gradient slope arithmetic).** Print probes at one decimal and compute the printed slope from those printed probes (the model's central difference is the same formula on unrounded values, so this changes the number by under 2 %; document it in the note).

**E2 (colon).** "Gene:" and "genes:" (no space before the colon in either language).

**E3 (close names).** Use the full method names in close (DE: "Gradientenabstieg", "Mustersuche", "Genetischer Algorithmus"), with two lines per chip if the width is too small.

**E4.** Same fix as L1.

## 5. Checks run

- `check-r2-algorithms.py` (stdlib only, run with `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`): 108 printed values match the model. Mismatches: the later child shown at run 9 but simulated at run 18 (S2); printed segments 497 vs 496 and 564 vs 563 (N2). The earlier "mismatch" lines for generation-1 children were a script error (I mapped the wrong generation) and are corrected in the script.
- The layout reports (`Beat6_EvolutionarySearch-en-layout.md`, `-de-layout.md`) say "none" for every key; they were not trusted (L13).
- The energy stills used as the depth benchmark: `Beat1_LeistungUndEnergie-summe`, `Beat5_Verschattung-raffstore`, `Beat8_Hauptgleichung-chain`, `AnlagenVerluste-sum`, `Beat1_LastZuerstSenken-reserve`, `Beat3_IsolateAirflow-compare`. I did not read the energy clip sources line by line; the depth points are from the frames.
