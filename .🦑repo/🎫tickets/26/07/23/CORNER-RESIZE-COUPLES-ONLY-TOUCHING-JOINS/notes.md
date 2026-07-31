# Corner resize — couple only touching joins

## Bug
After the peer-axis fix, corner drag stayed sticky: resizing one column’s vertical split, then dragging a corner, still moved both columns’ heights.

## Cause
`modePerpendicularJoinSeparators` used even `(i/count)` fractions, so left/right always looked “aligned” at 0.5 even when live sizes were 30/70 vs 50/50.

## Fix
Join fractions are size-weighted (cumulative child `%` / total). `resolveJoinCornerPeerCrossAxes` only peers joins within `MODE_JOIN_CORNER_TOUCH_EPS`. Misaligned corners resize only the windows that share that point; a true `+` (all four corners touching) still resizes all four.
