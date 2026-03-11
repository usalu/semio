# Design Drag Performance Long Task Optimization

## Goal
Make all long tasks during the "Design Drag Performance" test stay under 50ms budget.

## Context
- Test: `sketchpad.test.ts` "Design Drag Performance"
- Assertion: `expect(maxLongTaskDuration).toBeLessThanOrEqual(50)`
- Current: Fails with ~4923ms max long task, 134 long tasks total
- Design: Nakagin Capsule Tower — 180 pieces (nodes), 179 connections (edges)

## Root Cause Analysis
The 3841ms long task occurs due to zustand store subscriber cascades:
1. ReactFlow calls `store.setState({ nodesSelectionActive: false })` at mousedown even when already false
2. zustand always notifies ALL ~360 subscribers because `Object.is(newState, oldState)` fails on new object reference
3. CustomDesignEdgeLayer subscribes to ALL store changes, scheduling rAF → recompute 179 edges → setTick → React re-renders 179 SVG elements
4. During zoom: `store.setState({ transform })` fires every frame, triggering the same edge cascade

## Plan
1. Remove all [DEBUG] console logs from Design.tsx (adds overhead)
2. Patch zustand store.setState to skip no-op updates (prevents cascade when nodesSelectionActive already false)
3. Suppress CustomDesignEdgeLayer recomputation during drag/zoom via isPanningRef/isDraggingNodeRef
4. Add pointerdown capture listener to set isDraggingNodeRef BEFORE ReactFlow processes mousedown
5. Run test and verify pass
6. Run full test suite

## Changes
- [ ] Remove [DEBUG] console logs
- [ ] Patch zustand setState for no-op skip
- [ ] Suppress edge layer recompute during drag/zoom
- [ ] pointerdown capture for early drag flag
- [ ] Verify test pass
- [ ] Verify full suite pass

## Notes
- Previous optimizations: helperLines DOM fix, sync effect identity check, edge stability, CSS containment, pointer-events during drag
- selectNodesOnDrag={false} is already set
- Controlled mode: triggerNodeChanges only calls onNodesChange, not internal set()
- Our onNodesChangeReactFlow filters out select changes and returns early during drag
- Test cannot be modified
- No functionality can be removed
