# Audit: excel_anatomy (`Beat1_Anatomy`)

Scope: all 23 keyframes in `🗑️generated/frames/📊️excel/Beat1_Anatomy/en` and `de` (46 PNGs, viewed individually), `layout.md` (en: 39 strict findings in 11 of 23 frames; de: 75 in 20 of 23, mostly grid lines through cell text by design and card/tooltip overlays), the scene `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`, the storyboard, the scene brief, `research-excel.md`, `research-nakagin.md`, and the engine values of the `Type A` sheet and `🏢️nakagin` model (read through the engine, not typed).

## Verdict

Not ready to render. No defect is confirmed as a hard blocker by the frames alone, but one candidate blocker needs a 1080p check: the window font draws parentheses as square-bracket-like glyphs, which undermines the precedence and SUM lessons. About 12 major defects (ghost tab bar, clipped `#NAME?`, stale formula bar, outer width drawn wider than B2, inconsistent width/length colours, tooltips and card covering values, legend pointers missing, dependents arrows missing, wrong dimension marked red in the error clauses, the wrong perimeter never named). The numbers, Excel names, German error codes and the separator rule are correct. The narration arc is sound and both languages are complete.

## Findings table

Identical defects in en and de are listed once with lang `en+de`; where the German frame differs, it is noted.

