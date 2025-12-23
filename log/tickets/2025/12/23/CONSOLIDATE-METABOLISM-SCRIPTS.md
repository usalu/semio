---
slug: CONSOLIDATE-METABOLISM-SCRIPTS
prompt: >-
  Consolidate scripts/generate-metabolism-diff.tsx
  scripts/generate-validation.tsx scripts/regenerate-metabolism.tsx to
  update-metabolism.tsx
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-23T13:39:57.342Z'
  finished: '2025-12-23T13:42:09.548Z'
summary: Consolidate three Metabolism scripts into single update-metabolism.tsx
commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
model: composer-1
iterations:
  - prompt: >-
      Consolidate scripts/generate-metabolism-diff.tsx
      scripts/generate-validation.tsx scripts/regenerate-metabolism.tsx to
      update-metabolism.tsx
    date:
      started: '2025-12-23T13:41:28.996Z'
      ended: '2025-12-23T13:41:44.819Z'
    model: composer-1
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
    files:
      updated:
        - scripts/update-metabolism.tsx:
            lines:
              added: 500
              removed: 0
        - AGENTS.md:
            lines:
              added: 25
              removed: 20
      created: []
      removed:
        - scripts/generate-metabolism-diff.tsx
        - scripts/generate-validation.tsx
        - scripts/regenerate-metabolism.tsx
    lines:
      added: 525
      removed: 20
files:
  updated:
    - AGENTS.md:
        lines:
          added: 25
          removed: 20
    - scripts/update-metabolism.tsx:
        lines:
          added: 500
          removed: 0
  created: []
  removed:
    - scripts/generate-metabolism-diff.tsx
    - scripts/generate-validation.tsx
    - scripts/regenerate-metabolism.tsx
lines:
  added: 525
  removed: 20
---
# Previously

Three separate scripts handled Metabolism asset generation:
- `scripts/generate-metabolism-diff.tsx` - Generated diff files for testing (`diff_kit_metabolism.json`, `diff_kit_metabolism_inverted.json`, `kit_metabolism_diffed.json`)
- `scripts/generate-validation.tsx` - Generated `validation.json` from `kit_invalid.json`
- `scripts/regenerate-metabolism.tsx` - Regenerated `metabolism.zip` and copied to public folders

This separation required running multiple scripts to update all Metabolism assets, and the scripts had overlapping functionality (all loading the kit JSON).

# Plan

1. Create consolidated `scripts/update-metabolism.tsx` that combines all three operations
2. Organize code into regions: File Collection, Regenerate Zip, Generate Diff, Generate Validation
3. Update AGENTS.md documentation to reference the new script
4. Delete the three old scripts

# Changes

## Created Consolidated Script

Created `scripts/update-metabolism.tsx` that performs all three operations in sequence:

1. **Regenerate Zip**: Exports `metabolism.zip` from `kit_metabolism.json` and example files, copies to all public folders
2. **Generate Diff**: Creates comprehensive diff files for testing
3. **Generate Validation**: Generates `validation.json` from `kit_invalid.json`

The script uses regions to organize code:
- `#region File Collection` - Shared file collection logic
- `#region Regenerate Zip` - Zip generation and copying
- `#region Generate Diff` - Diff file generation
- `#region Generate Validation` - Validation JSON generation
- `#region App` - React UI component

## Updated Connector Naming

Fixed connector naming in diff generation to use "connector" instead of "port" (`new-connector`, `test-connector` instead of `new-port`, `test-port`).

## Updated Documentation

Updated `AGENTS.md` to reference the new consolidated script instead of the old `generate-validation.ts` script.

## Removed Old Scripts

Deleted the three separate scripts:
- `scripts/generate-metabolism-diff.tsx`
- `scripts/generate-validation.tsx`
- `scripts/regenerate-metabolism.tsx`
