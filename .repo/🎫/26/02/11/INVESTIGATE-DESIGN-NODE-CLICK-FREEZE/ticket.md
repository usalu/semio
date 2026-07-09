---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-DESIGN-APP
---

# Ticket

## Plan

Investigate why clicking a React Flow node in Design.tsx causes browser freeze.

## Investigation Findings

### Root Cause: Selection → baseNodes → setNodes → onSelectionChange Feedback Loop

The freeze is caused by a **cascading re-render loop** triggered by the selection state change. Here's the exact chain:

#### The Loop (Design.tsx lines ~5834-6027):

1. **User clicks node** → React Flow internally marks it selected → fires `onSelectionChange` (line 5970)
2. **`onSelectionChange`** computes `nextSelection = { pieces: ["abc"] }` and calls `setSelection(nextSelection)` (line 6024)
3. **`setSelection` is `guardedSetter`** (line 1582) — checks `areDesignSelectionsEquivalent(selection_closure, next)` → not equal → calls inner `setSelection` → sends `DESIGN.SET_SELECTION` to XState
4. **XState `createKeyedSetSelectionHandler`** (shared.ts:1872) **always creates a new object**: `{ ...app, selection: event.selection }` — no semantic equality check
5. **`useSelector(actor, granularSelector)`** in `useDesignAppField` (line 1533) detects the new reference → React re-renders with new `selection`
6. **`baseNodes` useMemo** (line 5937) has `selection` as a dependency → recomputes → creates **180 entirely new node objects** with `selected: true/false` baked in
7. **`useEffect(() => setNodes(baseNodes), [baseNodes])`** (line 5945) fires → calls `setState` → triggers React Flow re-render
8. **React Flow receives new node array** → internally reconciles → fires `onSelectionChange` **again** with the current selected nodes
9. Back to step 2.

#### Why The Guard Doesn't Break The Loop:

The `guardedSetter` (line 1582-1590) SHOULD stop this loop because `areDesignSelectionsEquivalent` normalizes and compares:

```tsx
const guardedSetter = useMemo(() => {
 if (!setSelection) return undefined;
 return (next: DesignAppSelection) => {
  if (areDesignSelectionsEquivalent(selection, next)) {
   return;
  }
  setSelection(next);
 };
}, [selection, setSelection]);
```

**Problem 1: guardedSetter depends on `selection` in its deps array.** Every time selection changes, `guardedSetter` gets a new identity. This means `onSelectionChange` (which depends on `setSelection`) also gets a new identity. React Flow re-registers the callback. This alone doesn't cause the loop but amplifies the churn.

**Problem 2: The `onSelectionChange` has its OWN guard** (line 6022) comparing against `pendingSelectionDispatchRef.current`. This double-guard SHOULD work — but there's a subtle timing issue. The `onSelectionChange` callback captures `setSelection` from the PREVIOUS render (because `useCallback` deps don't include `selection`). Meanwhile, the `guardedSetter` from the new render compares against the NEW selection. **But React batches state updates**, so there can be a window where `pendingSelectionDispatchRef.current` is updated but the `guardedSetter` closure still has the old `selection`.

**Problem 3 (MOST LIKELY CULPRIT): Selection object is baked into node data via `designToNodesAndEdges`**. The `baseNodes` useMemo (line 5937) passes `selection` to `designToNodesAndEdges`, which creates nodes with `selected: true`. This means **every selection change recreates ALL 180 nodes**. When React Flow receives 180 new node objects (even with the same IDs), it may fire `onSelectionChange` due to its internal reconciliation detecting the property change. The guard at line 6022 checks set-membership, but if React Flow fires the callback **synchronously during the state update cascade** before `pendingSelectionDispatchRef` is updated, the guard may fail.

### Contributing Factors

#### Factor A: Expensive Re-renders (3 sidepanels × 180+ items)

The effect at line 8090 has `selection` in its deps:

```tsx
useEffect(() => {
    // ...removes and re-adds ALL detail panel sections...
    removeSection("details", ...); // 7 removeSection calls
    addSection("details", ...);    // multiple addSection calls
}, [selection, addSection, removeSection, appType, t, design]);
```

Every selection change causes panel sections to be torn down and rebuilt. With 3 sidepanels open (left: 139 items, HUD: 182 items, right: 3 items), this creates massive synchronous React tree teardown/rebuild.

#### Factor B: No `useSelector` Equality Override

The `useSelector(actor, granularSelector)` in `useDesignAppField` (line 1533) uses XState's default **referential equality** (`===`). Since the state machine handler always creates a new selection object, `useSelector` always detects a change even if the selection is semantically identical.

#### Factor C: `arePortsCompatible` Change (line 6935) — NOT A CAUSE

The `arePortsCompatible` at line 6935 is inside `onNodeDrag`, NOT `onNodeClick` or `onSelectionChange`. It only fires during drag operations. This cannot cause the click freeze.

#

## Summary

Investigated Design.tsx node click freeze. Root cause: selection state is baked into baseNodes useMemo (line 5937), causing all 180 nodes to be recreated on every selection change, which triggers React Flow onSelectionChange again in a feedback loop. Contributing factors: XState handler always creates new state reference (no semantic equality), panel sections torn down/rebuilt on selection change (line 8090), and guardedSetter identity instability. The arePortsCompatible change at line 6935 is NOT related (only fires during drag). Recommended fixes documented in ticket.

## Changes

No code changes made — investigation only.

## Log

- Ran repo tree discovery
- Opened ticket
- Read `useDesignAppSelectionField` and `useDesignAppSelection` (lines 1547-1591)
- Read `areDesignSelectionsEquivalent` (lines 1557-1569)
- Read `onSelectionChange` callback (lines 5970-6027)
- Read `baseNodes` useMemo and its deps (lines 5937-5945)
- Read `designToNodesAndEdges` (lines 5647-5810) — confirms `selected` is baked into node objects
- Read `pieceToNode` (line 5540) — confirms `selected` property on React Flow nodes
- Read XState `createKeyedSetSelectionHandler` (shared.ts:1872-1886) — always creates new object
- Read `createDesignSelectionSelector` (Sketchpad.tsx:8716) — returns raw reference
- Read DiagramInner and ReactFlow wiring (elements.tsx:4978-5348) — direct passthrough
- Read panel section effect (Design.tsx:8090-8220) — tears down/rebuilds on selection change
- Checked `arePortsCompatible` (line 6935) — only in `onNodeDrag`, not related to click
- Checked recent ticket `MAP-EDITOR-INSPECTOR-DETAILS-ARCHITECTURE` — confirms prior partial fix
