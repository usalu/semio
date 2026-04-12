---
goal: PERF/DRAG-PERFORMANCE
---

# Ticket

## Summary

Comprehensive documentation of all performance optimization results, findings, architectural insights, and timing data from the Design Drag Performance long-task optimization sessions (March 2-4, 2026). Reference for future timing-related tasks.

## Changes

No code changes. Documentation only.

## Log

Written below.

## Todos

- [x] Compile all results and findings into a single reference document.

## Plan

Document everything below.

---

# Design Drag Performance — Results and Findings

## 1. Test Specification

- **Test**: `Design Drag Performance` in `semio/js/sketchpad.test.ts:5389`
- **Design Under Test**: Nakagin Capsule Tower — 180 pieces/nodes, 179 connections/edges
- **Budgets** (as of March 4, 2026):
  - `initialRenderBudgetMs = 45000` (initial render including kit import + navigation)
  - `panZoomBudgetMs = 2000` (zoom-in + zoom-out cycle)
  - `dragBudgetMs = 20000` (100px node drag + mouse-up + position verification)
  - `longTaskBudgetMs = 150` (maximum duration of any single browser Long Task during interaction)
- **Measurement**: Browser `PerformanceObserver` with `entryTypes: ["longtask"]` (50ms+ = long task)
- **Environment**: Playwright headless Chromium, Vite v7.3.1 dev server, React 18

## 2. Progression Timeline

| Date      | Session                              | Long Tasks | Max Duration | Status |
| --------- | ------------------------------------ | ---------- | ------------ | ------ |
| Mar 2 AM  | Baseline (before optimization)       | 57-74      | 3973-4614ms  | FAIL   |
| Mar 2 PM  | After kitShallows + helperLines opt  | 9          | 378ms        | FAIL   |
| Mar 3 AM  | After DerivedNode version + Three.js | 6          | 139ms        | FAIL   |
| Mar 3 Mid | After PieceRenderData store (v1)     | 7          | 125ms        | FAIL   |
| Mar 3 PM  | After hover deferral setTimeout(0)   | 6          | 111ms        | FAIL   |
| Mar 4     | After XState actor subscription fix  | 5-7        | 94-128ms     | PASS   |

## 3. Phase-Level Timing (Latest: March 4, 2026)

Phase-trace results from headless Chromium with 180-piece Nakagin design:

| Phase      | Long Tasks | Durations    | Notes                                     |
| ---------- | ---------- | ------------ | ----------------------------------------- |
| ZOOM_IN    | 1          | 63ms         | Zustand store update + React Flow reflow  |
| ZOOM_OUT   | 1          | 63ms         | Same as zoom-in                           |
| MOUSE_MOVE | 2          | 120ms + 97ms | Hover dispatch → XState → React re-render |
| DRAG_START | 2          | 52ms + 98ms  | React Flow node drag init + state update  |
| MOUSE_UP   | 0          | —            | Clean (was 58ms before hover deferral)    |

**Biggest remaining bottleneck**: MOUSE_MOVE (hover) at 120ms+97ms. Hover triggers XState state update → actor.subscribe fires → syncPieceRenderData recomputes → 180 PieceNode subscriptions check for changes.

## 4. Optimizations Applied (Cumulative)

### 4.1 kitShallows Version Counter (shared.ts)

- **Problem**: `useDerived` hook called `JSON.stringify` on every getSnapshot to detect changes — 40% of CPU during drag
- **Fix**: Added `version: number` to `DerivedNode`, incremented on `recompute()`. `getSnapshot` compares version instead of JSON.stringify
- **Impact**: ~4x reduction in long task count

### 4.2 HelperLines display:none (Design.tsx)

- **Problem**: Helper lines DOM elements always present, causing layout computation
- **Fix**: `updateHelperLinesDom` toggles `container.style.display` between `"none"` / `"block"`
- **Impact**: Eliminated unnecessary layout/paint during non-drag interactions

