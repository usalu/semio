# Fix Solar Beat5 Sun Labels

## Problem
`Sommersonne` / `Wintersonne` collided with the sun disc/glow in Beat5.

## Cause
- Labels sat too close to the soft glow (bbox pad under-estimated bloom).
- `Transform` summer→winter could morph text onto the disc.
- Stage sat too far left for type-scale labels.

## Fix
- After `_fit_stage`, shift stage right to reserve label margin.
- Place labels with inflated clearance (`1.4×` sun size + pad), prefer above-left, else hard left; nudge until clear of the inflated disc.
- Fade out summer label, move sun, then fade in `Wintersonne` (no Transform).
- Nudge `Durch Überhang blockiert` further below/left of the awning.
- Slightly lower summer sun for headroom under the subtitle.

## QA
- `frames/final_summer.png` — Sommersonne clear of sun glow
- `frames/final_winter.png` — Wintersonne clear above-left of sun
