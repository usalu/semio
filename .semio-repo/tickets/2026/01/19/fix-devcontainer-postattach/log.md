# Log: Fix Devcontainer PostAttach Command

## 2026-01-19

### Initial Investigation

- Received error from devcontainer startup:
  ```
  code or code-insiders is not installed
  postAttachCommand from devcontainer.json failed with exit code 127
  ```

- Root cause identified:
  1. `postAttachCommand` runs: `bash -lc "cd js/vscode\ncode --install-extension semio.vsix --force"`
  2. The `code` CLI is only available when VS Code attaches, not in CLI/headless environments
  3. The `semio.vsix` file doesn't exist in the `js/vscode/` directory

### Solution Design

Creating a robust `post-attach.sh` script that gracefully handles:
- Missing `code` CLI (non-VS Code environments)
- Missing `.vsix` file (needs to be built)
- Proper error handling and informative output

### Implementation

1. Created `.devcontainer/post-attach.sh` script that:
   - Checks for `code` or `code-insiders` CLI availability
   - Only attempts extension installation if CLI is available
   - Provides informative messages for each scenario
   - Exits successfully (exit code 0) in all cases

2. Updated `devcontainer.json` to use the script:
   - Changed from inline command to: `"postAttachCommand": "bash .devcontainer/post-attach.sh"`

### Testing

- Ran the script manually - works correctly
- Detects VS Code CLI when available
- Gracefully handles missing `.vsix` file with helpful message
- Would exit gracefully in headless environments (no error)

### Note

Found a pre-existing issue: The VS Code extension package.json has an npm-style scoped name (`@semio-repo/vscode`) which is invalid for VS Code extensions. This causes `npm run package` to fail. This is a separate issue from the devcontainer fix.
