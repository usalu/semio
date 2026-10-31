# Morph done: grasshopper casting (`Beat4_Casting`)

Scene: `A/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py`. Storyboard: `T/morph-grasshopper-casting.md`. Frames: `T/🗑️generated/morph/🦗️grasshopper/Beat4_Casting/{en,de}/` (`index.md` lists every animation).

## Morph beats implemented (19 events, all clauses with a morph)

| Clause | Morph | Direction |
| --- | --- | --- |
| types | width bar, lit-storey hull and slot plane square on the tower scatter into the three type chips | concrete to abstract |
| text_number | Number chip `2.5` morphs part-wise (box to capsule hull, text to window ring) into a stand-in, the capsule fades in | abstract to concrete |
| storeys | Integer chip morphs into the large gold storey hull on Tower B | abstract to concrete |
| ties | no morph; tower is now one static shell plus a live hull/tick overlay (fast) | |
| deliberate | top-storey hull morphs into the red crossed tick | concrete to abstract |
| point_plane | slot origin dot morphs into the `Slot origin` chip | concrete to abstract |
| wrong_way | world plane square (red) morphs into the red capsule, window arrow follows | abstract to concrete |
| line_plane | the core-to-slot line morphs into the plane axes (Z axis first) | concrete to concrete (line to plane) |
| explicit | Origin, X-Axis, Y-Axis chips gather into the built plane square | abstract to abstract (gather) |
| domain | Number chip `4.0 m` morphs into the 0 To 4 bar with the Divide Domain cuts on the capsule length | abstract to concrete |
| geometry | window circle morphs into the `Circle → Curve` chip, capsule body hull into `Box → Brep` | concrete to abstract |
| bool_int | chip `1` morphs into the window ring on the capsule | abstract to concrete |
| int_bool | chip `True` morphs into the window ring | abstract to concrete |
| locale | chip `2,5` morphs part-wise into a red `!` chip | abstract failure |
| fail | ghost capsule morphs into the red `null` chip | concrete to abstract |
| order | the `null` chip morphs part-wise into the `Type A` entry chip of the ladder (same token as in fail) | abstract chain |
| fix | Value List chip `2.5` morphs part-wise into the capsule stand-in, capsule fades in | abstract to concrete |
| matrix_fill | the example chips seen before scatter from a row into their matrix cells | concrete to abstract |
| recap | the matrix cells gather into the correct slot capsule | abstract to concrete |

Chains: slot origin dot to `Slot origin` chip (point_plane) then plane square (wrong_way) to red capsule; line to plane axes to built plane; capsule to `null` chip (fail) to `Type A` entry (order) to Value List result to capsule (fix).

## Mechanics

- `then(after, secs, seconds)` queues a morph that plays after a clause's first animations and before the narration hold; the `step` override reads it (works under `capture_morph.py`). `_later` delays a FadeIn inside its run.
- Part-wise `pairs=` is used for chip to capsule (`_standin`: body hull plus window ring) and chip to chip; `capsule_pairs` from the newer toolkit is not needed because the stand-in parts are built by the scene.
- Storey hull: `_band` is a convex hull around the whole Tower B body for the storeys shown (`_lit` = hull + count tag + ticks). The tower is drawn once as a static shell, only the cheap hull overlay is live; `ties` no longer redraws 281 faces per frame.
- Narration vs model: Tower B has 9 capsule storeys; the slider demo is `SLIDER = capsule_storeys - 0.4` (8.6 gives 9, Floor gives 8); "eleventh" became "top storey" / "oberste Geschoss".

## Frames checked

Mid frames of every morph clause read in en (types to recap) and in de (types, text_number, storeys, deliberate, wrong_way, explicit, domain, geometry, int_bool, locale, order, fix); en locale and order re-read after the part-wise fix. Fixed after reading: invisible white chip text on the light canvas, stray Slot-origin chip, dark hull blobs for chip to chip morphs (now part-wise).

## Open items

- Mid frames of chip to chip morphs (locale, order) are still small dark-ish hulls for a moment; readable but plain.
- The hull follows Tower B in the axon but is shown offset to the left tower body in the mid frames of storeys and ties; positions come from the model (`at`), not tuned by eye.
- order: the entry chip stays at the ladder entry while a second chip walks down (intended as the input marker).
- Not run: `capture.py` layout keyframes and the series presentation contract tests (full lifecycle exceeds the time limit); only the morph captures (en, de) were rendered without errors.

## Audit round (T/morph-audit-grasshopper-casting.md)

All 22 keys captured en and de (63 files each, `index.md` complete, no errors); mid frames of every morph clause read in en, key clauses in de.

Fixed:
- mismatch: dashed tether (`_morph.tether`) from the Panel to the ghost capsule; removed at text_number.
- ties: the storey hull morphs into the gold dot of the number line (dot follows the slider, clamped to the line), no fade; the ramp starts after the morph.
- storeys / deliberate hulls: tighter hull on Tower B; ticks and tick label (26 pt) on the same tower.
- One persistent capsule: `_stage` now splits `.base` (wall and slot capsule, shown once at point_plane) and `.over` (overlays); wrong_way, line_plane and explicit only add or remove overlays; at domain the persistent capsule morphs into the domain capsule drawing.
- wrong_way: red capsule at 0.78 opacity after the plane square morph; N, W, 90 degree tags and the window arrow appear only after it.
- bool_int / int_bool: the chip travels on a curved path to the window ring (`travel`), then morphs into it (two queued phases via `then(..., *more)`).
- fail: ghost capsule morphs part-wise (`capsule_pairs`) into a solid red `null` chip; order: the ladder fades in after the old canvas is gone; fix: the same `Type A` entry chip of order morphs into the `2.5` chip (no new chip), then into the capsule stand-in.
- Titles and ghost remnants: `_gone` fade-outs now finish at 40 percent of the clause so old canvases and titles are gone before the new ones are readable (types, storeys, explicit, locale, int_bool frames are clean).
- geometry chips: solid gold tokens with dark text (and German `Kreis → Kurve`); matrix_fill passes box and text pairs.

