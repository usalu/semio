# Duct Air Fill

## Problem
Transparent blue stopped short of the end disc, and filled particles / mismatched opacities stacked as overlapping colours.

## Fix
1. Barrel rectangle ends at `disc_x - r`; disc face is a filled circle at the **same** opacity — they abut, no alpha stack.
2. Teal rim is stroke-only on top.
3. `flow_animation(..., peak_opacity=0, stroke_width=1.6)` for stroke-only motion cues inside the fill.

## Check
Preview ~50s: fill colour continuous through the circular face (sampled above/below the yellow `r` line).
