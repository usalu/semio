# 📓️ terra-runtime-gateway report

Packet `runtime-gateway` (wave W2) — `🧠️runtime/…/🦀️gateway.rs`, `🦀️inbox.rs`, `🦀️presence.rs`.

## done

All three OWNS files replaced wholesale (scaffolds removed):

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️gateway.rs` — `CommandId`, `CorrelationId`,
  `Command` (this crate's own kernel-free envelope), `SinkFull`, `CommandSink` (generic-only, U3),
  `CommandTicket`, `OptimisticStatus`, `GatewayError::Full`, `CommandGateway<S: CommandSink>` with
  `new`/`try_submit`/`acknowledge`/`reject`/`mark_conflicted`/`status`/`len`/`is_empty`. 5 in-file tests.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️inbox.rs` — `ProjectionDelta` trait
  (associated `Key: Eq + Hash + Clone`), `InboxOverflow`, `ProjectionInbox<T: ProjectionDelta>` with
  `new`/`push`/`drain_into`/`len`/`is_empty`. 4 in-file tests.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️presence.rs` — `PresenceHub` (default
  constructible) with `new`/`record_own`/`record_peer`/`expire`/`flush`, private `PresenceKey`/
  `PeerEntry`/`PresenceEntry`. 5 in-file tests.

No edits outside these three files. `📦️glue.rs`/`Cargo.toml`/`📋️project.json`/`📜️script.ts` were
already scaffolded by `sol` in W1-style and untouched here — re-read from disk, still wire all three
modules correctly (`mod`/`pub use` for `gateway`/`inbox`/`presence` present at
`📦️glue.rs:31-36,45-47`).

Every non-test `fn` in all three files carries the `// 🚫️async: U1 …` tag (verified: 10/10 in
gateway.rs, 6/6 in inbox.rs, 7/7 in presence.rs — script in `📝️gate-w2-runtime-gateway-async-tags.txt`
if the coordinator wants to re-check). No `async fn`. No `dyn CommandSink` in actual code (one hit is
inside a doc comment explaining the U3 ruling). Brace-balance and region-pairing verified with a
non-cargo Python script.

## acceptance: UNRUN

Per U4 the coordinator (`sol`) runs these; only pasting the numbers below counts as acceptance.

```
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target timeout 600000 cargo check -p semio-framework-ui-runtime --lib
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target timeout 600000 cargo check -p semio-framework-ui-runtime --all-targets
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target timeout 600000 cargo test  -p semio-framework-ui-runtime --lib
```

