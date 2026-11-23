# Qv Window Outer Leaf Align

## Problem
Qv / Fensterlüftung panels and related houses still showed mid-wall glass: mechanical built a second `window_glyph` instead of the room casement; plant house used `wall=max(wall, 0.28)` so sash outer ≠ masonry outer; natural night window used `wall=0.22` on a `0.16` room wall.

## Fix
1. Mechanical: reuse `room["windows"][0]` with `emphasize_casement`.
2. Calculation plant house + `_window_wall`: true `wall=` thickness, cyan emphasize.
3. Natural / convection / need: outer-leaf wall match + emphasize.

## Tests
Mechanical forbids duplicate `window_glyph(room1["glass_x"]`; plant forbids `wall=max(wall, 0.28)`.
