---
slug: DROP-COORDS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix drop coordinates for diagram and scene
model: claude-sonnet-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

The drag-and-drop functionality was implemented but pieces were created at incorrect positions. The user reported that after dropping, the piece should appear where the mouse cursor was.

# Plan

1. ✅ Fix diagram drop coordinates to use local position relative to container
2. ✅ Add offset to center node on drop point (account for 50x50 node size)
3. ✅ Fix scene drop coordinates using proper orthographic camera unprojection
4. ✅ Add test assertion for piece position verification
5. ✅ Clean up debug logging

# Changes

## Design.tsx

### Diagram drop handler (handleDragEnd)

- Changed from using `screenToFlowPosition()` (which was unreliable due to viewport state) to calculating local coordinates directly
- Center is now: `{ u: localX/ICON_WIDTH - 0.5, v: -localY/ICON_WIDTH + 0.5 }`
- The -0.5 and +0.5 offsets center the 50x50 node on the drop point

### Scene drop handler (handleSceneDragEnd)

- Implemented proper orthographic camera unprojection
- Calculates camera right and up vectors from forward and up
- Uses viewport size and zoom (50) to calculate world position
- Raycasts to y=0 plane to get the intersection point

## sketchpad.test.ts

- Updated locator for draggable type avatar: `[data-slot="avatar"][title="New Type"]`
- Added position verification: checks that piece center is within 50px of drop point
