---
goal: AI-OPTIMIZED-REPO/SANDBOXED-REPO/ZERO-TOUCH-DEVCONTAINER
---

# Ticket

## Summary

Cleared VS Code extension caches during uninstall and documented the updated cache reset behavior.

## Changes

- .devcontainer/post-attach.sh: clear VS Code CachedProfilesData extension caches during uninstall.
- README.md: document VS Code cache cleanup in devcontainer extension flow.
- AGENTS.md: document VS Code cache cleanup in SRS, UI/UX, and codebase sections.

## Log

- 2026-02-06: Added VS Code extension cache cleanup alongside Cursor cache removal to ensure uninstall applies on window reload.
- 2026-02-06: Ticket close GitHub label update warned that repo/repo label was missing on issue 414.

## Todos

Plan:

1. Confirm uninstall flow clears VS Code caches in addition to Cursor caches.
2. Update devcontainer post-attach to remove VS Code extension caches on uninstall.
3. Update README.md and AGENTS.md to document VS Code cache cleanup.
4. Record changes and close the ticket.

## Plan

- Verify uninstall path clears VS Code cache files on reload.
- Update post-attach cleanup to include VS Code CachedProfilesData caches.
- Update README.md and AGENTS.md with VS Code cache invalidation details.
