# Clear Stack Formula From Room

Scene: Beat4_Auftrieb.

## Problem
Tall Δρ formula panel overlapped the room floor.

## Fix
1. Raise/slightly shorten the room and inlet.
2. `fit_band(..., bottom=-0.82)` so the diagram stays above the panel (gap ≈ 0.55).
3. Keep default formula `edge_buff` (raising it lifted the panel into the room).
