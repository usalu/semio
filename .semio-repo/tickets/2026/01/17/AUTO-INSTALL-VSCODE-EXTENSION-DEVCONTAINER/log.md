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
