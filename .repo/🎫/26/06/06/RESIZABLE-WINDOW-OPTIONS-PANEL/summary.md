# Resizable Window Options Panel

Unfolded window options rails can be resized from the left edge (width) and bottom edge (height).

## Changes

- Added `WindowMeasuresResizeHandle` with left/bottom mouse-drag resize
- `Window` tracks `measuresWidthPx` and optional `measuresHeightPx` when unfolded
- Resize handles hidden when folded or span-expanded
- Updated window measures tests

## Files

- `ui/react/index.tsx`
