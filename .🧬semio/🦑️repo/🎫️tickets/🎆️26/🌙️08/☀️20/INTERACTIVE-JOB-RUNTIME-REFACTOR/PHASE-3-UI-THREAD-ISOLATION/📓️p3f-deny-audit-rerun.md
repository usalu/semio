# P3f UI-Reachable Forbidden-Call Audit

After the Flow compiler repair, the permanent Phase-3 deny-mode audit was rerun from the repository
root:

```text
bun ./📜️script.ts verify interactivity
```

Result: exit `0`, deny mode clean.

- Scope: four UI-reachable roots and the single sanctioned worker-runtime root.
- Findings: one `blocking-bridge`, the permanent native renderer process entry point.
- Unlisted blocking bridges: zero.
- Synchronous filesystem/network/clipboard/process/database findings: zero.
- Unsanctioned thread/pool construction findings: zero.
- Stale allowlist entries: zero.
- One structurally invisible test-only entry and two predeclared out-of-scope entry/test entries were
  reported as metadata, not failures.

This proves the static P3c deny gate. Runtime UI-thread ownership and callback timing remain covered
by the renderer owner’s separate executable gates.
