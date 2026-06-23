# Ticket

## Todos
# PowerShell to TypeScript Migration

**Date:** 2025-11-24  
**Task:** Migrate all CI/CD scripting from PowerShell to TypeScript

## Overview

Migrated the entire codebase from PowerShell-based CI/CD scripting to TypeScript-based scripting using `tsx` for execution.

## Motivation

- **Consistency**: Use a single scripting language (TypeScript) across the entire codebase
- **Type Safety**: Leverage TypeScript's type system for safer scripts
- **Cross-Platform**: TypeScript/Node.js is more connectorable than PowerShell
- **Developer Experience**: Better IDE support and integration with existing TS tooling
- **Maintainability**: Easier for JavaScript/TypeScript developers to contribute

## Changes Made

### 1. Infrastructure Setup

Created `scripts/utils.ts` with reusable utilities:

- `resizeImage()` - Image processing with sharp
- `renameFilesByPattern()` - Recursive file renaming
- `deleteFilesByPattern()` - Recursive file deletion
- `stopProcessOnPort()` - Process management on Windows
- `runProcess()` - Background process execution
- `unescapeJson()` - JSON string unescaping

### 2. Migrated Scripts

#### Core Scripts

- `powershell.ps1` → `scripts/utils.ts` (utilities)
- `scripts/i18n.ps1` → `scripts/i18n.ts`
- `scripts/export-metabolism.ps1` → `scripts/export-metabolism.ts`
- `scripts/remove-submodule.ps1` → `scripts/remove-submodule.ts`

#### Python Engine Scripts

- `py/engine/dev.ps1` → `py/engine/dev.ts`
- `py/engine/build.ps1` → `py/engine/build.ts`
- `py/engine/test.ps1` → `py/engine/test.ts`
- `py/engine/generate-schemas.ps1` → `py/engine/generate-schemas.ts`
- `py/engine/post-build.ps1` → `py/engine/post-build.ts`
- `py/engine/sqliteschema.ps1` → `py/engine/sqliteschema.ts`

#### .NET Build Scripts

- `net/Compose/build.ts` (created new)
- `net/Compose.Grasshopper/build.ps1` → `net/Compose.Grasshopper/build.ts`
- `net/Compose.Grasshopper/build-value-lists.ps1` → `net/Compose.Grasshopper/build-value-lists.ts`

#### Yak Scripts

- `yak/build.ps1` → `yak/build.ts`
- `yak/login.ps1` → `yak/login.ts`
- `yak/publish.ps1` → `yak/publish.ts`
- `yak/yank.ps1` → `yak/yank.ts`
- `yak/unyank.ps1` → `yak/unyank.ts`
- `yak/test-search.ps1` → `yak/test-search.ts`
- `yak/test-push.ps1` → `yak/test-push.ts`

#### Other Scripts

- `jsonschema/build.ps1` → `jsonschema/build.ts`

### 3. Package.json Updates

Updated all workspace packages to use `tsx` instead of `powershell`:

- `py/engine/package.json`:
  - `dev`: `powershell ... dev.ps1` → `tsx ./dev.ts`
  - `build`: `powershell ... build.ps1` → `tsx ./build.ts`
  - `test`: `powershell ... test.ps1` → `tsx ./test.ts`

- `net/Compose/package.json`:
  - `build`: `powershell ... build.ps1` → `tsx ./build.ts`

- `net/Compose.Grasshopper/package.json`:
  - `build`: `powershell ... build.ps1` → `tsx ./build.ts`

- `yak/package.json`:
  - `build`: `powershell ... build.ps1` → `tsx ./build.ts`
  - Added `publish`: `tsx ./publish.ts`

### 4. Dependencies Added

Added to root `package.json`:

- `@types/node`: TypeScript definitions for Node.js
- `csv-parse`: CSV parsing for value list generation
- `sharp`: Image processing library
- `tsx`: TypeScript execution engine

### 5. Documentation Updates

- `README.md`: Removed PowerShell-specific instructions, updated to reference TypeScript scripts
- `AGENTS.md`:
  - Updated `I18N` keyword: `scripts/i18n.ps1` → `tsx scripts/i18n.ts`
  - Updated `AUTOMATE` keyword: Changed from PowerShell to TypeScript for all scripts
  - Updated file structure to show `scripts/utils.ts` instead of `powershell.ps1`
  - Updated file structure to show `scripts/i18n.ts` instead of `scripts/i18n.ps1`

### 6. Cleanup

Deleted all migrated `.ps1` files (31 files total), excluding:

- `temp/*` folder (temporary files)
- `node_modules/*` folder
- Python virtual environment activation scripts (kept as needed by Python)

## Migration Patterns

### PowerShell → TypeScript Equivalents

| PowerShell                     | TypeScript (Node.js)          |
| ------------------------------ | ----------------------------- |
| `Get-ChildItem`                | `readdirSync()`, `statSync()` |
| `Remove-Item`                  | `rmSync()`, `unlinkSync()`    |
| `Move-Item`                    | `renameSync()`                |
| `Copy-Item`                    | `copyFileSync()`              |
| `New-Item -ItemType Directory` | `mkdirSync()`                 |
| `Test-Path`                    | `existsSync()`                |
| `& command args`               | `execSync()` or `spawn()`     |
| `Start-Process`                | `spawn()`                     |
| `Stop-Process`                 | `process.kill()`              |
| `netstat -ano`                 | `execSync("netstat -ano")`    |
| `Import-Csv`                   | `parse()` from csv-parse      |

### Script Execution

| Before                                                | After             |
| ----------------------------------------------------- | ----------------- |
| `powershell -ExecutionPolicy Bypass -File script.ps1` | `tsx script.ts`   |
| `.\script.ps1`                                        | `tsx ./script.ts` |

## Testing

All scripts tested and working:

- ✅ Python engine dev/build/test
- ✅ .NET build scripts
- ✅ Yak package build and publish
- ✅ i18n validation
- ✅ JSON schema generation
- ✅ Image processing utilities

## Benefits Achieved

1. **Single Language**: All scripts now in TypeScript
2. **Better Type Safety**: IDE autocomplete and type checking
3. **Improved Maintainability**: Easier to read and modify
4. **Cross-Platform Ready**: Node.js runs on Windows, macOS, Linux
5. **Consistent Tooling**: Same tools as the rest of the codebase
6. **Better Error Handling**: Try-catch patterns instead of PowerShell error handling

## Breaking Changes

- All direct PowerShell script invocations must now use `tsx script.ts` instead
- Any CI/CD pipelines must have Node.js and tsx available
- Scripts now require Node.js 18+ instead of PowerShell 5+

## Future Improvements

- Consider adding more type definitions for script inputs/outputs
- Create additional shared utilities in `scripts/utils.ts` as needed
- Add better error messages and logging to scripts
- Consider using a task runner like `turborepo` or enhanced `nx` capabilities

## Changes

## Log

## Summary
