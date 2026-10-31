# Layout Fixes: Fitted Towers, Legend Markers, Box Chips

Ticket `GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS`. Source frames and findings: `🗑️generated/frames/🦗️grasshopper/Beat1_Anatomy/en/{hook,legend_data,box}.png`, `layout.md`.

## Root cause

After the Nakagin remodel the tower is taller (A: 42.14 m core plus 11.36 m capital, B: 36.62 m plus 10.83 m, podium footprint 30 m wide). Every `tower_axon` call carried a hand-tuned `scale=` and `at=`; the new model no longer fit those boxes (capitals over the "1 of 140 capsules" note, podium over the caption, shaft off the top and bottom of the viewport).

## 1. Fit helper (no drawing, no `🏢️nakagin` edit)

New shared module `🧲️fit/🐍️.py` (pure NumPy plus the Nakagin model, no Manim):

- `building_points(towers, podium, capital, core, capsules)`: world vertices of the model envelopes `tower_axon` draws (`core_solid`, `capital_solid`, `podium_solid`, `slot_solid` of every slot). About 0.1 s, no drawing.
- `building_fit(width, height, centre, view, top, bottom, margin, **parts)` returns `{"at", "scale"}`. The projected box (any axon name, `(azimuth, elevation)` or `north|south|east|west`, through `make_view`) is scaled to the free area (area minus a `top` label row, a `bottom` row and `margin`) and centred in it. `tower_axon` centres its drawing on the middle of the model box, so the returned `at` puts the projected box in the middle of the free area.
- Use: `self.tower_axon(**building_fit(w, h, centre=..., top=...), lit=...)`.

Wiring:

- GH canvas toolkit `🦗️grasshopper/🎬️clip/🎨️visuals/🦗️canvas/🐍️.py`: re-exports `building_fit`, adds `LABEL_ROW = 0.45` (height of the viewport title row) and `Canvas.gh_building_fit(viewport, ...)` that fits below a `gh_viewport`'s label row from its body size and centre.
- Excel toolkit `📊️excel/🎬️clip/🎨️visuals/🐍️.py`: re-exports `building_fit` (Excel has no canvas viewport; it fits explicit panel rectangles).
- Both `source_files()` lists (render provenance) now include `🧲️fit/🐍️.py`.

## 2. Scenes touched

| Scene | Before | After |
| --- | --- | --- |
| GH anatomy `hook` | `scale=0.065`, `at=VIEW+(0,-0.05)` | `self.gh_building_fit(viewport, towers=(0,1), podium=True, capital=True)` (scale 0.053) |
| GH anatomy viewport tower (legend_data, item/list/tree beats) and `_spot`/`_cap`/`_cap_to`/`_to_cap` | scales 0.092, 0.075 with `dy=-0.15` per state | module `_fit(towers, podium)` (one fit per tower, capitals off, below the label row); the `scale`/`dy` state and parameters are gone, so the live tower, the static copies and every morph target use the same numbers |
| GH capsule `Beat2_Capsule` | `scale=.105` at `VP+(0,.45)` | `building_fit(VP_W, VP_H, centre=VP, top=LABEL_ROW, bottom=1.0)` (counts panel row reserved) |
| GH casting `Beat4_Casting` | built the whole tower once at scale 0.1 to measure it (a second full draw) | `building_fit(5.4, 3.4, ...)` from model numbers: no trial drawing |
| GH trees `Beat5_DataTrees` (placement lines only) | `TOWER_AT`/`TOWER_SCALE` 0.125; mini towers `scale=0.05` | `TOWER_FIT = building_fit(5.4, 5.2, centre=(4.3, 0.62))`; mini towers fit 1.8 x 2.5 under their "expected/actual" labels. `_detail` (magnified slice, caller-chosen scale and span) left alone |
| GH tower `Beat3_Tower` | already fits every drawing with its own `fit(drawing)`; one redundant literal `scale=.1` removed | no other change (the pinned `_tower(scale=S_T)` is rescaled by `fit`) |
| Excel anatomy | `scale=0.105`/`0.085` at fixed points; dead `_tower` helper | `TOWER_FIT`, `SIDE_FIT`; dead helper removed |
| Excel program | `TOWER_AT`, `TOWER_SCALE=0.09` | `TOWER_FIT = building_fit(5.4, 4.4, centre=TOWER_AT)` (between the 2.4 and -2.42 label rows) |
| Excel references | `scale=0.1` | `TOWER_FIT = building_fit(5.4, 4.6, ...)` |
| Excel energy (4 calls) | 0.1, 0.08, 0.045, 0.085 | four fitted areas (centre stage, right panel under the counts, small counter tower, legend-scene tower) |

