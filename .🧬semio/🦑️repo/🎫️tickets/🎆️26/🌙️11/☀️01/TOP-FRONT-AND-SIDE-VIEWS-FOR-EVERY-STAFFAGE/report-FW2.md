# Report FW2: Section Views, Dimension Clearance, Caption Fit

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder. Details of the framework change are in `T/fw-status.md` ("Section views", "Layout fixes").

## Section views
- Schema: `representations[scale].sections = { front?, plan? }`, each a `ViewSection` `{ plane, label: {en, de} }` (JSON `$defs/ViewSection`, TS twin `ViewSection`; `Section` was already the member cross-section). Front plane = forward x of a vertical cut seen from the front; plan plane = height y of a horizontal cut seen from above.
- Validator `section-plane-outside` (en/de). Captions "Section A-A" / "Schnitt A-A" in `--views` sheets (wrapped to the cell), scene title lines and the React viewer. Cut-line marker (thin chain line, thick ends, arrows, letters) in the elevation at 1:100 and 1:50 only. The cast index records `section`.
- Content (all authored scales): bookshelf front A-A x 0.4; three-seat-sofa front A-A x 1.0; curtain-wall-bay front A-A x 0.7 and plan B-B y 2.075; masonry-wall-with-lintel front A-A x 1.25 and plan B-B y 1.5; sliding-door plan A-A y 1.0.

## Layout fixes
- Dimension line: extension lines start 1.5 mm outside the drawn geometry of the viewpoint and the dimension line stands 6 mm beyond it.
- Text width: Helvetica and Helvetica-Bold advance widths (`textWidthMm`, 4 % slack) instead of 0.56 em per character.

## Tests (direct vitest, 23 of 23 pass; tsc clean except the known d3-format)
- `S/🧪️tests/✂️section-views` (shapely oracle for the marker geometry, hand-computed fixture, validator, catalog, cast index).
- `S/🧪️tests/📐️dimension-clearance` (shapely distance to the geometry box; bollard 1:500, street-lamp 1:200 and every authored scale of both, three viewpoints).
- `S/🧪️tests/🔤️caption-fit` (PyMuPDF per-character Helvetica widths; scene text inside the scene; caption wrapping). Whole-string `fitz.get_text_length` drops accented letters, so the oracle sums per character.
- Run: `T/🧰️FW2/vitest.section.config.ts` from `S/📦️packages/🟦️typescript`. Each python oracle call takes about a minute on the loaded host, so the files set a 900 s test timeout.

## Paper check (one bun process, `T/🧰️FW2/sheets.ts`, output in `T/🗑️generated/FW2/sheets`)
Looked at the PNG of each sheet:
- retractable-bollard 1:500: dimension line and ticks stand clear of the glyph in all three views; captions read Side view, Front view, Top view; scale bar label inside.
- street-lamp 1:200: dimension clear in all views, including the plan; captions and scale bar fit.
- gas-boiler 1:200: title and subtitle sit inside the cells, scale bar complete.
- bookshelf 1:50: side view shows the A-A marker at mid width with arrows pointing backwards and the letter A at both ends; captions "Section A-A" (front) and "Top view" (plan).
- curtain-wall-bay 1:50: side view shows A-A (vertical, x 0.7) and B-B (horizontal, mid vision glass) markers; captions "Section A-A" and "Section B-B".
- masonry-wall-with-lintel 1:50: A-A through the middle of the opening and B-B at half opening height; captions correct.
Remaining small point: on the plan of tiny glyphs (bollard 1:500) the extension lines and the 0.42 m tick are close together because the span is under 3 mm; they stay clear of the glyph.

## gas-hob 1:200 "has no suitable representation"
`resolveRepresentation(gas-hob, 1:200, office-laser)` returns `kind: "unsuitable"`:
- violation `feature` on shape `burner`: measured 0.154 mm, required 0.2 mm (min feature of the `office-laser` profile); min stroke 0.13 mm is fine, min gap null.
- It is the print-suitability measure, not an authoring-completeness or unavailable decision. Alternatives offered: 1:100 and 1:50. The measure now runs over all poses and all three viewpoints, so the `burner` shape is probably the flame in a burning pose or the plan/front burner ring at 1:200; the earlier measure only saw the elevation. FW3's change to measure visible geometry may or may not change this: if the 0.154 mm burner is hidden or merged in the visible drawing it will pass, otherwise the gas-hob 1:200 burner shapes must be widened to 0.2 mm. I did not edit the asset.
- Consequence for my tests: the caption-fit scene test skips scale/asset pairs that raise `RepresentationError` (and asserts at least 12 scenes are drawn).
