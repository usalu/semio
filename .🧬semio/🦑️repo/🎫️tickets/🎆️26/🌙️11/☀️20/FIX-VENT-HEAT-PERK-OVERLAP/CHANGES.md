# Clear Unit Cards Before Perk Formula

Scene: `Beat1_Luftwaerme` perk beat.

## Problem
Yellow `V̇ · c_Luft` readout was `add`ed and ramped in the same `play` as `FadeOut(both, note)`, so it sat on the SI card for ~1 s.

## Fix
`_hide` + `remove` the unit cards first, then start the perk count and formula panel.
