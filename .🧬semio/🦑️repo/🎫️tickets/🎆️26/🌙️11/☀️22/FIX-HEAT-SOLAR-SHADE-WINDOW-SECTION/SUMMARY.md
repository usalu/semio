# Fix Solar Shade Window Section Realism

## Problem
`Beat5_Verschattung` window section looked wrong: a deep cyan sash across almost the full facade thickness (horizontal rails reading as a mid-window bar), and `facade_t = 0.5` vs slab `0.16` made the wall look much heavier than floor/ceiling/back wall.

## Fix
1. `window_glyph`: cap outer-leaf sash depth at `0.16` (`min(wall * 0.85, 0.16)`) so thick walls keep a shallow casement on the exterior face.
2. Beat5 room: `facade_t = 0.24`, slab/back `t = 0.20`, ceiling slab sits above the room (`ceiling_y + t/2`), overhang height matches slab.

## Tests
Solar-gains `shadeFacade` / `shadeWindow` + visuals `WindowOuterLeafPaneTest`.

## Preview
`videos/preview/de/speaker/s/58/v/Beat5_Verschattung.mp4`