| frame/key | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| all frames `grid` to `legend` (17 frames) | en+de | major | An empty tab-bar background (outline, separators, arrows, no labels) stays at the top of the stage from `grid` to `legend`. `present_anatomy` removes `strip` and its tabs, but not the bar background. | Remove the whole strip object (its background mobject too) right after `sheets`, or fade it out in the `grid` step. |
| `font` (all window frames with parentheses) | en+de | blocker (pending 1080p check) | `(` and `)` render like `[` and `]`: `=SUM[G12:G15]` (function), `=2*[B2+B3]` (precedence), `148 kWh/[m²·a]`, `=SUMME[G12:G15;1,5]` (locale). The bracket lesson reads as invalid syntax. | Check the window font's parenthesis glyphs at 1080p. If it is the font, use a face with round parentheses for the window and formula bar. |
| `grid` | en+de | major | Ghost bar (see first row) also hides the callout area above the window in `bars`. | As above. |
| `range` | en+de | major | Selection tint over B3:B4 greys the values "4.00 m" and "2.50 m" (de "4,00 m", "2,50 m") until they are nearly invisible. | Keep the text above the tint (or use an outline-only selection) so the values stay at full contrast. |
| `bars` | en+de | major | "formula bar" / "Bearbeitungsleiste" callout labels sit on the ghost tab bar; layout reports 67% of the label covered by a filled shape. The "Name Box" leader crosses the "File"/"Datei" menu. | Remove the ghost bar; place both labels above the window title with 0.3 units of clearance; route the Name Box leader around the menu. |
| `display` | en+de | major | The opaque cell card covers the right 35 px of column B: values in B3 to B8 are clipped ("4.00 m", "2.50 m", "1.30 m"). The capsule is still drawn although `show=False` was set just before. | Place the card over columns C to D, or beside the window, so B stays readable. Check that the live capsule redraws before the capture. |
| `display` | de | major | The card shows the number format as `0.00 "m"`. German Excel shows the format code with a decimal comma (`0,00 "m"`). See fact F1. | Format the code string with the locale decimal separator, as the rest of the German UI does. |
| `frames` (also `formula`, `recalc`, `div0`, `ref`) | en+de | major | Colour linkage: outer width is yellow and outer length cyan (address, range, function), while interior width is blue and interior length red (formula, frames, recalc, div0). The clause says "the same colours mark width and length on the floor", which is contradicted by the same concept appearing in two colours. | Use one colour pair for width and length in both outer and interior dimensions (for example dashed lines for interior), or say the difference explicitly. Set this in `_dim_color` and the interior dimension colours. |
| `hatch` | en+de | minor | The floor result (8.7 m²) is labelled only in the cell B9, not on the hatched floor, although the storyboard asked for an area label over the hatch. | Add a small badge with the area on the hatched floor, drawn from the engine value. |
| `precedence` | en+de | major | The typical mistake is never shown or named. `=2*B2+B3` (9.00 m, wrong outline) is never in the formula bar. The outline flips to the right result with no statement that the first formula was wrong. | Show the wrong formula for about 1.5 s with a red "forgot the brackets" callout, then the right one. Add this to the narration or caption ("the first formula adds before it multiplies"). |
| `function` | en+de | minor | The three fitting hatches are not labelled with the table rows (Bed, Bath, Storage), and the total 3.6 m² is not drawn on the capsule. Arguments of SUM are not annotated (storyboard asked). | Add small labels in the hatch colours and a total chip; annotate `G12:G15` and the parenthesis. |
| `recalc` | en+de | major | (a) The formula bar still reads `2.3` for B5 while the cell shows "2.80 m". (b) The outer capsule is drawn 0.5 m wider (`shift` is added to the outer width) while B2 still shows "2.50 m" in the same frame. | (a) Refresh the formula bar from the engine value during the ramp, or show the edit in the bar. (b) Keep the outer shell fixed and only move the interior walls, or change B2 through `apply` and redraw from it. |
| `trace` | en+de | major | Only the precedent box and arrow are visible. The dependent arrow to B10 (Volume, adjacent to B9) is hidden under the cell frame, although the caption names dependents. | Draw dependents with a visible offset arrow to the right of column B, or show rows 1 to 16 so that B15 and B16 are visible dependents. |
| `div0` | en+de | major | After clearing B6, the red marking is on the outer length "4.00 m" (`broken_dims=("length",)`), not on the empty interior length. "Free floor area" shows -3.6 m² without an error. The tooltip covers "Annual heating demand" and row 13. | Mark the interior length line (the one the cell is about). Show a sensible state, or explain that the zero area propagates to B16 (or hide B16). Move the tooltip above the window. |
| `name` | en+de | major | `#NAME?` is clipped to "#NAM…" in G16 because the column is too narrow for the error code. | Widen column G for error frames (or use a window column width that fits the error text), so the code is complete. |
| `ref` | en+de | major | The tooltip covers the Volume and Transmission H_T values. The red mark again sits on the outer length, not on the deleted row. Deleting row 6 also removes the surface row "Right side wall" (C6:F6), which is never shown or mentioned. | Move the tooltip. Mark the deleted (interior length) line. Mention or hide the surface-row side effect (for example, by naming the removed row in the caption or by using a helper row). |
| `value` | en+de | major | The tooltip covers the Volume value (row 10). The red mark is on the outer length (same as `div0`). | As for `div0` and `ref`. |
| `repair` | en+de | minor | The clause says "remove the causes", but the frame jumps to the repaired state. The learner never sees the action (retyping 3.8 m in B6 or undoing). | Animate the retyping (or an undo) of B6 so the cause is removed on screen. |
| `legend` | en+de | major | Badges 4 (reference) and 6 (function) have no pointer on the window, although the clause promises a miniature of each term's location. Badge 2 (worksheet) sits on row 10 "Volume", not on the "Type A" tab. Badge 7 partly covers "B9" in the Name Box. | Give 4 a spot (the reference frame of B5 or the B5 token) and 6 a spot (the SUM token in G16). Put badge 2 on the Type A tab (the tab centre, not +0.32 above it). Move badge 7 to the right of the Name Box. |
| `hook` | en+de | minor | The chip "capsules in the building: 140" overlaps the podium's lower edge (layout: object under chip). | Move the chip 0.2 units lower or raise the tower. |
| `constant` | en+de | minor | Only the text cell A2 ("Width") is shown. The clause "typed numbers are constants" needs a number selected too (B2, showing 2.5 in the formula bar). | Select B2 after A2 in the same clause. |
| `value` (de) / `div0` (de) | de | minor | The caption says "Eine leere Länge" and "die Länge". The cleared and typed cell is the interior length (B6). | Say "Innenlänge" in both captions (en: "An empty interior length"). |

## Fact findings

