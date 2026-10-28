# Verification round 2: Beat2_GridRandomSampling (`🎲️sampling`)

Read-only verifier. Clip: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py` (class `Beat2_GridRandomSampling`, 33 keys). Frames: `🗑️generated/frames/Beat2_GridRandomSampling/` (33 English, 33 German, 1280 × 720). Zoom crops and numeric output: `🗑️generated/verify-sampling/`. Scope note: the persistent title is absent on purpose and is not reported.

## Verdict: FAIL

One blocker (wrong printed arithmetic on the closing answer) and four majors (value-to-design attribution, a label collision, the random table not linked to the plane, and mixed decimals for the same quantity). Everything else is minor.

Defect counts: **blocker 1, major 4, minor 15**.

## What was checked

1. **Frames.** All 66 PNGs read (33 EN, 33 DE). Frames are dated 21:57 to 22:03, newer than the scene file (21:54), so no re-capture was needed.
2. **Numbers.** `T/check-r2-sampling.py` (cache Python, `-I`): 145 checks, 4 mismatches. Two of the four are outdated expectations (evaluate_2 and limits_4 parts now apportioned, 35 + 424 + 140 and 38 + 281 + 177, both sum correctly). The other two are real, minor (threshold 519.5 and probe centre tie). `T/verify-check-sampling.py` recomputes the exact per-design parts, bills, autonomy and shading for every design printed on the frames (output in `🗑️generated/verify-sampling/numbers-all.out`). Random draws, box, closest pair, LHS strata and factor levels (`10/45/80 %`, `0/0.6/1.2 m`, `90/180/270/360°`, `10/20/30 cm`) match the frames.
3. **Narration.** `T/check-sampling-narration.py`: OK (33 clauses). `T/check-sampling-captions.py` crashed with an XML parse error inside Manim's SVG cache (tool issue, not a clip defect), so the caption length check was not re-run here. Every caption on the frames is two lines or fewer.
4. **Depth.** Compared with `🗑️generated/energy-frames/Beat8_Hauptgleichung-chain.png`: the sampling clip now has the same devices (sun-lit sections, live meters, substituted equations, labelled stacked bars, bill, compare tiles). Its density is higher (see m12).

## Owner's checks

1. **Every plotted value has its meaning on screen at the same moment.** Mostly yes: plane points are room pictograms, colours are ramp-coloured with the ramp strip, kWh/a sits on tags and bars, shade and autonomy are meters, electricity is derived in the equation panel. Exceptions: M3 (random table) and M1 (tags for two columns).
2. **Numbers match the model; arithmetic holds as printed; consistent decimals.** Model values reproduce everywhere within rounding. Printed arithmetic fails on limits_4 (B1). Decimals are mixed for the same kWh/a quantity on the same screen (M4).
3. **Overlaps, clipping, readability, density.** Overlap on the axis chips (M2), tag ambiguity (M1), box edges and chips on thumbnails (m7, m8), star and triangle blob (m9). No clipping found; equation rows end at x ≈ 1262 of 1280 on run_2 and limits_4 (EN and DE), tight but inside. Density: m12.
4. **Narration and visuals in sync.** All 33 keys match their picture. Exceptions: the random table cannot be mapped to plane points in the still (M3), and "No draw falls inside this box" is true only on the boundary (m7).
5. **German.** Complete: every visible string and clause has a German version, no English left except the loanword "Sampling" and the terms WWR and Δ. Formats correct (1.681; 522,1; 0,25 CHF/kWh; 1.500 h/a; 0,15 m). Small DE issues: m5 (1,2 vs 1,20), m14 (chip starts lowercase).
6. **Depth comparable to energy clips.** Yes in structure and devices. Density is higher (m12) and the arithmetic failure in B1 is the one place where the energy clips are more exact.
7. **Audit-r2 defects.** Status in section 4.

## Defects

Keys are frame keys; `EN`/`DE` mark the language version. Coordinates are approximate pixels on the 1280 × 720 frame.

### Blocker

**B1. limits_4 (EN and DE): derivation arithmetic is false as printed.**
- Location: equation row 1, EN x 815–1180, y 450–470: "135 kWh/a heat ÷ COP 3.5 = 38 kWh/a". DE row 1 the same. The model's heating demand is 135.04, so 135.04 ÷ 3.5 = 38.58 and printed to integers that is 39, not 38. The 38 comes from the largest-remainder apportioning of the bar (parts 38.58, 280.60, 176.68 forced to 38 + 281 + 177 = 496), which the derivation then inherits.
- Fix: print the closing screen at one decimal, with every line computed from the same one-decimal inputs: `135.0 ÷ 3.5 = 38.6`; `841.8 ÷ 3.0 = 280.6`; `0.432 kW · 1.500 h/a · 27.3 % = 176.7`; `38.6 + 280.6 + 176.7 = 495.9 kWh/a`; `495.9 × 0.25 = 124.0 CHF/a` (exact 123.97). The bar labels then read 38.6, 280.6, 176.7 and total 495.9, the same as the chip (see M4). This closes exactly, unlike the current integers.

### Major

**M1. grid_3 and grid_4 (EN and DE): value tags attach to the wrong column.**
- Location: tags for the top row, EN `547 kWh/a` x 430–485 and `499 kWh/a` x 493–547, y 205–240. The 80 % column thumbnails start at x 553. The 80 % column's tags sit in the gutter on the left of its thumbnails, next to the 45 % tags, so "499" reads as a second tag of the 45 % column. The mid row (`522`, `616`) and bottom row (`637`, `775`) have the same pattern.
- Fix: the plane has no room to the right of the 80 % column (the sections start at x 650). Put the 80 % tags above or below each thumbnail, or move the 45 % tags to the left of their thumbnails, so each column keeps one side. Alternatively give every tag a short leader to its thumbnail.

**M2. lhs, marginals, marginals_2, cells (EN and DE): the "axis ticks: one per sample" chip collides with the x-axis title.**
- Location: EN chip x 273–490, y 566–598; title "Window-to-wall ratio WWR in %" x 228–536, y 548–568. The chip's top border runs through the lower part of the title. DE chip "Achsenstriche: einer je Stichprobe" x 241–522, y 566–598 touches "Fensterflächenanteil WWR in %".
- Fix: remove the bottom chip and fold the tick statement into the strata chip at the top ("9 strata per axis: 7.8 pp · 0.13 m · one tick per sample"; DE "9 Schichten je Achse: 7,8 %-Pkt. · 0,13 m · ein Strich je Stichprobe"). Nothing sits below the axis title then.

**M3. random and random_3 (EN and DE): the recorded-draw table cannot be mapped to the plane.**
- Location: table EN x 815–1230, y 415–500 (`#`, `WWR in %`, `d in m`, numbers 1–9). The plane shows nine unnumbered ghost thumbnails (x 170–600, y 225–475). The 0.1-scale thumbnails of round 1 were removed rather than improved, so the table is bare numbers and the link between table column k and plane point k is missing in the still. The scene notes describe a column highlight during the tour; that is motion and cannot be verified from stills.
- Fix: number the nine plane thumbnails 1–9 (small tags) and put a 0.15-scale thumbnail above each table column, so the still alone shows the link. If the tour highlights column k while the room visits point k, also capture one mid-tour still to show it.

