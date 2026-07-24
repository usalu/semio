# Fix Gumball Drag Cursor Scale

## Problem
3D gumball drag: selection flies away from the cursor (world delta ≫ pointer delta).

## Root causes
1. **Absolute host deltas × accumulate plugins (main):** After the realtime-gumball work, the world host sent absolute `start→pose` deltas every tick. CAD (`AmendLast`), lowpoly (scratch accumulate), and puzzle-without-session all **add** each dx. Absolute totals stacked → exponential fly-away. Ortho duck-typing alone could not fix this.
2. **Ortho `instanceof` (prior):** `gumballRayFromNdc` treated R3F ortho cameras as perspective across duplicate `three` copies.

## Fix
- Host: incremental `lastDispatched→pending` (rAF coalesce safe); drag-end sends only the remaining increment then `transformEnd`.
- Puzzle scratch: accumulate increments (match lowpoly/CAD).
- Keep ortho `isOrthographicCamera` duck-typing + `updateMatrixWorld` before unproject.
- `[DEBUG] gumball drag begin/tick/end` logs for runtime confirmation.
