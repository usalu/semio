# Ticket

## Todos

## Changes

## Log

## Summary

Extended ticket mechanism to support plan file iterations. When a plan is provided on ticket create, the original file is moved to the ticket folder as plan_1.md. On reopen with a plan, the file is moved as plan_N.md (where N is the iteration number). Added GetTicketPlanPathForIteration function, updated CreateTicket and ReopenTicket to handle plan file moves, added planPath parameter to TicketReopenInput in GraphQL schema and Go struct, updated MCP tool definitions, CLI command flags, and GraphQL resolvers.
