# Cursor MCP Zero Touch Cross Platform

## Summary

Updated `.cursor/mcp.json` so the repo MCP (`go run` of `repo/cursor`) works without manual path edits on devcontainer Linux, native Windows, and native macOS:

- `type: "stdio"` (required by Cursor MCP docs for STDIO servers).
- Package path via `"${workspaceFolder}${/}repo${/}cursor"` so resolution does not depend on process cwd.
- `GOWORK` points at the repo-root `go.work` so workspace modules resolve consistently.

## Verification

- `go build ./repo/cursor` from repo root succeeds on Windows (compile check).
