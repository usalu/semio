# Ticket

## Todos

# Plan

# Previously

# Plan

1. Update GraphQL schema - remove FileListInput, keep TicketCheckpointInput with required files
2. Update Go repo TicketMetrics - change interactions to checkpoints, add sections/definitions counts
3. Update CLI commands - rename ticketProgressCmd to ticketCheckpointCmd, remove files from create
4. Update MCP handlers - rename ticket_progress to ticket_checkpoint
5. Update VSCode extension - rename types and update command registrations
6. Verify builds

# Changes

## GraphQL Schema (graphql/repo/schema.graphql)

- Removed FileListInput type (was used by old TicketOpenInput)
- TicketCheckpointInput retained with required `files` parameter
- TicketOpenInput no longer has files field

## Go Repo (./semio-repo/cli/cli.go)

- ticketMetricsType: changed "interactions" field to "checkpoints"
- Added "sections" and "definitions" fields to ticketMetricsType
- Updated metrics resolver to compute checkpoints/sections/definitions counts from ticket data

## Go CLI (go/cli/main.go)

- Renamed ticketProgressCmd to ticketCheckpointCmd
- Removed --file flag from ticketOpenCmd (files no longer accepted on create)
- ticketCheckpointCmd now requires --file flag (can be repeated)
- Updated init() to register ticketCheckpointCmd instead of ticketProgressCmd

## Go MCP (go/mcp/main.go)

- ticket_open tool no longer has files parameter
- Renamed ticket_progress tool to ticket_checkpoint
- ticket_checkpoint requires files array
- Updated handler function names and implementation

## VSCode Extension (js/vscode/extension.ts)

- Renamed TicketIteration type to TicketCheckpointData
- Updated TicketFrontmatter.interactions to checkpoints
- Updated GraphQL query mappings (2 locations)
- Updated TicketItem children loop
- Updated SIDEBAR_COMMANDS: ticketProgress -> ticketCheckpoint
- Updated command registration: semio.ticketProgress -> semio.ticketCheckpoint
- Command now prompts for files instead of prompt text

## VSCode Extension Tests (js/vscode/extension.test.ts)

- Updated EXPECTED_COMMANDS: ticketProgress -> ticketCheckpoint

## Build Fixes

- Fixed empty ./semio-repo/cli/cli_test.go (added package declaration)

## Changes

## Log

## Summary

# Summary
