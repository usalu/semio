# Gumball live preview

## Root cause
`World3dHost` only dispatched `translateSelection` / `rotateSelection` / `scaleSelection` on gumball drag **end**. The gumball widget moved client-side, but scene instances stayed at committed document poses until release.

## Fix
- Framework `World3dHost` now dispatches incremental transform deltas on every gumball `onDrag` tick, brackets the gesture with `transformBegin` / `transformEnd`, and flushes any final micro-delta on drag end.
- Exported `gumballTransformDeltaBetweenPoses` helper + unit tests.
- Lowpoly render overlays the in-progress transform scratch projection so scratch-commit apps preview mid-drag too.

## Apps benefiting without plugin changes
Puzzle 3d/5d (coalesced amend), CAD, Shooting, Procedural 3d.
