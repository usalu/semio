# Fix KitApp Diagram Table Sync and Infinite Loop

## Summary

Two bugs: (1) `useFileUrls` caused infinite render loop because `getSnapshot` returned a new `Map` every call, violating `useSyncExternalStore` contract. (2) Files without extensions (GUID-named internal model files) appeared in both diagram and table.

## Root Causes

1. `getStoredKitFileUrls` returns `new Map()` on every call. `useSyncExternalStore` requires `getSnapshot` to return the same reference when data is unchanged. Fixed by caching with a serialized key in `useFileUrls`.
2. Internal model files stored by the engine have no file extension (e.g. `c7c924...`). Both `filteredKitFiles` (table) and `filteredDiagramFiles` (diagram) now require `file.name.includes(".")` to exclude them.

## Files

- `compose/sketchpad/index.tsx` (updated)

## Status

Closed
