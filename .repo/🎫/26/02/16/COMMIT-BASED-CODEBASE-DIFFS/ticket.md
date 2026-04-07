---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

## Changes

## Log

## Todos

## Plan

1. Add `InteractionFile` type (Path, ID, URI) and replace `Diff *TicketDiffs` with `Files []InteractionFile` on `Interaction`
2. Modify `CreateTicket`/`OpenTicket`/`ToolTicketOpen` to accept and store files
3. Modify `FinishTicket`/`ToolTicketClose` to stop computing diffs, just store files as `InteractionFile`
4. Update MCP handlers (`ticketOpen`, `ticketClose`) for new files parameter
5. Update CLI commands (`ticket open`/`close`) for new files parameter
6. Update GraphQL schema: add files to `TicketOpenInput`, `Interaction` type
7. Create commits infrastructure: `GetCommitsDir`, `CommitDiff` type, `SaveCommitDiff`
8. Implement post-commit git hook CLI command (`repo commit-hook`) that captures full codebase diff
9. Create git hook installer / hook script
10. Update README requirements
11. Update/extend tests
12. Build and verify
