# 📓️ Status

## 2026-09-18 — closing summary contradicted by the live tree (annotation, ticket stays closed)

Verified against the working tree today while repairing both `.mcp.json` MCP servers under ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END` (slice M1, report
`📓️m1-mcp-servers-start.md`). Four claims in `📓️closing-summary.md` do not hold on this branch — the
migration either never landed here or was reverted afterwards, and the ticket's closed status has been
misleading anyone who trusted it over the tree.

1. **"Rust by default for every entry point … `.mcp.json` … → Rust `semio mcp`; `SEMIO_REPO_IMPLEMENTATION=go`
   switches to Go"** — absent. `SEMIO_REPO_IMPLEMENTATION` exists only in `🎛️dashboard`
   (`🎛️dashboard/🌳️command-tree/🦀️.rs:196-198`, `🎛️dashboard/🧬️schema/🔣️.json:47`); nothing on the
   `.mcp.json` path reads it. `📜️script.ts`'s `runMcpStdioRepo` resolves exactly one binary through
   `resolveMcpBin()` (`🦑️repo/🔨️modules/📚️library/🟦️.ts`), and that binary is the **Go** server built from
   `🦑️repo/🔨️modules/💻️client/🔌️mcp`. The Rust `semio-repo-mcp` bin target
   (`⌨️cli/📦️packages/🦀️rust/Cargo.toml:22`) exists but is not what `.mcp.json` launches.
2. **"`💻️client` … removed"** — present and load-bearing:
   `🦑️repo/🔨️modules/💻️client/{⌨️cli,🔌️mcp,🧩️vscode,🪶️sqlite}`. `💻️client/🔌️mcp` is now the single
   canonical repo MCP Go module (its duplicate under `🔌️mcp/📦️packages/🐹️go`, which claimed the same
   `github.com/usalu/semio/repo/mcp` module path, was deleted on 2026-09-18).
3. **"both answer identical `initialize` + `tools/list` (9 tools…)"** — the surviving tree had only the
   6-tool set (no `goal_open`/`goal_close`/`goal_reopen`) in `💻️client/🔌️mcp/🗄️repository/🐹️.go` until it
   was reconciled back to 9 on 2026-09-18.

4. **The `initialize` defect this ticket fixed had regressed.** `📓️baseline-and-mcp-initialize-defect.md`
   describes it exactly: a strict `decodeExact` on `initialize` params answered `-32602 invalid initialize
   params` for any client sending extra fields. On 2026-09-18 the surviving Go server still did, so a real
   Claude Code frame (`clientInfo.title`, `capabilities.roots.listChanged`, `sampling`, `elicitation`) was
   rejected before `tools/list`. Re-fixed with a handshake-only lenient decoder
   (`💻️client/🔌️mcp/📜️protocol/🐹️.go` `DecodeOpenParams`, used at
   `💻️client/🔌️mcp/🖥️server/🐹️.go`'s `initialize` arm), and the module's `🤝️protocol-contract` suite now
   pins it against the `🔌️mcp` module's own `🧫️fixtures/🤝️initialize-lenient.json` contract, so it cannot
   regress silently a third time.
