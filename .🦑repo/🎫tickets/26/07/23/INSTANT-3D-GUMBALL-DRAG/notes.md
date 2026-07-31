# Instant 3D Gumball Drag

## Problem
Aggregator (puzzle3d) gumball drag felt laggy vs catalogue drag-and-drop. Mid-drag still dispatched every tick through WASM → scratch fixture → partial world composite rebuild → React instancesJson parse/re-render. The gumball pivot moved instantly, but selected meshes trailed.

## Fix
1. Mid-drag is local-only: `WorldInstancesLayer` captures selected instance root poses and imperatively applies start→current translate/rotate/scale (same instant path as catalogue drop ghosts).
2. Host skips mid-drag `translateSelection` / `rotateSelection` / `scaleSelection` WASM ticks.
3. On drag end: one absolute start→end delta onto the transform scratch, then `transformEnd` commits once.
4. Commit-hold reapplies final local poses until `instancesJson` catches up so release does not flash the pre-drag pose.
