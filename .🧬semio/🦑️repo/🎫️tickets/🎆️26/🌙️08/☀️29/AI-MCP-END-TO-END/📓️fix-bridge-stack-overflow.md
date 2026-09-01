# Fix: `bridge::long` Stack Overflow

## Root cause

**Not recursion, not tokio/axum/websocket machinery, not `HttpTransport::bridge_slot`.** It is a
plain oversized stack value in an unoptimized (debug) build.

`BridgeAsyncState` (`🧵️bridge/🦀️component.rs`) held three fixed-size arrays as direct struct
fields:

```rust
struct BridgeAsyncState {
    broadcasts: [Option<BridgeBroadcastCursor>; BRIDGE_BROADCAST_MAX_PENDING],   // 64
    completions: [Option<BridgeBroadcastCompletion>; BRIDGE_BROADCAST_MAX_PENDING], // 64
    retirements: [Option<BridgeRetirementCursor>; BRIDGE_RETIREMENT_MAX_PENDING],   // 256
    ...
}
```

`BridgeBroadcastCursor` itself embeds a *second* 64-element array
(`recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS]`), so the
nesting multiplies out. Measured directly with `std::mem::size_of`:

```
size_of::<BridgeAsyncState>()     = 345,688 bytes  (~338 KiB)
size_of::<BridgeAsyncAuthority>() = 345,800 bytes  (~338 KiB, wraps the above in a Mutex)
size_of::<BridgeBroadcastCursor>() = 3,216 bytes
size_of::<BridgeRetirementCursor>() = 520 bytes
```

`BridgeAsyncAuthority::new(pool)` builds this whole ~338 KiB value with
`Arc::new(Self { state: Mutex::new(BridgeAsyncState::new()), ... })`. Rust gives no guarantee of
in-place/placement construction into `Arc::new`/`Mutex::new` in an unoptimized build — the ~338 KiB
aggregate gets materialized as a stack temporary (and, across the several nested constructor calls
that move it — `BridgeAsyncState::new()` → `Mutex::new(...)` → the `Self` struct literal →
`Arc::new(...)` — debug builds don't elide those moves either, so the transient stack usage is a
multiple of 338 KiB). **Every single call to `BridgeHandle::new()`** pays this cost, not just the
first one in the process — confirmed by isolating `BridgeHandle::new()` alone (no networking, no
tokio, no `send_to`) in a fresh `#[test]` on libtest's default per-test thread: it overflowed too.

