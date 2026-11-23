# Plant House Windows Outer Leaf

## Problem
Anlagenverluste plant house showed teal casements on the inner wall leaf, with floor slabs past the facade so the under-window outer corner read as a gap.

## Fix
1. Derive `outer_l` / `outer_r` from wall centreline + thickness; floor slabs use those outers.
2. Left casement `inward=1`, right `inward=-1`, both `emphasize_casement` on the outer leaf.
3. Room/cellar fills span inner faces only.

## Tests
Source requires outer slab coords; Manim oracle checks sash edge = outer face.
