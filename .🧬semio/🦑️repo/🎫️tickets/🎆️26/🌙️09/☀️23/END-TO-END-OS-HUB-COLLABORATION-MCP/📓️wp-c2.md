# WP-C2 — Hub Backbone Sync Parity (Rust actor ↔ TS worker)

## Verdict

**PASS** — language-agnostic fixture + dual runners green; rooted divergences fixed (no compat layers).

## Scope (gap G2)

Prove hub backbone sync parity between:

- Rust native `ArtifactActor` (`inject_hub_frame`) — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`
- TS browser worker (`handleHubFrame`) — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`

Constraints applied: session-9 preamble; TS1 offline/outbox + cold-pair admission; FL2 Member lane untouched; C7 Welcome None/Tail + rebootstrap/cold-pair.

## Fixture + runners

| piece | path |
|---|---|
| schema | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚖️parity/` schema JSON |
| fixtures | same tree, fixtures JSON (7 scenarios, version 1) |
| Rust runner | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/` `rs` |
| TS runner | same, `ts` — registered from worker `import.meta.vitest` |

Scenarios: `fresh-connect`, `resume-flush-once`, `remote-commands-order`, `ack-rejected`, `rebootstrap-required`, `duplicate-commands-idempotent`, `presence-passthrough`.

## Divergences found + fixes

1. **RebootstrapRequired** — Rust previously failed bootstrap / flipped to `Backoff` via `schedule_reconnect` in the same turn. Aligned with TS: clear pack/spr/archive/frontier/resume, set `artifact_rebootstrap_required`, stay **`Connecting`**, arm reconnect timer without immediate Backoff.
2. **Commands idempotency** — filter already-known mutation ids before deliver (`known_op_ids` / `ingestedMutationIds`).
3. **Presence** — TS updates `peerCount` on Live when Presence arrives (parity with Rust).
4. **Welcome None/Tail** while rebootstrap required — refuse (C7: pair expected).
5. **Parity harness** — Rust: `#[tokio::test]` (TcpListener needs Tokio); preserve fixture mutation ids (no `#0` suffix); map `RemoteState::Backoff` → fixture `"offline"`.
6. **TS harness** — `connectSocket` emits `SocketHelloV1` (mirrors Rust `connect_test_socket`); cold-pair mock exposes `drop()` for rebootstrap.
7. **Neighbour** — sync unit tests that bind sockets now use `#[tokio::test]` instead of park-executor `async_test`.
8. **Unblocks** — duplicate `active_tool` fields in sync unit; `DemoDiff` `pub(super)`; directory streaming `next_chunk(HttpTransportTerminalGuard)`.

FL2: Member child-lane paths untouched.

## Commands + results

```text
# Rust parity
CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-kernel --lib --features sync,ureq backbone_parity
→ ok (1 passed)   log: ticket generated/rust-parity-09.txt

# Rust neighbour
CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-kernel --lib --features sync,ureq native_terminal_connection_failure
→ ok (1 passed)   log: ticket generated/rust-neighbour-03.txt

# TS parity
cd …/os/packages/typescript && bun ./script.ts test long -t "backbone parity"
→ 7 passed        log: ticket generated/ts-parity-03.txt

# TS neighbour (hub frame / outbox / rebootstrap / socket actor)
bun ./script.ts test long -t "handleHubFrame|rebootstrap|outbox|socket actor"
→ 9 passed        log: ticket generated/ts-neighbour-01.txt
```

Scratch: `.tmp-ticket/wp-c2/` (ASCII links + logs). Symlink `.tmp-wp-c2` retained.

## Residual

- Shared cargo `target-dir` under repo cache (config); private `CARGO_TARGET_DIR` only uplifts deliverables.
- Full sync unit suite not exhaustively re-run (filtered neighbour + parity only).
