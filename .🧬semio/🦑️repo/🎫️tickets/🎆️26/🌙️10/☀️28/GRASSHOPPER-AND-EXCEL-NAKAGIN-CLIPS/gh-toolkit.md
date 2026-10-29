# GH-TOOLKIT Report

Status: ready (api.md section 4), reconciled with the real engine (api.md section 3). `NakaginDrawing` is imported normally.

## Files

- `G/🎨️visuals/🐍️.py`: `ClipScene` (BIM interface: `language`, `tr`, `number`, `step`, `present`, `source_files`) over BIM `Drafting`, `NakaginDrawing`, `Canvas` (BIM `Panels` removed by the coordinator; nothing here used it).
- `G/🎨️visuals/🦗️canvas/🐍️.py`: `Canvas` mixin and `Widget` (parts, grip anchors that follow transforms), helpers `path_text`, `branches`, `bezier`, `offset_curve`, `clip_segment`.
- `G/🧪️tests/🎭️presentation/🐍️.py` + `🔣️.cases.json`: 12 tests (scene discovery by folder, bilingual narration, step keys, imports, lifecycle for the probe and all five existing `Beat*` scenes in en and de, widget zoo, every engine view builds, states and panels from the solution).
- `T/gh-toolkit/showcase.py` (canvas/widgets stills, canvas page now uses `canvas("anatomy")` view `box`), `T/gh-toolkit/views.py` (every view, strict layout).

## Reconciliation with the engine

- `gh_canvas(definition, solution, view=..., frame=...)` reads `canvas(key)`/`evaluate()` directly: widgets by catalogue `kind`, sliders show the solved value (integer rounding gives 0 decimals), panels show the real tree through `panel_rows` (path headers and indices only when `path_count > 1`, German decimal comma, cut at `max_lines`), Param Viewer shows the incoming tree (5 branches then `+n`), floating params show label and type, component states and bubbles come from `solution.state/msgs` (worst level, `tr(en, de)`).
- Wire kind from the data: `path_count > 1` tree, `count > 1` list, else item. Grips are found by parameter name.
- Positions: engine columns by x, order by y (135 engine units = 1 scene unit), objects in a column pushed down where they would touch, bubbles shifted along their target to stay clear of neighbours. `frame` draws the grey sheet and scales down to fit (never up); if the factor would be below 0.8 the panels are shortened and the layout retried.

## Verification

- All 51 views rendered as stills in `T/🗑️generated/gh-toolkit/views/` (English, plus German for views with bubbles); every widget inside the frame (0 outside); scale factor 1.0 for most, 0.80 to 0.99 for the largest (`tower/core`, `tower/capital`, `tower/clearance`, `tower/podium`, `capsule/window`), `report-en.md` and `report-de.md` list the strict layout results.
- Strict layout: 0 findings in about 45 views; remaining findings are wires drawn behind a component body (Division, Plane Origin, List Item) which the checker reports as strokes through a chip (the body is opaque, so nothing is visible through the text) and one orientation of slider domain labels that was fixed by widening the track.
- Showcase stills: 0 strict findings, en and de.

## Notes for scene authors

- Keep `scale >= 1` where possible; for the biggest views use `only=` or the engine `view` names, which are already at most 12 objects.
- Do not define methods named `_gh_*` on scenes.
- Full presentation test run is slow (about 6 minutes cold); `SEMIO_TEACHING_CACHE_DIR` speeds up font work.
