# Micro-Commit Prepare Hook Fail-Open Audit

## Finding

The installed `prepare-commit-msg` hook invoked the repo MCP client with `micro-commit` command arguments. That binary accepts no command arguments and exited with status 1. The hook used `exec`, so Git aborted every commit while micro-commit state was active.

## Resolution

The hook now invokes the pinned Bun workspace entrypoint (`📜️script.ts micro-commit prepare-commit-msg`) rather than the MCP client. If Bun is unavailable or message refresh fails, it removes only stale micro-commit preparation state, restores an empty GitKraken template, preserves the user-provided message, and exits 0.

## Verification

- The installed hook refreshed the active current draft and exited 0.
- A temporary repository with a deliberately failing Bun executable committed `Manual Commit` successfully through the hook; its stale active marker was removed.
- `git diff --check` passed.
