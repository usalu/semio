# Checkout Entrypoint Audit

The Codex and generic MCP configurations both start `repo dev mcp stdio os` with the generated composition catalog, the `os-mcp` workspace folder, six declared scopes, and the host operation policy. The existing `probe-semio-mcp.sh` exercises initialization and discovery only and truncates response lines.

The latest coordinator report records successful raw stdio discovery, resources, prompts, context, search, probe artifacts, typed errors, and gateway refusal on a rebuilt server. It records 0.14 seconds for initialization and 0.73 seconds for tool discovery. It does not establish a complete client journey: `artifact_create s.note.note` exceeds its operation budget, blocking typed actions, history, transactions, inference, and export. On-demand model generation and hot swap through the launcher, and a real Claude Code client, remain unverified.

The sibling test report identifies unresolved installed-catalog capability tests, a stale inference schema mirror, a plugin-runtime test API mismatch, and probe-document retirement failure. The subsequent prototype accounting profile reports passing policy fixture tests, but no cargo rebuild or measured speedup. These reports describe observations at their respective times; the shared checkout may have changed since.

Sources: `status-2026-11-25-evening.md`, `live-tool-defects-fixed.md`, `mcp-sibling-tests.md`, and `prototype-accounting-profile.md` in this ticket.
