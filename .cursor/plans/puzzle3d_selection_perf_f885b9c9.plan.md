---
name: Puzzle3d Selection Perf
overview: "Eliminate the O(N^2) selection hot paths in Puzzle 3D so Ctrl+A (select all) and large multi-selections are fast: replace the monolithic whole-snapshot selection subscription with fine-grained O(1) per-id membership, and aggregate the declarative inspector instead of building one UI section per selected row."
todos:
  - id: ticket
    content: Gather repo info via repo MCP search, read repo://goals, open a ticket associated to the most appropriate goal
    status: completed
  - id: store
    content: Refactor createSelectionSnapshotStore to derived Sets + per-id diff-notify; add useObjectSelected/useVortexSelected/useAttractionSelected hooks
    status: completed
  - id: scene
    content: Replace whole-snapshot selection consumption in Objects/ObjectItem/Vortex/CableBatch with O(1) per-id hooks; remove selectedVortexFullIds plumbing
    status: completed
  - id: inspector
    content: Aggregate buildPuzzle3dPlayInspectorBody with Map indexes (count + uniform/Mixed) instead of per-row sections
    status: completed
  - id: host
    content: Memoize host-derived kindCatalogs/kindCompatibility/blockedVortexFullIds in framework renderer keyed on fixture
    status: completed
  - id: verify
    content: Extend existing vitest blocks; run tests; runtime-verify Ctrl+A timing with temporary [DEBUG] logs then remove; close ticket
    status: in_progress
isProject: false
---

# Puzzle 3D Selection Performance Refactor

## Problem (verified)
On Ctrl+A every object, vortex, cable, and the inspector re-derive selection by linearly scanning the full `SelectionSnapshot`, which is O(N^2) plus a full scene/UI rebuild.

- `objectMatchesSelection(id, liveSelection)` runs per object and loops over **all** `vortexIds` (parsing each): [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) lines 2509-2522, called at 2545 and 3963.
- Each `Vortex` does `liveSelection.vortexIds.includes(fullId)`: [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) line 4239.
- Each `CableBatch` builds `new Set(liveSelection.attractionIds)` per render: line 4411.
- `useLiveSelection()` subscribes every node to the **whole** snapshot, so any selection change re-renders/recomputes every scene node: `createSelectionSnapshotStore` lines 484-505, `useLiveSelection` 517-523.
- Declarative inspector builds one section (with `objects.find` + `vortices.find`) per selected vortex/attraction -> O(V^2) and a huge UI tree: `buildPuzzle3dPlayInspectorBody` [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) lines 1363-1489.
- Host re-parses meta and recomputes blocked vortices on every snapshot (incl. selection-only): [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) lines 1178-1181.

## Approach

### 1. Fine-grained O(1) selection store (puzzle/3d/react/index.tsx)
Refactor `createSelectionSnapshotStore` (lines 484-505) to keep the snapshot AND derived `Set`s: `objectIdSet`, `vortexIdSet`, `vortexOwnerObjectIdSet` (parsed once), `attractionIdSet`. Add membership getters (`isObjectSelected`, `isVortexSelected`, `isAttractionSelected`, `getPrimaryObjectId`) and a **per-id subscription** that, on `setSnapshot`, diffs old vs new sets and notifies only listeners whose membership changed (mirrors the existing `subscribeObject` pattern in the object store at lines 2438-2453).

Add hooks: `useObjectSelected(objectId)`, `useVortexSelected(fullId)`, `useAttractionSelected(id)` using `useSyncExternalStore` with O(1) getSnapshot. Keep `useLiveSelection` only for the few places needing the full snapshot.

### 2. Replace whole-snapshot consumption in the scene
- `Objects` (2532-2553): stop computing `selected` via `objectMatchesSelection`; pass only `objectId` and let `ObjectItemById`/`ObjectItem` read `useObjectSelected(id)` (covers direct + vortex-owner selection via `vortexOwnerObjectIdSet`). Drop the `selection`/`selectedVortexFullIds` prop threading.
- `ObjectItem` (3953-4088): replace line 3963 scan with `useObjectSelected(props.id)`; replace `primarySelectionObjectId(liveSelection)` (3965) with store getter.
- `Vortex` (~4191, 4239): replace `liveSelection.vortexIds.includes(fullId)` with `useVortexSelected(fullId)`.
- `CableBatch` (4410-4411): replace `new Set(liveSelection.attractionIds)` with `useAttractionSelected(id)` per line, or the store's stable set.
- Remove now-dead `selectedVortexFullIds` plumbing through `PlayCanvas` (6809, 6894) and the host `new Set(...)` at framework renderer line 1181.

### 3. Aggregate the declarative inspector (puzzle/3d/play/index.ts)
Rework `buildPuzzle3dPlayInspectorBody` (1363-1489): build `Map` indexes for objects/vortices once; when many rows are selected, render aggregated sections (count + uniform/"Mixed" fields) like the existing object branch (1293-1361) instead of one section per row. This removes the O(V^2) `find` loops and the giant UI tree on Ctrl+A.

### 4. Host memoization (framework/product/playground/renderer/react/index.tsx)
Wrap `parseKindCompatibility`/`parseKindCatalogs`/`blockedVortexFullIdsFromAttractions` (1178-1180) in `useMemo` keyed on `snap.fixture` so selection-only snapshot bumps don't re-parse.

## Validation
- Extend existing in-file vitest blocks (no new test files): store membership/diff-notify tests near `createSelectionSnapshotStore` tests (index.tsx ~7213), aggregated-inspector test in play tests (~1759). Run `bun nx run @puzzle/3d/play:test` and the react package test.
- Runtime: run the 3D play dev server, add a temporary `[DEBUG]` `performance.now()` around the Ctrl+A path, confirm select-all drops from ~5s to sub-second, then remove the debug logs.

## Constraints
Edit existing files only; keep code in the existing regions; no backwards-compat shims; work inside a repo MCP ticket associated to the most appropriate goal.