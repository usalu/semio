---
goal: SKETCHPAD/DETAIL-PANEL
---

# Ticket

## Summary

Fixed a `PiecesSectionForm` runtime crash triggered when selection transitions changed render branches: conditional `useLabel(...)` hook invocations caused React hook-order mismatch (`Rendered fewer hooks than expected`).
## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Replaced conditional `useLabel(...)` calls in `PiecesSectionForm` with precomputed `t(...)` string values.
  - Removed temporary `[DEBUG] PiecesSectionForm ...` console logs from parent-connection lookup.
- Updated `semio/js/sketchpad.test.ts`:
  - Added regression assertions in Design flow for `Rendered fewer hooks than expected` and `<PiecesSectionForm>` console errors.

## Log
- Investigated crash report stack for `PiecesSectionForm` and mapped selection-flow branch changes to hook call order.
- Refactored hook-unsafe render-label calls in `PiecesSectionForm`.
- Executed Playwright Design flow to validate regression behavior.

## Todos
- [x] Identify crash root cause in `PiecesSectionForm`
- [x] Refactor render branch labels to avoid conditional hook execution
- [x] Remove temporary debug logs in crash path
- [x] Extend existing Design e2e assertions for hook-order runtime errors
- [x] Run Design e2e command
- [ ] Get full green Design e2e completion (current run times out in existing workbench expansion loop at `sketchpad.test.ts:2139`)

## Plan
1. Stabilize hook order in `PiecesSectionForm`.
2. Add explicit regression assertions for hook-order runtime errors.
3. Re-run Design e2e flow and capture outcomes.

## Research Findings

### 1. ConnectionScopeProvider (Sketchpad.tsx:4699)

Simple React context provider that wraps children with `{guid: props.guid}`:

```tsx
export const ConnectionScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(ConnectionScopeContext.Provider, { value }, props.children as any);
};
```

Creates a new `{guid}` object on every render (no `useMemo`), but this is fine since `useConnectionStore` only reads `connectionScope?.guid` (a string), not referential identity.

### 2. useConnection() (Sketchpad.tsx:4725)

```tsx
export function useConnection<T>(selector?, id?, deep?): T | Connection | null {
  return useSync<Connection, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ?? identitySelector);
}
```

Delegates to `useConnectionStore()` which:
1. Gets `designStore` from `useDesignStore()` context
2. Gets `connectionScope` from `useConnectionScope()` context
3. Resolves `connectionGuid = connectionScope?.guid ?? guid`
4. Calls `designStore.connection(connectionGuid)` which looks up from `this.connections` Map
5. **THROWS** if not found: `throw new Error("Connection store not found for connection ${connectionGuid}")`

Then `useSync()` uses `useSyncExternalStore()` with the `ConnectionStore`'s `onChanged` and `snapshot()` methods.

### 3. Connection Hooks (useConnectionGap, etc.) (Sketchpad.tsx:10736-10894)

All follow the same pattern:
```tsx
export function useConnectionGap(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback((value: number) => {
    if (connectionScope) commands.updateConnection("...", connectionScope.guid, { gap: value });
  }, [connectionScope, commands]);
  return conditionalHookResult(!!connectionScope && !!connection, connection?.gap ?? 0, setter);
}
```

Key observations:
- They read `connectionScope` from context (set by ConnectionScopeProvider)
- They call `useConnection()` which resolves via that same scope
- The setter uses `commands.updateConnection()` with the connection guid
- `conditionalHookResult` returns `[value, setter?, canSet]` - setter is only non-null when both scope and connection exist

### 4. parentConnection Lookup (Design.tsx:5274-5291)

```tsx
let parentConnection: Connection | null = null;
if (isSingle && piece) {
  const pieceId = getPieceId(piece);
  const pieceMeta = metadata.get(pieceId);
  if (pieceMeta?.parentPieceId) {
    parentConnection = allConnections.find((connection) =>
      (connection.connected.piece.guid === pieceId && connection.connecting.piece.guid === pieceMeta.parentPieceId) ||
      (connection.connecting.piece.guid === pieceId && connection.connected.piece.guid === pieceMeta.parentPieceId)
    ) || null;
  }
}
```

### 5. parentConnection Rendering (Design.tsx:5638-5645)

```tsx
{!hasNoValidPieces && parentConnection && (
  <TreeItem id="semio.sketchpad.app.design.panel.details.parentConnection">
    <ConnectionScopeProvider guid={parentConnection.guid}>
      <SingleConnectionInfo />
      <SingleConnectionFields />
    </ConnectionScopeProvider>
  </TreeItem>
)}
```

