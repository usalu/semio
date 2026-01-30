# Ticket

## Todos
- [x] Inspect VS Code extension section provider flow and repo section APIs.
- [x] Identify why open file sections are not returned and update implementation/tests.
- [x] Update dev docs (README.md, AGENTS.md) with section visibility behavior.

## Changes

## Log
- Reviewed section list flow in VS Code extension and repo GraphQL schema/resolvers.
- Implemented file section resolution in GraphQL, including file/range/children mapping.
- Updated CLI section list query and extension section fetching to normalize GraphQL output.
- Updated README.md and AGENTS.md with section tree and VS Code extension documentation updates.
- Created ticket metadata JSON with base commit to allow ticket closure.
- Closed the ticket via go/cli CLI because MCP ticket_close targets the wrong root path.

## Summary
Fixed VS Code sections view by resolving file sections in the repo GraphQL layer, updating CLI section list queries, normalizing section list output in the extension, and documenting the section tree behavior.
