# Plan: Fix semio-repo VSCode Extension Not Showing

## Problem
The semio-repo VSCode extension is not showing after devcontainer setup. The post-attach script reports:
```
Extension file not found at js/vscode/semio.vsix
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
- [ ] Verify semio.vsix exists
- [ ] Install the extension
