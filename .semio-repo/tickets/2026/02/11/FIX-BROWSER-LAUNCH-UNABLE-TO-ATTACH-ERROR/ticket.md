---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-FILE-MECHANISM
---

# Ticket

## Summary

Fix browser attach failure while keeping Chrome as external browser
## Changes

- `/workspaces/semio/.vscode/launch.json`: Replaced `type: "chrome"` with `type: "node"` for app dev launcher entries that run `npx nx ...` and rely on `serverReadyAction`
- `/workspaces/semio/.vscode/settings.json`: Added `workbench.externalBrowser` set to `chrome`

## Log

- Reopened existing ticket `2026/02/11/FIX-BROWSER-LAUNCH-UNABLE-TO-ATTACH-ERROR` because it already covers this exact issue
- Collected repo context with `./semio-repo/cli/cli tree browser`
- Found root issue in `.vscode/launch.json`: app dev launchers used `type: "chrome"` while launching node processes (`npx nx ...`), which triggers browser attach flow and fails with `unable to attach browser`
- Refactored all affected app dev entries to `type: "node"` so debugging no longer depends on browser attachment
- Kept browser opening via `serverReadyAction` and pinned VS Code external browser preference to Chrome to preserve Chrome as primary browser target
- Verified there are no remaining `type: "chrome"` launch entries in `.vscode/launch.json`

## Todos

- [x] Reopen matching ticket
- [x] Inspect launch and browser settings
- [x] Refactor launch configurations to avoid browser attachment
- [x] Keep Chrome as primary browser target
- [x] Verify modified launcher definitions

## Plan

1. Replace browser-attached dev launchers with non-attached node launchers.
2. Preserve URL auto-open behavior with `serverReadyAction`.
3. Ensure external browser target remains Chrome.
4. Close the ticket with updated files.
