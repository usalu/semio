# 📍 Sweep: Excel References (Beat2_References)

Verdict: **FIX**

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py` (class `Beat2_References`, mtime 04:32).
Frames: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/🗑️generated/frames/📊️excel/Beat2_References/{en,de}/`
Freshness: all 46 PNGs (23 clauses x 2 languages) are dated 04:42 to 05:02, newer than the scene file. No stale frames.
Reference numbers: `🏢️nakagin` model and `📊️excel/🎬️clip/🧱️model` (`attached_factor` = 0.5, Type A width 2.50 m, H_T 30.92 W/K).

Every frame was read in clause order (hook, pointer, relative, shift, lights, total, fixed, f4, absolute, slide, repair, mixed2, mixed, fill2d, sheet, stuck, pertype, threed, span, deleted, guard, names, recap).

## Defects

Frame order follows the narration. "Both" means the same defect appears in EN and DE.

1. **BLOCKER - relative, lights, total, absolute, slide, guard (both)**: The back-wall row shows F9 = 1.88 W/K, which is correct per the model (6.3 m² x 0.60 x b 0.5). The taught pattern is the fill-down of `=D3*E3`, which gives `=D9*E9` = 3.78 W/K. A student who fills down gets 3.78 and a total of 32.8 W/K, not the 30.92 on screen.
   Fix: show the row-9 formula as `=D9*E9*attached_factor` (the model's F9 formula) with a note "b = 0.5, back wall to unheated core", in the `lights` or `total` card. Keep the value 1.88.

2. **MAJOR - hook (both)**: The formula bar shows `2.5` while B2 shows `3.00 m` and the roof shows 12.0 m². The bar holds the stored value, the cell shows the animated value. Model width is 2.50 m.
   Fix: either hold B2 at `2.50 m` during the hook and let the change to 3.00 m happen in `pointer`, or write the animated value into the book so the formula bar matches.

3. **MAJOR - stuck (both)**: The Program "Total" row C8 shows 15.1 m² (model: `C8 = D8/B8`, a count-weighted mean). Beside it the visible column 8.7 + 30.9 + 3.1 sums to 42.7, so a student adding the column gets a different number.
   Fix: in `_stage_sheets` set the `win7` window rows to `(3, 7)` so the total row is cropped out of the stuck frame. (The sheet and pertype frames show 8.7 for all rows, where the mean and the sum agree, so they are fine.)

4. **MAJOR - hook, relative, shift, lights, total, fixed, f4, absolute, slide, repair, guard, names (DE)**: Type A column C clips the German labels "Rechte Seitenwa..." and "Rückwand zum...". Also clipped in DE: Program C3 "Nettogrundfl..." / "Nettogrundflä..." in sheet, stuck and deleted.
   Fix: widen column C in the German layout of the Type sheets and Program sheet (or auto-fit the column in `xl_window`).

5. **MAJOR - deleted (EN)**: In the small Energy window the label clips to "Heating load, one..." (the full label shows in span and threed).
   Fix: widen column A of that window (`cols "A:B"` with a larger width), or use "Heating load" in the small window.

6. **MAJOR - hook, sheet, stuck, deleted (DE)**: English label "Transmission H_T" in row 11 of the German Type sheets. The caption says "Transmissionswert H_T".
   Fix: change the German label in the model to "Transmissionswert H_T".

7. **MAJOR - mixed2 (both)**: The flux grid C19:E21 shows "10.0", "15.0", "46.6" with no unit, and the U column B19:B21 shows "0.50", "0.60", "2.33" with no unit. Only the chart shows W/m².
   Fix: set cell formats in the grid to `0.0 "W/m²"` (C19:E21) and `0.00 "W/(m²·K)"` (B19:B21), and widen the columns.

8. **MAJOR - sheet, stuck, deleted (both)**: The title bar of the small Excel window overlaps: the filename "Nakagin.xlsx" runs under the search pill ("Search" / "Suchen").
   Fix: in `xl_window`, drop or shift the search pill when the window is narrower than about 5.5 units.

9. **MAJOR - names (both)**: The Name Manager card overlaps the bottom edge of the formula card, and its heading ("Name Manager" / "Namens-Manager") sits on the border.
   Fix: move the manager card, its heading and rows down by about 0.35 scene units (card centre 0.1 to about -0.25).

10. **MAJOR - deleted (both)**: The error tooltip ("The formula refers to a cell that no longer exists" / "Die Formel verweist auf ...") covers the Type D row (C7:D7) of the Program table.
    Fix: draw the tooltip above the cell or drop it from this frame.

## Clean

- German narration and German on-screen text: no grammar errors found. Excel error names are right (#WERT!, #BEZUG!, ZÄHLENWENNS, SUMME, Freigeben). The DE chart and the Energie and Programm tabs are translated.
- The numbers in the lights, total, absolute, span, deleted and Energy frames reconcile with the model, apart from the back-wall factor in item 1 (30.92 W/K; 681, 704, 681, 681 W per type; 2,747 W; 2,066 W after moving Type B; 2,042.5 W after deleting Type C).
- Slide errors (#VALUE! / #WERT!, 5 W vs 120 W) are real Excel results of the wrong reference.
- Captions match the pictures in both languages.

## Verdict

FIX. Item 1 is a blocker. Items 2 to 10 are majors. Items 4, 6, 7 and 9 are the German-specific ones.