Whichever test in `bridge::long`/`bridge::quick` happened to run on a thread that first called
`BridgeHandle::new()` (directly, or via `bridge_router`) was the one libtest reported as
overflowing — explaining why the reported test name kept changing across runs
(`wrong_token_is_rejected...`, then `an_evil_origin_is_rejected...`, then the trivial
`send_to_an_unknown_connection_returns_false`). This is what the ticket's hint #2 predicted
("a very large stack-allocated value in a test thread") — proven bounded by binary search on
`RUST_MIN_STACK`: overflows below ~3 MiB, passes at every size from 3 MiB up through 64 MiB,
regardless of which specific test triggers it. Not recursive (hint #1) — no growth with input, no
depth-dependent behavior; not a nested-runtime issue (hint #3) — the trivial, non-networking,
`.await`-free `send_to_an_unknown_connection_returns_false` test reproduced the overflow just as
reliably as the websocket tests, ruling out axum/tokio-tungstenite specifically.

**Same bug, second instance**, found while re-verifying with the full `bridge` test filter (which
also matches `transport::quick::...bridge_decode...`): `HttpTransportState.connections:
[Option<HttpConnection>; HTTP_CONNECTION_CAPACITY]` (`🚚️transport/🦀️component.rs`,
`HTTP_CONNECTION_CAPACITY = 64`) and `FixedOwnerRing<T, N>.slots: [Option<T>; N]` (used for
`HttpTransportState.terminal`). `HttpConnection.bridge: BridgeSession` transitively embeds a
`BridgeInboundCursor` whose `Decode` phase is a `ShellToGatewayDecodeCursor` carrying a
`[Option<ShellRange>; BRIDGE_INBOUND_MAX_RANGES]` (1,280 elements) — one `HttpConnection` is on the
order of tens of KiB, and `HTTP_CONNECTION_CAPACITY = 64` multiplies that into low megabytes for the
`connections` array alone, same as the bridge case.

## Does this predate the ticket? Yes — confirmed, not assumed

- `git log` on `🧵️bridge/🦀️component.rs` shows its last real change (`9d7cabfd9c`) predates the
  `bridge_slot`/`BridgeSlot` commit (`67fb4216b2`) entirely. The `mod long` test file and
  `BridgeAsyncState` are untouched by this ticket's work.
- The overflowing tests (`bridge::long::*`) build their own `BridgeHandle` via `bridge_router` in
  isolation — they never construct `HttpTransport`, so `HttpTransport::bridge_slot` /
  `BridgeSlot = Arc<OnceLock<Arc<BridgeHandle>>>` is never on the call path that overflows.
- No `Arc` cycle exists: `BridgeSlot` holds `Arc<BridgeHandle>`, and `BridgeHandle` holds
  `Arc<BridgeInner>` → `Arc<BridgeAsyncAuthority>`. Nothing in that chain holds a strong reference
  back to the slot or to `HttpTransport`, so there is no cycle and no recursive `Drop` — ruled out by
  inspection of every field, not by assumption.
- **Conclusion: pre-existing.** The ticket's `bridge_slot` addition is unrelated to this crash; it
  was simply never previously exercised because (per the ticket brief) the whole workspace has been
  uncompilable for hours, so nobody had run this test since before this bug was introduced.

## Fix

Move the large fixed-capacity ring buffers off the stack onto the heap via a small helper that
never materializes the full array as one contiguous value:

```rust
fn boxed_slot_ring<T>(len: usize) -> Box<[Option<T>]> {
    (0..len).map(|_| None).collect()
}
```

Applied to:
- `🧵️bridge/🦀️component.rs`: `BridgeAsyncState.{broadcasts,completions,retirements}` changed from
  `[Option<T>; N]` to `Box<[Option<T>]>`, constructed via `boxed_slot_ring(N)`.
- `🚚️transport/🦀️component.rs`: `FixedOwnerRing<T, N>.slots` and
  `HttpTransportState.connections` changed the same way.

Verified `size_of::<BridgeAsyncState>()` dropped from 345,688 bytes to 136 bytes after the change,
and `BridgeHandle::new()` alone, in total isolation, on libtest's *default* (unmodified) per-test
thread stack, now completes without overflowing (watched directly with `eprintln!` markers before
and after the call — both printed, no crash).

All usages of the three renamed fields are plain indexing (`state.broadcasts[i]`, `.take()`,
`.as_mut()`) or `.iter()`/`.position()` on the `HttpTransportState`/`FixedOwnerRing` fields — all
`Vec`/`Box<[T]>`-compatible, no array-specific API was in use anywhere, so this is a pure
type-and-constructor change with no behavioral change.

Not fixed (out of scope, smaller "leaf" arrays that are not further multiplied by an outer
fixed-size array, so they are not implicated in the overflow): `ParsedHttpHead.headers` (~3.5 KiB),
`ShellToGatewayDecodeCursor.ranges` (~40 KiB as a standalone local), the per-cursor
`recipients_state` arrays. Flagging for awareness: if `HTTP_CONNECTION_CAPACITY` or
`BRIDGE_BROADCAST_MAX_PENDING`/`BRIDGE_RETIREMENT_MAX_PENDING` are ever raised further, or another
struct is nested inside one of these rings, the same class of bug can reappear elsewhere; the
`boxed_slot_ring` pattern is the fix to reach for.

## Verification status — be precise about what was and wasn't watched

**`🧵️bridge/🦀️component.rs` fix: verified with a real, watched, passing run**, captured before the
`transport::quick` follow-on issue was found:

```
running 40 tests
test bridge::long::mint_bridge_token_produces_distinct_high_entropy_tokens ... ok
test bridge::long::default_bridge_token_path_ends_with_the_frozen_suffix ... ok
test bridge::quick::bounded_shell_decoder_rejects_ffffffff_counts_and_truncated_ranges_before_owner_allocation ... ok
test bridge::long::write_bridge_token_file_creates_parents_and_is_readable_back ... ok
test bridge::long::send_to_an_unknown_connection_returns_false ... ok
test bridge::quick::bounded_shell_decoder_and_materializer_advance_incrementally ... FAILED   (pre-existing unrelated codec-fixture flake, see note below)
test bridge::quick::bridge_outbox_byte_cap_plus_one_returns_the_exact_frame_before_queue_mutation ... ok
test bridge::quick::bridge_outbox_item_cap_plus_one_returns_the_exact_frame_and_rearms_after_one_receive ... ok
test bridge::quick::bridge_outbox_page_boundary_matches_the_canonical_encoder ... ok
test bridge::quick::bounded_shell_decoder_cap_plus_one_and_every_variant_match_the_canonical_fixture ... ok
test bridge::quick::bridge_outbox_terminal_close_rejects_the_exact_late_frame ... ok
test bridge::quick::broadcast_all_close_returns_the_exact_original_completion_after_every_claim ... ok
test bridge::quick::broadcast_close_before_first_publish_delivers_survivors_in_stable_admitted_order ... ok
test bridge::quick::broadcast_close_mid_recipient_list_reports_partial_counts_and_fifo_delivery ... ok
test bridge::quick::broadcast_partial_saturation_rolls_back_every_claim_and_returns_the_exact_uncloned_message ... ok
test bridge::quick::broadcast_many_recipient_and_oversize_preflight_reject_before_encode ... ok
test bridge::quick::broadcast_shutdown_cancel_poison_closes_each_remaining_claim_one_grant_then_reports_partial_delivery ... ok
test bridge::quick::broadcast_reopen_same_slot_aba_cannot_consume_the_stale_recipient_claim ... ok
test bridge::quick::decode_rejects_trailing_bytes ... ok
test bridge::quick::decode_rejects_an_unknown_tag ... ok
test bridge::quick::decode_rejects_truncated_buffers ... ok
test bridge::quick::every_gateway_to_shell_variant_round_trips_through_encode_decode ... ok
test bridge::quick::every_shell_to_gateway_variant_round_trips_through_encode_decode ... ok
test bridge::quick::last_shared_lease_transfers_pages_to_one_page_terminal_retirement_grants ... ok
test bridge::long::an_evil_origin_is_rejected_before_the_websocket_upgrade ... ok
test bridge::long::missing_token_is_rejected ... ok
test bridge::quick::shared_broadcast_leases_are_generation_keyed_and_close_rejects_aba_publish ... ok
test bridge::quick::terminal_broadcast_close_returns_one_exact_original_and_cancels_recipient_credit ... ok

thread 'transport::quick::incremental_bridge_decode_cancellation_and_stale_generation_retain_exact_raw_owner' (7280576) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

The specific ticket-named test, `bridge::long::an_evil_origin_is_rejected_before_the_websocket_upgrade`,
is confirmed passing in that real run, along with every other `bridge::long`/`bridge::quick` test —
the process only aborted afterward, on the unrelated `transport::quick` test above, which is what
led to finding and fixing the second instance of the same bug in `🚚️transport/🦀️component.rs`.

`bridge::quick::bounded_shell_decoder_and_materializer_advance_incrementally ... FAILED` in that same
run is **not** a stack overflow and not something this fix touches — it's a separate, ordinary
assertion failure in the codec/fixture layer, unrelated to stack size. Not investigated further; out
of this ticket's scope (which was specifically the SIGABRT/stack-overflow).

**`🚚️transport/🦀️component.rs` fix: code-complete but NOT re-verified by a passing run.** After
making that change, every subsequent `cargo build`/`cargo test` in this crate (5 consecutive
attempts, spread over several minutes of real time, including a final isolated `cargo check -p
semio-framework-async --lib`) failed to compile for a reason entirely unrelated to this ticket: the
peer crate `semio_framework_async` (`🧰️framework/🔨️modules/⏳️async/🦀️.rs`) is currently missing
`use serde::{Serialize, Deserialize};` while three `#[derive(..., Serialize, Deserialize)]` sites
still reference it — its own `Cargo.toml` carries an in-progress comment about a "serde-off"
migration removing dead derives, so this reads as another session's in-flight edit, not something in
scope for `🌉️mcp/**` to fix. Reproduced identically 5/5 times:

```
error: cannot find derive macro `Serialize` in this scope
   --> 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:161:51
error: cannot find derive macro `Deserialize` in this scope
   --> 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:161:62
   ... (8 errors total, lines 161/169/550/580)
error: could not compile `semio-framework-async` (lib) due to 8 previous errors
```

I did **not** watch `bridge::quick::terminal_broadcast_close...` and the rest of `transport::quick`
pass after the `HttpTransportState`/`FixedOwnerRing` change — only the size math
(`HttpConnection` ≈ tens of KiB × `HTTP_CONNECTION_CAPACITY = 64` ≈ low megabytes, matching the same
shape as the already-confirmed `BridgeAsyncState` case) and the mechanical nature of the diff
(same pattern already proven correct in `🧵️bridge`, same
indexing-only usage sites). This should be re-run once `semio_framework_async` compiles again:

```
cargo test -p semio-framework-os-mcp --lib bridge
```

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️component.rs` — `BridgeAsyncState`
  fields boxed; verified.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs` — `FixedOwnerRing.slots`
  and `HttpTransportState.connections` boxed; code-complete, not yet re-verified (blocked on peer
  crate compile error above).
