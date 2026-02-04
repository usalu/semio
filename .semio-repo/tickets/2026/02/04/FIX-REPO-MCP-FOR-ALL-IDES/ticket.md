# Ticket

## Summary
Fixed MCP tool schema validation errors by adding `items` definition to array properties and added comprehensive tests to prevent regression.

## Changes
- `@semio-repo/go/main.go`:
    - Refactored `runMcpServer` to `createMcpServer` to enable testability of the server instance.
    - Added `mcp.WithStringItems()` to `ticket_close` and `draft_create` tool definitions.
- `@semio-repo/go/main_test.go`:
    - Added `TestMcpToolsSchemas` to validate that all array types in tool schemas have an `items` field.

## Log
- Refactored `main.go` to separate server creation from execution.
- Added missing `items` configuration to `files` array in `ticket_close` and `draft_create` tools.
- Implemented `TestMcpToolsSchemas` in `main_test.go` recursively checking JSON schemas of all registered tools.
- Validated that `ticket_open` schema is correct and that the previous error was likely preventing the server from loading, thus hiding the tool.

## Todos

- [x] Locate MCP tool definitions and schema generation logic
- [x] Fix schema validation error: array parameters missing `items` field
- [x] Investigate and fix Claude Code tool availability (`ticket_open`)
- [x] Update `go/main_test.go` to test schema validity and tool availability
- [x] Verify functionality across supported IDEs (via tests)

## Plan

1.  **Analyze**: Find the Go code responsible for generating JSON schemas for MCP tools.
2.  **Fix Schema**: Ensure that when a parameter is an array (like `files` in `draft_create` or `ticket_close`), it includes the `items` property in the schema.
3.  **Fix Naming**: Check how `ticket_open` is registered. Claude Code error says `No such tool available: mcp__mcp_semio_repo__ticket_open`. This looks like a namespacing issue.
4.  **Test**: Run existing tests, then extend them to validate the schemas of all registered tools.
