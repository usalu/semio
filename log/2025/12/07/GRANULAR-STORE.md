---
date: "2025-12-07T20:14:43.922Z"
slug: GRANULAR-STORE
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Implement granular store architecture with YPath and DerivedStore
model: claude-opus-4.5
---

# Previously

The current architecture has performance issues:

- `useFlattenDiff`, `usePiecesMetadata` grab entire `types` and `designs` arrays
- Every piece re-renders when any type/design changes
- Multiple hooks calling same expensive computations (flattenDesign, piecesMetadata)
- No caching of derived data between hooks

Existing infrastructure:

- `Store<TState>` base class with Y.js integration
- `createObserver`, `createFieldObserver`, `createFieldsObserver` helpers
- `createNestedArrayItemMembershipObserver` for targeted array item subscriptions
- `useSyncExternalStore` based hooks with field subscription registry

# Plan

Here’s a migration plan that gets you to a **clean, generic “subscribe to any nested field / derived field” architecture**, without worrying about shims or compatibility.

I’ll structure it as phases you can actually implement.

---

## Phase 0 – Target state (what we’re aiming for)

**Target architecture:**

1. **Base Yjs stores** (KitStore, DesignStore, PieceStore, …)
   - Expose a generic **path API**:
     - `onPathChanged(path, subscribe): Unsubscribe`
     - `getPathSnapshot(path): any`

2. **Derived layer** (per root store or per kit/design):
   - A `DerivedStore` that manages `DerivedNode`s:
     - Each `DerivedNode`:
       - Has a key (e.g. `"flatPiecePlane:<designGuid>:<pieceGuid>"`)
       - Has a set of base dependencies (Yjs paths in specific stores)
       - Has a `compute()` function
       - Caches value & only recomputes when dependencies change

3. **React hooks layer**:
   - Generic primitives:
     - `usePath(store, path, selector?)`
     - `useDerived(derivedStore, key, deps, compute, selector?)`

   - Domain hooks are **thin wrappers**:
     - `useFlatPiecePlane(pieceGuid)`
     - `useFlatPiecePlaneXAxisY(pieceGuid)`
     - `useSomethingDeepLikeConnectionGap(connectionGuid)`

   - No heavy logic inside hooks (unlike current `useFlattenDiff`, `usePiecesMetadata`).

Everything subscribes only to _exactly_ what it uses.

---

## Phase 1 – Extract & normalize domain computations

### 1.1. Make domain functions clearly pure

You already have pure-ish stuff like:

- `flattenDesign(kit, designGuid)`
- `applyDesignDiff(design, diff)`
- `piecesMetadata(kit, designGuid)`

Ensure these are:

- **Pure**: no global state, only input → output.
- **Non-React**: no hooks, no store access inside them.

If `piecesMetadata` internally reads from Yjs or stores anywhere, refactor it to take fully materialized `Kit` + `Design` snapshots only (it looks mostly there already).

### 1.2. Define “selector functions” on snapshots

Create a `selectors` module (pure functions on **snapshots**, not stores):

Examples:

- `getFlatDesign(kitSnapshot, designSnapshot)`
- `getPiecesMetadata(kitSnapshot, designGuid)` (essentially a thin wrapper over `piecesMetadata`)
- `getFlatPiecePlane(kitSnapshot, designGuid, pieceGuid)`

These are used later in the derived layer; React + stores never reimplement them.

---

## Phase 2 – Introduce a generic path abstraction on stores

You already have:

- `createFieldObserver`, `createFieldsObserver`
- `createNestedArrayItemMembershipObserver`, `useSyncNestedArrayItemMembership` (very similar to what we want, but special-cased).

### 2.1. Define `YPath` & helpers (non-React)

In a new module, define:

```ts
type YPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

export type YPath = YPathSegment[];
```

Then pure helpers:

- `getValueAtPath(rootYMap, path): any`
- `createPathObserver(rootYMap, path, subscribe): Disposable`

`createPathObserver` basically generalizes your existing `createNestedArrayItemMembershipObserver` and `createFieldObserver` logic: walk through Y.Map/Y.Array, attach observers at each segment, and re-run `getValueAtPath` when relevant changes happen.

### 2.2. Add path API to base Store

Extend your base `Store` (or each concrete store) with:

```ts
class Store<T> {
  // existing:
  // onChanged / onChangedDeep / onFieldChanged / snapshot / yMap ...

  onPathChanged(path: YPath, subscribe: Subscribe): Disposable {
    return createPathObserver(this.yMap, path, subscribe);
  }

  getPathSnapshot(path: YPath): any {
    return getValueAtPath(this.yMap, path);
  }
}
```

This is the **only low-level subscription primitive** you need from Yjs going forward.

---

## Phase 3 – Derived graph layer (generic, no domain yet)

