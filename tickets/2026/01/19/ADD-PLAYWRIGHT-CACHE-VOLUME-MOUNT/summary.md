# Summary: Add Playwright Cache Volume Mount

Added a volume mount for Playwright browser cache to persist browsers across container rebuilds.

## Change
Added to `.devcontainer/devcontainer.json` mounts array:
```json
"source=${localWorkspaceFolderBasename}-ms-playwright,target=/home/vscode/.cache/ms-playwright,type=volume"
```

This matches the existing pattern used for node_modules caching and uses the already-configured `PLAYWRIGHT_BROWSERS_PATH` environment variable.
