# Phase 1 MCP HTTP Owned Transport Implementation — 2026-08-23

## Source Status

Source implementation is ready for independent Phase 1 audit. This is not a Phase 1 acceptance claim. Cargo, Nx, Wasm, browser, network, and runtime test execution were intentionally not run under the packet constraints.

## Runtime Cutover

The live `run_http` entry now binds through `HttpTransport::start(server)?.wait()`. `start` owns a nonblocking `std::net::TcpListener`, transfers the server and listener into a retained `HttpTransportAuthority`, and submits one closure at a time to the process-wide `WorkerPool` `Lane::Io`. The synchronous `wait` remains only at the process-entry boundary. Live MCP transport and bridge production source no longer constructs or drives a Tokio runtime, calls `block_on`, starts a thread, calls `tokio::spawn`, owns a Tokio sync channel, or serves through Axum.

Axum/Tokio code remains behind `#[cfg(test)]` as the pre-existing differential adapter and integration oracle. No Axum, Tokio, socket, or worker-pool type was added to an exported transport interface.

## Retained Authorities and Limits

- Each pool closure calls `HttpTransportState::drive_one` exactly once.
- One turn performs at most one terminal-owner close, shutdown transfer, accept, socket read page, HTTP parser token, request dispatch, WebSocket frame, bridge-outbox receive, socket write page, or connection close.
- Read and write pages are capped at 16 KiB.
- The transport has 64 generation-keyed connection slots and a 64-owner terminal FIFO.
- Per-connection request and response ownership is capped at 1 MiB, with aggregate fixed byte ledgers covering all 64 slots.
- HTTP parsing caps request headers at 64 items, total header bytes at 65,536, header names at 64 bytes, header values at 8,192 bytes, and paths at 2,048 bytes.
- WebSocket payloads are capped at 1 MiB; control payloads remain capped at 125 bytes.
- A 15-second absolute header deadline terminalizes slowloris owners.
- Pool rejection retains the exact `Job`. Contended/saturated submission uses a generation-keyed, one-shot process-pool timer-wheel retry. Shutdown/poison transfers the exact closure to `take_terminal_job` and completes with an observable error.
- Socket readiness re-arm is generation-keyed and coalesced to one pending timer callback. Cancellation advances the run generation so stale callbacks cannot mutate an ABA run.
- Parser, I/O, cancellation, shutdown, and capacity faults transfer the exact socket plus retained ingress/egress bytes to `HttpTerminalConnection`. Callers can retrieve and explicitly close that owner. Internal terminal draining closes one FIFO owner per grant.

## HTTP and Bridge Contract

The live owned parser serves `/mcp` POST JSON-RPC, legacy `/mcp` GET SSE replay, and `/bridge` WebSocket upgrade on the same listener. Origin, bearer, bridge-token, WebSocket version, and MCP protocol-version rejection occurs before bridge registration or MCP dispatch.

SSE responses and WebSocket messages acquire the same response byte ledger used by ordinary HTTP responses, then leave the socket in 16 KiB write pages. The live bridge outbox is an owned 64-item/1-MiB fixed ring. `try_send_to` returns the exact rejected `GatewayToShell` frame on item capacity, byte capacity, unknown connection, or terminal close. The Tokio-compatible async receive adapter and Axum bridge server are test-only.

The owned WebSocket handshake uses local SHA-1 and Base64 helpers and matches the RFC 6455 accept vector. Client frames must be masked, final, RSV-clear, and one of binary, close, ping, or pong. Binary bridge frames preserve the existing `Hello`/`Welcome`, state/record, command, ping/pong, `Bye`, and close semantics.

Explicitly unsupported because the live bridge does not consume them: HTTP chunked transfer coding, persistent HTTP pipelining/keep-alive, WebSocket text frames, continuation/fragmented frames, extensions/compression, and non-version-13 upgrades. These forms fail closed instead of silently changing the existing consumed bridge protocol.