Still open:
- recap: gather hulls of the 9 cells overlap translucently for a moment; matrix_fill flight pills are small and still lose their text mid-flight.
- locale and order chip-to-chip morphs pass through a tiny pale chip with a dash glyph mid-way.
- The matrix step still fades the capsule out (no capsule-to-matrix morph).
- `capture.py` layout keyframes and the presentation contract tests were not run.

## Re-capture with the automatic part matching toolkit

Re-captured all 22 keys en and de after the toolkit default changed (parts matched automatically); no errors, 63 files each. Read en mid frames of text_number, storeys, deliberate, geometry, locale, order, matrix_fill, recap, fix: recap gather is now clean (the overlapping hulls are gone); the chip-to-chip morphs in locale and order and the matrix_fill flight pills remain small, other beats look as before.

## Chips stay full size (last two items)

- order: the `null` chip travels as-is along an arc to the ladder entry, then cross-fades into `Type A` at the destination (no shrink, text readable throughout).
- matrix_fill: each example pill (size 1.0, text readable) travels as-is along an arc to its cell, then cross-fades into the cell content.
- locale: the `2,5` chip is replaced in place by a `!` chip with a cross-fade (same size); the mid frame shows both glyphs overlapping for a moment.
- Re-captured locale, order, matrix_fill en and de, frames read. Open: capture.py layout keyframes and presentation contract tests not run.

## Lessons pass (12 general lessons of `morph-review-excel-references.md`)

1. One actor: chips are the capsule's values in flight, no side-card duplicates; kept.
2. Persist the bound pair: the storey hull stays until `ties` morphs it, chips and stand-ins stay at their targets; new: the origin dot leaves a faint ghost on the facade (`ghost=0.3`) while the `Slot origin` chip persists on the canvas.
3. Move the number: locale now moves the `2,5` chip (it flies from the text Panel to the Number parameter) and only then a red `!` badge pops beside it; no glyph cross-fade or scribble any more.
4. Trackers are used only for continuous quantities (slider ramp in ties, spin in explicit); discrete steps are chip morphs.
5. Actual events: failure = the `null` chip and red `!` badge, rotation = the red capsule turning into the green one, line becoming axes; kept.
6. Structure before morph: every morph is queued after the window swap (`then`), on a clean stage.
7. Legible: chips are size 1.0 to 1.5 with dark text on filled pills, flights follow arcs above text; matrix_fill and order chips stay full size.
8. Mid frames were read at 800 px wide contact sheets; not re-captured at 1280x720.
9. Both directions: concrete to abstract (types, deliberate, fail, geometry, matrix_fill) and abstract to concrete (text_number, storeys, wrong_way, domain, bool_int, int_bool, fix, recap), chained through the capsule and the `Type A` chip.
10. Narration timing: morphs start after the first animations of the clause, not exactly on the key word; not tuned per word.
11. Back-links exist: Integer chip to lit storeys, number chips to the capsule, window rings, matrix cells to the capsule in recap.
12. Hygiene: `pairs=` for chip to capsule stand-in, `capsule_pairs` for the ghost, no anonymous path tokens.

Re-captured point_plane, line_plane, locale, fail en and de without errors, frames read. Still open: lesson 8 at 1280x720, lesson 10 word-level timing, capture.py layout keyframes and contract tests.

## Final round (T/morph-final-grasshopper-casting.md, top 5)

1. One persistent Type A chip: it starts at the `Type A` Panel in `fail`, travels to the Center Box, the ghost capsule morphs into the red `null` chip next to it, in `order` the same chip travels to the ladder entry and walks down the four steps (the ladder no longer draws its own chip, `_order(walker=False)`), and in `fix` the same chip morphs into the `2.5` chip and the capsule. No `FadeTransform` swap remains.
2. locale: the `2,5` chip starts in the text Panel, flies to the Number input, then `split` separates a red comma glyph from the remaining `2 5` chip and the red `!` badge pops beside it.
3. text_number and storeys: the tracker token swaps are gone. The quoted Text chip `"2.5"` travels from the Panel to the input and `retext` turns it into the Number chip `2.5` (then into the capsule). The slider chip `8.6` travels to the Storeys input, `retext` turns it into the Integer chip `9`, a `.6 dropped` chip falls away.
4. storeys: the Integer chip `split`s into nine per-storey hulls on Tower B (`_band(i, i)`); in `ties` the nine hulls `gather` into the number-line dot; deliberate keeps the top-storey hull that lifts off into the red tick.
5. recap: the matrix cells gather into the three habit cards (Panel card from the number cells, planes card from the plane, domain, curve and brep cells, labels card from the Text cell), the correct capsule fades in beside them. matrix_fill still builds its nine pills fresh (same labels and colours as the earlier chips); reusing the original chip objects across clauses was not done.

Captured text_number, storeys, ties, deliberate, locale, fail, order, fix, recap en and de (fail, order, fix re-run after a syntax slip); frames read in en: text_number, storeys, ties, locale, fail, order, fix, recap. Not read again: the German frames of this round.
Open: matrix_fill chip identity (item 5 first half), the storey hull sits slightly left of the tower body, capture.py layout keyframes and contract tests not run.