### 6. Could flattenDesign Throw Silently?

`piecesMetadata()` (semio.ts:8170) calls `flattenDesign(kit, designGuid)` which:
- Throws if design not found in kit
- Uses BFS traversal to compute planes and set `semio.parentPieceId` attribute
- During BFS, missing connectors cause `console.error` + skip (NOT throw)
- Missing parent planes cause `console.error` + skip (NOT throw)

The `piecesMetadata` function is wrapped in `usePiecesMetadataMap` with a `useDerived()` call. If `flattenDesign` throws, `useDerived` would likely propagate the exception, which would crash the component. But if the design exists and pieces exist, `flattenDesign` should succeed without throwing - it gracefully handles missing connectors by skipping.

**Critical observation**: `flattenDesign` only sets `semio.parentPieceId` on **child** pieces (pieces discovered via BFS from a root). The **root** piece (first piece with a plane, or first piece in the component) does NOT get a `semio.parentPieceId` attribute. This is correct since root pieces have no parent.

### 7. Potential Issues

**Issue A: ConnectionStore must exist in DesignStore**

When `parentConnection` is found from `allConnections` (which comes from `useConnections()` → `designStore.snapshotConnections()`), the connection is guaranteed to exist in the `DesignStore`'s `connections` Map because `useConnections()` snapshots from those same stores. So `ConnectionScopeProvider guid={parentConnection.guid}` → `useConnectionStore()` → `designStore.connection(guid)` will succeed.

**Verdict: No issue. The connection store is guaranteed to exist.**

**Issue B: Hook Dependency Staleness**

The connection hooks (`useConnectionGap`, etc.) each independently call `useConnection()` and `useConnectionScope()`. Since these are all within the same `ConnectionScopeProvider`, they all resolve the same connection guid. The `useSync` hook uses `useSyncExternalStore` which subscribes to `ConnectionStore.onChanged`, so changes are reactive.

**Verdict: No staleness issue. All hooks subscribe independently to the same store.**

**Issue C: Parent Connection Found but Wrong Type**

The `allConnections` from `useConnections()` returns full `Connection` objects (snapshots of `ConnectionStore`). The `parentConnection` found by `Array.find()` is a snapshot object, not a store. However, only `parentConnection.guid` is passed to `ConnectionScopeProvider`, which then re-resolves the live store. So the snapshot data is only used for the guid lookup.

**Verdict: No issue. Store re-resolution is correct.**

**Issue D: Could metadata.get(pieceId) return undefined?**

If the piece is a root piece (no parent in the BFS tree), `pieceMeta.parentPieceId` will be `null` (since `findAttributeValue(p, "semio.parentPieceId", null)` defaults to null). The code correctly checks `if (pieceMeta?.parentPieceId)` which is falsy for null.

For child pieces, `parentPieceId` will be the parent's guid, and the connection search should find it.

**Verdict: Correct behavior.**

**Issue E: Connection search may fail if connection sides are structured differently**

The search compares `connection.connected.piece.guid` and `connection.connecting.piece.guid`. These are the raw piece guids from the Yjs-backed SideStore snapshots. The `parentPieceId` from metadata is also a raw piece guid. These should match.

**Verdict: No issue for same-design connections.**

### 8. Conclusion

The ConnectionScopeProvider + useConnection + connection hooks pipeline is **correctly implemented** for parentConnection rendering. The data flow is:

1. `usePiecesMetadataMap()` → `piecesMetadata(kit, designGuid)` → `flattenDesign()` → BFS sets `semio.parentPieceId` attribute
2. `metadata.get(pieceId)?.parentPieceId` gets the parent piece guid
3. `allConnections.find(...)` finds the connection linking child to parent
4. `<ConnectionScopeProvider guid={parentConnection.guid}>` provides context
5. `SingleConnectionInfo` reads connection data via `useConnection()` → `useSync(connectionStore)` → `connectionStore.snapshot()`
6. `SingleConnectionFields` reads/writes via `useConnectionGap()` etc. → same store + `commands.updateConnection()`

**All hooks would work correctly for a parentConnection from the same design.** The store lookup, reactivity, and setters are all properly wired.

**The only potential failure modes are:**
- `piecesMetadata` throws if the design doesn't exist in the kit (would crash the component)
- Missing connectors during flatten → piece gets no plane/parentPieceId, so it won't appear as a child (graceful degradation)
- If a piece has a parentPieceId but the corresponding connection was deleted → `parentConnection` stays null → TreeItem not rendered (correct)
