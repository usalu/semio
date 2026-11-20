# Reposition Volumenstrom Handover Chips

Scene: `Beat3_Volumenstrom` cover beat.

## Problem
`chips.next_to(box3, UP)` spanned a wide pair across the formula — Heizung sat on the room, Kühltechnik crowded the reserve/formula.

## Fix
1. Formula `edge_buff=1.7`, reserve nudged up.
2. Chips stacked `DOWN` on the bar column with guards vs room-right and formula-top.
3. Fallback: park to the right of the total bar if the mid slot is too tight.
