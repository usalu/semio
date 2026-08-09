# Notes

## Problem
Beat3 convection played warm-out then cold-in sequentially with sparse dots on flat paths — read as one bead crossing, not air exchanging.

## Fix
- `animate_flows()` — multiple coloured streams in one play; streak ellipses oriented to path tangent.
- `animate_flow()` — thin wrapper (Cooling scenes pick up streaks automatically).
- Beat3 paths: rise indoors → exit gap top → outdoor plume; sink outdoors → enter gap bottom → settle indoors (3 lanes each).
- Simultaneous orange/yellow + blue/cyan loops; tracked Luftpaket with glow while exchange continues.

## Verify
`manim -ql --disable_caching scene_1.py Beat3_Konvektion` — rendered clean (27 animations).
