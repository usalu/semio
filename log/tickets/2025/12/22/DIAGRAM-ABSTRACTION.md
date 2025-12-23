---
slug: DIAGRAM-ABSTRACTION
prompt: >-
  Diagrams should be generalized to be used for all diagrams (kit app, design
  app, quality app). Diagrams only work in controlled mode (state managment is
  done by the parent component). None of the apps (Kit.tsx, Design.tsx,
  Quality.tsx) should import react-flow directly or use any react-flow specific
  api. Elements.tsx should be the only file to import @xyflow/react and reexport
  the components as Diagram, Node, Edge, Handle, etc. All diagrams use the same
  coordinate system (one unit is equal to the diameter of the a circular
  nodes.). Optionally forced layout configs can be passed which every 50ms bulk
  updates all centers of the nodes through a d3-force layout.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-22T22:14:17.678Z"
iterations:
  - prompt: Initial implementation of diagram abstraction
    model: claude-sonnet-4-20250514
    date: "2025-12-22T22:14:17.678Z"
    files:
      updated:
        - js/js/sketchpad/elements.tsx
        - js/js/sketchpad/Kit.tsx
---

# Previously

No prior work on this ticket.

# Plan

1. Refactor elements.tsx to be the single source for @xyflow/react imports
2. Create unified Diagram API with controlled mode and d3-force layout support
3. Define DIAGRAM_UNIT constant (48px = diameter of circular nodes)
4. Export all necessary types and utilities for custom implementations
5. Update Kit.tsx to import from elements.tsx instead of d3-force directly
6. Verify Design.tsx and Quality.tsx already import from elements.tsx

# Changes

## elements.tsx

- Added d3-force imports and re-exports (forceSimulation, forceLink, forceManyBody, forceCollide, forceCenter)
- Added DIAGRAM_UNIT constant (48px) for unified coordinate system
- Created DiagramNodeData and DiagramEdgeData interfaces for controlled mode
- Created DiagramForceConfig interface with 50ms update interval default
- Created useForceSimulation hook for d3-force layout support
- Created new controlled-mode Diagram component with domain-neutral API
- Re-exported all necessary @xyflow/react components and types

## Kit.tsx

- Removed direct d3-force import
- Updated imports to use elements.tsx for all d3-force utilities and types
