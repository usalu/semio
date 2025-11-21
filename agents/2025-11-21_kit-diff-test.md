# Kit Diff Test

## Status

✅ Script created  
✅ Fixtures generated  
⚠️ Test created but vitest has configuration issues

## Accomplished

1. **Created `scripts/generate-kit-diff-fixtures.mjs`**
   - Loads metabolism kit from `assets/semio/kit_metabolism.json`
   - Uses seeded random generator (seed=42) for reproducible modifications
   - Generates comprehensive KitDiff exercising:
     - Kit metadata (name, version, description, icon, image, homepage, license)
     - Types (add, update with nested port changes, remove)
     - Designs (add, update, remove)
     - Qualities (add, update with benchmarks, remove)
     - Interfaces (add, update, remove)
     - Files (add, update, remove)
     - Folders (add, update, remove)
     - Authors (add, update, remove)
     - Attributes (add, update, remove)
   - Applies diff to create modified kit
   - Calculates inverse diff
   - Saves three JSON files in `assets/semio/`:
     - `diff_kit_metabolism.json` - forward diff
     - `diff_kit_metabolism_inverted.json` - inverse diff
     - `kit_metabolism_diffed.json` - modified kit

2. **Created `js/js/kit-diff.test.ts`**
   - Loads all four fixtures
   - Tests fixture loading
   - Tests kit metadata modifications
   - Tests types diff operations
   - Tests designs diff operations
   - Tests inverse diff metadata reversal
   - Tests inverse types operations reversal
   - Tests inverse designs operations reversal
   - Tests new type in diffed kit
   - Tests new design in diffed kit
   - Tests updated type in diffed kit
   - Tests updated design in diffed kit

## Known Issue

Vitest is failing with "No test suite found" for ALL test files in `js/js/`, not just the new kit-diff tests. This appears to be a systemic configuration issue with the vitest setup in this package, unrelated to the kit-diff test implementation.

Even the simplest possible test:
```typescript
import { describe, expect, it } from "vitest";
describe("Kit Diff Minimal", () => {
  it("should pass", () => {
    expect(true).toBe(true);
  });
});
```

Fails with the same error. The existing `simple.test.ts` and `semio.test.ts` also fail with the same error.

## Next Steps

1. Fix vitest configuration in `js/js/vite.config.ts` (separate issue)
2. Once vitest works, run kit-diff tests
3. Implement actual diff functions in `semio.ts` (getKitDiff, applyKitDiff, inverseKitDiff, mergeKitDiff)
4. Create more comprehensive tests that validate diff calculation and application

## Files Created/Modified

- ✅ `scripts/generate-kit-diff-fixtures.mjs`
- ✅ `assets/semio/diff_kit_metabolism.json`
- ✅ `assets/semio/diff_kit_metabolism_inverted.json`
- ✅ `assets/semio/kit_metabolism_diffed.json`
- ✅ `js/js/kit-diff.test.ts`
- ⚠️ `js/js/kit-diff-minimal.test.ts` (for debugging vitest issue)
