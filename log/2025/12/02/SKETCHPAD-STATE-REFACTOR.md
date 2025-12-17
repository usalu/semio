---
slug: SKETCHPAD-STATE-REFACTOR
summary: >-
  Refactor sketchpad state management to fix performance issues with
  overfetching
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.824Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The sketchpad state management was causing performance issues due to overfetching:

- Components using `useKit()` without selectors subscribed to the entire kit state
- Using selectors with `useKit()` doesn't fix overfetching - the subscription still happens at the store level
- Using `deep=true` caused expensive deep subscriptions that triggered re-renders on any nested change
- The `PieceMesh` and `TypeMesh` components used `useKit(undefined, undefined, true)` but only needed `kit.files`

The fundamental issue: **Selectors filter the return value but don't change what triggers re-renders.**

# Plan

1. ✅ Add systematic performance logging to hooks
2. ✅ Add field-level observers to KitStore (onTypesChanged, onFilesChanged, etc.)
3. ✅ Rewrite targeted hooks to use Y.Array-specific subscriptions
4. ✅ Update Design.tsx components to use targeted hooks
5. ✅ Update Kit.tsx components to use targeted hooks
6. ✅ Update Type.tsx components to use targeted hooks
7. 🔄 Test UI performance with metabolism kit (requires manual testing)

# Changes

## Sketchpad.tsx - KitStore

Added field-level observers that subscribe to specific Y.Arrays:

- `onTypesChanged(subscribe, deep)` - observes only yTypes
- `onFilesChanged(subscribe, deep)` - observes only yFiles
- `onDesignsChanged(subscribe, deep)` - observes only yDesigns
- `onQualitiesChanged(subscribe, deep)` - observes only yQualities
- `onAuthorsChanged(subscribe, deep)` - observes only yAuthors
- `onFoldersChanged(subscribe, deep)` - observes only yFolders
- `onScalarFieldChanged(key, subscribe)` - observes specific yKit field

## Sketchpad.tsx - Targeted Hooks

Rewrote hooks to use `useSyncExternalStore` with field-level observers:

- `useKitTypes(guid, deep)` - subscribes to yTypes only
- `useKitName(guid)` - subscribes to name field only
- `useKitDescription(guid)` - subscribes to description field only
- `useKitAuthors(guid, deep)` - subscribes to yAuthors only
- `useKitFiles(guid, deep)` - subscribes to yFiles only
- `useKitQualities(guid, deep)` - subscribes to yQualities only
- `useKitDesigns(guid, deep)` - subscribes to yDesigns only (NEW)
- `useKitFolders(guid, deep)` - subscribes to yFolders only (NEW)
- `useTypeFromKit(typeGuid, kitGuid)` - uses useKitTypes internally
- `useDesignFromKit(designGuid, kitGuid)` - uses useKitDesigns internally

Note: `useKitInterfaces`, `useKitTags`, `useKitConcepts` still use `useKit` because these don't have separate Y.Arrays in KitStore yet.

## Design.tsx

- `DesignAppFooter`: Uses `useKitTypes()` and `useKitTags()`
- `DesignSectionForm`: Removed unused `useKit()` call
- `PieceMesh`: Uses `useKitFiles()` instead of deep kit subscription

## Kit.tsx

- `AppContent`: Changed from `deep=true` to `deep=false`

## Type.tsx

- `TypeMesh`: Uses `useKitFiles()` instead of deep kit subscription