### 3.1. Implement `DerivedNode` and `DerivedStore`

Create a small generic module (no React):

```ts
type BaseDependency = {
  store: Store<any>;
  path: YPath;
};

class DerivedNode<T> {
  private deps: BaseDependency[];
  private compute: () => T;
  private value: T | undefined;
  private subscribers = new Set<() => void>();
  private unsubscribers: Disposable[] = [];

  constructor(deps: BaseDependency[], compute: () => T) {
    this.deps = deps;
    this.compute = compute;
    this.init();
  }

  private init() {
    this.unsubscribers = this.deps.map((d) =>
      d.store.onPathChanged(d.path, (cb) => {
        cb();
        this.recompute();
        return () => {};
      }),
    );
    this.recompute();
  }

  private recompute() {
    const prev = this.value;
    const next = this.compute();
    if (!deepEqual(prev, next)) {
      this.value = next;
      this.subscribers.forEach((cb) => cb());
    }
  }

  snapshot(): T {
    if (this.value === undefined) this.recompute();
    return this.value!;
  }

  subscribe(cb: () => void): () => void {
    this.subscribers.add(cb);
    return () => this.subscribers.delete(cb);
  }
}

class DerivedStore {
  private nodes = new Map<string, DerivedNode<any>>();

  getOrCreate<T>(key: string, deps: BaseDependency[], compute: () => T): DerivedNode<T> {
    if (!this.nodes.has(key)) {
      this.nodes.set(key, new DerivedNode<T>(deps, compute));
    }
    return this.nodes.get(key)! as DerivedNode<T>;
  }
}
```

You can have:

- One `DerivedStore` per kit
- One per design
- Or one global registry – choose what’s simplest.

---

## Phase 4 – React-level primitives

### 4.1. `usePath` hook

Using your existing `useSyncExternalStore` helpers as style reference :

```ts
export function usePath<T>(store: Store<any>, path: YPath, selector: (value: any) => T = (v) => v as T): T {
  return useSyncExternalStore(
    (onChange) =>
      store.onPathChanged(path, (cb) => {
        cb();
        onChange();
        return () => {};
      }),
    () => selector(store.getPathSnapshot(path)),
  );
}
```

This gives you “subscribe to any nested raw field” right away.

### 4.2. `useDerived` hook

```ts
export function useDerived<T, TSelected = T>(derivedStore: DerivedStore, key: string, deps: BaseDependency[], compute: () => T, selector: (value: T) => TSelected = (v) => v as any): TSelected {
  const node = useMemo(
    () => derivedStore.getOrCreate(key, deps, compute),
    [derivedStore, key], // assume deps are stable by construction
  );

  return useSyncExternalStore(
    (onChange) => node.subscribe(onChange),
    () => selector(node.snapshot()),
  );
}
```

Hooks no longer know anything about Yjs details – they just connect React to:

- base paths (`usePath`), or
- derived nodes (`useDerived`).

---

## Phase 5 – Move geometry / flattening into derived nodes

Now we start replacing the overfetching stuff.

Current situation:

- `useFlattenDiff` grabs whole `types` and `designs` via `useKitTypes` / `useKitDesigns`, then runs `flattenDesign`.
- `useFlatDesign` runs `applyDesignDiff` in the hook.
- `usePiecesMetadata` grabs full `types` + `designs` again, and runs `piecesMetadata(kit, designGuid)` for _all_ pieces.
- `useFlatPiece` calls `usePiecesMetadata` and then creates a transient flat piece.

We want:

- Derived nodes per design/piece with **minimal dependencies**.

### 5.1. Create `DesignDerivedStore`

For each `DesignStore`, attach a `DerivedStore`:

```ts
class DesignStore extends Store<DesignShallow | Design> {
  readonly derived = new DerivedStore();
}
```

Expose via context hook:

```ts
export function useDesignDerivedStore(): DerivedStore {
  const store = useDesignStore(identitySelector) as DesignStore;
  return store.derived;
}
```

### 5.2. Define per-piece geometry node builder

Create a pure builder that **declares dependencies** and how to compute:

