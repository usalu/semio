# Realistic Window Section Without Mid Muntin

## Problem
Windows in building section showed a horizontal bar through the middle of the pane — an elevation muntin that does not belong in a wall cut.

## Fix (`🏗️authoring/🎨️visuals`)
1. `window_glyph`: replace cross muntins with double glazing (two vertical glass cuts) plus sill/head rail joinery.
2. `window_opening`: project the same two glass lines with the sash.
3. `emphasize_casement`: also strengthen the glazing strokes.
4. Cases: `min_joinery` → 6.

Every beat using `house_section` / `room_section` / `window_glyph` picks this up for the whole ventilation video.