Expect `--all-targets` to fail to LINK the crate as a whole today, because `🦀️entity.rs`, `🦀️context.rs`,
`🦀️tracking.rs`, `🦀️present.rs`, `🦀️reconcile.rs`, `🦀️dispatch.rs`, `🦀️transaction.rs` are still empty
`SCAFFOLD` region stubs owned by sibling packets (`runtime-entity`, `runtime-present`,
`runtime-reconcile`, `runtime-transact`) that have not landed yet — `📦️glue.rs`'s `pub use
transaction::*` etc. re-export nothing from an empty module, which is harmless, but any packet whose
region stub already declares an unimplemented item signature would fail to compile the crate as a
whole. This is expected per-crate incompleteness during W2, not a defect in this packet — the three
files this packet owns are self-contained against `ui_contract` only and should compile and test green
in isolation once the sibling files at minimum parse (they do, as of this writing: empty region blocks).

## decisions

**`Command` envelope shape, and why it stays kernel-free.** `Command { id: CommandId(u64), correlation:
CorrelationId(u64), payload: ui_contract::UiValue }`. The master doc explicitly forbids this crate from
depending on the os-kernel's `DslValue` or any actor type (wasip2/wasm32-unknown-unknown compile
target), so the payload is the contract's own neutral `UiValue` — the same enum `UiIntent::args`/`input`
already use. A `CommandSink` impl on the host side (e.g. the planned
`🔌️plugin/🖥️host/🧠️ui-runtime-bridge.rs`) is what converts a `Command`'s `UiValue` payload into whatever
the kernel mailbox's real message type is; that conversion is explicitly out of this crate's scope by
design, matching how `DslValue`⇄`UiValue` conversions live in os-kernel, never here (per
`🦀️action.rs`'s own docs on `UiValue`).

**Two capacities, deliberately.** `CommandGateway::try_submit` checks its OWN outstanding-ticket bound
(`capacity`, tracking only `Pending` commands) BEFORE ever calling `CommandSink::try_send` — a full
local bound fails fast without even touching the sink. Only if that has headroom does it forward to the
sink, and a `SinkFull` from the sink surfaces identically as `GatewayError::Full`. This means a host can
size the gateway's `capacity` independently of (typically ≤) the real backing mailbox's own capacity, to
bound how much UI-side tracking memory one surface's outstanding commands are allowed to consume,
without needing to ask the sink what its capacity is.

**Exact signatures `runtime-transact` will call:**
- `CommandGateway::<S>::try_submit(&mut self, command: Command) -> Result<CommandTicket, GatewayError>`
  — call once per dispatched intent that needs to leave the transaction; on `Err(GatewayError::Full {
  command_id })` the caller sets the initiating control's `Activity` to `Waiting` (contract's
  `Activity`, not this crate's) and retries the same `Command` next `transact()` — this crate does not
  retry on the caller's behalf and does not hand the `Command` back on failure (the caller already owns
  the data it built the `Command` from and can reconstruct/reclone it for the retry).
- `CommandGateway::<S>::acknowledge/reject/mark_conflicted(&mut self, command_id: CommandId) -> bool` —
  called from wherever `transact()`'s drain-inbox step lands an ack/reject/conflict event; `bool` is
  `false`, not a panic, if the id is unknown (already resolved, or foreign to this gateway).
- `CommandGateway::<S>::status(&self, command_id: CommandId) -> Option<OptimisticStatus>` — the
  presenter-facing read; a control layers this over its own projection read before painting.
- `ProjectionInbox::<T>::push(&mut self, delta: T) -> Result<(), InboxOverflow>` — called from wherever a
  CQRS projection handler enqueues a delta; requires `T: ProjectionDelta` (an associated `Key:
  Eq+Hash+Clone` via `fn key(&self) -> Self::Key`) so same-key deltas coalesce automatically. Whatever
  concrete delta type `runtime-present`/`runtime-reconcile` defines needs one `impl ProjectionDelta for
  ThatType` — not provided here, since the delta shape itself belongs to those packets.
- `ProjectionInbox::<T>::drain_into(&mut self, limit: usize, out: &mut Vec<T>)` — `transact()`'s
  drain-inbox step calls this once per transaction with its own per-transaction work budget as `limit`;
  never call with an unbounded limit inside a frame transaction.
- `PresenceHub::record_own(&mut self, surface: SurfaceId, node_key: impl Into<String>, own:
  OwnPresence, ttl_ms: u32)` / `record_peer(&mut self, surface, node_key, mark: PeerMark, ttl_ms: u32,
  now_ms: u64)` — called wherever hover/selection/peer-cursor input lands, entirely outside document
  reconciliation.
- `PresenceHub::expire(&mut self, now_ms: u64)` then `PresenceHub::flush(&mut self) -> Vec<PresenceUpdate>`
  — call `expire` then `flush` once per transaction (or per outbound tick) with the current wall clock;
  the returned `Vec<PresenceUpdate>` is what goes out on the separate presence wire channel, never mixed
  into a `UiPatch`.

**Why own presence has no TTL.** `OwnPresence` is this session's own, always-current local state — there
is no "did the sender disconnect" question to answer for your own session, so giving it an expiry would
just mean a locally-idle-but-still-connected session's own hover mysteriously vanishing. Only
`PeerMark` entries — reports of what some OTHER, possibly-gone session last said — need `expires_at_ms`.

**`PresenceEntry` garbage-collection.** Once a key's own presence has gone back to `OwnPresence::default()`
and its peer set is empty, `flush()` reports it one last time (so a receiver's local copy actually
clears) and then removes the internal entry — so a hub with no live presence anywhere does not grow
`entries` without bound over a long session.

## registrar-requests

None. No dependency needed beyond the already-present `ui_contract`.

## deviations

None from the packet brief. `GatewayError` carries only the `Full` variant the brief specified —
nothing else in the tracked-ticket lifecycle needs a distinct error today (`acknowledge`/`reject`/
`mark_conflicted`/`status` on an unknown id return `bool`/`Option`, not `Result`, since "id not found"
is a normal, expected outcome for a presenter probing status, not an exceptional one).
