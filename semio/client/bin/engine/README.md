# Summary

Python engine serving hand-written OpenAPI and GraphQL schemas with validation and backend functionality.

# Docs

## Files

- `main.py` - Engine module (kit working view, local stores, semio hub client, REST, GraphQL and MCP) with its pytest suite, dev-mode startup flag and stdio MCP startup flag
- `script.ts` - `bun ./script.ts test` (uv sync, MCP App build, pytest), `bun ./script.ts dev mcp` (MCPJam inspector on the stdio MCP), `bun ./script.ts build`
- `schema.graphql` - Hand-written GraphQL SDL for the engine HTTP API (sibling bundles hold domain GraphQL, OpenAPI, and JSON Schema assets)

# 💯Requirements

## Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.

Engine startup MUST support a pure stdio MCP server mode.

## Hub

A remote kit is one semio hub session: `login_to_hub` stores the token per hub URL, `start_working_in_remote_kit` opens `GET /sessions/{id}/kit` as working view and `finish_working_in_kit` pushes every change as kit-scoped hub operations that all collaborators receive live.
