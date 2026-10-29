# Quality review: excel / program (`Beat3_Program`)

Reviewed: `plan.md` row, `scene-excel-program.md`, `verify-excel-program.md`, NARRATION (24 clauses), all 24 English keyframes in clause order, and de keyframes `countif`, `towers`, `xlookup`, `typo`. Frames dated Oct 29 02:06-02:17 (latest capture). Caveat: the verify report lists open clipping items (formula bar over header row in `table`, `countifs`, `towers`; "Type..." in C10; "Sum..." de); the `towers`/`countifs` frames still show the two-line formula bar covering column letters, so those items are still open and are not repeated below.

Overall: a clean, correct video with a good spine (fittings -> area -> 140-row table -> COUNTIF -> SUMPRODUCT -> swap -> sanity check). Cell values, unit formats, colour-linked references and the lit-capsule counting are strong. The weaknesses are didactic: the answer to the hook arrives late, the central calculation (count x area) is trivial because all four types have the same net area (8.74 m2), and several clauses show the tool with an empty or decorative drawing.

## Ranked improvements

1. **SUMPRODUCT and the swap prove nothing, because every type has 8.74 m2 (clauses `pull`, `sumproduct`, `chart`, `update`), M.**
   The result 1,223.6 m2 is just 140 x 8.74; the chart bars (314.6 / 305.9 / 297.2 / 305.9) differ only by count; the swap ends with "the total stays", which teaches that the formula is pointless. The types differ in fittings and free floor (5.1 / 4.4 / 3.3 / 5.1 m2, already drawn in `types`). Change `pull`/`sumproduct`/`chart`/`update` to use **free floor area** (net minus fittings) as the second operand (Type sheet B16, already computed): SUMPRODUCT(counts, free areas) is then a real, different number, bars differ clearly, and the swap C -> A visibly changes the total (+1.88 m2: 5.14 - 3.26). Keep net area as one extra column (`Net area`) so the total net area 1,223.6 m2 remains the check. Draw: each tower capsule tinted by its type, the free-floor hatch inside the plan chip beside each bar. Rewrite `update` to "the total changes by +1.9 m2: type A has more free floor than type C".

2. **The hook asks "how many of each, how much floor?" but the first answer comes at clause 11 of 24 (clauses `program` ... `volume`), M.**
   Nine clauses (about 40 percent of the video) stay on one capsule before any counting. Cut the lead-in: merge `area`+`fill` (the fill-down shows all three fittings at once), and merge `net`+`free`+`volume` into two clauses ("net area" / "free area and volume"). That gives 20 clauses and puts COUNTIF at clause 8. Alternatively keep the order and show in `hook` the answer as a teaser: the tower flashes its four colours with counts 36/35/34/35, then fades back to grey with "? m2" for the area. Why: a first-time student needs the payoff in sight to sit through the vocabulary.

3. **`program` is the emptiest frame and `free`/`sheet` leave half the stage unused (clauses 2, 3, 7), S/M.**
   `program` shows an empty capsule outline and three chips ("fittings", "areas", "users") that point at nothing, and "users" is never drawn anywhere (the occupant count 1 in `free` is only a number). Draw: in `program` the three chips pull their content in one at a time (fittings: the bed/bath/storage rectangles, areas: the dimension lines, users: one pictogram of a person in the capsule, placed on a free spot of the plan). In `free` show the pictogram next to B17 so "typed-in constant" has an object, and in `types` show the 1 person on every plan. Also align the Excel window at the same height in all clauses; in `free` the window jumps down and leaves a large empty top band.

4. **The structured table (clause `table`, `countif`) is shown, never made, M.**
   The student sees a table named `Capsules` and the notation `Capsules[Type]`, but not how to get it (Insert > Table / Ctrl+T) nor why it beats `M2:M141` (grows with new capsules). Add a short sub-step to `table`: the raw range M1:M141 gets a table frame, the name box shows "Capsules" being typed under Table Design, header row banding appears. Then `countif` keeps only one idea (COUNTIF counting) and the callout "Capsules[Type] = M2:M141" is drawn as a bracket along the column in the sheet instead of as a floating chip under the tower. Also in `countif` narration, split the second sentence: "the range M2:M141" and "lights up in the colour of type A" are two ideas.

5. **`typo` is the best teaching beat but the grey capsule is almost invisible, and `fix` shows no fix (clauses 22 and 23), S.**
   In `typo` one pale-grey capsule at lower left of the tower is the whole point; it is easily lost among 139 coloured ones. Draw: dim the other 139 capsules to 35 percent, ring the grey one in the error colour with a leader to the row `A-01-N-2` in the list, and show the cause as `"Type B "` with quotation marks in the formula bar (the space is then literally visible) plus `LEN = 7` vs `6`. In `fix` show the cursor deleting the space (or `=TRIM(M3)` / data validation as the prevention) and let the capsule fly back to orange; right now the frame is identical to the earlier state with no visible change. Add one sentence of prevention in `fix`: "Use a drop-down list (Data Validation) so the type can only be A to D." That is the architect-relevant lesson.

6. **`towers` does not show towers (clause 18), S.**
   Tower A and Tower B appear as two identical chips over one grey drawing, and the sum chip below ("472.0 + 751.6 = 1,223.6 m2") is not tied to anything. Draw: tint Tower A (14+14+13+13 = 54 capsules) light, B (86 capsules) dark; selecting F11 lights Tower A in the grey-white, F12 Tower B, F13 both with the sum bracket; show the capsule counts (54 / 86) beside the chips so 472.0 = 54 x 8.74 can be checked mentally. Same technique makes `countifs` readable: the lit capsules are 14 tiny outlines on a grey tower; tint the whole selected tower subtly and use bright fill, not yellow outlines, for the counted type.

