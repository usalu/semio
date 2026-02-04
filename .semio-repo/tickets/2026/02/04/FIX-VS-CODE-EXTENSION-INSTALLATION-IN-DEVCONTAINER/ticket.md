# Ticket

## Summary

Fixed bug in post-attach.sh where VSCODE_IPC_HOOK_CLI socket path was incorrectly treated as a CLI binary. Added -f check to ensure only regular files (not sockets) are used as CLI commands.
## Problem

The semio-repo VS Code extension wasn't being installed correctly in the devcontainer despite the post-attach script reporting success:
- The script showed "Extension installed via /home/vscode/.vscode-server/extensions"
- But the extension wasn't activating or visible in VS Code

## Root Cause

The `find_working_clis()` function in post-attach.sh had a bug in the CLI detection logic:

1. `VSCODE_IPC_HOOK_CLI` environment variable contains an IPC **socket** path (e.g., `/tmp/user/1000/vscode-ipc-*.sock`), not a CLI binary
2. The check `[ -x "$cli_path" ]` passes for sockets because sockets can have execute permission
3. When the script tried to run `"$cli_path" --install-extension`, it silently failed because you can't execute a socket
4. The script fell back to manually copying files to the extensions directory, but this method requires VS Code to reload to detect the new extension

## Solution

Added a check for regular file (`-f`) in addition to executable (`-x`):

```bash
# Before (buggy):
if [ -x "$cli_path" ]; then

# After (fixed):
if [ -f "$cli_path" ] && [ -x "$cli_path" ]; then
```

This ensures socket files (which have executable permission but are not regular files) are skipped, allowing the script to find and use the actual CLI binaries at paths like `/vscode/vscode-server/bin/linux-x64/*/bin/remote-cli/code`.

## Changes

- `.devcontainer/post-attach.sh`: Added `-f` check to prevent sockets from being treated as CLI binaries

## Log

- Investigated extension installation by checking extensions directory, extensions.json, and VS Code logs
- Found extension was installed but not activating
- Traced the issue to `VSCODE_IPC_HOOK_CLI` containing socket path instead of CLI path
- Verified fix works by testing CLI detection with the updated logic

## Todos

- [x] Investigate why extension doesn't activate
- [x] Identify root cause
- [x] Fix post-attach.sh

## Plan

N/A - simple bug fix
