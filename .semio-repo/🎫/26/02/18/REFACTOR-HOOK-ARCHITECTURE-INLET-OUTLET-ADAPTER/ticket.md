---
goal: DEVOPS
---

# Ticket

## Summary
Refactored hook architecture to inlet adapter → neutral implementation → outlet adapter pattern. CLI accepts native client events (VSCode PreToolUse, Windsurf pre_read_code, Cursor beforeReadFile, Claude-compatible PreToolUse) and resolves them to neutral events via per-client inlet adapters. Added HookAgentToolCodeEditing event, ToolKind classifier, 5 platform-specific resolvers, ResolveHookEvent master function. Updated all 10 config generators to fire native events. All 14 neutral events, 80+ test cases passing.

## Changes
- semio-repo/cli/main.go: Added HookAgentToolCodeEditing constant, ToolKind type, classifyTool function, inlet adapter functions (resolveCopilotEvent, resolveCursorEvent, resolveWindsurfEvent, resolveClaudeCompatibleEvent, ResolveHookEvent), updated vsCodeEventFromHookEvent outlet, dispatchHook, hookCommand, trackHookInOpenTicket, all 10 config generators
- semio-repo/cli/main_test.go: Updated all existing hook tests for code.editing and native events, added TestClassifyTool, TestResolveHookEvent (51 cases), TestResolvePreToolUse, TestResolvePostToolUse
- .github/copilot-instructions.md: Updated to use native event names (SessionStart, PreToolUse, PostToolUse) with inlet adapter documentation

## Log
- Explored codebase structure and hook implementation
- Added HookAgentToolCodeEditing constant and AllHookEvents entry
- Added ToolKind type and classifyTool function
- Added per-client inlet adapter resolvers
- Added ResolveHookEvent master function
- Updated hookCommand to use ResolveHookEvent
- Updated vsCodeEventFromHookEvent outlet adapter
- Updated dispatchHook and trackHookInOpenTicket for code.editing
- Updated all 10 config generators to fire native events
- Updated all existing tests
- Added 4 new test functions with 80+ test cases
- Fixed test expectations for replace_string_in_file and run_in_terminal classification
- All tests passing, CLI rebuilt

## Todos
- [x] Add HookAgentToolCodeEditing event
- [x] Add ToolKind and classifyTool
- [x] Add inlet adapter resolvers
- [x] Update hookCommand
- [x] Update outlet adapter
- [x] Update config generators
- [x] Update existing tests
- [x] Add new inlet adapter tests
- [x] Update copilot-instructions.md
- [x] Build and verify all tests pass

## Plan
Inlet adapter → neutral implementation → outlet adapter. Native events from each platform (VSCode PascalCase, Cursor camelCase, Windsurf snake_case, Claude PascalCase) are resolved to neutral HookEvents by classifying tools by name (plan, code_search, code_edit, terminal, generic). Config generators fire native events; the CLI resolves them.
