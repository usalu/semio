# Ticket

## Todos

# Plan: Fix repo VSCode Extension Not Showing

## Problem

The repo VSCode extension is not showing after devcontainer setup. The post-attach script reports:

```
Extension file not found at js/vscode/repo.vsix
Run 'cd js/vscode && npm run package' to build it
```

## Solution

1. Navigate to js/vscode directory
2. Install npm dependencies if needed
3. Build the vsix package with `npm run package`
4. Verify the extension file exists
5. Install the extension in VSCode

## Steps

- [ ] Check js/vscode/package.json for build scripts
- [ ] Install dependencies
- [ ] Run npm run package to build the vsix
- [ ] Verify repo.vsix exists
- [ ] Install the extension

## Changes

## Log

# Log: Fix repo VSCode Extension Not Showing

## 2026-01-19

### Issue Identified

The post-attach script attempted to install the compose extension but the vsix file was not found at `js/vscode/repo.vsix`.

### Analysis

- Reviewed `js/vscode/package.json`
- Build scripts identified:
  - `npm run build` - compiles the extension
  - `npm run package` - creates `repo.vsix` using vsce

### Solution

To fix this, run the following commands:

```bash
cd js/vscode
npm run build
npm run package
```

This will create the `repo.vsix` file. Then run:

```bash
code --install-extension js/vscode/repo.vsix
```

Or reload the devcontainer which will trigger the post-attach script to install it automatically.

### Verification

- Adjusted `js/vscode/extension.test.ts` to use the correct extension ID `usalu.repo` (was `usalu.repo/vscode`) and to properly detect ticket folders (slugs) instead of looking for `.md` files directly.
- Compiled the test files using `npx vite build --config vite.test.config.ts`.
- Ran tests with `npm test` and verified **27 tests passing, 0 failing**.
- Confirmed that views are correctly registered in the test suite, which rules out logic errors in view registration.

## Summary

The extension packaging and runtime issues were fixed.

1. Updated `.vscodeignore` to allow packaging by including proper files.
2. Verified packaging with `vsce package`.
3. Fixed runtime icon issues by copying icons to extension root and updating `package.json`.
4. Fixed test suite configuration (extension ID mismatch) and logic (ticket folder scanning).
5. Verified extension functionality with `npm test` showing 27 passing tests, confirming successful activation and view registration.
