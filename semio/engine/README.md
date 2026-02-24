# Summary

Python engine providing schema generation, validation, and backend functionality.

# Docs

## Files

- `engine.py` - Main engine module with Kit parsing, validation, transformation, dev-mode startup flag, and stdio MCP startup flag
- `engine.test.py` - Unit tests for engine functionality
- `generate-schemas.ts` - Generates GraphQL, JSON, and SQL schemas from TypeScript definitions
- `sqliteschema.ts` - SQLite schema generation utilities

# 💯Requirements

## Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.

Engine startup MUST support a pure stdio MCP server mode.
