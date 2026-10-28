# Verify r2: Beat5_ParetoFront (Pareto Optimality)

Clip: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🏆️pareto/🐍️.py`, class `Beat5_ParetoFront`, 32 keys. No source file was edited.

## 1. Verdict

**FAIL**, with 0 blockers, 1 major and 10 minor defects. Every central claim is now true, and every number matches the model to display rounding except m1 and m2. What still fails the ticket's own rule ("nothing overlaps another label, a line or the caption band") is the sun-ray bundle running through the dimension chips, and a dashed guide and the y-axis touching a legend title. The fixes are local and small.

| Severity | Count | IDs |
| --- | --- | --- |
| blocker | 0 | none |
| major | 1 | M1 |
| minor | 10 | m1 to m10 |

Tooling (not counted as clip defects): T1 to T3 in section 5.

## 2. Scope and method

- All 32 English and all 32 German keyframes were opened with the Read tool and checked key by key. Zoomed crops (PIL, `T/🗑️generated/verify-pareto/`) were used for every collision claim.
- Frames are newer than the scene: scene `🏆️pareto/🐍️.py` 20:51:43, toolkit `🏠️room/🐍️.py` 20:33, frames 20:52 to 20:57. No re-capture was needed.
- Keys: the 32 narration keys equal the 32 frame keys in both languages (`verify-check-pareto.py`, check 1).
- Scripts run:
  - `T/check-pareto-r2.py` (the newest contract, `check-pareto-r2.py` in the ticket): 0 failed. Log `T/🗑️generated/verify-pareto/check-pareto-r2.log`.
  - `T/check-r2-pareto.py`: 8 MISS lines, all stale. They test hard-coded screen values that the scene no longer draws (ramp ends 495/846 kWh/a and 1/7 %, bar sums 723/775, cooling +51, "each step saves less"). The current code draws the correct values, confirmed on the frames. Log `T/🗑️generated/verify-pareto/check-r2-pareto.log`. Do not use it as the sign-off gate.
  - `T/verify-check-pareto.py` (adapted for this pass, 105 checks): 103 ok, 2 FAIL. The two FAILs are m1 (heating and cooling deltas). Log `T/🗑️generated/verify-pareto/verify-check-pareto.log`.
- Layout reports `…-en-layout.md` and `…-de-layout.md` say "none" for every key, and the en report reads "strict findings: 0 in 0 of 32 frames". They detected none of the collisions below (see T1).

## 3. Defects

Severity: blocker (breaks a central claim or the owner's bar), major (breaks a ticket rule visibly), minor (local fix). Keys are `en-` / `de-` frames unless stated; coordinates are on the 1280 x 720 frame.

### M1 (major) Lines run through labels: sun rays and a dashed guide

- Keys with the ray bundle on a dimension chip (visible in the frames at the chip's position): en/de `room`, `objectives`, `grid_2`, `extremes`, `dominated`, `dominated_3`, `rule`, `rule_2`, `approximate` (chip "d 1.0 m" at about x 608 to 690, y 153 to 188), `walk`, `walk_2`, `walk_3`, `payback`, `third`, `third_2`. Confirmed by crop on `en-dominated`.
- Location: the dimension chip "d 0.0 m" in `en-dominated` (about x 620 to 707, y 155 to 190). The ray bundle from the sun runs through the right half of the box (crop `en-dominated-600-140.png`). The toolkit chip is unfilled, so rays show through.
- Second location: legend "Tile colour: daylight factor" / "Kachelfarbe: Tageslichtquotient" (scene line 990, `point=(-4.65, 1.25, 0)`). In `de-third`, `de-third_2`, `de-third_3` the title starts at x of about 88 and touches the y-axis. In `en-third_2` and `de-third_2` the title sits on the dashed front guide at y of about 229 (crop `de-third_2-80-200.png`).
- Fix: give the dimension chips an opaque fill and a z-index above the rays (toolkit `chip()` in `🎨️visuals/🏠️room/🐍️.py`, line 236, plus the depth chip at line 808). Start the ray bundle outside the chip or move the chip. For the legend, move it to x of at least -4.5 and lift it above the guide (scene line 990), so the title clears both the guide and the axis.

### m1 (minor) Bar-legend deltas show a change the model does not have

- Keys: en/de `walk_2` and `walk_3`; en/de `knee` (cooling).
- Location: `walk_2` bar legend, "Heat 97.7 kWh/a" in `walk` becomes "97.6 kWh/a -0.1" in `walk_2`. The model heating change for step 1 is 0.000 (97.649 both sides). The knee step 7 cooling prints +50.4, the model gives +50.455 (rounds to +50.5).
- Cause: `_segments` (scene line 330) apportions each design to the total, then `_bar_legend` (line 337) takes deltas of the apportioned values.
- Fix: compute the deltas from unrounded model values, and apportion the deltas to the printed total change (same largest-remainder helper), so they add up and a zero change shows as "±0.0".

### m2 (minor) Overhang shown with two precisions in one frame

- Keys: en/de `approximate`.
- Location: room chip "d 1.0 m" (about x 608 to 690, y 153 to 188). The pair card and the path rows print "d 0.99 m" (for example "d 0.60 → 0.99 m +124.8 CHF").
- Fix: print the refined overhang as "d 0.99 m" in the room chip too (two decimals when the value is not a tenth). `_change` (scene line 377) already does this for the delta chip only.

### m3 (minor) Payback chart: no unit on the y-axis, lifetime tick without unit

- Keys: en/de `payback`, `payback_2`, `knee`, `knee_2`, `knee_3`.
- Location: y ticks 0 / 200 / 400 / 600 at about x 566 to 596 carry no unit or axis title (`_payback_chart`, scene lines 811 to 850). The lifetime tick reads "30 lifetime" / "30 Lebensdauer" (line 828), with no "a" or "Jahre".
- Fix: add the axis title "cumulative CHF" (de "kumuliert CHF") and write the tick as "30 a lifetime" / "30 a Lebensdauer".

### m4 (minor) Intensity "11 kWh/m²a" has no area on screen

- Keys: `room_2`, `grid`, `objectives`, `extremes`, all bar frames.
- Location: label under the bar, for example "11 kWh/m²a" at about x 1155 to 1240, y 318 (walk frames) and at about x 920 to 1010, y 558 (`room_2`).
- Cause: the model divides by `floor = width × depth` = 8 m × 6 m = 48 m² (`🧮️model/🐍️.py`, line 142). The section draws only the 6 m depth, so the divisor is never shown.
- Fix: write "11 kWh/m² floor, 48 m²" once in `room_2`, or draw the 8 m width, or remove the intensity from the bar.

### m5 (minor) Symbols WWR, t and d are never explained

- Keys: every room frame, `grid` (chip "6 WWR × 3 t × 2 d = 36 designs").
- Location: chips "WWR 40 %", "t 12 cm", "d 0.6 m". The words "Window share", "Insulation", "Overhang" appear only in `room` ("What we can choose").
- Fix: print the chips as "Window 40 %", "Insulation 12 cm", "Overhang 0.6 m", and the grid chip as "6 window × 3 insulation × 2 overhang = 36 designs".

### m6 (minor) Caption wraps between a number and its unit

- Keys: `de-walk_2` (line 2 "kWh/a.", after "65" on line 1, about x 560 to 690, y 660); `en-third` (line 2 is "%." alone, after "in").
- Cause: `wrap_body_lines` (`🏗️authoring/🔤️fonts/🐍️.py`, line 304) calls `para.split()`, which also splits on U+00A0. The NBSP glue used by the scene is lost. The helper is shared by every clip.
- Fix: in the shared wrapper split on ASCII space and tab only. Then recheck all clips.

### m7 (minor) Last check row nearly touches the frame edge

- Keys: en/de `rule_2`.
- Location: the row "electricity: strictly less, -75 kWh/a" ends at x of about 1262 of 1280 (about 18 px).
- Fix: start the strict rows 0.2 units further left (scene lines 636 to 637, `SIDE_LEFT`), or split each row into two lines.

### m8 (minor) Grid tiles overlap in two clusters

- Keys: en/de `grid`, `front`, `front_2`, `weights`.
- Location: about x 390 to 450, y 415 to 445 (CHF 21,300 to 21,600 × 520 to 560 kWh/a, 4 to 5 tiles), and about x 240 to 300, y 400 to 440 (CHF 20,000 to 20,300 × 620 to 660). Crop `en-grid-330-400.png`.
- Fix: reduce `THUMB` (scene line 28) to about .048, or add short leaders for tiles that overlap by more than 30 %, keeping front tiles on top.

### m9 (minor) Quadrant label chip inside the box, low-contrast pair text

- Keys: en/de `dominated_2`.
- Location: chip "-75 kWh/a" at about x 133 to 240, y 277 to 307, inside the green quadrant. The pair-card grey text at about x 585 to 680, y 555 to 590 is about 14 px in MUTED on the dark ground.
- Fix: put the chip on the right of the vertical leg, outside the box (for example at its end, as the cost chip sits on the horizontal leg). Set the pair-card text to 16 px with at least 4.5:1 contrast.

### m10 (minor) Column header of the delta column sits below the table

- Keys: en/de `walk_2`, `walk_3`.
- Location: "Δ kWh/a vs. previous" under the "Total" row at about x 1050 to 1212, y 432 (scene line 357).
- Fix: place the header above the Δ values, in the same row as "Light / Cool / Heat / Total" headers.

## 4. Per-criterion check

1. **Every plotted value has its meaning in the room at the same moment.** Met. Tiles show the design; the room shows that design; the bar gives kWh/a with segments; cost chip; daylight: DF formula with sky angle and window area, the floor band brightens (mean 94 to 146 in the probe) and the lamp cones fade (crop `en-third` vs `en-third_2`); weights: "1 kWh/a is worth 18.2 CHF" and "890 CHF" printed in the frame; payback: "65 kWh/a × 0.25 CHF/kWh ≈ 16.2 CHF/a" and "192 CHF ÷ 16.2 CHF/a ≈ 12 a". Residual: m4, m5.
2. **Numbers, arithmetic, units, claims.** Almost all met. All of these were recomputed from the model: grid 36 designs (6 × 3 × 2); front 8 in 2D and 30 in 3D; costs and kWh/a of all tiles and cards; extremes gap +2,342 CHF and -129 kWh/a; dominance -2,486 CHF and -75 kWh/a; daylight rows +2,136 CHF, +127 kWh/a, +4.0 %; DF 2.4 % and 6.4 % (the printed formula gives the same); approximate path +124.8, +268.8, -685.4 CHF and -5.4, -21.4, +1.8 kWh/a, net -291.8 CHF and -25.0 kWh/a; payback table at 0.25 (12, 34, 309, 335, 624, 963, 2,680) and 0.40 (7, 21, 193, 209, 390, 602, 1,675); step 1 = 192 CHF, 65 kWh/a, 3.0 CHF per kWh/a; ratios strictly rising; "34 years" (33.6), "21 years" (21.0), "12 years" (11.8); ramp ends 520 and 776 kWh/a, 1.8 and 6.4 %; picks 1.0, 18.2, 890 CHF per kWh/a. Exceptions: m1, m2, m3, m4. The printed "168 CHF ÷ 0.063 CHF/a ≈ 2,680 a" is within the rounding range of the printed inputs (2,646 to 2,688), so it is not counted.
3. **Overlaps, clipping, readability.** Not met: M1, m7, m8, m9. No clipping of text or caption. The 13 px chart and table labels are legible at this size.
4. **Narration and visual sync.** Met. Each clause coincides with the drawn change, for example `walk_2` (step card and bracket), `knee_2` (red 34 a row), `knee_3` (21 a and the moved ring), `third_2` (rows), `third_4` (verdict row), `decision_2` (knee design). No clause claims something the frame does not show.
5. **German.** Complete: 32 of 32 keys, every chip, row, axis and caption. Formats are right: `19.829`, `0,25`, `3,0 CHF`, `6,4 %`, `2.342 CHF`. No untranslated string found. Defects: m6 (wrap), and the lifetime tick (m3). "Ersparnis", "Vordach", "Knie" are used consistently.
6. **Depth comparable to the energy clips.** Met for derivations: substituted equations with units appear in `extremes`, `walk_3`, `payback`, `knee`, `knee_2`, `knee_3`, `third`, `approximate` and `decision_2`, comparable to the energy stills. Still missing relative to them: a physical gauge for the daylight objective and an explained unit for the intensity (m4).
7. **Audit-r2 status** (IDs from `audit-r2-pareto.md`):
   - Fixed: D01, D02 (lamp cones fade and the room brightens; DF formula shown), D03, D04, D05, D06, D08, D09, D10, D11, D13, D14, D15, D16, D20, D21, D23, D27, D28, D29, D30, D31, D32, D33, D34, D37.
   - Partly fixed, residual as minor: D07 (m3), D17 (m1), D18 (m10), D25 (m9).
   - Open: D12 (tooling, T1), D19 (m2), D22 (M1), D24 (m7), D26 (m8), D35 (m6).
   - Not applicable: D36 (numbers correct).

## 5. Tooling (not clip defects, not counted above)

- **T1, layout check blind.** `layout_strict` reports no findings (its en report reads "strict findings: 0 in 0 of 32 frames") and misses every collision in M1, m7, m8 and m9. Extend it to text-box versus line, ray and axis overlaps, to a 12 px edge margin and to tile-versus-tile overlap, and make it fail on the current frames before it is used as a gate.
- **T2, stale contract script.** `T/check-r2-pareto.py` tests old screen values and reports 8 MISS lines that are false today. Retire it or update it to the current render; `T/check-pareto-r2.py` and `T/verify-check-pareto.py` are the current contracts.
- **T3, title not drawn.** `TITLE` ("Pareto Optimality" / "Pareto-Optimalität") is defined but no frame shows it, because the shared `ClipScene.present` no longer adds the title chip. This is a toolkit decision; confirm it for all clips.

## 6. Evidence

- Frames: `T/🗑️generated/frames/Beat5_ParetoFront/Beat5_ParetoFront-<en|de>-<key>.png` (64 files, all read).
- Crops: `T/🗑️generated/verify-pareto/en-dominated-600-140.png`, `de-third_2-80-200.png`, `en-third-750-190.png`, `en-third_2-750-190.png`, `en-grid-330-400.png`, `en-walk-560-420.png`, `en-walk_2-560-400.png`, `en-approximate-600-120.png`, `en-dominated_2-100-250.png`.
- Logs: `T/🗑️generated/verify-pareto/check-pareto-r2.log`, `check-r2-pareto.log`, `verify-check-pareto.log`.
- Check script: `T/verify-check-pareto.py`.
- Model facts used: `C/🧮️model/🐍️.py` (floor = 8 m × 6 m, line 142; daylight factor, lines 150 to 161).
