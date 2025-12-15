---
slug: KIT-SNAPSHOT-OPTIMIZATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Added dirty flag cache optimization to KitStore.snapshot() to prevent
  expensive rebuilds on unrelated UI updates
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

Performance issue identified: After importing the metabolism kit (~180 files, 9 types, 2 designs), expanding/collapsing type rows in Kit.tsx takes 4600-5000ms per operation.

Root cause analysis revealed:

1. `rows` useMemo depended on `expandedRows`, causing full recalculation on every expand/collapse
2. `useKitApp()` without selector triggers re-render on ANY state change via `useSyncDeep`
3. **MAJOR**: `KitStore.snapshot()` rebuilds ENTIRE kit structure + JSON.stringify hash comparison on every call

# Plan

1. Split `rows` useMemo into `allRows` (stable) and `visibleRows` (filtered by expandedRows) - DONE
2. Add `dirty` flag to KitStore to skip expensive snapshot rebuilds when kit hasn't changed - DONE
3. Wire up `markDirty()` call in `change()` method - DONE
4. Remove debug logging after verification - DONE
5. Run tests to verify performance - DONE

# Changes

## `js/js/sketchpad/Sketchpad.tsx`

### KitStore class

Added `dirty` flag and cache optimization:

```typescript
// Line 5182 - Added dirty flag
private dirty: boolean = true;

// Line 5608 - Added markDirty method
markDirty = () => {
  this.dirty = true;
};

// Line 5612-5646 - Modified snapshot() method with early return
snapshot = (): Kit => {
  if (!this.dirty && this.cache) {
    return this.cache;  // Fast path: return cached value
  }
  // ... expensive rebuild only when dirty ...
  this.dirty = false;
  return this.cache;
};

// Line 5831 - Set dirty=true in change() method
this.dirty = true;  // Added before cache invalidation
this.cache = undefined;
this.cacheHash = undefined;
```

## `js/js/sketchpad/Kit.tsx`

### Split rows calculation

Separated `allRows` (stable, depends only on kit data) from `rows` (visible rows, depends on `expandedRows`):

```typescript
// allRows: builds complete hierarchy once, independent of expansion state
const allRows = useMemo<TableRow[]>(() => {
  // Always build all children regardless of expansion state
  // isExpanded is always set to false here (computed later)
}, [kitDesigns, kitTypes, ...]);  // NO expandedRows dependency

// rows: fast O(n) visibility filter
const rows = useMemo<TableRow[]>(() => {
  // Filter allRows based on ancestor expansion state
  // Set isExpanded based on expandedRows
}, [allRows, expandedRows]);
```

## Test Results

Playwright test verifies UI interactions are responsive (<500ms per action).