## Source Fixtures and Verifier

Direct Rust fixtures cover:

- connection 64/+1 FIFO handback;
- request and response bytes/+1 exact handback;
- readiness/retry generation ABA rejection;
- RFC WebSocket handshake parity;
- masked partial frame retention;
- unmasked, fragmented, oversize, control, and close frame handling;
- one terminal close per grant;
- one read page and one HTTP parser token per grant;
- slowloris terminalization;
- cancellation/shutdown one-owner drain;
- one response write page per grant;
- owned health response parity with the current Axum adapter;
- owned bridge handshake/message/close/error parity;
- bridge outbox 64/+1, bytes/+1, re-arm after one receive, and exact late-frame rejection after terminal close.

The existing root `📜️script.ts` interactivity verifier now strips `#[cfg(test)]` items and denies live MCP Tokio runtime builders, Tokio spawn/sync ownership, Axum serve ownership, `block_on`, dynamic bridge outboxes, unbounded socket reads/writes, missing fixed byte admission, missing terminal retrieval, stale-generation readiness, and permissive unmasked WebSocket handling. Adversarial self-tests mutate each of those seams.

## Verification

Permitted source-only gates run for this packet:

- `rustfmt --edition 2021` and `rustfmt --edition 2021 --check` over the three edited MCP Rust files;
- `bun ./📜️script.ts verify interactivity --self-test`;
- `bun ./📜️script.ts verify interactivity`;
- production-source scans for Tokio runtime/sync, Axum live serve, `block_on`, runtime/thread construction, unbounded socket helpers, and bridge queue ownership;
- scoped and whole-worktree whitespace checks.

The final exact outcomes are recorded after the last gate run below.

## Remaining Phase 1 Blockers

The accepted ShardExecutor packet remains unchanged. Phase 1 remains open: the readiness audit's remaining source blocker is store-sync nested `block_on`/runtime ownership and its missing runtime proof. MCP transport source requires independent audit and source-only verification cannot establish compile or runtime behavior because Cargo and runtime tests were prohibited for this packet.

## Final Gate Outcomes

- `rustfmt --edition 2021`: PASS.
- `rustfmt --edition 2021 --check`: PASS.
- `bun ./📜️script.ts verify interactivity --self-test`: PASS, deny mode clean. The one reported blocking-bridge finding is the existing permitted renderer process-entry `block_on`; the MCP verifier contributed zero findings.
- `bun ./📜️script.ts verify interactivity`: PASS, deny mode clean with the same existing permitted process-entry finding.
- Production MCP scan: PASS. There is no live `tokio::runtime::{Runtime,Builder}`, `Runtime::new`, `block_on`, `tokio::sync`, Tokio channel, thread constructor, or unbounded bridge queue. Reported `tokio::spawn`, `axum::serve`, `write_all`, and `read_to_end` occurrences are confined to `#[cfg(test)]` differential fixtures and were excluded by the verifier's authored-production scan.
- Scoped MCP/script `git diff --check`: PASS.
- New report `git diff --no-index --check`: PASS (normalized expected new-file exit).
- Whole unstaged `git diff --check`: PASS.
- Whole staged `git diff --cached --check`: PASS.
- Whole `git diff HEAD --check`: PASS.

## Terra Rejection Remediation

The first independent P1o audit rejected the source packet on four concrete ownership/protocol findings, with the remediation packet also requiring bounded delimiter search. The source now closes each finding as follows:

