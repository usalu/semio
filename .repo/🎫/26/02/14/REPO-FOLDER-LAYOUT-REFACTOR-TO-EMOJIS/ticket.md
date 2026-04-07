---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Refactored all folder path references in the repo codebase to use the new emoji-based layout:

- `tickets` → `🎫` with 2-digit year format (YY/MM/DD/SLUG)
- `drafts` → `✍️`
- `goals` → `🎯`
- `contributors` → `👤`
- `prompt/templates` → `💬/📋`

Key changes:

1. Updated GetTicketsDir, GetDraftsPath, GetRepoGoalsDir, contributor path functions to use emoji folder names
2. Changed FormatDate to return 2-digit year (year % 100)
3. Updated GetTicketPath to use PadNumber(year, 2)
4. Updated ListTickets/StreamTickets year filtering to use PadNumber
5. Changed ticket URI format from %04d to %02d for year
6. Updated ISO date formatting to use 2000+year for full year
7. Removed legacy path fallback in delete ticket
8. Updated prompt template path to use 💬/📋
9. Updated vscode extension paths (tickets, goals, drafts) and removed legacy fallbacks
10. Updated CLAUDE.md, AGENTS.md, README.md documentation
11. Updated all test expectations in main_test.go

All relevant tests pass: ContributorDiscovery, TicketsNonEmpty, TicketTitleValidation, FilterTicketWorkspaceFiles, TicketListCommand, TicketOpenContinueKeyword, GoalCreate, GoalHierarchy, GoalList, GoalArtifactID, IdToUri, UriToId, LifecycleCommands, TicketLifecycle_NoGithub, CLI E2E Ticket/Goal Lifecycle, ToolTicketLifecycle, ToolDraftLifecycle.

## Changes

## Log

## Todos

## Plan
