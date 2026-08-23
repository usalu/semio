# P1o MCP Owned Transport Final Source Re-Audit — 2026-08-23

## Verdict

**ACCEPT — source packet only.** This accepts the repaired P1o owned-transport/broadcast-close packet at the current source snapshot. It does not accept Phase 1, compilation, scheduling/timing, socket interoperability, security runtime behavior, or any other concurrent work.

## Scope Read

I independently read the implementation handoff and all three preceding P1o audits. The first audit rejected malformed post-open input, WebSocket key validation, premature outbox allocation, and terminal retrieval. The first re-audit rejected unbounded generated inbound decoding and pre-admission broadcast cloning. The second re-audit rejected the close-after-admission publish panic. This audit reviewed the current transport, bridge, MCP entry, verifier, and the close-race fixtures without trusting the handoff's claims.

The live scoped diff is confined to the owned MCP transport packet plus its verifier: `🚚️transport/🦀️component.rs`, `🧵️bridge/🦀️component.rs`, `🌉️mcp/🦀️component.rs`, and `📜️script.ts`. The broader worktree is concurrently dirty; no P3/P8 source, Cargo, manifest, lock, or coordinator content was edited by this audit.

## Re-Audited Source Findings

- `BridgeOutbox::publish` returns `BridgeRejectedPublish { grant, encoded }` before mutation whenever the recipient is closed, stale, or lacks its reservation. The cursor preserves `rejected.grant` in `RecipientClosed` and drops only the rejected cloned lease; the retained shared lease remains owned by the cursor.
- `BridgeBroadcastCursor::step` advances exactly one recipient per retained turn. It deterministically records `Published` or `RecipientClosed`, retains ascending `ShellConnectionId` admission order, and completes as `Undelivered { frame: original, recipient_closed }` if delivery is zero; partial completion reports both counts.
- Admission checks frame size, recipient count, aggregate bytes, broadcast/completion capacity, retirement capacity, and every outbox claim before encoding or cloning the frame. Rollback cancels every prior claim and returns the original frame.
- Terminal closure is explicit and bounded: `close_one_terminal_broadcast_claim` closes one remaining claimed grant per call, while `close_one_terminal_retired_page` releases one fixed page per call. Shutdown/poison retain an exact rejected worker job; completion retrieval is fixed FIFO.
- Generation checks prevent an old connection's claim from entering a closed/reopened outbox. The final `Arc<BridgeEncodedFrame>` drop transfers fixed pages to the authority retirement ring rather than deep-dropping them on the caller's turn.
- The production WebSocket route uses `ShellToGatewayDecodeCursor` then `ShellToGatewayMaterializeCursor` before consuming ingress or mutating bridge state. The generated `ShellToGateway::decode` usages, Axum/Tokio server, and panic fixtures found by raw search are `#[cfg(test)]` oracle/fixture code, not the owned live path.
- The reviewed fixtures meaningfully exercise close-before-first publication, close-mid-list, all-closed exact-original completion, stale-generation ABA, terminal one-claim progression, and two-page last-lease retirement. The verifier's adversarial mutations reject panic, lost-grant, recipient-drain, unstable-order, pre-admission clone/encode, unbounded decode, and ordinary last-page drop variants. This is structural verification; Rust tests were not run because Cargo execution was prohibited.

No live broadcast-cursor `panic!` or `unreachable!` remains, and no clone or encoded-frame allocation occurs before aggregate admission. The repaired source contains one `BRIDGE_ASYNC_RETRY_MS` definition in the audited final snapshot.

## Gates Run

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` over the three scoped MCP Rust files | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; deny mode clean |
| `bun ./📜️script.ts verify interactivity` | PASS; deny mode clean |
| Production-path scan for Tokio runtime/builders/spawn/sync, Axum serve, `block_on`, dynamic bridge queue, pre-admission clone, and broadcast cursor panic/unreachable | PASS; raw test-only oracle hits classified correctly |
| `git diff --check` | PASS |
| `git diff --cached --check` | PASS |
| `git diff HEAD --check` | PASS |

Both verifier invocations report one existing `blocking-bridge` census finding and retained allowlist entries outside the MCP packet; deny mode classifies them clean and the MCP checks add no finding.

## Limitations and Remaining Phase 1 Work

Cargo compilation, Rust test execution, WorkerPool timing/race execution, real HTTP/SSE/WebSocket socket tests, browser/Wasm, network, Nx, and root lint were deliberately not run. Therefore this report makes no runtime, security-operation, throughput, or timing claim.

Phase 1 remains open. The readiness audit's separate store-sync nested runtime/`block_on` ownership and controlled runtime-proof work remain, along with the required serialized post-source compile and runtime gates. This narrow source acceptance is not Phase 1 acceptance.
