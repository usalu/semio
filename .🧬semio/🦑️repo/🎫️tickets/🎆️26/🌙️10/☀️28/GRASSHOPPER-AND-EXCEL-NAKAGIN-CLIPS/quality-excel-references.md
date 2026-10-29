# Quality review: excel references (`Beat2_References`)

Reviewed: all 22 en keyframes in clause order, de frames for f4, mixed2, threed, deleted, recap, the `NARRATION` of the scene file, `scene-excel-references.md`, `verify-excel-references.md`. Edited nothing else.

## Overall judgement

The scene is accurate and visually strong where it works (frames coloured like their references, capsule faces tinted by loss, tower lit by tower and type). The engine values are consistent with the Nakagin model. The weakness is the learning arc: 22 clauses with seven concepts in one video, the typical mistake arrives only at clause 17, and several clauses leave half the stage empty or abstract.

## Ranked improvements (max 10)

1. **Move `slide` + `repair` right after `absolute` (S).** Now: absolute (8) is followed by mixed2, mixed, fill2d, sheet, stuck, pertype, threed, span, names (9-16), and only then the dollar-sign mistake (17-18). A first-time student has forgotten the cause by then. Reorder to: ... absolute -> slide -> repair -> mixed2 ... The existing frames work unchanged; the recap already follows the same order. Learning gain: mistake and fix sit next to the rule.
2. **Cut or split the video: names, span (and threed) become a follow-up beat (M).** 22 clauses of about 8 s each is about 3 min of dense material. Names is a different topic (Name Manager) and the Name Manager card shows `theta_base`, which is never used. Keep `threed` as one clause, fold `span` into it as the same frame (the tab drag is the animation), drop `names` (or move it to the end as a one-clause teaser). Learning gain: less to hold, recap becomes five rows instead of six.
3. **Give `hook` a consequence on the Nakagin (M).** Now the frame shows only the sheet and one capsule, and the question "what follows when that cell changes?" is answered 8 s later. Draw the whole Nakagin tower on the right with type A capsules lit, and let B2 change 2.50 -> 3.00 m in the hook: the dimension arrow grows and D3, D4, D7, D9 flash yellow (dependants), then `pointer` explains why. The hook then lands on the building, not on a spreadsheet, as the brief demands.
4. **`pointer`: show every dependant, not only the roof (S).** The width change moves D3, D4, D7, D9 (roof, floor, front wall, back wall), but only the roof is hatched. Hatch all four faces and flash the four cells with a thin arrow from B2/B3 to D3. Add the length dimension next to the width dimension so "B2 and B3" are both visible on the capsule (blue = width B2, red = length B3, same colours as the frames).
5. **`lights` loses information and has no payoff (S).** After the fill every face is the same gold. Tint by U·A magnitude (window 3.09 W/K and floor/walls 6.00 W/K are the story), and finish with `=SUM(F3:F9)` in F10 = 30.92 W/K, which equals the "Transmission H_T 30.92 W/K" cell visible in the `hook` frame. That is a free, memorable "it adds up" moment and ties the opening to the middle. Optionally add G10 = 20 K x 30.92 W/K = 618 W against the 681 W heating load of the energy video (transmission only).
6. **`mixed2`: the example is not architectural and the right half is empty (M).** "Outer width x inner length = 9.5 m²" has no meaning for a student. Replace it with a table that does: U-values down (0.50, 0.60, 2.33) by temperature differences across (10, 20, 30 K), `=$A19*B$18`, or surface area down by U-value across. Draw the result as a small heat map bar chart beside the window, or if the footprint table stays, draw the capsule plan with outer and inner rectangle and light the rectangle of each filled cell (10.0, 9.5, 9.2, 8.7). Also add row/column labels (the corner reads "Area" only, the widths/lengths in A19:A20, B18:C18 are unlabelled).
7. **`stuck` should show the damage on both sides (M).** Wrong values (30.9 m², 3.1 m², 1051.3 m² totals) are visible but not marked, and the thumbnail of Type A has vanished (only a hatched plan remains). Red-frame C5:C7 with "expected 8.7 m²", and redraw the Type A thumbnail with a frame stepping B9 -> B10 -> B11 down the sheet (the same frame technique as `lights`), so the student sees where the reference slides. Note: totals use no thousands separator (`1051.3`) while D8 shows `1,223.6`: align the format.
8. **`deleted` hides the thing it talks about (S/M).** The caption says "the 3D sum quietly shrinks", but the sum cell (Energy B16) is not on screen; the card only prints `2,747.0 - 704.5 = 2,042.5 W`. Show the Energy sheet in a second small window (as `sheet` does for Type A) with B16 changing 2,747 W -> 2,043 W, no error icon, next to the `#REF!` window. The Type D row is also covered by the error tooltip, and the capsule tower lights all type C capsules although the sum is about one capsule: light one type C capsule instead. Use "2,043 W" in the card (704.5 rounds to 705 on a displayed 704 W; pick one number).
9. **Introduce the `Climate!` reference or defer it (S).** `fixed` shows `D10 =Climate!B2-Climate!B3` in the formula bar eleven clauses before sheet references are taught. Either say one half sentence ("D10 itself reads two climate cells"), or move the D10 definition to a plain typed `20 K` and let `sheet`/`names` introduce Climate. Also in `f4` draw a keycap `F4` that is pressed in each card and make the `$` appear in the formula bar of G3 (animation, not four static cards); add "Mac: Fn+F4" as a small note.
10. **German terminology and clause polish (S).** German uses "Art C", "Arten", "Kapselart" in narration but "Typ A..D" on the tabs and headers (`deleted`: "das Blatt der Art C" while the tab reads "Typ C"): use "Typ" everywhere. English `sheet`: "needs quotes" should be "needs single quotes (apostrophes)" to match the German "Apostrophe" and the glyph on screen. `recap` (de): the 3D rule text is set smaller than the other rows and the caption breaks as "Blatt-/und 3D-Bezüge"; shorten to "dieselbe Zelle auf allen Registern dazwischen" at the same size, or reword the caption. Name overlap: the two cards in `names` touch.

