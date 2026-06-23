---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed 3D model selection in Design app Scene window. Threaded click/hover event handlers through the entire mesh component chain (GLTFMesh, FBXMesh, OBJMesh, LoadedPieceMesh, PieceMesh) down to the `<primitive>` element. Previously events attached on a parent `<group>` did not reliably propagate from drei primitives in R3F v9. Also fixed pre-existing JSON import attributes in compose.ts.

## Changes

- `compose/js/sketchpad/Design.tsx`: Added `DesignMeshEventProps` interface. Updated GLTFMesh, FBXMesh, OBJMesh, LoadedPieceMesh, PieceMesh to accept and forward onClick/onDoubleClick/onPointerEnter/onPointerLeave handlers to `<primitive>` elements. Removed duplicate handlers from parent `<group>` to prevent double-firing.
- `compose/js/compose.ts`: Added missing `with { type: "json" }` import attributes to adjectives.json, animals.json, and constants.json imports.

## Log

1. Analyzed Scene section rendering chain: ModelPiece → PieceMesh → LoadedPieceMesh → GLTFMesh/FBXMesh/OBJMesh → `<primitive>`.
2. Identified that R3F event propagation from `<primitive>` children (GLTF meshes without `__r3f` fibers) to parent React `<group>` components is unreliable with drei Select wrapping.
3. Created `DesignMeshEventProps` interface and threaded handlers through the full chain.
4. Removed duplicate handlers from parent `<group>` to prevent double-fire in toggle selection mode.
5. Verified bidirectional sync: Diagram→Scene and Scene→Diagram share `useDesignAppSelection()` state. Diagram syncs via `useEffect([selection])` that maps selection to React Flow node `selected` prop.
6. Fixed pre-existing JSON import attribute issues in compose.ts blocking Playwright e2e tests.
7. Build passes (vite + storybook). 14 unit tests pass. Design e2e passes through all scene/diagram tests.

## Todos

- [x] Research Scene/Diagram selection code
- [x] Thread click handlers to primitives
- [x] Verify diagram-scene sync code
- [x] Handle Select box interaction
- [x] Build and verify compile
- [x] Run tests and verify

## Plan

The Design app Scene window uses R3F with drei's `<Select box multiple>` wrapper. Pieces with 3D models (types) render via `<primitive object={clonedScene}>` inside PieceMesh. Click event handlers were on a parent `<group>`, but R3F event propagation from primitive's internal meshes (which lack React fibers) to parent React components was unreliable. Fix: thread click handlers directly to the `<primitive>` element via props through the full component chain.
