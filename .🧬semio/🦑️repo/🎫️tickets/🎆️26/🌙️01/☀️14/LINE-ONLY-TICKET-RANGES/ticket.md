# Ticket

## Todos

- Review existing ticket range structures in repo, cli, mcp, and VS Code extension.
- Update range data structures, serialization, and consumers to use line-only ints.
- Update dev docs (README.md, AGENTS.md) to reflect ticket range format change.
- Verify usage sites for range handling and adjust related tests or mappings if present.

## Changes

## Log

User request: Change ticket ranges to line-only ints (start/end lines) instead of line/character objects. Update repo, cli, mcp, and VS Code extension. Files: go/cli/main.go, go/mcp/main.go, ./repo/cli/cli.go, js/vscode/extension.ts.
Changes: updated ticket section ranges to use start/end line numbers in repo; GraphQL section/definition ranges now return line-only integers; CLI, MCP, and VS Code extension queries updated to request range { start end }; docs updated in README.md and AGENTS.md.
Fixup: created ticket.json with base commit metadata so ticket_close can compute diffs.
Ticket close attempt failed via mcp tool with error: no base commit found for ticket.

## Summary

Bulk close
