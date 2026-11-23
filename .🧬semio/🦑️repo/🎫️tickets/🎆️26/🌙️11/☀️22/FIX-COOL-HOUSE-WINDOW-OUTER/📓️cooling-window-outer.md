# Cooling Outer-Leaf Windows

## Problem

Kühlung house scenes still read like mid-wall cyan sticks (Beat1 winter solar gains /
Cooling_01 poster). Lüftung and Heizen already use shared `window_glyph` casements flush
with the exterior leaf.

## Cause

1. Shared `house_section` / `room_section` already place outer-leaf casements.
2. Kühlung overlays still aim at the wall centreline (`w["x"]` / `room["glass_x"]`):
   sun rays, AHU paths, night-purge tracks, blinds anchors, and a white `Rectangle`
   frame centred on the glass centreline — recreating the mid-wall stick look.

## Fix

Aim every Kühlung window hit / overlay at `window["outer"]` and keep the shared
emphasized casement. Cover heating-comparison, solar-radiation, summer-strategy,
transmission-humidity.

## Tests

Language-agnostic geometry oracle for Kühlung houses; source asserts forbid centreline
ray/frame aiming where outer is required.
