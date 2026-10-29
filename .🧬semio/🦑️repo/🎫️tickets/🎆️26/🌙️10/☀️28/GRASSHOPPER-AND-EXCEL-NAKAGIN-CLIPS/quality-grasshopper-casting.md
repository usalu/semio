# Quality review: grasshopper casting (`Beat4_Casting`)

Reviewer: demanding teaching-video director (read-only). Read: all 21 English keyframes in clause order, German `fail`, `ties`, `recap`, scene report, verify report, storyboard. Goal: top-quality, fullblown, visual, didactically rich.

## Overall verdict

Technically clean and honest, but didactically thin: it is a tour of 10 small casts on a mostly static single capsule, with a hook that is a definition, and a heavy "check in Rhino 8" disclaimer on 12 of 21 frames that erodes the viewer's trust in what they are learning. The two strongest ideas (point becomes plane and the capsule faces the wrong way; Text "Type A" fails and a Value List fixes it) are good, but the wrong-way drawing is hard to read and the Nakagin disappears after clause 9. Fixing the items below makes it a clear step up.

## Ranked improvements (max 10)

1. **Verify in Rhino 8 now and remove the disclaimers (clauses 3, 5, 6, 8, 9, 10, 12, 13, 14, 16, 18, 21). Effort M.** A student sees "teaching assumption, check in Rhino 8" on over half the frames and cannot tell what to believe. Film or run each case once (Text "2.5" to Number, 10.6 to Integer, Point to Plane, Line to Plane, Number to Boolean, Number to Domain) with Param Viewer and Panel, store the results in the model with basis `verified`, and keep the marker only for what genuinely stays open. Drop any case that cannot be verified (Number to Domain is the weakest). A single "checked in Rhino 8" source line in the recap is stronger than twelve hedges.

2. **Open with the stake, not the definition (clause 1 hook). Effort S.** Today: three hexagons with type chips plus a tower, nothing wired, nothing happening. Instead play a 5 s teaser: one capsule dropped on a facade slot via a plain Point, its window staring along the wrong axis next to the correct one, then "why?". Resolve it in clause 11. The three type chips (Width Number, Storeys Integer, Plane) then come in as the answer. Highlight on the tower what each chip means (one capsule width dimension, the lit storey column, the slot plane); at present the right viewport is decorative.

3. **Make the wrong-way and explicit clauses read as architecture (clauses 8 to 11). Effort M.** The plan view shows unexplained red and green rectangles, a "Core" box, abstract arrows, and the viewport is still captioned "Perspective" although it is a plan (rename to "Top"). Draw a real 3D capsule with window disc on both planes: translucent red wrong capsule (world plane), solid green correct capsule, window arrows in the same colours, a west/north compass and a 90° arc between them. Name the two capsules in a legend ("world plane" / "slot plane"). In `explicit` animate the red capsule rotating into the green one while the wire turns green. This is the key lesson for architecture students and should get the most screen time.

4. **Show the cast itself happening on the wire (clauses 3, 5, 15, 16). Effort M.** In `text_number` the Panel shows "2.5" on both sides, so nothing visibly changes. Draw a token travelling the wire that changes type badge and form: `"2.5"` (Text, in quotes, left aligned) to `2.5` (Number, right aligned), and for storeys `10.6` to `11` with the discarded `.6` falling off. In `bool_int` and `int_bool` flip the toggle/slider live and show the window appearing and disappearing on the capsule; drop the Ø 1.30 m dimension there (distraction, and its gold on light grey is low contrast).

5. **Replace the abstract ladder (clause 4) with a worked example, or move it later. Effort M.** Four text boxes with CastTo/CastFrom jargon, before the learner has seen a single successful cast, while the right viewport shows an irrelevant static capsule. Run a token down the ladder (the "2.5" Text stops at step 3, "Type A" falls through to step 4 and turns red) and place the clause after `ties`, or after `fail` as the explanation of why it failed. Cut CastTo/CastFrom from the caption; say "the source tries, then the target tries".

6. **Make the lit-storey drawing countable (clauses 5, 6, 7, 21). Effort S.** Gold outlines on the teal tower are hard to count, and the viewer must trust the "11". Add numbered storey ticks along one facade column and a before/after ghost: in `deliberate` draw the 11th storey as a red ghost outline when Floor drops it to 10; in `ties` show 2 lit then 4 lit storeys with a pop-in count tag. The Number-to-Integer lesson becomes physical: one storey more or less of the tower.

