# Toolkit: ClipScene, Drafting And Panels For The BIM Clips

Code: `C/🎨️visuals/🐍️.py` (`ClipScene`), `C/🎨️visuals/📐️drafting/🐍️.py`, `C/🎨️visuals/🧾️panels/🐍️.py`; Villa drawings, axonometric and analysis views live in their own modules (`🏛️savoye`, `🏛️savoye/🧊️axon`, `🧮️analysis`) owned by the ORTHO, AXON and ANALYSIS-VIEW agents; each documents itself in `ortho.md`, `axon.md`, `analysis-views.md`. Contract and every signature: `T/api.md` section 3. Verified with `python T/toolkit-showcase.py <en|de> [board ...]`: stills `T/🗑️generated/toolkit/<board>-<language>.png`, layout report `toolkit-<language>-layout.md` (energy `layout_conflicts`).

## Principles

- One projection (`View`) and one return type (`Drawing` with `parts`, `layers`, `anchors`, `at`) for every drawing; the model is the only source of numbers (`_model.fn(...)` at call time).
- Mixins: each widget module exposes one class; `ClipScene` inherits them all; a module that does not exist yet contributes nothing.
- Energy look: colours from the energy palette (`INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE GREEN_GROUND`), type ladder 16.2 / 19.2 / 21.6, bundled Anta for text and bundled Share Tech Mono for STEP text (all printable ASCII verified present in the font, so no silent fallback shifts the columns).
- Both languages are first-class: every label through `self.tr`, numbers through `self.number` / `self.quantity` (en `1,250.5`, de `1.250,5`, unit after a narrow no-break space, true minus).
- Moving values: `self.live(builder, key=...)` rebuilds only when `key()` changes; `self.live_label(fn, point)` re-typesets a label only when its string changes. Copies made by `FadeIn`, `Transform` or `Indicate` are frozen snapshots.

## Widgets in this toolkit module set

### Drafting (`📐️drafting`)

`dimension(start, end, text, *, offset, color, flip)`, `level_mark(point, text, *, side)`, `leader(source, target, text)`, `hatch(rings, color, spacing, angle)` (clipped to polygons with holes), `badge`, `north_arrow`, `scale_bar`, `label`, `text`, `mono`, `segments`, `polyline`, `polygon`, `region`, `dashes`, `View`, `Drawing`, `Live`. Anchors: `l r u d` combinations place an edge on the point, `b` puts the baseline on the point (rows of a table share one baseline whatever their descenders).

Screenshot: `drafting-en.png` / `drafting-de.png` (hatched bay with two dimension lines, level marks, leader, north arrow, scale bar, badges, label, mono text), `live-dimension-en.png` (a `Live` rectangle with dimension at 2.0 m above and 3.4 m below, same builder).

### Panels (`🧾️panels`)

- `class_box`, `class_tree` with leaf classes `IfcColumn IfcSlab IfcWall IfcWindow` hanging off `IfcBuiltElement` (`class-tree-leaves-*.png`) and the expanded attribute box of one leaf with inherited attributes grey (`class-tree-*.png`).
- `property_table` with the basis column `sourced` / `assumed` (`property-table-*.png`).
- `step_panel` with the real `step_excerpt("B3")`: role colours, `only=`, `wrap=`, `token()`, width 12.4 and a narrow wrapped panel with `highlight_roles` (`step-panel-*.png`).
- `model_card` with the discipline axon as thumbnail (`model-cards-*.png`), `guid_chip`.

## Verification notes

- Every board is rendered in en and de at 1280 x 720 and checked with the energy layout guard; remaining reports are listed in `toolkit-<language>-layout.md`.
- Mono glyph coverage checked with the repository font code (`_font_supports("Share Tech Mono", c)` for every printable ASCII character: none missing). Sans lacks only `Omega`, arrows and comparison signs, which the font wrapper substitutes from the math face.
- Window / wrap logic of `step_panel` keeps one `Text` per display row, so letters, not runs, define the layout and the layout guard sees no spurious overlaps.
