# Ticket: Fix D3 Force Simulation To Match Canonical Example

## Todos
- [x] Add ref to track drag-initiated reheat
- [x] Update handleNodeDragStart with event.active pattern
- [x] Update handleNodeDrag to only update fixed position  
- [x] Update handleNodeDragStop with event.active pattern
- [x] Test drag behavior matches canonical example

## Changes

### Kit.tsx
Implementing proper D3 force simulation drag behavior following the canonical example pattern from https://d3js.org/d3-force.

Key changes:
- Add `isDragReheatActiveRef` to track if we initiated the simulation reheat
- Check simulation alpha before reheating (equivalent to `if (!event.active)`)
- Only cool down simulation if we were the ones who reheated it
- Remove unnecessary velocity/alpha manipulation during drag

## Log

### Implementation Complete
- Added `isDragReheatActiveRef` to track if drag initiated the simulation reheat
- Updated `handleNodeDragStart`:
  - Checks if `simulation.alpha() < 0.3` before reheating (equivalent to `if (!event.active)`)
  - Only calls `alphaTarget(0.3).restart()` if simulation needs reheating
  - Sets tracking ref when we reheat
  - Fixes node position to current simulation position
- Updated `handleNodeDrag`:
  - Simply updates fixed position (`fx`, `fy`) to follow cursor
  - No velocity manipulation or manual alpha adjustment
- Updated `handleNodeDragStop`:
  - Only resets `alphaTarget(0)` if we were the ones who reheated (equivalent to `if (!event.active)`)
  - Releases node by setting `fx` and `fy` to null
  - Clears tracking ref

The implementation now exactly matches the canonical D3 force example pattern where:
- Dragging a node reheats the simulation if it's cooled down
- Other nodes respond in real-time through force calculations
- Connected nodes are pulled via link forces
- Nearby nodes are repelled/attracted via charge forces
- Simulation cools down naturally after drag ends

## Summary

Implemented D3 force simulation drag behavior matching the canonical example at https://d3js.org/d3-force. Added isDragReheatActiveRef to track drag-initiated reheats, updated drag handlers to check simulation alpha before reheating (equivalent to event.active check), and only cool down if we initiated the reheat. The simulation now properly responds to drag with connected and nearby nodes moving in real-time through force calculations.
