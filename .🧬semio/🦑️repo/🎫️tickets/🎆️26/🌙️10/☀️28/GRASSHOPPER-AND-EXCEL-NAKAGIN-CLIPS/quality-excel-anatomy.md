# Quality review: excel / anatomy (`Beat1_Anatomy`)

Reviewed: `T/plan.md` row, `scene-excel-anatomy.md`, `verify-excel-anatomy.md`, the scene NARRATION (23 clauses) and keyframes `🗑️generated/frames/📊️excel/Beat1_Anatomy/en` (hook, workbook, grid, range, formula, precedence, function, recalc, trace, div0, ref, display, legend) and `de/locale`. Some PNGs predate the last scene edit (for example `legend`, `ref`), so items marked "verify in a new capture" may already be fixed.

Overall: a strong, well-structured video. The smallest example (one capsule, B5 times B6) is drawn live with colour-linked references, and the error section is causal rather than a list. Weak spots are the hook, the middle of the arc (23 clauses is at the top of the 14 to 24 range, 3 to 6 min), the legend and a few places where the screen is a table and a still capsule.

## Ranked improvements

1. **Hook on the tower, but nothing links tower to cell (clause `hook` -> `workbook`), S/M.**
   The tower is shown, then the workbook appears beside a shrunken tower. Highlight one Type A capsule (lit, others dimmed) in `hook` and keep it lit in `workbook`/`sheets`, then fly the same capsule into the right-hand pane in `grid`. Why: the student sees that "Type A sheet = this capsule" instead of being told. Draw: dim 139 capsules to 20 % opacity, glow on one Type A, and a dotted line from it to the "Type A" tab (`Indicate` on the tab).