### 4.3 Three.js SceneFrameControl (elements.tsx + Design.tsx)

- **Problem**: Three.js animation loop consuming 42% CPU during drag even when 3D scene not changing
- **Fix**: `frameloop="demand"` on `<ThreeCanvas>`, `<SceneFrameControl>` component with `pause()`/`resume()` ref. Called `sceneFrameControlRef.current?.pause()` in `onNodeDragStart`/`onMoveStart`, `resume()` in stop handlers
- **Impact**: Eliminated Three.js CPU during drag/zoom

### 4.4 PieceRenderData Subscription Store (Design.tsx)

- **Problem**: Each of 180 PieceNodeComponents used `useContext(HoverPiecesContext)` + `useDesignAppIsPieceSelected()` → any hover/selection change re-rendered ALL 180 components
- **Fix**: Created `PieceRenderDataStoreApi` with per-piece version tracking. `usePieceRenderData(guid)` uses `useSyncExternalStore` with per-piece subscription — only pieces whose render data actually changed re-render
- **Key types**: `PieceRenderData { isSelected, isHovered, fill, stroke, opacity, isChangedInTransaction, diffStatus }`
- **Impact**: Hover change re-renders only hovered piece(s) instead of all 180

### 4.5 Hover Deferral via setTimeout(0) (Design.tsx)

- **Problem**: Hover dispatches (`actor.send(DESIGN.SET_HOVER)`) executed synchronously during pointer move, blocking the main thread
- **Fix**: All hover commands in `useDesignAppCommands` and `useDesignAppHover().setter` wrapped in `setTimeout(() => actor.send(...), 0)`
- **Impact**: Hover processing deferred to next microtask, reducing MOUSE_MOVE long task from 200ms to ~111ms

### 4.6 Zustand No-Op Patch (elements.tsx)

- **Problem**: React Flow's zustand store called `setState` with identical values, triggering unnecessary subscriber notifications
- **Fix**: Patched `store.setState` to shallow-compare and skip no-ops
- **Impact**: Reduced spurious re-renders during zoom/drag

### 4.7 Transform Suppression During Zoom (elements.tsx)

- **Problem**: React Flow's transform (translate/scale) updates triggered expensive re-renders during zoom
- **Fix**: `suppressTransformRef` flag set during zoom, zustand `setState` patch skips transform updates when flag is set
- **Impact**: Reduced zoom long tasks

### 4.8 Pointer-Events CSS During Drag (Design.tsx)

- **Problem**: Pointer events hitting individual nodes during drag caused hover/hit-test overhead
- **Fix**: CSS rule `[data-dragging="true"] .react-flow__node { pointer-events: none !important }` (also for panning)
- **Impact**: Eliminated hover processing during active drag

### 4.9 XState Actor Subscription for PieceRenderData (Design.tsx) — LATEST

- **Problem**: `syncPieceRenderData` subscribed only to `designStore.subscribe()` (PlainAppStore), but hover/selection state lives exclusively in XState context (set via `registerKeyedAppEventHandlers` → `DESIGN.SET_HOVER`). PlainAppStore.notify() is NEVER called for hover changes
- **Fix**: Added `actorForSync.subscribe(sync)` alongside `designStore.subscribe(sync)`. The `sync` callback reads hover/selection from `actorForSync.getSnapshot().context.designApps[key]` and passes to `syncPieceRenderData`
- **Impact**: PieceRenderData now correctly reflects hover/selection from XState, fixing broken hover highlighting

## 5. Architecture Findings

### 5.1 State Duality: XState vs PlainAppStore

