# Plan: Auto-install VSCode Extension in Devcontainer

## Problem

The semio VSCode extension (`js/vscode/`) should be automatically installed when the devcontainer starts. Currently, users have to manually install it using "Developer: Install Extension From Location...".

The current approach in `post-create.sh` uses:
```bash
code --install-extension semio-repo.vsix --force || true
```

This doesn't work reliably because:
1. The `code` CLI may not be available during container creation
2. VSCode Server isn't running when postCreateCommand executes

## Solution

Use VSCode's `postAttachCommand` instead of `postCreateCommand` for the extension installation. The `postAttachCommand` runs after VSCode attaches to the container, ensuring the `code` CLI is available.

### Changes Required

1. **Modify `.devcontainer/devcontainer.json`**:
   - Add `postAttachCommand` to install the extension after VSCode attaches

2. **Modify `.devcontainer/post-create.sh`**:
   - Remove the extension installation step (keep build and package steps)

3. **Use `postAttachCommand` inline**:
   - Install the extension after VS Code attaches without adding new files

## Implementation Steps

1. Split extension installation from post-create.sh
2. Add postAttachCommand to devcontainer.json
