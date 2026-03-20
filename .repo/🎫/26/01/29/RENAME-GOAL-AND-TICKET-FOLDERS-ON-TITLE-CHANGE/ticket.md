# Ticket

## Todos

## Changes

## Log

## Summary

Added folder renaming when titles change for both tickets and goals. Three areas were fixed:

1. MCP ticketReopen handler: now reads the `title` parameter and calls UpdateTicketTitle (which renames the folder) before reopening.
2. MCP ticketClose handler: now reads the `title` parameter and calls UpdateTicketTitle before closing.
3. Goals (GoalReopen and GoalUpdate): added new UpdateGoalTitle() helper that slugifies the new title, renames the goal folder, and updates goal.ID. Both GoalReopen and GoalUpdate now use this helper instead of directly setting goal.Title.

Also fixed a pre-existing test bug in nogithub_test.go where ReopenTicket was called with missing goal/parent arguments.