```ts
function buildFlatPiecePlaneNode(
  derived: DerivedStore,
  kitStore: KitStore,
  designStore: DesignStore,
  designGuid: string,
  pieceGuid: string,
): DerivedNode<Plane> {
  const key = `flatPiecePlane:${designGuid}:${pieceGuid}`;

  // Base dependencies – only what affects this piece’s plane:
  const piecePath: YPath = [
    { kind: "mapKey", key: "pieces" },
    { kind: "arrayItemById", id: pieceGuid, idKey: "guid" },
  ];

  // For types/designs: either 1) look up typeGuid once, then create path, or
  // 2) depend on all types that this design uses.
  // Keep it clean/explicit – up to you how granular you go.
  const typeGuid = /* lookup from design snapshot */;
  const typePath: YPath = [
    { kind: "mapKey", key: "types" },
    { kind: "arrayItemById", id: typeGuid, idKey: "guid" },
  ];

  const deps: BaseDependency[] = [
    { store: designStore, path: piecePath },
    { store: kitStore, path: typePath },
    // other included designs/types if needed
  ];

  return derived.getOrCreate(key, deps, () => {
    const kitSnapshot = kitStore.snapshot();  // full or minimal; your call
    const designSnapshot = designStore.snapshot() as Design;
    return getFlatPiecePlane(kitSnapshot, designSnapshot, pieceGuid);
  });
}
```

Notice:

- The **only** dependency knowledge lives in this builder.
- If you later figure out you can trim dependencies further, you just adjust `deps` here.

### 5.3. Implement new hooks using derived nodes

```ts
export function useFlatPiecePlane(id?: Guid): Plane {
  const pieceScope = usePieceScope();
  const pieceGuid = (id ?? (pieceScope as any)) as string;

  const kitStore = useKitStore(identitySelector) as KitStore;
  const designStore = useDesignStore(identitySelector) as DesignStore;
  const derived = useDesignDerivedStore();

  const plane = useDerived(derived, `flatPiecePlane:${designStore.guid}:${pieceGuid}`, [], () => buildFlatPiecePlaneNode(derived, kitStore, designStore, designStore.guid, pieceGuid).snapshot());

  // default if undefined
  return (
    plane ?? {
      origin: { x: 0, y: 0, z: 0 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    }
  );
}

export function useFlatPiecePlaneXAxisY(id?: Guid): number {
  const plane = useFlatPiecePlane(id);
  return plane.xAxis.y;
}
```

In a stricter version, `useDerived` itself could take a `selector` argument (`(plane) => plane.xAxis.y`) to avoid re-rendering if only some plane fields changed.

### 5.4. Delete `useFlattenDiff`, `useFlatDesign`, `usePiecesMetadata`, `useFlatPiece`

Since we don’t care about backwards compatibility:

- Remove `useFlattenDiff` / `useFlatDesign` / `useFlatPieces` / `usePiecesMetadata` entirely.
- Replace their usages with new, narrower hooks:
  - Wherever you read `flatDesign.pieces`, decide what you _actually_ need:
    - For drawing: `useFlatPiecePlane`, `useFlatPieceCenter`.
    - For iterating: a derived node “flatPieces” (if you truly need the whole list).

---

## Phase 6 – Generalize pattern to “any nested item + field”

Once the infrastructure above is in place, the migration becomes systematic:

1. **For raw data** (no computation):
   - Replace custom niche hooks like `useSyncNestedArrayItemMembership`, `useSyncSelectionItemMembership` with generic `usePath` wrappers and/or a tiny domain helper.
   - Example:
     - `useIsPieceSelected(designStore, pieceGuid)` → `usePath(designStore, selectionPiecesPath, arr => arr.includes(pieceGuid))`.

2. **For derived data** (anything computed from snapshots):
   - Introduce a **derived node builder** that:
     - lists base deps (`BaseDependency[]`),
     - uses pure selectors to compute the result.

   - Expose a hook using `useDerived`.

3. **For very fine-grained fields** like `plane.xAxis.y`:
   - Either use `selector` on `useDerived`, or
   - Create dedicated derived nodes per field when that gives better reuse.

Because everything is expressed in:

- `YPath` + `Store` base API, and
- `DerivedStore` + `DerivedNode`,

you can subscribe to **any** path like:

- `designStore` path: `["pieces", byGuid(pieceGuid), "plane", "xAxis", "y"]`
- `kitStore` path: `["types", byGuid(typeGuid), "ports", arrayIndex(i), "compatibleInterfaces", arrayIndex(j)]`

and any composition/derivation on top is just another `DerivedNode`.

---

## Phase 7 – Clean-up & consolidation

With backwards compatibility not a concern, you can simplify aggressively:

1. **Remove deep, broad hooks**:
   - `useSyncDeep`, `useSyncFields`, `useKitTypes`, `useKitDesigns` where overkill.
   - Keep them only for debugging/inspecting, not for production rendering.

2. **Promote path/derived hooks as the default primitives**:
   - Document only:
     - `usePath`
     - `useDerived`
     - Domain-specific wrappers built on top.

3. **Make “compute in hooks” illegal** in code review:
   - All heavy computation must live either in:
     - pure selectors, or
     - derived nodes.

This leaves you with a very clean layering:

- **Yjs layer**: `Store` + `YPath`.
- **Domain layer**: pure snapshot selectors (`piecesMetadata` etc.).
- **Derived layer**: dependency graph of `DerivedNode`s.
- **React layer**: tiny hooks that just bind components to paths or derived nodes.

