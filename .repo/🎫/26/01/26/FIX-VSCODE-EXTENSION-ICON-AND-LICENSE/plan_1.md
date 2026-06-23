# Plan: Fix VS Code Extension Icon and License

## Problem Analysis

### 1. Activity Bar Icon Not Showing

**Root Cause**: The activity bar icon path in `js/vscode/package.json` line 37 is:
```json
"icon": "../../assets/icons/compose_codeicon.svg"
```

This path points to `/workspaces/semio/assets/icons/compose_codeicon.svg`, which is OUTSIDE the `js/vscode` package directory. When `vsce package` bundles the extension, it only includes files within the extension folder (or explicitly included).

The `.vscodeignore` has:
- Line 21: `assets/**` - excludes all assets
- Line 35: `!assets/icons/**` - includes back, but this refers to `js/vscode/assets/icons/` which doesn't exist

**Solution**: Copy the icon file to `js/vscode/icons/compose_codeicon.svg` and update the path in package.json to `./icons/compose_codeicon.svg`.

### 2. LICENSE.md Not Recognized

**Root Cause**: The `.vscodeignore` file at line 31 has `*.md` which excludes ALL markdown files, including `LICENSE.md`. The `vsce` tool looks for `LICENSE`, `LICENSE.md`, or `LICENSE.txt` but since `*.md` pattern excludes it, the license file is not included.

**Solution**: Add `!LICENSE.md` to `.vscodeignore` after line 31 to explicitly include the license file.

## Implementation Steps

1. Create `js/vscode/icons/` directory
2. Copy `assets/icons/compose_codeicon.svg` to `js/vscode/icons/compose_codeicon.svg`
3. Update `js/vscode/package.json` line 37 to use `./icons/compose_codeicon.svg`
4. Update `js/vscode/.vscodeignore` to add `!LICENSE.md` after `*.md`
5. Test by running `npm run package` in `js/vscode` directory
6. Verify the warning is gone and icon is included

## Files to Modify

- `js/vscode/package.json` - update icon path
- `js/vscode/.vscodeignore` - add LICENSE.md exception
- `js/vscode/icons/compose_codeicon.svg` - new file (copy from assets)
