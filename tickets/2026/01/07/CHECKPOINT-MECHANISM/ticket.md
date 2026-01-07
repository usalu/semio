---
slug: CHECKPOINT-MECHANISM
prompt: Refactor ticket system from iterations to checkpoints. Checkpoints are created when todos complete, require files, compute git diffs, and track affected sections/definitions.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-07T09:57:52Z"
commit: a1c46c41e5c88ce33dad3bafcb07738a56e4e25d
model: claude-opus-4
---
# Previously

# Plan

1. Update GraphQL schema - remove FileListInput, keep TicketCheckpointInput with required files
2. Update Go repo TicketMetrics - change iterations to checkpoints, add sections/definitions counts
3. Update CLI commands - rename ticketProgressCmd to ticketCheckpointCmd, remove files from create
4. Update MCP handlers - rename ticket_progress to ticket_checkpoint
5. Update VSCode extension - rename types and update command registrations
6. Verify builds

# Changes

## GraphQL Schema (graphql/repo/schema.graphql)
- Removed FileListInput type (was used by old TicketCreateInput)
- TicketCheckpointInput retained with required `files` parameter
- TicketCreateInput no longer has files field

## Go Repo (go/repo/repo.go)
- ticketMetricsType: changed "iterations" field to "checkpoints"
- Added "sections" and "definitions" fields to ticketMetricsType
- Updated metrics resolver to compute checkpoints/sections/definitions counts from ticket data

## Go CLI (go/cli/main.go)
- Renamed ticketProgressCmd to ticketCheckpointCmd
- Removed --file flag from ticketCreateCmd (files no longer accepted on create)
- ticketCheckpointCmd now requires --file flag (can be repeated)
- Updated init() to register ticketCheckpointCmd instead of ticketProgressCmd

## Go MCP (go/mcp/main.go)
- ticket_create tool no longer has files parameter
- Renamed ticket_progress tool to ticket_checkpoint
- ticket_checkpoint requires files array
- Updated handler function names and implementation

## VSCode Extension (js/vscode/extension.ts)
- Renamed TicketIteration type to TicketCheckpointData
- Updated TicketFrontmatter.iterations to checkpoints
- Updated GraphQL query mappings (2 locations)
- Updated TicketItem children loop
- Updated SIDEBAR_COMMANDS: ticketProgress -> ticketCheckpoint
- Updated command registration: semio.ticketProgress -> semio.ticketCheckpoint
- Command now prompts for files instead of prompt text

## VSCode Extension Tests (js/vscode/extension.test.ts)
- Updated EXPECTED_COMMANDS: ticketProgress -> ticketCheckpoint

## Build Fixes
- Fixed empty go/repo/repo_test.go (added package declaration)