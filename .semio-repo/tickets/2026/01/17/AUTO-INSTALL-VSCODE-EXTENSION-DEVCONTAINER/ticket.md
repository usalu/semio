# Ticket

## Todos
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

## Changes

## Log
# Log

## Analysis

Reviewed the devcontainer setup:
- `.devcontainer/devcontainer.json` - main devcontainer configuration
- `.devcontainer/post-create.sh` - runs after container creation, includes extension build/install
- `.devcontainer/post-start.sh` - runs on container start

Current extension installation in `post-create.sh` (lines 40-45):
```bash
echo "Building and installing semio VSCode extension..."
cd js/vscode
npm run build
npm run package
code --install-extension semio-repo.vsix --force || true
cd ../..
```

The `code --install-extension` command fails because VSCode isn't attached yet during `postCreateCommand`.

## Solution

The fix is to use `postAttachCommand` which runs after VSCode attaches to the container. This ensures the `code` CLI is available and can install extensions properly.

## Actions

- Added SPDX headers to devcontainer scripts for compliance.
- Added a post-attach installation command to install the packaged VS Code extension after attach.
- Prepared doc updates to capture the devcontainer extension install mechanism.

## Updates

- Added SPDX headers to `.devcontainer/post-create.sh` and `.devcontainer/post-start.sh`.
- Added `postAttachCommand` to `.devcontainer/devcontainer.json` for VS Code extension installation.
- Documented the devcontainer extension auto-install mechanism in README and AGENTS.

## Summary
# Summary

- Added devcontainer post-attach install of the packaged VS Code extension and SPDX headers for devcontainer scripts.
- Documented the devcontainer extension auto-install mechanism in README and AGENTS.

# Tests

- Not run (per instructions).
