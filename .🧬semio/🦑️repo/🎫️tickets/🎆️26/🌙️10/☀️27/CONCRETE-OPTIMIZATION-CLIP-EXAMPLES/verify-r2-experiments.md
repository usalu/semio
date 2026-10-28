# Verification r2: Beat3_FactorialExperiments (topic 🧪️experiments)

**Verdict: FAIL.** 1 blocker, 3 major, 17 minor.

Read-only verification of `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️experiments/🐍️.py` (class `Beat3_FactorialExperiments`). No source file was edited.

## 0. Scope and method

- NARRATION has **35** keys, not 36 (`scene-experiments.md` and `audit-r2-experiments.md` say 36; see m17). All 35 English and all 35 German frames were read at 1280 × 720. Zoomed crops and pixel probes were used where text was small or touching.
- Frames (21:34–21:39) are newer than the scene (21:31), the toolkit (`🎨️visuals`, 20:33) and the model (16:55). No re-capture was needed.
- Both layout reports read "strict findings: 0 in 0 of 35 frames" and "none" for every key. Those reports are not reliable (M3).
- `check-r2-experiments.py` still tests round-1 strings (for example "chain 70 % / EER −279"). Its single mismatch comes from the old positive-savings convention. It is superseded by `verify-check-experiments.py` (ticket root): 29 checks, 28 OK, 1 MISMATCH (the printed interaction formula, B1). Output: `🗑️generated/verify-experiments/check-run.txt`.
- Energy reference used for depth: `🗑️generated/energy-frames/Beat8_Hauptgleichung-chain.png`.

## 1. Checks (1)–(7)

1. **Plotted value ⇄ room at the same moment.** Holds for the four cards, the chains, the means, the factorial rooms, the waterfall and the model room. Fails for the curvature and confirm plot endpoints at 0 m and 1 m (M1).
2. **Numbers, arithmetic, units, sign.** Every printed chain was recomputed: ray 204 = 122 + 82; 7 × 122.4 × 0.977 = 837; 1,700.0 − 863.3 = 836.7 and 836.7 ÷ 3.0 = 278.9; 3 × 122.4 × 1.000 = 367; 938.4 − 571.5 = 366.9 and 366.9 ÷ 3.0 = 122.3; 7/3, 837/367 and 278.9/122.3 all round to 2.3; 427.25 − 251.65 = 175.6; 239.15 − 439.75 = −200.6; 339 + 88 − 100 − 39 = 288; 141.4 of 203.9 = 69 %; contrast −72.6 with error 5.7; U-wall 0.29 → 0.11. All hold except the printed interaction formula (B1). The effect sign convention (high − low, negative = saves) is applied consistently except one magnitude chain (m6). The mean of the four printed run values is 339.45, not 339.4 (m7).
3. **Overlaps, clipping, readability.** No clipped captions. Several label-on-line or label-on-shape touches remain (m1, m2, m3, m5, m10). German labels at about 13.6–14 px (legend, chips, table headers, waterfall) are legible at 1280 × 720. The layout checker is blind to these touches (M3).
4. **Narration ⇄ visual sync.** Holds for every key. The only exception is the endpoint display in M1. The 6 s sweep in `confirm_3` does match "the curve bends".
5. **German.** 35 of 35 keys complete. Decimal comma and dot thousands are correct (1.700,0; 1.428; 0,977; −278,9; 2,3 ×). Numeric tokens match English row by row. One caption line break splits "0,5" from "m" (m4).
6. **Depth vs energy clips.** Below the energy reference (M2).
7. **Audit r2 status.** See section 3.

## 2. Defects

### Blocker

**B1. Interaction formula does not compute as printed** (keys `difference`, EN and DE)
- Location: `_effects`, `form_ab`, lines 718–719 (same rows in both languages).
- Screen (EN): "Interaction A·B = ½ × effect at 70 % − effect at 30 %" and "−78.3 kWh/a = ½ × −278.9 kWh/a − [−122.3 kWh/a]". (DE: "Wechselwirkung A·B = ½ × Effekt bei 70 % - Effekt bei 30 %" and "-78,3 kWh/a = ½ × -278,9 kWh/a - [-122,3 kWh/a]".)
- Problem: the ½ binds only to the first term, so the line reads −17.2, not −78.3. The narration says "half the difference of the two overhang effects", and the difference is not grouped on screen. The check reports the same literal result.
- Fix: write the calculation in three exact steps, each with its operands: "difference = effect at 70 % − effect at 30 %", "−278.9 − (−122.3) = −156.6 kWh/a", "A·B = ½ × (−156.6) = −78.3 kWh/a". Do not use nested brackets: the bundled font draws "(" as "[", so grouping in brackets reads as "[ … [ … ]]". Build all three rows from `effects` and the savings constants. Apply to EN and DE.

### Major

