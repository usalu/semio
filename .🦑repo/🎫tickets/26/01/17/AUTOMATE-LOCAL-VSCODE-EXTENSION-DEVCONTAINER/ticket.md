# Ticket

## Todos

# Plan: Automate Local VSCode Extension Installation in Devcontainer

## Overview

The compose VSCode extension at `js/vscode/` needs to be automatically installed when the devcontainer is created or started.

## Current State

- VSCode extension source at `js/vscode/`
- Extension builds to `js/vscode/out/`
- No automation currently exists to install the local extension in the devcontainer
- `post-create.sh` handles dependencies but not local extension installation

## Approach

Use `@vscode/vsce` to package the extension and install it via `code --install-extension` in the post-create script.

## Steps

1. **Add `@vscode/vsce` as a dev dependency** to `js/vscode/package.json`
   - This tool packages VSCode extensions into `.vsix` files

2. **Add a `package` script** to `js/vscode/package.json`
   - Script: `"package": "vsce package --out repo.vsix"`
   - This creates a `repo.vsix` file in the extension directory

3. **Update `.devcontainer/post-create.sh`** to:
   - Build the VSCode extension
   - Package it into a `.vsix` file
   - Install it using `code --install-extension`

4. **Add `.vsix` to `.gitignore`** if not already present

## Files to Modify

- `js/vscode/package.json` - Add vsce dependency and package script
- `.devcontainer/post-create.sh` - Add extension build/install steps
- `.gitignore` - Add `*.vsix` pattern (if needed)

## Changes

## Log

# Log

**Status**: COMPLETED (ticket close failed due to no git repo)

## Investigation

- Analyzed the current devcontainer configuration at `.devcontainer/`
- Found the local VSCode extension at `js/vscode/` with its `package.json`
- Extension builds to `js/vscode/out/` but was not being installed in devcontainer
- No existing automation for installing the local extension

## Implementation

1. Added `@vscode/vsce` dependency to `js/vscode/package.json` for packaging
2. Added `package` script to create `.vsix` file: `vsce package --out repo.vsix`
3. Updated `.devcontainer/post-create.sh` to:
   - Build the extension with `npm run build`
   - Package it with `npm run package`
   - Install it with `code --install-extension repo.vsix --force`
4. Added `*.vsix` to `.gitignore` to avoid committing packaged extensions

## Notes

- The `--force` flag ensures the extension is reinstalled even if already present
- The `|| true` at the end of the install command prevents failure if `code` CLI is not available (e.g., during initial container build)

## Summary

Bulk close

## Changes

- Added `@vscode/vsce` dependency and `package` script to `js/vscode/package.json`
- Updated `.devcontainer/post-create.sh` to build, package, and install the extension
- Added `*.vsix` to `.gitignore`

## Result

When a new devcontainer is created, the compose VSCode extension will be automatically built and installed, making it immediately available for development.
