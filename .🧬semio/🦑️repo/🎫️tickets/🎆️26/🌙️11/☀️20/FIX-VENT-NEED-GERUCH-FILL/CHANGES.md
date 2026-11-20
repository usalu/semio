# Animate Geruch Gauge Fill

Scene: Beat1_Lueftungsbedarf.

## Problem
`FadeIn(odor_col)` ran in the same play as `odor → 1.0`, so the bar appeared already full.

## Fix
Match Feuchte: FadeIn the empty column, then animate the fill over 4 s.
