# ide-free progress (executor ide-free, wave 1021a)
- 10-21 01:2x HOLD received (no process > 200 MB until RESUME): only reads/edits of staged files; verification runs wait.
- DONE: staged/plans-map.json via build-plans-map.py (432 entries: 91 move, 340 archive tickets, 1 duplicate delete; 7 ticket.json source->local edits).
- Findings: plans have no commit-level provenance (history is synthetic), only 16 of 429 referenced by ticket.json; curated T2 evidence = 75; rest get own closed archive ticket (goal AI-OPTIMIZED-REPO).
- Cursor plan surfaces to change: tickets Go plan-source + Rust documents rs, client cli plan-resolution, MCP factories x2 (Go), client mcp repository.go, Rust mcp repository, mcp descriptions (3), mcp surface fixture (extraTicketArgument cursor->null), mcp schema desc, mcp tests (feature, unit rs, protocol-contract go), tickets unit tests, client ticket-plan tests, library commit bullets/taxonomy/historical test/micro-commit test, skill micro-commit, dashboard authority schema/test/fixture, workspace formatter.
- Skipped on purpose: coordinator tsconfig plugins:next (Next.js rewrites it), uv.lock (needs lock regeneration).