| ID | item | status | notes |
| --- | --- | --- | --- |
| F1 | German format code in the display card | defect (major) [CHECK] | German Excel shows number formats with the decimal comma (`0,00 "m"`). The card shows `0.00 "m"`. |
| F2 | Status bar values in `range` ("Average: 3 / Count: 3 / Sum: 9"; de "Mittelwert: 3 / Anzahl: 3 / Summe: 9") | minor [CHECK] | Excel formats the status bar with the format of the first selected cell (`0.00 "m"`), so "3.00 m" and "9.00 m" are expected. Confirm in the running app. |
| F3 | Error tooltips (en "The formula divides by zero or by an empty cell.", "Excel does not recognise a name or function.", "The formula refers to a cell that no longer exists.", "A value in the formula has the wrong type."; de translations) | minor [CHECK] | These are custom strings, not verified M365 texts. Confirm the real tooltips before filming or label them as teaching texts. |
| F4 | German error codes `#BEZUG!`, `#WERT!`, `#DIV/0!`, `#NAME?` | pass | Match German Excel. The research file marked them [CHECK]; confirm once in the German UI. |
| F5 | German function `SUMME`, argument separator `;`, decimal comma (`SUMME(G12:G15;1,5)`) | pass | Correct. `localize()` output checked: `=SUMME(G12:G15;1,5)`. |
| F6 | Type A values (engine, `🏢️nakagin` basis) | pass | Outer 2.50 x 4.00 x 2.50 m (sourced, Wikipedia), interior 2.30 x 3.80 x 2.10 m (sourced, Archiweb), window 1.30 m (sourced), net floor 8.74 -> "8.7 m²", volume 18.354 -> "18.4 m³", fittings 1.8 + 1.2 + 0.6 = 3.6 m², perimeter 9.00 (wrong) and 13.00 (right), annual demand 1,296 kWh/a, 148 kWh/(m²·a). |
| F7 | 140 capsules in four types | pass | Model slots: 140 (A 36, B 35, C 34, D 35). "Teaching model" is said once in hook (en and de). Research recommended a split of 63 / 77 by tower; the model gives 54 / 86. The model is authoritative, the clip states no per-tower number, so no defect here. Flag to the coordinator that `research-nakagin.md` is stale on the split. |
| F8 | Workbook `Nakagin.xlsx`, sheet order Program, Type A to D, Climate, Energy | pass | Matches the workbook model. German tab titles (Programm, Typ A to D, Klima, Energie) come from `sheet_titles`. |
| F9 | Excel error behaviour | pass | `#DIV/0!` after clearing the divisor, `#NAME?` for SUMM, `=B5*#REF!` after deleting row 6, `#VALUE!` for the text "3.8 m": all as Excel does. Only the side effect of deleting row 6 (surface row removed) is unmentioned. |
| F10 | Colour of the reference frames (Excel colours each reference and frames its cell) | pass | The narration matches Excel's edit-mode behaviour. |
| F11 | Dependents of B9 | pass | Engine returns Program C4, Type A B10, B15, B16. The scene shows only the visible ones (see `trace`). |
| F12 | German narration grammar and wording | minor | "Argumente trennt ein Semikolon, Dezimalzahlen ein Komma" (locale): "Dezimalstellen" fits better than "Dezimalzahlen". "Text, wo eine Zahl erwartet wird" (value): "wenn" is cleaner. "Merken Sie sich die Begriffe" (legend) breaks the "wir" register used elsewhere ("Vergrößern wir", "Beheben wir"). |
| F13 | English narration | pass | Grammar and terms are correct. "An empty length" and "here the length" should say "interior length" (see table). |

## Didactic findings

- Arc holds: hook, workbook, sheets, grid, address, range, bars, constant, display, formula, frames, hatch, precedence, function, locale, recalc, trace, the four errors, repair, legend. Each concept is taught on the smallest example (B2, B5 times B6, G12:G15).
- Display (format versus value) is shown with the card (misconception 1 addressed). It needs F1 fixed in German.
- Typical mistakes shown and fixed: #DIV/0!, #NAME?, #REF!, #VALUE! are each shown with cause and repair (repair is not animated, see table). Precedence is shown as an outline, but the mistake itself is never named (major, see table).
- Plan row gaps: "column" and "row" are narrated but not in the legend (the plan lists them); the storyboard's misconception "a formula needs the equals sign, text without it is only text" is not shown; the SUM argument annotation is missing; tab colours from the storyboard are not used.
- Dependents are narrated but not visible (see `trace`). Precedents only.
- Colour linkage contradicts the claim of "same colours" (see `frames`). A learner sees width yellow on the outside and blue inside.
- Cell B9 `=B5*B6` is the first formula (correct, shown with colour frames). Good.

## Known items (listed only, already assigned to the toolkit)

- Windows pop in without fade (Window cannot be copied, appear at clause boundary).
- The window tab bar fits only four tabs; the separate strip above shows all seven.
- German tab names come from the `_Titled` workaround (Typ A to D).

## Notes for the coordinator

- Remaining layout strict findings are mostly grid lines through text and card or tooltip overlays. The brief says "by design" for grid lines; the card and tooltips are not by design when they hide values (see table).
- The ghost bar is the largest visible regression; fixing it removes most of the `bars` and `grid` findings.
- Keyframes were reviewed as PNGs; animations (ramp, flip, hatch sweep) were not played.
