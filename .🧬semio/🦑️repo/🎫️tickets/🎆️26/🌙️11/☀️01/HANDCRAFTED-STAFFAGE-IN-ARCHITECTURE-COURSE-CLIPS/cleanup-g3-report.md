# Cleanup G3 report

## 1. Orphans removed
- Grep over `🎓️teaching` (no node_modules, no venv) confirmed `compare="bulb|monitor|vacuum"` is never used; only `watt_anchor` calls with laptop, person, seated, toaster, heater, sun exist.
- Removed `monitor`, `bulb`, `vacuum` from `_WATT_DEVICES` and `_WATT_CASTS` in V/🐍️.py.
- Removed rows `shared-watt-monitor|bulb|vacuum` from V/🧍️staffage/🔣️.json.
- V/🧪️tests/🐍️icons.py:70 compared against `_WATT_CASTS["bulb"]`; it now compares against `["laptop"]`. No cases json referenced them.

## 2. Docstring
`_filled_outline` now reads "Draw one connected opaque filled shape from a diagram icon outline." (emoji kept).

## 3. Optimization thermal rule
- Manifest rows switched from `dark-line` to `thermal`: `optimization-room-occupant`, `-room-radiator`, `-extrema-radiator` (environment `indoor-heated-winter`); `-room-cooler`, `-extrema-cooler` (environment `summer-heatwave-calm`). Poses unchanged, so no timeline. The occupant row is shared with the experiments clip, which therefore also renders thermal.
- Dropped the now-ignored `color=` arguments at the call sites (room occupant/radiator, `_cooler` lost its `color` parameter and both callers, extrema icons, experiments occupant).
- Looked at renders (captures, quality l): Beat1_LocalGlobalExtrema `response` (radiator red, cooler light blue, lamp yellow, readable beside the formula rows) and Beat4_NormalizedIndex `criteria` (orange thermal occupant in the room section). Both read fine; no layout change needed.

## 4. Tests
- Visuals `icons`, `staffage_only`, `scenes`: 24 tests OK.
- Optimization clip tests: 70 ran, 1 known failure (`$schema` draft-07 assertion), 1 error (`test_all_clauses_fit_...` needs `SEMIO_TEACHING_OUTPUT_ROOT`; with it set the caption-only test did not finish inside my time limit, it does not touch the changed code).
- Test runner: `run-g3-tests.py` in this folder (loads test files by path with PYTHONPATH=repo root).
