# Tool-Neutral Workspace Recovery

The existing ticket covers the requested work and remains open. The repository MCP `repo://goals` resource was read through the existing CLI's `mcp codex` transport; `ticket_reopen` confirmed that this ticket is already open. Its associated goal is AI Optimized Repo. No goal was changed.

## Fleet

- Coordinator: root commands, IDE launch wiring, integration and runtime acceptance.
- GPT 6.1 Sol High execution: neutral invocation, Nx/Bun bootstrap and dependency readiness.
- GPT 6.1 Sol High execution: Bazel selected input closure, caching and runtime acceptance.
- GPT 6.1 Sol Low audits: bootstrap/developer workflows, then MCP and native cache recovery. Low is the available setting corresponding to the requested Light setting. Four concurrent slots are the maximum.

## Plan

1. Establish schema-backed regression cases for command startup and selected builds before changing behavior.
2. Keep root command registration lazy, including verification lanes and policy imports, so unrelated application tests do not prevent developer tools from starting.
3. Repair bootstrap through the existing storage and invocation ownership protocols; preserve concurrent work and avoid modifying Git commands or worktrees.
4. Qualify tool-neutral selection and installed backends using real cold/warm executions, scoped changes, declared outputs and cancellation where supported.
5. Expose supported executable commands through the existing IDE configuration authority and verify that configuration against the same command contract.
6. Preserve audit Markdown and input fixtures; remove only this fleet's generated outputs at completion. Close the ticket through MCP when its requested work is complete.

## Initial Evidence

- Root `bun ./📜️script.ts --help` eagerly imports unrelated application test lanes and fails to resolve a missing workspace package.
- `bun repo --help` reaches native cache initialization before printing help and fails on an incomplete `.cache/b/l/image.json` birth.
- The current Bun is 1.4.2. The installed Node is 24.14.1 while the repository declares 24.15.0.
- Native MCP CLI transport works. The older standalone MCP binary fails to locate its description table.

Detailed findings and exact test results are maintained in each agent's dated report.
