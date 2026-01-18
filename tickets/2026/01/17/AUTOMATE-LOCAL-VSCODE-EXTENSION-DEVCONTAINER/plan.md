# Plan: Automate Local VSCode Extension Installation in Devcontainer

## Overview
The semio VSCode extension at `js/vscode/` needs to be automatically installed when the devcontainer is created or started.

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
   - Script: `"package": "vsce package --out semio.vsix"`
   - This creates a `semio.vsix` file in the extension directory

3. **Update `.devcontainer/post-create.sh`** to:
   - Build the VSCode extension
   - Package it into a `.vsix` file
   - Install it using `code --install-extension`

4. **Add `.vsix` to `.gitignore`** if not already present

## Files to Modify
- `js/vscode/package.json` - Add vsce dependency and package script
- `.devcontainer/post-create.sh` - Add extension build/install steps
- `.gitignore` - Add `*.vsix` pattern (if needed)
