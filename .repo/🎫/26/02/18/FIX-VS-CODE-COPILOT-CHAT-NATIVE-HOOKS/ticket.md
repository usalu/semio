---
goal: HOOKS-VSCODE/FIX-HOOKS
---

# Ticket

## Summary

Fixed 'Cannot read properties of undefined (reading hookSpecificOutput)' in VS Code Copilot Chat v0.37.6. Root cause: formatVSCodeHookOutput returned {} for UserPromptSubmit/PostToolUse/Stop/SubagentStop/PreCompact events. VS Code _toHookResult strips continue/stopReason/systemMessage leaving empty object → output=undefined. The UserPromptSubmit onSuccess callback accesses d.hookSpecificOutput without null-checking d. Fix: always include hookSpecificOutput for all VS Code hook events. Updated tests. All hook tests pass.
## Plan

Root cause (confirmed by reading VS Code Copilot Chat extension source at `/home/vscode/.vscode-server/extensions/github.copilot-chat-0.37.6/dist/extension.js`):

1. Our hook outputs `{}` for UserPromptSubmit, Stop, SubagentStop, PreCompact, PostToolUse (no message)
2. VS Code `_toHookResult` strips `continue`/`stopReason`/`systemMessage` fields → empty `l = {}` → `output = undefined`
3. `JS({hookType:"UserPromptSubmit", onSuccess: u => { let d = u; let p = d.hookSpecificOutput?.additionalContext ... }})` crashes because `d` is `undefined` (no null check unlike SessionStart/PreToolUse callbacks)

Fix in `formatVSCodeHookOutput`:
- Always include `hookSpecificOutput` for ALL events that VS Code may access `.hookSpecificOutput` on
- UserPromptSubmit: `{"hookSpecificOutput": {"hookEventName": "UserPromptSubmit"}}` (+ additionalContext if message)
- PostToolUse: always include `{"hookSpecificOutput": {"hookEventName": "PostToolUse"}}` (+ additionalContext if message)
- Stop: always include `{"hookSpecificOutput": {"hookEventName": "Stop"}}`
- SubagentStop: use top-level `{}` (VS Code uses top-level decision/reason, no hookSpecificOutput)
- PreCompact: always include something to avoid undefined output

Update tests to match new behavior.

## Todos

- [ ] Fix formatVSCodeHookOutput to always include hookSpecificOutput for UserPromptSubmit, PostToolUse, Stop, PreCompact
- [ ] Update tests to reflect new output format
- [ ] Build CLI and verify manually
- [ ] Close ticket

## Changes

## Log
