# Summary

Python engine serving hand-written OpenAPI and GraphQL schemas with validation and backend functionality.

# Docs

## Files

- `main.py` - Main engine module with Kit parsing, validation, transformation, dev-mode startup flag, and stdio MCP startup flag
- `engine.test.py` - Unit tests for engine functionality
- `🔗️schema.graphql` - Hand-written GraphQL SDL for the engine HTTP API (sibling bundles hold domain GraphQL, OpenAPI, and JSON Schema assets)

# 💯️Requirements

## Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.

Engine startup MUST support a pure stdio MCP server mode.
