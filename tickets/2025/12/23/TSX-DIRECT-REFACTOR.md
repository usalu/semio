---
slug: TSX-DIRECT-REFACTOR
prompt: Refactor all the scripts to be able to use tsx directly.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-23T13:28:13.198Z"
  finished: "2025-12-23T13:28:53.505Z"
summary: Refactored all scripts to use npx tsx directly instead of node --import tsx
commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
model: composer-1
iterations:
  - prompt: Refactor all the scripts to be able to use tsx directly.
    date:
      started: "2025-12-23T13:28:19.725Z"
      ended: "2025-12-23T13:28:30.700Z"
    model: composer-1
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
    files:
      updated:
        - preflight.ts:
            lines:
              added: 14
              removed: 8
        - scripts/log.tsx:
            lines:
              added: 1143
              removed: 0
        - scripts/generate-validation.tsx:
            lines:
              added: 63
              removed: 0
        - scripts/rename-files.tsx:
            lines:
              added: 367
              removed: 0
      created: []
      removed: []
    lines:
      added: 1587
      removed: 8
files:
  updated:
    - preflight.ts:
        lines:
          added: 14
          removed: 8
    - scripts/generate-validation.tsx:
        lines:
          added: 63
          removed: 0
    - scripts/log.tsx:
        lines:
          added: 1143
          removed: 0
    - scripts/rename-files.tsx:
        lines:
          added: 367
          removed: 0
  created: []
  removed: []
lines:
  added: 1587
  removed: 8
---

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
