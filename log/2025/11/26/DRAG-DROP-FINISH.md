---
date: "2025-11-26T18:16:40.379Z"
slug: DRAG-DROP-FINISH
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Finish drag and drop functionality for pieces in diagram and scene
model: claude-sonnet-4.5
---

# Previously

## Investigation

1. The drag starts correctly - `useDraggable` from `@dnd-kit/core` in `TypeTreeItem` triggers `setActiveDraggedType(type)`
2. The drag is detected - status bar shows "Draggable item type-... was dropped"
3. But the piece is NOT created

## Root Cause Analysis

### First attempt: Using `useDroppable` + custom events

The initial approach was to use `useDroppable` to register the diagram as a drop target and listen for `design-drag-end` events. However, this failed because:

1. Each GoldenLayout window (diagram, scene) is wrapped in its own `<DragDropProvider>` in `LayoutCanvas.tsx`
2. This means `useDroppable` registers with the window's local `DndContext`, not the main `DndContext` in Sketchpad.tsx
3. The collision detection shows `droppableContainersCount: 0` because no droppables are registered with the parent context

### Solution: Pointer coordinate-based drop detection

Instead of relying on `event.over` (which is always `undefined`), we:

1. Store the diagram's DOM element in a ref using `setDropZoneRef`
2. On `handleDragEnd`, calculate drop coordinates from `event.activatorEvent.clientX + delta.x`
3. Use `getBoundingClientRect()` to check if drop is within diagram bounds
4. If within bounds, use `reactFlowInstance.screenToFlowPosition()` to convert to diagram coordinates

This bypasses the DndKit droppable system entirely and works reliably.

### Bug: Piece type/design format

Found that the piece `type` and `design` fields must be objects with a `guid` property (e.g., `{ guid: "..." }`), not plain strings. The `TypeIdSchema` and `DesignIdSchema` define these as objects.

# Plan

1. ~~Add `useDroppable` to DesignDiagram wrapper with unique ID~~ (removed - doesn't work with GoldenLayout)
2. ✅ Store diagram ref using `setDropZoneRef` callback
3. ✅ Check pointer coordinates against diagram bounds in `handleDragEnd`
4. ✅ Create piece at pointer position when drop is within bounds
5. ✅ Clear `activeDraggedType`/`activeDraggedDesign` after drop
6. ✅ Repeat similar pattern for DesignAppScene (3D scene)
7. ✅ For scene: use normalized screen coordinates to calculate world position on y=0 plane
8. ✅ Remove debug logs

# Changes

## Design.tsx

### DesignDiagram - handleDragEnd

- Removed dependency on `event.over` (which was always undefined)
- Added bounds checking using `dropZoneRef.current.getBoundingClientRect()`
- Calculate drop position using `event.activatorEvent.clientX/clientY + delta.x/y`
- Check if drop position is within diagram bounds before creating piece
- Fixed piece creation to use `type: { guid: droppedType.guid }` instead of plain string

### DesignDiagram - setDropZoneRef

- Sets `data-drop-zone="diagram"` attribute on the wrapper div for test locators
- Stores ref for bounds checking

### DesignAppScene - handleSceneDragEnd (NEW)

- Same pointer coordinate-based drop detection as diagram
- Calculates world position from normalized screen coordinates
- Uses camera distance to scale the drop position appropriately
- Creates piece with a `plane` property (origin at calculated x/z, y=0)

## sketchpad.test.ts

### Drag and Drop Pieces test

- Rewrote to use manual mouse operations (`page.mouse.move/down/up`) instead of `dragTo` which was blocked by scroll area
- Uses `data-drop-zone` attributes to locate diagram and scene drop zones
- Verifies piece count after drag to diagram (expects 1)
- Verifies piece count after drag to scene (expects 2)

# Verified Working

All Playwright tests pass:

- Windows test: ✅
- Drag and Drop Pieces test: ✅
