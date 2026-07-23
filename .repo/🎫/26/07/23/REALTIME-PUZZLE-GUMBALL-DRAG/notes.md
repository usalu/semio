# Realtime Puzzle Gumball Drag

## Problem
Per-pointermove `translateSelection`/`rotateSelection` runs full attraction resolve + document delta + VCS amend + full UI refresh. SceneGumball also resets the pivot whenever `gumballTarget` updates mid-drag, which fights the live handle and breaks subsequent drags.

## Fix
1. Puzzle3d: `transformBegin` / mid-drag scratch / `transformEnd` commit (mirror lowpoly).
2. Mid-drag: apply absolute delta from drag-start pose onto a fixture base snapshot; emit no ops; refresh only the world composite body.
3. Framework: absolute-from-start gumball deltas + rAF latest-wins coalesce; SceneGumball ignores `gumballTarget` while dragging.
