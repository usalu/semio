# Fit Per-Person Rate Chip Boxes

Scene: `Beat1_Lueftungsbedarf` personen beat.

## Problem
Fixed `1.05 × 0.42` rectangles clipped the typeset `30 m³/h` fraction.

## Fix
Box width/height = label size + `1.6 · UI_SPACING` on each side; `4×` and total readout clear the wider chips.

Padding set to `2.4 · UI_SPACING` so the stacked m³/h fraction clears the frame.