2. **Legend frame is a list, not a recap (clause `legend`), M.**
   Eight bilingual terms with numbered badges is the densest, most abstract frame of the video, and the badges collide (badge 4 sits on the B5 text, 2 is under the tab, 6 has no pointer; the sample `=SUM(F3:F9)` does not match the workbook's `=SUM(G12:G15)`). Replace with a sequential 8-step pulse: each term lights its object in the window for about 1 s while the Nakagin capsule shows the matching quantity (cell B9 lights the hatched floor; function lights the three fitting hatches). Keep one language per run (the EN video should not show German words in the list; show the German term only in the de run). Why: recall happens by pointing at the real thing again. Verify in a new capture that the `SUM(F3:F9)` string is gone.

3. **`precedence` is still the most abstract clause for a first-time student (S/M).**
   The frame shows a row of text under the capsule ("=2*B2+B3 = 9.00 m"), but the geometry says more than the text. Draw the wrong formula first: highlight B2 once, plus B3 once, so three sides light in red on the capsule outline (2 times 2.5 + 4.0 = 9.0 m, the second 4.0 m side dark and dashed), then add the brackets as an animation (a blue bracket slides in around `B2+B3`), the fourth side lights and the number morphs 9.00 -> 13.00 m. Why: the loss of one side is the whole lesson. Check that the wrong formula really appears in the formula bar (the earlier verification flagged it as missing; the current frame shows only B19). Effort S if the scene already swaps, M if the formula bar needs a second state.

4. **`recalc` shows only the end state (S/M).**
   Frame shows 2.50 m interior width, 9.5 m² and 20.0 m³ after the change but not the before/after. Show the old value ghosted beside the new (2.30 -> 2.50 m, 8.7 -> 9.5 m²) with a counter ticking in B5, B9, B10 and the floor hatch growing across the same second, and flash B10 and B15 as "also changed". Why: "changing width updates everything" is the promise made in the hook; the viewer must see the same number move in the cell and the drawing. Draw: ValueTracker on B5 driving window and hatch; a small `Δ +0.8 m²` chip.

5. **`ref` clause: row deletion confuses (S).**
   The frame shows rows renumbered after the deletion, interior height now in row 6 and the area in row 8, while the caption says a wall surface vanished that is not visible. Show the deletion explicitly: select row 6 (Interior length), strike it through, then slide the rows up, `B8` gets the `=B5*#REF!` and the dashed capsule edge asks "?". Rewrite the caption to name the deleted cell ("deleting the Interior length row leaves no cell to point to") and drop the wall-surface remark unless a frame shows it. Why: students otherwise think `#REF!` is mysterious rather than "my source is gone".

6. **No relative vs absolute / copy reference, and no link between sheets, S/M (new clause).**
   An architecture student's very next step is copying a formula to Type B and linking Program to the type sheets. Add one clause before `function`: copy `=B5*B6` from Type A to Type B (drag the fill handle across the tab bar or paste on Type B) and show Type B's capsule getting its own floor from its own width and length; optionally mention `$B$2` in one sentence. Why: this is the actual reason for "one worksheet per capsule type". Cost: 1 clause, so keep total at or below 24 by merging `name` into `div0` (see item 7).

7. **Pace: merge or trim to keep a 24-clause video watchable, S.**
   Merge `workbook`+`sheets` (tabs highlight one at a time while the sentence runs), `range`+`bars` (Name Box and formula bar update live while B2:B4 is selected), and `name` into `div0` as "three typical errors" with a triad strip (#DIV/0!, #NAME?, #REF!). Each merged clause keeps one visible change. Why: the first five clauses are all vocabulary on an unchanged capsule; students lose the thread before the first formula.

8. **Errors lack a "how to fix" beat (clauses `div0`, `name`, `value`, S).**
   Each error shows cause and code but only `repair` shows a fix, for `#VALUE!`. Add a one-second fix per error in the frame: type 3.80 into B6 (div0 clears), correct SUMM -> SUM (name clears), undo row delete (ref clears). Why: the brief asks for "the typical mistake and its fix". Draw: a green check flash on the cell as the error tooltip is replaced by the number. Also keep the tooltip text but label it as a teaching string if it is not the real Microsoft wording (still flagged F3 in verification).

9. **Trace arrows only point down in one column (clause `trace`, S).**
   Precedents (blue range B5:B6 -> B9) and dependents (B9 -> B14/B15) are the same blue and visually merge. Use two colours (precedents blue, dependents orange) and tint the matching region on the capsule (the width/length dimension lines glow for precedents; a small energy chip for dependents "Demand per area"). Why: Excel itself draws both as blue arrows, but for a first-time viewer a legend "uses / is used by" avoids confusion.

10. **Display clause is correct but static (clause `display`, S).**
    The card covers the right half and the capsule vanishes, so the tower-plus-tool rule is broken for this clause. Show 8.74 vs 8.7 m² (full-precision value in the card, cell shows one decimal), and toggle the cell between `0.0 "m²"` and `0.00 "m²"` so the viewer sees stored value constant while the display changes; keep the capsule hatched on the right (shrink the card). Why: this is the concept students most often misunderstand ("Excel rounded my number").

## Three things that already work very well (keep)

1. The colour-linked references in `formula`/`frames`: `=B5*B6` tokens, cell frames, floor dimension lines and hatch all share blue/red/green. It teaches references and dimensions at once; no other tool tutorial does this.
2. The real Excel chrome (title bar, Name Box, formula bar, tab bar, status bar with Average/Count/Sum) and the German UI with `SUMME`, `;` and `,` in `locale` side by side with the English form. Names and German terms are correct and natural.
3. The cause-driven error section (`div0` empties the interior length and the capsule shows a dashed red "?" edge; `ref` deletes a source and the same edge goes red): errors are explained on the building, not as abstract codes.

## Summary

- Verdict: strong structure and correct terminology; READY after items 1, 2, 3 and 5 are in.
- Biggest gain: link one lit Type A capsule from hook to sheet, and animate the two bracket/no-bracket formulas on the capsule edges.
- Fix the legend (stale `SUM(F3:F9)`, colliding badges, mixed-language list) and the `ref` caption/frame mismatch.
- Add one clause on copying a formula to Type B (relative references) and merge vocabulary clauses to keep at most 24.
- Re-capture after the last scene edit and check keyframes of `legend`, `ref`, `precedence`, plus 1080p parentheses.
