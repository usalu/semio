---
goal: HOOKS
---

# Ticket

## Summary

Fixed IsToolBlocked false positives: replaced naive substring matching with segment-start matching. Commands like grep with git checkout in pattern are no longer blocked. Added splitCommandSegments and isCommandSegmentBlocked helpers. Added VS Code camelCase tool names to classifyTool. Added 7 false-positive test cases and 2 new unit test functions.
## Changes

## Log

## Todos

## Plan

### Root Cause
`IsToolBlocked` uses `strings.Contains` on the full command string, so `grep "git checkout" file.go` is blocked because "git checkout" appears in the grep arguments (false positive).

### Fix
1. Replace substring match with segment-start match: split command by shell operators (`&&`, `||`, `;`, `|`) and check if any segment **starts with** a blocked pattern.
2. Add camelCase VS Code tool names (`runInTerminal`, `editFiles`, `readFile`, etc.) to `classifyTool` so terminal commands are properly classified as `ToolKindTerminal`.
3. Add test cases for false positives.

### Files
- `semio-repo/cli/main.go`: `IsToolBlocked`, `classifyTool`
- `semio-repo/cli/main_test.go`: `TestIsToolBlocked`, `TestBlockedToolPatterns`

## Todos

- [x] Fix `IsToolBlocked` with segment-start matching
- [x] Add VS Code camelCase tool names to `classifyTool`
- [x] Add false-positive tests
- [x] Build and run tests
- [x] Close ticket
