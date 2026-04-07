---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Fixed duplicate hook execution by preventing Claude Code hook generation when Cursor hooks exist. Modified generateClaudeCodeConfig to skip hook generation when .cursor/hooks.json exists, preventing Cursor from loading hooks from both .cursor/hooks.json and .claude/settings.json. Hooks now execute only once as cursor-chat.

## Plan

Cursor automatically loads hooks from third-party tools like Claude Code (per Cursor docs). When both .cursor/hooks.json and .claude/settings.json have hooks, Cursor executes both, causing duplication.

Solution: Modify `generateClaudeCodeConfig` to check if `.cursor/hooks.json` exists. If it does, skip generating hooks in `.claude/settings.json` to prevent duplicate execution. Preserve other settings (permissions, etc.) in `.claude/settings.json`.

## Todos

- [x] Modify `generateClaudeCodeConfig` to check for `.cursor/hooks.json` existence
- [x] Skip hook generation when Cursor hooks are present
- [x] Preserve existing permissions and other settings in `.claude/settings.json`
- [x] Update tests to verify behavior
- [x] Run configure to regenerate config files
- [x] Verify hooks only execute once

## Changes

- Modified `generateClaudeCodeConfig` in `repo/cli/main.go` to skip hook generation when `.cursor/hooks.json` exists
- Updated test to verify hooks are not generated when Cursor hooks exist

## Log
