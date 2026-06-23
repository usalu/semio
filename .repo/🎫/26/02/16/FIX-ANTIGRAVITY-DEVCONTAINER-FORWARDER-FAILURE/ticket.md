---
goal: aioptimizedrepo/sandboxedrepo/zerotouchdevcontainer
---

# Ticket

## Summary

Enabled concurrent multi-IDE devcontainer use by persisting Cursor/Antigravity server homes, extending ownership repair, and making post-attach extension setup lock-protected and non-destructive across VS Code, Antigravity, Windsurf, and Cursor.
## Changes
- Added `socat` and `netcat-openbsd` to `.devcontainer/Dockerfile` apt dependencies so Antigravity/devcontainer forwarder subprocesses have required networking binaries available in container runtime.
- Added persistent devcontainer mounts for `/home/vscode/.cursor-server` and `/home/vscode/.antigravity-server` in `.devcontainer/devcontainer.json`.
- Added ownership normalization for `/home/vscode/.cursor-server` and `/home/vscode/.antigravity-server` in `.devcontainer/post-start.sh`.
- Added directory initialization for `/home/vscode/.cursor-server` and `/home/vscode/.antigravity-server` in `.devcontainer/Dockerfile`.
- Refactored `.devcontainer/post-attach.sh` extension install flow to be non-destructive and multi-editor-safe:
  - Added lock file serialization (`/tmp/compose-post-attach-extension-install.lock`) for concurrent attach sessions.
  - Removed uninstall/purge behavior that removed extensions and caches across IDE server homes.
  - Removed direct filesystem rewrite of extension directories and `extensions.json`.
  - Kept install-only `--install-extension --force` flow per detected CLI.
  - Converted extension install failures into warnings instead of hard attach failures.

## Log
- Opened ticket under `aioptimizedrepo/sandboxedrepo/zerotouchdevcontainer`.
- Investigated failure logs:
  - Remote server install succeeds.
  - `forwardPort` fails with `handleClient Error: subprocess terminated immediately with return code 127`.
  - Renderer WebSocket closes with status `1006`.
  - Remote FS provider registration fails (`ENOPRO`) as downstream symptom.
- Validated container runtime in current environment has no `socat`/`nc` binaries (`which socat`, `which nc`, `which netcat` returned empty).
- Applied Dockerfile dependency fix.
- Attempted ticket closure via CLI (`ticket close` and direct `graphql ticketClose`) but closure is blocked by a repo CLI validation/runtime bug returning `at least one file is required` despite provided file list.
- Updated ticket prompt via `ticket change` for multi-editor concurrent attach scope.
- Validated script syntax:
  - `bash -n .devcontainer/post-attach.sh`
  - `bash -n .devcontainer/post-start.sh`
  - `bash -n .devcontainer/post-create.sh`

## Todos
- Rebuild devcontainer image and reopen workspace from each IDE (VS Code, Antigravity, Windsurf, Cursor).
- Validate all four IDEs can stay attached simultaneously to the same running container.
- Confirm no attach session uninstalls or invalidates compose extension state for another active IDE.
- Confirm no `forwardPort ... return code 127`, no repeated WebSocket `1006`, and no `ENOPRO` remote filesystem errors in attach logs.

## Plan
- Ensure forwarding runtime dependencies and editor server-home persistence are configured in devcontainer.
- Make post-attach extension setup concurrency-safe and non-destructive across IDEs.
- Rebuild and validate multi-IDE concurrent attach lifecycle.
