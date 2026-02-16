# Plan - Kit Editor Enhancements

## Phase 1: Diagnostics and Simulation Fixes
- [ ] Instrument node/edge alignment with positionAbsolute checks, size validation, and optional endpoint markers.
- [ ] Trace simulation state during drag (alpha, alphaTarget, fx/fy) and confirm tick continuity.
- [ ] Fix alphaTarget handling with drag-start/drag-end guards and cleanup to avoid freezes.
- [ ] Enforce D3↔React Flow sync: simulation drives idle nodes, drag pins active node without jitter.

## Phase 2: Selection and Interaction
- [ ] Fix the "Auto-select" bug where multiple nodes are selected unexpectedly.
- [ ] Enable `selectionOnDrag` and configure Shift+Click behavior.
- [ ] Implement rectangular selection in the `Diagram` component and ensure it syncs with the app state.

## Phase 3: Filtering and Table UI
- [ ] Update the `KitToolbarFilters` to support multi-select filtering.
- [ ] Adjust table styles in `Kit.tsx` and `elements.tsx` to fix alignment issues.

## Phase 4: Optimization and Polish
- [ ] Fine-tune `defaultDiagramForceSettings` (charge, link distance, collide radius).
- [ ] Ensure consistent behavior between `Diagram` (shared) and `KitDiagram` (specific).
