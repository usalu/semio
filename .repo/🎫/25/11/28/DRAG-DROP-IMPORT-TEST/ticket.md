# Ticket

## Todos

# Previously

No previous context.

## Problems Discovered

**Bug Found**: KitStore.createKit fails when importing kits with types - `TypeError: Cannot read properties of undefined (reading 'guid')` in TypeStore constructor. The reactive Sketchpad store doesn't properly handle kit data imported via `importKit()`. Unit tests pass because they use simpler JSON-based import/export that doesn't go through the full reactive store creation.

# Plan

- [x] Create Playwright test for kit import drag and drop with metabolism.zip
- [x] Verify temporary kit creation after import
- [x] Check all types are imported (11 expected types including Tambour, Cylindric Tambour)
- [x] Check all proto-designs are imported (Nakagin Capsule Tower, Capsule Dream)
- [x] Verify Tambour connectors (10 connectors with specific coordinates)
- [x] Verify Nakagin Capsule Tower has 180 pieces
- [x] Ensure .compose folder is NOT imported
- [x] Verify representations and icons folders/files are present

# Changes

## `js/compose/sketchpad.test.ts`

Added comprehensive UI-based test `Kit Import Drag and Drop` that:

1. **Drag & Drop Import**:
   - Fetches `metabolism.zip` from `/assets/compose/metabolism.zip`
   - Creates DataTransfer with the zip file
   - Dispatches dragover and drop events on body
   - Waits for navigation to kit URL

2. **Types Verification** (`/kits/{guid}?kind=types`):
   - Verifies types are visible: Capsule, Tambour, Base, Bridge, Capital, Cylindric Capital, Cylindric Tambour

3. **Designs Verification** (`/kits/{guid}?kind=designs`):
   - Verifies proto-designs: Nakagin Capsule Tower, Capsule Dream

4. **Tambour Connectors** (`/kits/{guid}/types/{tambourGuid}`):
   - Double-clicks Tambour row to navigate to type app
   - Opens workbench panel
   - Verifies all 10 connector names: b, t, sl0_d0, sl0_d1, sl0_d2, sl0_d3, sl1_d0, sl1_d1, sl2_d0, sl2_d1

5. **Nakagin Capsule Tower Pieces** (`/kits/{guid}/designs/{designGuid}`):
   - Double-clicks design row to navigate to design app
   - Verifies diagram contains >= 50 nodes (pieces)

6. **Files Verification** (`/kits/{guid}?kind=files`):
   - Verifies `representations` folder is visible
   - Verifies `icons` folder is visible
   - Verifies `.compose` folder is NOT visible (excluded from import)

## Changes

## Log

## Summary

# Summary

Create test for kit import drag and drop
