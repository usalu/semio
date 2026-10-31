# Morph storyboard: grasshopper casting (`Beat4_Casting`)

Scene: `A/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` (A = `🎓️teaching/🏛️architecture`). Binding: `T/morph-plan.md`. Clauses stay the 22 existing ones (en and de, captions unchanged); no clause is merged or added. Every number comes from the engine (`evaluate`, `cast`, `rules`) or the Nakagin model, as now.

## Idea

Casting is about **what a value is while it travels**. So the one object that travels is the token, and the token is always some part of the Nakagin capsule that changes form: a dimension line becomes the Number chip, the Text chip becomes a Number, the Number becomes an Integer, the Integer becomes lit storeys, a point on the facade becomes a Point chip, then a plane square, then the capsule that stands on it. Failure is a morph that does not complete: "Type A" stops, cracks and leaves a `null`. The matrix at the end is assembled from the very chips the viewer has already followed.

Token palette (carried through all clauses): Text = gold ink (`INK_GOLD`), Number = teal (`INK_TEAL`), Integer = gold-green (`INK_GREEN`), Plane = green slot colour (`CAP_GOOD`), world plane and failures = red (`CAP_BAD`/`INK_RED`), Boolean = orange (`INK_ORANGE`).

## Morph beats (16)

Direction: C→A = concrete to abstract, A→C = abstract to concrete. "Carries" = what stays identical across the morph.

