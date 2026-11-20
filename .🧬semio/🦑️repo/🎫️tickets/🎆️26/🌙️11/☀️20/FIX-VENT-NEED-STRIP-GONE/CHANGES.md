# Hide Counting Strip With Formula

Scene: `Beat1_Lueftungsbedarf` personen.

## Problem
Morph into the formula left the live sources (`4 ×`, one `30 m³/h`, `= 120`) on screen after the blue boxes were gone.

## Fix
1. Snapshot sources with `_snap`.
2. `FadeOut` + `remove` every strip mobject (boxes and texts).
3. Only then morph the snaps into the formula panel.
