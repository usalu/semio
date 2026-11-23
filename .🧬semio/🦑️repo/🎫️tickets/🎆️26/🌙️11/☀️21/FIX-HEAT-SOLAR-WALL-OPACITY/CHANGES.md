# Solar Intensity Facade Wall Opacity

## Problem
Irradiance beat wall used opaque `section_material` (fill 1.0), reading as a solid cream block.

## Fix
After assembling the four wall panels, `.set_fill(opacity=0.4)` so the facade stays readable but translucent.

## Tests
Source asserts `set_fill(opacity=0.4)`.
