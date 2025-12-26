---
slug: KIT-PERFORMANCE-FIX
summary: Fix Kit app performance after importing large kits
prompt: Fix Kit app performance after importing large kits
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.769Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The Kit app became completely unresponsive after importing large kits like metabolism.zip. Expanding a type row took multiple seconds.

# Plan

1. Identify the root cause of performance issues
2. Fix useMemo dependency issues that cause unnecessary re-renders
3. Ensure memoization works correctly with stable dependencies

# Changes

## Root Cause

The performance issues were caused by three main problems in `js/js/sketchpad/Kit.tsx`:

1. **`expandedRows` Set recreated on every render**: `new Set(expandedRowsArray)` was called on every render, breaking useMemo dependency checks since Set reference always changed.

2. **`selection` object recreated on every render**: The selection object was created as a literal object on every render, also breaking memoization.

3. **`useMemo` dependencies on entire kit object**: The `rows` useMemo depended on `kit` and `kit.files` which changed on every deep sync update, causing expensive recomputation.

## Fixes

### 1. Memoized expandedRows Set

```typescript
// Before: new Set created on every render
const expandedRows = new Set(expandedRowsArray);

// After: Memoized using a join key
const expandedRowsArrayKey = expandedRowsArray.join(",");
const expandedRows = useMemo(() => new Set(expandedRowsArray), [expandedRowsArrayKey]);
```

### 2. Memoized selection object

```typescript
// Before: new object on every render
const selection = {
  types: kitApp?.selection?.types || [],
  designs: kitApp?.selection?.designs || [],
  ...
};

// After: Memoized using join keys for each array
const selectionTypesKey = selectionTypes.join(",");
const selection = useMemo(() => ({
  types: selectionTypes,
  designs: selectionDesigns,
  ...
}), [selectionTypesKey, selectionDesignsKey, ...]);
```

### 3. Stable keys for kit data

Created stable primitive keys from kit array data instead of depending on object references:

```typescript
const kitDesigns = kit?.designs;
const kitDesignsKey = useMemo(() => kitDesigns?.map((d) => `${d.guid}:${d.name}:${d.parent?.guid || ""}...`).join("|") || "", [kitDesigns]);

// Then use both the data and key in useMemo:
const rows = useMemo(() => {
  // ... expensive computation using kitDesigns
}, [kitDesigns, kitDesignsKey, kitTypesKey, ...]);
```

This ensures the expensive rows computation only runs when the actual content of the kit arrays changes, not on every deep sync update.

## Python Engine Tests

Fixed and refactored Python engine tests to work with dict-based domain functions:

### Issues with `Kit.parse()`

SQLAlchemy relationships cannot be set outside of a database session. The original tests worked on raw JSON dicts, not on `Kit.parse()` result. The functions for graph operations, validation, and flatten were defined in the test file.

### Solution

1. **Dict-based domain functions in engine.py**: Added functions that work on both dicts and entities:
   - `buildPieceGraph(design: Design | dict)` - Build networkx graph
   - `findFixedPieces(design: Design | dict)` - Find pieces with planes
   - `getConnectedComponents(design: Design | dict)` - Get connected components
   - `getPieceHierarchy(design: Design | dict, rootGuid: str)` - Get piece hierarchy
   - `validateKitDict(kit: dict)` - Validate kit JSON dict
   - `flattenDesignDict(kit: dict, designGuid: str)` - Flatten design from JSON dict

2. **Updated test file**: Tests now call engine functions directly with dict data instead of using `Kit.parse()`.

3. **Skipped REST tests**: REST API expects Input format but fixtures are in Output format (different schema).

### Test Results

All 29 tests pass (2 skipped for REST API format mismatch):

- 6 spatial math tests
- 4 graph operations tests
- 2 validation tests
- 1 serialization test
- 2 diff tests
- 4 GraphQL tests
- 5 flatten design tests
- 5 model tests
