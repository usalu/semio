---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Added event-specific data to all agent hook logs. Changed HookResultBase.Raw json tag to json:"-" to prevent duplication. Added Data json.RawMessage field to HookLogEntry. Updated logHook to marshal the full HookResult into entry.Data so every log now contains structured event-specific fields (tool name/input, prompt, command, query/include/exclude, path/old/new). Updated 3 hook log tests to verify data field. All pass.
## Changes

## Log
- Opened ticket

## Todos
- [ ] Change `HookResultBase.Raw` json tag to `json:"-"`
- [ ] Add `Data json.RawMessage` to `HookLogEntry`
- [ ] Update `logHook` to marshal result into `entry.Data`
- [ ] Update existing hook log tests to verify event-specific data in `data` field
- [ ] Run hook tests
- [ ] Close ticket

## Plan
1. Change `HookResultBase.Raw any` tag from `json:"raw,omitempty"` to `json:"-"` to prevent duplication in log
2. Add `Data json.RawMessage json:"data,omitempty"` to `HookLogEntry`
3. In `logHook`, marshal `result` into `entry.Data`
4. Update `TestHookLogging` to verify `entry.Data` has `agent.started` specific fields (session, client, LLM, parent)
5. Update `TestHookLoggingStdinInput` to verify `entry.Data` has `name` and `input` for tool.starting
6. Update `TestHookLoggingToolBlocked` to verify `entry.Data` is present with `allowed: false`
7. Run `go test -run TestHook` to verify all pass
