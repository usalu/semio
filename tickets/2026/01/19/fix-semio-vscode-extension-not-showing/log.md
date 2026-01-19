# Log: Fix semio-repo VSCode Extension Not Showing

## 2026-01-19

### Issue Identified
The post-attach script attempted to install the semio extension but the vsix file was not found at `js/vscode/semio.vsix`.

### Analysis
- Reviewed `js/vscode/package.json`
- Build scripts identified:
  - `npm run build` - compiles the extension
  - `npm run package` - creates `semio.vsix` using vsce

### Solution
To fix this, run the following commands:

```bash
cd js/vscode
npm run build
npm run package
```

This will create the `semio.vsix` file. Then run:

```bash
code --install-extension js/vscode/semio.vsix
```

Or reload the devcontainer which will trigger the post-attach script to install it automatically.
