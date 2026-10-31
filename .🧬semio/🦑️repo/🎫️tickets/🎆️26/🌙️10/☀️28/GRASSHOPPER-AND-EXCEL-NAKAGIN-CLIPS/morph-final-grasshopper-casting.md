# Final morph review: grasshopper casting (`Beat4_Casting`)

Judged from the scene code (838 lines), storyboard, done note, plan, general lessons and the en mid frames (types, text_number, storeys, ties, deliberate, wrong_way, explicit, domain, bool_int, locale, fail, order, matrix_fill, recap). Frames are 640x360, so code intent was weighed too.

## Per clause

| clause | rating | why |
| --- | --- | --- |
| teaser | ACCEPTABLE | Pure fade-in of the first stage; it is a hook, no idea is explained yet. |
| types | GOOD | Width bar, lit-storey hull and slot square on the real tower scatter (`_morph.scatter`) into the three parameter chips; origin visible on Tower B. Weak: the sources are small abstract marks, not the capsule dimension line the storyboard promised. |
| mismatch | ACCEPTABLE | Dashed `tether` from the Panel to the ghost capsule shows who asks for the value; nothing morphs, but the clause is a setup beat. |
| text_number | ACCEPTABLE | The chip `2.5` morphs part-wise into the capsule stand-in and the solid capsule appears (A to C works). The Text to Number change itself is still a `ValueTracker` token (`_token`, `live`) that swaps text and colour on the wire: that is a state sweep, not a retext, and the stand-in is oversized against the tower. |
| storeys | ACCEPTABLE | Integer chip `9` morphs into the gold hull, but the hull is a convex hull over the whole tower body, not 9 capsule storeys (frame shows it shifted left of the real tower B); the `.6 dropped` text appears from a tracker, not as a falling chip. |
| ties | GOOD | Hull morphs into the number-line dot, then the dot is driven by the slider (continuous quantity, tracker is justified); both ties show as arrows and the ring. Identity storey count to dot to ring is readable. |
| deliberate | ACCEPTABLE | Top-storey hull morphs into the red crossed tick (C to A, correct direction). The Floor component itself plays no part and the lost storey is a tick label, not a capsule vanishing from the tower. |
| point_plane | GOOD | Origin dot on the facade morphs to the `Slot origin` chip with a faint ghost left behind (lesson 2). The chip persists on the canvas. |
| wrong_way | GOOD | World plane token morphs into the translucent red capsule, then arrow, N/W and 90 degree tags arrive after it (frame confirms a clear red capsule on the facade). |
| line_plane | ACCEPTABLE | The line morphs into the plane axes with Z first; the abstract Line chip is not used, so the concrete line starts as a stroke from nowhere (FadeIn) rather than from the Line parameter. |
| explicit | GOOD | Origin, X-Axis and Y-Axis chips gather into the built plane; the red-to-green spin tracker shows the capsule turning into place. Frame shows axes on the green capsule. |
| domain | GOOD | Persistent capsule morphs into the domain drawing and the `4.0 m` chip morphs into the 0 To 4 bar with the cuts on the capsule length (frame: bar with 0 and 4.0 m on the edge). Strongest abstract to concrete link. |
| geometry | GOOD | Window ring morphs into the `Circle → Curve` chip and the capsule body hull into `Box → Brep`, both concrete to abstract. Chips are legible solid gold. |
| bool_int | ACCEPTABLE | Chip `1` travels along an arc and morphs into the window ring (frame: green chip on the window). The Toggle itself does not become the chip; the toggle flip is a tracker. |
| int_bool | ACCEPTABLE | Same mechanism with `True` chip into the orange ring; the number-to-Boolean cast (0, 1, 2) is shown by a tracker sweeping the slider while the chip waits, so the "any other value shows" idea has no visible chip per value. |
| locale | WEAK | The `2,5` chip floats above the Panel (frame: chip detached at top left of the canvas), then a `!` chip fades in; the capsule stays a plain ghost and nothing morphs or breaks. The failure is shown by a warning bubble, not by a morph that does not complete. |
| fail | WEAK | Ghost capsule morphs into a `null` chip, but the mid frame shows a small grey smear next to a red `null` chip; the Panel text `Type A` and the Center Box input do not take part, so the viewer cannot see that Type A is what died. |
| order | WEAK | The `null` chip walks to the ladder entry and cross-fades (`FadeTransform`) to `Type A`, i.e. the token changes identity by fade, and the mid frame shows an empty canvas with only `null` and the old caption; the ladder steps are then driven by a tracker. The promised "same Type A chip walks down the ladder" is not delivered. |
| fix | GOOD | The ladder's `Type A` entry chip morphs into the `2.5` chip, then into the capsule stand-in and the solid capsule: a real identity chain from the failed label to the corrected capsule. |
| matrix | WEAK | Everything fades; the capsule and the chips from the video are not the source of the matrix. This is only a lead-in to the next beat. |
| matrix_fill | ACCEPTABLE | Nine example pills travel on arcs (`travel`) into their cells and are replaced by `FadeTransform`; frame shows pills landing with text legible. The pills are newly made, not the chips seen earlier, so the "you have seen" link is by wording, not identity. |
| recap | ACCEPTABLE | Matrix cells gather into the slot capsule (back-link to the building, frame is clean), but the three habit cards fade in and are not connected to the chips or planes they refer to. |

