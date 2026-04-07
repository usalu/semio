# Ticket

## Todos
# Plan

- Review devcontainer Playwright cache mounts and environment to see why the cache resets on window reload.
- Update devcontainer cache path and startup steps so Playwright browsers persist across reloads.
- Update dev docs (README.md, AGENTS.md) with the Playwright caching mechanism.
- Verify changes by re-reading relevant configs and documenting next steps in the ticket log and summary.

## Changes

## Log
# Log: Add Playwright Cache Volume Mount

## 2026-01-19

### Analysis
- Reviewed `.devcontainer/devcontainer.json`
- Found that `PLAYWRIGHT_BROWSERS_PATH` is already set to `/home/vscode/.cache/ms-playwright`
- Node modules are cached via volume mount: `source=${localWorkspaceFolderBasename}-node_modules,target=${containerWorkspaceFolder}/node_modules,type=volume`
- Playwright cache was NOT persisted

### Implementation
Added volume mount for Playwright cache:
```json
"source=${localWorkspaceFolderBasename}-ms-playwright,target=/home/vscode/.cache/ms-playwright,type=volume"
```

This ensures Playwright browsers are persisted across container rebuilds, matching the pattern used for node_modules.

## Update
- Reopened ticket with latest prompt about Playwright cache reinstall after reload.
- Reviewed devcontainer config and Playwright install steps.
- Planning to adjust Playwright cache path and startup handling to persist across reloads.

## Work
- Moved Playwright cache path to the workspace node_modules cache directory in devcontainer env.
- Ensured Playwright install targets the shared cache path during devcontainer provisioning.
- Updated README and AGENTS docs to document the cache mechanism.

## Close
- Ticket closed with updated devcontainer Playwright cache handling and documentation updates.

## Summary
# Summary

- Updated devcontainer Playwright cache path to the workspace node_modules cache and aligned provisioning to install into it.
- Documented Playwright cache behavior in README and AGENTS dev docs.
