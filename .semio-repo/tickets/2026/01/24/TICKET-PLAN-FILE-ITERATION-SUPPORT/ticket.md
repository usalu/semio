# Ticket

## Todos

# Plan: Ticket Plan File Iteration Support

## Overview

Extend the ticket mechanism to support plan file interactions. When a plan is provided on ticket create, the original markdown file should be moved to the ticket folder. On reopen, the plan should be moved to `plan_ITERATION.md` to preserve history.

## Changes Required

### 1. Add GetTicketPlanPathForIteration Function

- Create new function `GetTicketPlanPathForIteration(year, month, day int, slug string, interaction int)`
- Returns `plan_{interaction}.md` path
- Keep `GetTicketPlanPath` as helper that calls `GetTicketPlanPathForIteration` with interaction 1

### 2. Update CreateTicket Function (line ~7628)

- Change from `plan.md` to `plan_1.md`
- If `planPath` is provided and file exists:
  - Move the file to ticket folder as `plan_1.md` instead of copying content
  - Remove original file after successful move
- Update `Ticket.PlanPath` to use interaction-based path

### 3. Add planPath to TicketReopenInput

- GraphQL schema: Add `planPath: String` to `TicketReopenInput` (line ~109)
- Go struct: Add `PlanPath string` to `TicketReopenInput` struct (line ~2636)

### 4. Update ReopenTicket Function (line ~8632)

- Add `planPath` parameter
- Calculate interaction index as `len(ticket.Data.Iterations) + 1`
- If `planPath` is provided and file exists:
  - Move the file to ticket folder as `plan_{interaction}.md`
  - Remove original file after successful move

### 5. Update MCP Tool Definition for ticket_reopen (line ~12706)

- Add `mcp.WithString("planPath", mcp.Description("Optional plan file path"))`

### 6. Update MCP Handler ticketReopen (line ~13383)

- Parse optional `planPath` argument
- Include in GraphQL mutation input

### 7. Update GraphQL Resolver for ticketReopen (line ~12387)

- Pass `planPath` to `ReopenTicket` function

### 8. Update GraphQL Input Type Registration (line ~11792)

- Add `planPath` field to `TicketReopenInput` GraphQL type

### 9. Update Ticket Struct

- Change `PlanPath` to store the current interaction's plan path

## File Changes Summary

- `graphql/repo/schema.graphql` - Add `planPath` to `TicketReopenInput`
- `./semio-repo/cli/main.go` - All implementation changes

## Testing

- Verify ticket create with planPath moves file to `plan_1.md`
- Verify ticket reopen with planPath moves file to `plan_2.md`, `plan_3.md`, etc.
- Verify interactions are properly tracked

## Changes

## Log

## Summary

Extended ticket mechanism to support plan file interactions. When a plan is provided on ticket create, the original file is moved to the ticket folder as plan_1.md. On reopen with a plan, the file is moved as plan_N.md (where N is the interaction number). Added GetTicketPlanPathForIteration function, updated CreateTicket and ReopenTicket to handle plan file moves, added planPath parameter to TicketReopenInput in GraphQL schema and Go struct, updated MCP tool definitions, CLI command flags, and GraphQL resolvers.
