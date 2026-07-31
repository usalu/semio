# Ticket

## Plan

- Uninstall any existing repo extension before install via CLI and extensions directory cleanup.
- Clear Cursor extension caches to avoid stale metadata.
- Update README.md and AGENTS.md to document uninstall-first flow and cache invalidation.

## Todos

Plan:

1. Remove any existing repo extension via IDE CLIs and extensions directory cleanup before install.
2. Clear Cursor extension caches so updated engines and manifests are reloaded.
3. Update README.md and AGENTS.md to document uninstall-first and cache invalidation behavior.
4. Record changes and summary in ticket.md and close the ticket.

## Changes

- .devcontainer/post-attach.sh: uninstall repo extension before install across IDE CLIs and extensions directories, clear Cursor caches, and replace extensions.json entries on install.
- README.md: document uninstall-first extension install flow and cache cleanup in devcontainer docs.
- AGENTS.md: document uninstall-first extension install and cache cleanup in SRS, UI/UX, and codebase sections.

## Log

- 2026-02-06: Added uninstall-before-install path for the repo extension, including extensions directory cleanup, extensions.json replacement, and Cursor cache invalidation.
- 2026-02-06: Ticket close GitHub label update warned that repo/repo label was missing on issue 218.

## Summary

Uninstall repo extension before installs, clear Cursor caches, and document uninstall-first devcontainer flow.