## Overall verdict: POLISH

The video is clearly more morph-driven than before: 19 events, both directions, the chains point to plane to capsule and Type A to capsule are real, and the strongest beats (domain, wrong_way, explicit, point_plane, types, fix) show a visible abstract to concrete transfer. It falls short of "perfectly" in the failure half (locale, fail, order), where the failed token does not stay one object, and in the Text to Number and Integer casts, which are still tracker-driven token swaps.

### Five highest-impact changes

1. **order (line 813) and fail (line 808): keep one Type A token.** Create the `Type A` chip at the Panel `t` (`canvas12.w[...]`, before `fail`), let it travel to `Center Box`, crack into `null` there (retext or a split into red shards), and in `order` walk the same object down the ladder with `_morph.travel` and `retext`, instead of `FadeTransform(blank, entry)`. Also keep the ghost capsule on screen until the chip arrives so the grey smear in the fail mid frame disappears.
2. **locale (line 797): make the failure a morph that stops.** Start the `2,5` chip in the Panel (`canvas11.w["tde"]`, not above it), travel it to the Number input, then split the comma glyph off in red (`_morph.split`) and leave the ghost capsule outline-only; drop the free-floating `!` chip.
3. **text_number (lines 666-674) and storeys (lines 686-693): replace the tracker tokens by `retext`.** Use `_morph.retext` to turn the quoted Text chip `"2.5"` into the Number chip `2.5` at the input, and the `8.6` Number chip into the Integer chip `9` with the `.6` chip falling away, then morph into the capsule/storey result; keep the trackers only for the slider and ties.
4. **storeys (line 691) and deliberate (line 706): bind the chip to real storeys.** Morph the Integer chip into the nine lit Tower B storeys through `_morph.split` of per-storey `silhouette()`s (the band is a tower-wide hull and sits left of the real body in the frame), and in deliberate let the top storey outline itself lift off and shrink into the red tick so the Floor result visibly removes a storey.
5. **matrix_fill (line 829) and recap (line 838): reuse the earlier chips and connect the cards.** Keep the `Type A`, `2.5`, `Slot origin`, `1` and `True` chips alive (park them small at the canvas edge when their clause ends) and fly those same objects into their cells; in recap let the matrix cells gather into the three habit cards (the plane square and the Type A chip each landing on their card) before the cards' text appears, so the closing sentence is a morph, not a fade-in.

Minor polish: line_plane starts the line as a fade-in rather than from a Line parameter chip; int_bool should show a chip per value (1, 0, 2) instead of one waiting `True` chip; capture.py layout keyframes and contract tests were never run (noted in the done file).
