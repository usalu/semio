---
slug: MODEL-LOADING-FIX
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Fix model loading in Type app by matching imported file blobs to existing kit
  file definitions
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

When opening a type (e.g., Tambour) in the Type app, the model was not displayed and console warnings appeared:

- `[TypeMesh] No models available for type`
- `[TypeMesh] File not found in kit for model`

The metabolism kit has all model files available in the zip, but they were not being correctly associated with the kit's file definitions.

# Plan

1. Investigate the import flow to understand why files weren't being matched
2. Fix the file association during import
3. Update tests to verify model loading works correctly

# Changes

## Root Cause

When importing a kit from a zip file:

1. `importKit()` extracts the kit from SQLite (with file definitions having specific GUIDs and folder references) and extracts file blobs (keyed by zip paths like "models/tambour.glb")
2. The old code then called `addFiles` which created **new** file definitions with **new** GUIDs instead of matching blobs to existing files
3. This caused a mismatch: the kit had file definitions with one set of GUIDs, but the blobs were stored under different GUIDs

## Solution

1. Added `buildFilePathMap()` method to `KitStore` that builds a map from storage paths to file GUIDs
2. Added `storeFileBlobs()` method to `KitStore` that stores blobs for existing kit files by matching paths
3. Modified `Home.tsx` import handlers to use `storeFileBlobs()` instead of creating duplicate files

## Files Changed

- `js/js/sketchpad/Sketchpad.tsx`: Added `buildFilePathMap()` and `storeFileBlobs()` methods to KitStore
- `js/js/sketchpad/Home.tsx`: Simplified import handlers to store blobs for existing files, added error logging
- `js/js/sketchpad.test.ts`: Enhanced Type test to verify no model warnings appear

## Verification

The Kit test passes, confirming the import flow works correctly. The Type test validates that no `[TypeMesh] No model` warnings appear after import.
