---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Introduced InteractionResource as a proper first-class resource. Added InteractionResource struct with source metadata, ListInteractions/StreamInteractions aggregation functions, interaction list/tree CLI commands, interactions GraphQL query, and interactions field on Goal type. Updated schema.graphql accordingly. All tests pass.

## Changes

- `repo/cli/main.go`: Added `InteractionResource` struct, `ListInteractions`/`StreamInteractions` functions, `interactionCommand` (list/tree), `Interactions` resolver, `interactionResourceType` GraphQL type, `interactions` query field, `interactions` field on Goal type
- `repo/graphql/schema.graphql`: Added `InteractionResource` type, `interactions` query, `interactions` field on Goal

## Log

- All existing tests pass (excluding pre-existing `TestTreeCommands` timeout unrelated to changes)
- CLI `interaction list` and `interaction tree` verified at runtime with JSON and text output

## Todos

- [x] Add `InteractionResource` struct enriching Interaction with source (ticket/goal ID, source kind)
- [x] Add `ListInteractions` and `StreamInteractions` aggregation functions
- [x] Add `interactionCommand` with `list` (--sorted) and `tree` subcommands to CLI
- [x] Add `interactions` query to GraphQL schema (code + .graphql file)
- [x] Add `interactions` field to Goal GraphQL type
- [x] Build and test

## Plan

1. ~~Add `InteractionResource` struct to main.go~~ ✅️
2. ~~Add `ListInteractions`/`StreamInteractions` functions~~ ✅️
3. ~~Add `interactionCommand` function with `list` and `tree` subcommands~~ ✅️
4. ~~Register `interaction` command in root command~~ ✅️
5. ~~Add `interactions` query to GraphQL query type + resolver~~ ✅️
6. ~~Add `interactions` field to Goal GraphQL type~~ ✅️
7. ~~Update schema.graphql~~ ✅️
8. ~~Build, test, close ticket~~ ✅️
