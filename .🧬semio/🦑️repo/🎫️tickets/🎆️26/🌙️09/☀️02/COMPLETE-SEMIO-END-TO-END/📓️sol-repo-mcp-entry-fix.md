# Repo MCP Entrypoint Fix

## Outcome

The checked-in repo MCP bootstrap now starts the owned repo MCP server directly. Client profile selection is carried in `SEMIO_REPO_MCP_CLIENT`, so the MCP binary receives no legacy CLI arguments and cannot fall through to `internal/mcpserver`.

The owned server now negotiates MCP `2025-11-25`, `2025-06-18`, `2025-03-26`, `2024-11-05`, and `2024-10-07`. The current official TypeScript SDK (`@modelcontextprotocol/sdk` 1.30.0) completed initialization against the exact checked-in Bun command.

Client-specific behavior is preserved:

- server names remain `repo`, `repo-cursor`, `repo-kiro`, `repo-copilot`, `repo-claude`, and `repo-codex`;
- Cursor, Copilot, Claude, and Codex expose `plan_id`;
- Kiro exposes `spec_id`;
- the resolved profile reaches ticket open/reopen operations for native client attribution and plan/spec archival.

The `repo-mcp` Nx test target now runs the owned MCP module tests before the legacy client-side bootstrap contract tests.

## Changed Files

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🐹️component.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🐹️protocol.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🐹️repository.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🐹️server.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️contract_test.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧫️fixtures/g2-contract.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧫️fixtures/entrypoint-contract.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-repo-mcp-entry-fix.md`

## Verification

- Red contract run before implementation: `bun nx run repo-mcp:test-quick -- -run RepositoryEntrypointContract` failed on missing `MCPProfileEnvironment` and `resolveMCPProfile`.
- Focused entry contract: `bun nx run repo-mcp:test-quick -- -run RepositoryEntrypointContract` passed.
- Owned G2 runtime suite: `bun nx run repo-mcp:test-quick -- -run G2` passed.
- Bootstrap asset contract: `bun nx run repo-mcp:test-quick -- -run McpBootstrapAssetsStayRepoRelative` passed.
- Real bootstrap initialization test: `bun nx run repo-mcp:test-quick -- -run McpStdioInitializeHandshake` passed.
- Full quick target: `bun nx run repo-mcp:test-quick` passed.
- Full repo MCP target: `bun nx run repo-mcp:test` passed.
- Official SDK against `.mcp.json` command: initialized, listed 8 resources, read `repo://goals` (9,222 bytes), listed 6 tools, and dispatched `ticket_reopen`; the deliberately invalid path returned the expected handler error without mutation.
- Official SDK against the Codex profile: server identity was `repo-codex`, `ticket_open` exposed `plan_id`, and it did not expose `spec_id`.

## Residual Risks

- The owned server currently registers 6 tools and 8 static resources. The legacy server advertises a broader repository surface, including additional resources/templates and operations. The startup/handshake blocker is fixed, but parity of the full legacy registry remains separate follow-up work.
- The successful end-to-end tool-path check intentionally used an invalid `ticket_reopen` path to avoid mutating repository management state. It proves protocol dispatch and handler execution, while successful ticket lifecycle mutation remains covered by the repository client tests rather than this live probe.