# Changes

## Phase 2: YPath Abstraction (shared.ts)

Added:

- `YPathSegment` type - Segment types for navigating Y.js structures (mapKey, arrayIndex, arrayItemById)
- `YPath` type - Array of segments for path navigation
- `yPathMapKey(key)` - Helper to create map key segment
- `yPathArrayIndex(index)` - Helper to create array index segment
- `yPathArrayItemById(id, idKey)` - Helper to find array item by id
- `getValueAtPath(root, path)` - Get value at path in Y.js structure
- `createPathObserver(root, path, subscribe)` - Create observer for specific path

## Phase 3: DerivedStore (shared.ts)

Added:

- `BaseDependency` interface - Store + path pair for derived dependencies
- `DerivedNode<T>` class - Node in derived dependency graph with caching
- `DerivedStore` class - Store for managing derived nodes with lazy initialization

## Phase 4: React Hooks (Sketchpad.tsx)

Added path API to base `Store` class:

- `onPathChanged(path, subscribe)` - Subscribe to path changes with registry
- `getPathSnapshot(path)` - Get value at path

Added path API to `DesignStore`:

- `onPathChanged(path, subscribe)` - Subscribe to path changes
- `getPathSnapshot(path)` - Get value at path
- `derived: DerivedStore` - Public derived store instance

Added path API to `KitStore`:

- `onPathChanged(path, subscribe)` - Subscribe to path changes
- `getPathSnapshot(path)` - Get value at path
- `derived: DerivedStore` - Public derived store instance

Added React hooks:

- `usePath(store, path, selector)` - Subscribe to specific path in Y.js store
- `useDerived(derivedStore, key, deps, compute, selector)` - Subscribe to derived value

## Phase 5: Complete Migration (Sketchpad.tsx, Design.tsx)

### Removed Old Hooks:

- `useFlattenDiff` - Grabbed entire types and designs arrays
- `useFlatDesign` - Applied flatten diff, causing overfetching
- `useFlatPieces` - Depended on useFlatDesign
- `usePiecesMetadata` (old version) - Computed full metadata Map in useMemo
- `useFlatPiece` - Used usePiecesMetadata for each piece
- `useFlatPiecePlane` (old version) - Used useFlatPiece
- `useFlatPieceCenter` (old version) - Used useFlatPiece
- `useIsConnectedPiece` (old version) - Used usePiecesMetadata
- `usePieceCenter` - Used usePiecesMetadata
- `usePiecePlane` - Used usePiecesMetadata
- `usePiecePlanes` - Used useFlatDesign
- `usePieceModelUrls` - Used useFlatDesign
- `usePieceDiffStatuses` - Used useFlatDesign
- `useDerivedPieceMetadata` (intermediate) - Per-piece compute, still overfetched
- `useDerivedFlatPiecePlane` (intermediate) - Now renamed
- `useDerivedFlatPieceCenter` (intermediate) - Now renamed
- `useDerivedIsConnectedPiece` (intermediate) - Now renamed
- `useDerivedPieceDepth` (intermediate) - Now renamed

### New Canonical Hooks:

- `PieceMetadata` type - Exported type for piece metadata
- `usePiecesMetadataMap()` - Returns cached Map using DerivedStore at design level
- `usePieceMetadata(pieceId?)` - Extracts from cached Map
- `useFlatPiecePlane(id?)` - Returns flattened plane
- `useFlatPieceCenter(id?)` - Returns flattened center
- `useIsConnectedPiece(id?)` - Returns connection status
- `usePieceDepth(id?)` - Returns hierarchy depth
- `useFixedPieceId(id?)` - Returns fixed piece ID (component root)
- `useParentPieceId(id?)` - Returns parent piece ID

### Design.tsx Refactoring:

- Updated imports: Replaced `useFlatDesign`, `usePiecesMetadata` with `usePiecesMetadataMap`, `useFixedPieceId`, `PieceMetadata`
- Refactored `designToNodesAndEdges()` to take `Map<string, PieceMetadata>` instead of `flattenedDesign`
- Updated `useDesignAppPieceCenter` and `useDesignAppPiecePlane` to use `usePiecesMetadataMap`
- Removed `flattenedDesign` dependency from `DesignDiagram` component

### Architecture Summary:

1. **Design-level caching**: `usePiecesMetadataMap` creates ONE DerivedNode per design that caches the full metadata Map
2. **Per-piece extraction**: `usePieceMetadata(id)` simply does `map.get(id)` - no recomputation
3. **Dependency tracking**: DerivedNode tracks pieces and connections paths, only recomputes when those change
4. **Clean API**: Simple hooks like `useFlatPiecePlane(id)` hide all complexity
