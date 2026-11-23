# Ht Hv Scene Windows Outer Leaf

## Problem
In "Vom Wärmestrom zur Jahresenergie", Transmission house and Lüftung wall casements read as thin mid-wall cyan lines — house scale 0.4 made the wall only ~0.07 thick; Lüftung wall was 0.2 with a weak sash.

## Fix
1. `_window_wall` default wall 0.32, outer-leaf `window_glyph` + `emphasize_casement`.
2. `_house` uses `emphasize_casement` (not bare `set_color`).
3. Scene2 house scale 0.4 → 0.7; overview house 0.55.

## Tests
Source requires wall 0.32, scale 0.7, and house emphasize.
