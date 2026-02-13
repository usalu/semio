---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

## Changes

## Log

## Todos

- [ ] Add `InteractionResource` struct enriching Interaction with source (ticket/goal ID, source kind)
- [ ] Add `ListInteractions` and `StreamInteractions` aggregation functions
- [ ] Add `interactionCommand` with `list` (--sorted) and `tree` subcommands to CLI
- [ ] Add `interactions` query to GraphQL schema (code + .graphql file)
- [ ] Add `interactions` field to Goal GraphQL type
- [ ] Build and test

## Plan

1. Add `InteractionResource` struct to main.go (enriches flat Interaction with source info)
2. Add `ListInteractions`/`StreamInteractions` functions that aggregate from tickets and goals
3. Add `interactionCommand` function with `list` and `tree` subcommands
4. Register `interaction` command in root command
5. Add `interactions` query to GraphQL query type + resolver
6. Add `interactions` field to Goal GraphQL type
7. Update schema.graphql
8. Build, test, close ticket