7. **`pull` and `xlookup` show the same fact twice and the drawing adds nothing (clauses 14, 15), S/M.**
   `pull` draws four identical hatched plans with 8.7 m2 under each; `xlookup` draws one hatched plan with 8.7 m2 and a count. XLOOKUP is introduced as "returns area and count" but the frame shows two separate result cells with only one formula visible (the second one is a different formula). Fix with improvement 1: `pull` brings **free area** from the four type sheets with a travelling copy of the sheet tab, `xlookup` finds "Type C" in the key column with a moving frame over A4:A7, the match row lights, the returned value flies to B15 and the matching plan lights. Also mention that XLOOKUP needs Microsoft 365 / Excel 2021 (older: INDEX/MATCH), since an architecture office may still run Excel 2016.

8. **Chart carries no insight (clause 17), S.**
   Four bars of 297 to 315 m2 on a 0-400 axis look the same; "each bar grows as its cell computes" is the only dynamic statement. With improvement 1 the bars differ. Additionally draw each bar next to its type's plan chip, and use a stacked column: fittings (hatch pattern) + free floor (plain) = 8.74 m2 x count. Then the chart really teaches "what do 140 capsules offer". Keep the type colours; the bar labels are readable.

9. **German terminology and table name (de frames), S.**
   The table is named `Capsules` in the German workbook (`ZÄHLENWENN(Capsules[Typ];A4)`, callout `Capsules[Typ] = M2:M141`, narration "Tabelle Capsules") while columns, tabs and function names are German. Name it `Kapseln` in the German workbook (`Kapseln[Typ]`) and in the German callout and narration. Terminology: the narration says "Nettogrundfläche" (DIN 277 NGF) in `net`, then "Nettofläche" in the sheet and later clauses; use one term (Nettogrundfläche, NGF) everywhere, with the column header abbreviation "NGF" if width is a problem. `Platz` as the name for item is understandable; "Position" is closer to the Nakagin slot.

10. **Recap is a vocabulary list again (clause 24) and the typo frames repeat the Metabolism point in too few words (clauses 19, 24), S.**
    The recap shows five function chips and the tower, with the chips in plain white. Replace by a sequence that re-lights the real objects for about 1 s each: list -> all 140 capsules appear row by row; COUNTIF -> four colours; XLOOKUP -> Type C plan; SUMPRODUCT -> free-floor hatches fill; SUMIF -> two towers tinted; end on "140 capsules, 1,2xx m2". For `swap`, add the Metabolist argument explicitly: the capsule is unplugged from its core and replaced; draw the capsule lifting off the tower and a new one going in, since that is the architectural point of the Nakagin (the list change is only the data side).

## Smaller notes (not ranked)

- Narration: `free` carries two ideas (subtraction and constant); `table` two (table, row selection); keep one idea per clause or split.
- `hook`: the tower is grey while the question is about four types; show the four colours behind the empty Program table at the end of the hook clause and fade.
- `update` chips "+1" and "-1" float above the window, far from B4 and B6; move them to the cells.
- `sumproduct` tile labels use 8.74 m2 while the sheet shows 8.7 m2; fine pedagogically (display vs stored value) but say it once ("shown 8.7, stored 8.74").
- Correctness check: all counts 36/35/34/35 = 140, tower 14/22 split, areas 8.74 m2, volume 18.354 m3, 1,223.6 m2 = 472.0 + 751.6, `COUNTIF` not matching `"Type B "` are all correct Excel behaviour; German function names, `;` and decimal comma are correct. Numbers match the verify report.

## Three things that already work very well (keep)

1. **Counting made visible (`countif`, `rest`, `countifs`)**: the counted capsules light up on the tower in the type colour, the counter on top ticks to 36 and then 140, and the filled-down dashed frame shows the criterion walking from A4 to A7. This is exactly the "cell = drawn object" bar of the series.
2. **Area from references (`area`, `fill`, `net`, `free`, `volume`)**: the formula `=D12*E12*F12` has colour-matched frames in the sheet and in the formula text, the bed is hatched while its width and length are dimensioned in yellow, then fill-down hatches the other fittings; the volume clause adds a real stored-value vs displayed-value card with the correct number format. Smallest example first, generalised smoothly.
3. **The typo and sanity check (`check`, `typo`, `fix`)**: a causal, concrete error (an invisible trailing space) with a red marker, a 139 != 140 chip, a TRUE/FALSE badge and a grey capsule is exactly the "typical mistake and its fix" beat, and it is native to what students will really do. Strengthen it per item 5 rather than replace it.

## Summary

1. Verdict: strong, correct structure; not yet "top quality" because the totals are trivial (all types 8.74 m2) and the answer to the hook comes late.
2. Biggest lever: use free floor area as the second operand so SUMPRODUCT, chart and swap change real numbers (item 1) and shorten the single-capsule lead-in (item 2).
3. Make the table, the typo capsule and the two towers visibly distinct on the tower drawing (items 4, 5, 6); fill empty stages in `program`, `free`, `xlookup`.
4. German: rename the table to `Kapseln` in the German workbook and unify Nettogrundfläche.
5. Keep: lit-capsule counting, colour-linked area formulas with hatches, trailing-space error beat.