1. Opened bridge binary messages are decoded and classified before any ingress drain or request-credit release. Malformed frames terminalize as `Malformed`; a second `Hello` and currently unconsumed `AppFrames` terminalize as `Unsupported`. The terminal owner retains the exact masked raw frame and socket rather than silently consuming it.
2. `Sec-WebSocket-Key` must occur exactly once, contain the canonical 24-character Base64 form with `==` padding, contain only the Base64 alphabet, have canonical zero padding bits, and decode into exactly 16 nonce bytes. Validation precedes accept-key hashing and bridge state mutation.
3. CRLF search takes an explicit end cursor. Request-line search stops at 4,096 bytes and aggregate header search stops at 65,536 bytes; a delimiter beyond either boundary is never observed.
4. `GatewayToShell::encoded_len` performs checked per-field/u32 wire-size preflight without allocation. The fixed outbox claims its item and byte credits before calling the encoder. Admitted messages encode incrementally into at most 64 fixed 16 KiB pages; transport writes those pages through a retained cursor, one 16 KiB socket write per grant, without recreating a contiguous bridge payload. Response-credit rejection retains the encoded pages in the exact terminal connection owner.
5. Terminal sockets default to a parked public FIFO handback policy. The pool no longer races `take_terminal_connection` by automatically closing the same owner. Retrieval pops one generation-keyed owner, invalidates the coalesced readiness generation, and schedules one capacity-change opportunity. The process-entry `wait` explicitly selects close policy, which closes one FIFO owner per pool grant. A full active-plus-terminal census parks until retrieval instead of readiness-polling on terminal capacity.

New direct fixtures cover malformed unknown-tag and unsupported post-open `Hello` raw retention, valid/duplicate/invalid-alphabet/padding/whitespace/width/noncanonical WebSocket keys, request-line/header cap boundaries with a late CRLF, outbox exact-cap/+1 pre-encode rejection and page boundaries, public terminal FIFO with same-slot generation ABA, retrieval, and process close mode. The verifier now adversarially mutates silent malformed continuation, consume-before-decode, duplicate/unvalidated keys, unbounded delimiter search, encode-before-preflight, allocation-before-claim, automatic terminal close, and readiness left armed after retrieval.

### Remediation Gate Rerun

- `rustfmt --edition 2021 --check`: PASS after remediation.
- `bun ./📜️script.ts verify interactivity`: PASS, deny mode clean.
- `bun ./📜️script.ts verify interactivity --self-test`: PASS, deny mode clean; every new adversarial mutation is rejected.
- Final rejection-pattern and runtime-ownership source scans: PASS with zero live matches.
- Final scoped MCP/script `git diff --check`: PASS.
- Final report `git diff --no-index --check`: PASS with the expected normalized new-file exit.
- Final whole unstaged, staged, and `HEAD` diff checks: PASS.

## Second Terra Re-audit Remediation — 2026-08-23

The second independent audit found two deeper ownership seams: generated `ShellToGateway::decode` still received untrusted bridge bytes before it allocated nested owners, and `broadcast` cloned/encoded independently for each recipient before aggregate admission. This remediation removes both live paths.

### Retained Shell Decode and Construction

- `ClientWebSocketFrame` now retains only the masked raw ingress range, mask, opcode, and exact consumed byte count. It does not allocate an unmasked payload.
- `ShellToGatewayDecodeCursor` covers all nine live tags. Its fixed 1,280-range table and 256-item ledger validate every scalar, `u32` count, field range, nested `Instances.window_ids`, `AppFrames.frames`, remaining-byte minimum, aggregate owned bytes, tag, boolean, enum, trailing byte, and UTF-8 sequence before any variable owner is constructed.
- One transport grant advances one tag/scalar/count/range, one UTF-8 byte, or one 16 KiB validated page. The exact masked raw frame remains in connection ingress throughout validation.
- A second `ShellToGatewayMaterializeCursor` owns the validated fixed pages. It exact-reserves one typed field/container and then copies at most one 16 KiB byte page or one Unicode scalar per grant. It hand-constructs every `ShellToGateway` variant; the live transport no longer calls generated `ShellToGateway::decode`.
- Connection generation is checked before every retained decode/materialize step. Malformed, over-capacity, unsupported, cancelled, or stale work terminalizes with the unchanged raw masked frame and its request-byte credit. Ingress is consumed exactly once only after a complete accepted typed message.

