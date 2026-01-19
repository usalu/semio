# Summary: Fix semio-repo VSCode Extension Not Showing

## Issue
The semio-repo VSCode extension was not showing in the activity bar after devcontainer setup. The post-attach script (`post-attach.sh`) tried to install the extension but the `.vsix` file didn't exist.

## Root Cause
The extension package file `js/vscode/semio.vsix` was not built/present in the repository.

## Solution
Build and package the extension manually:

```bash
cd js/vscode
npm run build
npm run package
code --install-extension semio.vsix
```

Or simply rebuild the devcontainer after building the vsix, as the post-attach script will automatically install it.

## Status
Manual intervention required - user needs to run the build commands.