**M1. Plotted 0 m and 1 m values have no room at that depth** (keys `curvature`, `confirm`, `confirm_2`; EN and DE)
- Location: `_curvature`, straight line and end tags at lines 812–818; room at the centre overhang at line 824; confirm tags at 836; sweep at 851.
- Problem: the plot shows 439.7 at 0 m and 239.1 at 1.0 m, but the room sits at 0.5 m with a 339.4 ghost. Only the `confirm_3` sweep (6 s, line 851) moves the room through 0 m and 1.0 m. In the stills nothing shows what the endpoints mean.
- Fix: drive the centre room's overhang tracker from 0 m to 1 m during `curvature` (with the straight-line marker and a bar that follows), or put two small rooms at 0 m and 1.0 m beside the plot, each with its bar and value. Keep the 0.5 m room for `confirm`.

**M2. Depth is below the energy clips** (all beats; EN and DE)
- Reference: `Beat8_Hauptgleichung-chain.png` has a headline chip with module and subtitle top-left, one large coloured substituted formula with units (about 30 px), a bar against a maximum, and a source tag.
- Here: no headline chip in any of the 35 frames (top-left is empty); the source tag is present (`present_experiments`, lines 482–483). The substituted formulas are 15–17 px in the right-hand panel (`_effects` 712–719, `_formula_rows`; `_term_rows` at 789, size 15; `_lines` 341–346). There is no large substituted equation per clause and no leader lines from the coloured terms (A teal, B orange, A·B tan) to the window, overhang or insulation on the room.
- Fix: add a headline chip top-left per beat (title plus one-line subtitle, 16–18 px). Set one substituted formula per clause at 26–30 px below the stage. Draw leader lines from each coloured term to its drawn part (window or overhang on the room, the card frame for the mean).

**M3. Layout reports are blind to touches** (both `Beat3_FactorialExperiments-{en,de}-layout.md`)
- Both report 0 findings in 35 frames, while m1, m2 and m3 are visible.
- Location: `T/layout_strict.py` (strict pass) and `layout_conflicts` in `🎨️visuals`.
- Fix: test text boxes against plotted polylines, dashed lines (mean lines in `_effects` line 705 are not avoided by `_near`), and filled shapes (room slab, wall, bar) with a margin of at least 4 px. Seed a known collision as a self-test; a "none" result is not accepted until the seed is caught.

### Minor

