---
goal: DEVOPS
---

# Ticket

## Summary

Added HookAgentToolSearched (agent.tool.searching.ended) event for post-tool-use of search tools. resolvePostToolUse now returns HookAgentToolSearched for ToolKindCodeSearch. Added HookResultAgentToolSearched struct, RunHook case, trackHookInOpenTicket case, and updated windsurf post_read_code resolver. All test expectations updated.

## Changes

- repo/cli/main.go: Added HookAgentToolCodeEditing constant, ToolKind type, classifyTool function, inlet adapter functions (resolveCopilotEvent, resolveCursorEvent, resolveWindsurfEvent, resolveClaudeCompatibleEvent, ResolveHookEvent), updated vsCodeEventFromHookEvent outlet, dispatchHook, hookCommand, trackHookInOpenTicket, all 10 config generators
- repo/cli/main_test.go: Updated all existing hook tests for code.editing and native events, added TestClassifyTool, TestResolveHookEvent (51 cases), TestResolvePreToolUse, TestResolvePostToolUse
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
- [ ] Add HookAgentToolSearched (agent.tool.searching.ended) event
- [ ] Wire HookAgentToolSearched into resolvePostToolUse for ToolKindCodeSearch
- [ ] Update tests for searching.ended

## Plan

Inlet adapter → neutral implementation → outlet adapter. Native events from each platform (VSCode PascalCase, Cursor camelCase, Windsurf snake_case, Claude PascalCase) are resolved to neutral HookEvents by classifying tools by name (plan, code_search, code_edit, terminal, generic). Config generators fire native events; the CLI resolves them.

### Phase 2: Add searching.ended event

list_dir and other search tools fall back to generic agent.tool.ended on post-tool-use because resolvePostToolUse has no case for ToolKindCodeSearch. Add HookAgentToolSearched = "agent.tool.searching.ended", add to AllHookEvents, and handle ToolKindCodeSearch in resolvePostToolUse.
