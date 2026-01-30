# Ticket

## Todos
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

## Changes

## Log

## Summary
# Summary
