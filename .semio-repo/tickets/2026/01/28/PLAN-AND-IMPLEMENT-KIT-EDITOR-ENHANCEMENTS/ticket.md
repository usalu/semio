# Ticket

## Todos

## Changes

- Updated `Kit.tsx` to run a local D3 force simulation with drag pinning, alphaTarget guards, and fallback cleanup while keeping React Flow as the renderer.
- Added edge alignment diagnostics and size-aware intersection math using absolute positions, plus optional debug markers and logs.
- Documented kit diagram simulation sync and drag behavior in `README.md` and `AGENTS.md`.

## Log

- **2026-01-28**: Initial analysis of `Kit.tsx` and `elements.tsx`. Identified that `KitDiagramInner` uses `ReactFlow` directly and has custom D3 simulation logic.
- **2026-01-28**: Created the following MD prompts for planning next steps as requested:
- **2026-01-28**: Implemented kit diagram simulation sync, drag reheating safeguards, and edge alignment diagnostics; updated dev docs.

### MD Prompts for Kit Editor Plan

- **Prompt 1: D3 Physics Optimization**
  "Analyze the `DiagramForceSettings` in `Kit.tsx`. How can we adjust `chargeStrength` and `collideRadius` to prevent overlapping without making the layout too sparse? Implement a dynamic strength adjustment based on node count."

- **Prompt 2: Multi-Select Integration**
  "Enable `selectionOnDrag` in the `KitDiagramInner`'s `ReactFlow` component. Update the selection handlers to manage a list of selected GUIDs instead of a single one. Ensure Shift+Click toggles individual nodes."

- **Prompt 3: Alignment Fix**
  "Inspect `FloatingEdge` and `DiagramNode` in `elements.tsx`. The edges appear offset from the center of the nodes. Adjust the handle positions or the edge source/target calculation to align with the visual center of the avatar circles."

## Summary

Implemented Kit diagram D3/React Flow sync with drag pinning, alphaTarget reheating/cooldown, and edge alignment diagnostics; updated README/AGENTS.
