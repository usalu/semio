# Ticket

## Todos

- [x] Create GoalUpdateInput and TicketChangeInput in Go
- [x] Implement GoalUpdate and TicketChange resolvers in Go
- [x] Update GraphQL schema
- [x] Fix due date propagation to GitHub milestones in ghUpdateMilestone
- [x] Add `change` command to `goalCommand` in CLI
- [x] Add `change` command to `ticketCommand` in CLI

## Changes

- Modified `go/repo/main.go` to add `GoalDates`, `GoalUpdateInput`, `TicketChangeInput`, `TicketChange` logic.
- Implemented `GoalUpdate` resolver calling `ToolGoalUpdate`.
- Implemented `TicketChange` resolver calling `ToolTicketChange`.
- Updated `ghUpdateMilestone` to support `dueOn`.
- Added `goal change` CLI command.
- Added `ticket change` CLI command.
- Registered changes in `mutationType` and `mutationResolver`.

## Log

- Refactored `go/repo/main.go` to support new functionality.
- Fixed compilation errors regarding interface implementation and pointers.
- Added CLI commands using `cobra`.
- Verified build success.

## Summary

Implemented goal change and ticket change commands
