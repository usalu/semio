# Log: Fix semio-repo VSCode Extension Not Showing

## 2026-01-19

### Issue Identified
The post-attach script attempted to install the semio extension but the vsix file was not found at `js/vscode/semio-repo.vsix`.

### Analysis
- Reviewed `js/vscode/package.json`
- Build scripts identified:
  - `npm run build` - compiles the extension
  - `npm run package` - creates `semio-repo.vsix` using vsce

### Solution
To fix this, run the following commands:

```bash
cd js/vscode
npm run build
npm run package
```

This will create the `semio-repo.vsix` file. Then run:

```bash
code --install-extension js/vscode/semio-repo.vsix
```

Or reload the devcontainer which will trigger the post-attach script to install it automatically.

### Verification
- Adjusted `js/vscode/extension.test.ts` to use the correct extension ID `usalu.semio-repo` (was `usalu.@semio-repo/vscode`) and to properly detect ticket folders (slugs) instead of looking for `.md` files directly.
- Compiled the test files using `npx vite build --config vite.test.config.ts`.
- Ran tests with `npm test` and verified **27 tests passing, 0 failing**.
- Confirmed that views are correctly registered in the test suite, which rules out logic errors in view registration.
