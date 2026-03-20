# Repo Binary Refactor Plan

## Prompt

Implement /workspaces/semio/plans/repo-binary-refactor-plan.md. Change/refactor/extend whatever is necessary to get it working. Update docs and ticket workspace.

## Log

- Created internal/events/events.go with streaming event schema.
- Added core request model, error codes, and engine scaffolding.
- Added CLI renderers (compact, jsonl) and exit code error type.
- Expanded CLI adapter commands and added JSON renderer.
- Wired MCP command to new adapter and updated MCP GraphQL tool.
- Updated VS Code extension to parse JSONL event streams.
- Expanded VS Code launch configurations with test variants and publish entries.
- Cleaned VS Code tasks to remove invalid killOnPort options and added repo publish/test variants.
- Updated repo package scripts for cmd/repo entrypoint and wrapped GraphQL results in engine responses.
- Added launch config publish/test variants across packages and removed root lifecycle configs.
- Cleaned VS Code tasks killOnPort warnings.

## Next

- Update VS Code launch configs and repo CLI tests.
- Update README.md and AGENTS.md documentation.
- Align AGENTS Codebase tree for ./repo/cli cmd layout.

## Todos

# Repo Binary Refactor Plan

1. Finalize engine/adapters wiring for CLI and MCP.
2. Port CLI commands to stream GraphQL via core engine.
3. Update MCP tools to use core engine events.
4. Update VS Code extension to consume JSONL event streams.
5. Update VS Code launch configs and tasks ordering.
6. Update tests for streaming JSONL contract.
7. Update README.md and AGENTS.md documentation.

## Summary

Bulk close
