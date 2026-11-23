# Detailed Outer-Leaf Windows Across Heating

## Problem
Heating house windows read as thin mid-wall teal bars without joinery, unlike the ventilation casements.

## Shared (authoring visuals)
1. window_glyph pane uses most of the outer leaf (0.85 of wall).
2. emphasize_casement strengthens sash and joinery strokes for overview scales.
3. house_section uses richer depth plus emphasized casements; house_wall 0.14 to 0.18.
4. room_section glazing is the shared outer-leaf window_glyph (not a centred stick).

## Heating clips
- conduction envelope house: emphasized Fenster/Haustuer with joinery in the highlight group.
- calculation window_wall and plant_house: wall plus inward plus emphasize.
- solar-gains facade and shading section glazing: outer-leaf casements.

## Tests
Language-agnostic house_window_outer case and room both-side outer asserts (Manim geometry oracle).
