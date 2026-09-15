# Context Menu Background Dismiss

## Symptom

In puzzle 3d (and any surface whose canvas calls `stopImmediatePropagation` on pointer-down capture, e.g. gumball drag), left-clicking empty viewport background left the context menu open.

## Cause

`ContextMenuController` registered outside-dismiss on `window` in the **bubble** phase. `stopImmediatePropagation` on canvas capture cancels bubbling, so the dismiss listener never ran.

`World3dHost` also did not clear `contextMenu` on empty background pick (`handleEmptyClick`), unlike the suggestion menu path.

Stale async `openSurfaceContextMenu` results could reopen the menu after dismiss.

## Fix

1. `ContextMenuController`: outside-dismiss on `window` **capture** (aligned with `CanvasPickMenu`).
2. `World3dHost`: `dismissContextMenu` with open epoch; clear on empty background click; ignore stale async opens.
