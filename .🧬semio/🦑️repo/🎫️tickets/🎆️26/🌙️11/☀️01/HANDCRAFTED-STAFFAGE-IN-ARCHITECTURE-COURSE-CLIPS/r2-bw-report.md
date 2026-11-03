# R2 bw: rename fallout check and suite re-run after the public model names

## 1. Grep for old private names

Searched `🎓️teaching/🏛️architecture/🏗️bim` for `_surface_finishes`, `_geometry_producers`, `_source` (as a name) and `_model._` / `__file__` uses of the model module.

- One stale use found and fixed: `🎬️clip/🧱️model/🧪️tests/⚡️slabs.py:15` called `api._source.cache_clear()`. Changed to `api.source_text.cache_clear()` (the public cached reader in `🎬️clip/🧱️model/🐍️.py:57`).
- No other old names remain. `surface_finishes`, `geometry_producers`, `source_text` and `source_path` are all defined in `🧱️model/🐍️.py` (lines 51, 57, 1208, 2778) and every caller uses the public names.
- Other `_`-prefixed names still used across modules (`_polyhedron_part`, `_solid_geometry`, `_find`, `_slab_geometry_uncached`, `_SLAB_GEOMETRIES`, `_material_bounds`, `_native_material_cells`) all exist in `🧱️model/🐍️.py`. Not part of this rename, so left as is.
- `G._source_revision` and `source._source_digest` in `🎨️visuals/🏛️savoye/🧱️core/📐️geometry.py` exist. No broken imports found by reading the references.

## 2. Suites

Runner: `T/🧰️bw/run.sh <name> <module>` (uv, same env as `T/m9_tests.sh`, stdin from `/dev/null`, `timeout 7200`) calling `T/🧰️bw/fnrun.py`, which runs the module's `test_*` functions. These two modules hold plain functions, not `unittest` classes, so `unittest` cannot collect them.

| suite | module | result |
| --- | --- | --- |
| BimStepExchangeContract | `🎬️clip/🧪️tests/🐍️.py:1636` | NOT RUN. `ModelCase.setUpClass` calls `building_geometry` (about 4 h base build), so skipped per the brief. |
| finishes tests | `🎬️clip/🧱️model/🎨️finishes/🧪️tests/🐍️.py` (16 functions) | INCOMPLETE. 6 ok, 0 failures, then the process stopped at the 2 h timeout (killed 07:34) inside function 7 (alphabetically after `test_nearly_coincident_...`). The run log has no `Ran` line. Function 7 and functions 8 to 16 are unverified. |
| savoye snapshot tests | `🎬️visuals/🏛️savoye/🧱️core/🧪️snapshot/🐍️.py` (3 functions) | 3 ok, 0 failures, exit 0. |

Note: the run for the finishes module had no output for about 1h45m after function 6, so function 7 is slow or hung. It was not diagnosed further.

## Files changed

- `🎬️clip/🧱️model/🧪️tests/⚡️slabs.py` line 15: `api._source.cache_clear()` to `api.source_text.cache_clear()`.

## Files added

- `T/🧰️bw/run.sh`, `T/🧰️bw/fnrun.py` (runner scripts, kept).
- `T/🗑️generated/bw` removed.

## Still open

- Run finishes tests again, with a longer timeout or a look at function 7, to finish the 16-function count.
- BimStepExchangeContract needs the full geometry build (about 4 h); not run.
