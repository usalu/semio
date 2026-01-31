# Ticket

## Todos

- [x] Implement MCP resource handlers for repo entities <!-- id: 0 -->
- [x] Register resources in MCP server <!-- id: 1 -->
- [x] Fix compilation errors with mcp-go library <!-- id: 2 -->

## Changes

- Modified `go/repo/main.go` to implement `handle*Resource` functions using internal GraphQL queries.
- Updated `go/repo/main.go` to register resources and resource templates in the MCP server.
- Refactored handler signatures to match `server.ResourceHandlerFunc` (`[]mcp.ResourceContents` return type).
- Removed `mcp.WithMIMEType` from `AddResourceTemplate` calls to fix type errors.

## Log

- Implemented resource handlers using `gql()` helper.
- Encountered compilation errors due to strict type checking in `mcp-go`.
- Debugged handler signature mismatch (library expects slice, code was returning struct ptr).
- Debugged option mismatch for `NewResourceTemplate`.
- Fixed all compilation errors and verified build.

## Summary

Exposed repository entities as MCP resources to allow LLMs to read codebase structure and metadata directly via the `semio-repo` server. Implemented resources include bundles, folders, files, sections, definitions, contributors, goals, tickets, policies, and violation-kinds. Resources are available via static URIs (e.g., `semio://bundles`) and parameterized templates (e.g., `semio://folder/{path}`).
