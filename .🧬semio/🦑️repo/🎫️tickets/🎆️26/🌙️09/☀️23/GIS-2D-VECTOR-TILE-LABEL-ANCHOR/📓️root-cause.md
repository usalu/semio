# GIS 2D vector tile label anchor

## Symptom

On the GIS 2D map (combined/vector modes), short alphanumeric labels such as `A81`, `B29` appear at wrong positions — typically clustered near tile centers instead of along the roads they name.

## Cause

`append_vector_tile_labels` placed labels using only `feature.points` or the first vertex of a polygon ring. OpenFreeMap / OpenMapTiles `transportation_name` (and many `water_name`) features are **LineString** geometries decoded into `VectorFeature.lines`, with an empty `points` vector.

When both were empty, the code fell back to `(extent/2, extent/2)` — the tile center — for every such label.

## Fix

`vector_tiles::feature_label_anchor` picks the first point, else the midpoint of the longest line, else the first ring vertex, else the tile center fallback.

## Verification

`cargo test -p semio-framework-surface vector_label_anchor_uses_line_midpoint_for_transportation_names`
