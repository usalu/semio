# Notes

## Why it was still visible

1. Plugin defaulted unset `active_utility_id` to `move` (fixed earlier) — WASM needed rebuild.
2. Collapsing the Transform utility collection left `move`/`rotate`/`scale` pressed while the ribbon looked idle — gumball stayed on.
3. Renderer treated missing `transformMode` as `move`, so a stale/partial selection blob could still light the gumball.

## Fix

- Puzzle: no default transform utility; emit `transformMode` only for `move`/`rotate`/`scale`.
- Framework `UtilityTree`: keep path synced to the pressed leaf; collapsing a collection that owns the pressed leaf deactivates the utility.
- Framework `WorldInstancesLayer`: `isWorldTransformGumballMode` — gumball requires `gumballActive` **and** an explicit transform mode.

## Verify

- Hard-refresh Aggregator / Puzzle 3D after WASM rebuild.
- Select an object with no utility pressed → no gumball.
- Press Move → gumball appears; Escape or collapse Transform → gumball gone.
