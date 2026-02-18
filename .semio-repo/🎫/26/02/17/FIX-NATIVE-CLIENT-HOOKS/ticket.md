---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Claude Code hooks fixed.
## Changes

- Regenerated `.claude/settings.json` with proper hooks via `configure` command
- Deleted `.claude/hooks.json` (was wrong format, ignored by Claude Code)

## Log

- Ran `./semio-repo/cli/cli configure` → regenerated all client hook configs
- Deleted `.claude/hooks.json` via semio-repo file_delete
- Verified `git stash` blocked correctly via stdin JSON parsing
- All hook tests pass (TestGenerateClaudeCodeConfig, TestExtractCommandFromStdin, etc.)

## Todos

- [x] Run configure to regenerate .claude/settings.json with proper hooks
- [x] Delete .claude/hooks.json (wrong format, ignored by Claude Code)
- [x] Verify hook CLI correctly parses stdin JSON from Claude Code

## Plan

Root cause: .claude/hooks.json had old wrong format (strings instead of {type,command} objects, PromptSubmit/SessionStop instead of UserPromptSubmit/SessionEnd) and Claude Code only reads from .claude/settings.json. Fix: run configure, delete hooks.json.
