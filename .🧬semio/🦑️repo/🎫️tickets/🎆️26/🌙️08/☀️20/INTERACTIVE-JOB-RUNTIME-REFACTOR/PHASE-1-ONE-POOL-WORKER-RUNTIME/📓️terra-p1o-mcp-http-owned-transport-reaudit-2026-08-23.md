# Terra P1o MCP Owned-Transport Source Re-Audit — 2026-08-23

## Verdict

**REJECT — source packet only.** The four previously rejected seams are repaired in the current source, but the live pre-consumption bridge decoder still makes attacker-declared, unbounded container allocations. A malformed post-open binary frame can therefore fail before it is classified and terminalized with its exact raw owner. This violates the packet's fixed-memory and malformed-frame ownership requirements.

This is a P1o source-only verdict, not a Phase 1 verdict.

## Scope and Limits

Read: the P1 readiness audit, accepted P1n audit, prior P1o rejection, updated P1o implementation report, current MCP transport/bridge/root/verifier source, and the live scoped diff.

Per instruction, I did not run Cargo, Nx, Wasm, browser, network, root lint, a socket host, or runtime/security integration tests. Rustfmt and the interactivity verifier are structural gates only; they do not prove compilation, timing, interoperability, or security execution.

## Prior Findings Closed

| Prior finding | Independent current-source result |
| --- | --- |
| Post-open malformed/unsupported bridge binary silently consumed | Closed. `parse_one_websocket_frame` decodes before `consume_websocket_ingress`; decode failure returns `Terminal(Malformed)` and post-open `Hello`/`AppFrames` returns `Terminal(Unsupported)`. Terminalization moves the unchanged raw ingress into `HttpTerminalConnection`. The direct fixture checks unknown-tag and second-`Hello` raw equality. |
| Invalid `Sec-WebSocket-Key` accepted | Closed. The handshake requires exactly one key, exact 24-byte canonical Base64 with `==`, alphabet validation, zero low padding bits, and a decoded 16-byte nonce before it computes `Sec-WebSocket-Accept`. The fixture covers duplicate, alphabet, padding, whitespace, width, and non-canonical-bit rejection. |
| Outbox encoded before capacity admission | Closed for `try_send_to`. `GatewayToShell::encoded_len` uses checked field lengths; `BridgeOutboxState::try_push` rejects before incrementing credits or creating a `BridgeEncodedFrame`. Accepted frames own no more than 64 16 KiB pages and transport writes the retained cursor in one page per turn. The exact-cap/+1 fixture checks no encode occurred on rejection. |
| Public terminal owner races automatic close | Closed. The default policy is `Handback`, so terminal FIFO owners are not drained automatically. `take_terminal_connection` pops one owner while holding the state lock, invalidates readiness, then schedules a capacity opportunity. Process-entry `wait` selects `Close`; that branch closes one FIFO owner per `drive_one` grant. Same-slot generation ABA and process-close fixtures are present. |
| Request/header delimiter scan can see beyond the cap | Closed. `find_crlf_bounded` receives `connection.ingress.len().min(line_bound)` with separate 4,096-byte request-line and 65,536-byte total-header caps. The cap-boundary fixture verifies a later CRLF is not searched. |

## Blocking Findings

1. **Malformed live bridge frames can request an unbounded allocation before classification.** `ShellToGateway::decode` is called on untrusted, masked WebSocket payloads before ingress is released, which is the intended ordering. Its `Instances` branch reads an attacker-controlled `u32` and immediately executes `Vec::with_capacity(len)` without first proving a feasible count from the remaining bytes or applying a protocol maximum. A five-byte decoded payload of tag `3` plus `0xffff_ffff` reaches that allocation path before `BridgeInstanceRef::decode` can report the missing bytes. It can request a multi-gigabyte/overflowing element allocation, panic or abort the process, and never produce `ConnectionTurn::Terminal(Malformed)` or the promised raw terminal owner. `wire::Reader::read_string_vec` and `read_bytes_vec` likewise iterate an unbounded declared count through `collect()`; they need the same remaining-byte and fixed-count admission discipline.

2. **`BridgeHandle::broadcast` bypasses the outbound preflight boundary.** It clones an arbitrary public `GatewayToShell` once per live connection before each clone reaches `try_send`. The connection count is capped at 64, but frame size is not capped before cloning. An over-capacity `ShellCommand`/`AppCommand` supplied to `broadcast` can thus allocate up to one arbitrary clone per connection even though every outbox would reject it; the method returns only a count, so it cannot return those rejected owners. This is outside the repaired `try_send_to` contract and prevents a packet-wide fixed/outbound-exact-rejection claim.

Either finding is sufficient for rejection. The first directly contradicts the explicit requirement that malformed binary ingress classify before credit consumption and terminalize its exact raw frame.

## Structural Evidence That Remains Sound

- `run_http` enters through `HttpTransport::start(server)?.wait()`. `start` binds a nonblocking standard listener, uses the process `WorkerPool` `Lane::Io`, and each submitted closure calls one `state.drive_one(pool.now_ms())`.
- The authored production portions of the transport/bridge/root have no Tokio runtime/builder, `block_on`, Tokio sync channel, Axum server, or explicit thread spawn. The located Tokio/Axum/read-to-end/write-all occurrences are in `#[cfg(test)]` oracle/fixture code.
- A turn is bounded to one accept, read page, parser token, dispatch, WebSocket frame, bridge receive, write page, terminal close, or retained state transition. I/O pages are 16 KiB; active connection and terminal rings are each 64; request/response ledgers and slowloris deadline remain present.
- Origin precedes endpoint dispatch. `/mcp` bearer validation precedes server dispatch. `/bridge` token, Upgrade/Connection, version, unique validated key, and registration ordering are present. RFC 6455 masking, FIN/RSV, opcode, payload, and control-frame limits are structurally enforced.
- `/mcp` POST, legacy SSE replay, `/bridge` Hello/Welcome/Ping/Pong/record/Bye paths, retained retries/readiness generation, cancellation, and one-owner close state machines are present. This is source inspection only, not behavioral acceptance.

## Gates Run

```text
rustfmt --edition 2021 --check \
  🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs
bun ./📜️script.ts verify interactivity --self-test
git diff --check
git diff --cached --check
git diff HEAD --check
```

All exited 0. The verifier reported its existing record-only test allowlist entry and still ended in deny-mode clean; its MCP structural self-tests passed. These results do not catch the semantic decoder/broadcast paths above.

## Required Repair and Future Proof

Before a source acceptance, make every inbound count and field length prove against remaining raw bytes and explicit fixed limits before allocation; use fallible fixed-capacity/vector admission so malformed declarations return `GatewayError` rather than panic. Add adversarial raw WebSocket fixtures for an `Instances` count of `u32::MAX`, vector/string-list count bombs, and exact raw terminal handback. Put a no-allocation size preflight ahead of `broadcast` cloning and either preserve per-recipient rejected owners through an explicit API or make broadcast acceptance/rejection semantics bounded and observable. Extend the verifier's adversarial corpus to reject removal of these limits.

After repair, compilation and real controlled socket/WebSocket/SSE, cancellation, slowloris, pool-contention, malformed-frame, and memory-bound probes remain required. Broader Phase 1 runtime and admission gates remain outside P1o.
