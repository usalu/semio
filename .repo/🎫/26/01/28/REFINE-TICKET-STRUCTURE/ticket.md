# Ticket

## Todos

- [x] Open ticket
- [x] Remove `progressCmd` from CLI
- [x] Remove `ToolTicketProgress` implementation
- [x] Remove `ticket_progress` MCP tool
- [x] Deduplicate `Ticket` struct (use `json:"-"` for redundant fields)
- [ ] Verify `ReadTicket` logic works (via tests/commands)
- [ ] Close ticket

## Changes

- Modified `./repo/cli/main.go` to remove `progress` command and deduplicate `ticket.json`.

## Log

- Removed `ticket progress` command and associated logic.
- Updated `Ticket` struct to ensure redundant fields (year, month, day, slugs, paths) are not stored in JSON.
- Verified path resolution handles hydration of these fields on read.

## Summary

Refactored ticket structure: removed progress command and deduplicated JSON storage