### Shared Broadcast Admission, Encoding, Leases, and Retirement

- `BridgeHandle::broadcast` now returns `Result<usize, GatewayToShell>`, so every synchronous admission rejection returns the exact original message.
- Admission checks the shared encoded byte count, checked aggregate recipient bytes, the fixed 64-cursor broadcast ring, the fixed 256-cursor retirement ring, and every recipient's item/byte/generation grant before retaining the message. Partial recipient admission rolls back every prior grant.
- The admitted original moves into `BridgeBroadcastCursor`. One process-wide `WorkerPool` I/O-lane closure writes exactly one 16 KiB fixed encoded page. No recipient clone and no contiguous encoded `Vec` is created.
- After page completion, recipients receive shallow `Arc<BridgeEncodedFrame>` leases carrying their outbox generation and exact byte credit. Closed/stale generations cannot publish or yield an ABA lease.
- Every shared encoded frame is constructed with a preclaimed retirement token. The last lease's `Drop` cannot deallocate all pages inline: it transfers the fixed page array into the bounded retirement ring. The process pool releases one page per grant. Generation-coalesced scheduling and timer-wheel retry retain the exact rejected closure; Shutdown/Poisoned exposes that job and one pending original broadcast, while `close_one_terminal_retired_page` drains one retained page owner per explicit terminal grant.

### Added Source Fixtures and Adversarial Mutations

Direct fixtures now cover `0xffffffff` at `Instances`, nested `read_string_vec`, and nested `read_bytes_vec` count positions; truncated counts/ranges; field cap/+1; parity for every variant; incremental scalar progress; cancellation/stale-generation raw owner handback; partial broadcast saturation rollback; 64/+1 recipients; oversize preflight; shared-lease identity; generation ABA/close; last-lease two-page terminal retirement; and terminal broadcast retrieval with exact original-message and recipient-credit handback.

The interactivity verifier now rejects a live generated bridge decoder, preflight payload allocation, missing count/minimum-range/aggregate-byte validation, non-retained materialization, clone/encode before broadcast admission, missing aggregate checked multiplication, missing retirement reservation, ordinary last-lease page drop, absent incremental page encoding, absent generation leases, and missing generation-coalesced terminal retrieval/retirement. Its synthetic good corpus and every new adversarial mutation pass.

### Second Remediation Source-only Status

The implementation is ready only for another independent source audit. P1o is not claimed accepted, Phase 1 remains open, and no compile/runtime statement is made because Cargo and runtime execution were explicitly prohibited. The accepted P1n source was not changed. The remaining readiness-audit blocker outside this packet is store-sync nested runtime/`block_on` ownership and its missing runtime proof.

### Second Remediation Final Gates

- `rustfmt --edition 2021` and `rustfmt --edition 2021 --check` over the three MCP Rust files: PASS.
- `bun ./📜️script.ts verify interactivity --self-test`: PASS, deny mode clean; the one reported blocking-bridge census item is an existing allowlisted process-entry boundary and the MCP authored-source/verifier checks contribute no failure.
- `bun ./📜️script.ts verify interactivity`: PASS with the same exact census status.
- Direct rejection-pattern/source-presence scans: PASS. There is no `ShellToGateway::decode`, Tokio runtime builder, `block_on`, Tokio synchronization owner, subsystem thread builder, resizable `GatewayToShell` queue, or broadcast `frame.clone()` in the authored live paths. Generated decode occurrences are confined to the `#[cfg(test)]` Axum oracle and codec/differential fixtures.
- Scoped MCP/script `git diff --check`: PASS.
- New report `git diff --no-index --check`: PASS for whitespace, with exit 1 solely because the report is an expected untracked new file.
- Whole unstaged `git diff --check`, staged `git diff --cached --check`, and combined `git diff HEAD --check`: PASS. The whole worktree remains concurrently dirty outside this packet; those unrelated owners were not edited or normalized.

