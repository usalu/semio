# Plan: Ensure semio-repo MCP Works Across All IDEs

## Overview
Make semio-repo MCP tool working in VSCode, Windsurf, Claude Code, Codex, and Cursor.

## Research Summary

### Configuration Formats by IDE

| IDE | Config Location | Format | Key |
|-----|-----------------|--------|-----|
| VSCode | `.vscode/mcp.json` | JSON | `servers` |
| Claude Code | `.mcp.json` | JSON | `mcpServers` |
| Cursor | `.cursor/mcp.json` | JSON | `mcpServers` |
| Windsurf | `.windsurf/mcp.json` (project) or `~/.codeium/windsurf/mcp_config.json` (global) | JSON | `mcpServers` |
| Codex | `~/.codex/config.toml` (global) or `.codex/config.toml` (project) | TOML | `[mcp_servers.<name>]` |

### Current State
- `.mcp.json` (root): Uses `go run ./go/repo mcp` - requires go toolchain
- `.vscode/mcp.json`: Uses `go/repo/repo mcp` - uses prebuilt binary

## Tasks

1. **Standardize MCP command approach**
   - Use prebuilt binary `./go/repo/repo mcp` for reliability
   - Binary already exists at `/workspaces/semio/go/repo/repo`

2. **Update existing configs**
   - [x] `.vscode/mcp.json` - Already configured correctly
   - [ ] `.mcp.json` - Update to use binary instead of `go run`

3. **Create new configs**
   - [ ] `.cursor/mcp.json` - Create for Cursor
   - [ ] `.windsurf/mcp.json` - Create for Windsurf (project-level)
   - [ ] `.codex/config.toml` - Create for Codex

4. **Verify all configs work**
   - Test that MCP tools are recognized in each environment

## Configuration Templates

### Claude Code (.mcp.json)
```json
{
  "mcpServers": {
    "semio-repo": {
      "type": "stdio",
      "command": "./go/repo/repo",
      "args": ["mcp"]
    }
  }
}
```

### Cursor (.cursor/mcp.json)
```json
{
  "mcpServers": {
    "semio-repo": {
      "command": "./go/repo/repo",
      "args": ["mcp"]
    }
  }
}
```

### Windsurf (.windsurf/mcp.json)
```json
{
  "mcpServers": {
    "semio-repo": {
      "command": "./go/repo/repo",
      "args": ["mcp"],
      "disabled": false
    }
  }
}
```

### Codex (.codex/config.toml)
```toml
[mcp_servers.semio-repo]
command = "./go/repo/repo"
args = ["mcp"]
enabled = true
```

## References
- [Claude Code MCP Docs](https://code.claude.com/docs/en/settings)
- [Cursor MCP Docs](https://cursor.com/docs/context/mcp)
- [Windsurf MCP Docs](https://docs.windsurf.com/windsurf/cascade/mcp)
- [Codex MCP Config](https://github.com/openai/codex/blob/main/docs/config.md)
