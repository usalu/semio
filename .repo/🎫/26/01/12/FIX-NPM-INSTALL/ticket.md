# Ticket

## Todos

# Plan - Fix npm Invalid Version error

1. **Diagnose**: Analyze `npm` log for `Invalid Version` error.
2. **Scan Package Files**: Find all `package.json` files in workspaces that lack a `version` field.
3. **Fix Versions**: Add `"version": "1.0.0"` to all identified `package.json` files to satisfy `npm` workspace requirements.
4. **Cleanup Lockfiles**: Remove corrupted or conflicting `package-lock.json` files (`root` and `js/vscode`) to allow a clean regeneration.
5. **Resolve Conflicts**: Align `@vitest/coverage-v8` versions with `vitest` version 3 to fix `ERESOLVE` errors.
6. **Verify**: Run `npm install` to confirm the fix.

## Changes

## Log

# Log - Fix npm Invalid Version error

- **Diagnostic**: Found that `npm install` failed with `TypeError: Invalid Version: `. This occurred during the placement of workspace dependencies.
- **Action**: Searched for missing `version` fields in `package.json` files.
- **Action**: Added `"version": "1.0.0"` to the following files:
  - [yak/package.json](yak/package.json)
  - [go/compose/package.json](go/compose/package.json)
  - [./repo/cli/package.json](./repo/cli/package.json)
  - [go/mcp/package.json](go/mcp/package.json)
  - [net/Compose/package.json](net/Compose/package.json)
  - [net/Compose.Grasshopper/package.json](net/Compose.Grasshopper/package.json)
  - [assets/logo/package.json](assets/logo/package.json)
  - [assets/icons/package.json](assets/icons/package.json)
  - [py/engine/package.json](py/engine/package.json)
  - [assets/images/package.json](assets/images/package.json)
  - [go/cli/package.json](go/cli/package.json)
- **Action**: Deleted corrupted lockfiles:
  - `d:\compose\package-lock.json`
  - `d:\compose\js\vscode\package-lock.json`
- **Correction**: Encountered `ERESOLVE` due to `@vitest/coverage-v8` version mismatch (V4 resolved instead of V3).
- **Action**: Updated `@vitest/coverage-v8` to `^3.0.8` in:
  - [js/compose/package.json](js/compose/package.json)
  - [js/desktop/package.json](js/desktop/package.json)
- **Verification**: Ran `npm install` successfully.

## Summary

# Summary - Fix npm Invalid Version error

Fixed `npm install` failure by adding missing `version` fields to workspace member `package.json` files and resolving `vitest` dependency version conflicts. Removed conflicting lockfiles to ensure a clean dependency tree.
