# r2-fb: figure bounds for transport casts

## Findings
- `_Motion` already used `figureBoundsM` (bp) for `get_top/bottom/x/centre`, but `.height` / `.width` (Manim `length_over_dim`) still measured every path, so `person.height` was 2.244 against a 1.638 figure. The static `staffage()` group, and with it the probe of `fitted_staffage`, ignored the figure bounds entirely.
- `staffage_height` returns the exporter `heightM`, which is already the asset-bounds height (the figure); documented now.
- Manifest transport casts are only the five breathing casts and the kettle. Laptops carry no transport. Every course call site (need `_stand`, `_room_occupants`; volume `_stand`, `_use_cast`; natural `_stand`; mechanical `_stand` and line 775) sizes by `staffage_height` and anchors the feet at the floor, so nothing used raw bounds. The kettle icon uses a window (fits the window). No scene call site needed a change, so no scene render changed; none was rendered.

## Fix (`V/🧍️staffage/🐍️.py`)
- New `_FigureBounded(VGroup)`: when `_figure_only`, `get_points_defining_boundary` and `length_over_dim` use the probe corners only. `_Motion` derives from it (its own override removed).
- `staffage()` returns `_Figure` (probes at `figureBoundsM`) for an unwindowed transport cast; `fitted_staffage` therefore fits, aligns and floors by the figure for both the still and the `rate` path. Non-transport casts unchanged.
- Test: `V/🧪️tests/🐍️scenes.py::test_transport_casts_are_sized_and_floored_by_their_figure` with JSON case `transport_figure` in `🔣️scenes.json` (five breathing casts: height and width follow the figure, bottom on the floor, plume reaches above the top).

## Results
- Staffage module suite + `staffage_only`: 64 tests OK (one cancellation test failed once under load with an orphaned PID and passed on re-run). `Fit` and the new scenes test OK.
- Earlier full run (scenes, staffage_only, integration, ventilation architecture): 48 tests, 2 errors, both in `integration.py` and not mine: `summer-strategy` has no `_person` and no `Beat1_LastZuerstSenken` (another agent's cooling edit in progress).

## Open
- Frame check of rendered scenes skipped: no scene geometry changed.
