# Active Border Only When Selected

## Problem
- Window dock stacks painted the primary active silhouette from layout `activeWindowId` (seeded on load), so Aggregator Perspective looked selected before any click.
- Introduction steps forced `borderKind="introduced"` forever and could not enter the shared activate → normal lifecycle.

## Fix
- Silhouette `data-active` / primary stroke for ModeDockStack follows `useSurfaceActive` (click/focus), same as panels and panes. Layout `activeWindowId` still drives tab fills and command focus.
- Activating any surface root clears its `data-introduced` stamps so the active stroke can win.
- Introduction info box: introduced until first activation, then active while selected, then normal after a background click (never re-enters introduced for that step).
- `useSurfaceActive` layout cleanup defers clearing the active root via `queueMicrotask` so same-commit re-registers do not drop the stroke.
