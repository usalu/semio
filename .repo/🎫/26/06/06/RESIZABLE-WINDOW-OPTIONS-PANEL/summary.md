# Resizable Window Options Panel

Unfolded window options rails resize horizontally from the left edge; height follows content up to the window bottom, then scrolls.

## Changes

- Added `WindowMeasuresResizeHandle` for left-edge width resize
- Height is content-driven (`flex-auto` body, `max-h-full` stack/overlay) with scrollbar when capped
- Removed bottom resize handle and explicit height state
- Updated window measures tests

## Files

- `ui/react/index.tsx`
