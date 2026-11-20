# Fix Ventilation Need Overlap

Scene: `Beat1_Lueftungsbedarf` in `🌬️ventilation/🎬️clip/❓️need`.

## Problem
The four `30 m³/h` chips, `4 ×`, `= 120`, and the `V̇_Pers` formula panel sat in the same band and cut through each other.

## Changes
1. Raise the room (`y 0.45→1.05`, slightly shorter) for a clear mid band.
2. Place rate chips at `y = -0.55` under the room; total to the right of the last chip.
3. Formula panel `edge_buff=2.05`; fade rate chips after they feed the panel.
4. Nudge winter/summer cards right/up so they clear the raised room.
