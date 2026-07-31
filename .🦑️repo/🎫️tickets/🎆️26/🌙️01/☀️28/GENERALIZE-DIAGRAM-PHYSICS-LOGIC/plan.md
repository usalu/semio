# Plan - Generalize Diagram Physics Logic

Refactor the D3 force-directed simulation from a bespoke implementation in `Kit.tsx` to a shared, reusable `Diagram` component in `elements.tsx`.

## Steps

1. **Generalize `Diagram` Component in `elements.tsx`**:
   - Integrate `forceSimulation` logic into `DiagramInner`.
   - Expose `forceConfig` prop to allow customization of forces (center, collide, manyBody, link, x, y).
   - Implement automatic "reheat" on node drag to maintain physically responsive transitions.
   - Support `selectionMode` (via `@xyflow/react`) and `onSelectionChange` props.
   - Expose `Position`, `Background`, `BaseEdge`, `Handle`, `ReactFlow`, `ReactFlowProvider`, and `applyNodeChanges` for use in app-specific diagram implementations.

2. **Refactor `Kit.tsx`**:
   - Remove `simulationRef`, `forceSimulation` setup, and manual `useEffect` simulation logic.
   - Replace `KitDiagramInner` return value with the new `<Diagram />` component.
   - Pass `nodeTypes`, `edgeTypes`, `forceConfig`, and interaction handlers (`onSelectionChange`, `onNodeMouseEnter`, etc.).
   - Clean up redundant imports and simulation-related types.

3. **Fix Related Issues in `Kit.tsx`**:
   - Restore `artifactKinds` constant used for URL search parameters.
   - Fix `generateUniqueName` type errors where a `string | undefined` was passed to a `string` parameter.
   - Fix `icon` type in `buildKitDiagramData` to allow both `string` (icons) and `React.JSX.Element` (from `getFileIcon`).

4. **Verification**:
   - Ensure the Kit editor compiles without errors.
   - Verify that nodes now have physics-based layout and respond to drags correctly.

5. **Documentation**:
   - Update `AGENTS.md` and `README.md` to reflect the new shared `Diagram` mechanism.
