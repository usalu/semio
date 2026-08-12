# Dashboard TUI Workforce — Progress

Ticket: `2026/08/12/DASHBOARD-TUI-WORKFORCE`

| Wave | Status | Notes |
|------|--------|-------|
| W0 unblock | done | [w0-unblock-summary.md](./w0-unblock-summary.md) |
| W1 VT + PTY | done | [w1a-vt.md](./w1a-vt.md), [w1b-pty.md](./w1b-pty.md) |
| W2 widget + clipboard | done | [w2-widget-clipboard.md](./w2-widget-clipboard.md) |
| W3 daemon + client | done | [w3-daemon-client.md](./w3-daemon-client.md) |
| W4 registry + wiring | done | [w4-registry.md](./w4-registry.md), [w4-wiring.md](./w4-wiring.md) |
| W5 workforce + agents | done (core) | [w5-workforce-agents.md](./w5-workforce-agents.md) — board UI / full E2E still thin |
| Verify | partial | crate unit tests green; full PTY smoke / daemon re-attach / two-model workflow not fully exercised end-to-end in this session |

## Commands
```bash
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts run
bun ./…/📜️script.ts daemon start
bun ./…/📜️script.ts daemon attach
bun ./…/📜️script.ts workflow status --ticket <ticket-dir>
```

## Runtime spot-check (this session)
- `semio daemon status/serve/stop` exercised against workspace root.
- `pty_*` UI tests re-run green.
- Interactive TUI / two-model workflow board not fully smoke-tested interactively.