## Broadcast Recipient Close-Race Remediation — 2026-08-23

The second source re-audit accepted the retained decoder and shared-page broadcast architecture but found one reachable panic: a recipient can close after aggregate admission and before its retained publish step. The source now treats that generation change as an ordinary deterministic recipient result.

- `BridgeOutbox::publish` returns `BridgeRejectedPublish` containing the exact rejected generation/byte grant and shared encoded lease. It does not mutate a closed or stale-generation outbox.
- `BridgeBroadcastCursor` retains every admitted recipient in stable connection-id order. One retained step resolves at most one recipient as `Published` or `RecipientClosed`; a closed recipient does not prevent subsequent still-valid recipients from publishing.
- No stale/closed publish branch uses `unreachable!` or `panic!`. The exact rejected grant remains attached to `RecipientClosed` until the whole broadcast reaches a completion boundary.
- Shared pages and the original shell owner remain retained until all admitted recipient claims are resolved. A zero-delivery result exposes `BridgeBroadcastCompletion::Undelivered` with the exact original frame and closed count. A partial/full delivery result exposes exact delivered and closed counts.
- Broadcast completions use a pre-reserved fixed FIFO. `take_broadcast_completion` makes zero-delivery handback and partial-delivery accounting observable.
- Cancel, pool Shutdown/Poisoned, and process close use `close_one_terminal_broadcast_claim`; each explicit grant closes at most one remaining recipient claim. Last-lease pages continue through the pre-reserved retirement cursor, one page owner per pool or explicit terminal grant.
- The cursor is stepped outside the bridge authority mutex, so an all-closed final shared lease can transfer its pages to the retirement ring without recursively locking the authority.

Direct fixtures cover a close before the first publish on a multi-page message, a close in the middle of a recipient list with surviving FIFO delivery, all recipients closing with exact original completion, same-slot generation ABA rejection, partial delivery followed by cancel/Shutdown/Poisoned terminal claim closure, public completion retrieval, and last-page terminal retirement. The verifier now rejects a reintroduced broadcast close panic, discarded rejected grant, multiple-recipient publish loop, dropped all-close original, unstable admitted ordering, or missing one-claim terminal close authority.

### Close-Race Source-only Status

The isolated P1o close-race finding is source-closed and this packet is ready for another independent source re-audit. This is not a P1o or Phase 1 acceptance claim. Cargo, Nx, Wasm, browser, network, root lint, compilation, and runtime/socket execution were not run. P1n remained untouched; store-sync runtime/`block_on` ownership and controlled runtime proof remain Phase 1 blockers outside this packet.

### Close-Race Final Gates

- `rustfmt --edition 2021 --check` over transport, bridge, and MCP root Rust files: PASS.
- `bun ./📜️script.ts verify interactivity --self-test`: PASS, deny mode clean; all close-race adversarial mutations are rejected and MCP contributes no verifier failure.
- `bun ./📜️script.ts verify interactivity`: PASS, deny mode clean; the output retains one existing allowlisted census finding and no MCP failure.
- Production-ownership scans: PASS. Live MCP source has no Tokio runtime builder, `Runtime::new`, `block_on`, Tokio synchronization owner, live Axum server, subsystem thread spawn, resizable `GatewayToShell` queue, old claimed-lease `unreachable!`, or broadcast clone-before-admission. Reported Tokio/Axum and panic occurrences are inside `#[cfg(test)]` differential/fixture code and are excluded by the authored-production verifier.
- Scoped MCP/script `git diff --check`: PASS.
- Report `git diff --no-index --check`: PASS for whitespace; exit 1 is the normalized expected difference for this untracked report.
- Whole unstaged `git diff --check`, staged `git diff --cached --check`, and combined `git diff HEAD --check`: PASS. Whole-worktree stats still include unrelated concurrent P3/P8/P10/store/dependency owners; they were inspected only and not edited by this packet.
