---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Fixed three issues: (1) Rebuilt stale CLI binary resolving agent.tool.searching.ended dispatch error that blocked PostToolUse hooks. (2) Removed ghost-prompt stub in trackHookInOpenTicket so prompts only come from agent.prompt.submitting hooks; saves session metadata before returning. (3) Isolated TestTicketOpenContinueKeyword with tmpDir to prevent real-filesystem interference. All tests pass.

## Changes

- [repo/cli/main.go](repo/cli/main.go): Removed ghost-prompt stub in `trackHookInOpenTicket` — when no prompt exists, saves ticket (persisting session+metadata) then returns early instead of creating an empty `TicketSessionPrompt`; events only attach to real `agent.prompt.submitting`-originated prompts
- [repo/cli/main_test.go](repo/cli/main_test.go): Added tmpDir+git isolation to `TestTicketOpenContinueKeyword` so it doesn't pick up real workspace tickets
- Rebuilt `repo/cli/cli` binary — dispatches `HookAgentToolSearched` (`agent.tool.searching.ended`) correctly, no more "unknown hook event" blocking error

## Log

## Todos

## Plan

1. Add `case HookAgentToolSearched` to `dispatchHook` in [repo/cli/main.go](repo/cli/main.go) — fix "unknown hook event: agent.tool.searching.ended" error
2. Remove empty-prompt stub creation in `trackHookInOpenTicket` — prompts must only come from `agent.prompt.submitting` hook events
3. Remove filter that drops events lacking file/pattern data — all events should be recorded
4. Build and run tests to confirm fixes

## Todos

- [x] Add HookAgentToolSearched case to dispatchHook
- [ ] Fix trackHookInOpenTicket: remove empty-prompt stub
- [ ] Fix trackHookInOpenTicket: remove file/pattern filter
- [ ] Build and test
