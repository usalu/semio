# Scene report: Pareto Optimality (`🏆️pareto`, `Beat5_ParetoFront`)

Owned file: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🏆️pareto/🐍️.py`. Class, `TITLE` ("Pareto Optimality" / "Pareto-Optimalität") and `construct()` → `self.present("pareto")` kept; `present_pareto` and helpers live in the scene file. Storyboard: `storyboard-pareto.md`; audit: `audit-pareto.md`.

## What the scene shows (32 clauses, about 4 min)

Stage: plot left (cost CHF across, electricity kWh/a up, ranges derived from the 36 grid designs), room section + stacked electricity bar right, a **reading band** below the room (x −0.85 … 7.1, y −1.0 … −2.5) that always carries the concrete façades of the current clause. One shared tracker set (`s.focus`) drives the room, the bar, the cost chip, the live cursor and a halo ring on the plot (drawn above the tiles), so a dot and its façade are always the same design.

| Act | Keys | Content |
| --- | --- | --- |
| A | `room`–`extremes` | big room with its own chips, cost chip and bar; axes appear; cursor; 36 dots with the level chip "6 WWR × 3 t × 2 d = 36 designs" while the room morphs through designs; hover on max-cooling / max-daylight / max-lighting; **extremes card**: cheapest and most frugal façade as thumbnails with cost and kWh/a plus the gap "+2,342 CHF / −129 kWh/a" computed from the model |
| B | `dominated`–`rule_2` | pair card (loser and winner thumbnails, the variables that differ in their colour: WWR 70 → 30 %, t 30 → 12 cm), quadrant with ΔCHF / ΔkWh/a legs, numbers; loser greys with a cross; check rows with the pair's numbers and the signed strict differences "−2,486 CHF · −75 kWh/a" |
| C | `front`–`sets` | cost sweep with running-minimum staircase and a three-line key (orange line, green steps, grey = a cheaper façade is better); front guide and the tile-colour legend (units on both ends); `approximate`: ghost design with its own numbers, "beats 5 front façades", pair card refinement vs the member it beats and two check rows (cost <, electricity <); `sets`: design space / objective space labels plus an "evaluation" arrow from the room to the plot |
| D | `walk`–`knee_3` | **live step card**: previous and next front façade side by side with "Step n of 7", changed variable in colour (d 0.0 → 0.6 m), cost and kWh/a; **in the room** a dashed outline of the target element (slab, window opening or insulation layer), a grey outline of the previous one and a delta chip "d 0.0 → 0.6 m" / "WWR 30 → 40 %" / "t 12 → 20 cm" with leader; **bar legend** shows the live kWh/a per segment and, while a step is walked, the signed change per segment (step 1: lighting +57, cooling −122, heating 0 = −65 kWh/a); chips extra cost / saved / price "3.0 CHF per kWh/a saved"; payback chart (x label "front step", ">90" for capped bars, lifetime line), tariff chip, knee ring with leader; tariff animates 0.25 → 0.40 CHF/kWh and the knee moves |
| E | `third`–`third_4` | dot colour = daylight factor (legend in %, units on both ends); room chips "Daylight factor 6.4 %" and "Lamps on 17 % of the time" from `daylight` and `autonomy` (toolkit lamps and daylight zone follow the same value); `third_2`: loser and dominator as pair card with check rows `21,346 > 19,210 CHF ✗`, `776 > 649 kWh/a ✗`, `6.4 > 2.4 % ✓` that stay through `third_3` (chip "Front 2 goals 8 → 3 goals 30") and are indicated one by one in `third_4` |
| F | `weights`–`decision_2` | weight slider, iso-score line (labelled "orange line: equal weighted score"), pick from `weighted_pick`; closing cards (thumbnails at scale .28) for cheapest, knee at the model tariff (title built from `pareto()["tariff"]`, plus "lifetime 30 a") and balanced weights with leaders to the plot |

Narration change: `walk_2` now names the cause ("Step one: a 0.6 m overhang costs 192 CHF and saves 65 kWh/a a year."); `check-pareto-narration.py` asserts that the first step adds exactly that overhang and changes nothing else.

## Narration audit (`audit-narration.md`) and refinement

All 41 pareto rows were checked against the model and the picture and applied, with these decisions: ß kept (`größeren`, `Zielgrößen`), "Vordach" everywhere (narration and on screen, no "Dachüberstand" left), "simulieren/Simulation", "Strombedarf" (axis title, check rows, slider, legend titles), "Fassadenkosten", "Zielgröße/objective" for the three objectives, kWh/a without "every year". `walk_2` first step verified: front[0] → front[1] changes only the overhang 0 → 0.6 m (WWR 30 %, t 12 cm unchanged), asserted by the check script. Not adopted literally: `decision_2` (auditor: "you decide") became a concrete answer shown in the room, see below.

- Opening: `room` is now the question "Which window share, insulation and overhang suit this seminar room's glazed façade?" with the baseline room on screen.
- Closing: `decision` says what the front does and who chooses (tariff, lifetime, weights); `decision_2` brings the room back at the knee design and answers "For this room, the knee: 30 % window share, 0.6 m overhang, 12 cm insulation at 0.25 CHF/kWh" (numbers asserted against `pareto()["knee"]` and the tariff by the check script).
- `front_2`: "Pareto front of the simulated designs" in narration and as a live label with the model's design count ("Pareto front of the 36 simulated designs").
- `approximate` (refinement from `T/model-refinement.md`): off the grid a deeper overhang dominates a front member. On screen: ghost tile at 21,260 CHF / 495 kWh/a labelled "optimum a search finds (algorithms clip)", "beats 2 front façades" (model `dominates`), the room at WWR 66 %, d 1.0 m, t 16 cm with rays stopped at the deep slab, pair card (front member WWR 50 % / d 0.6 m / t 30 cm vs the refinement, changed variables in colour), check rows `21,260 < 21,552 CHF`, `495 < 520 kWh/a` and the live gap "−292 CHF · −25 kWh/a". The narration holds no refinement number; the check script asserts that the refinement overhang is deeper than every simulated design and that it dominates its member.

## This pass: audit items

| Audit item | Verdict |
| --- | --- |
| third / third_2 daylight never reaches the room | Real. Lamp contrast and a daylight zone now come from the toolkit (`room_section`, coordinator decision); the scene adds the model-driven chips "Daylight factor" and "Lamps on … % of the time" plus the daylight legend. |
| walk_2 does not mark what changed | Real. Delta chip, target and previous outline in the room, live step card with changed variable in colour, per-segment deltas in the bar legend. |
| German "vs" in third_2 | Real. Rows are now language-neutral `a > b unit` strings. |
| third_2 dominator not on screen | Real. Pair card with both façades. |
| third_4 identical to third_3 | Real. Rows stay on screen and are indicated in turn. |
| Dimension labels struck by rows, chart title, slider labels, thumbnails | Real. Whole reading band re-laid out below y −0.9; chart moved right, slider moved down. |
| rule_2 shows no numbers | Partly. Signed differences added. A tied pair ("strict in one, equal in the other") does not exist in the 36 designs, so the "at least one" distinction is told by the words only. |
| payback axis unlabeled, title collides | Real. x label "front step", title and tariff chip moved, long years shown as ">90". |
| colour-ramp legends without units | Real. Own `_legend` with units on both ends (the toolkit `ramp_strip` puts the unit in the middle). |
| decision card hard-codes the tariff | Real. Built from the model tariff; lifetime line added. |
| approximate shows only the cost chip | Real. Readout, "beats N", pair card and check rows. |
| extremes shows only the frugal room | Real. Extremes card with the gap. |
| grid cursor hidden under tile | Real. Halo ring at z-index 10; tiles shrunk from .07 to .064. |
| bar shows only the total | Real. Live legend with kWh/a per segment and deltas. |
| "Sun 66°" is a fixed summer sun | Toolkit label (no season parameter). Toolkit request below. |
| "kWh/(m²·a)" brackets | The string is `kWh/(m²·a)`; the font draws the parentheses like brackets and the bar label is toolkit text, not a scene string. |
| objectives_2 chip on the axis | Chip is the toolkit `live_marker` readout; request below. |
| grid tile clutter | Tile size reduced; not spread with leaders (would hide the plot structure, the dots are positions). |
| front staircase unlabeled | Real. Three-line key in the reading band. |
| weights iso-line unlabeled | Real. Note beside the slider. |
| sets labels not attached to room parts | Partly. The design-space label stays under the room; an "evaluation" arrow from room to plot was added instead of three labels (the room chips already name window, overhang, insulation). |
| decision leaders cross the plot | Rejected: the cards sit on the right on purpose, leaders are thin and keep the link to the dot. |
| walk insulation layer barely visible | Toolkit drawing (insulation thickness ×1 in section); the walk now outlines the changed layer and prints "t 12 → 20 cm". |
| layout check blind to dimension labels | Toolkit/capture tooling; request below. |

## Model-driven contract

- No ids, ranges or numbers are typed in the drawing code: axes ranges, ticks, dots, level counts, dominated pair, steps, knee (`s.knee(tariff)`), refinement, third example, `front3`, picks, segment kWh/a, deltas, daylight factor and lamp share all come from `pareto()`, `weighted_pick()` and `evaluate()` through the toolkit.
- `NARRATION` is a literal list of `[key, en, de]`. Only few numbers appear in it (`walk_2`: 0.6 m, 192 CHF, 65 kWh/a; `walk_3`: 3.0 CHF per kWh/a; `payback`: 0.25 CHF/kWh, 12 years; `payback_2`: 30 years). `T/check-pareto-narration.py` verifies them and the qualitative claims (19 checks, 0 failures).

## Toolkit requests

1. `room_section(..., season_label=True)`: print "Summer solstice, noon" / "Sommersonnenwende, mittags" next to the sun label so that bars with heating are not read as winter sun.
2. `live_marker(readout=...)`: an option to keep the readout chip off the y axis line (it touches the axis at the far left).
3. `layout_conflicts`: include the "1 m" / "6 m" dimension labels of `room_section` in the overlap check (they were struck by rows in the previous version without being reported).
4. A generic live thumbnail pair / change overlay would serve the other clips as well (`_pair_card`, `_changes` in this scene file are the reference implementation).

## Verification

See the end of this file (filled after the final capture, smoke test and unit tests).

## Notes

- Used toolkit widgets: `plot_axes`, `live_marker`, `study_trackers`, `set_design`, `room_section`, `energy_bars`, `thumbnail`, `ramp`, plus `_room` helpers (`chip`, `text`, `place`, `parts`, `Number`, `result`, `values`, `dashed`, `design_key`). Built in the scene: sweep and staircase, quadrant, delta legs, check rows, pair card, change overlay, bar legend, tile legend, payback chart, weight slider, iso line, knee ring, daylight dot encoding.
- Live groups use `key` memoization (`s.index`, design key) so unchanged frames are not rebuilt.
- Temporary helpers in `T`: `pareto-dump3.py` (model dump), `check-pareto-narration.py` and `check-pareto-captions.py` (kept as contracts), `pareto-scene-before.py.txt` (scene before this pass).

## Round 2

Audit `audit-r2-pareto.md` (D01–D37) worked through; contract script `check-pareto-r2.py` (all claims and derived numbers against the model, 0 failures); keyframes `capture.py` en and de, strict layout check 0 findings in 32 en and 32 de frames, all frames viewed.

### Changes

- Claims made true: `knee_2` "the next step would take about 34 years to pay back" (33.6 a > 30 a), `knee` "each step costs more per kWh/a saved" (ratio 3.0, 8.4, 77, 84, 156, 241, 670 strictly rising; savings per step are not monotone), `knee_3` "at 0.40 CHF per kWh that step repays in 21 years", `grid` "six window shares, three insulation thicknesses and two overhang depths", `rule_2` "strictly better in at least one; here, in both", `approximate` "a bigger window shaded by a deeper overhang, paid for by thinner insulation", `payback` "about 12 years". Numbers in captions are glued with U+00A0.
- Colour ramps use the range of the 36 plotted designs: 520 … 776 kWh/a and 1.8 … 6.4 %.
- Bar legend (name, kWh/a with one decimal, signed change per step, total row) uses `_room.apportion`: parts add to the total (723.7 = 121.1 + 568.3 + 34.3), the deltas add to the total change.
- Dominance attribution: `approximate` walks the room from the beaten front member to the off-grid design, one variable at a time, with live change chip and rows `d 0.60 → 0.99 m +124.8 CHF −5.4 kWh/a`, `WWR 50 → 66 % +268.8 CHF −21.4 kWh/a`, `t 30 → 16 cm −685.4 CHF +1.8 kWh/a`, `net −291.8 CHF −25.0 kWh/a`.
- Cost anatomy in `room_2` (fixed 9,000 + wall 4,320 + insulation 1,037 + glass 5,280 + overhang 192 = 19,829 CHF, from the study prices) and the design choices (levels) in `room`.
- Derivations on screen: gaps with arithmetic (`21,552 − 19,210 = +2,342 CHF`, `520 − 649 = −129 kWh/a`, dominance legs), step equations (`19,402 − 19,210 = +192 CHF`, `649 − 584 = 65 kWh/a`, `192 CHF ÷ 65 kWh/a ≈ 3.0 CHF per kWh/a`), payback (`65 kWh/a × 0.25 CHF/kWh ≈ 16.2 CHF/a`, `192 CHF ÷ 16.2 CHF/a ≈ 12 a`, `12 a ≤ 30 a lifetime`), daylight factor through the toolkit `formula` (`DF = 0.6 · 7.2 m² · 75° / (180 m² · 0.75) = 2.4 %`, live with the room), weights (`1 kWh/a is worth 18.2 CHF`, price of the step into the pick and of the next step against it).
- Payback drawn: live chart of the yearly saving at the tariff adding up over 36 years against the extra cost, crossing point and lifetime line, with a table of the payback of every step; the tariff tween moves the crossing from 34 a to 21 a.
- Weights explained with two concrete designs: the pick and its next neighbour (rings green and red) with `price per kWh/a ≤ / > worth`; at 2 % to 98 % the worth runs 1 … 890 CHF per kWh/a and the pick is always the last front step whose price is below it (asserted for five weights).
- Daylight third objective: room daylight (rays, cones, dimmed lamps) from the toolkit, chips, formula, pair card with the daylight value, rows in words with signs, 2-objective and 3-objective front counters; `third_4` differs from `third_3` (verdict row replaces the daylight row).
- Collisions: legend columns, chart and table rows, leg chips, knee chip, badges placed by search, pair-card numbers above the caption zone, text ≥ 13 px.
- "Sun 66°" is explained by a note "summer noon sun, 66° above the horizon" (DE "Sommer-Mittagssonne, 66° über dem Horizont") beside every room.
- German: "Ersparnis", "spart", NBSP, formats `19.210`, `0,25`.

### Final beat list (32)

room, room_2, objectives, objectives_2, grid, grid_2, extremes, dominated, dominated_2, dominated_3, rule, rule_2, front, front_2, approximate, sets, walk, walk_2, walk_3, payback, payback_2, knee, knee_2, knee_3, third, third_2, third_3, third_4, weights, weights_2, decision, decision_2.

### Remaining risks

- The shared `ClipScene.present` (toolkit) no longer adds the scene title chip "Pareto Optimality"; all keyframes lack it. Toolkit decision, not changed here.
- "≈" is used wherever the printed inputs are rounded (e.g. 65 × 0.25 ≈ 16.2 CHF/a); "=" only where the printed integers close.
- Real-render time of the live groups (legend, chart, formulas rebuilt each frame during walks) not measured; capture takes about 2 minutes per language.
- The approximate path order (overhang, window, insulation) fixes the per-stage numbers; the net is order independent.

## Round 3

Verifier `verify-r2-pareto.md` (1 major, 10 minors). Re-captured en and de (strict layout 0 findings in 32 + 32 frames), frames read, `check-pareto-r2.py` 0 failures, `check-pareto-wrap.py` (no caption wraps between number and unit), `check-pareto-margin.py` (no ink within 24 px of the left, right or top frame edge).

- m1 deltas: the bar legend computes the changes from unrounded model values and apportions them to the total change (`_room.apportion`), so they add up (+57.4 −122.3 ±0.0 = −64.9) and a zero change prints "±0.0"; header "Δ kWh/a" now sits above the delta column (m10).
- m3: payback chart has the rotated axis unit "CHF" and the tick "30 a lifetime" ("30 a Lebensdauer").
- m4: `room_2` prints "533 kWh/a ÷ 48 m² floor ≈ 11.1 kWh/(m²·a)" below the bar.
- m5: the room choices card names the symbols ("Window share (WWR)", "Insulation thickness (t)", "Overhang depth (d)"); the grid chip reads "6 window shares × 3 insulations × 2 overhangs = 36 designs".
- m6: captions rewritten (`knee_2` de, `walk_2` de, `third` en/de, no "%" at a line start) so no wrap separates a number from its unit; the root cause (shared `wrap_body_lines` splits on U+00A0) stays in the shared fonts file.
- m7 margins: rows and legend end at x ≤ 6.84 (≥ 24 px), cost chip lowered, rule rows start at x 2.55.
- m8: tiles .046 with rings .04 larger, the clusters no longer overlap.
- m9: "−75 kWh/a" chip sits above the horizontal leg outside the quadrant; pair-card secondary text is 14 px in a lighter grey.
- M1 scene side: daylight legend moved right of the axis and below the guide; ray-through-chip remains with the toolkit (the chips are now opaque in the latest toolkit and the strict check reports none).
- Remaining (toolkit, not scene): the room chip reads "d 1.0 m" in `approximate` while the card and rows print "d 0.99 m" (toolkit chip has one decimal); the title chip is still not drawn by `ClipScene.present`.

## Round 4

Final verification `final-verify-pareto-algorithms.md` (pareto: 2 blockers, 5 majors, 14 minors). Re-captured en and de (strict layout 0 findings in 32 + 32 frames, no ink within 24 px of the frame edges, no caption wrap between a number and its unit), `check-pareto-r2.py` and `verify-check-pareto.py` 0 failures (the latter now tests the printed-value policy: changes are differences of the printed values).

- P-B1: the daylight equation prints `DF = 0.6 · 19.2 m² · 75° / (180 m² · 0.75) = 6.4 %` (German `TQ = …`); it is the scene's own `formula` row, the toolkit `derivation('daylight')` was not involved.
- P-B2: every Δ in the bar legend is the difference of the two printed values on the same frame (+57.4 −122.3 −0.1 = −65.0; knee_3 −57.3 +31.0 −24.5 = −50.8); no "±0.0" where the printed values differ.
- P-M1: the toolkit `energy_bars` got a compatible keyword `digits=0` (decimals of every printed kWh/a and of the intensity); the scene passes `digits=1`, so bar total, legend and derivation print one decimal (the bar widened to 1.1 and moved left so its total clears the scale rule and the frame margin).
- P-M2: the knee walk now ends on step 6 (step 7 saves 0.25 kWh/a, which no one-decimal Δ can show); the payback table still lists all 7 steps. Saved kWh/a and the yearly saving in the payback equations come from the printed totals (half-up), e.g. `65.0 kWh/a × 0.25 CHF/kWh ≈ 16.3 CHF/a`, `192 CHF ÷ 16.3 CHF/a ≈ 12 a`.
- P-M3: ticks 0, 10, 20 and `30 a` are all labelled in both languages; the lifetime is named in the verdict row.
- P-M4: legend rows moved 0.06 down (clear of the room wall); P-M5: the dashed drop lines of the cursor are a separate group, faded out in `third` and `sets`, so no guide runs under the legend title; legend and counters re-positioned.
- Minors: step labels `#1…#7` in the payback table, `dominates 2 front façades` (was "beats"), `d 0.6 → 0.99 m` in the path rows, ring of the 2D-dominated design white (the green ring is the 3-objective front), grid chip two lines clear of the axis title, grey legend "beaten by a cheaper façade", "best of both (not a façade)", DE `TQ`, `Sparsamste minus Günstigste`, shorter DE captions without orphans, shorter `besser` arrow, choices card clear of the room chips.
- Toolkit changes that landed meanwhile (chips now spelled out, e.g. "Vordach 0,6 m") made the plot and the left cards narrower: the plot is 4.5 wide, the intro room moved 0.6 right.
- Remaining: P-m14 (German "Vergleich" box close to the room's 6 m dimension) not changed; the room chip prints `d 1.0 m` while the card prints `d 0.99 m` (toolkit chip, one decimal).
