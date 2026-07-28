# CW5 — Wire v2 client integration

Single agent. Prerequisite: CW4 (the full `db` family, 24 crates) is complete and verified;
`protocol_wire` (one of the 12 `protocol` crates, complete since CW2) defines the wire types.
CW3 deliberately deferred this: framework/core's `HubClientFrame`/`HubServerFrame` (the old JSON
hub protocol) and `framework/sync`'s usage of them were left untouched on purpose, exactly for
this wave.

Read first: `/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md` Part 3 "Wire
protocol v2" and "Client refactor" sections (lines 87-93) for the full frame catalogue and client
behavior spec. Read `/Users/ueli/Documents/semio/protocol/wire/rs/lib.rs` in full — it already
implements `Lane{Command,Preview}`, `ClientFrame`/`ServerFrame` enums, and
`encode_client_frame`/`decode_client_frame`/`encode_server_frame`/`decode_server_frame`. This wave
does NOT touch the wire format itself — it wires framework/sync (the client) to speak it.

## Scope

1. **`framework/sync/rs/lib.rs`**: replace the JSON `HubClientFrame`/`HubServerFrame` usage
   (currently `use semio_framework_core::{HubClientFrame, HubServerFrame, OperationEnvelope,
   PresencePeer};` and the WS send/receive logic around `send_client_frame`/`on_hub_frame`, lines
   ~600-800 — re-read fresh, this is an approximate pointer) with `protocol_wire::{ClientFrame,
   ServerFrame, encode_client_frame, decode_client_frame, encode_server_frame, decode_server_frame,
   Lane}`. The WS transport moves from text (JSON) frames to **binary** frames for
   `Commands`/`Welcome.backlog`-equivalent/similar bulk payloads; keep control-only exchanges
   (Hello, Presence, simple Acks) as whatever `protocol_wire::ClientFrame`/`ServerFrame` actually
   define for them (read the enum variants — don't assume, the plan's frame catalogue is a spec
   summary, `protocol_wire`'s actual Rust types are ground truth).
2. Add `SyncSession`'s new behavior per the plan: `publish_preview`, `DocumentEvent::{Preview,
   CommandOutcome}`; on `Ack::Applied::Rejected` roll back the speculative head via the inverse
   machinery (now in `protocol_command`); on `Transformed` replace the local envelope.
   `hub_version: i64` becomes a `Frontier` (from `protocol_causal` or wherever the frontier type
   now lives post-CW3 — check). Resume tokens: `Hello` carries one, `Welcome` returns one.
3. **`framework/sync/worker/rs`**: update to match — read it fresh, it's the wasm-sandboxed worker
   wrapping `framework/sync`'s actor for the browser.
4. **`framework/product/os/core/js/backbone-worker.ts`**: this is the TS side of the same worker
   boundary — read it fresh. Frames become opaque bytes at this layer (per the plan: "one copy at
   the JS boundary, zero-copy views inside wasm; worker + backbone-worker.ts treat frames as opaque
   bytes").
5. **Binary fixtures**: `framework/sync/fixtures/` currently holds JSON fixtures for actor tests —
   per the plan, add/convert to binary fixtures shared between the Rust cargo tests and any
   vitest/TS tests exercising the same wire frames, so both sides test against byte-identical
   sample frames.

## What NOT to touch

`protocol_wire` itself (frame format is frozen from CW2). The hub server side (`os-hub`,
`compose-hub`) — that's CW6. Any app plugin crate — that's CW7.

## Verify

`cargo build -p semio-framework-sync -p semio-framework-sync-worker` (check exact crate name via
Cargo.toml) succeeds. `cargo test -p semio-framework-sync` passes, including any
`MemoryBackbone`/actor convergence tests now exercising binary frames. If a TS build/typecheck is
quickly runnable for `backbone-worker.ts` (check for a relevant `script.ts`/`package.json` test
target), run it; note if you can't verify the TS side compiles/tests cleanly rather than skipping
silently.

## Report

Write `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw5-report.txt`:
files touched, the exact new frame-handling flow, build/test results, and anything deferred with a
clear note for CW6 (hub rebuild, which is the actual server-side counterpart these client changes
need to talk to — the hub itself is still JSON-only until CW6 lands, so end-to-end wire
compatibility won't be real until then; that's expected and fine, note it rather than trying to
also touch the hub server yourself this wave).
