# 📓️ terra-runtime-transact-report

Packet `runtime-transact` (wave W3). Owns `🦀️dispatch.rs` and `🦀️transaction.rs` in
`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/`.

## Done

- `🦀️dispatch.rs` — region `🔖️Intent`: `HandleIntent` trait, `DEFAULT_REVISION_TOLERANCE = 1`, the
  pure predicate `is_stale_intent(intent_revision, current_revision, tolerance)` with 4 unit tests.
  Region `🔖️Outcome`: `DispatchOutcome { Handled, HandledWith{commands, deferred}, Stale, Unhandled }`,
  `DeferredOp { SubmitCommand, PublishPresence, Custom }`, `DeferredKey(pub &'static str)`.
- `🦀️transaction.rs` — region `🔖️Runtime`: dyn-free `SurfaceSlot` (fn-pointer vtable over
  `Box<dyn Any>`, modeled on `ui-render`'s `AnySurface`/`SurfaceVTable`), `TransactFault`,
  `EFFECT_STORM_BUDGET = 64`, `PROJECTION_DRAIN_LIMIT = 256`, `Transacted`, `UiRuntime<S, D>` with
  `new`/`store_mut`/`register_surface`/`submit_intent`/`push_delta`/`register_custom_deferred`/
  `request_wake`/`gateway`. Region `🔖️Transact`: `transact(&mut self, now_ms) -> Transacted`
  implementing the exact 6-step order from the packet spec, each step its own private helper
  (`drain_and_apply_deltas`, `route_intents`/`apply_outcome`/`try_submit`/`publish_presence`,
  `flush_effects_to_fixpoint`, `present_dirty_surfaces`, `reconcile_trees`).
- 9 integration tests in `🦀️transaction.rs` covering every item in the packet's TESTS list, plus 4
  unit tests in `🦀️dispatch.rs` for the revision predicate. All new `fn`s carry the U1 tag.
- `🦀️reconcile.rs` landed mid-session (concurrent `runtime-reconcile` packet) with exactly the
  `SurfaceReconciler::{new, reconcile, snapshot, mark_rejected}` shape the spec promised — re-read from
  disk per U2 before writing `SurfaceSlot`/`current_revision`, so this is wired against the real API,
  not a guess.

## Acceptance: UNRUN (U4 — I do not run cargo)

```
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --lib --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --all-targets --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/target cargo test -p semio-framework-ui-runtime --lib --timeout 600000
```
Baseline set: `semio-framework-number` (lib, 620 errors), `semio-framework-actor` (lib test, 499
errors) per U5 at anchor `5e7b8046be`. This packet's crate (`semio-framework-ui-runtime`) was GREEN
before this change; no gate here may be `--workspace`.

## Decisions

1. **The `crate::DependencyTracker` bridge lives in `transact`.** Neither `EntityStore`/`Context`
   (landed, not owned by this packet) nor `DependencyTracker` (landed) call each other — `Context::
   notify` only queues an `EntityId` onto `EntityStore`'s `pub(crate)` effect queues. `transact`'s
   fixpoint loop peeks `store.effects.notify` (a `pub(crate)` field, crate-internal, no edit to
   `entity.rs`/`context.rs` needed) before each `flush_effects` cycle and turns each id into a
   `tracking.notify_entity(id)` call. This is the concrete mechanism behind master.md's "presented/
   reconciled" pipeline — without it no entity mutation could ever mark a surface dirty.
2. **`pending_first_present` seeds a surface's unconditional first present.** `DependencyTracker::
   notify_entity` can only mark a surface dirty from a previously recorded read, which a brand-new
   surface has none of — a real gap, not covered by any landed sibling's public API (`dirty` is a
   private field of `DependencyTracker`, not `pub(crate)`). `register_surface` seeds the surface into
   this set; `present_dirty_surfaces` unions it with `tracking.drain_dirty()` each transact call. This
   mirrors `SurfaceReconciler::new`'s own "first reconcile always emits everything" contract.
3. **`Custom(DeferredKey)` resolves through an explicit fn-pointer handler table**
   (`UiRuntime::register_custom_deferred`, `HashMap<&'static str, fn(&mut EntityStore)>`) — dyn-free
   (U3), and a key nobody registered is a safe no-op rather than a panic, since a handler describing
   effects must never be able to crash the transaction that carries them out.
4. **`UiRuntime` is generic over the embedder's delta shape** (`UiRuntime<S: CommandSink, D:
   ProjectionDelta>`, plus an `apply_delta: fn(&mut EntityStore, D)` thunk) since `crate::inbox::
   ProjectionDelta` (landed) only defines `key()`, not an `apply`, and this crate has no way to know
   what a delta represents on its own. The master.md sketch's `inbox: ..` was explicitly elided —
   this fills that gap.
5. **`request_wake`/`pending_wakes` is this packet's own deadline source.** No landed sibling exposes
   an "earliest deadline" getter (`PresenceHub`'s TTL expiries are internal; no accessor). `next_wake_ms`
   is computed from this explicit min-of-pending-deadlines list, pruned of anything `<= now_ms` each
   `transact` call. Registrar-request below covers extending this once `PresenceHub` gets one.
6. **`Transacted` gained a `faults: Vec<TransactFault>` field** beyond the four the packet's code
   sketch showed. The spec's own text ("Exceeding it is a fault surfaced in the result, not a hang…
   with enough information to find it") is unsatisfiable without a field to carry it, and no other
   landed struct anywhere in this ticket carries fault data as a side channel — flagged here as a
   deviation, not silently added.
7. **`try_submit`'s refusal policy**: a `HandleIntent` outcome's `commands`/`SubmitCommand`s are
   attempted through the gateway immediately during dispatch; a refusal (`GatewayError::Full`) simply
   omits that command from `Transacted::commands` this transaction rather than retrying or blocking —
   consistent with "a full command mailbox surfaces backpressure without blocking the transaction".
8. **`DeferredOp::PublishPresence(PresenceUpdate)` decomposes into `record_own` + one `record_peer`
   per mark** — `PresenceHub`'s landed API takes those, not a whole `PresenceUpdate`.

## Registrar-requests (not acted on — outside this packet's OWNS list)

- `SurfaceReconciler` has no cheap revision accessor; `SurfaceSlot::current_revision` calls `.snapshot()`,
  which clones every retained node just to read one field. A `SurfaceReconciler::revision(&self) ->
  UiRevision` accessor would remove that cost from the hot revision-guard path.
- `PresenceHub` has no "earliest pending expiry" accessor, so `next_wake_ms` cannot yet factor in
  peer-mark TTL deadlines — only explicit `UiRuntime::request_wake` calls. A getter there would let
  `transact` fold presence TTLs into the same computation.

## Deviations

- `Transacted::faults` added beyond the packet's literal struct sketch (see Decision 6).
- `UiRuntime`'s exact field set beyond `store/tracking/inbox/gateway/presence/surfaces` (which the
  spec itself left open with a trailing `..`) — `pending_first_present`, `pending_intents`,
  `custom_handlers`, `pending_wakes`, `apply_delta` — all load-bearing for the six-step order and the
  TESTS list; documented above rather than left implicit.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️dispatch.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs`
