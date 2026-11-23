# Solar Shade Wall Flush Under Window

## Problem
Beat5 floor/ceiling slabs started at the facade *inner* face (`lintel.get_right()`), while sill/lintel masonry sat further out — a step/gap under the window. The casement also used `inward=-1` on a left facade (sash on the inner leaf).

## Fix
- Single `facade_t`; slabs start at `glass_out` so floor outer = sill outer.
- Casement `inward=1` (outer leaf on the exterior face).

## Tests
Source requires `slab_left … = glass_out`, `inward=1`, `facade_t`.
