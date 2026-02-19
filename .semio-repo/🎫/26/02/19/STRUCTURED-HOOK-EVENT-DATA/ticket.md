---
goal: HOOKS/EVENTDATA
---

# Ticket

## Summary

Refactored HookResult to return structured event-specific data matching the YAML schema. Added HookPlanStep struct, 15 extraction functions, populateEventData dispatcher, and 30+ tests. Each event type now returns its specific fields (session, timestamp, client, parent, sha, prompt, chat, name, input, response, steps, query, include, exclude, path, old, new, all, command, pid, terminated, stdout, stderr) plus a raw field with the original input JSON.
## Changes

- Extended HookResult with event-specific fields (session, timestamp, client, parent, sha, prompt, chat, name, input, response, steps, query, include, exclude, path, old, new, all, command, pid, terminated, stdout, stderr, raw)
- Added HookPlanStep struct for plan.updating steps
- Added extraction functions for each event type from client stdin JSON
- Modified dispatchHook to populate event-specific fields
- Updated hookCommand to include session/timestamp/client from HookContext
- Updated all tests

## Log

## Todos

- [x] Add HookPlanStep & extend HookResult
- [x] Add extraction functions for event data
- [x] Update dispatchHook to populate fields
- [x] Update hookCommand output with event data
- [x] Update tests for new HookResult fields
- [x] Build and run tests
- [x] Close ticket

## Plan

1. Extend HookResult with all event-specific fields + Raw
2. Add HookPlanStep struct
3. Add extraction functions for tool input, tool response, plan steps, code search, code edit, terminal, chat
4. Modify dispatchHook to populate fields per event type
5. Update hookCommand to pass session/timestamp/client to result
6. Update tests to verify new fields
7. Build, test, close
