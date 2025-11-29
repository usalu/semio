---
date: "2025-11-29T00:53:42.477Z"
slug: kit-import-test-fix
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix Kit Import Playwright Test
model: claude-sonnet-4.5
---

# Previously

The Kit Import Playwright test was failing due to multiple issues:

1. Navigation after import wasn't working with React Router
2. Import was failing with `Cannot read properties of undefined (reading 'guid')` errors
3. Test assertions were timing out waiting for UI elements

# Plan

1. Fix navigation after kit import using `navigateToKit`
2. Add defensive checks in KitStore, TypeStore, and DesignStore to handle undefined guids
3. Create robust Playwright test with proper waits and assertions

# Changes

## `js/js/sketchpad/Sketchpad.tsx`

- Added defensive checks in `KitStore` constructor to skip entities with missing guid:
  - `kit.attributes`, `kit.authors`, `kit.folders`, `kit.qualities`, `kit.types`, `kit.designs`, `kit.files`
- Added defensive checks in `TypeStore` constructor for author references
- Added defensive checks in `DesignStore` constructor for all nested entities:
  - `pieces`, `connections`, `attributes`, `stats`, `props`, `layers`, `groups`, `authors`
- Added fallback in `useNavigate` to use `window.location.href` if `reactNavigate` is undefined

## `js/js/sketchpad/Home.tsx`

- Updated `handleFileInputChange` to use `navigateToKit` for SPA navigation
- Changed kit creation to persist locally (`local=true`) to survive navigation

## `js/js/sketchpad.test.ts`

- Created "Kit Import" test that:
  - Imports `metabolism.zip` via file input
  - Waits for navigation to `/kits/...`
  - Verifies no import errors in console
  - Asserts types (Capsule, Tambour, Base) and designs (Nakagin Capsule Tower, Capsule Dream) are visible
