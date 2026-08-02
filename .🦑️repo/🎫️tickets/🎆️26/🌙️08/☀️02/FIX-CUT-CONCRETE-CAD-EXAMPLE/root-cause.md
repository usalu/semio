# Root Cause

CAD opened on `default_document()`: a single `object-box-1` on the shape pane and empty
`building_objects` / `energy_objects` / `structure_classic_objects`. Selecting the cut-concrete
example could also collapse back to that placeholder when `forest_play_document` treated an empty
shape import as fatal.

# Fix

- `initial_projection` → `forest_play_scene()`
- Remove empty-shape → `default_document()` fallback
- Fixture: `models[0].id = "spatial.shape"`, typology `spatial.shape.kernel.solid`
- TS tests: fix `.🔣️model.json` → `.model.json`
- Cache `forest_play_scene` via `OnceLock`
