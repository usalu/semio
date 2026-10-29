# Quality review: grasshopper_anatomy (`Beat1_Anatomy`)

Read: all 24 en keyframes in clause order, de `count`, `mistake`, `warning`, `scene-grasshopper-anatomy.md`, `verify-grasshopper-anatomy.md`. Frames are stills; motion (slider drag, pulses) is judged from captions and scene notes only.

Overall: a solid, correct and well-linked clip (slider/wire/dimension colour linkage, live viewport). It is weakest where it is a catalogue of definitions on a mostly empty canvas, and where the typical mistake is only told, not shown.

## Ranked improvements

1. **Hook shows no building (S).** Clause `hook` shows only one capsule on an empty canvas. Open with the Nakagin tower (Tower A/B, 140 capsule slots) and zoom/dissolve into one lit capsule, so the capsule is plainly a part of 140. Why: the series promise is "the building is a script"; students need the whole before the part. Draw: reuse the tower view from `item`/`count`, outline one capsule, then fly into the dimensioned capsule.
2. **The typical mistake is told, not fixed (M).** `mistake` ends on "27 capsules" and `address` fixes it, but the wrong reading never produces a visibly wrong result. Show the 27 outlined capsules (already there) together with a struck-out "expected: 1", then in `address` shrink to the single capsule, with a connected before/after strip. Why: the wrong and the right reading must be one contrasting pair, otherwise the sentence is easy to forget. Also say what a student can do about it (Panel shows path and index first).
3. **Dead time in `sliders`, `live`, `flow` (M).** The three stills are identical apart from the caption; `live` promises a 3.0 m width but the still shows 2.5. Make `live` an actual drag: width 2.5 to 3.0 and back with the gold dimension line, the Panel value and the box all moving together, plus a 2.5 / 3.0 readout at the slider. Merge `sliders` and `flow` ownership of the pulse so each clause adds a new thing.
4. **Add a "why a script, not a model" beat (M).** Before `script`, give the answer to "how do we hand it to a computer": the same capsule drawn once, then four types plus 140 placements by the same definition. A 3 s strip (one definition, types A to D) motivates every following concept.
5. **Tower is tiny and unlabelled in the data clauses (M).** In `item`, `series`, `count` the tower is about 150 px wide and the highlight is a thin yellow outline. Enlarge the viewport for those clauses (or add a zoom inset), label the storey heights with dimension ticks at 0, 2.75, 5.5 ... on the tower side so each Panel row maps to a storey, and colour the list row and capsule the same.
6. **Series wiring is hard to read (S).** The slider outputs loop into S and N with crossing wires, and the S (start) input is unexplained. Tag the three ports Start / Step / Count (de: Start / Schritt / Anzahl) and keep one colour per wire; say "start 0" once. Why: the Series component is the first with more than one input.
7. **Tree in `tree`/`address` has no tower-side address (M).** A path `{0;0;3;1}` is four numbers; only the capsule outline links it back. Draw the path as a stepwise highlight: building 0, tower 0, storey 3 (storey band), facade 1 (facade arrow), then index 1. Why: the numbers become locations instead of notation. This is the part students find hardest.
8. **Error and warning teach cause but not fix (S).** `warning` and `error` explain the colour but the repair is only "fix upstream". Add a second half beat: change "tall" to 2.5 and show the component return to grey and the box to the viewport. Keep the real English Grasshopper message, with the German line.
9. **Plugin clause is abstract (S).** `plugin` shows three boxes and a tab but no result. Show one Ladybug component on the canvas with a sun-hours tinted window, even qualitative. Mark the install route as "see Package Manager in your version" (the research marks the path as `[CHECK]`).
10. **Legend frames are dense (S).** `legend_tool` and `legend_data` both carry badges, a table and a four-term caption; the captions only voice five of eight (de: ok, en: stops at "slider" / "index" without a German word). Voice all rows over time and light each table row when spoken; this makes the legends a recap that is actually consumed.

## Language and correctness notes

- `box` still labels the body "Center Box" while later frames show "Box". Given `parts` explains name versus nickname, this is acceptable if the caption in `box` says "Center Box (nickname Box)"; otherwise keep it as the open verify item N1.
- `valuelist` is fixed ("index 2 -> Type C"), `count` de uses "Anzahl" (fixed); de `Number` / `tall` text in the warning stays English by design (engine message), the German caption covers it.
- `item` caption "0 metres above the first capsule storey" is correct but clumsy; prefer "height 0 m: the first storey".
- de `count` caption says "Slider Anzahl" while legend teaches "Schieberegler"; pick one (Schieberegler first, then Slider).
- Numbers checked against the model summary in `verify`: 2.5 x 4.0 x 2.5 m, 25 m3, window 1.3 / 1.6 m, series 0..22 and 0..27.5, 27 branches for Tower A: consistent.

## Three things that work very well (keep)

1. Colour linkage: slider ring, wire into X/Y/Z, dimension line and label share one colour; the viewport is a live drawing from the engine.
2. `preview` ("eye hides drawing, not data": empty viewport with 25 m3 in the Panel) and the grey/orange/red state sequence with the actual null in the output Panel.
3. The tree clauses: Panel rows with the highlighted index across 27 branches ("3 of 27 branches shown") tied to outlined capsules on the tower; paths use the real Nakagin hierarchy.

## Summary

1. Strong correct anatomy clip; 24 clauses, bilingual, numbers match the model.
2. Biggest gaps: no whole-building hook, static middle section (`sliders`/`live`/`flow`), mistake not shown as a contrast pair.
3. Make the tower larger and map list rows and path numbers to storeys and facades.
4. Add repair beats to warning/error and a result to the plugin clause.
5. Minor wording: N1 "Center Box", "Slider" vs "Schieberegler", legend voicing; verdict stays READY after those.