**M4. Same quantity, different decimals on the same screen (owner's check 2).**
- Examples: grid_2 board `522.1` vs tag and bar `522` (EN and DE). grid_3 board `499.2` vs bar `499`, grid_4 cards `Total 846` / `499` (integers) vs compare tiles `499.2`. gaps_3 board `561.8` vs bar `562`. reveal_2 chip `494.8 kWh/a` vs bar and equation `495`. limits_4 chip `495.9 kWh/a` vs bar, equation and bill `496`. limits_2 cards `494.8` and `495.9` with the same kWh/a totals as integers elsewhere. The scene's stated rule (integers on tags and bars, one decimal on boards, tiles and cards) is not what the cards show (grid_4, gaps_2, gaps_3 cards are integers).
- Fix: one convention for design totals across tags, bars, cards, boards, tiles, chips and equations. Recommended: one decimal everywhere for kWh/a totals (tag `499.2`, bar total `499.2`, bar parts from the same one-decimal apportioning); Δ values from the exact values (see m2). Apply the same rule to bills (CHF/a, integers) and overhang (two decimals, see m5).

### Minor

**m1. gaps_3 (EN and DE): bill "141 CHF/a" uses the rounded total.** Location: EN bill line x 666–866, y 565–582; DE `Stromrechnung 141 CHF/a`. The model gives 561.79 × 0.25 = 140.45, which prints 140; the 141 comes from 562 × 0.25 = 140.5. Fix: compute the bill from the unrounded electricity (140 CHF/a, or 140.4 with the one-decimal rule of M4).

**m2. compare_2 (EN and DE): Δ values are differences of displayed tiles.** Location: Δ labels grid x 735–905, y 325–340; random x 995–1180; LHS x 735–890, y 560–575. On screen 499.2 − 494.8 = 4.4 and 495.9 − 494.8 = 1.1 hold, but the model's exact differences are 4.35 (prints 4.3) and 1.03 (prints 1.0). Also the Δ bars are slivers (grid fill about 7 px of a 125 px outline) with no scale (D26 still open). Fix: compute Δ from unrounded values and print 4.3 / 67.0 / 1.0, or keep the displayed-difference rule and say so; add a scale "0 – 70 kWh/a" with ticks under the Δ bars.

**m3. limits (EN and DE): threshold chip shows 519.5 from the displayed optimum.** Location: chip x 210–553, y 170–203. 494.8 × 1.05 = 519.54 prints 519.5; the model threshold 494.84 × 1.05 = 519.58 prints 519.6, so a design at 519.55–519.58 is inside by the model and outside on screen. Fix: compute the threshold from the unrounded optimum (519.6) and use the same value for the filled region's boundary.

**m4. run_2, run_3, reveal_2, limits_4 (EN and DE): lamp line does not reproduce from its printed inputs.** Location: lamp row x 815–1262, y 505–525. EN limits_4 `0.43 kW · 1,500 h/a · 27 % on ≈ 177`: 0.43 × 1500 × 0.27 = 174.2; reveal_2 `28 % on ≈ 182`: 180.6. The "≈" hides a 3 kWh/a gap. Fix: print the power as 0.432 kW and the share with one decimal (27.3 %, 28.0 %), then drop "≈" (0.432 × 1500 × 0.273 = 176.8; 0.432 × 1500 × 0.280 = 181.4, both within rounding of the printed totals).

**m5. parameters, parameters_2, gaps, gaps_2 (EN and DE): overhang has two precisions on one screen.** Location: section chip `d 0.6 m` (EN x 825–898, y 215–245; DE `d 0,6 m`) against the slider label `0.60 m` (x 297–370, y 513–545) and the plane chip `WWR 45 % · d 0.60 m`. The gaps caption says "overhang 0.15 to 1.2 m" (DE "Vordach 0,15–1,2 m") while the chip says `d 0.15 m – 1.20 m`. Fix: overhang in two decimals everywhere in chips, sliders and captions (0.60 m, 1.20 m; DE 0,60 m, 1,20 m); one decimal on axis ticks only.

**m6. gaps (EN and DE): probe chip is an exact tie.** Location: chip `WWR 52 % · d 0.68 m` at x 905–1124, y 46–80. The box centre is WWR 52.49 % and d 0.675 m; 0.675 prints 0.68 with half-up and 0.67 with float (the check's mismatch). Fix: print the box edges (`WWR 35–70 %, d 0.15–1.20 m`) or the centre to three decimals in the probe chip (`d 0.675 m`).

**m7. gaps (EN and DE): empty-box edges run through sampled thumbnails.** Location: dashed box x 320–534, y 232–438. Left edge crosses draw 4 (34.99 % / 0.78 m, x 297–345, y 295–333); right edge crosses draw 7 (69.99 % / 0.62 m, x 510–558, y 326–367); bottom edge crosses draws 5 (47 % / 0.12 m, x 370–418) and 6 (58 % / 0.15 m, x 437–485, y 420–460). "No draw falls inside this box" holds only on the boundary, and the picture reads as contained. Fix: draw the thumbnails above the outline and state the rule as "no draw inside: WWR 35–70 %, overhang 0.15–1.2 m (edges included)", or shrink the dashed outline to the interior.

**m8. cells (EN and DE): empty-box chips sit on the frame and the frame crosses samples.** Location: chip `WWR 14 % – 76 % · d 0.47 m – 0.87 m` x 227–537, y 346–377 lies on the dashed bottom edge (y 377); the box edges pass through draw 14 % / 0.73 m (x 170–217, y 305–345) and draw 76 % / 0.60 m (x 548–595, y 330–370). Fix: put the numeric chip under the plane (outside the box), keep sample thumbnails off the outline, or use the label-only chip "largest empty box" and move the numbers to the cards.

**m9. reveal_2, gaps_3 (EN and DE): markers collide.** Location: reveal_2 star (66 % / 0.99 m, x 497–520, y 265–290) and LHS triangle (68 % / 1.00 m, x 515–530, y 258–285) form one blob; the star's outline covers the triangle. gaps_3 green ring (best draw) overlaps the orange pair ring. Fix: label the optimum and the LHS best with short leaders ("optimum 494.8", "best LHS 495.9") instead of relying on shape; shrink the pair ring to r 0.18 (round 2 D22 proposal).

**m10. run_2 and every room from run to limits_4 (EN and DE): daylight is drawn but its meaning is not labelled.** The room shows a daylight gradient and a yellow work-plane line (x 790–930, y 300 in the winter section; x 1080–1190 in the summer section) with no label. The scene never passes `light_label=True`, so "Tageslicht ≥ 2 % bis 3,8 m" and "Lampen an 34 %" are absent. The link daylight → lamps is carried by the meter "Daylight autonomy 66 % → lamps on 34 %" and the equation, which is adequate for run_2 but leaves the room's line unexplained (D01 partly resolved). Fix: pass `light_label=True` in `_section`, or label the work-plane line.

**m11. run, run_2, run_3 (shade and autonomy meters) and reveal_2, limits_4 (EN and DE): meters have no physical ends.** Location: "Glass in shade 0 %" bar x 756–937, y 358–370; "Glass in shade 89 %" x 1053–1234; "Daylight autonomy" bar x 847–1216, y 414–425. The energy clip Beat5 labels its scale ends. Fix: label both ends of each meter (`0 %` and `100 %`, DE `0 %` and `100 %`, or the words "ganz im Schatten" / "ohne Schatten" for shade).

**m12. run_2, run_3, reveal_2, limits_4 (EN and DE): equation panel density.** Location: equation panel x 815–1262, y 450–580 (five lines, bar, bill row), plus two section meters and the plane chip. About 25 numerals on one screen; the energy stills carry 1–3. Fix: move the `Bill` line to a separate beat, or drop it from run_2 (the tariff is already in the caption band of run).

**m13. random (EN and DE): table spacing.** Location: d row x 850–1250, y 485–500; values 0.30, 0.72, 0.78 are about 10 px apart. Readable but cramped. Fix: widen the table columns (the right column can start at x 800) or drop the `#` row and number the columns in the thumbnail row of M3.

**m14. gaps_3 (DE): lowercase chip.** Location: chip x 208–556, y 170–203 `bester der neun Ziehungen 561,8 kWh/a`. Fix: `Bester von neun Ziehungen: 561,8 kWh/a`.

**m15. marginals_2 (EN and DE): orange strip dims the chosen design.** Location: horizontal strip x 170–595, y 388–418 drawn over the ghost of draw 45 % / 0.33 m (x 370–420, y 385–425). Fix: draw the strip behind the ghosts (z below the thumbnails) with a 0.12 fill, or outline only (D24 proposal).

## Audit-r2 status (`audit-r2-sampling.md`)

| Defect | Status | Note |
| --- | --- | --- |
| D01 daylight never drawn (blocker) | Partly resolved | Meter and equation carry it; room line unlabelled, m10 |
| D02 parts do not add (blocker) | Resolved as sums, new arithmetic defect | apportioning now sums; limits_4 derivation broken, B1 |
| D03 "Cost" is the bill | Resolved | "Electricity bill" plus `Bill 522 × 0.25 = 131` |
| D04 "9 simulations → best of nine" | Resolved | limits_2 names the Latin hypercube set |
| D05 DE table headers in English | Resolved | `Fenster in %`, `Vordach in m`, `#` |
| D06 gaps caption names another region | Resolved | caption now states the box; boundary issue m7 |
| D07 random table abstract | Not resolved | thumbnails removed, numbers unlinked, M3 |
| D08 "room shows why" not shown | Resolved | grid_4 shows worst and best rooms with parts |
| D09 ghost bar overlaps the plane | Resolved | cards replace the ghost bar |
| D10 no substituted equation | Resolved | run_2, reveal_2, limits_4; arithmetic B1, m4 |
| D11 no shading or daylight meter | Partly resolved | meters exist, no ends, m11 |
| D12 reveal probe not on plane | Resolved | ring on the 80 % / 0 m corner |
| D13 cells box without numbers | Resolved | chip with both ranges, m8 |
| D14 threshold 519.6 vs 519.5 | Resolved as displayed; model 519.58 | m3 |
| D15 tie rounding (probe) | Not resolved | m6 |
| D16 mixed precision | Not resolved | M4 (major), m5 |
| D17 unexplained "1 m" | Resolved | removed |
| D18 "whole year" caption | Resolved | caption now says two solstice days |
| D19 gaps_3 caption vs chip | Resolved | |
| D20 gaps_2 only one room | Resolved | both rooms and cards shown |
| D21 naming on closing screens | Resolved | |
| D22 ring and marker clusters | Partly resolved | m9 |
| D23 10 % column on the axis | Resolved | thumbnails clear the axis |
| D24 tick legend and strip z-order | Partly resolved | legend added but collides (M2); strip still dims (m15) |
| D25 region outline and label | Resolved | region filled, label moved |
| D26 Δ slivers without scale | Not resolved | m2 |
| D27 secondary units grey | Resolved | units white |
| D28 DE word mismatch | Resolved | narration and bar both `Beleuchtung` |
| D29 layout checker blind | Not resolved | see verification notes |

## Verification notes (not defects)

- The layout reports `…-en-layout.md` and `…-de-layout.md` say "strict findings: 0 in 0 of 33 frames", meaning no frame was examined. They are not evidence; every visual finding above comes from reading the frames.
- `T/check-sampling-captions.py` crashed with an XML parse error in Manim's SVG cache. Caption length was not re-run; the frames show every caption at two lines or fewer.
- Motion (the random tour, the sweeps, the ring moves) was not verified; only the static keys were read.
- The evaluate_2 bar shows heating 35 where the model gives 34.4; this is the documented apportioning (sum 599 holds) and is not counted.
