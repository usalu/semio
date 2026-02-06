# Ticket

## Todos

- Identify where ticket commands and keyword parsing are implemented.
- Implement CONTINUE and NOTICKET keyword behavior in ticket flow.
- Update README.md and AGENTS.md documentation for the new keywords.
- Capture work in log.md and summary.md; close ticket with file list.

## Changes

## Log

- Added CONTINUE/NOTICKET keyword handling via OpenTicket in ./semio-repo/cli/main.go and wired it through CLI, GraphQL, MCP handlers.
- Added MCP ticket_open noIssue/planPath args and bool parsing helper.
- Added tests covering CONTINUE and NOTICKET behaviors.
- Updated README.md and AGENTS.md ticket documentation for keywords.

## Summary

Implemented CONTINUE/NOTICKET keyword handling in repo ticket open flows, added MCP argument support, and covered behavior with tests. Updated README.md and AGENTS.md to document the new keywords in ticket workflows and codebase notes.
