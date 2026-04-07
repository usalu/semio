---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Hidden --no-management from all CLI help output using MarkHidden on 8 cobra flag registrations (ticket open/close/reopen/change and goal open/close/reopen/change). Removed noManagement from MCP ticket_open tool parameter list. Feature remains fully functional for users who know it.

## Changes

- `repo/cli/main.go`: MarkHidden all `no-management` CLI flags; remove `noManagement` from MCP tool `WithBoolean` descriptions

## Log

## Todos

- [x] MarkHidden all `--no-management` CLI flags in cobra commands (8 total: ticket open/close/reopen/change, goal open/close/reopen/change)
- [x] Remove `noManagement` from MCP tool parameter descriptions (ticket_open)

## Plan

1. For each CLI flag registration `Flags().Bool("no-management", ...)`, add `MarkHidden("no-management")` after it
2. Remove the `mcp.WithBoolean("noManagement", ...)` line from MCP tool definitions
