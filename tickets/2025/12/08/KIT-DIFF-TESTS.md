---
slug: KIT-DIFF-TESTS
summary: Fix kit diff tests and regenerate assets
prompt: Fix kit diff tests and regenerate assets
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.886Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Kit diff tests were failing in TypeScript and Python because:

1. The generate-metabolism-diff.ts script was missing the `Tag` import
2. Python engine was missing quality and author diff functions
3. Python engine's `getKitDiffDict`, `applyKitDiffDict`, and `inverseKitDiffDict` were missing quality and author handling

# Plan

1. Add missing `Tag` import to generate-metabolism-diff.ts
2. Add quality and author diff functions to Python engine
3. Update Python `getKitDiffDict` to include qualities and authors
4. Update Python `applyKitDiffDict` to include qualities and authors
5. Update Python `inverseKitDiffDict` to include qualities and authors
6. Regenerate diff assets
7. Verify all tests pass

# Changes

## scripts/generate-metabolism-diff.ts

- Added missing `Tag` import

## py/engine/engine.py

- Added `_getQualityDiff()` - compute diff between two quality dicts
- Added `_applyQualityDiff()` - apply diff to a quality dict
- Added `_inverseQualityDiff()` - compute inverse of a quality diff
- Added `_getAuthorDiff()` - compute diff between two author dicts
- Added `_applyAuthorDiff()` - apply diff to an author dict
- Added `_inverseAuthorDiff()` - compute inverse of an author diff
- Updated `getKitDiffDict()` to include qualities and authors collection diffs
- Updated `applyKitDiffDict()` to apply qualities and authors diffs
- Updated `inverseKitDiffDict()` to inverse qualities and authors diffs

## assets/semio/

- Regenerated diff_kit_metabolism.json
- Regenerated diff_kit_metabolism_inverted.json
- Regenerated kit_metabolism_diffed.json

## Status

- TypeScript tests: PASSING
- Python tests: PASSING
- C# tests: SKIPPED (requires EntityId schema changes - `List<string>` needs to become `List<EntityId>`)
