# Fix Hook Mapping Spec

## Goal

Update hook event mapping to match exact specification for all native clients.

## Changes

1. Renamed `tool.calling` → `tool.starting` (HookToolCalling → HookToolStarting)
2. Removed `notification` event (HookNotification)
3. Added new events: `task.plan`, `task.starting`, `task.ended`, `terminal.starting`, `terminal.ended`
4. Updated all 7 native config generators (copilot, cursor, windsurf, claude-code, droid, codex, antigravity)
5. Updated copilot-instructions.md with new hook categories (task operations, terminal operations)
6. Updated all 32+ hook tests to match new event names and config structures
7. Updated dispatchHook with terminal.starting blocking support
8. Updated vsCodeEventFromHookEvent to map all new events
9. Updated hookCommand help text

## Summary

- 15 hook events: commit.starting, commit.ended, agent.starting, agent.ended, prompt.submit, compacting, tool.starting, tool.ended, task.plan, task.starting, task.ended, code.reading, code.edited, terminal.starting, terminal.ended
- All 32+ tests pass
- CLI properly rejects old event names (tool.calling, notification)

## Status

- [x] Read current implementation
- [x] Update HookEvent constants and AllHookEvents
- [x] Update dispatchHook, vsCodeEventFromHookEvent, formatVSCodeHookOutput
- [x] Update all config generators
- [x] Update copilot-instructions.md
- [x] Update tests
- [x] Run tests and verify