## Learning-arc notes

- Strong: smallest example first (D3 = B2*B3), then relative fill, then absolute with a reason (one input cell), then mixed, then cross-sheet. The recap table is the right closing form.
- Missing for a first-time student: why fill-down is used at all (no sentence on how to fill: double-click the fill handle / drag), and the keyboard shortcut for Mac. The `stuck`/`pertype` pair teaches a limitation and then the clumsy manual fix; one half-sentence that this is why tools like per-type tables exist would help. `pertype` shows no number on the capsule side, only a card; light the four sheets' thumbnails or show Type A..D tab strip with the frame jumping per tab.
- Redundant: `guard` repeats the second half of `deleted` with the same sum card; merge the `$` vs `#REF!` contrast into the `deleted` clause or give `guard` a new visual (the formula `=F3*$D$10` with a deleted D10 giving `#REF!`).

## Correctness checks

- Values: 36/35/34/35 = 140 per type, Tower A 54 and B 86 (fill2d table sums to 14+14+13+13 = 54 and 22+21+21+22 = 86), losses 100/120/120/120/59/62/38 W, U·A column sums to 30.92 W/K, 3D sum 681+681+704+681 = 2,747 W. All consistent with the model.
- Tool behaviour: relative shift, `$D$10`, `D$10`, `$D10` forms and F4 order (relative, absolute, row locked, column locked) correct. 3D reference semantics (tab order, moving a sheet out, deleting a sheet inside the span shrinks silently) correct. `#VALUE!` for `=F4*D11` where D11 is the text "Width" is correct. German names (`SUMME`, `ZÄHLENWENNS`, `#BEZUG!`, `Klima!`, `Programm`, `Typ A`) correct.
- German is natural overall; "Kreuztabelle" and "Fazit" are good. Replace "Register" by "Blattregister" once, then "Register" is fine.

## Three things that work very well (keep)

1. Reference frames and formula text share the same colours (blue/red/green/purple) in the window, the formula bar and the cards: students can read formulas by colour (`pointer`, `shift`, `mixed`).
2. Loss tint on the capsule (`absolute`, `slide`, `repair`) turns a wrong dollar sign into a visibly wrong building, and `repair` shows the same capsule fixed.
3. The recap table and the `f4` card (four forms, locked part named) are compact, accurate and visually calm; `fill2d` lighting the capsules of each tower/type cell ties a COUNTIFS table to the real building.

## Summary

1. Verdict: accurate and visual, but too long and in the wrong order; the typical mistake comes 9 clauses too late.
2. Biggest wins: move slide/repair after absolute, cut or split names/span, and make the hook show the tower with live dependants.
3. Replace the contrived `mixed2` example and add a payoff to `lights` (SUM = 30.92 W/K = H_T in the hook).
4. Make `stuck` and `deleted` show the damage on screen (expected value, Energy sum cell).
5. German: unify "Typ" (not "Art"), fix the small 3D row in the recap; English: "single quotes".
