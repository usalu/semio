# Log: Ensure semio-repo MCP Works Across All IDEs

## 2026-01-19

### Research Phase

Researched MCP configuration formats for each IDE:

| IDE | Config Location | Format | Key |
|-----|-----------------|--------|-----|
| VSCode | `.vscode/mcp.json` | JSON | `servers` |
| Claude Code | `.mcp.json` | JSON | `mcpServers` |
| Cursor | `.cursor/mcp.json` | JSON | `mcpServers` |
| Windsurf | `.windsurf/mcp.json` | JSON | `mcpServers` |
| Codex | `.codex/config.toml` | TOML | `[mcp_servers.<name>]` |

### Changes Made

1. **Updated `.mcp.json`** (Claude Code)
   - Changed command from `go run ./go/repo mcp` to `./go/repo/repo mcp`
   - Uses prebuilt binary for better reliability

2. **Updated `.vscode/mcp.json`** (VSCode)
   - Standardized command to `./go/repo/repo mcp`

3. **Created `.cursor/mcp.json`** (Cursor)
   - New file with semio-repo and playwright-test servers
   - Uses `mcpServers` key format

4. **Created `.windsurf/mcp.json`** (Windsurf)
   - New file with semio-repo and playwright-test servers
   - Includes `disabled: false` flag per Windsurf spec

5. **Created `.codex/config.toml`** (Codex)
   - New file with TOML format
   - Uses `[mcp_servers.<name>]` sections per Codex spec
   - Includes `enabled = true` flag

### Testing

Verified MCP server works correctly:
```bash
$ echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | ./go/repo/repo mcp
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-03-26","capabilities":{"tools":{"listChanged":true}},"serverInfo":{"name":"semio-repo","version":"1.0.0"}}}
```

Tools list verified with 30+ available tools including:
- `ticket_open`, `ticket_close`, `ticket_list`, `ticket_read`
- `file_create`, `file_delete`, `file_list`, `file_move`
- `folder_create`, `folder_delete`, `folder_list`, `folder_move`
- `section_create`, `section_delete`, `section_list`, `section_move`
- `policy_check`, `policy_list`, `analyze`, `fix`
- `graphql`, `integrate`, `definition_list`
- `contributor_add`, `contributor_list`, `contributor_remove`
- `project_list`, `project_tree`

### Note

The `ticket_reopen` command mentioned in CLAUDE.md does not exist in the MCP tools list. This may need to be implemented separately.
