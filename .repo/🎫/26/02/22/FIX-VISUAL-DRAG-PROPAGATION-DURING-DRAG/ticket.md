# Fix Visual Drag Propagation During Drag

## Status: OPEN

## Goal
r26.02-1/Running Sketchpad

## Problem
When dragging a node (e.g. TA), all downstream descendants (S, P, L, J, etc.) should visually move together during the drag. Currently, descendants only move after the drag is completed (on mouse-up) because `onNodeDrag` only updates `internals.positionAbsolute` (internal tracking) but never calls `setNodes()` to trigger React re-renders.

## Root Cause
In `onNodeDrag` handler in Design.tsx, the descendant loop updates `descInternalNode.internals.positionAbsolute.x/y` which is ReactFlow's internal tracking state. This does NOT cause React to re-render the DOM nodes. ReactFlow renders node positions from the React state `nodes` array via CSS `transform: translate(...)`, not from internal node positions.

## Fix
Call `setNodes()` with an updater function inside the descendant handling section of `onNodeDrag` to batch-update all descendant positions in React state, causing visual re-render during drag.

## Plan
- [x] Read current onNodeDrag code to assess state
- [ ] Fix visual drag propagation by adding setNodes() call
- [ ] Run and verify Playwright tests
- [ ] Close ticket

## Changes
- `compose/js/sketchpad/Design.tsx`: Add `setNodes()` call in onNodeDrag descendant handling

## Summary
