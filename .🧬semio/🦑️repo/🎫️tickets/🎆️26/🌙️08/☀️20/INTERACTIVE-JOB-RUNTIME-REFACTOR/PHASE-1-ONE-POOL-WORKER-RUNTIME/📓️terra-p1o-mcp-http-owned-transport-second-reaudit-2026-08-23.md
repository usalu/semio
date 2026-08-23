# Terra P1o MCP Owned-Transport Second Source Re-Audit — 2026-08-23

## Verdict

**REJECT — source packet only.** The second remediation closes the two previously reported bounded-decoder and pre-admission-clone defects, but a live recipient-close race still reaches `unreachable!` in retained broadcast publication. That is a normal stale-generation outcome under the authored ownership model, not an invariant violation. It can panic the process and does not provide deterministic rollback or terminal-owner handling.

This is a narrow P1o source verdict, not a Phase 1 verdict.

## Scope and Limits

Read: both prior P1o rejection reports, the updated implementation report, current transport/bridge/root/verifier sources, relevant direct fixtures, and the live scoped diff.

Cargo, Nx, Wasm, browser, network, root lint, compilation, socket-host execution, and runtime/security tests were not run as directed. This report therefore makes no runtime, timing, interoperability, or security-execution claim.

## Previous Rejections Rechecked

| Requirement | Current source result |
| --- | --- |
| No generated decoder/allocation on live untrusted Shell-to-Gateway bytes | Closed on the owned transport path. A binary WebSocket frame retains mask, raw offset, length, and consumed count only. `ShellToGatewayDecodeCursor` reads unmasked bytes from raw ingress; `parse_one_websocket_frame` contains no live `ShellToGateway::decode` call. The remaining calls are in `#[cfg(test)]` Axum/oracle/codec fixtures. |
| Counts, fields, nested containers, UTF-8, aggregate caps, and raw error ownership | Structurally closed. The cursor caps payload at 1 MiB, uses a 256-item ledger and fixed 1,280-range table, checks count × minimum remaining bytes, range end, aggregate owned bytes, scalar enums/bools, trailing bytes, and incremental UTF-8. Only after successful preflight does it copy fixed 16 KiB validation pages. `ShellToGatewayMaterializeCursor` uses fallible exact reservation and copies one field page or scalar per retained turn. Capacity/malformed faults occur before `consume_websocket_ingress`, so terminalization retains raw masked ingress. Direct count-bomb, truncated-range, cap/+1, parity, incremental-progress, and stale/cancellation fixtures are present. |
| `broadcast` clone/encode before aggregate admission | Closed at initial admission. `broadcast` computes checked encoded size, checks recipients and checked aggregate bytes, reserves fixed broadcast/retirement slots, claims every recipient before retaining the original frame, and returns the exact original on synchronous failure after rolling prior claims back. The retained cursor creates one 16 KiB page per `step`; recipients receive `Arc<BridgeEncodedFrame>` leases rather than deep clones. |
| Last shared lease and retirement | Structurally present. The frame `Drop` moves page owners into a pre-reserved retirement cursor; worker and terminal paths release one real page per grant. Retry state, terminal job retrieval, terminal original-broadcast retrieval, and explicit terminal page drain are present. |
| Earlier HTTP/WS findings | Still structurally closed: pre-consumption classification, raw terminal ownership, canonical unique Base64 WebSocket key validation, bounded CRLF search, `try_send_to` preflighted fixed-page output, Handback-vs-process-Close terminal policy, nonblocking WorkerPool `Lane::Io` steps, auth/origin/version ordering, HTTP/SSE, RFC frame limits, slowloris, retries, cancellation, and fixed connection/request/response bounds. |

## Blocking Finding

**Close after aggregate admission can panic during retained broadcast publish.** `BridgeHandle::broadcast` correctly claims every recipient while holding `connections`, then drops that lock and queues `BridgeBroadcastCursor`. A concurrent (or subsequent interleaved) transport close calls `unregister`, removes the entry, and `BridgeOutbox::close` advances its generation. When the retained cursor finishes its final fixed page, `BridgeBroadcastCursor::step` iterates the previously retained grants:

```rust
if claim.outbox.publish(claim.grant, Arc::clone(&encoded)).is_err() {
    unreachable!("claimed bridge broadcast lease became unavailable");
}
```

But `BridgeOutboxState::publish` intentionally returns `Err` when the outbox is closed or the grant generation is stale. The close is allowed after admission—the cursor owns only an `Arc<BridgeOutbox>`, not the connection-map lock—so this branch is reachable. It converts the required stale/closed generation rejection into a panic, leaving no defined all-recipient rollback, exact original terminal handback, or safe completed-broadcast policy.

The direct fixture `shared_broadcast_leases_are_generation_keyed_and_close_rejects_aba_publish` proves that `publish` can return `Err`, but it does not drive a `BridgeBroadcastCursor` through this timing: claim all recipients, close one before final page completion, then execute `step`. Consequently the fixture and text-level verifier cannot prove the asserted close-safe lease behavior.

This is sufficient to reject the packet. It directly violates the requested generation-lease/ABA and deterministic ownership contract.

## Required Repair

Define the close-after-admission state explicitly. At final publication, stale/closed recipient grants must be treated as an expected result, not `unreachable!`; preserve the invariant for already-published and not-yet-published recipients, and define whether a post-admission close yields a completed broadcast to surviving leases or atomically terminalizes/returns the original before any publication. In either design, cancel every unconsumed grant exactly once, preserve the original owner when terminalization requires it, and release shared pages only through the preclaimed retirement authority. Add a direct retained-cursor fixture covering a close between claim and final publish, including multi-page broadcast, mixed live/stale recipients, ABA generation reuse, retry/Shutdown/Poisoned retrieval, and terminal one-page retirement.

## Source Gates

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

All exited 0. The verifier reports one existing record-only test allowlist item and ends in deny-mode clean; its MCP mutations pass, but it does not model this retained close-after-admission interleaving. Production scans found no live generated decoder, Tokio runtime/builder, `block_on`, Tokio sync/channel, Axum server, explicit thread spawn, dynamic `GatewayToShell` queue, or broadcast `frame.clone()`; listed instances are test-only.

## Remaining P1 Limits

After source repair, the packet still needs compilation and controlled runtime probes for actual sockets/WebSocket/SSE, close-after-admission, pool contention, cancellation, slowloris, malformed UTF-8/counts, retirement, and shutdown/poison behavior. Broader Phase 1 admission/runtime work remains outside this packet, including the readiness audit's store-sync runtime/`block_on` ownership and runtime proof.