7. **Fix the matrix so it teaches the pattern, not just a record (clauses 19, 20). Effort M.** Ten sparse cells of 56, the 2.5 cell is squeezed and unreadable, and empty cells are ambiguous (not tried, or not possible?). Group rows and columns into families (numbers, truth values, geometry), grey-hatch the cells that are not castable, add the cell most students meet daily (Integer to Number, always fine) and Number to Text, and show a one-sentence rule on screen ("moving along a family works; across needs a component"). Replace the first row's tiny "2.5" with the same width as the others. Put the capsule thumbnails left of the matrix as in the storyboard.

8. **Cut or merge the two trivial geometry casts (clauses 13 and 14). Effort S.** Circle to Curve and Box to Brep are near-static frames (one yellow outline, six face chips) that carry little learning and both need the marker. Merge them into one clause "Circle, Box, Line are all special Curve or Brep: Grasshopper treats them as the general type", with the capsule body and window highlighted in one move. The freed time goes to items 3 and 4, and to a missing common mistake (see 9). Alternatively replace them with the daily case Integer slider to Number input (units: millimetres vs metres).

9. **Add the classic architecture mistake: units and locale. Effort M.** Missing for a first-time student: a Panel that reads "2,5" in a German Windows Rhino, or a slider in mm feeding a width in m. Show the Panel "2,5" (German) arrive in the Number input, then the capsule jumps in size (2.5 mm or a fail), and state the fix (set the Panel text with a point, or use a Number Slider). Fits after `fix` and before the matrix.

10. **Rework the recap into three habits (clause 21). Effort S.** The English frame carries a bilingual glossary (Casting · Typumwandlung) which an English viewer cannot use, and the lower half is two small Panels with another marker. Show language-only terms, and three habit cards each tied to a scene already seen: "Always check with a Panel (10.6 to 11)", "Build planes explicitly (the capsule faces out)", "Labels are not numbers; use a Value List". End on the correct capsule on its slot, not the lit tower alone.

## Further small points (not ranked)

- Contrast: gold dimension and axis labels (`2.50 m`, `4.0 m`, domain tick `0`) are ink on light grey, about 2:1; fill them with the dark plate used for `Ø 1.30 m`.
- German: the number line prints `2.5`, `3.5` with dots while slider and Panel print `3,5` (in `ties` the Panel even prints `3 , 5` with a gap); viewport label "Perspective" is English in German; the Panel text `Plane at x -13,85` stays English (acceptable, it is the component's own output, but label it).
- `fail` (de): the English error balloon beneath a German balloon is clear and now inside the canvas, keep it.
- `fail`: the empty output Panel beside Center Box should say "null" so the viewer reads blank as a value.
- Clause 12 to 16 caption lines are short and have empty stage space; use it for the type chips of both ends of each cast.

## Three things that already work very well (keep)

1. **Fail then fix pair (clauses 17 and 18).** Red Center Box, the exact message in a red balloon, ghost capsule with "no width, no capsule", then the Value List with "Type A = 2.5" turning the component grey and the capsule solid. A clear mistake and repair on the Nakagin capsule type, in both languages.
2. **Engine-driven values and honest sourcing.** Everything on screen (Panels, lit storeys, planes, the failure text) comes from the engine and model, with McNeel/Rutten source lines and the dashed matrix cells. The strict evidence handling is a house-quality strength; just reduce the volume (item 1).
3. **Number line with ties to even (clause 6).** The 2.5 to 2 and 3.5 to 4 arrows plus the lit-storey count make the least intuitive behaviour concrete, and the slider-driven repeats keep one idea per clause.

## Summary (5 lines)

1. Clean and correct, but a catalogue of casts with a definition hook; the best moments are fail/fix and ties.
2. Biggest gains: verify in Rhino 8 and drop most "check" markers; open with the wrong-way capsule teaser.
3. Redraw wrong_way and explicit as 3D capsules with window arrows and a 90° arc; animate the casts on the wire.
4. Move or rework the abstract order ladder; merge the trivial Circle/Box clauses; add the units/locale mistake.
5. Rebuild the matrix into families and end the recap with three habits, English-only terms in the English video.
