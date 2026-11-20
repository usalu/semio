# Hard Gap Under Room For Flow Panel

Scene: Beat4_Auftrieb (flow / V̇ panel).

## Problem
V̇ formula looked glued to the room because:
1. Clearance used `room.get_bottom()` (walls) instead of the lower white **floor slab**.
2. Tall captions could shove the panel upward.
3. `A = 0.1 m²` hung **under** the slab into the formula band.

## Fix
1. Clear under `min(room, floor)` by 1.05; room clearance always wins.
2. Raise room + `TOP_LOW`/`TOP_HIGH` together.
3. Place `A` inside the room (right of the inlet).
4. Flatten `panel_q` units; shorten the flow caption.
