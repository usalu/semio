# 100 W Label Placement

## Problem
On the viewer frame, `≈100 W` sat on the person's right and bled into the cream room wall.

## Fix
1. Build the label, `next_to(person, LEFT, buff=0.18)`.
2. Lock y to mid-torso (`foot + 0.55·height`).
3. Clamp center x so the whole glyph stays in the air inset (0.7 from each wall).

## Check
Orange glyph x≈251–328; right wall≈456; clearance≈128 px; intersection with cream = 0.
