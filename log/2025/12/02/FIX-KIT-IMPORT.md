---
date: "2025-12-02T17:31:33.308Z"
slug: FIX-KIT-IMPORT
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix kit import and design app infinite loop
model: claude-opus-4.5
---

# Previously

Playwright test for Design app was timing out when importing the metabolism kit. The test failed at `page.waitForURL` because:

1. The `sqliteToKit` function hangs when querying a non-existent `tag` table
2. The `DesignAppFooter` component crashes due to React `useSyncExternalStore` infinite loop

# Plan

1. Fix `sqliteToKit` to handle missing tables gracefully
2. Fix selector stability issues causing infinite loops in `useSyncExternalStore`
3. Fix undefined `kit` variable in `DesignAppFooter`
4. Clean up debug logging
5. Verify test passes

# Changes

## semio.ts

- Added `safeExecResult` helper that checks if table exists before querying
- Used `safeExecResult` for the `tag` table query which may not exist in older kits
- Query `sqlite_master` for existing tables at start of conversion

## Sketchpad.tsx

- Added stable empty array constants for all targeted kit hooks (EMPTY_TYPES, EMPTY_AUTHORS, etc.)
- Created stable selector functions instead of inline functions to avoid infinite loop with `useSyncExternalStore`
- The issue was that inline selectors like `(k) => k.types ?? []` are recreated each render, which causes `getSnapshot` callback to be recreated, triggering re-renders in an infinite loop

## Design.tsx

- Fixed `useDesignAppSelectedModelTags` to use stable `EMPTY_MODEL_TAGS` constant
- Fixed `useDesignAppSelection` to use stable `EMPTY_SELECTION` constant
- Fixed `getTypesWithTag` callback that referenced undefined `kit` variable - now uses `types` from `useKitTypes()` hook

## Type.tsx

- Fixed `useTypeAppSelection` to use stable `EMPTY_TYPE_SELECTION` constant
- Fixed `useTypeAppSelectedModelTags` to use stable `EMPTY_MODEL_TAG_ARRAY` constant
