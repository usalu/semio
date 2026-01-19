# Plan: Fix Devcontainer PostAttach Command

## Problem

The `postAttachCommand` in `.devcontainer/devcontainer.json` fails with exit code 127 because:

1. The `code` or `code-insiders` CLI is not available in all container environments (only available when VS Code attaches)
2. The `semio.vsix` file doesn't exist - it needs to be built first

## Solution

1. Create a robust `post-attach.sh` script that:
   - Checks if the `code` CLI is available
   - Builds the `.vsix` file if it doesn't exist
   - Installs the extension only if both conditions are met
   - Exits gracefully without error when running in non-VS Code environments

2. Update `devcontainer.json` to use the script instead of inline command

## Implementation Steps

1. Create `.devcontainer/post-attach.sh` script
2. Update `devcontainer.json` to reference the script
3. Make the script handle edge cases gracefully

## Success Criteria

- Devcontainer starts without errors in all environments
- VS Code extension is installed when VS Code is attached
- Non-VS Code environments (CLI, headless) don't fail