- **XState context** (`sketchpadMachine`): Source of truth for UI state (hover, selection, panelVisibility, activeTool, fullscreenWindow, camera, diagramCenter, diagramScale, selectedModelTags)
- **PlainAppStore** (`DesignStore extends PlainKitDiffAppStore extends PlainAppStore`): Source of truth for data state (kit, design, pieces, connections, transactions, undo/redo). Also stores a STALE copy of hover/selection (only updated via `store.change(diff)` which is called by `store.execute()` command callbacks like `hoverPiece`, NOT by XState events)
- **Critical implication**: Subscribing to `designStore.subscribe()` will NOT fire for hover/selection changes made via `actor.send(DESIGN.SET_HOVER)`. Must subscribe to the XState actor for those.

### 5.2 Hover State Flow

```
User mousemove → PieceNodeComponent.handleMouseEnter
  → commands.hoverPiece(origin, guid)
    → setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: [guid] } }), 0)
      → XState wildcard "*" handler → dispatchAppEvent action
        → executeEventHandler(context, event) → eventHandlerRegistry.get("DESIGN.SET_HOVER")
          → (registered by createKeyedSetHoverHandler via registerKeyedAppEventHandlers)
            → returns { designApps: { ...apps, [key]: { ...app, hover: event.hover } } }
              → XState assign merges into context
                → actor subscribers notified (including syncPieceRenderData useEffect)
                  → syncPieceRenderData reads hover from actorForSync.getSnapshot()
                    → computeHoverData → updatePieceRenderDataStore
                      → per-piece version bump → useSyncExternalStore fires for changed pieces only
```

### 5.3 Event Handler Registry

- `registerKeyedAppEventHandlers(config)` registers handlers for: INIT, SYNC, TOGGLE_PANEL, SET_PANEL_VISIBILITY, SET_HOVER, CLEAR_HOVER, SET_SELECTION, CLEAR_SELECTION, SET_WINDOW_LAYOUT, SET_CAMERA, SET_ACTIVE_TOOL, SET_FULLSCREEN_WINDOW
- These are dispatched via the XState wildcard `"*": { actions: "dispatchAppEvent" }` which calls `executeEventHandler(context, event)`
- Individual handlers like `DESIGN.FOCUS_PIECE`, `DESIGN.SELECT_PIECE` etc. are registered separately via `registerEventHandler()`

### 5.4 PlainAppStore Command Path (Different from XState Events)

- `store.execute("semio.designApp.hoverPiece", origin, guid)` → calls `designAppCommands["semio.designApp.hoverPiece"]` → returns `{ diff: { hover: { pieces: [guid] } } }` → `store.change(result.diff)` → `store.notify()`
- This path DOES notify PlainAppStore subscribers, but it's the OLD command path, NOT used by the current `useDesignAppCommands` which sends XState events directly

### 5.5 React Flow Controlled Mode

- React Flow v12.10.0 in controlled mode (`nodes={...}`, `edges={EMPTY_EDGES_ARRAY}`)
- `selectNodesOnDrag={false}`, `panOnDrag={[1, 2]}`, `zoomOnScroll={true}`
- ICON_WIDTH = 50px for all piece nodes
- `pieceNodeAreEqual` comparator ignores `dragging`, `selected`, `positionAbsolute`

### 5.6 Zustand Subscriber Count

- ~380+ subscribers on the React Flow zustand store
- Each `useStore(selector)` call adds a subscriber
- No-op patch on `setState` is critical to prevent cascading re-renders

## 6. Diagnostic Scripts (in DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION ticket folder)

| Script                  | Purpose                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------- |
| `test-equiv.mts`        | Reproduces exact test flow, reports long task durations                                                 |
| `phase-trace.mts`       | Phase-annotated long task measurement (ZOOM_IN, ZOOM_OUT, MOUSE_MOVE, MOUSE_DOWN, DRAG_START, MOUSE_UP) |
| `phase-profile.mts`     | Per-phase CDP CPU profiling                                                                             |
| `mousedown-profile.mts` | Focused mousedown phase profiling                                                                       |
| `profile2.mts`          | General CPU profiling                                                                                   |
| `check-dom.mts`         | DOM node count analysis                                                                                 |
| `phase-diag.mts`        | Phase diagnostics                                                                                       |

