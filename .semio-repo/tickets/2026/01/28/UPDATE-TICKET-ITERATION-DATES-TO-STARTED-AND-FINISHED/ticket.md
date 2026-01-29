# Ticket

## Todos

## Changes

### js/vscode/extension.ts
- Updated GraphQL `Tickets` query to use `dates` instead of `date`.
- Updated `TicketsProvider` to map `dates` from GraphQL response.

## Log
- Renamed `date` to `dates` in `go/repo/main.go` and `graphql/repo/schema.graphql`.
- Updated VS Code extension to use the renamed `dates` field in GraphQL queries and data mapping.

## Summary

Renamed 'date' to 'dates' in Ticket GraphQL object and schema to align with the new naming convention where iterations and tickets have started/finished dates. Updated VS Code extension to use the renamed field. Go structs already supported started/finished dates for both tickets and iterations.
