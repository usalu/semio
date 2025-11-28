---
date: "2025-11-28T22:00:31.396Z"
slug: DRAG-DROP-IMPORT-TEST
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Create test for kit import drag and drop
model: claude-sonnet-4.5
---

# Previously

No previous context.

# Plan

- [x] Create Playwright test for kit import drag and drop with metabolism.zip
- [x] Verify temporary kit creation after import
- [x] Check all types are imported (11 expected types including Tambour, Cylindric Tambour)
- [x] Check all proto-designs are imported (Nakagin Capsule Tower, Capsule Dream)
- [x] Verify Tambour ports (10 ports with specific coordinates)
- [x] Verify Nakagin Capsule Tower has 180 pieces
- [x] Ensure .semio folder is NOT imported
- [x] Verify representations and icons folders/files are present

# Changes

## `js/js/sketchpad.test.ts`

Added comprehensive test `Kit Import Drag and Drop` that:

1. Fetches `metabolism.zip` from `/assets/semio/metabolism.zip`
2. Creates DataTransfer with the zip file for drop simulation
3. Dispatches dragover and drop events on the canvas
4. Verifies navigation to kit URL after import
5. Validates the imported kit:
   - Kit is temporary (local=false, remote=false)
   - Kit name is "Metabolism"
   - Contains 11 expected type names
   - Contains 2 proto-designs (Nakagin Capsule Tower, Capsule Dream)
   - Tambour type has 10 ports with exact coordinates validated to 0.001 tolerance
   - Nakagin Capsule Tower design has 180 pieces
   - No .semio folder files imported
   - Has representations folder with >100 files (glb, 3dm)
   - Has icons folder with >30 files (svg, 3dm)
   - Specific representation files verified (base.glb, tambour.glb, etc.)
   - Specific icon files verified (base.svg, tambour.svg, metabolism.svg, etc.)