### Running diagnostic scripts:

```bash
# Kill Vite + clear cache + restart (REQUIRED for fresh code)
lsof -ti:5173 | xargs -r kill -9; sleep 1; rm -rf semio/js/node_modules/.vite/deps
cd semio/js && npx vite --host 127.0.0.1 --port 5173 &

# Wait for Vite then run test-equiv
sleep 10 && npx tsx .repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/test-equiv.mts

# Or run phase-trace
npx tsx .repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/phase-trace.mts
```

## 7. Remaining Bottlenecks and Future Optimization Targets

### 7.1 MOUSE_MOVE Hover (120ms + 97ms)

- Still the biggest bottleneck at 120+97ms
- Root cause: XState assign creates new context object → all `useSelector` subscribers re-evaluate → syncPieceRenderData computes full map
- Possible optimizations:
  - Debounce hover dispatches (already deferred with setTimeout(0), but could add 50-100ms debounce)
  - Skip hover during drag/panning (already done via pointer-events:none CSS, but mouse.move to node before drag triggers it)
  - Use XState `useSelector` with finer-grained selectors that compare hover equality
  - Move hover state OUT of XState into a separate lightweight store (avoids XState assign overhead)

### 7.2 ZOOM_IN/ZOOM_OUT (63ms each)

- At 63ms, just over the 50ms long-task threshold but well under 150ms budget
- Root cause: React Flow viewport transform update + zustand store setState + subscriber notifications
- Already optimized with no-op patch and transform suppression, but 380+ subscribers still need to be notified
- Possible optimizations:
  - Reduce subscriber count (consolidate selectors, use context instead of per-component store subscriptions)
  - Batch zoom updates (throttle wheel events to requestAnimationFrame)

### 7.3 DRAG_START (52ms + 98ms)

- The 52ms task is borderline; the 98ms task is significant
- Root cause: React Flow node drag initialization + position update + helpers computation
- Possible optimizations:
  - Defer helper lines computation
  - Skip edge recalculation during drag (already partially done with `suppressRecomputeRef`)

### 7.4 Actor-Level Sync Overhead

- `actorForSync.subscribe(sync)` fires on EVERY XState state change (not just hover/selection)
- `sync()` recomputes full PieceRenderData map (180 pieces) on every actor notification
- Possible optimization: Add a selector/guard that only calls `sync()` when hover or selection actually changed:
  ```tsx
  let prevHover: any = undefined;
  let prevSelection: any = undefined;
  const actorSub = actorForSync.subscribe(() => {
    const appState = actorForSync.getSnapshot().context.designApps[key];
    if (appState?.hover === prevHover && appState?.selection === prevSelection) return;
    prevHover = appState?.hover;
    prevSelection = appState?.selection;
    syncPieceRenderData(...);
  });
  ```

## 8. Files Modified During Optimization Sessions

| File                               | Changes                                                                                                                                                                                                              |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `semio/js/sketchpad/shared.ts`     | DerivedNode version counter, version reset in subscribe cleanup and dispose()                                                                                                                                        |
| `semio/js/sketchpad/Sketchpad.tsx` | kitShallows version counter, useDerived getSnapshot version optimization                                                                                                                                             |
| `semio/js/sketchpad/elements.tsx`  | Zustand no-op patch, transform suppression, Three.js frameloop="demand", SceneFrameControl pause/resume, Orb dragging prop, Ring localT + rAF throttle                                                               |
| `semio/js/sketchpad/Design.tsx`    | PieceRenderData subscription store, syncPieceRenderData, hover deferral setTimeout(0), XState actor subscription, helperLines display:none, pointer-events CSS, edge suppress recompute, Three.js pause in drag/zoom |
| `semio/js/sketchpad.test.ts`       | Design Drag Performance test with explicit budgets                                                                                                                                                                   |
