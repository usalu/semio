# Terra P1o MCP Owned-Transport Source Audit — 2026-08-23

## Verdict

**REJECT — source packet only.** The Tokio/Axum runtime cutover and retained WorkerPool I/O turn are structurally present, but the live owned `/bridge` path still silently consumes malformed binary bridge messages. That directly fails the required no-silent-unsupported-consumption gate. Additional protocol and ownership defects also prevent acceptance.

This is a narrow P1o source verdict, not a Phase 1 verdict.

## Evidence Read

I read the Phase 1/2 readiness audit, the accepted final P1n audit, the P1o implementation report, and the current MCP root, transport, bridge, async-pool, and verifier sources. The live packet scope is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs`
- `📜️script.ts`

## Blocking Findings

1. **Malformed post-`Hello` binary bridge frames are silently consumed.** `parse_one_websocket_frame` first drains the exact received frame and releases its request credit. For an opened bridge it then maps `ShellToGateway::decode(&frame.payload)` failure to `ConnectionTurn::Keep(HttpConnectionPhase::DrainBridgeOutbox)` instead of a terminal/error result. The connection stays registered and the rejected binary frame is neither surfaced nor closed. The test-only Axum oracle has the same historical skip behavior, so the parity test cannot prove the requested stronger rule. There is no direct fixture or verifier adversary for this branch.

2. **The RFC 6455 handshake does not validate `Sec-WebSocket-Key`.** The owned handshake accepts every non-empty key up to 128 bytes and hashes its literal text. It neither Base64-decodes the field nor requires the decoded nonce to be 16 bytes. This is not RFC 6455 handshake validation and is a live security/protocol defect, despite the one RFC accept-vector fixture passing.

3. **The bridge outbox measures a frame by allocating `frame.encode()` before capacity rejection.** `BridgeOutboxState::try_push` calls `frame.encode().len()` before comparing against its 1 MiB ledger. `BridgeHandle::try_send_to` is public and `GatewayToShell` carries owned vectors with no pre-encode bound, so an over-capacity caller can cause an additional arbitrary-size encoding allocation before receiving its exact rejected owner. The outbox slot count is fixed, but byte admission is not preflighted at the public boundary.

4. **Terminal connection retrieval is not a deterministic owner handback.** `take_terminal_connection` exists, but the immediately scheduled next `drive_one` pops the same terminal FIFO and calls `owner.close()`. There is no terminal-notification/claim authority that reserves it for the public taker. A caller may win a race, but source does not provide the claimed retrievable exact-owner contract; internal close is the normal next-turn behavior.

The first finding alone is sufficient for this REJECT. Findings 2–4 are independent repair requirements, not speculative runtime concerns.

## Structural Gates That Passed

| Gate                         | Result                                                                                                                                                                                                                                                                                                                       |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| One process-pool I/O turn    | `HttpTransport::start` acquires the process singleton with `ProcessKind::InteractiveNative`; authority submission uses `Lane::Io`; each closure invokes `state.drive_one(pool.now_ms())` once. Process-entry `run_http` owns the permitted `transport.start(server)?.wait()` boundary.                                       |
| Live Tokio/Axum retirement   | The production portions of transport/bridge/root contain no Tokio runtime builder, `block_on`, Tokio sync channel, Axum serve, or MCP-owned thread spawn. Located Tokio/Axum/read-to-end/write-all usages are test-only fixtures/differential adapters.                                                                      |
| Per-turn I/O shape           | The state machine performs one terminal close, accept, read page, parser token, dispatch, WebSocket frame, outbox receive, write page, or release per `drive_one`. Read/write pages are 16 KiB.                                                                                                                              |
| Fixed active state           | Active connection slots and terminal FIFO are fixed at 64. Active ingress/egress use 1 MiB per-connection and aggregate request/response byte credits. Retry and readiness callbacks are one-shot and generation-checked.                                                                                                    |
| Early security ordering      | Origin precedes path handling; `/mcp` bearer validation precedes MCP dispatch; `/bridge` token, Upgrade/Connection, and version checks precede registration. Client WebSocket frames require FIN, RSV clear, mask, known opcode, payload/control bounds; text, continuation/fragmented, and unsupported opcodes terminalize. |
| Bridge lifecycle/outbox core | Hello registers only after decode and produces Welcome; Ping/Pong, record, Bye/close, fixed 64-item/1 MiB FIFO, exact item/byte/unknown/closed `try_send_to` handback, and one received outbox frame per turn are present.                                                                                                   |

## Fixtures And Verifier Assessment

The source contains useful direct fixtures for connection and byte caps, stale generation equality, partial masked frames, unmasked/fragmented/oversize WebSocket rejection, one terminal FIFO close, one read/parser/write page, slowloris, cancellation, and bridge outbox limits. `bun ./📜️script.ts verify interactivity --self-test` meaningfully mutates obvious runtime-builder, unbounded I/O, absent byte-credit, missing terminal, unkeyed readiness, unmasked-WebSocket, and dynamic-outbox strings.

Those fixtures do not cover malformed-but-complete `ShellToGateway` payload after registration, invalid WebSocket key encoding/nonce width, a pre-encode over-capacity outbox frame, or deterministic external terminal-owner retrieval. Consequently a clean verifier cannot close the above semantic gaps.

## Commands Run

```text
rustfmt --edition 2021 --check [three scoped MCP Rust paths]
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity --self-test
git diff --check
git diff --cached --check
git diff HEAD --check
```

All commands exited 0. The interactivity report’s single record-only test-only allowlist finding is outside this packet and is structurally invisible to the scanner. Scoped variants of the three diff checks also passed.

## Deliberate Limits And Remaining Phase 1 Blockers

Cargo, Nx, Wasm, browser, network, root lint, compilation, and runtime/security execution were not run as directed. This audit therefore makes no runtime, timing, socket interoperability, or cryptographic-implementation claim.

After the four source findings are repaired, the owning lane still needs compilation plus targeted real socket/WebSocket/SSE/cancellation/slowloris and pool-contention probes. The broader readiness audit still leaves Phase 1 open for store-sync runtime ownership and serialized runtime proof; P1n remains accepted only as its own source packet.
