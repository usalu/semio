---
date:
  created: '2025-12-02T21:44:16.196Z'
  updated: '2025-12-02T21:44:16.196Z'
slug: FIX-DIFF-TEST
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: 'Fix diff test - schema changed, concepts now objects'
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

The diff test was failing at line 60: `areKitDiffsEqual(computedInverseDiff, kitDiffInverted)` returns false.

Root cause: The schema changed. `concepts` in Kit is now `Concept[]` (array of objects with `guid`, `name`, `description`, `icon`, `attributes`) but the generate-metabolism-diff.ts script was setting `concepts` as string array.

# Plan

1. Update generate-metabolism-diff.ts to use proper Concept objects
2. Regenerate the diff files by running the script
3. Run tests to verify

# Changes

## `generate-metabolism-diff.ts`

- Added `Tag` and `Concept` imports
- Changed `concepts` assignment from string array to proper `Concept[]` objects
- Enhanced comprehensive diff coverage for all entity types: types, designs, tags, concepts, interfaces, qualities, files, folders, authors, attributes
- Each entity type now tests: add, remove, update operations

## `semio.ts`

- Fixed `InterfaceDiffSchema` to allow nullable `description` and `icon` fields
- Fixed `getInterfaceDiff` to use `null` instead of `undefined` for cleared values
- Fixed `inverseInterfaceDiff` to use `null` instead of `undefined` for original undefined values
- Fixed `applyInterfaceDiff` to properly handle `null` (clear property) vs `undefined` (no change)

## Generated files

- Regenerated `diff_kit_metabolism.json`, `diff_kit_metabolism_inverted.json`, `kit_metabolism_diffed.json`