The trees file was re-read before each edit; only the three tower placement lines and the `TOWER_*` constant changed.

## 3. Legend markers (anatomy)

- Constants `TOOL_ORDER`/`DATA_ORDER` replace the inline `range(1, 9)` and `(1, 2, 4, 3, 5, 6)`; a test asserts each is a permutation of `1..len(terms)`, so every marker maps to exactly one filled row (tool: 8 terms, 8 markers; data: 6 terms, 6 markers).
- Overlaps removed in `legend_data`: marker 1 moved diagonally above the first item row (it sat on top of the bracket marker 2); marker 6 moved diagonally above its row (it touched marker 3 on the left).
- `legend_tool`: marker 1 sat on the Center Box input grip. The mini Center Box moved 0.25 right and the tie marker 1 sits 0.3 right of the file chip, clear of the component.
- Stale discs: the frame showed eight discs beside six rows. A probe with the real `_table`/`_fly_discs` (`layout-fixes/probe_legend_discs.py`) shows `FadeOut(table)` removes the tool legend's discs in the current code, so the extra discs were not reproducible from the present source; the eight-disc legend was the previous tool legend and is now covered by the order constants. Final frames from `final-cap.sh` should confirm.

## 4. Box chips

`box` clause: the three value chips (2.5, 4.0, 2.5) landed on the three input grips (0.33 apart, chips 0.33 tall) and hid X/Y/Z. They now land on the middle of each wire (`wire_paths`, the same Bezier the morphing dimension lines become), at size 0.8 (`_hop(..., size=)` added). The wires are 52 px apart at the middle, so the chips neither touch each other nor the input labels, and the sliders' output grips stay visible.

## Verification

No capture was started: `final-cap.sh en` and `de` were both running, so a second capture of either language was skipped.

- `🧲️fit/🧪️tests/🐍️.py` (new, 5 tests, `🔣️.cases.json`): for six areas (viewport whole, tower A, tower B, tall viewport with a counts row, stage panel, mini tower) the fitted drawing lies inside the free area, touches it on the limiting axis, and is centred; projection and bounding boxes are computed with rhino3dm (`Plane`, `PlaneToPlane`, `PointCloud`), and core, capital (roof corners), podium footprint and every capsule centre read straight from `🏢️nakagin/🔣️.json` stay inside. Also: fewer parts never shrink the drawing; elevations fit; the `axon` case matches the drawing's `VIEWS`. Result: OK.
- `🦗️grasshopper/🎬️clip/🧪️tests/📐️layout/🐍️.py` (new, 2 tests): AST scan of the nine GH and Excel scene files: no `tower_axon`/`tower_elevation` call carries a literal `scale`; both legend orders are permutations of their term counts. Result: OK.
- GH registry tests OK (5); Excel clip tests OK (85). GH engine tests: 66 run, 3 failures unrelated to this work (`test_wrap_retains_each_actual_family...` 8 != 4, two casting cases expecting "Number" where the engine says "Integer"; engine/model files are being changed by others).
- All nine scene modules import (the module-level `TOWER_FIT` constants evaluate).
- Native lifecycle run (real animation setup, Cairo frame, finite geometry, `layout-fixes/lifecycle.py en Beat1_Anatomy Beat2_Capsule Beat4_Casting Beat5_DataTrees`) was started in the background (PID 16712, log `🗑️generated/layout-fixes/lifecycle-en.log`); it was still inside `Beat5_DataTrees` after 15 minutes because each new fitted scale rebuilds the tower drawing (minutes per build), so it is not a result. Not yet verified at frame level: run `capture.py ... Beat1_Anatomy <lang> hook legend_data legend_tool box` once `final-cap.sh` has finished.

Temporary files: `layout-fixes/` (probes, `lifecycle.py`); logs in `🗑️generated/layout-fixes/`.
