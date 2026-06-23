# Ticket

## Todos
# Previously

Kit diagram had several issues:

- Node icons were different from table avatars (used icon components vs TableAvatar)
- Node edge handles had visible circles larger than the node circles
- Nodes were not draggable
- Table and diagram states were not synchronized (filtering, expansion, hover)
- Selection on nodes fired events but selection was not reflected

# Plan

1. Update KitArtifactNode to use TableAvatar component for consistent icons
2. Fix Handle components to be completely invisible (no visible circles)
3. Enable nodesDraggable={true} in ReactFlow
4. Add filtering logic to KitDiagramInner that syncs with table's expandedRows and filterSearch
5. Fix hover handlers to use useKitAppClearHover for proper cleanup
6. Add comprehensive Playwright tests for all features

# Changes

## js/compose/sketchpad/Kit.tsx

- **KitArtifactNode**: Replaced custom icon rendering with TableAvatar component for consistent appearance with table rows
- **KitArtifactNode**: Changed Handle components from `!w-2 !h-2 !opacity-0` to `!w-0 !h-0 !bg-transparent !border-none` to completely hide edge connection points
- **KitArtifactNode**: Added `useKitAppClearHover` hook for proper hover state cleanup on mouse leave
- **KitArtifactNode**: Updated container styling to use `cursor-grab active:cursor-grabbing` for drag indication
- **KitDiagramInner**: Updated `visibleGuids` to include ALL artifact types (types, designs, qualities, ports, tags, concepts, files, folders, authors) instead of just types/designs
- **KitDiagramInner**: Removed parent expansion filtering - all nodes are now visible regardless of table row expansion state
- **KitDiagramInner**: Modified node/edge building to filter based on visible GUIDs from search filter only
- **KitDiagramInner**: Added `visibleGuids` to effect dependency for resetting fit view on filter changes
- **FloatingEdge**: Fixed edge endpoint calculation by offsetting `positionAbsolute` to node center (add NODE_RADIUS to top-left corner position)
- **ReactFlow**: Added `nodesDraggable={true}` prop to enable node dragging

## js/compose/sketchpad.test.ts

- Added "Kit Diagram - Node Icons Match Table Avatars" test: verifies nodes contain TableAvatar elements
- Added "Kit Diagram - Node Dragging" test: verifies nodes can be dragged and position changes
- Added "Kit Diagram - Table Selection Sync" test: verifies table selection updates diagram state
- Added "Kit Diagram - Node Click Selection" test: verifies clicking diagram nodes updates selection
- Added "Kit Diagram - Hover Sync" test: verifies hover state sync between table and diagram
- Added "Kit Diagram - Filter Sync" test: verifies search filter reduces diagram node count
- Added "Kit Diagram - All Artifact Types Visible" test: verifies all artifact types (types, designs, qualities, ports, tags, concepts, files, folders, authors) are rendered as nodes
- Added "Kit Diagram - Edges Connect Nodes" test: verifies edges exist and have valid SVG paths
- Added "Kit Diagram - Node Dragging Updates Position" test: verifies ReactFlow receives drag events

## Changes

## Log

## Summary
# Summary

"Kit diagram: sync icons, dragging, filtering, selection with table"
