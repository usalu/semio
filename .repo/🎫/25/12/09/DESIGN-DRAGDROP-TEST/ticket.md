# Ticket

## Todos

# Previously

The existing Design test verifies:

- Navigation to design app
- Diagram and scene visibility
- Pan operations on diagram and scene
- Hover interactions on pieces
- Flat plane/center validation for Nakagin Capsule Tower design

No drag and drop functionality was tested previously.

# Plan

1. Add helper functions for:
   - Creating an empty design (`initEmptyDesign`)
   - Getting pieces from the active design (`getDesignPieces`)
   - Getting plane data from all pieces (`getPiecePlanes`)

2. Create new test "Design Drag and Drop" that:
   - Creates a new empty design
   - Opens workbench panel to access types
   - Drags types from workbench to scene at 5 positions (center, 4 corners)
   - Pans the canvas using middle mouse button
   - Drops 2 more pieces after panning
   - Validates that all dropped pieces have correct plane properties:
     - origin.z = 0 (on ground plane)
     - xAxis = (1, 0, 0)
     - yAxis = (0, 1, 0)

3. The test uses dnd-kit's drag mechanism via pointer events simulation

# Changes

- Added `initEmptyDesign` helper function
- Added `dragTypeToPosition` helper function (may not be needed directly)
- Added `getPiecePlanes` helper function
- Added `getDesignPieces` helper function
- Added "Design Drag and Drop" test with:
  - 5 drop positions: center, top-left, top-right, bottom-left, bottom-right
  - Canvas panning followed by 2 more drops
  - Plane validation for origin.z=0, xAxis=(1,0,0), yAxis=(0,1,0)

## Changes

## Log

## Summary

# Summary

> -
