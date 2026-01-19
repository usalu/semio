# Summary: Fix Devcontainer PostAttach Command

Fixed the devcontainer `postAttachCommand` that was failing with exit code 127 when `code` CLI was unavailable.

## Changes

1. Created `.devcontainer/post-attach.sh` - A robust script that:
   - Detects `code` or `code-insiders` CLI availability
   - Installs the extension only when CLI and `.vsix` file are present
   - Exits gracefully (exit code 0) in headless/CLI environments
   - Provides informative messages for each scenario

2. Updated `.devcontainer/devcontainer.json`:
   - Changed inline command to use the new script

## Files Modified

- `.devcontainer/post-attach.sh` (created)
- `.devcontainer/devcontainer.json` (updated)

## Result

The devcontainer now starts successfully in all environments:
- VS Code: Extension is auto-installed when available
- Headless/CLI: Gracefully skips installation without errors