**m1. Bar label overlaps the room's floor slab** (`confirm`, `confirm_2`, `confirm_3`; EN and DE)
- "298.3 kWh/a" starts on the slab end at about x 555 px; the first glyph overlaps it. Location: `_bar` label at line 327 (`base + DOWN * .22`), `BIG_BAR` at line 40, used at lines 503 and 825.
- Fix: anchor the label right of the slab (move `BIG_BAR` right by about 0.3 units or align the label's right edge with the bar).

**m2. Answer labels abut the room wall** (`answer`; EN and DE)
- "287.8 kWh/a" and "190.5 kWh/a" start at the dashed right wall (x ≈ 364 and 958 px). Location: `_answer` lines 949–950, through `_bar` line 327.
- Fix: move both labels 0.15 units right, or put them under the bar outside the wall line.

**m3. "287.8" tag grazes the dashed mean line** (`main_2`; EN and DE; D31 still open)
- Location: tag placement in `_plot` line 597 (`_near` with `("down", "downright", "right")`); the dashed 239.15 line is drawn in `_effects` line 705 and is not in `_near`'s avoid list.
- Fix: pass the mean lines as `avoid` when placing the tags in `_effects`, or place the 287.8 tag to the right.

**m4. German caption splits number and unit** (`confirm`, DE)
- "…50 % Fensteranteil, 0,5" / "m Vordach." Location: NARRATION row `confirm`, line 105.
- Fix: use a non-breaking space (U+00A0) between each number and its unit in the German narration, and confirm the caption wrap keeps it.

**m5. Waterfall labels are tight** (`model`; EN and DE; D34 still open)
- "window" and "overhang" are about 4 px apart; "Fenster" and "Vordach" about 7 px. Location: `_waterfall` names at line 731, labels at line 755 (14 px, width 0.8), pitch `WF_PITCH` at line 41.
- Fix: 13 px with pitch 0.9, or two-line labels ("Mittel-wert" / "Fenster").

**m6. Ratio chain uses magnitudes, the strip uses signed effects** (`interaction`; EN and DE)
- "cooling electricity saved 278.9 ÷ 122.3 = 2.3" against "−278.9" and "−122.3" on the plot. Location: `_savings` lines 656–657.
- Fix: use signed values ("−278.9 ÷ −122.3 = 2.3"), or write "magnitude" in the line.

**m7. Mean of four does not close from the printed values** (`model_2`; EN and DE)
- The chip prints "mean of 4 339.4 kWh/a" (`_model_fit` line 797, from the unrounded model 339.42). The four printed run values 312.8, 190.5, 566.7 and 287.8 average to 339.45.
- Fix: print the four run values to two decimals (312.79, 190.49, 566.66, 287.76 → mean 339.42), or label the mean "from unrounded values".

**m8. Unsupported "?" hint** (`curvature`; EN and DE)
- "? may bend" / "? Krümmung" (line 819) carries no value. Fix: remove it, or put the simulated curve (or its label "simulated, 50 % window") there.

**m9. Drawn crosses do not show the 0.977 factor** (`saving_70`; EN and DE; D03 residual)
- The chain says "7 rays × 122.4 × 0.977", but all seven direct strands are drawn as fully crossed. Location: blocked strands in `_section` lines 216–229, chain at line 634.
- Fix: shorten each cross strand to the shading factor, or label the crosses "98 % of direct sun stopped".

**m10. Plan table rule and card order** (`plan`, `plan_2`, `response`; EN and DE; D26 and D40 residual)
- "[%]" and "[m]" touch the header rule at y ≈ 180 px (`_table` line 379). Cards read 2, 4 above 1, 3 while the table lists 1 to 4 (`_plan` line 567, `for i in (0, 2, 1, 3)`).
- Fix: move the rule down 0.05 units, and number or order the cards to match the table.

**m11. Repeat-measure beat has no room or values** (`repeat_2`; EN and DE)
- The scatter is grey dots in a dashed band with no design and no scale. It is the only beat without a physical room. Location: `_replicates` lines 871–878.
- Fix: draw the centre room beside the band, with its 298.3 kWh/a bar and the band labelled as an assumed spread.

**m12. Caption repeats the title** (`question_2`; EN and DE)
- The caption repeats the on-screen title word for word (title at line 513, NARRATION line 81). Fix: shorten the caption, or drop the title.

**m13. Room icon is an empty box** (`question_3`; EN and DE; D13 residual)
- Location: façade icon at line 524 and the icon row at lines 525–529. Fix: draw the 8 × 6 m plan icon with the south façade marked, or reuse the compass plan.

**m14. Thumbnails in the comparison beats** (`plan`, `purpose`, `purpose_2`; EN and DE; D37 residual)
- Cards are 0.44 scale (`CARD_S`, line 38); `purpose` mini-rooms 0.3 (line 907). Hero rooms exist for question, response, model, confirm and answer. Fix: one larger hero room in `purpose_2` and `purpose` for the centre run and the fraction.

**m15. Reason line and C effect differ without a note** (`purpose_4`; EN and DE)
- "cooling 285.9 → 289.4" (+3.5) at 70 %, 1.0 m sits next to "C +5.7", which is the average over all designs. Location: `_fraction` lines 928–934. Fix: add "at this design" to the reason line, or label the +5.7 as the average.

**m16. Simulated 1.0 m end is unlabelled** (`curvature`, `confirm_3`; EN and DE; D08 still open)
- The straight line ends at 239.1 (line 812); the simulated curve ends at 235.8 and is not labelled (`_curve` in `confirm_3`, line 849). Fix: label the simulated end "235.8", or draw the straight line through the two simulated ends and name it as such.

**m17. Documentation count** (`T/scene-experiments.md` lines 3 and 35; `T/audit-r2-experiments.md` line 3)
- Both say 36 rows. NARRATION has 35 (checked with `ast`). Fix: correct the count in both files.

## 3. Audit r2 status (43 defects)

- **Resolved (33):** D02, D03 (residual m9), D04, D05, D06, D07 (residual m7), D09, D10, D11 (residual m8), D12, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D27, D28, D29, D32, D33, D35, D38, D39, D41, D42, D43.
- **Partly (4):** D01 (sign now consistent, but the formula is wrong: B1); D13 (m13); D26 (m10); D37 (m14).
- **Open (6):** D08 (m16); D30 (M3); D31 (m3); D34 (m5); D36 (M2); D40 (m10).

## 4. Checked, not defects

- Insulation thickness is drawn to scale: the wall band is 5 px at 10 cm and 13 px at 30 cm in `purpose_2`.
- `main` and `main_2` have no residual ghost bars (pixel probe). Ghosts appear only where intended (`saving_70`, `interaction`, `difference`, `answer`).
- The minus sign is a short glyph that is close to a hyphen. It is still readable and not a defect.
- Parentheses render as square brackets, which the toolkit notes. Readable; it is why B1 needs separate rows, not nested brackets.

## 5. Files

- Verdict: `T/verify-r2-experiments.md` (this file).
- Check script: `T/verify-check-experiments.py` (run with the cache Python, output in `🗑️generated/verify-experiments/check-run.txt`).
- Crops and probes: `T/🗑️generated/verify-experiments/` (`crop-*.png`, `verify-crop-experiments.py`, `verify-pixel-probe-experiments.py`, `probe-insulation.py`, `crop-extra.py`).