| # | Clause | Concrete object (Nakagin / viewport) | Abstract object (canvas) | Dir | Carries over | Time | Narration while it happens |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `types` | capsule A width dimension line (2.50 m), the lit storey column of Tower B (11), the slot plane square on the west facade | three type chips on the canvas parameters: Number `2.50`, Integer `11`, Plane `{0;1;3;3}` | C→A (scatter, staggered) | colour (teal, gold-green, green), value, order left to right | 1.6 s | "every parameter has a type: width is a Number, storeys an Integer, placement a Plane" |
| 2 | `mismatch` | ghost capsule (outline only, no width) | Text param chip `"2.5"` waiting in the Panel | A↔C tether | gold text colour; dashed tether from Panel to the ghost shows which capsule asks for the value | 1.0 s | "When the types differ, Grasshopper tries to cast the data." |
| 3 | `text_number` | the capsule's width dimension line 2.50 m, ghost becomes solid | the quoted Text chip `"2.5"` riding the wire, then the Number chip `2.5` | A→C (retext on the wire, then token stretches into the dimension line) | the digits 2.5 and gold-to-teal colour change at the input; width of the dimension equals the chip value | 2.0 s | "The Number input reads it as the number 2.5, and the capsule becomes 2.5 metres wide." |
| 4 | `storeys` | Tower B storeys lighting bottom-up, numbered ticks | slider value chip `10.6` Number, then Integer chip `11` (".6 dropped" chip falls away) | A→C (split: the Integer chip splits into 11 lit storeys, staggered) | value 11, gold-green colour, storey order | 2.0 s | "A slider says 10.6 ... Grasshopper rounds, here to 11." |
| 5 | `ties` | lit storeys 2 and 4 of Tower B | number-line dot at 2.5 and 3.5; circle at 2 and 4 | C↔A (dot morphs along the arrow into the ring; lit storeys gather into the Integer chip, then split again) | the value 2.5 / 3.5 and the resulting storey count | 3.0 s | "at exactly .5 it rounds to the even number: 2.5 gives 2, 3.5 gives 4" |
| 6 | `deliberate` | the 11th storey (lit outline on the tower) | red crossed ghost tick `11` and the Floor result chip `10` | C→A (the storey outline lifts off, shrinks into the ghost tick) | position of the 11th storey, red for dropped | 1.4 s | "the eleventh storey disappears" |
| 7 | `point_plane` | slot origin dot on the west facade, then the plane axes square at that point | Point chip `Slot origin` flying to the Point parameter, then Plane chip back to the facade | chain: C→A (dot becomes Point chip, tether kept), then A→C (chip becomes the plane square with world axes) | position x/y/z of the origin; chip colour becomes the plane green | 1.6 s | "A Point ... becomes a plane at that point with the world axes." |
| 8 | `wrong_way` | red capsule on the world plane (translucent), window arrow, 90° arc | the plane square (world axes) | A→C (the plane square morphs into the red capsule outline, the window arrow grows out of it) | plane origin, axes directions, red | 1.6 s | "The world plane points it north: the window turns by 90°." |
| 9 | `line_plane` | the core-to-slot line on the facade, then the plane whose Z follows it | Line chip `Core to slot`; Plane chip | chain: A→C (Line chip becomes the line stroke), then C→C (line stroke morphs into the plane's Z axis, X and Y grow with "guessed" dashes) | line endpoints = Z axis ends, blue Z colour | 1.6 s | "A Line becomes a plane whose Z axis follows the line." |
| 10 | `explicit` | the red capsule rotating into the green slot capsule, plane at the slot | three chips Origin, X-Axis, Y-Axis converging into Construct Plane | A→C (gather: chips collapse into the component, then the plane square grows on the facade; red capsule morphs into the green one) | origin and axis vectors, red turns green | 2.4 s | "Build the plane explicitly ... the capsule turns into place and faces outwards." |
| 11 | `domain` | capsule A length edge (4.0 m dimension), the cuts along the length | Number chip `4.0`, Domain bar `0 To 4`, segment tokens | chain: C→A (length dimension becomes the Number chip, then the Domain bar), A→C (bar splits into Divide Domain segments that drop back as cut ticks on the capsule) | 4.0 and the segment boundaries | 2.2 s | "the capsule length 4.0 becomes the interval 0 To 4, and Divide Domain cuts it" |
| 12 | `geometry` | window outline of the capsule, the capsule body (six faces) | Curve chip, Brep chip | C→A twice: the window circle shrinks into the Curve chip; the box unfolds into six face tiles (unfold) that stack into the Brep chip | the circle and the six faces; gold highlight | 2.0 s | "A Circle is a Curve and a Box is a Brep" |
| 13 | `bool_int` | the window disc on the capsule (appears or vanishes) | Toggle knob, Boolean chip `True`, Integer chip `1` | A→C chain: knob slides, chip `True` retexts into `1`, then into the window disc on the capsule; `False` shrinks the disc back into the chip `0` | orange to gold-green, the disc follows the chip | 2.4 s | "False is 0, True is 1, and the window follows." |
| 14 | `int_bool` | the window disc | slider value 1, 0, 2 as Number chips, Boolean result chip | A→C (chip `2` retexts to `True` and morphs into the disc; chip `0` retexts to `False` and the disc folds into it) | the number, orange/green | 2.6 s | "0 hides it, any other value shows it" |
| 15 | `locale` | ghost capsule | Panel text `2,5` chip hitting the Number parameter | C→A failure morph: the chip reaches the input, its comma glyph splits off red, the ghost capsule stays outline-only | the glyphs 2 , 5, red on the comma | 1.8 s | "the text 2,5 may fail as a Number" |
| 16 | `fail` + `order` + `fix` | ghost capsule, then the correct capsule with 2.50 m | the `Type A` chip: travels, stops at Center Box (crack, `null`), keeps its identity on the ladder, then a Value List `Type A = 2.5` morphs into the number and the dimension | C→A (ghost capsule collapses into the `null` Panel), A→A (the same chip steps down the ladder), A→C (Value List item retexts to `2.5`, chip stretches into the dimension line, ghost becomes solid) | the label "Type A" through three clauses; width 2.5 at the end | fail 1.6 s, order 2.6 s, fix 1.8 s | "Type A fails every step ... A Value List delivers the real width of Type A." |
| 17 | `matrix_fill` | the example chips seen before: `"2.5"`, `10.6 → 11`, world XY, Z = line, `0 To 4`, outline, six faces, `True → 1`, `0 → False` | their matrix cells | C→A (scatter from the stage positions into the cells, staggered) | chip text and colour become the cell text and colour | 3.0 s | "Each marked cell is a cast you have seen" |
| 18 | `recap` | the correct capsule standing on its slot | the three habit cards (Panel, plane, Value List) and the matrix | A→C (gather: the matrix cells collapse into the three cards, the cards shrink into the slot capsule) | green, the slot capsule | 1.8 s | "Three habits ... " |

Beats 7, 9, 11, 13, 14 and 16 are chains: the same object takes at least three forms (dot → Point chip → plane square; Line chip → line → Z axis; length edge → Number → Domain → cut ticks; Type A chip → null → ladder token → Value List item → dimension).

Count: 18 beats of which A→C 9, C→A 6, chains 3. The brief range 8–14 is exceeded on purpose where one clause carries two forms of the same object; the essential set (one idea per clause) is beats 1, 3, 4, 7, 8, 10, 11, 12, 13, 16, 17, 18.

## Replaced fade-swaps

- `types`: the three decorative chips under the tower are removed; they are born from the tower.
- `text_number`, `storeys`: the token that rode the wire in two forms now has the chain Text → Number → dimension line and Number → Integer → lit storeys.
- `point_plane` → `wrong_way` → `line_plane` → `explicit`: no more fade of the whole stage between clauses; the plane square becomes the capsule, the line becomes the Z axis, the chips become the plane.
- `fail` → `order`: the ladder no longer appears from nothing; the failed chip walks down.
- `matrix_fill`: cells are filled by flying chips, not by fade-in.

## Implementation notes (Phase B)

- Toolkit: `🎓️teaching.🏛️architecture.🔀️morph.🐍️` (`morph`, `scatter`, `gather`, `split`, `unfold`, `retext`, `token`, `tether`, `travel`, `silhouette`, `proxy`); tower groups only via `proxy()`.
- The existing live trackers (`value`, `flag`, `level`, `spin`) stay; morphs animate between steps, the live redraws remain the end state.
- Heavy groups: the tower (`_tower`) is never morphed per face; storeys light via `silhouette()` of the lit rows.
