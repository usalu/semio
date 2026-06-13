---
name: Voxel Brush Alt Drag Paint
overview: Fix the puzzle3d voxel brush so that holding Alt and moving the mouse paints a continuous trail of voxels, instead of only painting a single voxel on Alt keydown or Alt+left-click.
todos:
  - id: fix-onmove
    content: Update onMove in VoxelBrushBridge (puzzle/3d/react/index.tsx) to commitAt + setAltPainting(true) when event.altKey is held during a pointer move.
    status: completed
  - id: verify
    content: Run the puzzle3d play app and confirm via the [DEBUG] commit log that holding Alt and dragging paints a one-voxel-per-cell trail.
    status: completed
isProject: false
---

## Problem

In `VoxelBrushBridge` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)), voxels are committed only on `Alt` keydown ([line 8830](puzzle/3d/react/index.tsx)) and on left-button + Alt pointerdown ([line 8853](puzzle/3d/react/index.tsx)). The `onMove` handler ([line 8848](puzzle/3d/react/index.tsx)) updates only `cursorCad` and never commits, so holding Alt while dragging paints nothing.

## Fix

In `onMove`, after updating `cursorCad`, paint when Alt is held during the move:

- If `event.altKey` is true: call `setAltPainting(true)` (keeps the preview/alt state consistent if the keydown was missed, e.g. pointer re-entering the canvas) and `commitAt(cad)`.
- The existing `commitAt` dedup via `lastCommitKeyRef` already prevents duplicate commits within the same cell, so a drag produces a clean one-voxel-per-cell trail.
- Keep the existing keydown and pointerdown behavior unchanged.

This makes the interaction: hold Alt and move the mouse to continuously paint voxels along the cursor path.

## Notes

- The temporary `[DEBUG]` log already present in `commitAt` ([line 8818](puzzle/3d/react/index.tsx)) will surface each commit at runtime for verification; keep it for now to confirm the drag-paint trail behaves, then it can be removed.
- Per repo workflow, do this inside a ticket (reopen an existing puzzle3d voxel-brush ticket if one matches, otherwise open a new one) and add the change using the existing region/subregion structure in the brush section of `index.tsx`.