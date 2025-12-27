---
slug: VSCODE-EXTENSION-FIX
summary: Fix VS Code extension linting and test setup
prompt: Fix VS Code extension linting and test setup
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.722Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The VS Code extension for semio was not showing validation errors on invalid kit JSON files. The test infrastructure was also not properly set up.

Problems identified:

1. The `kit_invalid.json` fixture had an outdated schema (missing required fields like `guid` for models, `t` for connectors, `guid` for layers, `name` instead of `path` for files/folders, etc.)
2. The VS Code extension tests were not being built - only `extension.ts` was compiled to `out/`, but `extension.test.ts` was not
3. Missing test dependencies (`@vscode/test-cli`, `@vscode/test-electron`, `@types/mocha`)
4. Incorrect path resolution in test file pointing to fixture
5. Missing `uuid` dependency for the extension

# Plan

1. Update `kit_invalid.json` fixture to match current Zod schema in `semio.ts`
2. Add validation domain logic tests to `semio.test.ts`
3. Set up proper VS Code extension test infrastructure
4. Fix the extension build and test configuration

# Changes

## 1. Fixed `assets/semio/kit_invalid.json`

- Added `t` property to connectors (required)
- Added `guid` and `file` properties to models (required)
- Added `guid` property to layers (required)
- Changed `path` to `name` for files and folders
- Changed `remoteUrl` to `remote` for files
- Changed `type` in pieces from string to object with `guid`
- Changed `piece` and `connector` in connections to objects with `guid`
- Moved `connections` inside the design (where they belong)

## 2. Added validation tests to `js/js/semio.test.ts`

- Added import for `InvalidKit`, `validateSemioKit`, `hasSemioErrors`
- Added "Validation" test suite with 3 tests:
  - "Valid kit has no errors" - validates MetabolismKit has no errors
  - "Invalid kit has all expected errors" - validates InvalidKit triggers all 11 validation rules
  - "Fixes can be applied to resolve issues" - validates fixes work correctly

## 3. Updated `js/vscode/package.json`

- Added test dependencies: `@types/mocha`, `@vscode/test-cli`, `@vscode/test-electron`
- Added `uuid` dependency (needed by @semio/js)
- Added `test` script: `vscode-test`
- Updated `build` script to also build tests

## 4. Created `js/vscode/vite.test.config.ts`

- Separate vite config for building test file
- Outputs to `out/test/extension.test.js`
- Uses `emptyOutDir: false` to not delete extension build

## 5. Fixed `js/vscode/extension.test.ts`

- Corrected path to fixture: `../../../../assets/semio/kit_invalid.json`
