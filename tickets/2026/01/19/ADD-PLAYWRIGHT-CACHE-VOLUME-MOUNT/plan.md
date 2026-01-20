# Plan: Add Playwright Cache Volume Mount

## Context
The devcontainer.json already has:
- Volume mount for `node_modules` (line 80)
- `PLAYWRIGHT_BROWSERS_PATH` env var set to `/home/vscode/.cache/ms-playwright` (line 91)

However, the Playwright browser cache is not persisted between container rebuilds.

## Task
Add a volume mount for the Playwright cache directory to persist browsers across container rebuilds.

## Implementation
1. Add a new volume mount entry in the `mounts` array:
   ```
   "source=${localWorkspaceFolderBasename}-ms-playwright,target=/home/vscode/.cache/ms-playwright,type=volume"
   ```

This will persist Playwright browsers the same way node_modules is persisted.
