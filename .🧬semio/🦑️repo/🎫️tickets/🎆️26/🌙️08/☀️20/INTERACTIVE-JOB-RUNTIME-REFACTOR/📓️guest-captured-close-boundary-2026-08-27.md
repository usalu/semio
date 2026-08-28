# Guest-Captured Instance Close Boundary

## Historical Investigation Checkpoint

The following evidence describes the initial read-only investigation on 2026-08-27, before the new lifecycle schema and codec were mounted. The absent WIT records and old 34-byte transport below are historical findings, not the current source state. Current implementation and acceptance are recorded in `📓️guest-lifecycle-wire-contract-2026-08-27.md`; the native aggregate remains unfinished.

- `plugin/🚪️lifetime/🦀️component.rs` owns the native `PluginInstanceCloseLease`: capture stores a weak reference to the exact `RuntimeAppCell`; begin verifies pointer identity before detaching; the admitted close generation comes from the runtime; `is_retired` checks the exact cell and pump under nonblocking locks. Production guest use of `plugin_capture_instance_close` was not found; current call sites are native tests.
- `plugin/🧬️schema/📜️component.wit` has `instance-open-event` and `instance-close-event { instance }`, but no captured lifetime token, opened/captured receipt, close receipt, or receipt acknowledgement. Reactor `turn-result` has only patches/effects/presence/wake/status/fuel/command ingress.
- `plugin/⚛️reactor/🦀️component.rs` extracts numeric close IDs before reducing events, calls `plugin_destroy_app(runtime, id)`, and then advances global cleanup helpers. `plugin_destroy_app` calls the unbound `plugin_begin_instance_close(..., None)`. Capturing by ID at this late point would bind a delayed close to a replacement app.
- The native lease does not certify reactor tasks, requests, resumes, timers, metadata, patch trackers, pending patches, job/render bindings, turn-patch handbacks, or UI descendants. Several current helper results are discarded. Global emptiness or absence of a numeric ID is not an exact receipt.
- `ReactorCloseState` is currently removed from its fixed registry while advancing; a fault/unwind can lose the structural cursor owner. Pending-patch close admission must become explicit rather than silently refusing a saturated registry.
- The existing framework actor 34-byte close record is host-side transport. Its activation generation and an injected actor API are not a guest-issued app-lifetime capture or proof of native descendant retirement.

## Proposed Canonical Boundary

Use the existing reactor poll/event protocol, not a parallel close export, a polling subscriber, a synthetic host generation, or an Idle shortcut.

1. Pre-admit a fixed guest lifecycle-owner slot and a retained fixed receipt slot before publishing a newly opened app. After the app is created, capture its exact native lease immediately; allocate a checked, nonwrapping guest lifetime serial from the runtime owner. Keep this capture in the slot before publishing `Captured(instance, guestLifetime, openRequest)`.
2. The host retains the actual worker/actor activation object plus this guest-issued serial. A reused numeric instance within the same activation receives a different serial. A replacement worker has a different captured activation object and cannot accept a late receipt from the previous worker. The host open-request correlation is echoed only for correlation; it is not native lifetime authority.
3. Replace the numeric close event with an exact fixed request containing `(instance, guestLifetime, closeRequest)`. Validate it against the already captured slot before any detach or descendant mutation. Never recapture the current app when a delayed close arrives. Exhausted serials, full owner/receipt slots, foreign lifetimes, and stale requests fail before mutation.
4. Preflight all descendant-close admissions, then move their exact owners into the same lifecycle aggregate. Keep its native lease and each cursor structurally stored across every step and catch boundary. Advance one bounded child step and account actual work; a fault retains the owner for recovery rather than dropping, forgetting, or inferring retirement.
5. Return `Accepted` only after that ownership transfer is committed. Return `Retired` only after the native lease and every captured descendant owner have exact terminal witnesses. A native cell/pump witness alone is insufficient.
6. Deliver at most one fixed lifecycle receipt per turn from a retained outbox. Require an exact receipt acknowledgement; backpressure, synchronous transport failure, duplicate begin, and repeated poll must preserve the same receipt and ownership. The final acknowledgement releases the aggregate slot. A stale acknowledgement cannot release a replacement slot.

The minimal schema therefore needs a guest lifetime key, fixed open/close request correlation, captured/accepted/retired receipt variants, and exact receipt ACK. It must not express arbitrary payload lists or reuse command-ingress authority. JS must keep all `u64` values lossless. Native and WIT reducers must consume the same domain-owned record.

## Ownership Split

Proposed reservation for this lane: a new reactor lifecycle module, exact captured native/descendant owner and admission/retirement reducers, associated schema/neutral fixtures, and narrow kernel/WIT lifecycle record joins. Existing native lease internals remain the authority; changes to shared reactor entry signatures and generated mappings need a single coordinated source checkpoint.

Demonstrator retains generated worker/actor export and ShardClient activation/scheduler ownership. It should consume the canonical captured/receipt protocol once mounted, not fabricate guest callbacks in the interim. Renderer retains the independent JS UI aggregate; its terminal receipt is an additional host-side obligation before disposal.

No source region has been edited or reserved against an active peer without coordinator agreement. The tutorial retained-root/update/Store packet remains independent; broad reactor migration is not hidden inside it.

## Required Acceptance Laws

- Same activation, same numeric instance reused: old close, delayed Captured/Accepted/Retired, and old ACK do not affect the new native cell or slot.
- Guest serial exhaustion and every fixed admission saturation leave the live app and all descendants untouched.
- Exact duplicate request is idempotent; wrong request/serial/worker activation is rejected without shifting another receipt.
- Saturated output retains each fixed receipt; ACK is exact and does not equate transport delivery with descendant retirement.
- All descendant categories are populated; final native retirement alone cannot emit Retired. Aliased documents and pending patch acknowledgements block only their exact aggregate.
- Fault/unwind while advancing leaves the same structurally owned cursor reachable; held-lock paths report blocked without dropping or waiting.
- Missing/backward clocks and elapsed time at or above 8,000 microseconds never publish terminal success, while exact owners remain recoverable.

These are proposed gates, not executed runtime evidence. Existing native close, actor wire, and injected-worker tests do not establish the missing guest lifecycle join.

## Additional Exact Handback Joins

The current Kernel `UiTurnPatchRetireArena::close_one` still uses `patch.ops.pop()` and then removes the patch, while its arena wrapper takes a blocking mutex. Reactor pending external/returned patches use the same cold pop path. These must join the existing typed `UiPatchOps::close_step` and retain exact allocation ownership rather than credit a whole operation destructor as one item. A global `close_ui_turn_patch_owner_one` result cannot identify one app's descendants.

`UiTurnPatchTransportLease` already has an exact slot/epoch/session key, but its current Drop and take-owner use the global blocking arena. Capturing this exact key as a retained handback witness can support a lifecycle join; the current session-wide helper alone is not that proof. Typed document close already retains an exact slot/generation/epoch and waits for aliases before final descendant retirement. That existing exact owner, not global document-arena emptiness, is the model for aggregate handoffs.

These source observations are integration obligations, not newly executed tests. Shared UI generic-list and resident-meter regions remain owned by the Retained lane; this lane owns only the coordinated Kernel/reactor lifecycle and handback joins.
