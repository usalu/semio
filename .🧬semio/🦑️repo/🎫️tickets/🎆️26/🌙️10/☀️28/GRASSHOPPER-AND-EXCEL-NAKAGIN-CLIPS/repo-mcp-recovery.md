# Correct-Root Repository MCP Recovery

The saved repo-mcp-client.py uses an explicitly selected repository binary and workspace cwd, structured JSON-RPC, bounded replies and an owned process lifetime. Read-only initialization, tool discovery (including ticket_close) and repo://goals succeeded. No ticket or goal was modified.

The current PATH launcher and6db cache image cannot serve MCP and requested a full install. An already installed full aa03 cache image serves the required interface, so no build, download or global setup mutation was needed. The client requires an explicit binary argument and is portable; the host-specific path belongs to this invocation only. Initial Unicode console metadata inspection failed under CP1252 and was rerun with ASCII-escaped metadata; no source or account data changed.

```json
{
  "root": "C:\\git\\semio",
  "binary": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.exe",
  "tool_count": 55,
  "server": {
    "name": "repo",
    "version": "1.0.0"
  },
  "updatedtutorials_visible": true
}
```

The ticket remains open. Actual close requires the final accepted gates and a concrete structured request containing the exact changed-file list.

## Independent Read-Only Client Review

The explicit executable argument and marker-derived workspace cwd avoid the broken PATH launcher and unrelated repository roots. UTF8 newline JSON-RPC uses initialize, initialized notification and sequential matching request IDs; structured tools/call avoids shell interpolation. Smoke mode performs only discovery/goals reads. Request mode accepts only ticket_close, checks protocol/tool errors and retains the result. No close/goal mutation was invoked by this review.

Two concrete bounded-lifetime follow-ups were sent to root. First, incoming.get(timeout45) is inside the unmatched-message loop, so repeated notifications/unrelated IDs can reset the timeout indefinitely; use a single monotonic deadline and remaining time percall. Second, after terminate/wait5, a POSIX process ignoring SIGTERM can leave the second timeout uncaught and the owned child alive; use a bounded kill/reap fallback for that exact owned child only. No unrelated/global process termination is appropriate.

A ticket-close request that times out or loses protocol response has an unknown external outcome. Do not blindly retry: preserve the concrete request/attempt and inspect actual ticket state first. This is separate from final permission/goal scope; no mutation is authorized at this review stage. Root owns the later reviewed exact request and final accepted gates.

## Bounded Transport and Unknown Outcomes

Independent review found that per-message waits could extend a response deadline and that POSIX shutdown needed a final owned-child kill/reap stage. Portable notification schedules and graceful/terminate/kill cases were observed red, then passed after a single monotonic deadline and bounded escalation were implemented. Close-attempt cases also observed missing uncertainty detail and ambiguous duplicate errors; current code exclusively records the request before dispatch, preserves unknown failure type/message, and blocks blind retries until recorded ticket state is inspected. All three retained stdlib tests pass (0.043s); fake close requests never reached a real server or modified a ticket/goal.

The refreshed read-only MCP smoke67070 timed out at tools/list under the host shortage and exited1 after owned cleanup. It did not dispatch a mutation. The preceding initialization/tools/goals success remains the explicit interface-availability snapshot; this failed refresh is not presented as a current full smoke pass. No further speculative retry was submitted.

### Saved Repair Reread

Independently reread the repaired client. A single monotonic deadline governs each reply loop despite unrelated messages. Owned shutdown has3s graceful, terminate5s and kill5s stages. An exclusive attempt marker is created before ticket_close; it retains pending/unknown outcome and failure type/message, blocks blind retries and records completed results. These resolve the requested source findings. Root records three portable fake-close tests PASS0.043s, with no real mutation.

The refreshed read-only tools/list smoke timed out45s under host pressure and exited1 after owned cleanup. This remains a NONPASS, not a replacement for the earlier successful discovery or proof of current service liveness. No ticket/goal close was performed by this review. Final accepted gates and current ticket-state verification still precede any real close.
