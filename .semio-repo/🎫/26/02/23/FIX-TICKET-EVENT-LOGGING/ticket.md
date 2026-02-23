---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Fixed event logging
## Changes

## Log

## Todos

## Plan

1. Add `case HookAgentToolSearched` to `dispatchHook` in [semio-repo/cli/main.go](semio-repo/cli/main.go) — fix "unknown hook event: agent.tool.searching.ended" error
2. Remove empty-prompt stub creation in `trackHookInOpenTicket` — prompts must only come from `agent.prompt.submitting` hook events
3. Remove filter that drops events lacking file/pattern data — all events should be recorded
4. Build and run tests to confirm fixes

## Todos

- [x] Add HookAgentToolSearched case to dispatchHook
- [ ] Fix trackHookInOpenTicket: remove empty-prompt stub
- [ ] Fix trackHookInOpenTicket: remove file/pattern filter
- [ ] Build and test
