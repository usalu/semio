# Previously

Scripts were executed using `node --import tsx` in `preflight.ts`. The `scripts/log.tsx` file used CommonJS `require()` syntax which doesn't work in ES modules. The `scripts/generate-validation.tsx` file imported from `@semio/assets` package which had module resolution issues. Help text in scripts referenced the old execution method.

# Plan

Refactor all scripts to use `npx tsx` directly instead of `node --import tsx`. Fix ES module compatibility issues in scripts that were preventing direct `tsx` execution.

# Changes

- Updated `preflight.ts` to use `npx tsx` instead of `node --import tsx` for all hook executions
- Fixed `scripts/log.tsx`:
  - Converted `require("gray-matter")` to ES module import: `import matter from "gray-matter"`
  - Removed `require.main === module` check (not needed in ES modules)
- Fixed `scripts/generate-validation.tsx`:
  - Changed import from `@semio/assets` to relative path: `import InvalidKit from "../assets/semio/kit_invalid.json"`
- Updated help text in `scripts/rename-files.tsx` to show `npx tsx` usage
- Added `COMPOSER_1` model to `scripts/log.tsx` Model enum for future ticket tracking
