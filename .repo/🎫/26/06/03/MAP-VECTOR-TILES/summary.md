# Map Vector Tile Labels

Fixed map labels not rendering and extended label support across all vector styles.

## Root causes fixed

1. **SVG text never painted** — `render_group` in `infinite/cavas/vello/lib.rs` dropped `usvg::Node::Text`. Now renders `text.flattened()` with stroke-before-fill for halo legibility.
2. **Wrong font family** — bundled `MapLabelSans.ttf` registers as family `Anta` in fontdb. `append_label` now resolves the family from the loaded font.
3. **Unclassified centroids** — demotile/OpenMapTiles country centroids use empty `class`; `place_label_visible` now treats `""` as country-band labels.

## Features

- Extracted `append_vector_tile_labels` in `gis/map/rs/lib.rs`; called after colored/figure-ground/inverted geometry passes, gated only by `vis.labels`.
- Street labels (`transportation_name`) no longer require the roads geometry toggle.
- Per-style default for Labels window toggle: Colored on, Figure-Ground and Inverted off (`MAP_VECTOR_STYLE_DEFAULT_LABELS` in play controller).

## Tests

- `infinite_cavas`: `append_label_renders_glyphs`
- `gis_map`: place hierarchy, colored/figure-ground label scene growth, camera/tile fixture setup
- `gis/map/play`: per-vector-style default labels visibility

## Files

- `infinite/cavas/vello/lib.rs`
- `gis/map/rs/lib.rs`
- `gis/map/react/index.tsx`
- `gis/map/play/index.ts`
