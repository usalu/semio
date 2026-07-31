# Plan: Fix D3 Force Simulation To Match Canonical Example

## Goal

Implement the D3 force simulation exactly as shown in the canonical d3-force example at https://d3js.org/d3-force, ensuring proper drag behavior with correct event.active checks and alphaTarget management.

## Current Issues

1. The drag handlers don't check `event.active` before reheating/cooling the simulation
2. The simulation behavior doesn't match the canonical example where other nodes move smoothly during drag

## Key Differences from Canonical Example

### Canonical Example Pattern:

```javascript
function dragstarted(event) {
 if (!event.active) simulation.alphaTarget(0.3).restart();
 event.subject.fx = event.subject.x;
 event.subject.fy = event.subject.y;
}

function dragged(event) {
 event.subject.fx = event.x;
 event.subject.fy = event.y;
}

function dragended(event) {
 if (!event.active) simulation.alphaTarget(0);
 event.subject.fx = null;
 event.subject.fy = null;
}
```

### React Flow Adaptation

Since React Flow provides its own drag handlers (not D3's drag behavior), we need to:

1. Track if we initiated the reheat to avoid redundant restarts
2. Only reheat if the simulation isn't already active from another source
3. Only cool down if we were the ones who reheated it

## Implementation Steps

1. Add a ref to track if simulation is already reheated by our drag: `isDraggingActiveRef`
2. Update `handleNodeDragStart`:
   - Check if simulation alpha is low enough to need reheating
   - Only call `alphaTarget(0.3).restart()` if needed
   - Set tracking ref
3. Update `handleNodeDrag`:
   - Simply update fixed position (no velocity manipulation)
4. Update `handleNodeDragStop`:
   - Only reset alphaTarget if we were the ones who reheated
   - Release fixed position
   - Clear tracking ref

## Files to Change

- `/workspaces/semio/js/compose/sketchpad/Kit.tsx` - Update drag handlers to match canonical pattern
