---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY
---

# Ticket

## Summary

Refactored interactions from duration-based (dates.created/dates.finished) to pure moment-based events (date). Removed InteractionDates/TicketDates structs. Updated all lifecycle functions, GraphQL schema, VS Code extension. Migrated 641 ticket JSONs and 54 goal JSONs. All tests pass.
## Changes

- `semio-repo/cli/main.go`: Removed `InteractionDates`, `TicketDates` structs. Changed `Interaction.Dates` to `Interaction.Date string`. Removed `Ticket.Started`/`Ticket.Finished` fields, `TicketData.Dates`. Removed `GoalDates.Closed`. Updated `CreateTicket`, `FinishTicket`, `ReopenTicket`, `GoalCreate`, `GoalClose`, `GoalReopen` to use `Date`. Added `GetDateStarted()`/`GetDateFinished()` deriving from interactions. Updated GraphQL schema (removed `interactionDatesType`, added `date` field to `interactionType`). Updated SQLite checkpoint export. Updated `ToolTicketRead`, `ToolTicketClose`, `ToolTicketList`, tree building code.
- `semio-repo/cli/main_test.go`: Updated `TestInteractionUnmarshalAuthorShapes` to use new `date` format instead of `dates.created`.
- `semio-repo/vscode/queries.ts`: Updated GraphQL queries from `dates { created finished }` to `date` on interactions.
- `semio-repo/vscode/extension.ts`: Updated `TicketInteraction` interface from `dates: { created, finished }` to `date: string`.
- `semio-repo/vscode/extension.test.ts`: Updated `TicketInteraction` test fixture.
- `semio-repo/vscode/generated/graphql.ts`: Updated generated types (removed `InteractionDates`, changed `Interaction.dates` to `Interaction.date`).
- `semio-repo/graphql/schema.graphql`: Removed `InteractionDates` type, updated `Interaction` type with `date: String!`.
- Migrated 641 ticket JSONs: removed top-level `started`/`finished`/`dates`, converted interaction `dates.created` to `date`.
- Migrated 54 goal JSONs: converted interaction `dates.created` to `date`.

## Log

- Explored codebase structure and interaction model
- Removed InteractionDates, TicketDates structs
- Updated Interaction struct: Dates → Date
- Removed Ticket.Started/Finished, TicketData.Dates, GoalDates.Closed
- Updated all creation/close/reopen functions
- Updated GraphQL schema and types
- Updated SQLite checkpoint export
- Updated VS Code extension queries, types, tests
- Migrated all existing ticket and goal JSONs
- All tests pass

## Todos

- [x] Remove InteractionDates, TicketDates structs
- [x] Update Interaction struct (Dates → Date)
- [x] Remove Ticket.Started/Finished fields
- [x] Update Create/Finish/Reopen ticket functions
- [x] Update Goal create/close/reopen functions
- [x] Update GraphQL schema
- [x] Update SQLite checkpoint code
- [x] Update VS Code extension
- [x] Update tests
- [x] Migrate 641 ticket JSONs
- [x] Migrate 54 goal JSONs
- [x] Build and run tests

## Plan

1. Remove InteractionDates/TicketDates structs from main.go
2. Update Interaction struct to use Date string instead of Dates InteractionDates
3. Remove Started/Finished from Ticket, Dates from TicketData, Closed from GoalDates
4. Update all ticket/goal lifecycle functions
5. Update GraphQL schema, SQLite export, entity rendering
6. Update VS Code extension types and queries
7. Migrate all existing ticket/goal JSONs using jq
8. Run tests to verify
