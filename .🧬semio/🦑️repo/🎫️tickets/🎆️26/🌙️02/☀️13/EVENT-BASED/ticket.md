---
goal: repo
---

# Ticket

## Summary

Event-based architecture: shared repo/go with EventKind and payloads; CLI Emit() on all changing interactions; server /events, contributor_work, conflict warnings, Discord notifications, GitHub push cleanup

## Changes

- repo/go: events.go (EventKind, Event, payloads), emit.go (Emit)
- repo/go.work: cli, go, server
- repo/cli: repopkg import, Emit calls in CreateTicket, FinishTicket, ReopenTicket, TicketChange, GoalCreate/Close/Reopen/Change, ContributorAdd/Remove, TodoCreate/Delete
- repo/server: /events handler, contributor_work table, onCLIEvent (conflict check, Discord), onCommitEvent, handleGitHubPushEvent

## Log

## Todos

## Plan
