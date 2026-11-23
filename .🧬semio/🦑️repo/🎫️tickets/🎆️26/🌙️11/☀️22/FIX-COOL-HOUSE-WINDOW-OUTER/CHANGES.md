# Detailed Outer-Leaf Windows In Cooling

## Problem
Kühlung house windows read as mid-wall cyan sticks; Lüftung/Heizen use detailed outer-leaf casements.

## Shared
- `window_glyph` pane uses 85% of the wall (outer leaf), matching the documented intent.

## Kühlung clips
- heating-comparison: solar rays and AHU paths hit `window["outer"]`.
- solar-radiation: house/room rays hit outer; F_F frame hugs the sash via SurroundingRectangle.
- summer-strategy: blinds and purge tracks use outer.
- transmission-humidity: through-tracks use outer.

## Tests
Language-agnostic geometry + source asserts under each touched chapter's tests folder.
