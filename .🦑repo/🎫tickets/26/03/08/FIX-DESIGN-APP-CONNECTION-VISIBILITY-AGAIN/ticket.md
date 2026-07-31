---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed connection visibility by restoring edges={edges} to Diagram component and removing broken CustomDesignEdgeLayer ViewportPortal. 0 TS errors, 15/15 unit tests pass.

## Changes

- Changed `edges={EMPTY_EDGES_ARRAY}` to `edges={edges}` in the Diagram component call in Design.tsx
- Removed `CustomDesignEdgeLayer` ViewportPortal from the Diagram panels prop in Design.tsx

## Log

- Investigated Design.tsx connection/edge rendering pipeline
- Found `edges={EMPTY_EDGES_ARRAY}` at line 8862 suppressing all ReactFlow edges
- Found `CustomDesignEdgeLayer` mounted in ViewportPortal as custom SVG replacement that doesn't render visibly
- Confirmed this is same regression as ticket 2026/03/03/FIX-DESIGN-APP-CONNECTION-VISIBILITY
- Applied fix: restored `edges={edges}` and removed CustomDesignEdgeLayer portal
- TypeScript compilation: 0 errors
- Vitest unit tests: 15/15 passed

## Todos

- [x] Investigate connection/edge rendering
- [x] Find root cause (EMPTY_EDGES_ARRAY passed instead of edges)
- [x] Fix by restoring edges={edges} and removing broken custom edge layer
- [x] Run unit tests (15/15 passed)

## Plan

1. Pass real edges array to Diagram component instead of EMPTY_EDGES_ARRAY
2. Remove broken CustomDesignEdgeLayer ViewportPortal from panels
